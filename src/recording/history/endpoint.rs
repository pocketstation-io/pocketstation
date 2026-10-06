use std::collections::HashSet;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::{AudioHistory, AudioHistoryError, AudioHistoryState};
use crate::endpoint::{
    EndpointAudioReceiver, EndpointCancellationOutcome, EndpointDriverFactory,
    EndpointDriverFinalization, EndpointDriverObservations, EndpointFailure, EndpointFailureStage,
    EndpointGroupId, EndpointPortInput, EndpointPreparationGroup, EndpointReceiver,
    EndpointShutdownMode, EndpointStartGate, PreparedEndpointDriver, RunningEndpointDriver,
};
use crate::frame::{RouteId, SampleSpec, SessionId, StemId};
use crate::timing::TimelineMapping;

const RUNNING: u8 = 0;
const DRAIN: u8 = 1;
const ABORT: u8 = 2;

pub(crate) struct HistoryFactory(pub AudioHistory);

struct Input {
    receiver: EndpointAudioReceiver,
    spec: SampleSpec,
    session_id: SessionId,
    stem_id: StemId,
    mapping: TimelineMapping,
}

impl EndpointDriverFactory for HistoryFactory {
    fn preparation_group(
        &self,
        _route_id: RouteId,
        _config: &crate::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, EndpointFailure> {
        Ok(EndpointPreparationGroup::Shared(EndpointGroupId::new(
            "session.audio-history.v1",
        )))
    }

    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        if inputs.is_empty() || inputs.len() > 64 {
            return Err(failure(
                EndpointFailureStage::Prepare,
                AudioHistoryError::InvalidLimits,
            ));
        }
        let mut stems = HashSet::new();
        let mut prepared = Vec::with_capacity(inputs.len());
        let mut expected_session_id = None;
        let mut expected_origin_ns = None;
        for input in inputs {
            let (receiver, context) = input.into_parts();
            let stem_id = context.route_context().audio_stem_id().ok_or_else(|| {
                failure(
                    EndpointFailureStage::Prepare,
                    AudioHistoryError::InvalidFrame,
                )
            })?;
            let session_id = context.session_id();
            let origin_ns = context.session_timeline_origin().monotonic_timestamp_ns();
            if !stems.insert(stem_id)
                || expected_session_id.is_some_and(|id| id != session_id)
                || expected_origin_ns.is_some_and(|ns| ns != origin_ns)
            {
                return Err(failure(
                    EndpointFailureStage::Prepare,
                    AudioHistoryError::InvalidFrame,
                ));
            }
            expected_session_id = Some(session_id);
            expected_origin_ns = Some(origin_ns);
            let EndpointReceiver::Audio {
                receiver,
                sample_spec,
            } = receiver
            else {
                return Err(failure(
                    EndpointFailureStage::Prepare,
                    AudioHistoryError::InvalidFrame,
                ));
            };
            prepared.push(Input {
                receiver,
                spec: sample_spec,
                session_id,
                stem_id,
                mapping: TimelineMapping::new(origin_ns, 0),
            });
        }
        Ok(Box::new(Prepared {
            history: self.0.clone(),
            inputs: prepared,
        }))
    }
}

struct Prepared {
    history: AudioHistory,
    inputs: Vec<Input>,
}

impl PreparedEndpointDriver for Prepared {
    fn start(
        self: Box<Self>,
        gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let shutdown = Arc::new(AtomicU8::new(RUNNING));
        let worker_shutdown = Arc::clone(&shutdown);
        let history = self.history.clone();
        let worker_history = history.clone();
        let inputs = self.inputs;
        let worker = thread::Builder::new()
            .name("pks-audio-history".into())
            .spawn(move || {
                let result = run(worker_history.clone(), inputs, &gate, &worker_shutdown);
                if result.is_err() {
                    if let Ok(mut state) = worker_history.shared.lock() {
                        state.clear();
                        state.observations.state = AudioHistoryState::Failed;
                    }
                }
                result
            })
            .map_err(|error| {
                EndpointFailure::new(EndpointFailureStage::Start, error.to_string())
            })?;
        Ok(Box::new(Running {
            history,
            shutdown,
            worker: Some(worker),
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        if let Ok(mut state) = self.history.shared.lock() {
            state.clear();
            state.observations.state = AudioHistoryState::Cancelled;
        }
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

struct Running {
    history: AudioHistory,
    shutdown: Arc<AtomicU8>,
    worker: Option<JoinHandle<Result<(), AudioHistoryError>>>,
}

impl Running {
    fn join(&mut self) -> Result<(), EndpointFailure> {
        let result = self.worker.take().map_or(Ok(()), |worker| {
            worker
                .join()
                .map_err(|_| AudioHistoryError::Failed)
                .and_then(|result| result)
        });
        if result.is_err() {
            if let Ok(mut state) = self.history.shared.lock() {
                state.clear();
                state.observations.state = AudioHistoryState::Failed;
            }
        }
        result.map_err(|error| failure(EndpointFailureStage::JoinFinalize, error))
    }
}

impl RunningEndpointDriver for Running {
    fn observations(&self) -> EndpointDriverObservations {
        match self.history.observations() {
            Ok(value) => EndpointDriverObservations {
                frames_received_total: value.received_buffers_total,
                frames_delivered_total: value
                    .received_buffers_total
                    .saturating_sub(value.rejected_buffers_total),
                frames_dropped_total: value.rejected_buffers_total,
                discontinuities_total: value.discontinuities_total,
                failures_total: u64::from(value.state == AudioHistoryState::Failed),
            },
            Err(_) => EndpointDriverObservations {
                failures_total: 1,
                ..Default::default()
            },
        }
    }
    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        self.shutdown.fetch_max(DRAIN, Ordering::Release);
        Ok(())
    }
    fn request_shutdown(&mut self, mode: EndpointShutdownMode) -> Result<(), EndpointFailure> {
        self.shutdown.fetch_max(
            match mode {
                EndpointShutdownMode::Drain => DRAIN,
                EndpointShutdownMode::Abort => ABORT,
            },
            Ordering::Release,
        );
        Ok(())
    }
    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        self.shutdown.fetch_max(DRAIN, Ordering::Release);
        let result = self.join();
        EndpointDriverFinalization {
            observations: self.observations(),
            result,
        }
    }
}

/// Control-thread destructor: signal abort and join the owned worker; never
/// installed on callbacks, realtime partitions or audio buffer destructors.
impl Drop for Running {
    fn drop(&mut self) {
        if self.worker.is_some() {
            self.shutdown.store(ABORT, Ordering::Release);
            let _ = self.join();
        }
    }
}

fn run(
    history: AudioHistory,
    mut inputs: Vec<Input>,
    gate: &EndpointStartGate,
    shutdown: &AtomicU8,
) -> Result<(), AudioHistoryError> {
    while !gate.is_open() && shutdown.load(Ordering::Acquire) == RUNNING {
        thread::park_timeout(Duration::from_millis(1));
    }
    if !gate.is_open() {
        let mut state = history
            .shared
            .lock()
            .map_err(|_| AudioHistoryError::Failed)?;
        state.clear();
        state.observations.state = AudioHistoryState::Cancelled;
        return Ok(());
    }
    history
        .shared
        .lock()
        .map_err(|_| AudioHistoryError::Failed)?
        .observations
        .state = AudioHistoryState::Running;
    loop {
        if shutdown.load(Ordering::Acquire) == ABORT {
            let mut state = history
                .shared
                .lock()
                .map_err(|_| AudioHistoryError::Failed)?;
            state.clear();
            state.observations.state = AudioHistoryState::Cancelled;
            return Ok(());
        }
        let mut progressed = false;
        for input in &mut inputs {
            for _ in 0..64 {
                if shutdown.load(Ordering::Acquire) == ABORT {
                    break;
                }
                let Some(frame) = input.receiver.try_recv() else {
                    break;
                };
                progressed = true;
                let lineage = frame.lineage();
                if lineage.session_id() != input.session_id
                    || lineage.stem_id() != input.stem_id
                    || frame.source_id() != lineage.source_id()
                    || frame.sample_rate_hz() != input.spec.sample_rate_hz
                    || frame.channels() != input.spec.channels
                {
                    input.receiver.mark_worker_failure();
                    return Err(AudioHistoryError::InvalidFrame);
                }
                let timestamp_ns = input
                    .mapping
                    .normalize_timestamp_ns(frame.timestamp_ns())
                    .ok_or(AudioHistoryError::InvalidFrame)?;
                let result = history
                    .shared
                    .lock()
                    .map_err(|_| AudioHistoryError::Failed)?
                    .push(lineage, input.spec, timestamp_ns, frame.samples());
                if let Err(error) = result {
                    input.receiver.mark_worker_failure();
                    return Err(error);
                }
            }
        }
        if !progressed {
            if shutdown.load(Ordering::Acquire) == DRAIN {
                history
                    .shared
                    .lock()
                    .map_err(|_| AudioHistoryError::Failed)?
                    .observations
                    .state = AudioHistoryState::Complete;
                return Ok(());
            }
            thread::park_timeout(Duration::from_millis(1));
        }
    }
}

fn failure(stage: EndpointFailureStage, error: AudioHistoryError) -> EndpointFailure {
    EndpointFailure::new(stage, error.to_string()).with_external_details(
        error.code(),
        crate::endpoint::EndpointFailureRetryability::ReconfigurationRequired,
    )
}

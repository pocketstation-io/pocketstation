use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::endpoint::{
    EndpointCancellationOutcome, EndpointDriverFactory, EndpointDriverFinalization,
    EndpointDriverInput, EndpointDriverObservations, EndpointFailure, EndpointFailureStage,
    EndpointGroupId, EndpointStartGate, PreparedEndpointDriver, RunningEndpointDriver,
    SessionTimelineOrigin,
};
use crate::frame::{EndpointId, RouteId, SessionId, StemId};
use crate::runtime::{PlanEdgeFrame, PlanEdgeReceiver};
use crate::timing::TimelineMapping;

use crate::recording::{
    MultistemRecording, PermissionDecision, PermissionScope, RecorderError, RecorderLineageField,
    RecorderStemConfig, RecordingObservations, RecordingOutcome, RecordingState, StemLabel,
};

const SESSION_RECORDER_IDLE_WAIT_MS: u64 = 1;

#[derive(Debug, thiserror::Error)]
pub enum MultistemEndpointError {
    #[error("multistem endpoint group must contain at least one stem")]
    NoStems,
    #[error("multistem endpoint group contains duplicate endpoint {0:?}")]
    DuplicateEndpoint(EndpointId),
    #[error("multistem endpoint '{label}' belongs to session {actual}, expected {expected}")]
    SessionMismatch {
        label: String,
        actual: u64,
        expected: u64,
    },
}

#[derive(Debug, Clone)]
pub struct MultistemEndpointStem {
    pub endpoint_id: EndpointId,
    pub recorder: RecorderStemConfig,
}

/// Session-scoped coordinator for one explicitly declared recording group.
///
/// The Session startup transaction supplies the exact group as one
/// `prepare_batch` call. This type never discovers or groups unrelated
/// endpoints through process-global state.
pub struct MultistemEndpointCoordinator {
    output_root: PathBuf,
    session_id: SessionId,
    group_id: EndpointGroupId,
    stems: Vec<MultistemEndpointStem>,
    receipt_state: Arc<MultistemRecordingReceiptState>,
}

#[derive(Clone)]
pub struct MultistemRecordingReceipt {
    state: Arc<MultistemRecordingReceiptState>,
}

impl MultistemRecordingReceipt {
    pub fn result(&self) -> Option<&RecordingOutcome> {
        self.state.result.get()
    }
}

#[derive(Default)]
struct MultistemRecordingReceiptState {
    result: OnceLock<RecordingOutcome>,
}

impl MultistemEndpointCoordinator {
    pub fn new(
        output_root: impl Into<PathBuf>,
        session_id: SessionId,
        group_id: EndpointGroupId,
        stems: Vec<MultistemEndpointStem>,
    ) -> Result<Self, MultistemEndpointError> {
        if stems.is_empty() {
            return Err(MultistemEndpointError::NoStems);
        }
        let mut endpoint_ids = HashSet::with_capacity(stems.len());
        for stem in &stems {
            if !endpoint_ids.insert(stem.endpoint_id) {
                return Err(MultistemEndpointError::DuplicateEndpoint(stem.endpoint_id));
            }
            if stem.recorder.session_id != session_id {
                return Err(MultistemEndpointError::SessionMismatch {
                    label: stem.recorder.label.as_str().to_owned(),
                    actual: stem.recorder.session_id.0,
                    expected: session_id.0,
                });
            }
        }
        Ok(Self {
            output_root: output_root.into(),
            session_id,
            group_id,
            stems,
            receipt_state: Arc::new(MultistemRecordingReceiptState::default()),
        })
    }

    pub fn output_root(&self) -> &Path {
        &self.output_root
    }

    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn stem_count(&self) -> usize {
        self.stems.len()
    }

    pub fn group_id(&self) -> &EndpointGroupId {
        &self.group_id
    }

    pub fn receipt(&self) -> MultistemRecordingReceipt {
        MultistemRecordingReceipt {
            state: Arc::clone(&self.receipt_state),
        }
    }
}

impl EndpointDriverFactory for MultistemEndpointCoordinator {
    fn prepare(
        &self,
        inputs: Vec<EndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        if inputs.len() != self.stems.len() {
            return Err(prepare_failure(format!(
                "recording group requires {} endpoint inputs, received {}",
                self.stems.len(),
                inputs.len()
            )));
        }

        let mut remaining_inputs = inputs;
        let mut prepared_stems = Vec::with_capacity(self.stems.len());
        for declaration in &self.stems {
            let Some(input_index) = remaining_inputs
                .iter()
                .position(|input| input.context().endpoint_id() == declaration.endpoint_id)
            else {
                return Err(prepare_failure(format!(
                    "recording group is missing endpoint {:?}",
                    declaration.endpoint_id
                )));
            };
            let input = remaining_inputs.swap_remove(input_index);
            let (receiver, context) = input.into_parts();
            if context.session_id() != self.session_id {
                return Err(prepare_failure(format!(
                    "endpoint {:?} belongs to session {}, expected {}",
                    declaration.endpoint_id,
                    context.session_id().0,
                    self.session_id.0
                )));
            }
            let declared_group_id = context
                .node_configuration()
                .get("recording_group_id")
                .ok_or_else(|| {
                    prepare_failure("recording endpoint is missing recording_group_id")
                })?;
            if declared_group_id != self.group_id.as_str() {
                return Err(prepare_failure(format!(
                    "endpoint {:?} belongs to recording group {declared_group_id:?}, expected {:?}",
                    declaration.endpoint_id,
                    self.group_id.as_str()
                )));
            }
            let declared_label = context
                .node_configuration()
                .get("stem_name")
                .ok_or_else(|| prepare_failure("recording endpoint is missing stem_name"))?;
            if declared_label != declaration.recorder.label.as_str() {
                return Err(prepare_failure(format!(
                    "endpoint {:?} declares stem_name {declared_label:?}, expected {:?}",
                    declaration.endpoint_id,
                    declaration.recorder.label.as_str()
                )));
            }
            let sample_spec = context.node_prepare_context().sample_spec;
            if sample_spec.sample_rate_hz != declaration.recorder.sample_rate_hz
                || sample_spec.channels != declaration.recorder.channels
            {
                return Err(prepare_failure(format!(
                    "endpoint {:?} sample spec is {} Hz/{} ch, expected {} Hz/{} ch",
                    declaration.endpoint_id,
                    sample_spec.sample_rate_hz,
                    sample_spec.channels,
                    declaration.recorder.sample_rate_hz,
                    declaration.recorder.channels
                )));
            }
            prepared_stems.push((declaration.recorder.clone(), receiver));
        }
        if !remaining_inputs.is_empty() {
            return Err(prepare_failure(
                "recording group contains an undeclared endpoint",
            ));
        }

        Ok(Box::new(PreparedMultistemEndpoint {
            output_root: self.output_root.clone(),
            session_id: self.session_id,
            stems: prepared_stems,
            receipt_state: Arc::clone(&self.receipt_state),
        }))
    }
}

/// Canonical Session-owned multistem recorder declaration.
///
/// Unlike the 0.1 compatibility coordinator, callers supply no capture
/// identity. Session startup contributes exact endpoint, route, stem, sample,
/// and timeline context; the first delivered frame contributes capture-time
/// source, clock, generation, and permission lineage.
pub struct SessionMultistemEndpointCoordinator {
    output_root: PathBuf,
    group_id: EndpointGroupId,
    receipt_state: Arc<MultistemRecordingReceiptState>,
}

impl SessionMultistemEndpointCoordinator {
    pub fn new(output_root: impl Into<PathBuf>, group_id: EndpointGroupId) -> Self {
        Self {
            output_root: output_root.into(),
            group_id,
            receipt_state: Arc::new(MultistemRecordingReceiptState::default()),
        }
    }

    pub fn output_root(&self) -> &Path {
        &self.output_root
    }

    pub fn group_id(&self) -> &EndpointGroupId {
        &self.group_id
    }

    pub fn receipt(&self) -> MultistemRecordingReceipt {
        MultistemRecordingReceipt {
            state: Arc::clone(&self.receipt_state),
        }
    }
}

impl EndpointDriverFactory for SessionMultistemEndpointCoordinator {
    fn prepare(
        &self,
        inputs: Vec<EndpointDriverInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        let session_id = inputs
            .first()
            .map(|input| input.context().session_id())
            .ok_or_else(|| prepare_failure("recording group must contain at least one endpoint"))?;
        let mut endpoint_ids = HashSet::with_capacity(inputs.len());
        let mut stem_ids = HashSet::with_capacity(inputs.len());
        let mut route_ids = HashSet::with_capacity(inputs.len());
        let mut labels = HashSet::with_capacity(inputs.len());
        let mut common_origin = None;
        let mut prepared_stems = Vec::with_capacity(inputs.len());

        for input in inputs {
            let (receiver, context) = input.into_parts();
            if context.session_id() != session_id {
                return Err(prepare_failure(format!(
                    "endpoint {:?} belongs to session {}, expected {}",
                    context.endpoint_id(),
                    context.session_id().0,
                    session_id.0
                )));
            }
            if !endpoint_ids.insert(context.endpoint_id()) {
                return Err(prepare_failure(format!(
                    "recording group contains duplicate endpoint {:?}",
                    context.endpoint_id()
                )));
            }
            let route_context = context.route_context().ok_or_else(|| {
                prepare_failure(format!(
                    "endpoint {:?} is missing typed Session route context",
                    context.endpoint_id()
                ))
            })?;
            if !stem_ids.insert(route_context.stem_id()) {
                return Err(prepare_failure(format!(
                    "recording group contains duplicate stem {:?}",
                    route_context.stem_id()
                )));
            }
            if !route_ids.insert(route_context.route_id()) {
                return Err(prepare_failure(format!(
                    "recording group contains duplicate route {:?}",
                    route_context.route_id()
                )));
            }
            let timeline_origin = context.session_timeline_origin().ok_or_else(|| {
                prepare_failure(format!(
                    "endpoint {:?} is missing the Session timeline origin",
                    context.endpoint_id()
                ))
            })?;
            if let Some(expected_origin) = common_origin {
                if timeline_origin != expected_origin {
                    return Err(prepare_failure(format!(
                        "endpoint {:?} has a different Session timeline origin",
                        context.endpoint_id()
                    )));
                }
            } else {
                common_origin = Some(timeline_origin);
            }
            let declared_group_id = context
                .node_configuration()
                .get("recording_group_id")
                .ok_or_else(|| {
                    prepare_failure("recording endpoint is missing recording_group_id")
                })?;
            if declared_group_id != self.group_id.as_str() {
                return Err(prepare_failure(format!(
                    "endpoint {:?} belongs to recording group {declared_group_id:?}, expected {:?}",
                    context.endpoint_id(),
                    self.group_id.as_str()
                )));
            }
            let label = StemLabel::new(
                context
                    .node_configuration()
                    .get("stem_name")
                    .ok_or_else(|| prepare_failure("recording endpoint is missing stem_name"))?,
            )
            .map_err(|error| prepare_failure(error.to_string()))?;
            if !labels.insert(label.clone()) {
                return Err(prepare_failure(format!(
                    "recording group contains duplicate stem label {:?}",
                    label.as_str()
                )));
            }
            let sample_spec = context.node_prepare_context().sample_spec;
            if sample_spec.sample_rate_hz == 0 || sample_spec.channels == 0 {
                return Err(prepare_failure(format!(
                    "endpoint {:?} has invalid sample spec {} Hz/{} ch",
                    context.endpoint_id(),
                    sample_spec.sample_rate_hz,
                    sample_spec.channels
                )));
            }
            prepared_stems.push(SessionPreparedStem {
                endpoint_id: context.endpoint_id(),
                label,
                stem_id: route_context.stem_id(),
                route_id: route_context.route_id(),
                sample_rate_hz: sample_spec.sample_rate_hz,
                channels: sample_spec.channels,
                receiver,
            });
        }

        let timeline_origin = common_origin.ok_or_else(|| {
            prepare_failure("recording group is missing a Session timeline origin")
        })?;
        Ok(Box::new(PreparedSessionMultistemEndpoint {
            output_root: self.output_root.clone(),
            session_id,
            timeline_origin,
            stems: prepared_stems,
            receipt_state: Arc::clone(&self.receipt_state),
        }))
    }
}

struct SessionPreparedStem {
    endpoint_id: EndpointId,
    label: StemLabel,
    stem_id: StemId,
    route_id: RouteId,
    sample_rate_hz: u32,
    channels: u8,
    receiver: PlanEdgeReceiver,
}

struct PreparedSessionMultistemEndpoint {
    output_root: PathBuf,
    session_id: SessionId,
    timeline_origin: SessionTimelineOrigin,
    stems: Vec<SessionPreparedStem>,
    receipt_state: Arc<MultistemRecordingReceiptState>,
}

impl PreparedEndpointDriver for PreparedSessionMultistemEndpoint {
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        for path in [
            self.output_root
                .join(format!("session-{}", self.session_id.0)),
            self.output_root
                .join(format!(".session-{}.pending", self.session_id.0)),
        ] {
            if path.exists() {
                return Err(EndpointFailure::new(
                    EndpointFailureStage::Start,
                    RecorderError::OutputExists(path).to_string(),
                ));
            }
        }
        let worker = SessionRecorderWorker::spawn(
            self.output_root,
            self.session_id,
            self.timeline_origin,
            self.stems,
            start_gate,
        )
        .map_err(|error| EndpointFailure::new(EndpointFailureStage::Start, error.to_string()))?;
        Ok(Box::new(RunningSessionMultistemEndpoint {
            worker: Some(worker),
            receipt_state: self.receipt_state,
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

struct RunningSessionMultistemEndpoint {
    worker: Option<SessionRecorderWorker>,
    receipt_state: Arc<MultistemRecordingReceiptState>,
}

impl RunningEndpointDriver for RunningSessionMultistemEndpoint {
    fn observations(&self) -> EndpointDriverObservations {
        self.worker
            .as_ref()
            .map_or_else(EndpointDriverObservations::default, |worker| {
                worker.observations()
            })
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        if let Some(worker) = &self.worker {
            worker.request_stop();
        }
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        let Some(worker) = self.worker.take() else {
            return failed_finalization("Session multistem recorder was already finalized");
        };
        match worker.join() {
            SessionRecorderWorkerOutcome::CancelledBeforeStart => EndpointDriverFinalization {
                observations: EndpointDriverObservations::default(),
                result: Ok(()),
            },
            SessionRecorderWorkerOutcome::Failed {
                message,
                observations,
            } => EndpointDriverFinalization {
                observations,
                result: Err(EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    message,
                )),
            },
            SessionRecorderWorkerOutcome::Finished(outcome) => {
                let mut observations = finalized_observations(&outcome);
                let mut result = recording_outcome_result(&outcome);
                if self.receipt_state.result.set(outcome).is_err() {
                    observations.failures_total = observations.failures_total.saturating_add(1);
                    result = Err(EndpointFailure::new(
                        EndpointFailureStage::JoinFinalize,
                        "multistem recording receipt was already finalized",
                    ));
                }
                EndpointDriverFinalization {
                    observations,
                    result,
                }
            }
        }
    }
}

fn failed_finalization(message: impl Into<String>) -> EndpointDriverFinalization {
    EndpointDriverFinalization {
        observations: EndpointDriverObservations {
            failures_total: 1,
            ..EndpointDriverObservations::default()
        },
        result: Err(EndpointFailure::new(
            EndpointFailureStage::JoinFinalize,
            message,
        )),
    }
}

struct SessionRecorderWorker {
    stop_requested: Arc<AtomicBool>,
    join_handle: Option<JoinHandle<SessionRecorderWorkerOutcome>>,
    telemetry: Arc<SessionRecorderTelemetry>,
}

impl SessionRecorderWorker {
    fn spawn(
        output_root: PathBuf,
        session_id: SessionId,
        timeline_origin: SessionTimelineOrigin,
        stems: Vec<SessionPreparedStem>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Self, std::io::Error> {
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop_requested);
        let telemetry = Arc::new(SessionRecorderTelemetry::default());
        let worker_telemetry = Arc::clone(&telemetry);
        let join_handle = thread::Builder::new()
            .name(format!("pocketstation-session-recorder-{}", session_id.0))
            .spawn(move || {
                run_session_recorder(
                    &output_root,
                    session_id,
                    timeline_origin,
                    stems,
                    &start_gate,
                    &worker_stop,
                    &worker_telemetry,
                )
            })?;
        Ok(Self {
            stop_requested,
            join_handle: Some(join_handle),
            telemetry,
        })
    }

    fn request_stop(&self) {
        self.stop_requested.store(true, Ordering::Release);
        if let Some(join_handle) = &self.join_handle {
            join_handle.thread().unpark();
        }
    }

    fn observations(&self) -> EndpointDriverObservations {
        self.telemetry.snapshot()
    }

    fn join(mut self) -> SessionRecorderWorkerOutcome {
        let Some(join_handle) = self.join_handle.take() else {
            return SessionRecorderWorkerOutcome::Failed {
                message: "Session multistem recorder join handle was already consumed".to_owned(),
                observations: EndpointDriverObservations {
                    failures_total: 1,
                    ..EndpointDriverObservations::default()
                },
            };
        };
        join_handle
            .join()
            .unwrap_or_else(|_| SessionRecorderWorkerOutcome::Failed {
                message: "Session multistem recorder worker panicked".to_owned(),
                observations: EndpointDriverObservations {
                    failures_total: 1,
                    ..EndpointDriverObservations::default()
                },
            })
    }
}

impl Drop for SessionRecorderWorker {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.thread().unpark();
            let _ = join_handle.join();
        }
    }
}

#[derive(Default)]
struct SessionRecorderTelemetry {
    frames_received_total: AtomicU64,
    frames_delivered_total: AtomicU64,
    frames_dropped_total: AtomicU64,
    discontinuities_total: AtomicU64,
    failures_total: AtomicU64,
}

impl SessionRecorderTelemetry {
    fn record_initialization_progress(&self, received_frames: u64) {
        self.frames_received_total
            .store(received_frames, Ordering::Release);
    }

    fn update(&self, observations: EndpointDriverObservations) {
        self.frames_received_total
            .store(observations.frames_received_total, Ordering::Relaxed);
        self.frames_delivered_total
            .store(observations.frames_delivered_total, Ordering::Relaxed);
        self.frames_dropped_total
            .store(observations.frames_dropped_total, Ordering::Relaxed);
        self.discontinuities_total
            .store(observations.discontinuities_total, Ordering::Relaxed);
        self.failures_total
            .store(observations.failures_total, Ordering::Relaxed);
    }

    fn record_initialization_failure(&self, received_frames: u64) {
        self.frames_received_total
            .store(received_frames, Ordering::Relaxed);
        self.failures_total.store(1, Ordering::Relaxed);
    }

    fn snapshot(&self) -> EndpointDriverObservations {
        EndpointDriverObservations {
            frames_received_total: self.frames_received_total.load(Ordering::Acquire),
            frames_delivered_total: self.frames_delivered_total.load(Ordering::Relaxed),
            frames_dropped_total: self.frames_dropped_total.load(Ordering::Relaxed),
            discontinuities_total: self.discontinuities_total.load(Ordering::Relaxed),
            failures_total: self.failures_total.load(Ordering::Relaxed),
        }
    }
}

enum SessionRecorderWorkerOutcome {
    CancelledBeforeStart,
    Finished(RecordingOutcome),
    Failed {
        message: String,
        observations: EndpointDriverObservations,
    },
}

fn run_session_recorder(
    output_root: &Path,
    session_id: SessionId,
    timeline_origin: SessionTimelineOrigin,
    mut stems: Vec<SessionPreparedStem>,
    start_gate: &EndpointStartGate,
    stop_requested: &AtomicBool,
    telemetry: &SessionRecorderTelemetry,
) -> SessionRecorderWorkerOutcome {
    while !start_gate.is_open() {
        if stop_requested.load(Ordering::Acquire) {
            return SessionRecorderWorkerOutcome::CancelledBeforeStart;
        }
        thread::park_timeout(Duration::from_millis(SESSION_RECORDER_IDLE_WAIT_MS));
    }

    let mut first_frames = std::iter::repeat_with(|| None)
        .take(stems.len())
        .collect::<Vec<Option<PlanEdgeFrame>>>();
    let mut received_frames = 0;
    while first_frames.iter().any(Option::is_none) {
        let mut made_progress = false;
        for (stem, first_frame) in stems.iter_mut().zip(&mut first_frames) {
            if first_frame.is_some() {
                continue;
            }
            if let Some(frame) = stem.receiver.try_recv() {
                received_frames += 1;
                telemetry.record_initialization_progress(received_frames);
                made_progress = true;
                if let Err(error) = validate_initial_frame(session_id, stem, &frame) {
                    stem.receiver.mark_worker_failure();
                    telemetry.record_initialization_failure(received_frames);
                    return SessionRecorderWorkerOutcome::Failed {
                        message: format!(
                            "endpoint {:?} route {:?}: {error}",
                            stem.endpoint_id, stem.route_id
                        ),
                        observations: telemetry.snapshot(),
                    };
                }
                *first_frame = Some(frame);
            }
        }
        if first_frames.iter().all(Option::is_some) {
            break;
        }
        if stop_requested.load(Ordering::Acquire) {
            telemetry.record_initialization_failure(received_frames);
            return SessionRecorderWorkerOutcome::Failed {
                message: "recording stopped before every stem delivered authoritative lineage"
                    .to_owned(),
                observations: telemetry.snapshot(),
            };
        }
        if !made_progress {
            thread::park_timeout(Duration::from_millis(SESSION_RECORDER_IDLE_WAIT_MS));
        }
    }

    let mut recorder_stems = Vec::with_capacity(stems.len());
    for (stem, first_frame) in stems.into_iter().zip(first_frames) {
        let Some(first_frame) = first_frame else {
            stem.receiver.mark_worker_failure();
            telemetry.record_initialization_failure(received_frames);
            return SessionRecorderWorkerOutcome::Failed {
                message: format!(
                    "endpoint {:?} route {:?} stem {:?} is missing its authoritative first frame",
                    stem.endpoint_id,
                    stem.route_id,
                    stem.label.as_str(),
                ),
                observations: telemetry.snapshot(),
            };
        };
        let config = match derive_recorder_config(session_id, timeline_origin, &stem, &first_frame)
        {
            Ok(config) => config,
            Err(error) => {
                stem.receiver.mark_worker_failure();
                telemetry.record_initialization_failure(received_frames);
                return SessionRecorderWorkerOutcome::Failed {
                    message: format!(
                        "endpoint {:?} route {:?}: {error}",
                        stem.endpoint_id, stem.route_id
                    ),
                    observations: telemetry.snapshot(),
                };
            }
        };
        recorder_stems.push((config, stem.receiver, first_frame));
    }
    let recording =
        match MultistemRecording::start_observed(output_root, session_id, recorder_stems) {
            Ok(recording) => recording,
            Err(error) => {
                telemetry.record_initialization_failure(received_frames);
                return SessionRecorderWorkerOutcome::Failed {
                    message: error.to_string(),
                    observations: telemetry.snapshot(),
                };
            }
        };

    while !stop_requested.load(Ordering::Acquire) {
        telemetry.update(endpoint_observations(recording.observations()));
        thread::park_timeout(Duration::from_millis(SESSION_RECORDER_IDLE_WAIT_MS));
    }
    recording.request_stop();
    match recording.finish() {
        Ok(outcome) => {
            telemetry.update(finalized_observations(&outcome));
            SessionRecorderWorkerOutcome::Finished(outcome)
        }
        Err(error) => {
            let mut observations = telemetry.snapshot();
            observations.failures_total = observations.failures_total.saturating_add(1);
            telemetry.update(observations);
            SessionRecorderWorkerOutcome::Failed {
                message: error.to_string(),
                observations,
            }
        }
    }
}

fn validate_initial_frame(
    session_id: SessionId,
    stem: &SessionPreparedStem,
    frame: &PlanEdgeFrame,
) -> Result<(), RecorderError> {
    let lineage = frame
        .lineage()
        .ok_or_else(|| RecorderError::MissingFrameLineage(stem.label.as_str().to_owned()))?;
    if lineage.session_id != session_id {
        return Err(RecorderError::LineageMismatch {
            label: stem.label.as_str().to_owned(),
            field: RecorderLineageField::Session,
            actual: lineage.session_id.0,
            expected: session_id.0,
        });
    }
    if lineage.stem_id != stem.stem_id {
        return Err(RecorderError::LineageMismatch {
            label: stem.label.as_str().to_owned(),
            field: RecorderLineageField::Stem,
            actual: lineage.stem_id.0,
            expected: stem.stem_id.0,
        });
    }
    if frame.sample_rate_hz() != stem.sample_rate_hz || frame.channels() != stem.channels {
        return Err(RecorderError::FrameSpecMismatch {
            label: stem.label.as_str().to_owned(),
            actual_rate_hz: frame.sample_rate_hz(),
            actual_channels: frame.channels(),
            expected_rate_hz: stem.sample_rate_hz,
            expected_channels: stem.channels,
        });
    }
    Ok(())
}

fn derive_recorder_config(
    session_id: SessionId,
    timeline_origin: SessionTimelineOrigin,
    stem: &SessionPreparedStem,
    frame: &PlanEdgeFrame,
) -> Result<RecorderStemConfig, RecorderError> {
    validate_initial_frame(session_id, stem, frame)?;
    let lineage = frame
        .lineage()
        .ok_or_else(|| RecorderError::MissingFrameLineage(stem.label.as_str().to_owned()))?;
    Ok(RecorderStemConfig {
        session_id,
        source_id: lineage.source_id,
        stem_id: stem.stem_id,
        clock_id: lineage.clock_id,
        source_generation: lineage.source_generation,
        permission_epoch: lineage.permission_epoch,
        // Delivery through a running Session proves this Session grant. It
        // does not assert an operating-system permission decision.
        permission_scope: PermissionScope::SessionCaptureGrant,
        permission: PermissionDecision::Allowed,
        label: stem.label.clone(),
        sample_rate_hz: stem.sample_rate_hz,
        channels: stem.channels,
        timeline_mapping: TimelineMapping::new(timeline_origin.monotonic_timestamp_ns(), 0),
    })
}

fn prepare_failure(message: impl Into<String>) -> EndpointFailure {
    EndpointFailure::new(EndpointFailureStage::Prepare, message)
}

struct PreparedMultistemEndpoint {
    output_root: PathBuf,
    session_id: SessionId,
    stems: Vec<(RecorderStemConfig, crate::runtime::PlanEdgeReceiver)>,
    receipt_state: Arc<MultistemRecordingReceiptState>,
}

impl PreparedEndpointDriver for PreparedMultistemEndpoint {
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let recording = MultistemRecording::start_gated(
            &self.output_root,
            self.session_id,
            self.stems,
            Arc::clone(&start_gate),
        )
        .map_err(|error| EndpointFailure::new(EndpointFailureStage::Start, error.to_string()))?;
        Ok(Box::new(RunningMultistemEndpoint {
            recording: Some(recording),
            start_gate,
            receipt_state: self.receipt_state,
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

struct RunningMultistemEndpoint {
    recording: Option<MultistemRecording>,
    start_gate: Arc<EndpointStartGate>,
    receipt_state: Arc<MultistemRecordingReceiptState>,
}

impl RunningEndpointDriver for RunningMultistemEndpoint {
    fn observations(&self) -> EndpointDriverObservations {
        self.recording
            .as_ref()
            .map_or_else(EndpointDriverObservations::default, |recording| {
                endpoint_observations(recording.observations())
            })
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        if let Some(recording) = &self.recording {
            recording.request_stop();
        }
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        let Some(recording) = self.recording.take() else {
            return EndpointDriverFinalization {
                observations: EndpointDriverObservations {
                    failures_total: 1,
                    ..EndpointDriverObservations::default()
                },
                result: Err(EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    "multistem recording was already finalized",
                )),
            };
        };
        let live_observations = endpoint_observations(recording.observations());
        if !self.start_gate.is_open() {
            return match recording.rollback_before_start() {
                Ok(observations) => EndpointDriverFinalization {
                    observations: endpoint_observations(observations),
                    result: Ok(()),
                },
                Err(error) => EndpointDriverFinalization {
                    observations: EndpointDriverObservations {
                        failures_total: live_observations.failures_total.saturating_add(1),
                        ..live_observations
                    },
                    result: Err(EndpointFailure::new(
                        EndpointFailureStage::JoinFinalize,
                        error.to_string(),
                    )),
                },
            };
        }
        match recording.finish() {
            Ok(outcome) => {
                let mut observations = finalized_observations(&outcome);
                let mut result = recording_outcome_result(&outcome);
                if self.receipt_state.result.set(outcome).is_err() {
                    observations.failures_total = observations.failures_total.saturating_add(1);
                    result = Err(EndpointFailure::new(
                        EndpointFailureStage::JoinFinalize,
                        "multistem recording receipt was already finalized",
                    ));
                }
                EndpointDriverFinalization {
                    observations,
                    result,
                }
            }
            Err(error) => EndpointDriverFinalization {
                observations: EndpointDriverObservations {
                    failures_total: live_observations.failures_total.saturating_add(1),
                    ..live_observations
                },
                result: Err(EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    error.to_string(),
                )),
            },
        }
    }
}

fn endpoint_observations(observations: RecordingObservations) -> EndpointDriverObservations {
    EndpointDriverObservations {
        frames_received_total: observations.frames_received_total,
        frames_delivered_total: observations.frames_written_total,
        frames_dropped_total: observations.frames_rejected_total,
        discontinuities_total: observations.discontinuities_total,
        failures_total: observations.failures_total,
    }
}

fn finalized_observations(outcome: &RecordingOutcome) -> EndpointDriverObservations {
    outcome.stems.iter().fold(
        EndpointDriverObservations::default(),
        |mut observations, stem| {
            observations.frames_received_total = observations
                .frames_received_total
                .saturating_add(stem.edge_observations.frames_delivered_total);
            observations.frames_delivered_total = observations
                .frames_delivered_total
                .saturating_add(stem.written_frames);
            observations.frames_dropped_total = observations
                .frames_dropped_total
                .saturating_add(stem.edge_observations.frames_dropped_total)
                .saturating_add(stem.stale_frames);
            observations.discontinuities_total = observations
                .discontinuities_total
                .saturating_add(stem.gap_ranges.len() as u64);
            if stem.error.is_some() {
                observations.failures_total = observations.failures_total.saturating_add(1);
            }
            observations
        },
    )
}

fn recording_outcome_result(outcome: &RecordingOutcome) -> Result<(), EndpointFailure> {
    if outcome.state == RecordingState::Complete && outcome.failed_stems == 0 {
        return Ok(());
    }
    let failures = outcome
        .stems
        .iter()
        .filter_map(|stem| stem.error.as_deref())
        .collect::<Vec<_>>()
        .join("; ");
    Err(EndpointFailure::new(
        EndpointFailureStage::JoinFinalize,
        if failures.is_empty() {
            "multistem recording finalized incomplete".to_owned()
        } else {
            failures
        },
    ))
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::thread;
    use std::time::Duration;

    use crate::endpoint::{
        endpoint_start_gate, EndpointDriverRegistry, EndpointPrepareContext, EndpointRouteContext,
        OperatorId, SessionTimelineOrigin,
    };
    use crate::frame::{
        AudioBufferPool, AudioFrame, ClockDomainId, FrameLineage, LineagedAudioFrame, RouteId,
        SampleFormat, SampleSpec, SourceId, StemId, StreamId,
    };
    use crate::graph::compiler::Compiler;
    use crate::graph::dsl::Pipeline;
    use crate::graph::node::{NodeConfig, PrepareContext};
    use crate::graph::planner::RuntimePlanner;
    use crate::graph::register_builtins;
    use crate::graph::registry::NodeRegistry;
    use crate::graph::spec::{EdgeId, NodeId};
    use crate::graph::NodeTypeId;
    use crate::runtime::{PlanEdgeFrame, PlanEdgeReceiver, PlanEdgeRouter};
    use crate::timing::TimelineMapping;
    use tempfile::TempDir;

    use super::*;
    use crate::recording::{PermissionDecision, PermissionScope, StemLabel};

    const SESSION_ID: SessionId = SessionId(42);
    const GROUP_ID: &str = "session.multistem.default.v1";
    const OPERATOR_ID: &str = "io.pocketstation.recording.wav-stems.v1";
    const NODE_TYPE_ID: &str = "endpoint.recording.multistem";
    const FRAME_SAMPLES: usize = 960;

    fn stem(endpoint_id: u64, source_id: u64, stem_id: u64, label: &str) -> MultistemEndpointStem {
        MultistemEndpointStem {
            endpoint_id: EndpointId(endpoint_id),
            recorder: RecorderStemConfig {
                session_id: SESSION_ID,
                source_id: SourceId(source_id),
                stem_id: StemId(stem_id),
                clock_id: ClockDomainId(source_id as u32),
                source_generation: 1,
                permission_epoch: 2,
                permission_scope: PermissionScope::SessionCaptureGrant,
                permission: PermissionDecision::Allowed,
                label: StemLabel::new(label).unwrap(),
                sample_rate_hz: 48_000,
                channels: 1,
                timeline_mapping: TimelineMapping::new(0, 0),
            },
        }
    }

    fn input(
        receiver: PlanEdgeReceiver,
        endpoint_id: EndpointId,
        label: &str,
    ) -> EndpointDriverInput {
        EndpointDriverInput::new(
            receiver,
            EndpointPrepareContext::new(
                SESSION_ID,
                endpoint_id,
                NodeConfig::new()
                    .with("stem_name", label)
                    .with("recording_group_id", GROUP_ID),
                PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved)),
            ),
        )
    }

    fn session_input(
        receiver: PlanEdgeReceiver,
        endpoint_id: EndpointId,
        stem_id: StemId,
        route_id: RouteId,
        label: &str,
        timeline_origin_ns: u64,
    ) -> EndpointDriverInput {
        EndpointDriverInput::new(
            receiver,
            EndpointPrepareContext::new(
                SESSION_ID,
                endpoint_id,
                NodeConfig::new()
                    .with("stem_name", label)
                    .with("recording_group_id", GROUP_ID),
                PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved)),
            )
            .with_session_route(
                EndpointRouteContext::new(stem_id, route_id),
                SessionTimelineOrigin::from_monotonic_timestamp_ns(timeline_origin_ns),
            ),
        )
    }

    fn lineaged_frame(
        source_id: u64,
        stem_id: u64,
        sequence_number: u64,
        value: f32,
    ) -> LineagedAudioFrame {
        let pool = AudioBufferPool::new(1, FRAME_SAMPLES);
        let mut buffer = pool.acquire().unwrap();
        buffer.copy_from_slice(&vec![value; FRAME_SAMPLES]);
        let timestamp_ns = sequence_number.saturating_mul(20_000_000);
        LineagedAudioFrame::new(
            AudioFrame::new(
                StreamId(source_id),
                SourceId(source_id),
                sequence_number,
                timestamp_ns,
                1,
                buffer,
            ),
            FrameLineage {
                session_id: SESSION_ID,
                source_id: SourceId(source_id),
                stem_id: StemId(stem_id),
                clock_id: ClockDomainId(source_id as u32),
                sequence_num: sequence_number,
                timestamp_start_ns: timestamp_ns,
                duration_ns: 20_000_000,
                source_generation: 1,
                discontinuity_epoch: 0,
                permission_epoch: 2,
            },
        )
        .unwrap()
    }

    fn lineaged_frame_with_permission(
        source_id: u64,
        stem_id: u64,
        sequence_number: u64,
        permission_epoch: u64,
        value: f32,
    ) -> LineagedAudioFrame {
        let pool = AudioBufferPool::new(1, FRAME_SAMPLES);
        let mut buffer = pool.acquire().unwrap();
        buffer.copy_from_slice(&vec![value; FRAME_SAMPLES]);
        let timestamp_ns = sequence_number.saturating_mul(20_000_000);
        LineagedAudioFrame::new(
            AudioFrame::new(
                StreamId(source_id),
                SourceId(source_id),
                sequence_number,
                timestamp_ns,
                1,
                buffer,
            ),
            FrameLineage {
                session_id: SESSION_ID,
                source_id: SourceId(source_id),
                stem_id: StemId(stem_id),
                clock_id: ClockDomainId(source_id as u32),
                sequence_num: sequence_number,
                timestamp_start_ns: timestamp_ns,
                duration_ns: 20_000_000,
                source_generation: 7,
                discontinuity_epoch: 0,
                permission_epoch,
            },
        )
        .unwrap()
    }

    fn raw_frame(source_id: u64, sequence_number: u64, value: f32) -> AudioFrame {
        let pool = AudioBufferPool::new(1, FRAME_SAMPLES);
        let mut buffer = pool.acquire().unwrap();
        buffer.copy_from_slice(&vec![value; FRAME_SAMPLES]);
        AudioFrame::new(
            StreamId(source_id),
            SourceId(source_id),
            sequence_number,
            sequence_number.saturating_mul(20_000_000),
            1,
            buffer,
        )
    }

    fn router_with_sources(
        source_count: usize,
    ) -> (
        PlanEdgeRouter,
        Vec<PlanEdgeReceiver>,
        Vec<NodeId>,
        Vec<EdgeId>,
    ) {
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry).unwrap();
        let mut graph = Pipeline::new();
        let mut source_ids = Vec::with_capacity(source_count);
        for _ in 0..source_count {
            let source = graph.add_node("passthrough", NodeConfig::new());
            let sink = graph.add_node("passthrough", NodeConfig::new());
            graph.connect(source.out("out"), sink.in_("in"));
            source_ids.push(source.id());
        }
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (router, receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let edge_ids = receivers.iter().map(PlanEdgeReceiver::edge_id).collect();
        (router, receivers, source_ids, edge_ids)
    }

    fn endpoint_registry(
        coordinator: MultistemEndpointCoordinator,
    ) -> (EndpointDriverRegistry, OperatorId, NodeTypeId) {
        let operator_id = OperatorId::new(OPERATOR_ID);
        let node_type_id = NodeTypeId::from(NODE_TYPE_ID);
        let mut registry = EndpointDriverRegistry::new();
        registry
            .register(
                operator_id.clone(),
                node_type_id.clone(),
                Arc::new(coordinator),
            )
            .unwrap();
        (registry, operator_id, node_type_id)
    }

    fn session_endpoint_registry(
        coordinator: SessionMultistemEndpointCoordinator,
    ) -> (EndpointDriverRegistry, OperatorId, NodeTypeId) {
        let operator_id = OperatorId::new(OPERATOR_ID);
        let node_type_id = NodeTypeId::from(NODE_TYPE_ID);
        let mut registry = EndpointDriverRegistry::new();
        registry
            .register(
                operator_id.clone(),
                node_type_id.clone(),
                Arc::new(coordinator),
            )
            .unwrap();
        (registry, operator_id, node_type_id)
    }

    fn wait_for_received(running: &crate::endpoint::RunningEndpoint, expected_frames: u64) {
        for _ in 0..200 {
            if running.observations().frames_received_total >= expected_frames {
                return;
            }
            thread::sleep(Duration::from_millis(1));
        }
        panic!("recording endpoint did not receive {expected_frames} frames");
    }

    fn wait_for_failure(running: &crate::endpoint::RunningEndpoint) {
        for _ in 0..200 {
            if running.observations().failures_total > 0 {
                return;
            }
            thread::sleep(Duration::from_millis(1));
        }
        panic!("recording endpoint did not report its worker failure");
    }

    #[test]
    fn given_two_stem_batch_when_gate_opens_then_one_session_manifest_finalizes_both_stems() {
        let temp_dir = TempDir::new().unwrap();
        let declarations = vec![
            stem(101, 1, 11, "application"),
            stem(102, 2, 12, "microphone"),
        ];
        let coordinator = MultistemEndpointCoordinator::new(
            temp_dir.path(),
            SESSION_ID,
            EndpointGroupId::new(GROUP_ID),
            declarations,
        )
        .unwrap();
        let receipt = coordinator.receipt();
        let (registry, operator_id, node_type_id) = endpoint_registry(coordinator);
        let (mut router, mut receivers, source_nodes, edge_ids) = router_with_sources(2);
        let prepared = registry
            .prepare_batch(
                &operator_id,
                &node_type_id,
                vec![
                    input(receivers.remove(0), EndpointId(101), "application"),
                    input(receivers.remove(0), EndpointId(102), "microphone"),
                ],
            )
            .unwrap();
        let (gate_controller, gate) = endpoint_start_gate();
        let mut running = prepared.start(gate).unwrap();
        assert!(!temp_dir.path().join("session-42").exists());
        assert!(!temp_dir.path().join("session-42/manifest.json").exists());

        router.dispatch_lineaged_from(source_nodes[0], "out", lineaged_frame(1, 11, 0, 0.25), 1);
        router.dispatch_lineaged_from(source_nodes[1], "out", lineaged_frame(2, 12, 0, -0.5), 1);
        thread::sleep(Duration::from_millis(5));
        assert_eq!(running.observations().frames_received_total, 0);
        assert!(edge_ids.iter().all(|edge_id| {
            router
                .observations(*edge_id)
                .is_some_and(|observations| observations.queue_depth_frames == 1)
        }));

        gate_controller.open();
        wait_for_received(&running, 2);
        running.request_stop();
        let finalization = running.join_and_finalize();

        assert!(finalization.is_success());
        assert_eq!(finalization.observations.frames_delivered_total, 2);
        let receipt_result = receipt
            .result()
            .expect("finalized recording receipt must remain available");
        assert_eq!(receipt_result.state, RecordingState::Complete);
        assert_eq!(receipt_result.stems.len(), 2);
        assert_eq!(receipt_result.stems[0].written_frames, 1);
        assert_eq!(receipt_result.stems[1].written_frames, 1);
        let session_dir = temp_dir.path().join("session-42");
        let manifest: serde_json::Value =
            serde_json::from_reader(File::open(session_dir.join("manifest.json")).unwrap())
                .unwrap();
        assert_eq!(manifest["state"], "complete");
        assert_eq!(manifest["stems"].as_array().unwrap().len(), 2);
        assert_eq!(
            fs::read_dir(temp_dir.path())
                .unwrap()
                .filter_map(Result::ok)
                .count(),
            1
        );
    }

    #[test]
    fn given_partial_recording_batch_when_prepared_then_all_inputs_roll_back_without_artifacts() {
        let temp_dir = TempDir::new().unwrap();
        let coordinator = MultistemEndpointCoordinator::new(
            temp_dir.path(),
            SESSION_ID,
            EndpointGroupId::new(GROUP_ID),
            vec![
                stem(101, 1, 11, "application"),
                stem(102, 2, 12, "microphone"),
            ],
        )
        .unwrap();
        let (registry, operator_id, node_type_id) = endpoint_registry(coordinator);
        let (_router, mut receivers, _source_nodes, _edge_ids) = router_with_sources(1);

        let result = registry.prepare_batch(
            &operator_id,
            &node_type_id,
            vec![input(
                receivers.pop().unwrap(),
                EndpointId(101),
                "application",
            )],
        );

        assert!(matches!(
            result,
            Err(crate::endpoint::EndpointPrepareError::Driver(_))
        ));
        assert!(!temp_dir.path().join("session-42").exists());
    }

    #[test]
    fn given_ready_recording_group_when_startup_rolls_back_before_gate_then_pending_artifacts_are_removed(
    ) {
        let temp_dir = TempDir::new().unwrap();
        let coordinator = MultistemEndpointCoordinator::new(
            temp_dir.path(),
            SESSION_ID,
            EndpointGroupId::new(GROUP_ID),
            vec![
                stem(101, 1, 11, "application"),
                stem(102, 2, 12, "microphone"),
            ],
        )
        .unwrap();
        let (registry, operator_id, node_type_id) = endpoint_registry(coordinator);
        let (_router, mut receivers, _source_nodes, _edge_ids) = router_with_sources(2);
        let prepared = registry
            .prepare_batch(
                &operator_id,
                &node_type_id,
                vec![
                    input(receivers.remove(0), EndpointId(101), "application"),
                    input(receivers.remove(0), EndpointId(102), "microphone"),
                ],
            )
            .unwrap();
        let (_gate_controller, gate) = endpoint_start_gate();
        let mut running = prepared.start(gate).unwrap();
        assert!(!temp_dir.path().join("session-42").exists());

        running.request_stop();
        let outcome = running.join_and_finalize();

        assert!(outcome.is_success());
        assert_eq!(outcome.observations, EndpointDriverObservations::default());
        assert!(!temp_dir.path().join("session-42").exists());
        assert!(!temp_dir.path().join(".session-42.pending").exists());
    }

    #[test]
    fn given_recorder_worker_failure_when_finalized_then_manifest_and_typed_outcome_remain_failed()
    {
        let temp_dir = TempDir::new().unwrap();
        let coordinator = MultistemEndpointCoordinator::new(
            temp_dir.path(),
            SESSION_ID,
            EndpointGroupId::new(GROUP_ID),
            vec![stem(101, 99, 11, "application")],
        )
        .unwrap();
        let (registry, operator_id, node_type_id) = endpoint_registry(coordinator);
        let (mut router, mut receivers, source_nodes, _edge_ids) = router_with_sources(1);
        let prepared = registry
            .prepare_batch(
                &operator_id,
                &node_type_id,
                vec![input(
                    receivers.pop().unwrap(),
                    EndpointId(101),
                    "application",
                )],
            )
            .unwrap();
        let (gate_controller, gate) = endpoint_start_gate();
        let mut running = prepared.start(gate).unwrap();
        gate_controller.open();

        router.dispatch_lineaged_from(source_nodes[0], "out", lineaged_frame(1, 11, 0, 0.25), 1);
        wait_for_received(&running, 1);
        running.request_stop();
        let finalization = running.join_and_finalize();

        assert!(!finalization.is_success());
        assert_eq!(finalization.observations.frames_received_total, 1);
        assert_eq!(finalization.observations.failures_total, 1);
        assert_eq!(
            finalization.join_finalize_result.unwrap_err().stage(),
            EndpointFailureStage::JoinFinalize
        );
        let manifest: serde_json::Value = serde_json::from_reader(
            File::open(temp_dir.path().join("session-42/manifest.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(manifest["state"], "incomplete");
    }

    #[test]
    fn given_failed_recording_branch_when_more_frames_dispatch_then_healthy_branch_remains_independent(
    ) {
        let temp_dir = TempDir::new().unwrap();
        let mut node_registry = NodeRegistry::new();
        register_builtins(&mut node_registry).unwrap();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let recorder_sink = graph.add_node("passthrough", NodeConfig::new());
        let healthy_sink = graph.add_node("passthrough", NodeConfig::new());
        let recorder_edge = graph.connect(source.out("out"), recorder_sink.in_("in"));
        let healthy_edge = graph.connect(source.out("out"), healthy_sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &node_registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut router, mut receivers) = PlanEdgeRouter::new(&plan, &ir).unwrap();
        let recorder_index = receivers
            .iter()
            .position(|receiver| receiver.edge_id() == recorder_edge)
            .unwrap();
        let recorder_receiver = receivers.swap_remove(recorder_index);
        let healthy_receiver = receivers
            .iter_mut()
            .find(|receiver| receiver.edge_id() == healthy_edge)
            .unwrap();
        let coordinator = MultistemEndpointCoordinator::new(
            temp_dir.path(),
            SESSION_ID,
            EndpointGroupId::new(GROUP_ID),
            vec![stem(101, 99, 11, "application")],
        )
        .unwrap();
        let (registry, operator_id, node_type_id) = endpoint_registry(coordinator);
        let prepared = registry
            .prepare_batch(
                &operator_id,
                &node_type_id,
                vec![input(recorder_receiver, EndpointId(101), "application")],
            )
            .unwrap();
        let (gate_controller, gate) = endpoint_start_gate();
        let running = prepared.start(gate).unwrap();
        gate_controller.open();

        router.dispatch_lineaged_from(source.id(), "out", lineaged_frame(1, 11, 0, 0.25), 1);
        assert!(matches!(
            healthy_receiver.try_recv(),
            Some(PlanEdgeFrame::LineagedExclusive(_)) | Some(PlanEdgeFrame::LineagedShared(_))
        ));
        wait_for_received(&running, 1);
        router.dispatch_lineaged_from(
            source.id(),
            "out",
            lineaged_frame(1, 11, 1, 0.5),
            20_000_001,
        );

        assert_eq!(healthy_receiver.try_recv().unwrap().sequence_number(), 1);
        assert_eq!(
            router
                .observations(healthy_edge)
                .unwrap()
                .frames_dropped_total,
            0
        );
        assert!(!running.join_and_finalize().is_success());
    }

    #[test]
    fn given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin(
    ) {
        let temp_dir = TempDir::new().unwrap();
        let coordinator = SessionMultistemEndpointCoordinator::new(
            temp_dir.path(),
            EndpointGroupId::new(GROUP_ID),
        );
        let receipt = coordinator.receipt();
        let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
        let (mut router, mut receivers, source_nodes, _edge_ids) = router_with_sources(2);
        let prepared = registry
            .prepare_batch(
                &operator_id,
                &node_type_id,
                vec![
                    session_input(
                        receivers.remove(0),
                        EndpointId(101),
                        StemId(11),
                        RouteId(21),
                        "application",
                        0,
                    ),
                    session_input(
                        receivers.remove(0),
                        EndpointId(102),
                        StemId(12),
                        RouteId(22),
                        "microphone",
                        0,
                    ),
                ],
            )
            .unwrap();
        let (gate_controller, gate) = endpoint_start_gate();
        let mut running = prepared.start(gate).unwrap();
        gate_controller.open();

        router.dispatch_lineaged_from(
            source_nodes[0],
            "out",
            lineaged_frame_with_permission(31, 11, 0, 4, 0.25),
            1,
        );
        router.dispatch_lineaged_from(
            source_nodes[1],
            "out",
            lineaged_frame_with_permission(32, 12, 0, 5, -0.5),
            1,
        );
        wait_for_received(&running, 2);
        running.request_stop();
        let finalization = running.join_and_finalize();

        assert!(finalization.is_success());
        let outcome = receipt.result().expect("recording receipt must finalize");
        assert_eq!(outcome.state, RecordingState::Complete);
        assert_eq!(outcome.completed_stems, 2);
        let manifest: serde_json::Value =
            serde_json::from_reader(File::open(outcome.session_dir.join("manifest.json")).unwrap())
                .unwrap();
        let stems = manifest["stems"].as_array().unwrap();
        assert_eq!(stems[0]["source_id"], 31);
        assert_eq!(stems[0]["stem_id"], 11);
        assert_eq!(stems[0]["clock_id"], 31);
        assert_eq!(stems[0]["source_generation"], 7);
        assert_eq!(stems[0]["permission_epoch"], 4);
        assert_eq!(stems[0]["source_timeline_origin_ns"], 0);
        assert_eq!(stems[0]["session_timeline_origin_ns"], 0);
        assert_eq!(stems[1]["source_id"], 32);
        assert_eq!(stems[1]["stem_id"], 12);
        assert_eq!(stems[1]["permission_epoch"], 5);
        assert_eq!(stems[1]["source_timeline_origin_ns"], 0);
        assert_eq!(stems[1]["session_timeline_origin_ns"], 0);
    }

    #[test]
    fn given_session_recorder_when_first_frame_is_raw_then_it_fails_closed_without_artifact() {
        let temp_dir = TempDir::new().unwrap();
        let coordinator = SessionMultistemEndpointCoordinator::new(
            temp_dir.path(),
            EndpointGroupId::new(GROUP_ID),
        );
        let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
        let (mut router, mut receivers, source_nodes, _edge_ids) = router_with_sources(1);
        let prepared = registry
            .prepare_batch(
                &operator_id,
                &node_type_id,
                vec![session_input(
                    receivers.pop().unwrap(),
                    EndpointId(101),
                    StemId(11),
                    RouteId(21),
                    "application",
                    0,
                )],
            )
            .unwrap();
        let (gate_controller, gate) = endpoint_start_gate();
        let running = prepared.start(gate).unwrap();
        gate_controller.open();

        router.dispatch_from(source_nodes[0], "out", raw_frame(31, 0, 0.25), 1);
        wait_for_failure(&running);
        let finalization = running.join_and_finalize();

        assert!(!finalization.is_success());
        assert_eq!(finalization.observations.frames_received_total, 1);
        assert_eq!(finalization.observations.failures_total, 1);
        assert!(finalization
            .join_finalize_result
            .unwrap_err()
            .message()
            .contains("raw frame"));
        assert!(!temp_dir.path().join("session-42").exists());
    }

    #[test]
    fn given_session_recorder_input_without_typed_route_when_prepared_then_it_is_rejected() {
        let temp_dir = TempDir::new().unwrap();
        let coordinator = SessionMultistemEndpointCoordinator::new(
            temp_dir.path(),
            EndpointGroupId::new(GROUP_ID),
        );
        let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
        let (_router, mut receivers, _source_nodes, _edge_ids) = router_with_sources(1);

        let error = match registry.prepare_batch(
            &operator_id,
            &node_type_id,
            vec![input(
                receivers.pop().unwrap(),
                EndpointId(101),
                "application",
            )],
        ) {
            Ok(_) => panic!("missing typed route context must fail preparation"),
            Err(error) => error,
        };

        assert!(error
            .to_string()
            .contains("missing typed Session route context"));
        assert!(!temp_dir.path().join("session-42").exists());
    }

    #[test]
    fn given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed() {
        let temp_dir = TempDir::new().unwrap();
        let coordinator = SessionMultistemEndpointCoordinator::new(
            temp_dir.path(),
            EndpointGroupId::new(GROUP_ID),
        );
        let receipt = coordinator.receipt();
        let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
        let (mut router, mut receivers, source_nodes, _edge_ids) = router_with_sources(1);
        let prepared = registry
            .prepare_batch(
                &operator_id,
                &node_type_id,
                vec![session_input(
                    receivers.pop().unwrap(),
                    EndpointId(101),
                    StemId(11),
                    RouteId(21),
                    "application",
                    0,
                )],
            )
            .unwrap();
        let (gate_controller, gate) = endpoint_start_gate();
        let mut running = prepared.start(gate).unwrap();
        gate_controller.open();

        router.dispatch_lineaged_from(
            source_nodes[0],
            "out",
            lineaged_frame_with_permission(31, 11, 0, 4, 0.25),
            1,
        );
        wait_for_received(&running, 1);
        router.dispatch_lineaged_from(
            source_nodes[0],
            "out",
            lineaged_frame_with_permission(31, 11, 1, 5, 0.5),
            20_000_001,
        );
        wait_for_failure(&running);
        running.request_stop();
        let finalization = running.join_and_finalize();

        assert!(!finalization.is_success());
        assert_eq!(finalization.observations.frames_received_total, 2);
        assert_eq!(finalization.observations.failures_total, 1);
        let outcome = receipt
            .result()
            .expect("failed recording receipt must finalize");
        assert_eq!(outcome.state, RecordingState::Incomplete);
        assert!(outcome.stems[0]
            .error
            .as_deref()
            .is_some_and(|error| error.contains("PermissionEpoch")));
    }
}

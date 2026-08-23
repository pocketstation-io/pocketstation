use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::endpoint::{
    EndpointAudioReceiver, EndpointCancellationOutcome, EndpointDriverFactory,
    EndpointDriverFinalization, EndpointDriverObservations, EndpointFailure, EndpointFailureStage,
    EndpointGroupId, EndpointPortInput, EndpointPreparationGroup, EndpointReceiver,
    EndpointStartGate, PreparedEndpointDriver, RunningEndpointDriver, SessionTimelineOrigin,
};
use crate::frame::{EndpointId, RouteId, SessionId, StemId};
use crate::runtime::PlanEdgeFrame;
use crate::timing::TimelineMapping;

use crate::recording::{
    MultistemRecording, PermissionDecision, PermissionScope, RecorderError, RecorderLineageField,
    RecorderStemConfig, RecordingObservations, RecordingOutcome, RecordingState, StemLabel,
};

const SESSION_RECORDER_IDLE_WAIT_MS: u64 = 1;
#[doc = "Defines the public multistem group configuration key value."]
pub const MULTISTEM_GROUP_CONFIGURATION_KEY: &str = "recording_group_id";
#[doc = "Defines the public multistem name configuration key value."]
pub const MULTISTEM_NAME_CONFIGURATION_KEY: &str = "stem_name";

#[derive(Clone)]
#[doc = "Retains the identity and observation access returned for multistem recording."]
pub struct MultistemRecordingReceipt {
    state: Arc<MultistemRecordingReceiptState>,
}

impl MultistemRecordingReceipt {
    #[doc = "Returns the result represented by `MultistemRecordingReceipt`."]
    pub fn result(&self) -> Option<&RecordingOutcome> {
        self.state.result.get()
    }
}

#[derive(Default)]
struct MultistemRecordingReceiptState {
    result: OnceLock<RecordingOutcome>,
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
    #[doc = "Creates a new `SessionMultistemEndpointCoordinator`."]
    pub fn new(output_root: impl Into<PathBuf>, group_id: EndpointGroupId) -> Self {
        Self {
            output_root: output_root.into(),
            group_id,
            receipt_state: Arc::new(MultistemRecordingReceiptState::default()),
        }
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the output root held by `SessionMultistemEndpointCoordinator`."]
    pub fn output_root(&self) -> &Path {
        &self.output_root
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the group identifier held by `SessionMultistemEndpointCoordinator`."]
    pub fn group_id(&self) -> &EndpointGroupId {
        &self.group_id
    }

    #[doc = "Returns the receipt held by `SessionMultistemEndpointCoordinator`."]
    pub fn receipt(&self) -> MultistemRecordingReceipt {
        MultistemRecordingReceipt {
            state: Arc::clone(&self.receipt_state),
        }
    }
}

impl EndpointDriverFactory for SessionMultistemEndpointCoordinator {
    #[doc = "Returns the preparation group held by `SessionMultistemEndpointCoordinator`."]
    fn preparation_group(
        &self,
        _route_id: RouteId,
        configuration: &crate::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, EndpointFailure> {
        let declared_group_id = configuration
            .get(MULTISTEM_GROUP_CONFIGURATION_KEY)
            .ok_or_else(|| prepare_failure("recording endpoint is missing recording_group_id"))?;
        if declared_group_id != self.group_id.as_str() {
            return Err(prepare_failure(format!(
                "recording endpoint belongs to group {declared_group_id:?}, expected {:?}",
                self.group_id.as_str()
            )));
        }
        Ok(EndpointPreparationGroup::Shared(self.group_id.clone()))
    }

    #[doc = "Prepares resources required by `SessionMultistemEndpointCoordinator`."]
    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
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
            let route_context = context.route_context();
            let stem_id = route_context.audio_stem_id().ok_or_else(|| {
                prepare_failure(format!(
                    "endpoint {:?} is not bound to an audio stem",
                    context.endpoint_id()
                ))
            })?;
            if !stem_ids.insert(stem_id) {
                return Err(prepare_failure(format!(
                    "recording group contains duplicate stem {:?}",
                    stem_id
                )));
            }
            if !route_ids.insert(route_context.route_id()) {
                return Err(prepare_failure(format!(
                    "recording group contains duplicate route {:?}",
                    route_context.route_id()
                )));
            }
            let timeline_origin = context.session_timeline_origin();
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
                .get(MULTISTEM_GROUP_CONFIGURATION_KEY)
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
                    .get(MULTISTEM_NAME_CONFIGURATION_KEY)
                    .ok_or_else(|| prepare_failure("recording endpoint is missing stem_name"))?,
            )
            .map_err(|error| prepare_failure(error.to_string()))?;
            if !labels.insert(label.clone()) {
                return Err(prepare_failure(format!(
                    "recording group contains duplicate stem label {:?}",
                    label.as_str()
                )));
            }
            let EndpointReceiver::Audio {
                receiver,
                sample_spec,
            } = receiver
            else {
                return Err(prepare_failure(
                    "multistem recording accepts only realtime audio inputs",
                ));
            };
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
                stem_id,
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
            identity: RecordingSessionIdentity {
                session_id,
                group_id: self.group_id.clone(),
            },
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
    receiver: EndpointAudioReceiver,
}

#[derive(Clone)]
struct RecordingSessionIdentity {
    session_id: SessionId,
    group_id: EndpointGroupId,
}

struct PreparedSessionMultistemEndpoint {
    output_root: PathBuf,
    identity: RecordingSessionIdentity,
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
                .join(format!("session-{}", self.identity.session_id.0)),
            self.output_root
                .join(format!(".session-{}.pending", self.identity.session_id.0)),
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
            self.identity,
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
        identity: RecordingSessionIdentity,
        timeline_origin: SessionTimelineOrigin,
        stems: Vec<SessionPreparedStem>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Self, std::io::Error> {
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop_requested);
        let telemetry = Arc::new(SessionRecorderTelemetry::default());
        let worker_telemetry = Arc::clone(&telemetry);
        let join_handle = thread::Builder::new()
            .name(format!(
                "pocketstation-session-recorder-{}",
                identity.session_id.0
            ))
            .spawn(move || {
                run_session_recorder(
                    &output_root,
                    identity,
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
    identity: RecordingSessionIdentity,
    timeline_origin: SessionTimelineOrigin,
    mut stems: Vec<SessionPreparedStem>,
    start_gate: &EndpointStartGate,
    stop_requested: &AtomicBool,
    telemetry: &SessionRecorderTelemetry,
) -> SessionRecorderWorkerOutcome {
    let session_id = identity.session_id;
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
                let frame = frame.into_inner();
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
        recorder_stems.push((config, stem.receiver.into_inner(), first_frame));
    }
    let recording = match MultistemRecording::start_observed(
        output_root,
        session_id,
        identity.group_id,
        recorder_stems,
    ) {
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
    let lineage = frame.lineage();
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
    let lineage = frame.lineage();
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
mod tests;

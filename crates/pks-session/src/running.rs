use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use pks_capture::{
    prepare_capture, CallbackCaptureBackend, CaptureError, CaptureLineageSeed, CaptureMode,
    CapturePrepareRequest, CaptureStopOutcome, InputDeviceSelector, SourceRuntimeEventReceive,
};
use pks_endpoint::{
    endpoint_start_gate, EndpointDriverInput, EndpointDriverRegistry, EndpointFailure,
    EndpointFinalizationOutcome, EndpointPrepareContext, EndpointPrepareError,
    EndpointStartFailure, PreparedEndpoint, RunningEndpoint,
};
use pks_frame::{EndpointId, RouteId, SessionId, StemId};
use pks_graph::PrepareContext;
use pks_runtime::{
    PlanRunnerDrainPolicy, PlanRunnerError, PlanRunnerFinishSummary, PlanSourceSendOutcome,
    RealtimePlanRunner,
};

use crate::compiler::endpoint_node_config;
use crate::events::{session_event_channel, SessionEventSender};
use crate::{
    ApplicationSelector, DeviceSelector, PreparedSession, PreparedWorkerMapping,
    SessionComponentId, SessionControlFailure, SessionEventReceiver, SessionFinalizationFailure,
    SessionFinalizationStage, SessionLifecycleState, SessionRollbackFailure, SessionRollbackStage,
    SessionSourceFailure, SessionTerminalOutcome, Source, RECORDING_GROUP_CONFIGURATION_KEY,
};

pub struct CaptureBackendSet<'backend> {
    pub application: &'backend dyn CallbackCaptureBackend,
    pub microphone: &'backend dyn CallbackCaptureBackend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionStartOptions {
    pub capture_frame_capacity_frames: usize,
    pub capture_runtime_event_capacity_events: usize,
    pub runtime_work_budget_frames: usize,
    pub runtime_idle_poll_ms: u64,
    pub runtime_ready_timeout_ms: u64,
    pub session_event_capacity_events: usize,
}

impl Default for SessionStartOptions {
    fn default() -> Self {
        Self {
            capture_frame_capacity_frames: 32,
            capture_runtime_event_capacity_events: 8,
            runtime_work_budget_frames: 64,
            runtime_idle_poll_ms: 1,
            runtime_ready_timeout_ms: 1_000,
            session_event_capacity_events: 32,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionStartError {
    #[error("invalid Session start options: {reason}")]
    InvalidOptions { reason: &'static str },
    #[error("Session requires exactly one application and one microphone source")]
    UnsupportedSourceTopology,
    #[error("endpoint {endpoint_id:?} declaration is absent")]
    MissingEndpointDeclaration { endpoint_id: pks_frame::EndpointId },
    #[error("endpoint preparation failed: {source}")]
    EndpointPrepare {
        #[source]
        source: EndpointPrepareError,
        rollback_failures_total: u64,
    },
    #[error("capture preparation failed for stem {stem_id:?}: {source}")]
    CapturePrepare {
        stem_id: StemId,
        #[source]
        source: CaptureError,
        rollback_failures_total: u64,
    },
    #[error("capture open failed for stem {stem_id:?}: {source}")]
    CaptureOpen {
        stem_id: StemId,
        #[source]
        source: CaptureError,
        rollback_failures_total: u64,
    },
    #[error("endpoint start failed: {source}")]
    EndpointStart {
        #[source]
        source: EndpointStartFailure,
        rollback_failures_total: u64,
    },
    #[error("runtime runner preparation failed: {source}")]
    RuntimeRunner {
        #[source]
        source: PlanRunnerError,
        rollback_failures_total: u64,
    },
    #[error("runtime worker thread could not start: {message}")]
    RuntimeWorkerSpawn {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("runtime worker did not become ready: {message}")]
    RuntimeWorkerReady {
        message: String,
        rollback_failures_total: u64,
    },
}

impl SessionStartError {
    pub const fn rollback_failures_total(&self) -> u64 {
        match self {
            Self::EndpointPrepare {
                rollback_failures_total,
                ..
            }
            | Self::CapturePrepare {
                rollback_failures_total,
                ..
            }
            | Self::CaptureOpen {
                rollback_failures_total,
                ..
            }
            | Self::EndpointStart {
                rollback_failures_total,
                ..
            }
            | Self::RuntimeWorkerSpawn {
                rollback_failures_total,
                ..
            }
            | Self::RuntimeRunner {
                rollback_failures_total,
                ..
            }
            | Self::RuntimeWorkerReady {
                rollback_failures_total,
                ..
            } => *rollback_failures_total,
            Self::InvalidOptions { .. }
            | Self::UnsupportedSourceTopology
            | Self::MissingEndpointDeclaration { .. } => 0,
        }
    }
}

#[derive(Debug)]
pub struct SessionStartFailure {
    error: SessionStartError,
    event_receiver: Option<SessionEventReceiver>,
    rollback_failures: Box<[SessionRollbackFailure]>,
}

impl SessionStartFailure {
    fn input(error: SessionStartError) -> Self {
        Self {
            error,
            event_receiver: None,
            rollback_failures: Box::new([]),
        }
    }

    pub const fn error(&self) -> &SessionStartError {
        &self.error
    }

    pub fn rollback_failures(&self) -> &[SessionRollbackFailure] {
        &self.rollback_failures
    }

    pub fn take_event_receiver(&mut self) -> Option<SessionEventReceiver> {
        self.event_receiver.take()
    }

    pub fn into_error(self) -> SessionStartError {
        self.error
    }
}

impl std::fmt::Display for SessionStartFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for SessionStartFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

struct RuntimeSource {
    stem_id: StemId,
    capture: pks_capture::CaptureOwner,
    sender: pks_runtime::PlanSourceSender,
}

struct OpenedCapture {
    stem_id: StemId,
    owner: pks_capture::CaptureOwner,
}

struct RuntimeWorkerStart {
    sources: Vec<RuntimeSource>,
    runner: RealtimePlanRunner,
    options: SessionStartOptions,
    session_id: SessionId,
    event_sender: SessionEventSender,
}

#[derive(Debug)]
struct RuntimeWorkerOutcome {
    captures: Vec<(StemId, Result<CaptureStopOutcome, CaptureError>)>,
    runner: Result<PlanRunnerFinishSummary, PlanRunnerError>,
    runtime_events_total: u64,
    runtime_failures_total: u64,
    lineage_failures_total: u64,
    source_send_rejections_total: u64,
    source_failures: Vec<SessionSourceFailure>,
}

struct PreparedEndpointBinding {
    identities: Vec<(RouteId, EndpointId)>,
    endpoint: PreparedEndpoint,
}

struct RunningEndpointBinding {
    identities: Vec<(RouteId, EndpointId)>,
    endpoint: RunningEndpoint,
}

#[derive(Default)]
struct StartupRollback {
    failures: Vec<SessionRollbackFailure>,
}

impl StartupRollback {
    fn failures_total(&self) -> u64 {
        self.failures.len() as u64
    }

    fn append(&mut self, mut other: Self) {
        self.failures.append(&mut other.failures);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionStopOutcome {
    runtime_worker_panicked: bool,
    capture_finalization_failures_total: u64,
    endpoint_finalization_failures_total: u64,
    runtime_failures_total: u64,
    lineage_failures_total: u64,
    source_send_rejections_total: u64,
    runtime_events_total: u64,
}

impl SessionStopOutcome {
    pub fn is_success(&self) -> bool {
        !self.runtime_worker_panicked
            && self.capture_finalization_failures_total == 0
            && self.endpoint_finalization_failures_total == 0
            && self.runtime_failures_total == 0
            && self.lineage_failures_total == 0
            && self.source_send_rejections_total == 0
    }

    pub const fn capture_finalization_failures_total(&self) -> u64 {
        self.capture_finalization_failures_total
    }

    pub const fn endpoint_finalization_failures_total(&self) -> u64 {
        self.endpoint_finalization_failures_total
    }

    pub const fn runtime_events_total(&self) -> u64 {
        self.runtime_events_total
    }
}

pub struct RunningSession {
    session_id: SessionId,
    stop_requested: Arc<AtomicBool>,
    runtime_worker: Option<JoinHandle<Option<RuntimeWorkerOutcome>>>,
    endpoints: Vec<RunningEndpointBinding>,
    event_sender: SessionEventSender,
    event_receiver: Option<SessionEventReceiver>,
    stop_outcome: Option<SessionStopOutcome>,
}

impl RunningSession {
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn take_event_receiver(&mut self) -> Option<SessionEventReceiver> {
        self.event_receiver.take()
    }

    pub fn stop(&mut self) -> SessionStopOutcome {
        match self.stop_outcome {
            Some(outcome) => outcome,
            None => {
                let outcome = self.stop_once();
                self.stop_outcome = Some(outcome);
                outcome
            }
        }
    }

    fn stop_once(&mut self) -> SessionStopOutcome {
        let _ = self
            .event_sender
            .publish_lifecycle(self.session_id, SessionLifecycleState::Stopping);
        self.stop_requested.store(true, Ordering::Release);
        let worker = self.runtime_worker.take().map(JoinHandle::join);
        for endpoint in &mut self.endpoints {
            let _ = endpoint.endpoint.request_stop();
        }
        let endpoint_outcomes = self
            .endpoints
            .drain(..)
            .map(|binding| (binding.identities, binding.endpoint.join_and_finalize()))
            .collect::<Vec<_>>();
        let outcome = stop_outcome(&worker, &endpoint_outcomes);
        publish_terminal_events(
            self.session_id,
            &self.event_sender,
            &worker,
            &endpoint_outcomes,
            outcome,
        );
        outcome
    }
}

impl Drop for RunningSession {
    fn drop(&mut self) {
        if self.stop_outcome.is_none() {
            let _ = self.stop_once();
        }
    }
}

pub fn start_prepared_session(
    prepared: PreparedSession,
    capture_backends: CaptureBackendSet<'_>,
    endpoint_registry: &EndpointDriverRegistry,
    prepare_context: &PrepareContext,
    options: SessionStartOptions,
) -> Result<RunningSession, SessionStartFailure> {
    validate_start_options(options).map_err(SessionStartFailure::input)?;
    validate_source_topology(&prepared).map_err(SessionStartFailure::input)?;
    let PreparedSession {
        spec,
        executor,
        source_mappings,
        source_inputs,
        worker_mappings,
        cancellation,
    } = prepared;
    let session_id = spec.session_id();
    let (event_sender, event_receiver) =
        session_event_channel(options.session_event_capacity_events);
    let _ = event_sender.publish_lifecycle(session_id, SessionLifecycleState::Starting);
    let (gate_controller, start_gate) = endpoint_start_gate();

    let mut prepared_endpoints =
        match prepare_endpoints(&spec, worker_mappings, endpoint_registry, prepare_context) {
            Ok(endpoints) => endpoints,
            Err((error, rollback_failures)) => {
                return Err(complete_start_failure(
                    session_id,
                    &event_sender,
                    event_receiver,
                    error,
                    rollback_failures,
                ));
            }
        };
    let mut running_endpoints = Vec::with_capacity(prepared_endpoints.len());
    while let Some(endpoint) = prepared_endpoints.pop() {
        match endpoint.endpoint.start(Arc::clone(&start_gate)) {
            Ok(running) => running_endpoints.push(RunningEndpointBinding {
                identities: endpoint.identities,
                endpoint: running,
            }),
            Err(source) => {
                let mut rollback = rollback_running_endpoints(running_endpoints);
                rollback.append(rollback_prepared_endpoints(prepared_endpoints));
                let rollback_failures_total = rollback.failures_total();
                return Err(complete_start_failure(
                    session_id,
                    &event_sender,
                    event_receiver,
                    SessionStartError::EndpointStart {
                        source,
                        rollback_failures_total,
                    },
                    rollback.failures,
                ));
            }
        }
    }
    running_endpoints.reverse();

    let captures = match prepare_and_open_captures(&spec, capture_backends, options) {
        Ok(captures) => captures,
        Err(error) => {
            let (error, rollback_failures) = error.rollback(running_endpoints);
            return Err(complete_start_failure(
                session_id,
                &event_sender,
                event_receiver,
                error,
                rollback_failures,
            ));
        }
    };

    let runner = match RealtimePlanRunner::new(executor, source_inputs, cancellation) {
        Ok(runner) => runner,
        Err(source) => {
            let mut rollback = rollback_captures(captures);
            rollback.append(rollback_running_endpoints(running_endpoints));
            let rollback_failures_total = rollback.failures_total();
            return Err(complete_start_failure(
                session_id,
                &event_sender,
                event_receiver,
                SessionStartError::RuntimeRunner {
                    source,
                    rollback_failures_total,
                },
                rollback.failures,
            ));
        }
    };
    let runtime_sources = captures
        .into_iter()
        .zip(source_mappings)
        .map(|(capture, mapping)| RuntimeSource {
            stem_id: mapping.stem_id,
            capture: capture.owner,
            sender: mapping.sender,
        })
        .collect::<Vec<_>>();
    let stop_requested = Arc::new(AtomicBool::new(false));
    let worker_stop_requested = Arc::clone(&stop_requested);
    let (start_tx, start_rx) = std::sync::mpsc::sync_channel::<RuntimeWorkerStart>(1);
    let (ready_tx, ready_rx) = std::sync::mpsc::sync_channel::<()>(1);
    let runtime_worker = match std::thread::Builder::new()
        .name("pks-session-runtime".to_owned())
        .spawn(move || match start_rx.recv() {
            Ok(start) => {
                let _ = ready_tx.send(());
                Some(run_runtime_worker(
                    start.sources,
                    start.runner,
                    worker_stop_requested,
                    start.options,
                    start.session_id,
                    start.event_sender,
                ))
            }
            Err(_) => None,
        }) {
        Ok(worker) => worker,
        Err(error) => {
            let mut rollback = rollback_runtime_resources(runtime_sources, runner);
            rollback.append(rollback_running_endpoints(running_endpoints));
            let rollback_failures_total = rollback.failures_total();
            return Err(complete_start_failure(
                session_id,
                &event_sender,
                event_receiver,
                SessionStartError::RuntimeWorkerSpawn {
                    message: error.to_string(),
                    rollback_failures_total,
                },
                rollback.failures,
            ));
        }
    };
    let start = RuntimeWorkerStart {
        sources: runtime_sources,
        runner,
        options,
        session_id,
        event_sender: event_sender.clone(),
    };
    if let Err(error) = start_tx.send(start) {
        let RuntimeWorkerStart {
            sources, runner, ..
        } = error.0;
        let mut rollback = rollback_runtime_resources(sources, runner);
        let _ = runtime_worker.join();
        rollback.append(rollback_running_endpoints(running_endpoints));
        let rollback_failures_total = rollback.failures_total();
        return Err(complete_start_failure(
            session_id,
            &event_sender,
            event_receiver,
            SessionStartError::RuntimeWorkerReady {
                message: "runtime worker rejected its bounded Start command".to_owned(),
                rollback_failures_total,
            },
            rollback.failures,
        ));
    }
    if let Err(error) =
        ready_rx.recv_timeout(Duration::from_millis(options.runtime_ready_timeout_ms))
    {
        stop_requested.store(true, Ordering::Release);
        let mut rollback = match runtime_worker.join() {
            Ok(Some(outcome)) => rollback_worker_outcome(outcome),
            Ok(None) | Err(_) => StartupRollback {
                failures: vec![SessionRollbackFailure::new(
                    SessionRollbackStage::DiscardRuntimeQueues,
                    SessionControlFailure::new(
                        SessionComponentId::Runtime,
                        "join_runtime_worker",
                        "runtime worker did not return an outcome",
                    ),
                )],
            },
        };
        rollback.append(rollback_running_endpoints(running_endpoints));
        let rollback_failures_total = rollback.failures_total();
        return Err(complete_start_failure(
            session_id,
            &event_sender,
            event_receiver,
            SessionStartError::RuntimeWorkerReady {
                message: error.to_string(),
                rollback_failures_total,
            },
            rollback.failures,
        ));
    }

    // `gate_controller` is the sole controller returned by `endpoint_start_gate`
    // and has not been shared or opened on any preceding path.
    let _opened = gate_controller.open();
    let _ = event_sender.publish_lifecycle(session_id, SessionLifecycleState::Running);
    Ok(RunningSession {
        session_id,
        stop_requested,
        runtime_worker: Some(runtime_worker),
        endpoints: running_endpoints,
        event_sender,
        event_receiver: Some(event_receiver),
        stop_outcome: None,
    })
}

fn validate_start_options(options: SessionStartOptions) -> Result<(), SessionStartError> {
    let reason = if options.capture_frame_capacity_frames == 0 {
        Some("capture frame capacity must be greater than zero")
    } else if options.capture_runtime_event_capacity_events == 0 {
        Some("capture runtime-event capacity must be greater than zero")
    } else if options.runtime_work_budget_frames == 0 {
        Some("runtime work budget must be greater than zero")
    } else if options.runtime_idle_poll_ms == 0 {
        Some("runtime idle poll interval must be greater than zero")
    } else if options.runtime_ready_timeout_ms == 0 {
        Some("runtime ready timeout must be greater than zero")
    } else if options.session_event_capacity_events == 0 {
        Some("session event capacity must be greater than zero")
    } else {
        None
    };
    match reason {
        Some(reason) => Err(SessionStartError::InvalidOptions { reason }),
        None => Ok(()),
    }
}

fn validate_source_topology(prepared: &PreparedSession) -> Result<(), SessionStartError> {
    let application_sources = prepared
        .spec
        .stems()
        .iter()
        .filter(|stem| matches!(stem.source(), Source::Application(_)))
        .count();
    let microphone_sources = prepared
        .spec
        .stems()
        .iter()
        .filter(|stem| matches!(stem.source(), Source::Microphone(_)))
        .count();
    if application_sources == 1 && microphone_sources == 1 {
        Ok(())
    } else {
        Err(SessionStartError::UnsupportedSourceTopology)
    }
}

fn prepare_endpoints(
    spec: &crate::SessionSpec,
    mut worker_mappings: Vec<PreparedWorkerMapping>,
    endpoint_registry: &EndpointDriverRegistry,
    prepare_context: &PrepareContext,
) -> Result<Vec<PreparedEndpointBinding>, (SessionStartError, Vec<SessionRollbackFailure>)> {
    let session_id = spec.session_id();
    let mut endpoints = Vec::with_capacity(worker_mappings.len());
    while !worker_mappings.is_empty() {
        let mapping = worker_mappings.remove(0);
        let endpoint = spec
            .endpoints()
            .iter()
            .find(|endpoint| endpoint.id() == mapping.endpoint_id)
            .ok_or_else(|| {
                (
                    SessionStartError::MissingEndpointDeclaration {
                        endpoint_id: mapping.endpoint_id,
                    },
                    Vec::new(),
                )
            })?;
        let recording_group_id = endpoint
            .configuration()
            .get(RECORDING_GROUP_CONFIGURATION_KEY);
        let mut grouped_mappings = vec![mapping];
        if let Some(recording_group_id) = recording_group_id {
            let mut index = 0;
            while index < worker_mappings.len() {
                let candidate = &worker_mappings[index];
                let Some(candidate_endpoint) = spec
                    .endpoints()
                    .iter()
                    .find(|endpoint| endpoint.id() == candidate.endpoint_id)
                else {
                    return Err((
                        SessionStartError::MissingEndpointDeclaration {
                            endpoint_id: candidate.endpoint_id,
                        },
                        Vec::new(),
                    ));
                };
                let same_group = candidate_endpoint.operator_id() == endpoint.operator_id()
                    && candidate_endpoint.node_type_id() == endpoint.node_type_id()
                    && candidate_endpoint
                        .configuration()
                        .get(RECORDING_GROUP_CONFIGURATION_KEY)
                        == Some(recording_group_id);
                if same_group {
                    grouped_mappings.push(worker_mappings.remove(index));
                } else {
                    index += 1;
                }
            }
        }
        let mut identities = Vec::with_capacity(grouped_mappings.len());
        let mut inputs = Vec::with_capacity(grouped_mappings.len());
        for mapping in grouped_mappings {
            let grouped_endpoint = spec
                .endpoints()
                .iter()
                .find(|candidate| candidate.id() == mapping.endpoint_id)
                .ok_or_else(|| {
                    (
                        SessionStartError::MissingEndpointDeclaration {
                            endpoint_id: mapping.endpoint_id,
                        },
                        Vec::new(),
                    )
                })?;
            let grouped_stem = spec
                .stems()
                .iter()
                .find(|candidate| candidate.id() == mapping.stem_id)
                .ok_or_else(|| (SessionStartError::UnsupportedSourceTopology, Vec::new()))?;
            let node_configuration =
                endpoint_node_config(session_id, grouped_stem, grouped_endpoint, mapping.route_id);
            let context = EndpointPrepareContext::new(
                session_id,
                grouped_endpoint.id(),
                node_configuration,
                prepare_context.clone(),
            );
            identities.push((mapping.route_id, mapping.endpoint_id));
            inputs.push(EndpointDriverInput::new(mapping.receiver, context));
        }
        match endpoint_registry.prepare_batch(
            endpoint.operator_id(),
            endpoint.node_type_id(),
            inputs,
        ) {
            Ok(endpoint) => endpoints.push(PreparedEndpointBinding {
                identities,
                endpoint,
            }),
            Err(source) => {
                let rollback = rollback_prepared_endpoints(endpoints);
                return Err((
                    SessionStartError::EndpointPrepare {
                        source,
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                ));
            }
        }
    }
    Ok(endpoints)
}

fn prepare_and_open_captures(
    spec: &crate::SessionSpec,
    backends: CaptureBackendSet<'_>,
    options: SessionStartOptions,
) -> Result<Vec<OpenedCapture>, CaptureAcquisitionError> {
    let mut captures = Vec::with_capacity(spec.stems().len());
    for stem in spec.stems() {
        let binding = match stem.source() {
            Source::Application(_) => backends.application,
            Source::Microphone(_) => backends.microphone,
        };
        let request = CapturePrepareRequest {
            mode: capture_mode(stem.source()),
            lineage_seed: CaptureLineageSeed::new(spec.session_id(), stem.id()),
            frame_capacity_frames: options.capture_frame_capacity_frames,
            runtime_event_capacity_events: options.capture_runtime_event_capacity_events,
        };
        let prepared = match prepare_capture(binding, request) {
            Ok(prepared) => prepared,
            Err(source) => {
                return Err(CaptureAcquisitionError::Prepare {
                    stem_id: stem.id(),
                    source,
                    prior_captures: captures,
                });
            }
        };
        let capture = match prepared.open() {
            Ok(capture) => capture,
            Err(source) => {
                return Err(CaptureAcquisitionError::Open {
                    stem_id: stem.id(),
                    source,
                    prior_captures: captures,
                });
            }
        };
        captures.push(OpenedCapture {
            stem_id: stem.id(),
            owner: capture,
        });
    }
    Ok(captures)
}

enum CaptureAcquisitionError {
    Prepare {
        stem_id: StemId,
        source: CaptureError,
        prior_captures: Vec<OpenedCapture>,
    },
    Open {
        stem_id: StemId,
        source: CaptureError,
        prior_captures: Vec<OpenedCapture>,
    },
}

impl CaptureAcquisitionError {
    fn rollback(
        self,
        running_endpoints: Vec<RunningEndpointBinding>,
    ) -> (SessionStartError, Vec<SessionRollbackFailure>) {
        match self {
            Self::Prepare {
                stem_id,
                source,
                prior_captures,
            } => {
                let mut rollback = rollback_captures(prior_captures);
                rollback.append(rollback_running_endpoints(running_endpoints));
                (
                    SessionStartError::CapturePrepare {
                        stem_id,
                        source,
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                )
            }
            Self::Open {
                stem_id,
                source,
                prior_captures,
            } => {
                let mut rollback = rollback_captures(prior_captures);
                rollback.append(rollback_running_endpoints(running_endpoints));
                (
                    SessionStartError::CaptureOpen {
                        stem_id,
                        source,
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                )
            }
        }
    }
}

fn capture_mode(source: &Source) -> CaptureMode {
    match source {
        Source::Application(ApplicationSelector::BundleId(bundle_id))
        | Source::Application(ApplicationSelector::Name(bundle_id)) => {
            CaptureMode::Application(bundle_id.clone())
        }
        Source::Application(ApplicationSelector::ProcessId(process_id)) => {
            CaptureMode::Process(process_id.get())
        }
        Source::Application(ApplicationSelector::StableId(stable_id)) => {
            CaptureMode::ExactApplicationStable {
                stable_id: stable_id.clone(),
            }
        }
        Source::Microphone(DeviceSelector::Default) => {
            CaptureMode::InputDevice(InputDeviceSelector::Default)
        }
        Source::Microphone(DeviceSelector::Id(device_id)) => {
            CaptureMode::InputDevice(InputDeviceSelector::StableId(device_id.as_str().to_owned()))
        }
    }
}

fn run_runtime_worker(
    mut sources: Vec<RuntimeSource>,
    mut runner: RealtimePlanRunner,
    stop_requested: Arc<AtomicBool>,
    options: SessionStartOptions,
    session_id: SessionId,
    event_sender: SessionEventSender,
) -> RuntimeWorkerOutcome {
    let mut runtime_events_total = 0u64;
    let mut runtime_failures_total = 0u64;
    let mut lineage_failures_total = 0u64;
    let mut source_send_rejections_total = 0u64;
    let mut source_failures = Vec::new();
    'runtime: while !stop_requested.load(Ordering::Acquire) {
        let mut work_observed = false;
        for source in &mut sources {
            loop {
                match source.capture.try_next_lineaged_frame() {
                    Ok(Some(frame)) => {
                        work_observed = true;
                        if matches!(
                            source.sender.try_send(frame),
                            PlanSourceSendOutcome::Rejected { .. }
                        ) {
                            source_send_rejections_total =
                                source_send_rejections_total.saturating_add(1);
                        }
                    }
                    Ok(None) => break,
                    Err(_) => {
                        lineage_failures_total = lineage_failures_total.saturating_add(1);
                        break 'runtime;
                    }
                }
            }
            if let SourceRuntimeEventReceive::Event(event) = source.capture.try_recv_runtime_event()
            {
                runtime_events_total = runtime_events_total.saturating_add(1);
                runtime_failures_total = runtime_failures_total.saturating_add(1);
                let failure = SessionSourceFailure::new(source.stem_id, event);
                let _ = event_sender.publish_source(session_id, failure.clone());
                source_failures.push(failure);
                break 'runtime;
            }
        }
        if runner
            .process_ready(options.runtime_work_budget_frames)
            .is_err()
        {
            runtime_failures_total = runtime_failures_total.saturating_add(1);
            break;
        }
        if !work_observed {
            std::thread::sleep(Duration::from_millis(options.runtime_idle_poll_ms));
        }
    }
    let captures = sources
        .into_iter()
        .map(|source| (source.stem_id, source.capture.stop_and_join()))
        .collect();
    let runner = runner.finish(
        PlanRunnerDrainPolicy::DiscardQueued,
        options.runtime_work_budget_frames,
    );
    RuntimeWorkerOutcome {
        captures,
        runner,
        runtime_events_total,
        runtime_failures_total,
        lineage_failures_total,
        source_send_rejections_total,
        source_failures,
    }
}

fn rollback_prepared_endpoints(endpoints: Vec<PreparedEndpointBinding>) -> StartupRollback {
    let mut rollback = StartupRollback::default();
    for binding in endpoints.into_iter().rev() {
        let outcome = binding.endpoint.cancel_preparation();
        if let Err(error) = outcome.result {
            for (route_id, endpoint_id) in binding.identities {
                rollback.failures.push(SessionRollbackFailure::new(
                    SessionRollbackStage::CancelEndpointPreparation,
                    SessionControlFailure::new(
                        SessionComponentId::Endpoint {
                            route_id,
                            endpoint_id,
                        },
                        "cancel_endpoint_preparation",
                        error.to_string(),
                    ),
                ));
            }
        }
    }
    rollback
}

fn rollback_running_endpoints(endpoints: Vec<RunningEndpointBinding>) -> StartupRollback {
    let mut rollback = StartupRollback::default();
    for binding in endpoints.into_iter().rev() {
        let outcome = binding.endpoint.join_and_finalize();
        for error in [
            outcome.request_stop_result.err(),
            outcome.join_finalize_result.err(),
        ]
        .into_iter()
        .flatten()
        {
            for (route_id, endpoint_id) in &binding.identities {
                rollback.failures.push(SessionRollbackFailure::new(
                    SessionRollbackStage::FinalizeStartedEndpoint,
                    SessionControlFailure::new(
                        SessionComponentId::Endpoint {
                            route_id: *route_id,
                            endpoint_id: *endpoint_id,
                        },
                        "finalize_started_endpoint",
                        error.to_string(),
                    ),
                ));
            }
        }
    }
    rollback
}

fn rollback_captures(captures: Vec<OpenedCapture>) -> StartupRollback {
    let mut rollback = StartupRollback::default();
    for capture in captures.into_iter().rev() {
        if let Err(error) = capture.owner.stop_and_join() {
            rollback.failures.push(SessionRollbackFailure::new(
                SessionRollbackStage::StopOpenedCapture,
                SessionControlFailure::new(
                    SessionComponentId::Source {
                        stem_id: capture.stem_id,
                    },
                    "stop_opened_capture",
                    error.to_string(),
                ),
            ));
        }
    }
    rollback
}

fn rollback_runtime_resources(
    sources: Vec<RuntimeSource>,
    mut runner: RealtimePlanRunner,
) -> StartupRollback {
    let mut rollback = StartupRollback::default();
    for source in sources.into_iter().rev() {
        if let Err(error) = source.capture.stop_and_join() {
            rollback.failures.push(SessionRollbackFailure::new(
                SessionRollbackStage::StopOpenedCapture,
                SessionControlFailure::new(
                    SessionComponentId::Source {
                        stem_id: source.stem_id,
                    },
                    "stop_opened_capture",
                    error.to_string(),
                ),
            ));
        }
    }
    if let Err(error) = runner.finish(PlanRunnerDrainPolicy::DiscardQueued, 1) {
        rollback.failures.push(SessionRollbackFailure::new(
            SessionRollbackStage::DiscardRuntimeQueues,
            SessionControlFailure::new(
                SessionComponentId::Runtime,
                "discard_runtime_queues",
                error.to_string(),
            ),
        ));
    }
    rollback
}

fn rollback_worker_outcome(outcome: RuntimeWorkerOutcome) -> StartupRollback {
    let mut rollback = StartupRollback::default();
    for (stem_id, result) in outcome.captures {
        if let Err(error) = result {
            rollback.failures.push(SessionRollbackFailure::new(
                SessionRollbackStage::StopOpenedCapture,
                SessionControlFailure::new(
                    SessionComponentId::Source { stem_id },
                    "stop_opened_capture",
                    error.to_string(),
                ),
            ));
        }
    }
    if let Err(error) = outcome.runner {
        rollback.failures.push(SessionRollbackFailure::new(
            SessionRollbackStage::DiscardRuntimeQueues,
            SessionControlFailure::new(
                SessionComponentId::Runtime,
                "discard_runtime_queues",
                error.to_string(),
            ),
        ));
    }
    rollback
}

fn stop_outcome(
    worker: &Option<std::thread::Result<Option<RuntimeWorkerOutcome>>>,
    endpoint_outcomes: &[(Vec<(RouteId, EndpointId)>, EndpointFinalizationOutcome)],
) -> SessionStopOutcome {
    let endpoint_finalization_failures_total = endpoint_outcomes
        .iter()
        .filter(|(_, outcome)| !outcome.is_success())
        .count() as u64;
    match worker {
        Some(Ok(Some(worker))) => SessionStopOutcome {
            runtime_worker_panicked: false,
            capture_finalization_failures_total: worker
                .captures
                .iter()
                .filter(|(_, result)| result.is_err())
                .count() as u64,
            endpoint_finalization_failures_total,
            runtime_failures_total: worker.runtime_failures_total
                + u64::from(worker.runner.is_err()),
            lineage_failures_total: worker.lineage_failures_total,
            source_send_rejections_total: worker.source_send_rejections_total,
            runtime_events_total: worker.runtime_events_total,
        },
        Some(Ok(None)) | Some(Err(_)) | None => SessionStopOutcome {
            runtime_worker_panicked: true,
            capture_finalization_failures_total: 1,
            endpoint_finalization_failures_total,
            runtime_failures_total: 1,
            lineage_failures_total: 0,
            source_send_rejections_total: 0,
            runtime_events_total: 0,
        },
    }
}

fn publish_terminal_events(
    session_id: SessionId,
    event_sender: &SessionEventSender,
    worker: &Option<std::thread::Result<Option<RuntimeWorkerOutcome>>>,
    endpoint_outcomes: &[(Vec<(RouteId, EndpointId)>, EndpointFinalizationOutcome)],
    outcome: SessionStopOutcome,
) {
    let source_failures = match worker {
        Some(Ok(Some(worker))) => worker.source_failures.clone(),
        _ => Vec::new(),
    };
    let mut endpoint_failures = Vec::new();
    let mut finalization_failures = Vec::new();

    if let Some(Ok(Some(worker))) = worker {
        for (stem_id, result) in &worker.captures {
            if let Err(error) = result {
                push_finalization_failure(
                    session_id,
                    event_sender,
                    &mut finalization_failures,
                    SessionFinalizationStage::StopCapture,
                    SessionComponentId::Source { stem_id: *stem_id },
                    "stop_capture",
                    error.to_string(),
                );
            }
        }
        if let Err(error) = &worker.runner {
            push_finalization_failure(
                session_id,
                event_sender,
                &mut finalization_failures,
                SessionFinalizationStage::DrainRuntime,
                SessionComponentId::Runtime,
                "finish_runtime",
                error.to_string(),
            );
        }
        if worker.lineage_failures_total > 0 {
            push_finalization_failure(
                session_id,
                event_sender,
                &mut finalization_failures,
                SessionFinalizationStage::DrainRuntime,
                SessionComponentId::Runtime,
                "validate_lineage",
                "capture lineage validation failed",
            );
        }
        if worker.source_send_rejections_total > 0 {
            push_finalization_failure(
                session_id,
                event_sender,
                &mut finalization_failures,
                SessionFinalizationStage::DrainRuntime,
                SessionComponentId::Runtime,
                "route_source_frame",
                "runtime source input rejected a frame",
            );
        }
    } else {
        push_finalization_failure(
            session_id,
            event_sender,
            &mut finalization_failures,
            SessionFinalizationStage::DrainRuntime,
            SessionComponentId::Runtime,
            "join_runtime_worker",
            "runtime worker did not return an outcome",
        );
    }

    for (identities, endpoint_outcome) in endpoint_outcomes {
        let (request_endpoint_failures, request_finalization_failures) =
            publish_endpoint_outcome_failures(
                session_id,
                event_sender,
                identities,
                &endpoint_outcome.request_stop_result,
                SessionFinalizationStage::RequestEndpointStop,
                "request_endpoint_stop",
            );
        endpoint_failures.extend(request_endpoint_failures);
        finalization_failures.extend(request_finalization_failures);
        let (join_endpoint_failures, join_finalization_failures) =
            publish_endpoint_outcome_failures(
                session_id,
                event_sender,
                identities,
                &endpoint_outcome.join_finalize_result,
                SessionFinalizationStage::FinalizeEndpoint,
                "finalize_endpoint",
            );
        endpoint_failures.extend(join_endpoint_failures);
        finalization_failures.extend(join_finalization_failures);
    }

    let terminal = SessionTerminalOutcome::new(
        session_id,
        source_failures,
        endpoint_failures,
        Vec::new(),
        finalization_failures,
    );
    let lifecycle = if outcome.is_success() {
        SessionLifecycleState::Stopped
    } else {
        SessionLifecycleState::Failed
    };
    let _ = event_sender.publish_lifecycle(session_id, lifecycle);
    let _ = event_sender.publish_terminal(terminal);
}

fn publish_endpoint_outcome_failures(
    session_id: SessionId,
    event_sender: &SessionEventSender,
    identities: &[(RouteId, EndpointId)],
    result: &Result<(), EndpointFailure>,
    stage: SessionFinalizationStage,
    operation: &'static str,
) -> (
    Vec<crate::SessionEndpointFailure>,
    Vec<SessionFinalizationFailure>,
) {
    let mut endpoint_failures = Vec::new();
    let mut finalization_failures = Vec::new();
    let Err(error) = result else {
        return (endpoint_failures, finalization_failures);
    };
    for (route_id, endpoint_id) in identities {
        let endpoint_failure =
            crate::SessionEndpointFailure::new(*route_id, *endpoint_id, error.clone());
        let _ = event_sender.publish_endpoint(session_id, endpoint_failure.clone());
        endpoint_failures.push(endpoint_failure);
        push_finalization_failure(
            session_id,
            event_sender,
            &mut finalization_failures,
            stage,
            SessionComponentId::Endpoint {
                route_id: *route_id,
                endpoint_id: *endpoint_id,
            },
            operation,
            error.to_string(),
        );
    }
    (endpoint_failures, finalization_failures)
}

fn push_finalization_failure(
    session_id: SessionId,
    event_sender: &SessionEventSender,
    failures: &mut Vec<SessionFinalizationFailure>,
    stage: SessionFinalizationStage,
    component: SessionComponentId,
    operation: &'static str,
    error_class: impl Into<String>,
) {
    let failure = SessionFinalizationFailure::new(
        stage,
        SessionControlFailure::new(component, operation, error_class),
    );
    let _ = event_sender.publish_finalization(session_id, failure.clone());
    failures.push(failure);
}

fn complete_start_failure(
    session_id: SessionId,
    event_sender: &SessionEventSender,
    event_receiver: SessionEventReceiver,
    error: SessionStartError,
    rollback_failures: Vec<SessionRollbackFailure>,
) -> SessionStartFailure {
    for failure in &rollback_failures {
        let _ = event_sender.publish_rollback(session_id, failure.clone());
    }
    let _ = event_sender.publish_lifecycle(session_id, SessionLifecycleState::Failed);
    let _ = event_sender.publish_terminal(SessionTerminalOutcome::failed_start(
        session_id,
        rollback_failures.clone(),
    ));
    SessionStartFailure {
        error,
        event_receiver: Some(event_receiver),
        rollback_failures: rollback_failures.into_boxed_slice(),
    }
}

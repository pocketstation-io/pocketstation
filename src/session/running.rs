use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::capture::{
    capture_delivery_start_gate, prepare_capture_with_start_gate, CallbackCaptureBackend,
    CaptureDeliveryStartGate, CaptureError, CaptureLineageSeed, CaptureMode,
    CaptureObservationReceipt, CapturePrepareRequest, CaptureStopOutcome, InputDeviceSelector,
    SourceRuntimeEventReceive,
};
use crate::endpoint::{
    endpoint_start_gate, DerivedEndpointDriverInput, EndpointDriverInput,
    EndpointDriverObservations, EndpointDriverRegistry, EndpointFailure,
    EndpointFinalizationOutcome, EndpointPrepareContext, EndpointPrepareError,
    EndpointRouteContext, EndpointSignalRouteContext, EndpointStartFailure, PreparedEndpoint,
    RunningEndpoint, SessionTimelineOrigin,
};
use crate::frame::{ClockDomainId, EndpointId, RouteId, SessionId, StemId};
use crate::runtime::{
    AsyncOperatorObservationHandle, AsyncOperatorObservations, AsyncOperatorOutput,
    AsyncOperatorOutputObservationHandle, AsyncOperatorWorker, AsyncRuntimeHost,
    CompiledOperatorInputContract, GeneratedAudioBridge, GeneratedAudioBridgeSpec,
    PlanEdgeObservationHandle, PlanRunnerDrainPolicy, PlanRunnerError, PlanRunnerFinishSummary,
    PlanSourceObservationHandle, PlanSourceSendOutcome, RealtimePlanRunner,
};

use crate::session::events::{session_event_channel, SessionEventSender};
use crate::session::runtime_prepare::{
    PreparedDerivedRouteMapping, PreparedExternalSourceMapping, PreparedExternalSourceTarget,
    PreparedOperatorMapping, PreparedWorkerOrigin,
};
use crate::session::{
    ApplicationSelector, DeviceSelector, EndpointObservationStage, OperatorInstanceId,
    PreparedSession, PreparedSourceRuntime, PreparedWorkerMapping, SessionComponentId,
    SessionControlFailure, SessionDerivedRouteMetrics, SessionEventReceiver,
    SessionExternalSourceMetrics, SessionFinalizationFailure, SessionFinalizationStage,
    SessionLifecycleState, SessionOperatorMetrics, SessionRollbackFailure, SessionRollbackStage,
    SessionRouteMetrics, SessionSourceFailure, SessionSourceMetrics, SessionTerminalOutcome,
    SessionTraceRecorderHandle, Source, SourceOutputBranchSpec, SourceOutputIdentity,
    SourceRegistry, SourceRuntime, SourceRuntimeObservationHandle, SourceSessionContext,
    RECORDING_GROUP_CONFIGURATION_KEY,
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

/// Thread-safe cancellation request for a Session that has not reached
/// `Running` yet.
#[derive(Clone, Debug, Default)]
pub struct SessionStartCancellation {
    requested: Arc<AtomicBool>,
}

impl SessionStartCancellation {
    pub fn request(&self) {
        self.requested.store(true, Ordering::Release);
    }

    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionStartError {
    #[error("invalid Session start options: {reason}")]
    InvalidOptions { reason: &'static str },
    #[error("Session requires exactly one application and one microphone source")]
    UnsupportedSourceTopology,
    #[error("external source preparation failed: {message}")]
    ExternalSourcePrepare {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("external source audio ingress failed: {message}")]
    ExternalAudioBridge {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("external source start failed: {message}")]
    ExternalSourceStart {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("async operator runtime host could not start: {message}")]
    OperatorRuntimeHost {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("operator {operator_instance_id:?} preparation failed: {message}")]
    OperatorPrepare {
        operator_instance_id: OperatorInstanceId,
        message: String,
        rollback_failures_total: u64,
    },
    #[error("endpoint {endpoint_id:?} declaration is absent")]
    MissingEndpointDeclaration {
        endpoint_id: crate::frame::EndpointId,
    },
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
    #[error("Session start was cancelled")]
    Cancelled { rollback_failures_total: u64 },
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
            }
            | Self::OperatorRuntimeHost {
                rollback_failures_total,
                ..
            }
            | Self::OperatorPrepare {
                rollback_failures_total,
                ..
            }
            | Self::ExternalSourcePrepare {
                rollback_failures_total,
                ..
            }
            | Self::ExternalAudioBridge {
                rollback_failures_total,
                ..
            }
            | Self::ExternalSourceStart {
                rollback_failures_total,
                ..
            }
            | Self::Cancelled {
                rollback_failures_total,
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
    capture: crate::capture::CaptureOwner,
    sender: crate::runtime::PlanSourceSender,
}

struct OpenedCapture {
    stem_id: StemId,
    owner: crate::capture::CaptureOwner,
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

struct PendingDerivedEndpointInput {
    mapping: PreparedDerivedRouteMapping,
    output: AsyncOperatorOutput,
}

struct PreparedOperatorRuntime {
    instance_id: OperatorInstanceId,
    worker: AsyncOperatorWorker,
    input_edge: PlanEdgeObservationHandle,
    observations: AsyncOperatorObservationHandle,
    lifecycle_timeout: Duration,
    derived_inputs: Vec<PendingDerivedEndpointInput>,
}

struct RunningOperatorBinding {
    instance_id: OperatorInstanceId,
    worker: AsyncOperatorWorker,
    input_edge: PlanEdgeObservationHandle,
    observations: AsyncOperatorObservationHandle,
    lifecycle_timeout: Duration,
}

struct SourceObservationBinding {
    stem_id: StemId,
    capture: CaptureObservationReceipt,
    ingress: PlanSourceObservationHandle,
}

struct RouteObservationBinding {
    route_id: RouteId,
    endpoint_id: EndpointId,
    edge: PlanEdgeObservationHandle,
}

struct DerivedRouteObservationBinding {
    route_id: RouteId,
    endpoint_id: EndpointId,
    output: AsyncOperatorOutputObservationHandle,
}

struct PreparedExternalRuntimeBinding {
    instance_id: crate::session::SourceInstanceId,
    source_id: crate::frame::SourceId,
    runtime: PreparedSourceRuntime,
}

struct RunningExternalRuntimeBinding {
    instance_id: crate::session::SourceInstanceId,
    source_id: crate::frame::SourceId,
    runtime: SourceRuntime,
    observations: SourceRuntimeObservationHandle,
}

type IndexedSessionMetrics = (
    Box<[SessionSourceMetrics]>,
    Box<[SessionExternalSourceMetrics]>,
    Box<[SessionRouteMetrics]>,
    Box<[SessionOperatorMetrics]>,
    Box<[SessionDerivedRouteMetrics]>,
);

type DerivedEndpointPreparationResult = Result<
    (
        Vec<PreparedEndpointBinding>,
        Vec<DerivedRouteObservationBinding>,
    ),
    (SessionStartError, Vec<SessionRollbackFailure>),
>;

#[derive(Clone, Copy)]
struct FinalEndpointObservation {
    route_id: RouteId,
    endpoint_id: EndpointId,
    observations: EndpointDriverObservations,
    finalization_failures_total: u64,
}

#[derive(Clone, Copy)]
struct FinalOperatorObservation {
    operator_instance_id: OperatorInstanceId,
    input_edge: crate::runtime::EdgeObservations,
    observations: AsyncOperatorObservations,
    finalization_failures_total: u64,
}

struct OperatorFinalizationOutcome {
    operator_instance_id: OperatorInstanceId,
    input_edge: crate::runtime::EdgeObservations,
    observations: AsyncOperatorObservations,
    error: Option<String>,
}

#[derive(Clone, Copy)]
enum OperatorTermination {
    Finish,
    Cancel,
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
    operator_finalization_failures_total: u64,
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
            && self.operator_finalization_failures_total == 0
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

    pub const fn operator_finalization_failures_total(&self) -> u64 {
        self.operator_finalization_failures_total
    }

    pub const fn runtime_worker_panicked(&self) -> bool {
        self.runtime_worker_panicked
    }

    pub const fn runtime_failures_total(&self) -> u64 {
        self.runtime_failures_total
    }

    pub const fn lineage_failures_total(&self) -> u64 {
        self.lineage_failures_total
    }

    pub const fn source_send_rejections_total(&self) -> u64 {
        self.source_send_rejections_total
    }

    pub const fn runtime_events_total(&self) -> u64 {
        self.runtime_events_total
    }
}

pub struct RunningSession {
    session_id: SessionId,
    stop_requested: Arc<AtomicBool>,
    runtime_worker: Option<JoinHandle<Option<RuntimeWorkerOutcome>>>,
    async_runtime_host: Option<AsyncRuntimeHost>,
    operators: Vec<RunningOperatorBinding>,
    endpoints: Vec<RunningEndpointBinding>,
    event_sender: SessionEventSender,
    event_receiver: Option<SessionEventReceiver>,
    source_observations: Vec<SourceObservationBinding>,
    external_sources: Vec<RunningExternalRuntimeBinding>,
    external_audio_bridges: Vec<GeneratedAudioBridge>,
    route_observations: Vec<RouteObservationBinding>,
    derived_route_observations: Vec<DerivedRouteObservationBinding>,
    final_operator_observations: Vec<FinalOperatorObservation>,
    final_endpoint_observations: Vec<FinalEndpointObservation>,
    final_external_source_observations: Vec<SessionExternalSourceMetrics>,
    stop_outcome: Option<SessionStopOutcome>,
}

impl RunningSession {
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn take_event_receiver(&mut self) -> Option<SessionEventReceiver> {
        self.event_receiver.take()
    }

    pub(crate) fn indexed_metrics(
        &self,
    ) -> (Box<[SessionSourceMetrics]>, Box<[SessionRouteMetrics]>) {
        let sources = self
            .source_observations
            .iter()
            .map(|binding| SessionSourceMetrics {
                stem_id: binding.stem_id,
                capture: binding.capture.observations(),
                ingress: binding.ingress.observations(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let routes = self
            .route_observations
            .iter()
            .map(|binding| {
                let (endpoint, endpoint_observation_stage, finalization_failures_total) =
                    self.endpoint_observations(binding.route_id, binding.endpoint_id);
                SessionRouteMetrics {
                    route_id: binding.route_id,
                    endpoint_id: binding.endpoint_id,
                    edge: binding.edge.observations(),
                    endpoint,
                    endpoint_observation_stage,
                    endpoint_finalization_failures_total: finalization_failures_total,
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        (sources, routes)
    }

    pub(crate) fn indexed_metrics_full(&self) -> IndexedSessionMetrics {
        let (sources, routes) = self.indexed_metrics();
        let external_sources = if self.external_sources.is_empty() {
            self.final_external_source_observations
                .clone()
                .into_boxed_slice()
        } else {
            self.external_sources
                .iter()
                .map(|binding| SessionExternalSourceMetrics {
                    source_instance_id: binding.instance_id,
                    source_id: binding.source_id,
                    runtime: binding.observations.snapshot(),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
        };
        let operators =
            self.operators
                .iter()
                .map(|binding| {
                    let finalized = self.final_operator_observations.iter().find(|observation| {
                        observation.operator_instance_id == binding.instance_id
                    });
                    SessionOperatorMetrics {
                        operator_instance_id: binding.instance_id,
                        input_edge: finalized.map_or_else(
                            || binding.input_edge.observations(),
                            |observation| observation.input_edge,
                        ),
                        worker: finalized.map_or_else(
                            || binding.observations.snapshot(),
                            |observation| observation.observations,
                        ),
                        finalization_failures_total: finalized
                            .map_or(0, |observation| observation.finalization_failures_total),
                    }
                })
                .chain(
                    self.final_operator_observations
                        .iter()
                        .filter(|observation| {
                            !self.operators.iter().any(|binding| {
                                binding.instance_id == observation.operator_instance_id
                            })
                        })
                        .map(|observation| SessionOperatorMetrics {
                            operator_instance_id: observation.operator_instance_id,
                            input_edge: observation.input_edge,
                            worker: observation.observations,
                            finalization_failures_total: observation.finalization_failures_total,
                        }),
                )
                .collect::<Vec<_>>()
                .into_boxed_slice();
        let derived_routes = self
            .derived_route_observations
            .iter()
            .map(|binding| {
                let (endpoint, endpoint_observation_stage, finalization_failures_total) =
                    self.endpoint_observations(binding.route_id, binding.endpoint_id);
                SessionDerivedRouteMetrics {
                    route_id: binding.route_id,
                    endpoint_id: binding.endpoint_id,
                    output: binding.output.snapshot(),
                    endpoint,
                    endpoint_observation_stage,
                    endpoint_finalization_failures_total: finalization_failures_total,
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        (sources, external_sources, routes, operators, derived_routes)
    }

    pub fn stop(&mut self) -> SessionStopOutcome {
        match self.stop_outcome {
            Some(outcome) => outcome,
            None => {
                let outcome = self.stop_once(OperatorTermination::Finish);
                self.stop_outcome = Some(outcome);
                outcome
            }
        }
    }

    pub fn cancel(&mut self) -> SessionStopOutcome {
        match self.stop_outcome {
            Some(outcome) => outcome,
            None => {
                let outcome = self.stop_once(OperatorTermination::Cancel);
                self.stop_outcome = Some(outcome);
                outcome
            }
        }
    }

    fn stop_once(&mut self, operator_termination: OperatorTermination) -> SessionStopOutcome {
        let _ = self
            .event_sender
            .publish_lifecycle(self.session_id, SessionLifecycleState::Stopping);
        for source in &self.external_sources {
            source.runtime.cancel();
        }
        self.stop_requested.store(true, Ordering::Release);
        let mut final_external_source_observations =
            Vec::with_capacity(self.external_sources.len());
        let external_source_failures_total = self
            .external_sources
            .drain(..)
            .map(|mut source| {
                let failed = u64::from(source.runtime.join().is_err());
                final_external_source_observations.push(SessionExternalSourceMetrics {
                    source_instance_id: source.instance_id,
                    source_id: source.source_id,
                    runtime: source.observations.snapshot(),
                });
                failed
            })
            .sum::<u64>();
        self.final_external_source_observations = final_external_source_observations;
        for bridge in self.external_audio_bridges.drain(..) {
            bridge.cancel_and_join();
        }
        let worker = self.runtime_worker.take().map(JoinHandle::join);
        let (operator_outcomes, operator_runtime_shutdown_error) =
            self.async_runtime_host.take().map_or_else(
                || (Vec::new(), None),
                |host| {
                    let outcomes = terminate_operators(
                        &host,
                        self.operators.drain(..).collect(),
                        operator_termination,
                    );
                    let shutdown_error = host.shutdown().err().map(|error| error.to_string());
                    (outcomes, shutdown_error)
                },
            );
        self.final_operator_observations = operator_outcomes
            .iter()
            .map(|outcome| FinalOperatorObservation {
                operator_instance_id: outcome.operator_instance_id,
                input_edge: outcome.input_edge,
                observations: outcome.observations,
                finalization_failures_total: u64::from(outcome.error.is_some()),
            })
            .collect();
        for endpoint in &mut self.endpoints {
            let _ = endpoint.endpoint.request_stop();
        }
        let endpoint_outcomes = self
            .endpoints
            .drain(..)
            .map(|binding| (binding.identities, binding.endpoint.join_and_finalize()))
            .collect::<Vec<_>>();
        self.final_endpoint_observations = endpoint_outcomes
            .iter()
            .flat_map(|(identities, outcome)| {
                identities
                    .iter()
                    .map(move |(route_id, endpoint_id)| FinalEndpointObservation {
                        route_id: *route_id,
                        endpoint_id: *endpoint_id,
                        observations: outcome.observations,
                        finalization_failures_total: u64::from(
                            outcome.request_stop_result.is_err(),
                        ) + u64::from(
                            outcome.join_finalize_result.is_err(),
                        ),
                    })
            })
            .collect();
        let outcome = stop_outcome(
            &worker,
            &operator_outcomes,
            operator_runtime_shutdown_error.as_deref(),
            &endpoint_outcomes,
        );
        let outcome = SessionStopOutcome {
            runtime_failures_total: outcome
                .runtime_failures_total
                .saturating_add(external_source_failures_total),
            ..outcome
        };
        publish_terminal_events(
            self.session_id,
            &self.event_sender,
            &worker,
            &operator_outcomes,
            operator_runtime_shutdown_error.as_deref(),
            &endpoint_outcomes,
            outcome,
        );
        outcome
    }

    fn endpoint_observations(
        &self,
        route_id: RouteId,
        endpoint_id: EndpointId,
    ) -> (
        Option<EndpointDriverObservations>,
        EndpointObservationStage,
        u64,
    ) {
        if let Some(finalized) = self.final_endpoint_observations.iter().find(|observation| {
            observation.route_id == route_id && observation.endpoint_id == endpoint_id
        }) {
            return (
                Some(finalized.observations),
                EndpointObservationStage::Finalized,
                finalized.finalization_failures_total,
            );
        }
        self.endpoints
            .iter()
            .find(|binding| binding.identities.contains(&(route_id, endpoint_id)))
            .map_or(
                (None, EndpointObservationStage::Unavailable, 0),
                |binding| {
                    (
                        Some(binding.endpoint.observations()),
                        EndpointObservationStage::Live,
                        0,
                    )
                },
            )
    }
}

impl Drop for RunningSession {
    fn drop(&mut self) {
        if self.stop_outcome.is_none() {
            let _ = self.stop_once(OperatorTermination::Finish);
        }
    }
}

pub fn start_prepared_session(
    prepared: PreparedSession,
    capture_backends: CaptureBackendSet<'_>,
    endpoint_registry: &EndpointDriverRegistry,
    options: SessionStartOptions,
) -> Result<RunningSession, SessionStartFailure> {
    start_prepared_session_cancellable(
        prepared,
        capture_backends,
        endpoint_registry,
        options,
        SessionStartCancellation::default(),
    )
}

pub fn start_prepared_session_cancellable(
    prepared: PreparedSession,
    capture_backends: CaptureBackendSet<'_>,
    endpoint_registry: &EndpointDriverRegistry,
    options: SessionStartOptions,
    start_cancellation: SessionStartCancellation,
) -> Result<RunningSession, SessionStartFailure> {
    let source_registry = SourceRegistry::default();
    start_prepared_session_cancellable_with_trace(
        prepared,
        capture_backends,
        endpoint_registry,
        &source_registry,
        options,
        start_cancellation,
        None,
    )
}

pub(crate) fn start_prepared_session_cancellable_with_trace(
    prepared: PreparedSession,
    capture_backends: CaptureBackendSet<'_>,
    endpoint_registry: &EndpointDriverRegistry,
    source_registry: &SourceRegistry,
    options: SessionStartOptions,
    start_cancellation: SessionStartCancellation,
    session_trace_recorder: Option<SessionTraceRecorderHandle>,
) -> Result<RunningSession, SessionStartFailure> {
    validate_start_options(options).map_err(SessionStartFailure::input)?;
    validate_source_topology(&prepared).map_err(SessionStartFailure::input)?;
    let PreparedSession {
        spec,
        executor,
        source_mappings,
        source_inputs,
        worker_mappings,
        operator_mappings,
        external_source_mappings,
        cancellation,
    } = prepared;
    let session_id = spec.session_id();
    let (event_sender, event_receiver) = session_event_channel(
        options.session_event_capacity_events,
        session_trace_recorder,
    );
    let _ = event_sender.publish_lifecycle(session_id, SessionLifecycleState::Starting);
    if start_cancellation.is_requested() {
        return Err(complete_start_failure(
            session_id,
            &event_sender,
            event_receiver,
            SessionStartError::Cancelled {
                rollback_failures_total: 0,
            },
            Vec::new(),
        ));
    }
    let session_timeline_origin =
        SessionTimelineOrigin::from_monotonic_timestamp_ns(crate::timing::monotonic_timestamp_ns());
    let (gate_controller, start_gate) = endpoint_start_gate();
    let (capture_gate_controller, capture_start_gate) = capture_delivery_start_gate();
    let (
        mut prepared_external_sources,
        external_audio_bridges,
        mut prepared_external_endpoints,
        external_route_observations,
    ) = match prepare_external_source_runtimes(
        session_id,
        external_source_mappings,
        source_registry,
        endpoint_registry,
        session_timeline_origin,
    ) {
        Ok(prepared) => prepared,
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
    let source_ingress_observations = source_mappings
        .iter()
        .map(|mapping| (mapping.stem_id, mapping.sender.observation_handle()))
        .collect::<Vec<_>>();
    let route_observations = worker_mappings
        .iter()
        .map(|mapping| RouteObservationBinding {
            route_id: mapping.route_id,
            endpoint_id: mapping.endpoint_id,
            edge: mapping.receiver.observation_handle(),
        })
        .collect::<Vec<_>>();

    let mut prepared_endpoints = match prepare_endpoints(
        &spec,
        worker_mappings,
        endpoint_registry,
        session_timeline_origin,
    ) {
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
    prepared_endpoints.append(&mut prepared_external_endpoints);
    if start_cancellation.is_requested() {
        let rollback = rollback_prepared_endpoints(prepared_endpoints);
        return Err(complete_start_failure(
            session_id,
            &event_sender,
            event_receiver,
            SessionStartError::Cancelled {
                rollback_failures_total: rollback.failures_total(),
            },
            rollback.failures,
        ));
    }
    let mut running_endpoints = Vec::with_capacity(prepared_endpoints.len());
    while let Some(endpoint) = prepared_endpoints.pop() {
        if start_cancellation.is_requested() {
            prepared_endpoints.push(endpoint);
            let mut rollback = rollback_running_endpoints(running_endpoints);
            rollback.append(rollback_prepared_endpoints(prepared_endpoints));
            return Err(complete_start_failure(
                session_id,
                &event_sender,
                event_receiver,
                SessionStartError::Cancelled {
                    rollback_failures_total: rollback.failures_total(),
                },
                rollback.failures,
            ));
        }
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

    let captures = match prepare_and_open_captures(
        &spec,
        capture_backends,
        options,
        Arc::clone(&capture_start_gate),
    ) {
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
    if start_cancellation.is_requested() {
        let mut rollback = rollback_captures(captures);
        rollback.append(rollback_running_endpoints(running_endpoints));
        return Err(complete_start_failure(
            session_id,
            &event_sender,
            event_receiver,
            SessionStartError::Cancelled {
                rollback_failures_total: rollback.failures_total(),
            },
            rollback.failures,
        ));
    }

    let source_observations = captures
        .iter()
        .zip(source_ingress_observations)
        .map(|(capture, (stem_id, ingress))| {
            debug_assert_eq!(capture.stem_id, stem_id);
            SourceObservationBinding {
                stem_id: capture.stem_id,
                capture: capture.owner.observation_receipt(),
                ingress,
            }
        })
        .collect::<Vec<_>>();
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
        .name("pocketstation-session-runtime".to_owned())
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

    let async_runtime_host = if operator_mappings.is_empty() {
        None
    } else {
        match AsyncRuntimeHost::new("pocketstation-session-operators") {
            Ok(host) => Some(host),
            Err(error) => {
                let mut rollback = rollback_started_runtime(&stop_requested, runtime_worker);
                rollback.append(rollback_running_endpoints(running_endpoints));
                let rollback_failures_total = rollback.failures_total();
                return Err(complete_start_failure(
                    session_id,
                    &event_sender,
                    event_receiver,
                    SessionStartError::OperatorRuntimeHost {
                        message: error.to_string(),
                        rollback_failures_total,
                    },
                    rollback.failures,
                ));
            }
        }
    };
    let mut prepared_operators = match async_runtime_host.as_ref() {
        Some(host) => match prepare_operator_runtimes(host, session_id, operator_mappings) {
            Ok(operators) => operators,
            Err((mut error, operator_rollback_failures)) => {
                let mut rollback = StartupRollback {
                    failures: operator_rollback_failures,
                };
                rollback.append(rollback_started_runtime(&stop_requested, runtime_worker));
                rollback.append(rollback_running_endpoints(running_endpoints));
                let rollback_failures_total = rollback.failures_total();
                if let SessionStartError::OperatorPrepare {
                    rollback_failures_total: total,
                    ..
                } = &mut error
                {
                    *total = rollback_failures_total;
                }
                return Err(complete_start_failure(
                    session_id,
                    &event_sender,
                    event_receiver,
                    error,
                    rollback.failures,
                ));
            }
        },
        None => Vec::new(),
    };
    let raw_endpoint_count = running_endpoints.len();
    let (mut prepared_derived_endpoints, derived_route_observations) =
        match prepare_derived_endpoints(
            &spec,
            &mut prepared_operators,
            endpoint_registry,
            session_timeline_origin,
        ) {
            Ok(prepared) => prepared,
            Err((error, endpoint_rollback_failures)) => {
                let mut rollback = StartupRollback {
                    failures: endpoint_rollback_failures,
                };
                if let Some(host) = async_runtime_host.as_ref() {
                    rollback.append(rollback_operator_runtimes(host, prepared_operators));
                }
                rollback.append(rollback_started_runtime(&stop_requested, runtime_worker));
                rollback.append(rollback_running_endpoints(running_endpoints));
                let error = match error {
                    SessionStartError::EndpointPrepare { source, .. } => {
                        SessionStartError::EndpointPrepare {
                            source,
                            rollback_failures_total: rollback.failures_total(),
                        }
                    }
                    other => other,
                };
                return Err(complete_start_failure(
                    session_id,
                    &event_sender,
                    event_receiver,
                    error,
                    rollback.failures,
                ));
            }
        };
    while let Some(endpoint) = prepared_derived_endpoints.pop() {
        if start_cancellation.is_requested() {
            prepared_derived_endpoints.push(endpoint);
            let mut rollback = rollback_prepared_endpoints(prepared_derived_endpoints);
            let running_derived_endpoints = running_endpoints.split_off(raw_endpoint_count);
            rollback.append(rollback_running_endpoints(running_derived_endpoints));
            if let Some(host) = async_runtime_host.as_ref() {
                rollback.append(rollback_operator_runtimes(host, prepared_operators));
            }
            rollback.append(rollback_started_runtime(&stop_requested, runtime_worker));
            rollback.append(rollback_running_endpoints(running_endpoints));
            return Err(complete_start_failure(
                session_id,
                &event_sender,
                event_receiver,
                SessionStartError::Cancelled {
                    rollback_failures_total: rollback.failures_total(),
                },
                rollback.failures,
            ));
        }
        match endpoint.endpoint.start(Arc::clone(&start_gate)) {
            Ok(running) => running_endpoints.push(RunningEndpointBinding {
                identities: endpoint.identities,
                endpoint: running,
            }),
            Err(source) => {
                let mut rollback = rollback_prepared_endpoints(prepared_derived_endpoints);
                let running_derived_endpoints = running_endpoints.split_off(raw_endpoint_count);
                rollback.append(rollback_running_endpoints(running_derived_endpoints));
                if let Some(host) = async_runtime_host.as_ref() {
                    rollback.append(rollback_operator_runtimes(host, prepared_operators));
                }
                rollback.append(rollback_started_runtime(&stop_requested, runtime_worker));
                rollback.append(rollback_running_endpoints(running_endpoints));
                return Err(complete_start_failure(
                    session_id,
                    &event_sender,
                    event_receiver,
                    SessionStartError::EndpointStart {
                        source,
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                ));
            }
        }
    }
    let running_operators = prepared_operators
        .into_iter()
        .map(|operator| RunningOperatorBinding {
            instance_id: operator.instance_id,
            worker: operator.worker,
            input_edge: operator.input_edge,
            observations: operator.observations,
            lifecycle_timeout: operator.lifecycle_timeout,
        })
        .collect::<Vec<_>>();

    let mut external_sources: Vec<RunningExternalRuntimeBinding> =
        Vec::with_capacity(prepared_external_sources.len());
    while let Some(prepared_source) = prepared_external_sources.pop() {
        let observations;
        let runtime = match prepared_source.runtime.start() {
            Ok(runtime) => {
                observations = runtime.observations();
                runtime
            }
            Err(error) => {
                for source in &external_sources {
                    source.runtime.cancel();
                }
                for mut source in external_sources {
                    let _ = source.runtime.join();
                }
                let running_derived_endpoints = running_endpoints.split_off(raw_endpoint_count);
                let mut rollback = rollback_running_endpoints(running_derived_endpoints);
                if let Some(host) = async_runtime_host.as_ref() {
                    rollback.append(rollback_running_operators(host, running_operators));
                }
                rollback.append(rollback_started_runtime(&stop_requested, runtime_worker));
                rollback.append(rollback_running_endpoints(running_endpoints));
                return Err(complete_start_failure(
                    session_id,
                    &event_sender,
                    event_receiver,
                    SessionStartError::ExternalSourceStart {
                        message: error.to_string(),
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                ));
            }
        };
        external_sources.push(RunningExternalRuntimeBinding {
            instance_id: prepared_source.instance_id,
            source_id: prepared_source.source_id,
            runtime,
            observations,
        });
    }

    if start_cancellation.is_requested() {
        let running_derived_endpoints = running_endpoints.split_off(raw_endpoint_count);
        let mut rollback = rollback_running_endpoints(running_derived_endpoints);
        if let Some(host) = async_runtime_host.as_ref() {
            rollback.append(rollback_running_operators(host, running_operators));
        }
        rollback.append(rollback_started_runtime(&stop_requested, runtime_worker));
        rollback.append(rollback_running_endpoints(running_endpoints));
        return Err(complete_start_failure(
            session_id,
            &event_sender,
            event_receiver,
            SessionStartError::Cancelled {
                rollback_failures_total: rollback.failures_total(),
            },
            rollback.failures,
        ));
    }

    // `gate_controller` is the sole controller returned by `endpoint_start_gate`
    // and has not been shared or opened on any preceding path.
    let _endpoints_opened = gate_controller.open();
    let _capture_delivery_opened = capture_gate_controller.open();
    let _ = event_sender.publish_lifecycle(session_id, SessionLifecycleState::Running);
    Ok(RunningSession {
        session_id,
        stop_requested,
        runtime_worker: Some(runtime_worker),
        async_runtime_host,
        operators: running_operators,
        endpoints: running_endpoints,
        event_sender,
        event_receiver: Some(event_receiver),
        source_observations,
        external_sources,
        external_audio_bridges,
        route_observations,
        derived_route_observations: derived_route_observations
            .into_iter()
            .chain(external_route_observations)
            .collect(),
        final_operator_observations: Vec::new(),
        final_endpoint_observations: Vec::new(),
        final_external_source_observations: Vec::new(),
        stop_outcome: None,
    })
}

type ExternalSourcePreparation = (
    Vec<PreparedExternalRuntimeBinding>,
    Vec<GeneratedAudioBridge>,
    Vec<PreparedEndpointBinding>,
    Vec<DerivedRouteObservationBinding>,
);

fn prepare_external_source_runtimes(
    session_id: SessionId,
    mappings: Vec<PreparedExternalSourceMapping>,
    source_registry: &SourceRegistry,
    endpoint_registry: &EndpointDriverRegistry,
    session_timeline_origin: SessionTimelineOrigin,
) -> Result<ExternalSourcePreparation, (SessionStartError, Vec<SessionRollbackFailure>)> {
    let mut sources = Vec::with_capacity(mappings.len());
    let mut bridges = Vec::new();
    let mut endpoints = Vec::new();
    let mut route_observations = Vec::new();
    for mapping in mappings {
        let branch_specs = mapping
            .branches
            .iter()
            .map(|branch| SourceOutputBranchSpec {
                output_port: branch.output_port.clone(),
                branch: branch.branch,
            })
            .collect::<Vec<_>>();
        let mut output_identities = Vec::new();
        for branch in &mapping.branches {
            if !output_identities
                .iter()
                .any(|identity: &SourceOutputIdentity| identity.output_port == branch.output_port)
            {
                output_identities.push(SourceOutputIdentity {
                    output_port: branch.output_port.clone(),
                    stream_id: branch.stream_id,
                });
            }
        }
        let (runtime, receivers) = match source_registry.prepare_session(
            &mapping.source_type_id,
            &mapping.configuration,
            &branch_specs,
            SourceSessionContext {
                session_id,
                source_id: mapping.source_id,
                outputs: output_identities,
            },
        ) {
            Ok(prepared) => prepared,
            Err(error) => {
                let rollback = rollback_prepared_endpoints(endpoints);
                return Err((
                    SessionStartError::ExternalSourcePrepare {
                        message: error.to_string(),
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                ));
            }
        };
        if receivers.len() != mapping.branches.len() {
            let rollback = rollback_prepared_endpoints(endpoints);
            return Err((
                SessionStartError::ExternalSourcePrepare {
                    message: "prepared source receiver topology does not match compiled branches"
                        .to_owned(),
                    rollback_failures_total: rollback.failures_total(),
                },
                rollback.failures,
            ));
        }
        for (branch, receiver) in mapping.branches.into_iter().zip(receivers) {
            debug_assert_eq!(branch.output_port, receiver.output_port);
            match branch.target {
                PreparedExternalSourceTarget::AudioIngress(audio) => {
                    let pool_slots = branch.branch.capacity_signals.clamp(1, 64);
                    let bridge = GeneratedAudioBridge::spawn(
                        receiver.receiver,
                        audio.sender,
                        GeneratedAudioBridgeSpec {
                            session_id,
                            stem_id: audio.stem_id,
                            stream_id: audio.stream_id,
                            source_id: audio.source_id,
                            clock_id: ClockDomainId(0),
                            sample_spec: audio.sample_spec,
                            samples_per_frame: audio.samples_per_frame,
                            pool_slots,
                        },
                    )
                    .map_err(|error| {
                        let rollback = rollback_prepared_endpoints(std::mem::take(&mut endpoints));
                        (
                            SessionStartError::ExternalAudioBridge {
                                message: error.to_string(),
                                rollback_failures_total: rollback.failures_total(),
                            },
                            rollback.failures,
                        )
                    })?;
                    bridges.push(bridge);
                }
                PreparedExternalSourceTarget::TypedEndpoint(route) => {
                    let observation = receiver.receiver.observation_handle();
                    let context = EndpointPrepareContext::new_signal(
                        session_id,
                        route.endpoint_id,
                        route.node_configuration,
                    )
                    .with_signal_route(
                        EndpointSignalRouteContext::new(
                            route.route_id,
                            route.source_id,
                            route.stream_id,
                        ),
                        session_timeline_origin,
                    )
                    .with_derived_contract(
                        route.signal_spec,
                        route.media,
                        route.edge_contract,
                    );
                    let endpoint = match endpoint_registry.prepare_derived_batch(
                        &route.endpoint_operator_id,
                        &route.endpoint_node_type_id,
                        vec![DerivedEndpointDriverInput::new(receiver.receiver, context)],
                    ) {
                        Ok(endpoint) => endpoint,
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
                    };
                    route_observations.push(DerivedRouteObservationBinding {
                        route_id: route.route_id,
                        endpoint_id: route.endpoint_id,
                        output: observation,
                    });
                    endpoints.push(PreparedEndpointBinding {
                        identities: vec![(route.route_id, route.endpoint_id)],
                        endpoint,
                    });
                }
            }
        }
        sources.push(PreparedExternalRuntimeBinding {
            instance_id: mapping.instance_id,
            source_id: mapping.source_id,
            runtime,
        });
    }
    Ok((sources, bridges, endpoints, route_observations))
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
    let built_in_topology_valid = (application_sources == 0 && microphone_sources == 0)
        || (application_sources == 1 && microphone_sources == 1);
    if built_in_topology_valid
        && (!prepared.spec.source_instances().is_empty() || application_sources == 1)
    {
        Ok(())
    } else {
        Err(SessionStartError::UnsupportedSourceTopology)
    }
}

fn prepare_endpoints(
    spec: &crate::session::SessionSpec,
    mut worker_mappings: Vec<PreparedWorkerMapping>,
    endpoint_registry: &EndpointDriverRegistry,
    session_timeline_origin: SessionTimelineOrigin,
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
            if let PreparedWorkerOrigin::Stem(stem_id) = mapping.origin {
                if !spec
                    .stems()
                    .iter()
                    .any(|candidate| candidate.id() == stem_id)
                {
                    return Err((SessionStartError::UnsupportedSourceTopology, Vec::new()));
                }
            }
            let mut context = EndpointPrepareContext::new(
                session_id,
                grouped_endpoint.id(),
                mapping.node_configuration,
                mapping.prepare_context,
            )
            .with_session_route(
                EndpointRouteContext::new(mapping.origin.stem_id(), mapping.route_id),
                session_timeline_origin,
            );
            if let PreparedWorkerOrigin::ExternalAudio {
                source_id,
                stream_id,
                ..
            } = mapping.origin
            {
                context = context.with_signal_route(
                    EndpointSignalRouteContext::new(mapping.route_id, source_id, stream_id),
                    session_timeline_origin,
                );
            }
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

fn prepare_operator_runtimes(
    host: &AsyncRuntimeHost,
    session_id: SessionId,
    operator_mappings: Vec<PreparedOperatorMapping>,
) -> Result<Vec<PreparedOperatorRuntime>, (SessionStartError, Vec<SessionRollbackFailure>)> {
    let mut prepared = Vec::with_capacity(operator_mappings.len());
    for mapping in operator_mappings {
        let PreparedOperatorMapping {
            node_id,
            instance_id,
            stem_id,
            factory,
            node_configuration,
            prepare_context,
            receiver,
            derived_routes,
        } = mapping;
        let input_contract = CompiledOperatorInputContract {
            edge_id: receiver.edge_id(),
            operator_node: node_id,
            session_id,
            stem_id,
            source_id: None,
        };
        let input_edge = receiver.observation_handle();
        let lifecycle_timeout = Duration::from_millis(
            u64::from(factory.manifest().deadline.process_timeout_ms).saturating_add(250),
        );
        let output_branches = derived_routes
            .iter()
            .map(|route| route.output_branch)
            .collect::<Vec<_>>();
        let result = host.execute(lifecycle_timeout, async move {
            AsyncOperatorWorker::prepare_and_spawn_from_plan_edge(
                factory,
                &node_configuration,
                prepare_context,
                receiver,
                input_contract,
                &output_branches,
            )
            .await
        });
        let (worker, outputs) = match result {
            Ok(Ok(prepared_worker)) => prepared_worker,
            Ok(Err(error)) => {
                let rollback = rollback_operator_runtimes(host, prepared);
                return Err((
                    SessionStartError::OperatorPrepare {
                        operator_instance_id: instance_id,
                        message: error.to_string(),
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                ));
            }
            Err(error) => {
                let rollback = rollback_operator_runtimes(host, prepared);
                return Err((
                    SessionStartError::OperatorPrepare {
                        operator_instance_id: instance_id,
                        message: error.to_string(),
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                ));
            }
        };
        if outputs.len() != derived_routes.len() {
            let observations = worker.observations();
            let mut incomplete = vec![PreparedOperatorRuntime {
                instance_id,
                worker,
                input_edge,
                observations,
                lifecycle_timeout,
                derived_inputs: Vec::new(),
            }];
            incomplete.append(&mut prepared);
            let rollback = rollback_operator_runtimes(host, incomplete);
            return Err((
                SessionStartError::OperatorPrepare {
                    operator_instance_id: instance_id,
                    message: "operator output branch count did not match compiled routes"
                        .to_owned(),
                    rollback_failures_total: rollback.failures_total(),
                },
                rollback.failures,
            ));
        }
        let observations = worker.observations();
        let derived_inputs = derived_routes
            .into_iter()
            .zip(outputs)
            .map(|(mapping, output)| PendingDerivedEndpointInput { mapping, output })
            .collect();
        prepared.push(PreparedOperatorRuntime {
            instance_id,
            worker,
            input_edge,
            observations,
            lifecycle_timeout,
            derived_inputs,
        });
    }
    Ok(prepared)
}

fn prepare_derived_endpoints(
    spec: &crate::session::SessionSpec,
    operators: &mut [PreparedOperatorRuntime],
    endpoint_registry: &EndpointDriverRegistry,
    session_timeline_origin: SessionTimelineOrigin,
) -> DerivedEndpointPreparationResult {
    let session_id = spec.session_id();
    let mut pending = operators
        .iter_mut()
        .flat_map(|operator| std::mem::take(&mut operator.derived_inputs))
        .collect::<Vec<_>>();
    let mut endpoints = Vec::with_capacity(pending.len());
    let mut route_observations = Vec::with_capacity(pending.len());
    while !pending.is_empty() {
        let input = pending.remove(0);
        let endpoint = spec
            .endpoints()
            .iter()
            .find(|endpoint| endpoint.id() == input.mapping.endpoint_id)
            .ok_or_else(|| {
                (
                    SessionStartError::MissingEndpointDeclaration {
                        endpoint_id: input.mapping.endpoint_id,
                    },
                    Vec::new(),
                )
            })?;
        let recording_group_id = endpoint
            .configuration()
            .get(RECORDING_GROUP_CONFIGURATION_KEY);
        let mut grouped_inputs = vec![input];
        if let Some(recording_group_id) = recording_group_id {
            let mut index = 0;
            while index < pending.len() {
                let candidate = &pending[index];
                let Some(candidate_endpoint) = spec
                    .endpoints()
                    .iter()
                    .find(|endpoint| endpoint.id() == candidate.mapping.endpoint_id)
                else {
                    return Err((
                        SessionStartError::MissingEndpointDeclaration {
                            endpoint_id: candidate.mapping.endpoint_id,
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
                    grouped_inputs.push(pending.remove(index));
                } else {
                    index += 1;
                }
            }
        }
        let mut identities = Vec::with_capacity(grouped_inputs.len());
        let mut driver_inputs = Vec::with_capacity(grouped_inputs.len());
        let mut grouped_observations = Vec::with_capacity(grouped_inputs.len());
        for input in grouped_inputs {
            let mapping = input.mapping;
            let context = EndpointPrepareContext::new(
                session_id,
                mapping.endpoint_id,
                mapping.node_configuration,
                mapping.prepare_context,
            )
            .with_session_route(
                EndpointRouteContext::new(mapping.stem_id, mapping.route_id),
                session_timeline_origin,
            )
            .with_derived_contract(
                mapping.signal_spec,
                mapping.output_branch.edge_contract.media,
                mapping.output_branch.edge_contract,
            );
            identities.push((mapping.route_id, mapping.endpoint_id));
            grouped_observations.push(DerivedRouteObservationBinding {
                route_id: mapping.route_id,
                endpoint_id: mapping.endpoint_id,
                output: input.output.observation_handle(),
            });
            driver_inputs.push(DerivedEndpointDriverInput::new(input.output, context));
        }
        match endpoint_registry.prepare_derived_batch(
            endpoint.operator_id(),
            endpoint.node_type_id(),
            driver_inputs,
        ) {
            Ok(endpoint) => {
                route_observations.extend(grouped_observations);
                endpoints.push(PreparedEndpointBinding {
                    identities,
                    endpoint,
                });
            }
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
    Ok((endpoints, route_observations))
}

fn prepare_and_open_captures(
    spec: &crate::session::SessionSpec,
    backends: CaptureBackendSet<'_>,
    options: SessionStartOptions,
    start_gate: Arc<CaptureDeliveryStartGate>,
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
        let prepared =
            match prepare_capture_with_start_gate(binding, request, Arc::clone(&start_gate)) {
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
        Source::Application(ApplicationSelector::ProcessInstance {
            process_id,
            stable_id,
        }) => CaptureMode::ExactApplication {
            process_id: process_id.get(),
            stable_id: stable_id.clone(),
        },
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
    sources: Vec<RuntimeSource>,
    mut runner: RealtimePlanRunner,
    stop_requested: Arc<AtomicBool>,
    options: SessionStartOptions,
    session_id: SessionId,
    event_sender: SessionEventSender,
) -> RuntimeWorkerOutcome {
    let mut sources = sources
        .into_iter()
        .map(|source| (source.stem_id, Some(source.capture), source.sender))
        .collect::<Vec<_>>();
    let mut captures = Vec::with_capacity(sources.len());
    let mut runtime_events_total = 0u64;
    let mut runtime_failures_total = 0u64;
    let mut lineage_failures_total = 0u64;
    let mut source_send_rejections_total = 0u64;
    let mut source_failures = Vec::new();
    while !stop_requested.load(Ordering::Acquire) {
        let mut work_observed = false;
        for (stem_id, capture, sender) in &mut sources {
            let Some(active_capture) = capture.as_mut() else {
                continue;
            };
            loop {
                match active_capture.try_next_lineaged_frame() {
                    Ok(Some(frame)) => {
                        work_observed = true;
                        if matches!(
                            sender.try_send(frame),
                            PlanSourceSendOutcome::Rejected { .. }
                        ) {
                            source_send_rejections_total =
                                source_send_rejections_total.saturating_add(1);
                        }
                    }
                    Ok(None) => break,
                    Err(_) => {
                        lineage_failures_total = lineage_failures_total.saturating_add(1);
                        if let Some(failed_capture) = capture.take() {
                            captures.push((*stem_id, failed_capture.stop_and_join()));
                        }
                        break;
                    }
                }
            }
            let Some(active_capture) = capture.as_ref() else {
                continue;
            };
            if let SourceRuntimeEventReceive::Event(event) = active_capture.try_recv_runtime_event()
            {
                runtime_events_total = runtime_events_total.saturating_add(1);
                runtime_failures_total = runtime_failures_total.saturating_add(1);
                let failure = SessionSourceFailure::new(*stem_id, event);
                let _ = event_sender.publish_source(session_id, failure.clone());
                source_failures.push(failure);
                if let Some(failed_capture) = capture.take() {
                    captures.push((*stem_id, failed_capture.stop_and_join()));
                }
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
    captures.extend(
        sources
            .into_iter()
            .filter_map(|(stem_id, capture, _sender)| {
                capture.map(|capture| (stem_id, capture.stop_and_join()))
            }),
    );
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

fn rollback_operator_runtimes(
    host: &AsyncRuntimeHost,
    operators: Vec<PreparedOperatorRuntime>,
) -> StartupRollback {
    let mut rollback = StartupRollback::default();
    for operator in operators.into_iter().rev() {
        drop(operator.derived_inputs);
        let observations = operator.observations;
        let result = host.execute(
            operator.lifecycle_timeout,
            operator.worker.cancel_and_join(),
        );
        let error = match result {
            Ok(Ok(())) => None,
            Ok(Err(error)) => Some(error.to_string()),
            Err(error) => Some(error.to_string()),
        };
        if let Some(error) = error {
            rollback.failures.push(SessionRollbackFailure::new(
                SessionRollbackStage::CancelOperator,
                SessionControlFailure::new(
                    SessionComponentId::Operator {
                        operator_instance_id: operator.instance_id,
                    },
                    "cancel_operator",
                    error,
                ),
            ));
        }
        let _ = observations.snapshot();
    }
    rollback
}

fn terminate_operators(
    host: &AsyncRuntimeHost,
    operators: Vec<RunningOperatorBinding>,
    termination: OperatorTermination,
) -> Vec<OperatorFinalizationOutcome> {
    operators
        .into_iter()
        .map(|operator| {
            let result = match termination {
                OperatorTermination::Finish => host.execute(
                    operator.lifecycle_timeout,
                    operator.worker.finish_and_join(),
                ),
                OperatorTermination::Cancel => host.execute(
                    operator.lifecycle_timeout,
                    operator.worker.cancel_and_join(),
                ),
            };
            let error = match result {
                Ok(Ok(())) => None,
                Ok(Err(error)) => Some(error.to_string()),
                Err(error) => Some(error.to_string()),
            };
            OperatorFinalizationOutcome {
                operator_instance_id: operator.instance_id,
                input_edge: operator.input_edge.observations(),
                observations: operator.observations.snapshot(),
                error,
            }
        })
        .collect()
}

fn rollback_running_operators(
    host: &AsyncRuntimeHost,
    operators: Vec<RunningOperatorBinding>,
) -> StartupRollback {
    let mut rollback = StartupRollback::default();
    for operator in operators.into_iter().rev() {
        let result = host.execute(
            operator.lifecycle_timeout,
            operator.worker.cancel_and_join(),
        );
        let error = match result {
            Ok(Ok(())) => None,
            Ok(Err(error)) => Some(error.to_string()),
            Err(error) => Some(error.to_string()),
        };
        if let Some(error) = error {
            rollback.failures.push(SessionRollbackFailure::new(
                SessionRollbackStage::CancelOperator,
                SessionControlFailure::new(
                    SessionComponentId::Operator {
                        operator_instance_id: operator.instance_id,
                    },
                    "cancel_operator",
                    error,
                ),
            ));
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

fn rollback_started_runtime(
    stop_requested: &AtomicBool,
    worker: JoinHandle<Option<RuntimeWorkerOutcome>>,
) -> StartupRollback {
    stop_requested.store(true, Ordering::Release);
    match worker.join() {
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
    }
}

fn stop_outcome(
    worker: &Option<std::thread::Result<Option<RuntimeWorkerOutcome>>>,
    operator_outcomes: &[OperatorFinalizationOutcome],
    operator_runtime_shutdown_error: Option<&str>,
    endpoint_outcomes: &[(Vec<(RouteId, EndpointId)>, EndpointFinalizationOutcome)],
) -> SessionStopOutcome {
    let operator_finalization_failures_total = operator_outcomes
        .iter()
        .filter(|outcome| outcome.error.is_some())
        .count() as u64
        + u64::from(operator_runtime_shutdown_error.is_some());
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
            operator_finalization_failures_total,
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
            operator_finalization_failures_total,
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
    operator_outcomes: &[OperatorFinalizationOutcome],
    operator_runtime_shutdown_error: Option<&str>,
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

    for operator in operator_outcomes {
        if let Some(error) = &operator.error {
            push_finalization_failure(
                session_id,
                event_sender,
                &mut finalization_failures,
                SessionFinalizationStage::DrainOperator,
                SessionComponentId::Operator {
                    operator_instance_id: operator.operator_instance_id,
                },
                "finish_operator",
                error.clone(),
            );
        }
    }
    if let Some(error) = operator_runtime_shutdown_error {
        push_finalization_failure(
            session_id,
            event_sender,
            &mut finalization_failures,
            SessionFinalizationStage::DrainOperator,
            SessionComponentId::Runtime,
            "shutdown_operator_runtime",
            error,
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
    Vec<crate::session::SessionEndpointFailure>,
    Vec<SessionFinalizationFailure>,
) {
    let mut endpoint_failures = Vec::new();
    let mut finalization_failures = Vec::new();
    let Err(error) = result else {
        return (endpoint_failures, finalization_failures);
    };
    for (route_id, endpoint_id) in identities {
        let endpoint_failure =
            crate::session::SessionEndpointFailure::new(*route_id, *endpoint_id, error.clone());
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

#[cfg(test)]
mod selector_tests {
    use crate::capture::{CaptureMode, SourceKind, StableSourceId};
    use crate::frame::Platform;

    use super::capture_mode;
    use crate::session::{ApplicationSelector, ProcessId, Source};

    #[test]
    fn given_process_instance_selector_when_capture_mode_built_then_exact_identity_is_preserved() {
        let stable_id = StableSourceId::new(
            Platform::Windows,
            SourceKind::Application,
            "wasapi:pid:42:creation-100ns:133801234567890000",
        );
        let source = Source::application(ApplicationSelector::process_instance(
            ProcessId::new(42),
            stable_id.clone(),
        ));

        assert_eq!(
            capture_mode(&source),
            CaptureMode::ExactApplication {
                process_id: 42,
                stable_id,
            }
        );
    }
}

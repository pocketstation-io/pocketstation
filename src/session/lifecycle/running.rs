use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::capture::{
    capture_delivery_start_gate, prepare_capture_with_start_gate, CaptureDeliveryStartGate,
    CaptureError, CaptureLineageSeed, CaptureMode, CapturePrepareRequest, CaptureStopOutcome,
    InputDeviceSelector, SourceRuntimeEventReceive,
};
use crate::endpoint::{
    endpoint_start_gate, EndpointDriverObservations, EndpointDriverRegistry, EndpointFailure,
    EndpointFinalizationOutcome, EndpointPortInput, EndpointPrepareContext, EndpointRouteContext,
    RunningEndpoint, SessionTimelineOrigin,
};
use crate::frame::{ClockDomainId, EndpointId, RouteId, SessionId, StemId};
use crate::runtime::{
    AsyncOperatorNamedOutputBranchSpec, AsyncOperatorObservationHandle, AsyncOperatorObservations,
    AsyncOperatorOutput, AsyncOperatorTypedInput, AsyncOperatorWorker, AsyncRuntimeHost,
    CompiledOperatorInputContract, GeneratedAudioBridge, GeneratedAudioBridgeSpec,
    PlanRunnerDrainPolicy, PlanRunnerError, PlanRunnerFinishSummary, PlanSourceSendError,
    PlanSourceSendOutcome, RealtimePlanRunner, SessionOperatorInput, SidecarHost, SidecarHostError,
    SidecarHostSnapshot, SidecarMessage,
};

use crate::session::lifecycle::control::{
    validate_source_topology, validate_start_options, CaptureBackendSet, SessionStartCancellation,
    SessionStartError, SessionStartFailure, SessionStartOptions, SessionStopOutcome,
};
use crate::session::lifecycle::endpoint_setup::{
    prepare_endpoint_batches, prepare_endpoints, rollback_prepared_endpoints,
    PendingEndpointPortInput, PreparedEndpointBinding,
};
use crate::session::lifecycle::events::{session_event_channel, SessionEventSender};
use crate::session::lifecycle::operator_inputs::{
    OperatorInputObservation, OperatorInputObservationBinding,
};
use crate::session::lifecycle::rollback::StartupRollback;
use crate::session::lifecycle::telemetry::{
    DerivedRouteObservationBinding, FinalEndpointObservation, FinalOperatorObservation,
    IndexedSessionMetrics, RouteObservationBinding, SourceObservationBinding,
};
use crate::session::prepare::{
    PreparedExternalSourceMapping, PreparedExternalSourceTarget, PreparedOperatorInputMapping,
    PreparedOperatorMapping, PreparedOperatorOutputTarget, PreparedSignalRouteMapping,
};
use crate::session::{
    ApplicationSelector, DeviceSelector, EndpointObservationStage, OperatorInstanceId,
    PreparedSession, PreparedSourceRuntime, SessionAudioReentryMetrics, SessionComponentId,
    SessionControlFailure, SessionDerivedRouteMetrics, SessionEventReceiver,
    SessionExternalSourceMetrics, SessionFinalizationFailure, SessionFinalizationStage,
    SessionLifecycleState, SessionOperatorInputMetrics, SessionOperatorMetrics,
    SessionRollbackFailure, SessionRollbackStage, SessionRouteMetrics, SessionSidecarMetrics,
    SessionSourceFailure, SessionSourceMetrics, SessionTerminalOutcome, SessionTraceRecorderHandle,
    Source, SourceOutputBranchSpec, SourceOutputIdentity, SourceRegistry, SourceRuntime,
    SourceRuntimeObservationHandle, SourceSessionContext,
};

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

struct RunningEndpointBinding {
    identities: Vec<(RouteId, EndpointId)>,
    endpoint: RunningEndpoint,
}

struct PendingSignalEndpointInput {
    mapping: PreparedSignalRouteMapping,
    output: AsyncOperatorOutput,
}

struct PreparedOperatorRuntime {
    instance_id: OperatorInstanceId,
    worker: AsyncOperatorWorker,
    input_edges: Vec<OperatorInputObservationBinding>,
    observations: AsyncOperatorObservationHandle,
    lifecycle_timeout: Duration,
    signal_inputs: Vec<PendingSignalEndpointInput>,
    generated_audio_bridges: Vec<GeneratedAudioBridge>,
}

struct RunningOperatorBinding {
    instance_id: OperatorInstanceId,
    worker: AsyncOperatorWorker,
    input_edges: Vec<OperatorInputObservationBinding>,
    observations: AsyncOperatorObservationHandle,
    lifecycle_timeout: Duration,
    generated_audio_bridges: Vec<GeneratedAudioBridge>,
}

struct PendingOperatorTypedInput {
    operator_instance_id: OperatorInstanceId,
    input_port: String,
    input: AsyncOperatorTypedInput,
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

type SignalEndpointPreparationResult = Result<
    (
        Vec<PreparedEndpointBinding>,
        Vec<DerivedRouteObservationBinding>,
    ),
    (SessionStartError, Vec<SessionRollbackFailure>),
>;

struct OperatorFinalizationOutcome {
    operator_instance_id: OperatorInstanceId,
    input_edge: crate::runtime::EdgeObservations,
    input_ports: Box<[SessionOperatorInputMetrics]>,
    observations: AsyncOperatorObservations,
    audio_reentries: Box<[SessionAudioReentryMetrics]>,
    error: Option<String>,
}

struct SidecarFinalizationOutcome {
    sidecar_id: u64,
    observations: SidecarHostSnapshot,
    error: Option<String>,
}

struct SessionFinalizationOutcomes<'a> {
    operators: &'a [OperatorFinalizationOutcome],
    operator_runtime_shutdown_error: Option<&'a str>,
    endpoints: &'a [(Vec<(RouteId, EndpointId)>, EndpointFinalizationOutcome)],
    sidecars: &'a [SidecarFinalizationOutcome],
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum OperatorTermination {
    Finish,
    Cancel,
}

pub struct RunningSession {
    session_id: SessionId,
    state: SessionLifecycleState,
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
    sidecars: Vec<SidecarHost>,
    route_observations: Vec<RouteObservationBinding>,
    derived_route_observations: Vec<DerivedRouteObservationBinding>,
    final_operator_observations: Vec<FinalOperatorObservation>,
    final_endpoint_observations: Vec<FinalEndpointObservation>,
    final_external_source_observations: Vec<SessionExternalSourceMetrics>,
    final_audio_reentry_observations: Vec<SessionAudioReentryMetrics>,
    final_sidecar_observations: Vec<SessionSidecarMetrics>,
    stop_outcome: Option<SessionStopOutcome>,
}

impl RunningSession {
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    /// Returns the authoritative current lifecycle state owned by this Session.
    pub const fn state(&self) -> SessionLifecycleState {
        self.state
    }

    pub fn take_event_receiver(&mut self) -> Option<SessionEventReceiver> {
        self.event_receiver.take()
    }

    pub fn operator_metrics(&self) -> Box<[SessionOperatorMetrics]> {
        self.indexed_metrics_full().3
    }

    pub fn external_source_metrics(&self) -> Box<[SessionExternalSourceMetrics]> {
        self.indexed_metrics_full().1
    }

    pub fn derived_route_metrics(&self) -> Box<[SessionDerivedRouteMetrics]> {
        self.indexed_metrics_full().4
    }

    pub fn route_discarded_output_frames_total(&self, route_id: RouteId) -> Option<u64> {
        self.route_observations
            .iter()
            .find(|binding| binding.route_id == route_id)
            .map(|binding| binding.edge.discarded_output_frames_total())
    }

    pub fn audio_reentry_metrics(&self) -> Box<[SessionAudioReentryMetrics]> {
        if !self.operators.is_empty() {
            return self
                .operators
                .iter()
                .flat_map(|operator| {
                    operator.generated_audio_bridges.iter().map(|bridge| {
                        SessionAudioReentryMetrics::from_bridge(
                            operator.instance_id,
                            bridge.stem_id(),
                            bridge.observations().snapshot(),
                        )
                    })
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
        }
        self.final_audio_reentry_observations
            .clone()
            .into_boxed_slice()
    }

    pub fn sidecar_metrics(&self) -> Box<[SessionSidecarMetrics]> {
        if !self.sidecars.is_empty() {
            return self
                .sidecars
                .iter()
                .map(|sidecar| SessionSidecarMetrics {
                    sidecar_id: sidecar.id(),
                    host: sidecar.observations().snapshot(),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
        }
        self.final_sidecar_observations.clone().into_boxed_slice()
    }

    pub fn try_send_sidecar_signal(
        &self,
        sidecar_id: u64,
        message: SidecarMessage,
    ) -> Result<(), SidecarHostError> {
        self.sidecars
            .iter()
            .find(|sidecar| sidecar.id() == sidecar_id)
            .ok_or(SidecarHostError::UnknownSidecar(sidecar_id))?
            .try_send_signal(message)
    }

    pub fn try_receive_sidecar_signal(
        &self,
        sidecar_id: u64,
    ) -> Result<Option<SidecarMessage>, SidecarHostError> {
        self.sidecars
            .iter()
            .find(|sidecar| sidecar.id() == sidecar_id)
            .ok_or(SidecarHostError::UnknownSidecar(sidecar_id))?
            .try_receive_signal()
    }

    pub fn receive_sidecar_signal(
        &self,
        sidecar_id: u64,
    ) -> Result<SidecarMessage, SidecarHostError> {
        self.sidecars
            .iter()
            .find(|sidecar| sidecar.id() == sidecar_id)
            .ok_or(SidecarHostError::UnknownSidecar(sidecar_id))?
            .receive_signal()
    }

    pub(crate) fn attach_sidecars(&mut self, sidecars: Vec<SidecarHost>) {
        debug_assert!(self.sidecars.is_empty());
        self.sidecars = sidecars;
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
                            || OperatorInputObservationBinding::aggregate(&binding.input_edges),
                            |observation| observation.input_edge,
                        ),
                        input_ports: finalized.map_or_else(
                            || OperatorInputObservationBinding::per_port(&binding.input_edges),
                            |observation| observation.input_ports.clone(),
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
                            input_ports: observation.input_ports.clone(),
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
        self.state = SessionLifecycleState::Stopping;
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
        let sidecar_outcomes = self
            .sidecars
            .drain(..)
            .map(|mut sidecar| {
                let sidecar_id = sidecar.id();
                let result = match operator_termination {
                    OperatorTermination::Finish => sidecar.close_and_reap(),
                    OperatorTermination::Cancel => sidecar.cancel_and_reap(),
                };
                let error = match result {
                    Ok(status) if status.success() => None,
                    Ok(status) => Some(format!("sidecar exited with {status}")),
                    Err(error) => Some(error.to_string()),
                };
                SidecarFinalizationOutcome {
                    sidecar_id,
                    observations: sidecar.observations().snapshot(),
                    error,
                }
            })
            .collect::<Vec<_>>();
        self.final_sidecar_observations = sidecar_outcomes
            .iter()
            .map(|outcome| SessionSidecarMetrics {
                sidecar_id: outcome.sidecar_id,
                host: outcome.observations,
            })
            .collect();
        let sidecar_failures_total = sidecar_outcomes
            .iter()
            .filter(|outcome| outcome.error.is_some())
            .count() as u64;
        let worker = self.runtime_worker.take().map(JoinHandle::join);
        let (operator_outcomes, operator_runtime_shutdown_error) =
            self.async_runtime_host.take().map_or_else(
                || (Vec::new(), None),
                |host| {
                    let outcomes = terminate_operators(
                        &host,
                        std::mem::take(&mut self.operators),
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
                input_ports: outcome.input_ports.clone(),
                observations: outcome.observations,
                finalization_failures_total: u64::from(outcome.error.is_some()),
            })
            .collect();
        self.final_audio_reentry_observations = operator_outcomes
            .iter()
            .flat_map(|outcome| outcome.audio_reentries.iter().copied())
            .collect();
        let endpoint_shutdown = match operator_termination {
            OperatorTermination::Finish => crate::EndpointShutdownMode::Drain,
            OperatorTermination::Cancel => crate::EndpointShutdownMode::Abort,
        };
        for endpoint in &mut self.endpoints {
            let _ = endpoint.endpoint.request_shutdown(endpoint_shutdown);
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
                .saturating_add(external_source_failures_total)
                .saturating_add(sidecar_failures_total),
            ..outcome
        };
        publish_terminal_events(
            self.session_id,
            &self.event_sender,
            &worker,
            SessionFinalizationOutcomes {
                operators: &operator_outcomes,
                operator_runtime_shutdown_error: operator_runtime_shutdown_error.as_deref(),
                endpoints: &endpoint_outcomes,
                sidecars: &sidecar_outcomes,
            },
            outcome,
        );
        self.state = if outcome.is_success() {
            SessionLifecycleState::Stopped
        } else {
            SessionLifecycleState::Failed
        };
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

#[cfg(any(test, feature = "internal-testing"))]
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

#[cfg(any(test, feature = "internal-testing"))]
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
        pending_source_operator_inputs,
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
        Some(host) => match prepare_operator_runtimes(
            host,
            session_id,
            operator_mappings,
            pending_source_operator_inputs,
        ) {
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
    let (mut prepared_signal_endpoints, derived_route_observations) =
        match collect_and_prepare_operator_endpoint_inputs(
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
    while let Some(endpoint) = prepared_signal_endpoints.pop() {
        if start_cancellation.is_requested() {
            prepared_signal_endpoints.push(endpoint);
            let mut rollback = rollback_prepared_endpoints(prepared_signal_endpoints);
            let running_signal_endpoints = running_endpoints.split_off(raw_endpoint_count);
            rollback.append(rollback_running_endpoints(running_signal_endpoints));
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
                let mut rollback = rollback_prepared_endpoints(prepared_signal_endpoints);
                let running_signal_endpoints = running_endpoints.split_off(raw_endpoint_count);
                rollback.append(rollback_running_endpoints(running_signal_endpoints));
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
            input_edges: operator.input_edges,
            observations: operator.observations,
            lifecycle_timeout: operator.lifecycle_timeout,
            generated_audio_bridges: operator.generated_audio_bridges,
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
                let running_signal_endpoints = running_endpoints.split_off(raw_endpoint_count);
                let mut rollback = rollback_running_endpoints(running_signal_endpoints);
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
        let running_signal_endpoints = running_endpoints.split_off(raw_endpoint_count);
        let mut rollback = rollback_running_endpoints(running_signal_endpoints);
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
        state: SessionLifecycleState::Running,
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
        sidecars: Vec::new(),
        route_observations,
        derived_route_observations: derived_route_observations
            .into_iter()
            .chain(external_route_observations)
            .collect(),
        final_operator_observations: Vec::new(),
        final_endpoint_observations: Vec::new(),
        final_external_source_observations: Vec::new(),
        final_audio_reentry_observations: Vec::new(),
        final_sidecar_observations: Vec::new(),
        stop_outcome: None,
    })
}

type ExternalSourcePreparation = (
    Vec<PreparedExternalRuntimeBinding>,
    Vec<GeneratedAudioBridge>,
    Vec<PreparedEndpointBinding>,
    Vec<DerivedRouteObservationBinding>,
    Vec<PendingOperatorTypedInput>,
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
    let mut operator_inputs = Vec::new();
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
                let rollback = rollback_prepared_endpoints(std::mem::take(&mut endpoints));
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
                    let context = EndpointPrepareContext::new(
                        session_id,
                        route.endpoint_id,
                        EndpointRouteContext::from_source(
                            route.route_id,
                            route.source_id,
                            route.stream_id,
                            None,
                        ),
                        session_timeline_origin,
                        route.node_configuration,
                    )
                    .with_connector_id(route.connector_id);
                    let endpoint = match endpoint_registry.prepare_batch(
                        &route.endpoint_operator_id,
                        &route.endpoint_node_type_id,
                        vec![EndpointPortInput::signal(
                            route.input_port,
                            route.signal_spec,
                            route.media,
                            route.edge_contract,
                            receiver.receiver,
                            context,
                        )],
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
                PreparedExternalSourceTarget::OperatorInput(input) => {
                    operator_inputs.push(PendingOperatorTypedInput {
                        operator_instance_id: input.operator_instance_id,
                        input_port: input.input_port.clone(),
                        input: AsyncOperatorTypedInput {
                            port_name: input.input_port,
                            receiver: receiver.receiver,
                            edge_id: Some(input.edge_id),
                            signal_spec: input.signal_spec,
                            media: input.media,
                            edge_contract: input.edge_contract,
                            capacity_signals: input.capacity_signals,
                        },
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
    Ok((
        sources,
        bridges,
        endpoints,
        route_observations,
        operator_inputs,
    ))
}

fn prepare_operator_runtimes(
    host: &AsyncRuntimeHost,
    session_id: SessionId,
    operator_mappings: Vec<PreparedOperatorMapping>,
    mut pending_inputs: Vec<PendingOperatorTypedInput>,
) -> Result<Vec<PreparedOperatorRuntime>, (SessionStartError, Vec<SessionRollbackFailure>)> {
    let mut prepared = Vec::with_capacity(operator_mappings.len());
    let typed_contracts = operator_mappings
        .iter()
        .flat_map(|mapping| {
            mapping.inputs.iter().filter_map(|input| match input {
                PreparedOperatorInputMapping::Typed {
                    edge_id,
                    input_port,
                    signal_spec,
                    media,
                    edge_contract,
                    capacity_signals,
                    ..
                } => Some((
                    (mapping.instance_id, input_port.clone()),
                    (
                        *edge_id,
                        signal_spec.clone(),
                        *media,
                        *edge_contract,
                        *capacity_signals,
                    ),
                )),
                PreparedOperatorInputMapping::Compiled { .. } => None,
            })
        })
        .collect::<std::collections::HashMap<_, _>>();
    let mut remaining = operator_mappings;
    while !remaining.is_empty() {
        let ready_index = remaining.iter().position(|mapping| {
            mapping.inputs.iter().all(|input| match input {
                PreparedOperatorInputMapping::Compiled { .. } => true,
                PreparedOperatorInputMapping::Typed { input_port, .. } => {
                    pending_inputs.iter().any(|pending| {
                        pending.operator_instance_id == mapping.instance_id
                            && pending.input_port == *input_port
                    })
                }
            })
        });
        let Some(ready_index) = ready_index else {
            let instance_id = remaining[0].instance_id;
            let rollback = rollback_operator_runtimes(host, prepared);
            return Err((
                SessionStartError::OperatorPrepare {
                    operator_instance_id: instance_id,
                    message: "operator dependency topology produced no runnable instance"
                        .to_owned(),
                    rollback_failures_total: rollback.failures_total(),
                },
                rollback.failures,
            ));
        };
        let mapping = remaining.remove(ready_index);
        let PreparedOperatorMapping {
            node_id,
            instance_id,
            factory,
            node_configuration,
            inputs,
            mut outputs,
        } = mapping;
        outputs.sort_by_key(|output| {
            factory
                .manifest()
                .output_ports()
                .position(|port| port.name == output.output_port)
                .unwrap_or(usize::MAX)
        });
        let mut worker_inputs = Vec::with_capacity(inputs.len());
        let mut input_edges = Vec::with_capacity(inputs.len());
        for input in inputs {
            match input {
                PreparedOperatorInputMapping::Compiled {
                    stem_id,
                    input_port,
                    signal_spec,
                    media,
                    edge_contract,
                    capacity_signals,
                    receiver,
                } => {
                    input_edges.push(OperatorInputObservationBinding {
                        port_name: input_port.clone(),
                        observation: OperatorInputObservation::Plan(receiver.observation_handle()),
                    });
                    worker_inputs.push(SessionOperatorInput::Compiled {
                        contract: CompiledOperatorInputContract {
                            edge_id: receiver.edge_id(),
                            operator_node: node_id,
                            session_id,
                            stem_id,
                            source_id: None,
                            input_port,
                            signal_spec,
                            media,
                            edge_contract,
                            capacity_signals,
                        },
                        receiver,
                    });
                }
                PreparedOperatorInputMapping::Typed {
                    edge_id,
                    input_port,
                    signal_spec,
                    media,
                    edge_contract,
                    capacity_signals,
                    origin,
                } => {
                    let Some(index) = pending_inputs.iter().position(|pending| {
                        pending.operator_instance_id == instance_id
                            && pending.input_port == input_port
                    }) else {
                        let rollback = rollback_operator_runtimes(host, prepared);
                        return Err((
                            SessionStartError::OperatorPrepare {
                                operator_instance_id: instance_id,
                                message: format!(
                                    "compiled typed input '{input_port}' from {origin:?} was not produced"
                                ),
                                rollback_failures_total: rollback.failures_total(),
                            },
                            rollback.failures,
                        ));
                    };
                    let pending = pending_inputs.remove(index);
                    if pending.input.edge_id != Some(edge_id)
                        || pending.input.signal_spec != signal_spec
                        || pending.input.media != media
                        || pending.input.edge_contract != edge_contract
                        || pending.input.capacity_signals != capacity_signals
                    {
                        let rollback = rollback_operator_runtimes(host, prepared);
                        return Err((
                            SessionStartError::OperatorPrepare {
                                operator_instance_id: instance_id,
                                message: format!(
                                    "typed input '{input_port}' disagrees with compiled edge {edge_id:?} from {origin:?}"
                                ),
                                rollback_failures_total: rollback.failures_total(),
                            },
                            rollback.failures,
                        ));
                    }
                    input_edges.push(OperatorInputObservationBinding {
                        port_name: input_port,
                        observation: OperatorInputObservation::Typed(
                            pending.input.receiver.observation_handle(),
                        ),
                    });
                    worker_inputs.push(SessionOperatorInput::Typed(pending.input));
                }
            }
        }
        let lifecycle_timeout = Duration::from_millis(
            u64::from(factory.manifest().deadline.process_timeout_ms).saturating_add(250),
        );
        let output_branches = outputs
            .iter()
            .map(|output| (output.output_port.clone(), output.branch))
            .collect::<Vec<_>>();
        let factory_for_spawn = Arc::clone(&factory);
        let configuration_for_spawn = node_configuration.clone();
        let result = host.execute(lifecycle_timeout, async move {
            let specifications = output_branches
                .iter()
                .map(|(output_port, branch)| AsyncOperatorNamedOutputBranchSpec {
                    output_port,
                    branch: *branch,
                })
                .collect::<Vec<_>>();
            AsyncOperatorWorker::prepare_and_spawn_session_composed(
                factory_for_spawn,
                &configuration_for_spawn,
                worker_inputs,
                &specifications,
            )
            .await
        });
        let (worker, runtime_outputs) = match result {
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
        if runtime_outputs.len() != outputs.len() {
            let observations = worker.observations();
            let mut incomplete = vec![PreparedOperatorRuntime {
                instance_id,
                worker,
                input_edges,
                observations,
                lifecycle_timeout,
                signal_inputs: Vec::new(),
                generated_audio_bridges: Vec::new(),
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
        let mut signal_inputs = Vec::new();
        let mut generated_audio_bridges = Vec::new();
        for (mapping, output) in outputs.into_iter().zip(runtime_outputs) {
            if mapping.output_port != output.output_port {
                let mut incomplete = vec![PreparedOperatorRuntime {
                    instance_id,
                    worker,
                    input_edges,
                    observations,
                    lifecycle_timeout,
                    signal_inputs,
                    generated_audio_bridges,
                }];
                incomplete.append(&mut prepared);
                let rollback = rollback_operator_runtimes(host, incomplete);
                return Err((
                    SessionStartError::OperatorPrepare {
                        operator_instance_id: instance_id,
                        message: "operator output port order did not match compiled routes"
                            .to_owned(),
                        rollback_failures_total: rollback.failures_total(),
                    },
                    rollback.failures,
                ));
            }
            match mapping.target {
                PreparedOperatorOutputTarget::SignalEndpoint(mapping) => {
                    signal_inputs.push(PendingSignalEndpointInput {
                        mapping: *mapping,
                        output: output.receiver,
                    });
                }
                PreparedOperatorOutputTarget::OperatorInput {
                    operator_instance_id,
                    input_port,
                } => {
                    let Some((edge_id, signal_spec, media, edge_contract, capacity_signals)) =
                        typed_contracts
                            .get(&(operator_instance_id, input_port.clone()))
                            .cloned()
                    else {
                        let mut incomplete = vec![PreparedOperatorRuntime {
                            instance_id,
                            worker,
                            input_edges,
                            observations,
                            lifecycle_timeout,
                            signal_inputs,
                            generated_audio_bridges,
                        }];
                        incomplete.append(&mut prepared);
                        let rollback = rollback_operator_runtimes(host, incomplete);
                        return Err((
                            SessionStartError::OperatorPrepare {
                                operator_instance_id,
                                message: format!(
                                    "compiled downstream typed input '{input_port}' has no edge contract"
                                ),
                                rollback_failures_total: rollback.failures_total(),
                            },
                            rollback.failures,
                        ));
                    };
                    pending_inputs.push(PendingOperatorTypedInput {
                        operator_instance_id,
                        input_port: input_port.clone(),
                        input: AsyncOperatorTypedInput {
                            port_name: input_port,
                            receiver: output.receiver,
                            edge_id: Some(edge_id),
                            signal_spec,
                            media,
                            edge_contract,
                            capacity_signals,
                        },
                    });
                }
                PreparedOperatorOutputTarget::GeneratedAudio(generated) => {
                    let bridge = match GeneratedAudioBridge::spawn(
                        output.receiver,
                        generated.sender,
                        GeneratedAudioBridgeSpec {
                            session_id,
                            stem_id: generated.stem_id,
                            stream_id: generated.stream_id,
                            source_id: generated.source_id,
                            clock_id: ClockDomainId(0),
                            sample_spec: generated.sample_spec,
                            samples_per_frame: generated.samples_per_frame,
                            pool_slots: mapping.branch.capacity_signals.clamp(1, 64),
                        },
                    ) {
                        Ok(bridge) => bridge,
                        Err(error) => {
                            let mut incomplete = vec![PreparedOperatorRuntime {
                                instance_id,
                                worker,
                                input_edges,
                                observations,
                                lifecycle_timeout,
                                signal_inputs,
                                generated_audio_bridges,
                            }];
                            incomplete.append(&mut prepared);
                            let rollback = rollback_operator_runtimes(host, incomplete);
                            return Err((
                                SessionStartError::GeneratedAudioBridge {
                                    message: error.to_string(),
                                    rollback_failures_total: rollback.failures_total(),
                                },
                                rollback.failures,
                            ));
                        }
                    };
                    generated_audio_bridges.push(bridge);
                }
            }
        }
        prepared.push(PreparedOperatorRuntime {
            instance_id,
            worker,
            input_edges,
            observations,
            lifecycle_timeout,
            signal_inputs,
            generated_audio_bridges,
        });
    }
    if !pending_inputs.is_empty() {
        let instance_id = pending_inputs[0].operator_instance_id;
        let rollback = rollback_operator_runtimes(host, prepared);
        return Err((
            SessionStartError::OperatorPrepare {
                operator_instance_id: instance_id,
                message: "Session retained an unconsumed operator input receiver".to_owned(),
                rollback_failures_total: rollback.failures_total(),
            },
            rollback.failures,
        ));
    }
    Ok(prepared)
}

fn collect_and_prepare_operator_endpoint_inputs(
    spec: &crate::session::SessionSpec,
    operators: &mut [PreparedOperatorRuntime],
    endpoint_registry: &EndpointDriverRegistry,
    session_timeline_origin: SessionTimelineOrigin,
) -> SignalEndpointPreparationResult {
    let session_id = spec.session_id();
    let signal_inputs = operators
        .iter_mut()
        .flat_map(|operator| std::mem::take(&mut operator.signal_inputs))
        .collect::<Vec<_>>();
    let mut pending = Vec::with_capacity(signal_inputs.len());
    for input in signal_inputs {
        let mapping = input.mapping;
        let route_context = match (mapping.signal_origin, mapping.stem_id) {
            (Some((source_id, stream_id)), stem_id) => {
                EndpointRouteContext::from_source(mapping.route_id, source_id, stream_id, stem_id)
            }
            (None, Some(stem_id)) => EndpointRouteContext::from_stem(mapping.route_id, stem_id),
            (None, None) => EndpointRouteContext::signal(mapping.route_id),
        };
        let context = EndpointPrepareContext::new(
            session_id,
            mapping.endpoint_id,
            route_context,
            session_timeline_origin,
            mapping.node_configuration,
        )
        .with_connector_id(mapping.connector_id);
        let signal_observation = input.output.observation_handle();
        pending.push(PendingEndpointPortInput {
            route_id: mapping.route_id,
            endpoint_id: mapping.endpoint_id,
            input: EndpointPortInput::signal(
                mapping.input_port,
                mapping.signal_spec,
                mapping.output_branch.edge_contract.media,
                mapping.output_branch.edge_contract,
                input.output,
                context,
            ),
            signal_observation: Some(signal_observation),
        });
    }
    prepare_endpoint_batches(spec, pending, endpoint_registry)
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
                        if let PlanSourceSendOutcome::Rejected { error, frame } =
                            sender.try_send(frame)
                        {
                            drop(frame);
                            match error {
                                PlanSourceSendError::Cancelled | PlanSourceSendError::Full => {
                                    source_send_rejections_total =
                                        source_send_rejections_total.saturating_add(1);
                                }
                            }
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

fn rollback_operator_runtimes(
    host: &AsyncRuntimeHost,
    operators: Vec<PreparedOperatorRuntime>,
) -> StartupRollback {
    let mut rollback = StartupRollback::default();
    for operator in operators.into_iter().rev() {
        drop(operator.signal_inputs);
        for bridge in operator.generated_audio_bridges {
            bridge.cancel_and_join();
        }
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
    mut operators: Vec<RunningOperatorBinding>,
    termination: OperatorTermination,
) -> Vec<OperatorFinalizationOutcome> {
    if termination == OperatorTermination::Cancel {
        operators.reverse();
    }
    operators
        .into_iter()
        .map(|operator| {
            let input_edge = OperatorInputObservationBinding::aggregate(&operator.input_edges);
            let input_ports = OperatorInputObservationBinding::per_port(&operator.input_edges);
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
            let audio_reentries = operator
                .generated_audio_bridges
                .into_iter()
                .map(|bridge| {
                    let stem_id = bridge.stem_id();
                    let observations = bridge.observations();
                    match termination {
                        OperatorTermination::Finish => bridge.finish_and_join(),
                        OperatorTermination::Cancel => bridge.cancel_and_join(),
                    }
                    SessionAudioReentryMetrics::from_bridge(
                        operator.instance_id,
                        stem_id,
                        observations.snapshot(),
                    )
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
            OperatorFinalizationOutcome {
                operator_instance_id: operator.instance_id,
                input_edge,
                input_ports,
                observations: operator.observations.snapshot(),
                audio_reentries,
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
        for bridge in operator.generated_audio_bridges {
            bridge.cancel_and_join();
        }
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
    finalization: SessionFinalizationOutcomes<'_>,
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

    for operator in finalization.operators {
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
    if let Some(error) = finalization.operator_runtime_shutdown_error {
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

    for (identities, endpoint_outcome) in finalization.endpoints {
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

    for sidecar in finalization.sidecars {
        if let Some(error) = &sidecar.error {
            push_finalization_failure(
                session_id,
                event_sender,
                &mut finalization_failures,
                SessionFinalizationStage::DrainSidecar,
                SessionComponentId::Sidecar {
                    sidecar_id: sidecar.sidecar_id,
                },
                "close_and_reap_sidecar",
                error.clone(),
            );
        }
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

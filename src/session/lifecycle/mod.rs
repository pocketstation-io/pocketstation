//! Session engine construction, transactional startup, observations, and stop.

mod control;
mod endpoint_setup;
mod engine;
mod events;
mod host;
mod observations;
mod operator_inputs;
mod rollback;
mod running;
mod telemetry;
mod trace;

pub use control::{
    CaptureBackendSet, SessionStartCancellation, SessionStartError, SessionStartFailure,
    SessionStartOptions, SessionStopOutcome,
};
pub use engine::{
    EndpointExtensionRegistrationError, SessionEngine, SessionEngineBuildError,
    SessionEngineBuilder, SessionEngineStartError,
};
pub use events::{
    SessionComponentId, SessionControlFailure, SessionEndpointFailure, SessionEvent,
    SessionEventKind, SessionEventReceive, SessionEventReceiver, SessionFinalizationFailure,
    SessionFinalizationStage, SessionLifecycleState, SessionRollbackFailure, SessionRollbackStage,
    SessionSourceFailure, SessionTerminalOutcome, SessionTerminalState,
};
pub use host::{
    NativeSessionEngineHostOptions, SessionEngineHost, SessionEngineHostBuildError,
    SessionEngineHostBuilder,
};
pub use observations::{
    EndpointObservationStage, RouteLatencyMeasurement, SessionAudioReentryMetrics,
    SessionDerivedRouteMetrics, SessionEventQueueObservations, SessionExternalSourceMetrics,
    SessionMetricsSnapshot, SessionOperatorInputMetrics, SessionOperatorMetrics,
    SessionRouteDropObservations, SessionRouteLatencyBoundary, SessionRouteLatencyObservations,
    SessionRouteLatencyUnit, SessionRouteMetrics, SessionRouteObservationInterval,
    SessionSidecarMetrics, SessionSourceMetrics,
};
pub(crate) use running::start_prepared_session_cancellable_with_trace;
pub use running::RunningSession;
#[cfg(any(test, feature = "internal-testing"))]
pub use running::{start_prepared_session, start_prepared_session_cancellable};
pub use trace::{
    SessionTrace, SessionTraceRecorder, SessionTraceRecorderFinishError,
    SessionTraceRecorderHandle, SessionTraceRecorderOutcome, SessionTraceRecorderStartError,
    SessionTraceValidation, SessionTraceValidationError,
};
pub use trace::{SessionTraceRecord, SessionTraceRecordKind, SessionTraceTerminal};

#[cfg(test)]
mod tests {
    mod engine;
    mod running;
}

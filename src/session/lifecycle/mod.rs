//! Session engine construction, transactional startup, observations, and stop.

mod endpoint_transaction;
mod engine;
mod events;
mod host;
mod metric_bindings;
mod observations;
mod operator_observations;
mod rollback;
mod running;
mod start_contract;
mod trace;

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
    EndpointObservationStage, SessionAudioReentryMetrics, SessionDerivedRouteMetrics,
    SessionEventQueueObservations, SessionExternalSourceMetrics, SessionMetricsSnapshot,
    SessionOperatorInputMetrics, SessionOperatorMetrics, SessionRouteDropObservations,
    SessionRouteLatencyBoundary, SessionRouteLatencyObservations, SessionRouteLatencyUnit,
    SessionRouteMetrics, SessionRouteObservationInterval, SessionSidecarMetrics,
    SessionSourceMetrics,
};
pub(crate) use running::start_prepared_session_cancellable_with_trace;
pub use running::RunningSession;
#[cfg(any(test, feature = "internal-testing"))]
pub use running::{start_prepared_session, start_prepared_session_cancellable};
pub use start_contract::{
    CaptureBackendSet, SessionStartCancellation, SessionStartError, SessionStartFailure,
    SessionStartOptions, SessionStopOutcome,
};
pub use trace::{
    SessionTrace, SessionTraceRecorder, SessionTraceRecorderFinishError,
    SessionTraceRecorderHandle, SessionTraceRecorderOutcome, SessionTraceRecorderStartError,
    SessionTraceValidation, SessionTraceValidationError,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use trace::{SessionTraceRecord, SessionTraceRecordKind, SessionTraceTerminal};

#[cfg(test)]
mod tests {
    mod engine;
    mod running;
}

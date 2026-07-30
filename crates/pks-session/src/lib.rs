mod compiler;
mod draft;
mod endpoint;
mod engine;
#[cfg(test)]
mod engine_tests;
mod error;
mod error_code;
mod events;
mod foreign_audio;
mod foreign_audio_endpoint;
mod host;
mod observations;
mod recording;
mod running;
#[cfg(test)]
mod running_tests;
mod runtime_prepare;
mod selector;
mod spec;
mod structural_nodes;

pub use compiler::{
    CompiledSession, OperatorRegistry, OperatorRegistryError, SessionCompileError, SessionCompiler,
    APPLICATION_SOURCE_NODE_TYPE_ID, BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID,
    CONNECTOR_NODE_TYPE_ID, DEFAULT_MULTISTEM_RECORDING_GROUP_ID, MICROPHONE_SOURCE_NODE_TYPE_ID,
    RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID, RECORDING_GROUP_CONFIGURATION_KEY,
};
pub use draft::{EndpointHandle, Session, StemHandle};
pub use endpoint::{EndpointConfiguration, EndpointDescriptor, OperatorId};
pub use engine::{
    SessionEngine, SessionEngineBuildError, SessionEngineBuilder, SessionEngineRegistrationError,
    SessionEngineStartError,
};
pub use error::SessionError;
pub use error_code::{
    polled_audio_poll_error_code, session_declaration_error_code, session_start_failure_code,
    session_stop_failure_codes, PolledAudioPollErrorCode, SessionDeclarationErrorCode,
    SessionRuntimeErrorCode, SessionStartErrorCode, SessionStopCode, SessionStopFailureCode,
};
pub use events::{
    SessionComponentId, SessionControlFailure, SessionEndpointFailure, SessionEvent,
    SessionEventKind, SessionEventReceive, SessionEventReceiver, SessionFinalizationFailure,
    SessionFinalizationStage, SessionLifecycleState, SessionRollbackFailure, SessionRollbackStage,
    SessionSourceFailure, SessionTerminalOutcome, SessionTerminalState,
};
pub use foreign_audio::{
    PolledAudioBatchLease, PolledAudioEndpoint, PolledAudioEndpointConfig,
    PolledAudioEndpointConfigError, PolledAudioFrame, PolledAudioObservations,
    PolledAudioPollError, PolledAudioReceipt, POLLED_AUDIO_OPERATOR_ID,
};
pub use host::{
    NativeSessionEngineHostOptions, SessionEngineHost, SessionEngineHostBuildError,
    SessionEngineHostBuilder,
};
pub use observations::{
    EndpointObservationStage, SessionEventQueueObservations, SessionMetricsSnapshot,
    SessionRouteMetrics, SessionSourceMetrics,
};
pub use recording::{
    session_recording_outcome_error_code, SessionRecordingErrorCode, SessionRecordingObservations,
    SessionRecordingOutcome, SessionRecordingReceipt, SessionRecordingState,
    SessionRecordingStemOutcome,
};
pub use running::{
    start_prepared_session, start_prepared_session_cancellable, CaptureBackendSet, RunningSession,
    SessionStartCancellation, SessionStartError, SessionStartFailure, SessionStartOptions,
    SessionStopOutcome,
};
pub use runtime_prepare::{
    prepare_session_runtime, PreparedSession, PreparedSourceMapping, PreparedWorkerMapping,
    SessionPrepareError,
};
pub use selector::{ApplicationSelector, DeviceId, DeviceSelector, ProcessId, Source};
pub use spec::{
    EndpointSpec, RouteSpec, SessionSpec, SessionSpecVersion, StemSpec, SESSION_SPEC_VERSION,
};
pub use structural_nodes::{
    register_session_structural_nodes, SessionStructuralNodeRegistrationError,
};

pub use pks_frame::{ConnectorId, EndpointId, RouteId, SessionId, StemId};
pub use pks_graph::NodeTypeId;

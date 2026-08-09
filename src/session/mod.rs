mod compiler;
mod draft;
mod endpoint;
mod engine;
#[cfg(test)]
mod engine_tests;
mod error;
mod error_code;
mod events;
#[cfg(test)]
mod external_source;
#[cfg(test)]
mod external_source_lifecycle;
mod foreign_audio;
mod foreign_audio_endpoint;
mod host;
mod observations;
#[cfg(test)]
mod operator_connections;
mod recording;
mod running;
#[cfg(test)]
mod running_tests;
mod runtime_prepare;
mod selector;
mod session_trace;
mod source_extension;
#[cfg(test)]
mod source_registration;
mod spec;
mod structural_nodes;
mod typed_stream;

pub use compiler::{
    CompiledSession, SessionCompileError, SessionCompiler, APPLICATION_SOURCE_NODE_TYPE_ID,
    BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID,
    DEFAULT_MULTISTEM_RECORDING_GROUP_ID, MICROPHONE_SOURCE_NODE_TYPE_ID, RECORDER_NODE_TYPE_ID,
    RECORDER_OPERATOR_ID, RECORDING_GROUP_CONFIGURATION_KEY,
};
pub use draft::{
    DerivedStreamHandle, EndpointHandle, Operator, OperatorInputHandle, OperatorInstanceHandle,
    Session, SourceInstanceHandle, SourceOutputHandle, StemHandle,
};
pub use endpoint::{EndpointConfiguration, EndpointDescriptor, OperatorId};
pub use engine::{
    SessionEngine, SessionEngineBuildError, SessionEngineBuilder, SessionEngineStartError,
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
    EndpointObservationStage, SessionDerivedRouteMetrics, SessionEventQueueObservations,
    SessionExternalSourceMetrics, SessionMetricsSnapshot, SessionOperatorInputMetrics,
    SessionOperatorMetrics, SessionRouteDropObservations, SessionRouteLatencyBoundary,
    SessionRouteLatencyObservations, SessionRouteLatencyUnit, SessionRouteMetrics,
    SessionRouteObservationInterval, SessionSourceMetrics,
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
pub use session_trace::{
    SessionTrace, SessionTraceRecord, SessionTraceRecordKind, SessionTraceRecorder,
    SessionTraceRecorderFinishError, SessionTraceRecorderHandle, SessionTraceRecorderOutcome,
    SessionTraceRecorderStartError, SessionTraceTerminal, SessionTraceValidation,
    SessionTraceValidationError,
};
pub use source_extension::{
    PreparedSourceRuntime, SourceCancellation, SourceConfiguration, SourceDriver,
    SourceDriverError, SourceEmission, SourceFactory, SourceManifest, SourceManifestError,
    SourceOutputBranchSpec, SourceOutputIdentity, SourceOutputReceiver, SourcePrepareContext,
    SourceRegistrationError, SourceRegistry, SourceRuntime, SourceRuntimeError,
    SourceRuntimeObservationHandle, SourceRuntimeObservations, SourceSessionContext, SourceTypeId,
};
#[allow(deprecated)]
pub use spec::{
    DerivedRouteSpec, EndpointSpec, OperatorConnectionSpec, OperatorInputOrigin,
    OperatorInstanceId, OperatorInstanceSpec, OperatorSpec, RouteSpec, SessionSpec,
    SessionSpecVersion, SourceInstanceId, SourceInstanceSpec, SourceOutputSpec, SourceRouteSpec,
    StemSpec, SESSION_SPEC_VERSION,
};
pub use structural_nodes::{
    register_session_structural_nodes, SessionStructuralNodeRegistrationError,
};
pub use typed_stream::{Stream, StreamSignal, TypedOperator, TypedStreamError};

pub use crate::frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, ConnectorId, EncryptionMode, EndpointId,
    FrameLineage, RouteId, SampleFormat, SampleSpec, SessionId, SourceId, StemId, StreamId,
};
pub use crate::graph::{
    AsyncNode, AsyncNodeFuture, AsyncOperatorFactory, AsyncOperatorManifest,
    AsyncOperatorManifestError, AudioCaps, BackpressurePolicy, BinaryFormat, ChannelLayout,
    ClockDomain, Codec, ConfigError, CopyPolicy, DeliverySemantics, EdgeContract,
    EdgeObservabilityLevel, EventFormat, ExecutionPartition, LossPolicy, MediaCaps, MediaKind,
    Multiplicity, NodeConfig as OperatorConfiguration, NodeDefinition, NodeDescriptor, NodeError,
    NodeRegistrationError, NodeTypeId, OperatorCancellationPolicy, OperatorDeadlinePolicy,
    OperatorFailurePolicy, OperatorOutputRolePolicy, OperatorPermissionPolicy, PortDirection,
    PortSpec, PrepareContext, SafetyContract, SchemaRef, SemanticRole, SignalClass,
    SignalContinuityError, SignalContinuityObservation, SignalContinuityTracker, SignalDerivation,
    SignalDerivationError, SignalEnvelope, SignalEnvelopeError, SignalId, SignalLineage,
    SignalPayload, SignalSpec, SignalSpecError, SignalTiming, TextFormat,
};
pub use crate::runtime::{AsyncOperatorOutputObservationHandle, AsyncOperatorOutputObservations};

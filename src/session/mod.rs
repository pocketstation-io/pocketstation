mod compile;
pub(crate) mod declaration;
pub(crate) mod error;
pub(crate) mod error_code;
pub(crate) mod extensions;
pub(crate) mod lifecycle;
mod prepare;

pub use crate::endpoint::{
    PolledAudioBatchLease, PolledAudioEndpoint, PolledAudioEndpointConfig,
    PolledAudioEndpointConfigError, PolledAudioFrame, PolledAudioObservations,
    PolledAudioPollError, PolledAudioReceipt,
};
pub use compile::{
    CompiledSession, SessionCompileDiagnostic, SessionCompileError, SessionCompiler,
};
pub use declaration::{ApplicationSelector, DeviceSelector, Source};
pub use declaration::{
    ConnectionSpec, ConnectionTarget, EndpointSpec, OperatorInstanceId, SessionSpec,
    SourceInstanceId, SourceOutputSpec, StreamOrigin,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use declaration::{
    DerivedStreamHandle, OperatorInputHandle, OperatorInstanceHandle, SourceInstanceHandle,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use declaration::{DeviceId, ProcessId};
pub use declaration::{
    EndpointConfiguration, EndpointDescriptor, OperatorId, BROWSER_NODE_TYPE_ID,
    BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID,
};
pub use declaration::{EndpointHandle, Operator, Session, SourceOutputHandle, StemHandle};
#[cfg(any(test, feature = "internal-testing"))]
pub use declaration::{OperatorInstanceSpec, StemSpec};
#[cfg(any(test, feature = "internal-testing"))]
#[allow(deprecated)]
pub use declaration::{OperatorSpec, SessionSpecVersion, SESSION_SPEC_VERSION};
#[cfg(any(test, feature = "internal-testing"))]
pub use declaration::{Stream, StreamSignal, TypedOperator, TypedStreamError};
pub use error::SessionError;
#[cfg(any(test, feature = "internal-testing"))]
pub use error_code::{
    polled_audio_poll_error_code, session_declaration_error_code, session_start_failure_code,
    PolledAudioPollErrorCode, SessionDeclarationErrorCode, SessionStartErrorCode,
};
pub use error_code::{
    session_stop_failure_codes, SessionRuntimeErrorCode, SessionStopCode, SessionStopFailureCode,
};
pub use extensions::SessionGraphRegistrationError;
#[cfg(any(test, feature = "internal-testing"))]
pub use extensions::{
    register_session_graph_nodes, APPLICATION_SOURCE_NODE_TYPE_ID, MICROPHONE_SOURCE_NODE_TYPE_ID,
};
pub use extensions::{
    session_recording_outcome_error_code, SessionRecordingErrorCode, SessionRecordingObservations,
    SessionRecordingState, SessionRecordingStemOutcome,
};
pub use extensions::{
    PreparedSourceRuntime, SourceConfiguration, SourceOutputBranchSpec, SourceOutputIdentity,
    SourceRegistry, SourceRuntime, SourceRuntimeObservationHandle, SourceRuntimeObservations,
    SourceSessionContext, SourceTypeId,
};
pub use extensions::{SessionRecordingOutcome, SessionRecordingReceipt};
#[cfg(any(test, feature = "internal-testing"))]
pub use extensions::{
    SourceCancellation, SourceDriver, SourceDriverError, SourceEmission, SourceFactory,
    SourceManifest, SourceManifestError, SourceOutputReceiver, SourcePrepareContext,
    SourceRegistrationError, SourceRuntimeError,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use extensions::{
    DEFAULT_MULTISTEM_RECORDING_GROUP_ID, RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
    RECORDING_GROUP_CONFIGURATION_KEY,
};
pub use lifecycle::SessionTraceRecorderHandle;
#[cfg(any(test, feature = "internal-testing"))]
pub use lifecycle::{start_prepared_session, start_prepared_session_cancellable};
pub use lifecycle::{
    CaptureBackendSet, RunningSession, SessionStartCancellation, SessionStartError,
    SessionStartFailure, SessionStartOptions, SessionStopOutcome,
};
pub use lifecycle::{
    EndpointExtensionRegistrationError, SessionEngine, SessionEngineBuildError,
    SessionEngineBuilder, SessionEngineStartError,
};
pub use lifecycle::{
    EndpointObservationStage, SessionAudioReentryMetrics, SessionDerivedRouteMetrics,
    SessionExternalSourceMetrics, SessionMetricsSnapshot, SessionOperatorInputMetrics,
    SessionOperatorMetrics, SessionRouteMetrics, SessionSidecarMetrics, SessionSourceMetrics,
};
pub use lifecycle::{
    NativeSessionEngineHostOptions, SessionEngineHost, SessionEngineHostBuildError,
    SessionEngineHostBuilder,
};
pub use lifecycle::{
    SessionComponentId, SessionControlFailure, SessionEndpointFailure, SessionEvent,
    SessionEventKind, SessionEventReceive, SessionEventReceiver, SessionFinalizationFailure,
    SessionFinalizationStage, SessionLifecycleState, SessionRollbackFailure, SessionRollbackStage,
    SessionSourceFailure, SessionTerminalOutcome, SessionTerminalState,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use lifecycle::{
    SessionEventQueueObservations, SessionRouteDropObservations, SessionRouteLatencyBoundary,
    SessionRouteLatencyObservations, SessionRouteLatencyUnit, SessionRouteObservationInterval,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use lifecycle::{
    SessionTrace, SessionTraceRecorder, SessionTraceRecorderFinishError,
    SessionTraceRecorderOutcome, SessionTraceRecorderStartError, SessionTraceValidation,
    SessionTraceValidationError,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use lifecycle::{SessionTraceRecord, SessionTraceRecordKind, SessionTraceTerminal};
#[cfg(any(test, feature = "internal-testing"))]
pub use prepare::PreparedSourceMapping;
#[cfg(any(test, feature = "internal-testing"))]
pub use prepare::PreparedWorkerMapping;
pub use prepare::{prepare_session_runtime, PreparedSession, SessionPrepareError};

#[cfg(any(test, feature = "internal-testing"))]
pub use crate::frame::ConnectorId;
#[cfg(any(test, feature = "internal-testing"))]
pub use crate::frame::{
    AudioBufferPool, AudioFrame, AudioFrameBuildError, ClockDomainId, FrameLineage,
    FrameLineageBuildError, SourceId, StreamId,
};
pub use crate::frame::{EndpointId, RouteId, SampleFormat, SampleSpec, SessionId, StemId};
#[cfg(any(test, feature = "internal-testing"))]
pub use crate::graph::NodeRegistrationError;
#[cfg(any(test, feature = "internal-testing"))]
pub use crate::graph::{
    AsyncNode, AsyncNodeFuture, AsyncOperatorFactory, AsyncOperatorManifest,
    AsyncOperatorManifestError, AudioCaps, BackpressurePolicy, BinaryFormat, ChannelLayout,
    ClockDomain, Codec, ConfigError, CopyPolicy, DeliverySemantics, EdgeContract,
    EdgeObservabilityLevel, EventFormat, ExecutionPartition, LossPolicy, MediaCaps, MediaKind,
    Multiplicity, NodeDefinition, NodeDescriptor, NodeError, OperatorCancellationPolicy,
    OperatorDeadlinePolicy, OperatorFailurePolicy, OperatorOutputRolePolicy,
    OperatorPermissionPolicy, PortDirection, PortPrepareContext, PortSpec, PrepareContext,
    SafetyContract, SchemaRef, SemanticRole, SignalClass, SignalContinuityError,
    SignalContinuityObservation, SignalContinuityTracker, SignalDerivation, SignalDerivationError,
    SignalEnvelope, SignalEnvelopeError, SignalId, SignalLineage, SignalPayload, SignalSpec,
    SignalSpecError, SignalTiming, TextFormat,
};
pub use crate::graph::{NodeConfig as OperatorConfiguration, NodeTypeId};

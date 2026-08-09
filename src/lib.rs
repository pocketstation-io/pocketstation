//! PocketStation's source-aware desktop audio SDK.

// Internal module APIs replace the former cross-crate public surfaces. A
// normal consumer cannot reach them, so rustc would classify those retained
// implementation/test entry points as dead. Repository validation enables
// `internal-testing`, where every such surface is compiled and linted with
// warnings denied.
#![cfg_attr(not(feature = "internal-testing"), allow(dead_code, unused_imports))]

mod abi;
mod capture;
mod codec;
mod dsp;
pub mod endpoint;
mod frame;
mod graph;
mod recording;
mod runtime;
mod session;
mod timing;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[cfg(feature = "conformance-fixtures")]
pub mod conformance;
mod error_code;

pub use crate::session::{
    polled_audio_poll_error_code, session_declaration_error_code,
    session_recording_outcome_error_code, session_start_failure_code, PolledAudioPollErrorCode,
    SessionDeclarationErrorCode, SessionRuntimeErrorCode, SessionStartErrorCode, SessionStopCode,
    SessionStopFailureCode,
};

pub use crate::capture::{
    CallbackCaptureBackend, CaptureAuthorizationSnapshot, CaptureCapabilityState,
    CaptureOpenOutcome, CapturePermissionLifecycle, CapturePermissionTransition, CaptureScope,
    CaptureSessionGrant, PermissionEpoch, PermissionObservation, SourceLifecycleEventKind,
};

/// Reads the current microphone authorization state without prompting.
///
/// macOS exposes an authoritative query. Other desktop backends currently
/// return `NotObservable`; callers must not reinterpret that value as allowed
/// or denied. Permission prompting remains an explicit host-application action.
pub fn microphone_permission_observation() -> PermissionObservation {
    #[cfg(target_os = "macos")]
    {
        crate::capture::platform::macos::microphone_permission_observation()
    }
    #[cfg(not(target_os = "macos"))]
    {
        PermissionObservation::NotObservable
    }
}
pub use crate::endpoint::{
    DerivedEndpointDriverInput, EndpointCancellationOutcome, EndpointDriverFactory,
    EndpointDriverFinalization, EndpointDriverInput, EndpointDriverObservations, EndpointFailure,
    EndpointFailureStage, EndpointFinalizationOutcome, EndpointPrepareContext,
    EndpointRouteContext, EndpointSignalRouteContext, EndpointStartFailure,
    EndpointStartFailureCause, EndpointStartGate, PreparedEndpointDriver, RunningEndpointDriver,
    SessionTimelineOrigin,
};
pub use crate::graph::{
    AsyncOperatorEdgePrepareContext, AsyncOperatorPrepareContext, NodeConfig, NodeId,
};
pub use crate::runtime::{
    plan_source_channel, AsyncOperatorInput, AsyncOperatorNamedOutput,
    AsyncOperatorNamedOutputBranchSpec, AsyncOperatorTypedInput, AsyncOperatorWorker,
    AsyncOperatorWorkerError, GeneratedAudioBridge, GeneratedAudioBridgeObservationHandle,
    GeneratedAudioBridgeObservations, GeneratedAudioBridgeSpec, GeneratedAudioBridgeStartError,
    PlanRunnerCancellation, PlanSourceSender, SidecarMessage, SidecarMessageKind,
    SidecarProtocolError, SidecarProtocolLimits, TypedEdgeBranchSpec, TypedEdgeBuildError,
    TypedEdgeFanout, TypedEdgeObservationHandle, TypedEdgeObservations, TypedEdgePublishError,
    TypedEdgePublishReport, TypedEdgeReceiver, SIDECAR_PROTOCOL_MAJOR, SIDECAR_PROTOCOL_MINOR,
};
pub use crate::session::{
    ApplicationSelector, AsyncNode, AsyncNodeFuture, AsyncOperatorFactory, AsyncOperatorManifest,
    AsyncOperatorManifestError, AsyncOperatorOutputObservationHandle,
    AsyncOperatorOutputObservations, AudioBufferPool, AudioCaps, AudioFrame, BackpressurePolicy,
    BinaryFormat, ChannelLayout, ClockDomain, ClockDomainId, Codec, ConfigError, CopyPolicy,
    DeliverySemantics, DerivedStreamHandle, DeviceId, DeviceSelector, EdgeContract,
    EdgeObservabilityLevel, EncryptionMode, EndpointConfiguration, EndpointDescriptor,
    EndpointHandle, EventFormat, ExecutionPartition, FrameLineage, LossPolicy, MediaCaps,
    MediaKind, Multiplicity, NodeDefinition, NodeDescriptor, NodeError, NodeTypeId, Operator,
    OperatorCancellationPolicy, OperatorConfiguration, OperatorDeadlinePolicy,
    OperatorFailurePolicy, OperatorId, OperatorInputOrigin, OperatorOutputRolePolicy,
    OperatorPermissionPolicy, PortDirection, PortSpec, PrepareContext, PreparedSourceRuntime,
    ProcessId, SafetyContract, SampleFormat, SampleSpec, SchemaRef, SemanticRole,
    SessionControlFailure, SessionDerivedRouteMetrics, SessionError, SessionEvent,
    SessionEventKind, SessionEventQueueObservations, SessionEventReceive,
    SessionExternalSourceMetrics, SessionId, SessionLifecycleState, SessionMetricsSnapshot,
    SessionOperatorMetrics, SessionRecordingErrorCode, SessionRecordingObservations,
    SessionRecordingOutcome, SessionRecordingState, SessionRecordingStemOutcome,
    SessionRouteDropObservations, SessionRouteLatencyBoundary, SessionRouteLatencyObservations,
    SessionRouteLatencyUnit, SessionRouteMetrics, SessionRouteObservationInterval,
    SessionSourceMetrics, SessionStartCancellation, SessionStopOutcome, SessionTerminalState,
    SessionTrace, SessionTraceRecorder, SessionTraceRecorderFinishError,
    SessionTraceRecorderOutcome, SessionTraceRecorderStartError, SessionTraceValidation,
    SessionTraceValidationError, SignalClass, SignalContinuityError, SignalContinuityObservation,
    SignalContinuityTracker, SignalDerivation, SignalDerivationError, SignalEnvelope,
    SignalEnvelopeError, SignalId, SignalLineage, SignalPayload, SignalSpec, SignalSpecError,
    SignalTiming, Source, SourceCancellation, SourceConfiguration, SourceDriver, SourceDriverError,
    SourceEmission, SourceFactory, SourceId, SourceInstanceHandle, SourceInstanceId,
    SourceInstanceSpec, SourceManifest, SourceManifestError, SourceOutputBranchSpec,
    SourceOutputHandle, SourceOutputIdentity, SourceOutputReceiver, SourceOutputSpec,
    SourcePrepareContext, SourceRegistrationError, SourceRegistry, SourceRouteSpec, SourceRuntime,
    SourceRuntimeError, SourceRuntimeObservationHandle, SourceRuntimeObservations,
    SourceSessionContext, SourceTypeId, StemHandle, StemId, Stream, StreamId, StreamSignal,
    TextFormat, TypedOperator, TypedStreamError,
};

/// Canonical types required by an external asynchronous Operator package.
///
/// Provider implementations depend on this public module instead of importing
/// PocketStation's internal graph or runtime crates.
pub mod operator {
    pub use crate::graph::{
        AsyncOperatorEdgePrepareContext, AsyncOperatorPrepareContext, NodeConfig,
    };
    pub use crate::runtime::{
        AsyncOperatorInput, AsyncOperatorNamedOutput, AsyncOperatorNamedOutputBranchSpec,
        AsyncOperatorTypedInput, AsyncOperatorWorker, AsyncOperatorWorkerError,
    };
    pub use crate::session::{
        AsyncNode, AsyncNodeFuture, AsyncOperatorFactory, AsyncOperatorManifest,
        AsyncOperatorManifestError, AsyncOperatorOutputObservationHandle,
        AsyncOperatorOutputObservations, AudioBufferPool, AudioCaps, AudioFrame,
        BackpressurePolicy, BinaryFormat, ChannelLayout, ClockDomain, ClockDomainId, Codec,
        ConfigError, CopyPolicy, DeliverySemantics, EdgeContract, EdgeObservabilityLevel,
        EncryptionMode, EventFormat, ExecutionPartition, FrameLineage, LossPolicy, MediaCaps,
        MediaKind, Multiplicity, NodeDefinition, NodeDescriptor, NodeError, NodeTypeId, Operator,
        OperatorCancellationPolicy, OperatorConfiguration, OperatorDeadlinePolicy,
        OperatorFailurePolicy, OperatorId, OperatorOutputRolePolicy, OperatorPermissionPolicy,
        PortDirection, PortSpec, PrepareContext, SafetyContract, SampleFormat, SampleSpec,
        SchemaRef, SemanticRole, SessionId, SignalClass, SignalContinuityError,
        SignalContinuityObservation, SignalContinuityTracker, SignalDerivation,
        SignalDerivationError, SignalEnvelope, SignalEnvelopeError, SignalId, SignalLineage,
        SignalPayload, SignalSpec, SignalSpecError, SignalTiming, SourceId, StemId, StreamId,
        TextFormat,
    };
}

/// Non-product exports used only by PocketStation's own integration tests and
/// benchmarks. This module is absent from normal builds.
#[cfg(feature = "internal-testing")]
#[doc(hidden)]
pub mod internal {
    pub mod capture {
        pub use crate::capture::*;
        #[cfg(target_os = "linux")]
        pub mod linux {
            pub use crate::capture::platform::linux::*;
        }
        #[cfg(target_os = "macos")]
        pub mod macos {
            pub use crate::capture::platform::macos::*;
        }
        #[cfg(target_os = "windows")]
        pub mod windows {
            pub use crate::capture::platform::windows::*;
        }
    }
    pub mod codec {
        pub use crate::codec::*;
    }
    pub mod dsp {
        pub use crate::dsp::*;
    }
    pub mod frame {
        pub use crate::frame::*;
    }
    pub mod graph {
        pub use crate::graph::*;
        pub mod compiler {
            pub use crate::graph::compiler::*;
        }
        pub mod dsl {
            pub use crate::graph::dsl::*;
        }
        pub mod node {
            pub use crate::graph::node::*;
        }
        pub mod planner {
            pub use crate::graph::planner::*;
        }
        pub mod registry {
            pub use crate::graph::registry::*;
        }
    }
    pub mod runtime {
        pub use crate::runtime::*;
    }
    pub mod recording {
        pub use crate::recording::*;
    }
    pub mod session {
        pub use crate::session::*;
    }
    pub mod timing {
        pub use crate::timing::*;
    }
    pub mod endpoint {
        pub use crate::endpoint::*;
    }

    pub use capture::captured_frame_stream;
    pub use codec::{
        OpusApplication, OpusChannels, OpusConfig, OpusDecoder, OpusEncoder, OpusFrameDuration,
        OpusSampleRate, OPUS_FRAME_SAMPLES, OPUS_MAX_PACKET_BYTES,
    };
    pub use frame::{
        AudioBufferPool, AudioFrame, SourceId, StreamId, POOL_MAX_SLOTS, POOL_SLOT_SAMPLES,
    };
}

use crate::session::{
    NativeSessionEngineHostOptions, SessionEngineHost, SessionEngineHostBuildError,
    SessionEngineHostBuilder, SessionEngineStartError, BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID,
    CONNECTOR_NODE_TYPE_ID,
};

pub struct Session {
    declaration: crate::session::Session,
    host_builder: Option<SessionEngineHostBuilder>,
    polled_audio_endpoint: crate::session::PolledAudioEndpointConfig,
    recording_root: Option<PathBuf>,
    sample_spec: SampleSpec,
    endpoint_registrations: Mutex<Vec<EndpointDriverRegistration>>,
    endpoint_definitions: Mutex<Vec<Arc<dyn NodeDefinition>>>,
    operator_registrations: Mutex<Vec<Arc<dyn AsyncOperatorFactory>>>,
    source_registrations: Mutex<Vec<Arc<dyn SourceFactory>>>,
    capture_backends: Option<CaptureBackendConfiguration>,
    session_trace: Option<SessionTraceConfiguration>,
}

struct CaptureBackendConfiguration {
    application: Arc<dyn CallbackCaptureBackend>,
    microphone: Arc<dyn CallbackCaptureBackend>,
}

struct EndpointDriverRegistration {
    operator_id: OperatorId,
    node_type_id: crate::session::NodeTypeId,
    factory: Arc<dyn EndpointDriverFactory>,
}

struct SessionTraceConfiguration {
    path: PathBuf,
    capacity_records: usize,
}

/// Setup-time configuration for the public Rust Session.
pub struct SessionBuilder {
    recording_root: Option<PathBuf>,
    sample_spec: SampleSpec,
    capture_backends: Option<CaptureBackendConfiguration>,
    session_trace: Option<SessionTraceConfiguration>,
}

impl Default for SessionBuilder {
    fn default() -> Self {
        Self {
            recording_root: None,
            sample_spec: SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
            capture_backends: None,
            session_trace: None,
        }
    }
}

impl SessionBuilder {
    /// Configures the artifact root used by declared multistem recording routes.
    #[must_use]
    pub fn recording_root(mut self, output_root: impl Into<PathBuf>) -> Self {
        self.recording_root = Some(output_root.into());
        self
    }

    /// Declares the exact canonical PCM format produced by the configured
    /// capture backends and consumed by compiled Session routes.
    #[must_use]
    pub fn sample_spec(mut self, sample_spec: SampleSpec) -> Self {
        self.sample_spec = sample_spec;
        self
    }

    /// Uses caller-owned capture backends while retaining the canonical
    /// Session compiler, runtime, endpoint lifecycle, and recording ownership.
    #[must_use]
    pub fn capture_backends(
        mut self,
        application: Arc<dyn CallbackCaptureBackend>,
        microphone: Arc<dyn CallbackCaptureBackend>,
    ) -> Self {
        self.capture_backends = Some(CaptureBackendConfiguration {
            application,
            microphone,
        });
        self
    }

    /// Enables the bounded Session Session trace recorder.
    ///
    /// The output path must not exist. Records are transferred to a dedicated
    /// non-realtime writer through a bounded queue; overflow remains explicit
    /// and makes deterministic replay fail closed.
    #[must_use]
    pub fn session_trace(mut self, path: impl Into<PathBuf>, capacity_records: usize) -> Self {
        self.session_trace = Some(SessionTraceConfiguration {
            path: path.into(),
            capacity_records,
        });
        self
    }

    /// Builds the Session declaration owner.
    #[must_use]
    pub fn build(self) -> Session {
        Session {
            declaration: crate::session::Session::new(),
            host_builder: None,
            polled_audio_endpoint: crate::session::PolledAudioEndpointConfig::default(),
            recording_root: self.recording_root,
            sample_spec: self.sample_spec,
            endpoint_registrations: Mutex::new(Vec::new()),
            endpoint_definitions: Mutex::new(Vec::new()),
            operator_registrations: Mutex::new(Vec::new()),
            source_registrations: Mutex::new(Vec::new()),
            capture_backends: self.capture_backends,
            session_trace: self.session_trace,
        }
    }
}

impl Session {
    pub fn new() -> Self {
        Self {
            declaration: crate::session::Session::new(),
            host_builder: None,
            polled_audio_endpoint: crate::session::PolledAudioEndpointConfig::default(),
            recording_root: None,
            sample_spec: SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
            endpoint_registrations: Mutex::new(Vec::new()),
            endpoint_definitions: Mutex::new(Vec::new()),
            operator_registrations: Mutex::new(Vec::new()),
            source_registrations: Mutex::new(Vec::new()),
            capture_backends: None,
            session_trace: None,
        }
    }

    pub fn builder() -> SessionBuilder {
        SessionBuilder::default()
    }

    pub fn id(&self) -> crate::session::SessionId {
        self.declaration.id()
    }

    pub fn capture(&self, source: Source) -> Result<StemHandle, SessionError> {
        self.declaration.capture(source)
    }

    /// Declares one instance of an open external source type.
    ///
    /// The selected output names are validated against the registered source
    /// manifest when this Session is compiled by the canonical engine.
    pub fn source(
        &self,
        source_type_id: SourceTypeId,
        configuration: SourceConfiguration,
    ) -> Result<SourceInstanceHandle, SessionError> {
        self.declaration.source(source_type_id, configuration)
    }

    /// Retains an external source factory for this Session's canonical engine.
    pub fn register_source(
        &self,
        factory: Arc<dyn SourceFactory>,
    ) -> Result<(), SessionSourceError> {
        self.source_registrations
            .lock()
            .map_err(|_| SessionSourceError::RegistrationStateUnavailable)?
            .push(factory);
        Ok(())
    }

    pub fn endpoint(&self, descriptor: EndpointDescriptor) -> Result<EndpointHandle, SessionError> {
        self.declaration.endpoint(descriptor)
    }

    pub fn register_operator(
        &self,
        factory: Arc<dyn AsyncOperatorFactory>,
    ) -> Result<(), SessionOperatorError> {
        self.operator_registrations
            .lock()
            .map_err(|_| SessionOperatorError::RegistrationStateUnavailable)?
            .push(factory);
        Ok(())
    }

    pub fn polled_audio(&self) -> Result<EndpointHandle, SessionError> {
        self.declaration.polled_audio()
    }

    /// Declares an external connector. Register its implementation after route
    /// identities are available with [`Self::register_connector_driver`].
    pub fn connector(
        &self,
        operator_id: OperatorId,
        configuration: EndpointConfiguration,
    ) -> Result<EndpointHandle, SessionError> {
        self.declaration.connector(operator_id, configuration)
    }

    /// Registers the externally owned implementation for a declared connector.
    pub fn register_connector_driver(
        &self,
        operator_id: OperatorId,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<(), SessionEndpointError> {
        self.register_endpoint_driver(
            operator_id,
            crate::session::NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
            factory,
        )
    }

    /// Declares a browser/remote receiver. Register its transport implementation
    /// with [`Self::register_browser_driver`].
    pub fn browser(&self, receiver_uri: impl Into<String>) -> Result<EndpointHandle, SessionError> {
        self.declaration.browser(receiver_uri)
    }

    /// Registers the externally owned browser/remote transport implementation.
    pub fn register_browser_driver(
        &self,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<(), SessionEndpointError> {
        self.register_endpoint_driver(
            OperatorId::new(BROWSER_OPERATOR_ID),
            crate::session::NodeTypeId::from(BROWSER_NODE_TYPE_ID),
            factory,
        )
    }

    pub fn register_endpoint_driver(
        &self,
        operator_id: OperatorId,
        node_type_id: NodeTypeId,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<(), SessionEndpointError> {
        let mut registrations = self
            .endpoint_registrations
            .lock()
            .map_err(|_| SessionEndpointError::RegistrationStateUnavailable)?;
        registrations.push(EndpointDriverRegistration {
            operator_id,
            node_type_id,
            factory,
        });
        Ok(())
    }

    /// Registers the compiler-visible contract for an externally owned typed
    /// endpoint. The endpoint package owns both this definition and its driver;
    /// PocketStation owns validation and lifecycle execution.
    pub fn register_endpoint_definition(
        &self,
        definition: Arc<dyn NodeDefinition>,
    ) -> Result<(), SessionEndpointError> {
        self.endpoint_definitions
            .lock()
            .map_err(|_| SessionEndpointError::RegistrationStateUnavailable)?
            .push(definition);
        Ok(())
    }

    pub fn start(self) -> Result<RunningSession, SessionStartError> {
        self.start_cancellable(SessionStartCancellation::default())
    }

    pub fn start_cancellable(
        self,
        cancellation: SessionStartCancellation,
    ) -> Result<RunningSession, SessionStartError> {
        let Self {
            declaration,
            host_builder,
            polled_audio_endpoint,
            recording_root,
            sample_spec,
            endpoint_registrations,
            endpoint_definitions,
            operator_registrations,
            source_registrations,
            capture_backends,
            session_trace,
        } = self;
        let mut session_trace_recorder = session_trace
            .map(|configuration| {
                SessionTraceRecorder::start(
                    configuration.path,
                    declaration.id(),
                    configuration.capacity_records,
                )
            })
            .transpose()?;
        let recording_declared = declaration
            .declares_multistem_recording()
            .map_err(|error| {
                SessionStartError::Engine(crate::session::SessionEngineStartError::Freeze(error))
            })?;
        let mut host_builder = match host_builder {
            Some(builder) => builder,
            None => match capture_backends {
                Some(backends) => SessionEngineHostBuilder::with_capture_backends(
                    NativeSessionEngineHostOptions {
                        sample_spec,
                        ..NativeSessionEngineHostOptions::default()
                    },
                    backends.application,
                    backends.microphone,
                )?,
                None => SessionEngineHostBuilder::native(NativeSessionEngineHostOptions {
                    sample_spec,
                    ..NativeSessionEngineHostOptions::default()
                })?,
            },
        };
        let _ = host_builder.register_polled_audio_endpoint(polled_audio_endpoint)?;
        let endpoint_definitions = endpoint_definitions
            .into_inner()
            .map_err(|_| SessionStartError::EndpointRegistrationStateUnavailable)?;
        for definition in endpoint_definitions {
            let _ = host_builder.register_endpoint_definition(definition)?;
        }
        let endpoint_registrations = endpoint_registrations
            .into_inner()
            .map_err(|_| SessionStartError::EndpointRegistrationStateUnavailable)?;
        for registration in endpoint_registrations {
            let _ = host_builder.register_endpoint_driver(
                registration.operator_id,
                registration.node_type_id,
                registration.factory,
            )?;
        }
        let operator_registrations = operator_registrations
            .into_inner()
            .map_err(|_| SessionStartError::OperatorRegistrationStateUnavailable)?;
        for factory in operator_registrations {
            let _ = host_builder.register_async_operator(factory)?;
        }
        let source_registrations = source_registrations
            .into_inner()
            .map_err(|_| SessionStartError::SourceRegistrationStateUnavailable)?;
        for factory in source_registrations {
            let _ = host_builder
                .engine_builder()
                .register_source_factory(factory)?;
        }
        if let Some(recorder) = &session_trace_recorder {
            let _ = host_builder
                .engine_builder()
                .set_session_trace(recorder.handle());
        }
        if let Some(output_root) = recording_root.filter(|root| !root.as_os_str().is_empty()) {
            let _ = host_builder.register_multistem_recording(output_root)?;
        }
        let host = host_builder.build()?;
        let recording_receipt = host.recording_receipt(0);
        if recording_declared && recording_receipt.is_none() {
            return Err(SessionStartError::MissingRecordingConfiguration);
        }
        let compiled = host.compile(declaration)?;
        let receipt = host
            .polled_audio_receipt(0)
            .ok_or(SessionStartError::MissingAudioReceipt)?;
        let mut running = host.start_compiled_cancellable(compiled, cancellation)?;
        let Some(events) = running.take_event_receiver() else {
            let _ = running.stop();
            return Err(SessionStartError::MissingEventReceiver);
        };
        Ok(RunningSession {
            host,
            running,
            events,
            receipt,
            recording_receipt,
            session_trace_recorder: session_trace_recorder.take(),
            session_trace_result: None,
            stopped: false,
        })
    }

    #[cfg(feature = "conformance-fixtures")]
    fn with_host_builder(
        host_builder: SessionEngineHostBuilder,
        polled_audio_endpoint: crate::session::PolledAudioEndpointConfig,
        recording_root: Option<PathBuf>,
    ) -> Result<Self, SessionEngineHostBuildError> {
        Ok(Self {
            declaration: crate::session::Session::new(),
            host_builder: Some(host_builder),
            polled_audio_endpoint,
            recording_root,
            sample_spec: SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
            endpoint_registrations: Mutex::new(Vec::new()),
            endpoint_definitions: Mutex::new(Vec::new()),
            operator_registrations: Mutex::new(Vec::new()),
            source_registrations: Mutex::new(Vec::new()),
            capture_backends: None,
            session_trace: None,
        })
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RunningSession {
    host: SessionEngineHost,
    running: crate::session::RunningSession,
    events: crate::session::SessionEventReceiver,
    receipt: crate::session::PolledAudioReceipt,
    recording_receipt: Option<crate::session::SessionRecordingReceipt>,
    session_trace_recorder: Option<SessionTraceRecorder>,
    session_trace_result:
        Option<Result<SessionTraceRecorderOutcome, SessionTraceRecorderFinishError>>,
    stopped: bool,
}

impl RunningSession {
    pub fn session_id(&self) -> crate::session::SessionId {
        self.running.session_id()
    }

    pub fn try_poll_audio(&self) -> Result<PolledAudioBatchLease, PolledAudioPollError> {
        self.receipt.try_poll()
    }

    pub fn audio_observations(&self) -> PolledAudioObservations {
        self.receipt.observations()
    }

    pub fn recording_outcome(&self) -> Option<&crate::session::SessionRecordingOutcome> {
        self.recording_receipt
            .as_ref()
            .and_then(crate::session::SessionRecordingReceipt::outcome)
    }

    pub fn try_recv_event(&self) -> SessionEventReceive {
        self.events.try_recv()
    }

    pub fn event_observations(&self) -> SessionEventQueueObservations {
        self.events.observations()
    }

    pub fn metrics_snapshot(&self) -> Result<SessionMetricsSnapshot, SessionRuntimeError> {
        self.host
            .metrics_snapshot(&self.events, 0, Some(&self.running))
            .ok_or(SessionRuntimeError::MissingMetricsSnapshot)
    }

    pub fn session_trace_outcome(
        &self,
    ) -> Option<Result<&SessionTraceRecorderOutcome, &SessionTraceRecorderFinishError>> {
        self.session_trace_result
            .as_ref()
            .map(|result| result.as_ref())
    }

    pub fn stop(&mut self) -> SessionStopResult {
        let disposition = if self.stopped {
            SessionStopDisposition::AlreadyStopped
        } else {
            self.stopped = true;
            SessionStopDisposition::Stopped
        };
        let outcome = self.running.stop();
        self.finish_session_trace();
        SessionStopResult {
            disposition,
            outcome,
        }
    }

    /// Cancels active asynchronous Operators, then finalizes capture, runtime,
    /// endpoints, and recording through the same bounded Session authority.
    pub fn cancel(&mut self) -> SessionCancelResult {
        let disposition = if self.stopped {
            SessionCancelDisposition::AlreadyStopped
        } else {
            self.stopped = true;
            SessionCancelDisposition::Cancelled
        };
        let outcome = self.running.cancel();
        self.finish_session_trace();
        SessionCancelResult {
            disposition,
            outcome,
        }
    }

    fn finish_session_trace(&mut self) {
        if self.session_trace_result.is_some() {
            return;
        }
        let Some(recorder) = &mut self.session_trace_recorder else {
            return;
        };
        self.session_trace_result = Some(recorder.finish().cloned());
    }
}

pub use crate::session::{
    PolledAudioBatchLease, PolledAudioFrame, PolledAudioObservations, PolledAudioPollError,
};

#[derive(Debug, thiserror::Error)]
pub enum SessionStartError {
    #[error("Session session trace setup failed: {0}")]
    TraceRecorder(#[from] SessionTraceRecorderStartError),
    #[error("native Session host setup failed: {0}")]
    Host(#[from] SessionEngineHostBuildError),
    #[error("canonical Session start failed: {0}")]
    Engine(#[from] SessionEngineStartError),
    #[error("native Session host did not retain its bounded audio receipt")]
    MissingAudioReceipt,
    #[error("recording routes require an explicit Session recording root")]
    MissingRecordingConfiguration,
    #[error("canonical running Session did not retain its event receiver")]
    MissingEventReceiver,
    #[error("Session endpoint-registration state became unavailable before start")]
    EndpointRegistrationStateUnavailable,
    #[error("Session operator-registration state became unavailable before start")]
    OperatorRegistrationStateUnavailable,
    #[error("Session source-registration state became unavailable before start")]
    SourceRegistrationStateUnavailable,
    #[error("Session source registration failed: {0}")]
    SourceRegistration(#[from] SourceRegistrationError),
}

impl SessionStartError {
    pub fn kind(&self) -> SessionStartErrorKind {
        match self {
            Self::Host(_) | Self::TraceRecorder(_) => SessionStartErrorKind::Host,
            Self::Engine(error)
                if matches!(
                    error.start_failure().map(|failure| failure.error()),
                    Some(crate::session::SessionStartError::Cancelled { .. })
                ) =>
            {
                SessionStartErrorKind::Cancelled
            }
            Self::Engine(SessionEngineStartError::Freeze(SessionError::InvalidSelector {
                ..
            })) => SessionStartErrorKind::InvalidSelector,
            Self::Engine(_) => SessionStartErrorKind::Engine,
            Self::MissingRecordingConfiguration => {
                SessionStartErrorKind::MissingRecordingConfiguration
            }
            Self::MissingAudioReceipt
            | Self::MissingEventReceiver
            | Self::EndpointRegistrationStateUnavailable
            | Self::OperatorRegistrationStateUnavailable
            | Self::SourceRegistrationStateUnavailable => SessionStartErrorKind::Invariant,
            Self::SourceRegistration(_) => SessionStartErrorKind::Engine,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.kind() == SessionStartErrorKind::Cancelled
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionEndpointError {
    #[error("Session endpoint-registration state is unavailable")]
    RegistrationStateUnavailable,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionOperatorError {
    #[error("Session operator-registration state is unavailable")]
    RegistrationStateUnavailable,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionSourceError {
    #[error("Session source-registration state is unavailable")]
    RegistrationStateUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStartErrorKind {
    Host,
    Engine,
    Cancelled,
    InvalidSelector,
    MissingRecordingConfiguration,
    Invariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SessionRuntimeError {
    #[error("native running Session did not expose a metrics snapshot")]
    MissingMetricsSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStopDisposition {
    Stopped,
    AlreadyStopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionCancelDisposition {
    Cancelled,
    AlreadyStopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionCancelResult {
    disposition: SessionCancelDisposition,
    outcome: SessionStopOutcome,
}

impl SessionCancelResult {
    pub fn disposition(self) -> SessionCancelDisposition {
        self.disposition
    }

    pub fn outcome(self) -> SessionStopOutcome {
        self.outcome
    }

    pub fn is_success(self) -> bool {
        self.outcome.is_success()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionStopResult {
    disposition: SessionStopDisposition,
    outcome: SessionStopOutcome,
}

impl SessionStopResult {
    pub fn disposition(self) -> SessionStopDisposition {
        self.disposition
    }

    pub fn outcome(self) -> SessionStopOutcome {
        self.outcome
    }

    pub fn is_success(self) -> bool {
        self.outcome.is_success()
    }
}

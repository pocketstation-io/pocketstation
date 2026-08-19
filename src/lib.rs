#![doc = include_str!("../README.md")]

mod abi;
mod capture;
pub mod codec;
pub mod connector;
mod endpoint;
mod frame;
/// Stable signal, port, capability, partition, and extension contracts.
///
/// Compiler IR, registries, runtime plans, and execution machinery remain
/// private even though the contract namespace is public.
pub mod graph;
pub mod native_extension;
mod recording;
mod runtime;
mod secret;
mod session;
pub mod timing;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[cfg(feature = "conformance-fixtures")]
pub mod conformance;
mod error_code;

pub use crate::session::error_code::{
    polled_audio_poll_error_code, session_declaration_error_code, session_start_failure_code,
    PolledAudioPollErrorCode, SessionDeclarationErrorCode, SessionRuntimeErrorCode,
    SessionStartErrorCode, SessionStopCode, SessionStopFailureCode,
};

pub use crate::capture::{
    application_capture_available, discover_sources, resolve_query, ActiveCaptureBackend,
    CallbackCaptureBackend, CaptureAuthorizationSnapshot, CaptureCapabilityState, CaptureDelivery,
    CaptureError, CaptureMode, CaptureObservationHandle, CaptureObservations, CaptureOpenOutcome,
    CapturePermissionLifecycle, CapturePermissionTransition, CaptureRuntimeFailure,
    CaptureRuntimeFailureClass, CaptureScope, CaptureSessionGrant, CaptureSource,
    CapturedFrameDelivery, CapturedFrameObservationHandle, CapturedFrameSender,
    CapturedFrameStreamStats, InputDeviceSelector, LocalSourceProvider, PermissionEpoch,
    PermissionObservation, PreparedCaptureBackend, ProcessTreeScope, SelectorPersistenceScope,
    SourceGeneration, SourceIdentityStrength, SourceKind, SourceLifecycleEventKind, SourceProvider,
    SourceQuery, SourceRecoveryRequirement, SourceRuntimeEvent, SourceRuntimeEventDelivery,
    SourceRuntimeEventObservationHandle, SourceRuntimeEventObservations, SourceRuntimeEventSender,
    SourceState, StableSourceId,
};

/// Reads the current microphone authorization state without prompting.
///
/// macOS and supported Windows hosts expose authoritative non-prompting
/// queries. Linux currently returns `NotObservable`; callers must not
/// reinterpret that value as allowed or denied. Permission prompting remains
/// an explicit host-application action.
pub fn microphone_permission_observation() -> PermissionObservation {
    #[cfg(all(target_os = "macos", feature = "native-capture"))]
    {
        crate::capture::platform::macos::microphone_permission_observation()
    }
    #[cfg(all(target_os = "windows", feature = "native-capture"))]
    {
        crate::capture::platform::windows::microphone_permission_observation()
    }
    #[cfg(not(all(
        feature = "native-capture",
        any(target_os = "macos", target_os = "windows")
    )))]
    {
        PermissionObservation::NotObservable
    }
}
pub use crate::endpoint::{
    EndpointAudioFrame, EndpointAudioReceiver, EndpointCancellationOutcome, EndpointDriverFactory,
    EndpointDriverFinalization, EndpointDriverObservations, EndpointFailure,
    EndpointFailureRetryability, EndpointFailureStage, EndpointGroupId, EndpointInputOrigin,
    EndpointPortInput, EndpointPreparationGroup, EndpointPrepareContext, EndpointReceiver,
    EndpointRouteContext, EndpointShutdownMode, EndpointSignalReceiver, EndpointStartGate,
    PreparedEndpointDriver, RunningEndpointDriver, SessionTimelineOrigin,
};
pub use crate::frame::{
    AudioBufferPool, AudioFrame, AudioFrameBuildError, ClockDomainId, ConnectorId, EndpointId,
    FrameLineage, FrameLineageBuildError, RouteId, SampleFormat, SampleSpec, SessionId, SourceId,
    StemId, StreamId,
};
pub use crate::graph::{
    AsyncNode, AsyncNodeFuture, AsyncOperatorFactory, AsyncOperatorManifest,
    AsyncOperatorManifestError, AsyncOperatorPrepareContext, AudioCaps, BackpressurePolicy,
    BinaryFormat, ChannelLayout, ClockDomain, Codec, ConfigError, CopyPolicy, DeliverySemantics,
    EdgeContract, EdgeObservabilityLevel, EventFormat, ExecutionPartition, LossPolicy, MediaCaps,
    MediaKind, Multiplicity, NodeConfig as OperatorConfiguration, NodeDefinition, NodeDescriptor,
    NodeError, NodeTypeId, OperatorCancellationPolicy, OperatorDeadlinePolicy,
    OperatorFailurePolicy, OperatorId, OperatorOutputRolePolicy, OperatorPermissionPolicy,
    PortDirection, PortPrepareContext, PortSpec, SafetyContract, SchemaRef, SemanticRole,
    SignalClass, SignalContinuityError, SignalContinuityObservation, SignalContinuityTracker,
    SignalDerivation, SignalDerivationError, SignalEnvelope, SignalEnvelopeError, SignalId,
    SignalLineage, SignalLineageError, SignalPayload, SignalSpec, SignalSpecError, SignalTiming,
    SignalTimingError, TextFormat,
};
pub use crate::session::declaration::{
    ApplicationSelector, ConnectionSpec, ConnectionTarget, DerivedStreamHandle, DeviceId,
    DeviceSelector, EndpointConfiguration, EndpointDescriptor, EndpointHandle, Operator,
    OperatorInputHandle, OperatorInstanceHandle, OperatorInstanceId, OperatorInstanceSpec,
    ProcessId, Source, SourceInstanceHandle, SourceInstanceId, SourceInstanceSpec,
    SourceOutputHandle, SourceOutputSpec, StemHandle, Stream, StreamOrigin, StreamSignal,
    TypedOperator, TypedStreamError,
};
pub use crate::session::error::SessionError;
pub use crate::session::extensions::{
    AudioInput, AudioInputBuffer, AudioInputBufferAcquireError, AudioInputBufferError,
    AudioInputConfig, AudioInputConfigError, AudioInputError, AudioInputObservations,
    AudioInputWriteError, AudioInputWriteErrorKind, AudioInputWriter, PcmSource,
    SourceCancellation, SourceConfiguration, SourceDriver, SourceDriverError, SourceEmission,
    SourceFactory, SourceManifest, SourceManifestError, SourceOutputIdentity, SourcePrepareContext,
    SourceRegistrationError, SourceRuntimeObservations, SourceSessionContext, SourceTypeId,
    SourceTypeIdError, PCM_SOURCE_TYPE_ID,
};
pub use crate::session::lifecycle::{
    SessionAudioReentryMetrics, SessionControlFailure, SessionDerivedRouteMetrics, SessionEvent,
    SessionEventKind, SessionEventQueueObservations, SessionEventReceive,
    SessionExternalSourceMetrics, SessionLifecycleState, SessionMetricsSnapshot,
    SessionOperatorInputMetrics, SessionOperatorMetrics, SessionRouteDropObservations,
    SessionRouteLatencyBoundary, SessionRouteLatencyObservations, SessionRouteLatencyUnit,
    SessionRouteMetrics, SessionRouteObservationInterval, SessionSidecarMetrics,
    SessionSourceMetrics, SessionStartCancellation, SessionStopOutcome, SessionTerminalState,
    SessionTrace, SessionTraceRecorder, SessionTraceRecorderFinishError,
    SessionTraceRecorderOutcome, SessionTraceRecorderStartError, SessionTraceValidation,
    SessionTraceValidationError,
};
pub use crate::session::{
    session_recording_outcome_error_code, SessionRecordingErrorCode, SessionRecordingObservations,
    SessionRecordingOutcome, SessionRecordingState, SessionRecordingStemOutcome,
};

pub use crate::frame::{
    AudioBufferHandle, AudioBufferWriteError, Platform, SharedAudioBufferHandle, SharedAudioFrame,
};
pub use crate::runtime::{
    AsyncOperatorObservations, AsyncOperatorOutputObservations, EdgeObservations,
    PlanSourceInputObservations,
};
pub use crate::runtime::{
    SidecarDeadlines, SidecarHostError, SidecarHostSnapshot, SidecarMessage, SidecarMessageKind,
    SidecarProcessSpec, SidecarProtocolLimits, SidecarState,
};

/// Non-product exports used only by PocketStation's own integration tests and
/// benchmarks. This module is absent from normal builds.
#[cfg(feature = "internal-testing")]
#[doc(hidden)]
pub mod internal {
    pub mod capture {
        pub use crate::capture::*;
        #[cfg(all(target_os = "linux", feature = "native-capture"))]
        pub mod linux {
            pub use crate::capture::platform::linux::*;
        }
        #[cfg(all(target_os = "macos", feature = "native-capture"))]
        pub mod macos {
            pub use crate::capture::platform::macos::{
                discover_input_sources_native, discover_sources_native, tap_available,
                InternalDesktopCaptureBackend as DesktopCaptureBackend,
                InternalDesktopCaptureSource as DesktopCaptureSource,
                InternalSystemLoopbackSource as SystemLoopbackSource, MacosInputSource,
            };
        }
        #[cfg(all(target_os = "windows", feature = "native-capture"))]
        pub mod windows {
            pub use crate::capture::platform::windows::*;
        }
    }
    pub mod frame {
        pub use crate::frame::*;
    }
    pub mod codec {
        pub use crate::codec::*;
    }
    pub mod graph {
        pub use crate::graph::*;
        pub mod compile {
            pub use crate::graph::compile::*;
        }
        pub mod compiler {
            pub use crate::graph::compile::Compiler;
        }
        pub mod dsl {
            pub use crate::graph::dsl::*;
        }
        pub mod ir {
            pub use crate::graph::ir::*;
        }
        pub mod node {
            pub use crate::graph::node::*;
        }
        pub mod plan {
            pub use crate::graph::plan::*;
        }
        pub mod planner {
            pub use crate::graph::compile::RuntimePlanner;
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
    endpoint_extensions: Mutex<Vec<EndpointExtensionRegistration>>,
    operator_registrations: Mutex<Vec<Arc<dyn AsyncOperatorFactory>>>,
    source_registrations: Mutex<Vec<Arc<dyn SourceFactory>>>,
    audio_input_factory: Mutex<Option<Arc<crate::session::extensions::AudioInputFactory>>>,
    sidecar_registrations: Mutex<Vec<SidecarProcessSpec>>,
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

struct EndpointExtensionRegistration {
    operator_id: OperatorId,
    definition: Arc<dyn NodeDefinition>,
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
            endpoint_extensions: Mutex::new(Vec::new()),
            operator_registrations: Mutex::new(Vec::new()),
            source_registrations: Mutex::new(Vec::new()),
            audio_input_factory: Mutex::new(None),
            sidecar_registrations: Mutex::new(Vec::new()),
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
            endpoint_extensions: Mutex::new(Vec::new()),
            operator_registrations: Mutex::new(Vec::new()),
            source_registrations: Mutex::new(Vec::new()),
            audio_input_factory: Mutex::new(None),
            sidecar_registrations: Mutex::new(Vec::new()),
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

    /// Declares the low-level bounded PCM source and returns its explicit
    /// Session handles and producer writer ownership.
    pub fn pcm_source(
        &self,
        configuration: AudioInputConfig,
    ) -> Result<PcmSource, AudioInputError> {
        if configuration.sample_spec() != self.sample_spec {
            return Err(AudioInputError::IncompatibleContract);
        }
        let mut factory_slot = self
            .audio_input_factory
            .lock()
            .map_err(|_| AudioInputError::RegistrationStateUnavailable)?;
        let factory = match factory_slot.as_ref() {
            Some(factory) => Arc::clone(factory),
            None => {
                let factory = crate::session::extensions::AudioInputFactory::new(configuration)?;
                self.source_registrations
                    .lock()
                    .map_err(|_| AudioInputError::RegistrationStateUnavailable)?
                    .push(Arc::clone(&factory) as Arc<dyn SourceFactory>);
                *factory_slot = Some(Arc::clone(&factory));
                factory
            }
        };
        let reservation = factory.reserve(configuration)?;
        let source = self.declaration.source(
            SourceTypeId::new(PCM_SOURCE_TYPE_ID)?,
            SourceConfiguration::default(),
        )?;
        let writer = factory.bind(source.source_id(), reservation);
        let output = match source.output("audio") {
            Ok(output) => output,
            Err(error) => {
                factory.cancel(source.source_id());
                return Err(error.into());
            }
        };
        Ok(PcmSource::new(source, output, writer))
    }

    /// Opens a bounded input for audio already owned by the embedding
    /// application.
    ///
    /// Route [`AudioInput::output`] like any other source, then write complete
    /// frames directly through [`AudioInput::try_write`] or its preallocated
    /// buffer API. Session owns source, stream, stem, sequence, timing, and
    /// discontinuity identity. No OS loopback capture or second runtime is
    /// involved.
    pub fn audio_input(
        &self,
        configuration: AudioInputConfig,
    ) -> Result<AudioInput, AudioInputError> {
        self.pcm_source(configuration).map(AudioInput::new)
    }

    /// Declares exactly one operator instance. Connect streams to named inputs
    /// and select named outputs through the returned Session-scoped handle.
    pub fn operator(&self, operator: Operator) -> Result<OperatorInstanceHandle, SessionError> {
        self.declaration.operator(operator)
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

    /// Registers one language-neutral sidecar under this Session's bounded
    /// process lifecycle. The child is spawned only during transactional start.
    pub fn register_sidecar(&self, spec: SidecarProcessSpec) -> Result<(), SessionSidecarError> {
        self.sidecar_registrations
            .lock()
            .map_err(|_| SessionSidecarError::RegistrationStateUnavailable)?
            .push(spec);
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
    #[deprecated(
        since = "1.0.0",
        note = "use pocketstation::connector::Connector and Session::register_connector"
    )]
    pub fn connector(
        &self,
        operator_id: OperatorId,
        configuration: EndpointConfiguration,
    ) -> Result<EndpointHandle, SessionError> {
        self.declaration.connector(operator_id, configuration)
    }

    /// Registers the externally owned implementation for a declared connector.
    #[deprecated(
        since = "1.0.0",
        note = "use pocketstation::connector::Connector and Session::register_connector"
    )]
    pub fn register_connector_driver(
        &self,
        operator_id: OperatorId,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<(), SessionEndpointError> {
        self.register_audio_endpoint_driver(
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
        self.register_audio_endpoint_driver(
            OperatorId::new(BROWSER_OPERATOR_ID),
            crate::session::NodeTypeId::from(BROWSER_NODE_TYPE_ID),
            factory,
        )
    }

    fn register_audio_endpoint_driver(
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

    /// Registers one externally owned endpoint as a single compiler/runtime
    /// extension. The endpoint definition and driver cannot be installed
    /// independently through this authority.
    pub fn register_endpoint(
        &self,
        operator_id: OperatorId,
        definition: Arc<dyn NodeDefinition>,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<(), SessionEndpointError> {
        let node_type_id = definition.descriptor().type_id().clone();
        let mut extensions = self
            .endpoint_extensions
            .lock()
            .map_err(|_| SessionEndpointError::RegistrationStateUnavailable)?;
        let registrations = self
            .endpoint_registrations
            .lock()
            .map_err(|_| SessionEndpointError::RegistrationStateUnavailable)?;
        if extensions
            .iter()
            .any(|entry| entry.operator_id == operator_id)
            || registrations
                .iter()
                .any(|entry| entry.operator_id == operator_id)
        {
            return Err(SessionEndpointError::DuplicateOperatorId {
                operator_id: operator_id.as_str().to_owned(),
            });
        }
        if extensions
            .iter()
            .any(|entry| entry.definition.descriptor().type_id() == &node_type_id)
            || registrations
                .iter()
                .any(|entry| entry.node_type_id == node_type_id)
        {
            return Err(SessionEndpointError::DuplicateNodeTypeId {
                node_type_id: node_type_id.as_str().to_owned(),
            });
        }
        drop(registrations);
        extensions.push(EndpointExtensionRegistration {
            operator_id,
            definition,
            factory,
        });
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
            endpoint_extensions,
            operator_registrations,
            source_registrations,
            audio_input_factory: _audio_input_factory,
            sidecar_registrations,
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
                SessionStartError::from(crate::session::SessionEngineStartError::Freeze(error))
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
        let endpoint_extensions = endpoint_extensions
            .into_inner()
            .map_err(|_| SessionStartError::invariant("endpoint registration state unavailable"))?;
        for registration in endpoint_extensions {
            let _ = host_builder.register_endpoint(
                registration.operator_id,
                registration.definition,
                registration.factory,
            )?;
        }
        let endpoint_registrations = endpoint_registrations
            .into_inner()
            .map_err(|_| SessionStartError::invariant("endpoint registration state unavailable"))?;
        for registration in endpoint_registrations {
            let _ = host_builder.register_audio_endpoint_driver(
                registration.operator_id,
                registration.node_type_id,
                registration.factory,
            )?;
        }
        let operator_registrations = operator_registrations
            .into_inner()
            .map_err(|_| SessionStartError::invariant("operator registration state unavailable"))?;
        for factory in operator_registrations {
            let _ = host_builder.register_async_operator(factory)?;
        }
        let source_registrations = source_registrations
            .into_inner()
            .map_err(|_| SessionStartError::invariant("source registration state unavailable"))?;
        for factory in source_registrations {
            let _ = host_builder
                .engine_builder()
                .register_source_factory(factory)?;
        }
        let sidecar_registrations = sidecar_registrations
            .into_inner()
            .map_err(|_| SessionStartError::invariant("sidecar registration state unavailable"))?;
        for spec in sidecar_registrations {
            let _ = host_builder.register_sidecar_process(spec)?;
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
            return Err(SessionStartError::new(
                SessionStartErrorCode::MissingRecordingConfiguration,
                "recording routes require an explicit Session recording root",
            ));
        }
        let compiled = host.compile(declaration)?;
        let receipt = host.polled_audio_receipt(0).ok_or_else(|| {
            SessionStartError::new(
                SessionStartErrorCode::MissingAudioReceipt,
                "native Session host did not retain its bounded audio receipt",
            )
        })?;
        let mut running = host.start_compiled_cancellable(compiled, cancellation)?;
        let Some(events) = running.take_event_receiver() else {
            let _ = running.stop();
            return Err(SessionStartError::new(
                SessionStartErrorCode::MissingEventReceiver,
                "canonical running Session did not retain its event receiver",
            ));
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
            endpoint_extensions: Mutex::new(Vec::new()),
            operator_registrations: Mutex::new(Vec::new()),
            source_registrations: Mutex::new(Vec::new()),
            audio_input_factory: Mutex::new(None),
            sidecar_registrations: Mutex::new(Vec::new()),
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

    /// Returns one finalizable observation handle per Session-owned operator
    /// instance, including exact per-input-port edge counters.
    pub fn operator_metrics(&self) -> Box<[SessionOperatorMetrics]> {
        self.running.operator_metrics()
    }

    /// Returns one observation handle per Session-owned external source.
    pub fn external_source_metrics(&self) -> Box<[SessionExternalSourceMetrics]> {
        self.running.external_source_metrics()
    }

    pub fn sidecar_metrics(&self) -> Box<[SessionSidecarMetrics]> {
        self.running.sidecar_metrics()
    }

    pub fn try_send_sidecar_signal(
        &self,
        sidecar_id: u64,
        message: SidecarMessage,
    ) -> Result<(), SidecarHostError> {
        self.running.try_send_sidecar_signal(sidecar_id, message)
    }

    pub fn try_receive_sidecar_signal(
        &self,
        sidecar_id: u64,
    ) -> Result<Option<SidecarMessage>, SidecarHostError> {
        self.running.try_receive_sidecar_signal(sidecar_id)
    }

    pub fn receive_sidecar_signal(
        &self,
        sidecar_id: u64,
    ) -> Result<SidecarMessage, SidecarHostError> {
        self.running.receive_sidecar_signal(sidecar_id)
    }

    /// Returns one observation handle per derived operator-output route.
    pub fn derived_route_metrics(&self) -> Box<[SessionDerivedRouteMetrics]> {
        self.running.derived_route_metrics()
    }

    /// Returns exact queue, pool, loss, and lifecycle accounting for every
    /// Session-owned typed-PCM reentry into the specialized audio lane.
    pub fn audio_reentry_metrics(&self) -> Box<[SessionAudioReentryMetrics]> {
        self.running.audio_reentry_metrics()
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

/// Stable façade error for Session startup.
///
/// Internal compiler, runtime, capture-owner, and host error types never cross
/// this boundary. Language bindings and external Rust callers consume the
/// stable code and diagnostic message instead of depending on engine owners.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct SessionStartError {
    code: SessionStartErrorCode,
    message: String,
}

impl SessionStartError {
    fn new(code: SessionStartErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    fn invariant(message: impl Into<String>) -> Self {
        Self::new(SessionStartErrorCode::HostSetupFailed, message)
    }

    pub const fn code(&self) -> SessionStartErrorCode {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> SessionStartErrorKind {
        match self.code {
            SessionStartErrorCode::StartCancelled => SessionStartErrorKind::Cancelled,
            SessionStartErrorCode::InvalidSelector => SessionStartErrorKind::InvalidSelector,
            SessionStartErrorCode::MissingRecordingConfiguration => {
                SessionStartErrorKind::MissingRecordingConfiguration
            }
            SessionStartErrorCode::MissingAudioReceipt
            | SessionStartErrorCode::MissingEventReceiver => SessionStartErrorKind::Invariant,
            SessionStartErrorCode::HostSetupFailed
            | SessionStartErrorCode::UnsupportedPlatform
            | SessionStartErrorCode::TraceRecorderSetupFailed => SessionStartErrorKind::Host,
            _ => SessionStartErrorKind::Engine,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.kind() == SessionStartErrorKind::Cancelled
    }
}

impl From<SessionTraceRecorderStartError> for SessionStartError {
    fn from(error: SessionTraceRecorderStartError) -> Self {
        Self::new(
            SessionStartErrorCode::TraceRecorderSetupFailed,
            format!("Session trace setup failed: {error}"),
        )
    }
}

impl From<SessionEngineHostBuildError> for SessionStartError {
    fn from(error: SessionEngineHostBuildError) -> Self {
        let code = if matches!(
            error,
            crate::session::SessionEngineHostBuildError::UnsupportedPlatform
        ) {
            SessionStartErrorCode::UnsupportedPlatform
        } else {
            SessionStartErrorCode::HostSetupFailed
        };
        Self::new(code, format!("native Session host setup failed: {error}"))
    }
}

impl From<SessionEngineStartError> for SessionStartError {
    fn from(error: SessionEngineStartError) -> Self {
        let code = match &error {
            SessionEngineStartError::Freeze(SessionError::InvalidSelector { .. }) => {
                SessionStartErrorCode::InvalidSelector
            }
            SessionEngineStartError::Freeze(_) => SessionStartErrorCode::DeclarationInvalid,
            SessionEngineStartError::Compile(_) => SessionStartErrorCode::CompileFailed,
            SessionEngineStartError::Prepare(_) => SessionStartErrorCode::RuntimePrepareFailed,
            SessionEngineStartError::Start(failure) => session_start_failure_code(failure.error()),
            SessionEngineStartError::Sidecar(_) => SessionStartErrorCode::RuntimeStartFailed,
        };
        Self::new(code, format!("canonical Session start failed: {error}"))
    }
}

impl From<SourceRegistrationError> for SessionStartError {
    fn from(error: SourceRegistrationError) -> Self {
        Self::new(
            SessionStartErrorCode::CompileFailed,
            format!("Session source registration failed: {error}"),
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionEndpointError {
    #[error("Session endpoint-registration state is unavailable")]
    RegistrationStateUnavailable,
    #[error("Session endpoint operator id '{operator_id}' is already registered")]
    DuplicateOperatorId { operator_id: String },
    #[error("Session endpoint node type id '{node_type_id}' is already registered")]
    DuplicateNodeTypeId { node_type_id: String },
}

#[derive(Debug, thiserror::Error)]
pub enum SessionSidecarError {
    #[error("Session sidecar-registration state is unavailable")]
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

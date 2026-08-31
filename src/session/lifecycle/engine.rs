use std::sync::Arc;

use crate::endpoint::{EndpointDriverFactory, EndpointDriverRegistry, EndpointDriverRegistryError};
use crate::frame::AudioFrameDuration;
use crate::graph::{
    AsyncOperatorFactory, NodeDefinition, NodeRegistrationError, NodeRegistry, NodeTypeId,
    PrepareContext,
};
use crate::runtime::{SidecarHost, SidecarHostError, SidecarProcessSpec};

use crate::session::compile::SessionGraphLowerer;
use crate::session::extensions::builtins::{
    audio_endpoint_boundary_definition_with_frame_samples,
    register_session_graph_nodes_with_sample_spec,
};
use crate::session::extensions::source::{
    source_node_definition, SourceFactory, SourceRegistrationError, SourceRegistry,
};
#[cfg(any(test, feature = "internal-testing"))]
use crate::session::extensions::source::{SourceManifest, SourceTypeId};
use crate::session::{
    prepare_session_runtime, CaptureBackendSet, CompiledSession, OperatorId, RunningSession,
    Session, SessionCompileError, SessionCompiler, SessionError, SessionGraphRegistrationError,
    SessionPrepareError, SessionStartFailure, SessionStartOptions, SessionTraceRecorderHandle,
};

/// Registers the components and runtime configuration for one Session.
///
/// The builder owns only registration and runtime configuration. Concrete
/// capture and endpoint implementations enter through their existing contracts
/// and remain owned by their respective packages.
pub struct SessionEngineBuilder {
    node_registry: NodeRegistry,
    prepare_context: PrepareContext,
    source_queue_capacity_frames: usize,
    start_options: SessionStartOptions,
    endpoint_registry: EndpointDriverRegistry,
    source_registry: SourceRegistry,
    graph_lowerers: Vec<Arc<dyn SessionGraphLowerer>>,
    sidecar_processes: Vec<SidecarProcessSpec>,
    session_trace_recorder: Option<SessionTraceRecorderHandle>,
}

impl SessionEngineBuilder {
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn new(
        prepare_context: PrepareContext,
        source_queue_capacity_frames: usize,
        start_options: SessionStartOptions,
    ) -> Result<Self, SessionEngineBuildError> {
        Self::new_with_audio_frame_duration(
            prepare_context,
            AudioFrameDuration::default(),
            source_queue_capacity_frames,
            start_options,
        )
    }

    pub fn new_with_audio_frame_duration(
        prepare_context: PrepareContext,
        audio_frame_duration: AudioFrameDuration,
        source_queue_capacity_frames: usize,
        start_options: SessionStartOptions,
    ) -> Result<Self, SessionEngineBuildError> {
        let mut node_registry = NodeRegistry::new();
        let graph_lowerers = register_session_graph_nodes_with_sample_spec(
            &mut node_registry,
            prepare_context.sample_spec,
            audio_frame_duration,
        )?;
        Ok(Self {
            node_registry,
            prepare_context,
            source_queue_capacity_frames,
            start_options,
            endpoint_registry: EndpointDriverRegistry::new(),
            source_registry: SourceRegistry::default(),
            graph_lowerers,
            sidecar_processes: Vec::new(),
            session_trace_recorder: None,
        })
    }

    pub fn set_session_trace(
        &mut self,
        session_trace_recorder: SessionTraceRecorderHandle,
    ) -> &mut Self {
        self.session_trace_recorder = Some(session_trace_recorder);
        self
    }

    /// Registers an audio Endpoint that accepts the finite frame size declared
    /// by each connected AudioBus.
    ///
    /// Sources keep authority over their frame cadence. The Endpoint boundary
    /// validates the shared sample format and sample rate without rewriting an
    /// application-owned stream to the Session's capture cadence.
    pub fn register_audio_endpoint_driver(
        &mut self,
        operator_id: OperatorId,
        node_type_id: NodeTypeId,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<&mut Self, EndpointExtensionRegistrationError> {
        self.register_endpoint(
            operator_id,
            audio_endpoint_boundary_definition_with_frame_samples(
                node_type_id,
                self.prepare_context.sample_spec,
                None,
            ),
            factory,
        )
    }

    /// Atomically registers an endpoint's compiler contract and runtime driver.
    ///
    /// Preflight checks both registries before either is mutated, preventing a
    /// half-registered extension from escaping setup.
    pub fn register_endpoint(
        &mut self,
        operator_id: OperatorId,
        definition: Arc<dyn NodeDefinition>,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<&mut Self, EndpointExtensionRegistrationError> {
        let node_type_id = definition.descriptor().type_id;
        self.endpoint_registry
            .validate_registration(&operator_id, &node_type_id)?;
        match self.node_registry.definition(&node_type_id) {
            Some(existing) if existing.descriptor() != definition.descriptor() => {
                return Err(EndpointExtensionRegistrationError::ConflictingDefinition {
                    node_type_id: node_type_id.as_str().to_owned(),
                });
            }
            Some(_) => {}
            None => self.node_registry.register_definition(definition)?,
        }
        self.endpoint_registry
            .register(operator_id, node_type_id, factory)?;
        Ok(self)
    }

    pub fn register_async_operator(
        &mut self,
        factory: Arc<dyn AsyncOperatorFactory>,
    ) -> Result<&mut Self, NodeRegistrationError> {
        self.node_registry.register_async(factory)?;
        Ok(self)
    }

    /// Retains one externally implemented sidecar under the Session
    /// lifecycle. IDs are unique within the engine so observations and
    /// shutdown failures remain attributable without process-global state.
    pub fn register_sidecar_process(
        &mut self,
        spec: SidecarProcessSpec,
    ) -> Result<&mut Self, SessionEngineBuildError> {
        if self
            .sidecar_processes
            .iter()
            .any(|registered| registered.id == spec.id)
        {
            return Err(SessionEngineBuildError::DuplicateSidecarId {
                sidecar_id: spec.id,
            });
        }
        self.sidecar_processes.push(spec);
        Ok(self)
    }

    /// Registers one externally implemented source contract by stable type ID.
    ///
    /// Registration validates the complete manifest and rejects duplicate IDs;
    /// a later registration can never silently replace the first factory.
    pub fn register_source_factory(
        &mut self,
        factory: Arc<dyn SourceFactory>,
    ) -> Result<&mut Self, SourceRegistrationError> {
        factory
            .manifest()
            .validate()
            .map_err(SourceRegistrationError::InvalidManifest)?;
        let source_type_id = factory.manifest().source_type_id.clone();
        if self.source_registry.manifest(&source_type_id).is_some() {
            return Err(SourceRegistrationError::DuplicateSourceType(source_type_id));
        }
        let node_type_id = NodeTypeId::from(source_type_id.as_str());
        if self.node_registry.contains(&node_type_id) {
            return Err(SourceRegistrationError::NodeTypeConflict(source_type_id));
        }
        self.node_registry
            .register_definition(source_node_definition(Arc::clone(&factory)))
            .map_err(|_| SourceRegistrationError::NodeTypeConflict(source_type_id.clone()))?;
        self.source_registry.register(factory)?;
        Ok(self)
    }

    /// Returns the validated manifest currently registered for `source_type_id`.
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn source_manifest(&self, source_type_id: &SourceTypeId) -> Option<&SourceManifest> {
        self.source_registry.manifest(source_type_id)
    }

    /// Consumes all setup state so no partially populated registry can escape.
    pub fn build(self) -> Result<SessionEngine, SessionEngineBuildError> {
        validate_engine_configuration(
            &self.prepare_context,
            self.source_queue_capacity_frames,
            self.start_options,
        )?;

        Ok(SessionEngine {
            node_registry: self.node_registry,
            endpoint_registry: self.endpoint_registry,
            source_registry: self.source_registry,
            graph_lowerers: self.graph_lowerers,
            prepare_context: self.prepare_context,
            source_queue_capacity_frames: self.source_queue_capacity_frames,
            start_options: self.start_options,
            sidecar_processes: self.sidecar_processes,
            session_trace_recorder: self.session_trace_recorder,
        })
    }
}

/// Composes one Rust Session engine from its owned runtime components.
///
/// Compilation, runtime preparation, capture ownership, endpoint lifecycle,
/// scheduling, rollback, and finalization remain implemented by their existing
/// owners. This type only assembles those contracts in their required order.
pub struct SessionEngine {
    node_registry: NodeRegistry,
    endpoint_registry: EndpointDriverRegistry,
    source_registry: SourceRegistry,
    graph_lowerers: Vec<Arc<dyn SessionGraphLowerer>>,
    prepare_context: PrepareContext,
    source_queue_capacity_frames: usize,
    start_options: SessionStartOptions,
    sidecar_processes: Vec<SidecarProcessSpec>,
    session_trace_recorder: Option<SessionTraceRecorderHandle>,
}

impl SessionEngine {
    /// Returns the validated source manifest retained by this engine.
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn source_manifest(&self, source_type_id: &SourceTypeId) -> Option<&SourceManifest> {
        self.source_registry.manifest(source_type_id)
    }

    pub fn compile(&self, session: Session) -> Result<CompiledSession, SessionEngineStartError> {
        let spec = session.freeze()?;
        SessionCompiler::with_sources(
            &self.node_registry,
            &self.endpoint_registry,
            &self.source_registry,
            &self.graph_lowerers,
        )
        .compile(spec)
        .map_err(SessionEngineStartError::Compile)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn start_compiled(
        &self,
        compiled: CompiledSession,
        capture_backends: CaptureBackendSet<'_>,
    ) -> Result<RunningSession, SessionEngineStartError> {
        let prepared = prepare_session_runtime(
            compiled,
            &self.node_registry,
            &self.prepare_context,
            self.source_queue_capacity_frames,
        )?;
        let running = crate::session::lifecycle::start_prepared_session_cancellable_with_trace(
            prepared,
            capture_backends,
            &self.endpoint_registry,
            &self.source_registry,
            self.start_options,
            crate::session::SessionStartCancellation::default(),
            self.session_trace_recorder.clone(),
        )
        .map_err(SessionEngineStartError::Start)?;
        attach_sidecars(running, &self.sidecar_processes)
    }

    pub fn start_compiled_cancellable(
        &self,
        compiled: CompiledSession,
        capture_backends: CaptureBackendSet<'_>,
        start_cancellation: crate::session::SessionStartCancellation,
    ) -> Result<RunningSession, SessionEngineStartError> {
        let prepared = prepare_session_runtime(
            compiled,
            &self.node_registry,
            &self.prepare_context,
            self.source_queue_capacity_frames,
        )?;
        let running = crate::session::lifecycle::start_prepared_session_cancellable_with_trace(
            prepared,
            capture_backends,
            &self.endpoint_registry,
            &self.source_registry,
            self.start_options,
            start_cancellation,
            self.session_trace_recorder.clone(),
        )
        .map_err(SessionEngineStartError::Start)?;
        attach_sidecars(running, &self.sidecar_processes)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn start(
        &self,
        session: Session,
        capture_backends: CaptureBackendSet<'_>,
    ) -> Result<RunningSession, SessionEngineStartError> {
        let compiled = self.compile(session)?;
        self.start_compiled(compiled, capture_backends)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionEngineBuildError {
    #[error(transparent)]
    StructuralNodeRegistration(#[from] SessionGraphRegistrationError),
    #[error("Session engine configuration is invalid: {reason}")]
    InvalidConfiguration { reason: &'static str },
    #[error("sidecar process ID {sidecar_id} is already registered")]
    DuplicateSidecarId { sidecar_id: u64 },
}

#[derive(Debug, thiserror::Error)]
pub enum EndpointExtensionRegistrationError {
    #[error(transparent)]
    Definition(#[from] NodeRegistrationError),
    #[error(transparent)]
    Driver(#[from] EndpointDriverRegistryError),
    #[error("endpoint node type {node_type_id} is already registered with a different contract")]
    ConflictingDefinition { node_type_id: String },
}

#[derive(Debug, thiserror::Error)]
pub enum SessionEngineStartError {
    #[error("Session declaration freeze failed: {0}")]
    Freeze(#[from] SessionError),
    #[error("Session compilation failed: {0}")]
    Compile(#[from] SessionCompileError),
    #[error("Session runtime preparation failed: {0}")]
    Prepare(#[from] SessionPrepareError),
    #[error("Session transactional start failed: {0}")]
    Start(#[source] SessionStartFailure),
    #[error("Session sidecar start failed: {0}")]
    Sidecar(#[from] SidecarHostError),
}

impl SessionEngineStartError {
    pub const fn start_failure(&self) -> Option<&SessionStartFailure> {
        match self {
            Self::Start(failure) => Some(failure),
            Self::Freeze(_) | Self::Compile(_) | Self::Prepare(_) | Self::Sidecar(_) => None,
        }
    }

    pub fn into_start_failure(self) -> Option<SessionStartFailure> {
        match self {
            Self::Start(failure) => Some(failure),
            Self::Freeze(_) | Self::Compile(_) | Self::Prepare(_) | Self::Sidecar(_) => None,
        }
    }
}

fn attach_sidecars(
    mut running: RunningSession,
    specs: &[SidecarProcessSpec],
) -> Result<RunningSession, SessionEngineStartError> {
    let mut sidecars = Vec::with_capacity(specs.len());
    for spec in specs {
        match SidecarHost::spawn(spec.clone()) {
            Ok(sidecar) => sidecars.push(sidecar),
            Err(error) => {
                for sidecar in &mut sidecars {
                    let _ = sidecar.cancel_and_reap();
                }
                let _ = running.cancel();
                return Err(SessionEngineStartError::Sidecar(error));
            }
        }
    }
    running.attach_sidecars(sidecars);
    Ok(running)
}

fn validate_engine_configuration(
    prepare_context: &PrepareContext,
    source_queue_capacity_frames: usize,
    options: SessionStartOptions,
) -> Result<(), SessionEngineBuildError> {
    let reason = if prepare_context.sample_spec.sample_rate_hz == 0 {
        Some("sample rate must be greater than zero hertz")
    } else if !matches!(prepare_context.sample_spec.channels, 1 | 2) {
        Some("channel count must be one or two")
    } else if source_queue_capacity_frames == 0 {
        Some("source queue capacity must be greater than zero frames")
    } else if options.capture_frame_capacity_frames == 0 {
        Some("capture frame capacity must be greater than zero frames")
    } else if options.capture_runtime_event_capacity_events == 0 {
        Some("capture runtime-event capacity must be greater than zero events")
    } else if options.runtime_work_budget_frames == 0 {
        Some("runtime work budget must be greater than zero frames")
    } else if options.runtime_idle_poll_ms == 0 {
        Some("runtime idle poll interval must be greater than zero milliseconds")
    } else if options.runtime_ready_timeout_ms == 0 {
        Some("runtime ready timeout must be greater than zero milliseconds")
    } else if options.session_event_capacity_events == 0 {
        Some("Session event capacity must be greater than zero events")
    } else {
        None
    };
    match reason {
        Some(reason) => Err(SessionEngineBuildError::InvalidConfiguration { reason }),
        None => Ok(()),
    }
}

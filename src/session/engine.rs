use std::sync::Arc;

use crate::endpoint::{EndpointDriverFactory, EndpointDriverRegistry, EndpointDriverRegistryError};
use crate::graph::{
    AsyncOperatorFactory, NodeDefinition, NodeRegistrationError, NodeRegistry, NodeTypeId,
    PrepareContext,
};

use crate::session::source_extension::{
    source_node_definition, SourceFactory, SourceManifest, SourceRegistrationError, SourceRegistry,
    SourceTypeId,
};
use crate::session::structural_nodes::register_session_structural_nodes_with_sample_spec;
use crate::session::{
    prepare_session_runtime, CaptureBackendSet, CompiledSession, OperatorId, RunningSession,
    Session, SessionCompileError, SessionCompiler, SessionError, SessionPrepareError,
    SessionStartFailure, SessionStartOptions, SessionStructuralNodeRegistrationError,
    SessionTraceRecorderHandle,
};

/// Setup-time builder for one canonical Session composition environment.
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
    session_trace_recorder: Option<SessionTraceRecorderHandle>,
}

impl SessionEngineBuilder {
    pub fn new(
        prepare_context: PrepareContext,
        source_queue_capacity_frames: usize,
        start_options: SessionStartOptions,
    ) -> Result<Self, SessionEngineBuildError> {
        let mut node_registry = NodeRegistry::new();
        register_session_structural_nodes_with_sample_spec(
            &mut node_registry,
            prepare_context.sample_spec,
        )?;
        Ok(Self {
            node_registry,
            prepare_context,
            source_queue_capacity_frames,
            start_options,
            endpoint_registry: EndpointDriverRegistry::new(),
            source_registry: SourceRegistry::default(),
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

    pub fn register_endpoint_driver(
        &mut self,
        operator_id: OperatorId,
        node_type_id: NodeTypeId,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<&mut Self, EndpointDriverRegistryError> {
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
    pub fn source_manifest(&self, source_type_id: &SourceTypeId) -> Option<&SourceManifest> {
        self.source_registry.manifest(source_type_id)
    }

    pub fn register_endpoint_definition(
        &mut self,
        definition: Arc<dyn NodeDefinition>,
    ) -> Result<&mut Self, NodeRegistrationError> {
        self.node_registry.register_definition(definition)?;
        Ok(self)
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
            prepare_context: self.prepare_context,
            source_queue_capacity_frames: self.source_queue_capacity_frames,
            start_options: self.start_options,
            session_trace_recorder: self.session_trace_recorder,
        })
    }
}

/// Canonical production composition path for one safe Rust Session engine.
///
/// Compilation, runtime preparation, capture ownership, endpoint lifecycle,
/// scheduling, rollback, and finalization remain implemented by their existing
/// owners. This type only assembles those contracts in their required order.
pub struct SessionEngine {
    node_registry: NodeRegistry,
    endpoint_registry: EndpointDriverRegistry,
    source_registry: SourceRegistry,
    prepare_context: PrepareContext,
    source_queue_capacity_frames: usize,
    start_options: SessionStartOptions,
    session_trace_recorder: Option<SessionTraceRecorderHandle>,
}

impl SessionEngine {
    /// Returns the validated source manifest retained by this engine.
    pub fn source_manifest(&self, source_type_id: &SourceTypeId) -> Option<&SourceManifest> {
        self.source_registry.manifest(source_type_id)
    }

    pub fn compile(&self, session: Session) -> Result<CompiledSession, SessionEngineStartError> {
        let spec = session.freeze()?;
        SessionCompiler::with_sources(
            &self.node_registry,
            &self.endpoint_registry,
            &self.source_registry,
        )
        .compile(spec)
        .map_err(SessionEngineStartError::Compile)
    }

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
        crate::session::running::start_prepared_session_cancellable_with_trace(
            prepared,
            capture_backends,
            &self.endpoint_registry,
            self.start_options,
            crate::session::SessionStartCancellation::default(),
            self.session_trace_recorder.clone(),
        )
        .map_err(SessionEngineStartError::Start)
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
        crate::session::running::start_prepared_session_cancellable_with_trace(
            prepared,
            capture_backends,
            &self.endpoint_registry,
            self.start_options,
            start_cancellation,
            self.session_trace_recorder.clone(),
        )
        .map_err(SessionEngineStartError::Start)
    }

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
    StructuralNodeRegistration(#[from] SessionStructuralNodeRegistrationError),
    #[error("Session engine configuration is invalid: {reason}")]
    InvalidConfiguration { reason: &'static str },
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
}

impl SessionEngineStartError {
    pub const fn start_failure(&self) -> Option<&SessionStartFailure> {
        match self {
            Self::Start(failure) => Some(failure),
            Self::Freeze(_) | Self::Compile(_) | Self::Prepare(_) => None,
        }
    }

    pub fn into_start_failure(self) -> Option<SessionStartFailure> {
        match self {
            Self::Start(failure) => Some(failure),
            Self::Freeze(_) | Self::Compile(_) | Self::Prepare(_) => None,
        }
    }
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

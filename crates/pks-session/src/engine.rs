use std::sync::Arc;

use pks_endpoint::{EndpointDriverFactory, EndpointDriverRegistry, EndpointDriverRegistryError};
use pks_graph::{NodeRegistry, NodeTypeId, PrepareContext};

use crate::{
    prepare_session_runtime, register_session_structural_nodes, start_prepared_session,
    CaptureBackendSet, OperatorId, OperatorRegistry, OperatorRegistryError, RunningSession,
    Session, SessionCompileError, SessionCompiler, SessionError, SessionPrepareError,
    SessionStartFailure, SessionStartOptions, SessionStructuralNodeRegistrationError,
};

struct EndpointDriverRegistration {
    operator_id: OperatorId,
    node_type_id: NodeTypeId,
    factory: Arc<dyn EndpointDriverFactory>,
}

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
    endpoint_registrations: Vec<EndpointDriverRegistration>,
}

impl SessionEngineBuilder {
    pub fn new(
        prepare_context: PrepareContext,
        source_queue_capacity_frames: usize,
        start_options: SessionStartOptions,
    ) -> Result<Self, SessionEngineBuildError> {
        let mut node_registry = NodeRegistry::new();
        register_session_structural_nodes(&mut node_registry)?;
        Ok(Self {
            node_registry,
            prepare_context,
            source_queue_capacity_frames,
            start_options,
            endpoint_registrations: Vec::new(),
        })
    }

    pub fn register_endpoint_driver(
        &mut self,
        operator_id: OperatorId,
        node_type_id: NodeTypeId,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<&mut Self, SessionEngineRegistrationError> {
        if operator_id.as_str().trim().is_empty() {
            return Err(SessionEngineRegistrationError::EmptyOperatorId);
        }
        if node_type_id.as_str().trim().is_empty() {
            return Err(SessionEngineRegistrationError::EmptyNodeTypeId);
        }
        if let Some(existing) = self
            .endpoint_registrations
            .iter()
            .find(|registration| registration.operator_id == operator_id)
        {
            if existing.node_type_id == node_type_id {
                return Err(SessionEngineRegistrationError::DuplicateEndpointDriver {
                    operator_id: operator_id.as_str().to_owned(),
                    node_type_id: node_type_id.as_str().to_owned(),
                });
            }
            return Err(SessionEngineRegistrationError::OperatorNodeTypeConflict {
                operator_id: operator_id.as_str().to_owned(),
                registered_node_type_id: existing.node_type_id.as_str().to_owned(),
                requested_node_type_id: node_type_id.as_str().to_owned(),
            });
        }
        self.endpoint_registrations
            .push(EndpointDriverRegistration {
                operator_id,
                node_type_id,
                factory,
            });
        Ok(self)
    }

    /// Consumes all setup state so no partially populated registry can escape.
    pub fn build(self) -> Result<SessionEngine, SessionEngineBuildError> {
        validate_engine_configuration(self.source_queue_capacity_frames, self.start_options)?;

        let mut operator_registry = OperatorRegistry::new();
        let mut endpoint_registry = EndpointDriverRegistry::new();
        for registration in self.endpoint_registrations {
            operator_registry
                .register(
                    registration.operator_id.clone(),
                    registration.node_type_id.clone(),
                )
                .map_err(SessionEngineBuildError::OperatorRegistration)?;
            endpoint_registry
                .register(
                    registration.operator_id,
                    registration.node_type_id,
                    registration.factory,
                )
                .map_err(SessionEngineBuildError::EndpointRegistration)?;
        }

        Ok(SessionEngine {
            node_registry: self.node_registry,
            operator_registry,
            endpoint_registry,
            prepare_context: self.prepare_context,
            source_queue_capacity_frames: self.source_queue_capacity_frames,
            start_options: self.start_options,
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
    operator_registry: OperatorRegistry,
    endpoint_registry: EndpointDriverRegistry,
    prepare_context: PrepareContext,
    source_queue_capacity_frames: usize,
    start_options: SessionStartOptions,
}

impl SessionEngine {
    pub fn start(
        &self,
        session: Session,
        capture_backends: CaptureBackendSet<'_>,
    ) -> Result<RunningSession, SessionEngineStartError> {
        let spec = session.freeze()?;
        let compiled =
            SessionCompiler::new(&self.node_registry, &self.operator_registry).compile(spec)?;
        let prepared = prepare_session_runtime(
            compiled,
            &self.node_registry,
            &self.prepare_context,
            self.source_queue_capacity_frames,
        )?;
        start_prepared_session(
            prepared,
            capture_backends,
            &self.endpoint_registry,
            &self.prepare_context,
            self.start_options,
        )
        .map_err(SessionEngineStartError::Start)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SessionEngineRegistrationError {
    #[error("endpoint operator id cannot be empty")]
    EmptyOperatorId,
    #[error("endpoint node type id cannot be empty")]
    EmptyNodeTypeId,
    #[error(
        "endpoint driver is already registered for operator '{operator_id}' and node type '{node_type_id}'"
    )]
    DuplicateEndpointDriver {
        operator_id: String,
        node_type_id: String,
    },
    #[error(
        "operator '{operator_id}' is already mapped to node type '{registered_node_type_id}', not '{requested_node_type_id}'"
    )]
    OperatorNodeTypeConflict {
        operator_id: String,
        registered_node_type_id: String,
        requested_node_type_id: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum SessionEngineBuildError {
    #[error(transparent)]
    StructuralNodeRegistration(#[from] SessionStructuralNodeRegistrationError),
    #[error("Session engine configuration is invalid: {reason}")]
    InvalidConfiguration { reason: &'static str },
    #[error("Session operator registration failed: {0}")]
    OperatorRegistration(#[source] OperatorRegistryError),
    #[error("Session endpoint registration failed: {0}")]
    EndpointRegistration(#[source] EndpointDriverRegistryError),
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
    source_queue_capacity_frames: usize,
    options: SessionStartOptions,
) -> Result<(), SessionEngineBuildError> {
    let reason = if source_queue_capacity_frames == 0 {
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

use crate::graph::signal::preparation::{AsyncNodeFuture, AsyncOperatorPrepareContext};
use crate::graph::{
    BackpressurePolicy, CopyPolicy, EdgeContract, ExecutionPartition, NodeConfig, NodeDescriptor,
    NodeError, OperatorId, PortDirection, PortSpec, SafetyContract, SignalSpec,
};
use crate::graph::{SemanticRole, SignalEnvelope};

/// Async operator contract for model, connector, transport, and control-plane work.
///
/// `AsyncNode` is intentionally separate from `RuntimeNode`: realtime nodes process
/// `AudioFrame` synchronously on allocation-free executors, while async nodes may
/// await, allocate, and perform I/O only after a Bridge leaves the hot path.
pub trait AsyncNode: Send {
    fn prepare<'a>(
        &'a mut self,
        cx: &'a AsyncOperatorPrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>>;

    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>>;

    fn process_port<'a>(
        &'a mut self,
        _input_port: &'a str,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        self.process(input)
    }

    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async { Ok(Vec::new()) })
    }

    fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }

    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorPermissionPolicy {
    pub network_allowed: bool,
    pub filesystem_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorDeadlinePolicy {
    pub process_timeout_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorCancellationPolicy {
    DiscardQueued,
    DrainQueued,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorFailurePolicy {
    Continue,
    StopWorker,
}

#[derive(Debug, Clone, Default)]
pub struct OperatorOutputRolePolicy {
    pub allowed: Vec<SemanticRole>,
    pub terminal: Vec<SemanticRole>,
}

impl OperatorOutputRolePolicy {
    pub fn accepts(&self, signal: &SignalSpec, declared_port: &SignalSpec) -> bool {
        if self.allowed.is_empty() {
            return signal.role == declared_port.role;
        }
        signal.role.as_ref().is_some_and(|role| {
            self.allowed
                .iter()
                .any(|declared| declared.as_str() == role.as_str())
        })
    }

    pub fn is_terminal(&self, signal: &SignalSpec) -> bool {
        signal.role.as_ref().is_some_and(|role| {
            self.terminal
                .iter()
                .any(|declared| declared.as_str() == role.as_str())
        })
    }

    fn validate(&self) -> Result<(), AsyncOperatorManifestError> {
        if self
            .allowed
            .iter()
            .chain(self.terminal.iter())
            .any(|role| role.as_str().trim().is_empty())
        {
            return Err(AsyncOperatorManifestError::EmptyOutputRole);
        }
        if has_duplicate_roles(&self.allowed) || has_duplicate_roles(&self.terminal) {
            return Err(AsyncOperatorManifestError::DuplicateOutputRole);
        }
        if self.terminal.iter().any(|terminal| {
            !self
                .allowed
                .iter()
                .any(|allowed| allowed.as_str() == terminal.as_str())
        }) {
            return Err(AsyncOperatorManifestError::TerminalOutputRoleNotAllowed);
        }
        Ok(())
    }
}

fn has_duplicate_roles(roles: &[SemanticRole]) -> bool {
    roles.iter().enumerate().any(|(index, role)| {
        roles[index + 1..]
            .iter()
            .any(|other| other.as_str() == role.as_str())
    })
}

#[derive(Debug, Clone)]
pub struct AsyncOperatorManifest {
    pub(crate) operator_id: OperatorId,
    pub(crate) revision: u32,
    pub(crate) generation: u32,
    pub(crate) node: NodeDescriptor,
    pub(crate) input_edge: EdgeContract,
    pub(crate) output_edge: EdgeContract,
    pub(crate) queue_capacity_frames: usize,
    pub(crate) permission: OperatorPermissionPolicy,
    pub(crate) deadline: OperatorDeadlinePolicy,
    pub(crate) cancellation: OperatorCancellationPolicy,
    pub(crate) failure: OperatorFailurePolicy,
    pub(crate) output_roles: OperatorOutputRolePolicy,
}

impl AsyncOperatorManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operator_id: OperatorId,
        revision: u32,
        generation: u32,
        node: NodeDescriptor,
        input_edge: EdgeContract,
        output_edge: EdgeContract,
        queue_capacity_frames: usize,
        permission: OperatorPermissionPolicy,
        deadline: OperatorDeadlinePolicy,
        cancellation: OperatorCancellationPolicy,
        failure: OperatorFailurePolicy,
        output_roles: OperatorOutputRolePolicy,
    ) -> Result<Self, AsyncOperatorManifestError> {
        let manifest = Self {
            operator_id,
            revision,
            generation,
            node,
            input_edge,
            output_edge,
            queue_capacity_frames,
            permission,
            deadline,
            cancellation,
            failure,
            output_roles,
        };
        manifest.validate()?;
        Ok(manifest)
    }

    pub const fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

    pub const fn revision(&self) -> u32 {
        self.revision
    }

    pub const fn generation(&self) -> u32 {
        self.generation
    }

    pub const fn node(&self) -> &NodeDescriptor {
        &self.node
    }

    pub const fn input_edge(&self) -> EdgeContract {
        self.input_edge
    }

    pub const fn output_edge(&self) -> EdgeContract {
        self.output_edge
    }

    pub const fn queue_capacity_frames(&self) -> usize {
        self.queue_capacity_frames
    }

    pub const fn permission(&self) -> OperatorPermissionPolicy {
        self.permission
    }

    pub const fn deadline(&self) -> OperatorDeadlinePolicy {
        self.deadline
    }

    pub const fn cancellation(&self) -> OperatorCancellationPolicy {
        self.cancellation
    }

    pub const fn failure(&self) -> OperatorFailurePolicy {
        self.failure
    }

    pub const fn output_roles(&self) -> &OperatorOutputRolePolicy {
        &self.output_roles
    }

    pub fn input_ports(&self) -> impl Iterator<Item = &PortSpec> {
        self.node
            .inputs
            .iter()
            .filter(|port| port.direction == PortDirection::Input)
    }

    pub fn output_ports(&self) -> impl Iterator<Item = &PortSpec> {
        self.node
            .outputs
            .iter()
            .filter(|port| port.direction == PortDirection::Output)
    }

    pub fn validate(&self) -> Result<(), AsyncOperatorManifestError> {
        if self.operator_id.as_str().trim().is_empty() {
            return Err(AsyncOperatorManifestError::EmptyOperatorId);
        }
        if self.revision == 0 {
            return Err(AsyncOperatorManifestError::ZeroRevision);
        }
        if self.generation == 0 {
            return Err(AsyncOperatorManifestError::ZeroGeneration);
        }
        if self.queue_capacity_frames == 0 {
            return Err(AsyncOperatorManifestError::ZeroQueueCapacity);
        }
        if self.deadline.process_timeout_ms == 0 {
            return Err(AsyncOperatorManifestError::ZeroProcessTimeout);
        }
        if !matches!(
            self.node.execution,
            ExecutionPartition::AsyncWorker
                | ExecutionPartition::BlockingWorker
                | ExecutionPartition::External
        ) {
            return Err(AsyncOperatorManifestError::RealtimePartition);
        }
        if !self.node.safety.is_valid_for(self.node.execution) {
            return Err(AsyncOperatorManifestError::InvalidSafetyContract);
        }
        if matches!(
            self.node.safety,
            SafetyContract::NetworkAllowed | SafetyContract::ExternalService
        ) && !self.permission.network_allowed
        {
            return Err(AsyncOperatorManifestError::NetworkPermissionMismatch);
        }
        if self.input_ports().next().is_none() {
            return Err(AsyncOperatorManifestError::MissingInputPort);
        }
        if self.output_ports().next().is_none() {
            return Err(AsyncOperatorManifestError::MissingOutputPort);
        }
        for input_port in self.input_ports() {
            input_port
                .signal
                .validate()
                .map_err(|_| AsyncOperatorManifestError::InvalidInputSignal)?;
            if !input_port.media.supports_signal(&input_port.signal) {
                return Err(AsyncOperatorManifestError::InputSignalMediaMismatch);
            }
            if !self.input_edge.media.is_compatible_with(&input_port.media) {
                return Err(AsyncOperatorManifestError::InputEdgeMediaMismatch);
            }
        }
        for output_port in self.output_ports() {
            output_port
                .signal
                .validate()
                .map_err(|_| AsyncOperatorManifestError::InvalidOutputSignal)?;
            if !output_port.media.supports_signal(&output_port.signal) {
                return Err(AsyncOperatorManifestError::OutputSignalMediaMismatch);
            }
            if !self
                .output_edge
                .media
                .is_compatible_with(&output_port.media)
            {
                return Err(AsyncOperatorManifestError::OutputEdgeMediaMismatch);
            }
        }
        if self.input_edge.backpressure != BackpressurePolicy::DropNewest {
            return Err(AsyncOperatorManifestError::UnsupportedBackpressure);
        }
        if self.input_edge.copy_policy != CopyPolicy::CopyToBranchPool {
            return Err(AsyncOperatorManifestError::UnsupportedInputCopyPolicy);
        }
        if self.output_edge.backpressure != BackpressurePolicy::BoundedQueue {
            return Err(AsyncOperatorManifestError::UnsupportedOutputBackpressure);
        }
        self.output_roles.validate()?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AsyncOperatorManifestError {
    #[error("operator id is empty")]
    EmptyOperatorId,
    #[error("operator revision must be non-zero")]
    ZeroRevision,
    #[error("operator generation must be non-zero")]
    ZeroGeneration,
    #[error("operator queue capacity must be non-zero")]
    ZeroQueueCapacity,
    #[error("operator process timeout must be non-zero")]
    ZeroProcessTimeout,
    #[error("async operator cannot execute in a realtime partition")]
    RealtimePartition,
    #[error("operator safety contract does not match its execution partition")]
    InvalidSafetyContract,
    #[error("operator safety contract requires network permission")]
    NetworkPermissionMismatch,
    #[error("operator manifest has no typed input port")]
    MissingInputPort,
    #[error("operator manifest has no typed output port")]
    MissingOutputPort,
    #[error("async operator bridge currently requires DropNewest backpressure")]
    UnsupportedBackpressure,
    #[error("async operator bridge requires CopyToBranchPool input ownership")]
    UnsupportedInputCopyPolicy,
    #[error("async operator output currently requires BoundedQueue backpressure")]
    UnsupportedOutputBackpressure,
    #[error("operator input edge media does not match its typed input port")]
    InputEdgeMediaMismatch,
    #[error("operator output edge media does not match its typed output port")]
    OutputEdgeMediaMismatch,
    #[error("operator input SignalSpec is invalid")]
    InvalidInputSignal,
    #[error("operator output SignalSpec is invalid")]
    InvalidOutputSignal,
    #[error("operator input SignalSpec does not have a compatible payload representation")]
    InputSignalMediaMismatch,
    #[error("operator output SignalSpec does not have a compatible payload representation")]
    OutputSignalMediaMismatch,
    #[error("operator output role declarations cannot be empty strings")]
    EmptyOutputRole,
    #[error("operator output role declarations cannot contain duplicates")]
    DuplicateOutputRole,
    #[error("every terminal output role must also be an allowed output role")]
    TerminalOutputRoleNotAllowed,
}

pub trait AsyncOperatorFactory: Send + Sync {
    fn manifest(&self) -> &AsyncOperatorManifest;
    fn validate_config(&self, configuration: &NodeConfig) -> Result<(), crate::graph::ConfigError>;
    fn resolve_manifest(
        &self,
        configuration: &NodeConfig,
    ) -> Result<AsyncOperatorManifest, crate::graph::ConfigError> {
        self.validate_config(configuration)?;
        Ok(self.manifest().clone())
    }
    fn create(&self, configuration: &NodeConfig) -> Result<Box<dyn AsyncNode>, NodeError>;
}

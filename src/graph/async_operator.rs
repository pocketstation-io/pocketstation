use crate::graph::{AsyncNode, SemanticRole};
use crate::graph::{
    BackpressurePolicy, CopyPolicy, EdgeContract, ExecutionPartition, NodeConfig, NodeDescriptor,
    NodeError, OperatorId, PortDirection, PortSpec, SafetyContract, SignalSpec,
};

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
    pub operator_id: OperatorId,
    pub revision: u32,
    pub generation: u32,
    pub node: NodeDescriptor,
    pub input_edge: EdgeContract,
    pub output_edge: EdgeContract,
    pub queue_capacity_frames: usize,
    pub permission: OperatorPermissionPolicy,
    pub deadline: OperatorDeadlinePolicy,
    pub cancellation: OperatorCancellationPolicy,
    pub failure: OperatorFailurePolicy,
    pub output_roles: OperatorOutputRolePolicy,
}

impl AsyncOperatorManifest {
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

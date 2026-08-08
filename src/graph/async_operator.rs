use crate::frame::{ConnectorId, FrameLineage};

use crate::graph::{AsyncNode, SemanticRole};
use crate::graph::{
    BackpressurePolicy, CopyPolicy, EdgeContract, ExecutionPartition, NodeConfig, NodeDescriptor,
    NodeError, OperatorId, PortDirection, PortSpec, SafetyContract, SignalSpec, TextFormat,
};

pub const TRANSCRIPT_PARTIAL_ROLE: &str = "transcript.partial";
pub const TRANSCRIPT_FINAL_ROLE: &str = "transcript.final";

pub fn transcript_partial_spec() -> SignalSpec {
    SignalSpec::text(TextFormat::Utf8).with_role(TRANSCRIPT_PARTIAL_ROLE)
}

pub fn transcript_final_spec() -> SignalSpec {
    SignalSpec::text(TextFormat::Utf8).with_role(TRANSCRIPT_FINAL_ROLE)
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
        let input_port_count = self.input_ports().count();
        if input_port_count == 0 {
            return Err(AsyncOperatorManifestError::MissingInputPort);
        }
        if input_port_count > 1 {
            return Err(AsyncOperatorManifestError::AmbiguousInputPort);
        }
        let output_port_count = self.output_ports().count();
        if output_port_count == 0 {
            return Err(AsyncOperatorManifestError::MissingOutputPort);
        }
        if output_port_count > 1 {
            return Err(AsyncOperatorManifestError::AmbiguousOutputPort);
        }
        let input_port = self
            .input_ports()
            .next()
            .ok_or(AsyncOperatorManifestError::MissingInputPort)?;
        let output_port = self
            .output_ports()
            .next()
            .ok_or(AsyncOperatorManifestError::MissingOutputPort)?;
        input_port
            .signal
            .validate()
            .map_err(|_| AsyncOperatorManifestError::InvalidInputSignal)?;
        output_port
            .signal
            .validate()
            .map_err(|_| AsyncOperatorManifestError::InvalidOutputSignal)?;
        if !input_port.media.supports_signal(&input_port.signal) {
            return Err(AsyncOperatorManifestError::InputSignalMediaMismatch);
        }
        if !output_port.media.supports_signal(&output_port.signal) {
            return Err(AsyncOperatorManifestError::OutputSignalMediaMismatch);
        }
        if !self.input_edge.media.is_compatible_with(&input_port.media) {
            return Err(AsyncOperatorManifestError::InputEdgeMediaMismatch);
        }
        if !self
            .output_edge
            .media
            .is_compatible_with(&output_port.media)
        {
            return Err(AsyncOperatorManifestError::OutputEdgeMediaMismatch);
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
    #[error("operator manifest has multiple inputs but through(operator) has no port selection")]
    AmbiguousInputPort,
    #[error("operator manifest has no typed output port")]
    MissingOutputPort,
    #[error("operator manifest has multiple outputs but through(operator) has no port selection")]
    AmbiguousOutputPort,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedSignalLineage {
    pub base: FrameLineage,
    pub timestamp_end_ns: u64,
    pub operator_id: OperatorId,
    pub operator_revision: u32,
    pub operator_generation: u32,
    pub connector_id: Option<ConnectorId>,
}

impl DerivedSignalLineage {
    pub fn new(
        base: FrameLineage,
        timestamp_end_ns: u64,
        operator_id: OperatorId,
        operator_revision: u32,
        operator_generation: u32,
        connector_id: Option<ConnectorId>,
    ) -> Result<Self, DerivedSignalLineageError> {
        if timestamp_end_ns < base.timestamp_start_ns {
            return Err(DerivedSignalLineageError::InvalidTimestampRange);
        }
        if operator_id.as_str().trim().is_empty() {
            return Err(DerivedSignalLineageError::EmptyOperatorId);
        }
        if operator_revision == 0 || operator_generation == 0 {
            return Err(DerivedSignalLineageError::ZeroOperatorVersion);
        }
        Ok(Self {
            base,
            timestamp_end_ns,
            operator_id,
            operator_revision,
            operator_generation,
            connector_id,
        })
    }

    pub fn semantic_role<'signal>(&self, signal: &'signal SignalSpec) -> Option<&'signal str> {
        signal.role.as_ref().map(SemanticRole::as_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum DerivedSignalLineageError {
    #[error("derived timestamp range ends before its base frame")]
    InvalidTimestampRange,
    #[error("derived operator id is empty")]
    EmptyOperatorId,
    #[error("derived operator revision and generation must be non-zero")]
    ZeroOperatorVersion,
}

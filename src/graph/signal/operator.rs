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
    #[doc = "Prepares resources required by `AsyncNode`."]
    fn prepare<'a>(
        &'a mut self,
        cx: &'a AsyncOperatorPrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>>;

    #[doc = "Processes an input value through `AsyncNode`."]
    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>>;

    #[doc = "Returns the process port held by `AsyncNode`."]
    fn process_port<'a>(
        &'a mut self,
        _input_port: &'a str,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        self.process(input)
    }

    #[doc = "Flushes pending output from `AsyncNode` at the end of a run."]
    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async { Ok(Vec::new()) })
    }

    #[doc = "Requests cancellation of `AsyncNode`."]
    fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }

    #[doc = "Closes `AsyncNode` to further work."]
    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Configures operator permission."]
pub struct OperatorPermissionPolicy {
    #[doc = "Reports whether network is allowed for `OperatorPermissionPolicy`."]
    pub network_allowed: bool,
    #[doc = "Reports whether filesystem is allowed for `OperatorPermissionPolicy`."]
    pub filesystem_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Configures operator deadline."]
pub struct OperatorDeadlinePolicy {
    #[doc = "Stores the process timeout value for `OperatorDeadlinePolicy`, in milliseconds."]
    pub process_timeout_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the operator cancellation policy used by PocketStation."]
pub enum OperatorCancellationPolicy {
    #[doc = "Cancels an operator using the discard queued policy."]
    DiscardQueued,
    #[doc = "Cancels an operator using the drain queued policy."]
    DrainQueued,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the operator failure policy used by PocketStation."]
pub enum OperatorFailurePolicy {
    #[doc = "Handles an operator failure using the continue policy."]
    Continue,
    #[doc = "Handles an operator failure using the stop worker policy."]
    StopWorker,
}

#[derive(Debug, Clone, Default)]
#[doc = "Configures operator output role."]
pub struct OperatorOutputRolePolicy {
    #[doc = "Stores the allowed component of `OperatorOutputRolePolicy`."]
    pub allowed: Vec<SemanticRole>,
    #[doc = "Indicates whether terminal applies to `OperatorOutputRolePolicy`."]
    pub terminal: Vec<SemanticRole>,
}

impl OperatorOutputRolePolicy {
    #[doc = "Returns whether accepts is true for `OperatorOutputRolePolicy`."]
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

    #[doc = "Reports whether terminal is true for `OperatorOutputRolePolicy`."]
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
#[doc = "Declares an asynchronous operator's ports, execution partition, failure policy, and cancellation policy."]
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
    #[doc = "Creates a new `AsyncOperatorManifest`."]
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

    #[doc = "Returns the operator identifier held by `AsyncOperatorManifest`."]
    pub const fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

    #[doc = "Returns the revision held by `AsyncOperatorManifest`."]
    pub const fn revision(&self) -> u32 {
        self.revision
    }

    #[doc = "Returns the generation held by `AsyncOperatorManifest`."]
    pub const fn generation(&self) -> u32 {
        self.generation
    }

    #[doc = "Returns the node held by `AsyncOperatorManifest`."]
    pub const fn node(&self) -> &NodeDescriptor {
        &self.node
    }

    #[doc = "Returns the input edge held by `AsyncOperatorManifest`."]
    pub const fn input_edge(&self) -> EdgeContract {
        self.input_edge
    }

    #[doc = "Returns the output edge held by `AsyncOperatorManifest`."]
    pub const fn output_edge(&self) -> EdgeContract {
        self.output_edge
    }

    #[doc = "Returns the queue capacity frames held by `AsyncOperatorManifest`."]
    pub const fn queue_capacity_frames(&self) -> usize {
        self.queue_capacity_frames
    }

    #[doc = "Returns the permission held by `AsyncOperatorManifest`."]
    pub const fn permission(&self) -> OperatorPermissionPolicy {
        self.permission
    }

    #[doc = "Returns the deadline held by `AsyncOperatorManifest`."]
    pub const fn deadline(&self) -> OperatorDeadlinePolicy {
        self.deadline
    }

    #[doc = "Returns the cancellation held by `AsyncOperatorManifest`."]
    pub const fn cancellation(&self) -> OperatorCancellationPolicy {
        self.cancellation
    }

    #[doc = "Returns the failure held by `AsyncOperatorManifest`."]
    pub const fn failure(&self) -> OperatorFailurePolicy {
        self.failure
    }

    #[doc = "Returns the output roles held by `AsyncOperatorManifest`."]
    pub const fn output_roles(&self) -> &OperatorOutputRolePolicy {
        &self.output_roles
    }

    #[doc = "Returns the input ports held by `AsyncOperatorManifest`."]
    pub fn input_ports(&self) -> impl Iterator<Item = &PortSpec> {
        self.node
            .inputs
            .iter()
            .filter(|port| port.direction == PortDirection::Input)
    }

    #[doc = "Returns the output ports held by `AsyncOperatorManifest`."]
    pub fn output_ports(&self) -> impl Iterator<Item = &PortSpec> {
        self.node
            .outputs
            .iter()
            .filter(|port| port.direction == PortDirection::Output)
    }

    #[doc = "Validates `AsyncOperatorManifest` against its declared contract."]
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
#[doc = "Classifies failures surfaced by async operator manifest operations."]
pub enum AsyncOperatorManifestError {
    #[error("operator id is empty")]
    #[doc = "Reports that operator identifier is empty."]
    EmptyOperatorId,
    #[error("operator revision must be non-zero")]
    #[doc = "Reports that revision must be greater than zero."]
    ZeroRevision,
    #[error("operator generation must be non-zero")]
    #[doc = "Reports that generation must be greater than zero."]
    ZeroGeneration,
    #[error("operator queue capacity must be non-zero")]
    #[doc = "Reports that queue capacity must be greater than zero."]
    ZeroQueueCapacity,
    #[error("operator process timeout must be non-zero")]
    #[doc = "Reports that process timeout must be greater than zero."]
    ZeroProcessTimeout,
    #[error("async operator cannot execute in a realtime partition")]
    #[doc = "Classifies a failure at the realtime partition stage or component of `AsyncOperatorManifestError`."]
    RealtimePartition,
    #[error("operator safety contract does not match its execution partition")]
    #[doc = "Reports that the supplied safety contract is invalid."]
    InvalidSafetyContract,
    #[error("operator safety contract requires network permission")]
    #[doc = "Reports that network permission does not match the expected contract."]
    NetworkPermissionMismatch,
    #[error("operator manifest has no typed input port")]
    #[doc = "Reports that the required input port is missing."]
    MissingInputPort,
    #[error("operator manifest has no typed output port")]
    #[doc = "Reports that the required output port is missing."]
    MissingOutputPort,
    #[error("async operator bridge currently requires DropNewest backpressure")]
    #[doc = "Reports that the requested backpressure is unsupported."]
    UnsupportedBackpressure,
    #[error("async operator bridge requires CopyToBranchPool input ownership")]
    #[doc = "Reports that the requested input copy policy is unsupported."]
    UnsupportedInputCopyPolicy,
    #[error("async operator output currently requires BoundedQueue backpressure")]
    #[doc = "Reports that the requested output backpressure is unsupported."]
    UnsupportedOutputBackpressure,
    #[error("operator input edge media does not match its typed input port")]
    #[doc = "Reports that input edge media does not match the expected contract."]
    InputEdgeMediaMismatch,
    #[error("operator output edge media does not match its typed output port")]
    #[doc = "Reports that output edge media does not match the expected contract."]
    OutputEdgeMediaMismatch,
    #[error("operator input SignalSpec is invalid")]
    #[doc = "Reports that the supplied input signal is invalid."]
    InvalidInputSignal,
    #[error("operator output SignalSpec is invalid")]
    #[doc = "Reports that the supplied output signal is invalid."]
    InvalidOutputSignal,
    #[error("operator input SignalSpec does not have a compatible payload representation")]
    #[doc = "Reports that input signal media does not match the expected contract."]
    InputSignalMediaMismatch,
    #[error("operator output SignalSpec does not have a compatible payload representation")]
    #[doc = "Reports that output signal media does not match the expected contract."]
    OutputSignalMediaMismatch,
    #[error("operator output role declarations cannot be empty strings")]
    #[doc = "Reports that output role is empty."]
    EmptyOutputRole,
    #[error("operator output role declarations cannot contain duplicates")]
    #[doc = "Reports that output role duplicates an existing declaration or record."]
    DuplicateOutputRole,
    #[error("every terminal output role must also be an allowed output role")]
    #[doc = "Classifies a failure at the terminal output role not allowed stage or component of `AsyncOperatorManifestError`."]
    TerminalOutputRoleNotAllowed,
}

#[doc = "Implement this trait to provide async operator behavior to PocketStation; its methods define the preparation and runtime contract."]
pub trait AsyncOperatorFactory: Send + Sync {
    #[doc = "Returns the manifest held by `AsyncOperatorFactory`."]
    fn manifest(&self) -> &AsyncOperatorManifest;
    #[doc = "Validates supplied node configuration against the schema declared by `AsyncOperatorFactory`."]
    fn validate_config(&self, configuration: &NodeConfig) -> Result<(), crate::graph::ConfigError>;
    #[doc = "Resolves and validates the operator manifest exposed by `AsyncOperatorFactory`."]
    fn resolve_manifest(
        &self,
        configuration: &NodeConfig,
    ) -> Result<AsyncOperatorManifest, crate::graph::ConfigError> {
        self.validate_config(configuration)?;
        Ok(self.manifest().clone())
    }
    #[doc = "Creates the runtime implementation described by `AsyncOperatorFactory`."]
    fn create(&self, configuration: &NodeConfig) -> Result<Box<dyn AsyncNode>, NodeError>;
}

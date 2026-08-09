//! pocketstation-graph — builds a typed GraphSpec.
//!
//! The builder (`dsl::Pipeline`) only assembles a `GraphSpec`; it does not execute.
//! Compilation (verification + lowering → RuntimePlan) and execution live downstream
//! (Waves 4–6). Nodes are registry-backed (`registry::NodeRegistry`), not closed enums,
//! so third parties register `NodeFactory`s instead of editing this crate.
//!
//! Phase 2 additions (CRATE_OWNERSHIP.md):
//!   - `signal`: `SignalSpec`, `SignalClass`, `SemanticRole`, `SchemaRef`, `SignalId`
//!   - `partition`: `ExecutionPartition` (WHERE), `SafetyContract` (WHAT)
//!   - `async_node`: `AsyncNode` for model/connector work outside realtime partitions

pub mod async_node;
pub mod async_operator;
pub mod builtins;
pub mod compiler;
pub mod contracts;
pub mod dsl;
pub mod ir;
pub mod node;
pub mod operator;
pub mod partition;
pub mod plan;
pub mod planner;
pub mod registry;
pub mod runtime_node;
pub mod signal;
#[cfg(test)]
mod source;
pub mod spec;

pub use async_node::{
    AsyncNode, AsyncNodeFuture, SignalContinuityError, SignalContinuityObservation,
    SignalContinuityTracker, SignalDerivation, SignalDerivationError, SignalEnvelope,
    SignalEnvelopeError, SignalLineage, SignalPayload, SignalTiming,
};
pub use async_operator::{
    AsyncOperatorFactory, AsyncOperatorManifest, AsyncOperatorManifestError,
    OperatorCancellationPolicy, OperatorDeadlinePolicy, OperatorFailurePolicy,
    OperatorOutputRolePolicy, OperatorPermissionPolicy,
};
pub use builtins::register_builtins;
pub use contracts::{
    AudioCaps, BackpressurePolicy, ChannelLayout, ClockDomain, CopyPolicy, DeliverySemantics,
    EdgeContract, EdgeObservabilityLevel, LossPolicy, MediaCaps, MediaKind, Multiplicity,
    PortDirection, PortSpec,
};
pub use dsl::{NodeHandle, Pipeline};
pub use node::{ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PrepareContext};
pub use operator::{OperatorId, OPERATOR_ID_SYNTAX_VERSION};
pub use partition::{ExecutionPartition, SafetyContract};
pub use registry::{
    NodeDefinition, NodeDefinitionRef, NodeFactory, NodeRegistrationError, NodeRegistry,
};
pub use runtime_node::RuntimeNode;
pub use signal::{
    BinaryFormat, Codec, EventFormat, SchemaRef, SemanticRole, SignalClass, SignalId, SignalSpec,
    SignalSpecError, TextFormat,
};
pub use spec::{EdgeId, EdgeSpec, GraphSpec, InputPortRef, NodeId, NodeSpec, OutputPortRef};

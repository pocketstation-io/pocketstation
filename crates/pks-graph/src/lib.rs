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
pub mod builtins;
pub mod compiler;
pub mod dsl;
pub mod ir;
pub mod node;
pub mod partition;
pub mod plan;
pub mod planner;
pub mod registry;
pub mod runtime_node;
pub mod signal;
pub mod spec;

pub use async_node::{AsyncEnvelope, AsyncNode, AsyncNodeFuture, AsyncSignal};
pub use builtins::register_builtins;
pub use dsl::{NodeHandle, Pipeline};
pub use node::{ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PrepareContext};
pub use partition::{ExecutionPartition, SafetyContract};
pub use registry::{NodeFactory, NodeRegistry};
pub use runtime_node::RuntimeNode;
pub use signal::{
    BinaryFormat, Codec, EventFormat, SchemaRef, SemanticRole, SignalClass, SignalId, SignalSpec,
    TextFormat,
};
pub use spec::{EdgeId, EdgeSpec, GraphSpec, InputPortRef, NodeId, NodeSpec, OutputPortRef};

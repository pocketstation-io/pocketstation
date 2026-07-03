//! pocketstation-graph — builds a typed GraphSpec.
//!
//! The builder (`dsl::Pipeline`) only assembles a `GraphSpec`; it does not execute.
//! Compilation (verification + lowering → RuntimePlan) and execution live downstream
//! (Waves 4–6). Nodes are registry-backed (`registry::NodeRegistry`), not closed enums,
//! so third parties register `NodeFactory`s instead of editing this crate.

pub mod builtins;
pub mod compiler;
pub mod dsl;
pub mod ir;
pub mod node;
pub mod plan;
pub mod planner;
pub mod registry;
pub mod runtime_node;
pub mod spec;

pub use builtins::register_builtins;
pub use dsl::{NodeHandle, Pipeline};
pub use node::{
    ConfigError, ExecutionClass, NodeConfig, NodeDescriptor, NodeError, NodeKind, NodeTypeId,
    PrepareContext,
};
pub use registry::{NodeFactory, NodeRegistry};
pub use runtime_node::RuntimeNode;
pub use spec::{EdgeId, EdgeSpec, GraphSpec, InputPortRef, NodeId, NodeSpec, OutputPortRef};

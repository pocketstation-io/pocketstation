//! GraphSpec — the immutable description a builder produces and the compiler consumes.
//! Node specifications are preserved verbatim (type id + config); nothing is discarded.

use crate::graph::contracts::EdgeContract;
use crate::graph::node::{NodeConfig, NodeTypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub(crate) u32);

impl NodeId {
    /// Creates a stable runtime node identifier for externally assembled plans.
    pub const fn from_index(index: u32) -> Self {
        Self(index)
    }

    pub fn index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeId(pub(crate) u32);

impl EdgeId {
    pub fn index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputPortRef {
    pub node: NodeId,
    pub port: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputPortRef {
    pub node: NodeId,
    pub port: String,
}

#[derive(Debug, Clone)]
pub struct NodeSpec {
    pub id: NodeId,
    pub type_id: NodeTypeId,
    pub config: NodeConfig,
}

#[derive(Debug, Clone)]
pub struct EdgeSpec {
    pub id: EdgeId,
    pub from: OutputPortRef,
    pub to: InputPortRef,
    pub requested: Option<EdgeContract>, // None = compiler negotiates from port caps (Wave 4)
}

#[derive(Debug, Clone, Default)]
pub struct GraphSpec {
    pub nodes: Vec<NodeSpec>,
    pub edges: Vec<EdgeSpec>,
}

impl GraphSpec {
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    pub fn node(&self, id: NodeId) -> Option<&NodeSpec> {
        self.nodes.iter().find(|n| n.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_default_graph_spec_when_built_then_has_no_nodes_or_edges() {
        let spec = GraphSpec::default();
        assert_eq!(spec.node_count(), 0);
        assert_eq!(spec.edge_count(), 0);
    }

    #[test]
    fn given_node_spec_in_graph_when_looked_up_by_id_then_returns_it() {
        let spec = GraphSpec {
            nodes: vec![NodeSpec {
                id: NodeId(7),
                type_id: NodeTypeId::from("gain"),
                config: NodeConfig::new(),
            }],
            edges: Vec::new(),
        };
        assert_eq!(
            spec.node(NodeId(7)).map(|n| n.type_id.as_str()),
            Some("gain")
        );
        assert!(spec.node(NodeId(0)).is_none());
    }
}

//! GraphSpec — the immutable description a builder produces and the compiler consumes.
//! Node specifications are preserved verbatim (type id + config); nothing is discarded.

use crate::graph::node::{NodeConfig, NodeTypeId};
use crate::graph::ports::EdgeContract;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[doc = "Uniquely identifies node."]
pub struct NodeId(pub(crate) u32);

impl NodeId {
    /// Creates a stable runtime node identifier for externally assembled plans.
    pub const fn from_index(index: u32) -> Self {
        Self(index)
    }

    #[doc = "Returns the index associated with `NodeId`."]
    pub fn index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[doc = "Uniquely identifies edge."]
pub struct EdgeId(pub(crate) u32);

impl EdgeId {
    #[doc = "Returns the index associated with `EdgeId`."]
    pub fn index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Represents output port ref in the PocketStation API."]
pub struct OutputPortRef {
    #[doc = "Stores the node associated with `OutputPortRef`."]
    pub node: NodeId,
    #[doc = "Stores the port associated with `OutputPortRef`."]
    pub port: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Represents input port ref in the PocketStation API."]
pub struct InputPortRef {
    #[doc = "Stores the node associated with `InputPortRef`."]
    pub node: NodeId,
    #[doc = "Stores the port associated with `InputPortRef`."]
    pub port: String,
}

#[derive(Debug, Clone)]
#[doc = "Configures node."]
pub struct NodeSpec {
    #[doc = "Identifies the id associated with `NodeSpec`."]
    pub id: NodeId,
    #[doc = "Identifies the type associated with `NodeSpec`."]
    pub type_id: NodeTypeId,
    #[doc = "Stores the config associated with `NodeSpec`."]
    pub config: NodeConfig,
}

#[derive(Debug, Clone)]
#[doc = "Configures edge."]
pub struct EdgeSpec {
    #[doc = "Identifies the id associated with `EdgeSpec`."]
    pub id: EdgeId,
    #[doc = "Identifies the origin represented by `EdgeSpec`."]
    pub from: OutputPortRef,
    #[doc = "Identifies the destination represented by `EdgeSpec`."]
    pub to: InputPortRef,
    #[doc = "Stores the requested associated with `EdgeSpec`."]
    pub requested: Option<EdgeContract>, // None = compiler negotiates from port caps (Wave 4)
}

#[derive(Debug, Clone, Default)]
#[doc = "Configures graph."]
pub struct GraphSpec {
    #[doc = "Stores the nodes associated with `GraphSpec`."]
    pub nodes: Vec<NodeSpec>,
    #[doc = "Stores the edges associated with `GraphSpec`."]
    pub edges: Vec<EdgeSpec>,
}

impl GraphSpec {
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    #[cfg(any(test, feature = "internal-testing"))]
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

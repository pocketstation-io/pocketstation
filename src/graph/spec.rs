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

    #[doc = "Returns the index held by `NodeId`."]
    pub fn index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[doc = "Uniquely identifies edge."]
pub struct EdgeId(pub(crate) u32);

impl EdgeId {
    #[doc = "Returns the index held by `EdgeId`."]
    pub fn index(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Names an operator output port used as the origin of a graph connection."]
pub struct OutputPortRef {
    #[doc = "References the node participating in `OutputPortRef`."]
    pub node: NodeId,
    #[doc = "References the port participating in `OutputPortRef`."]
    pub port: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Names an operator or endpoint input port used as the target of a graph connection."]
pub struct InputPortRef {
    #[doc = "References the node participating in `InputPortRef`."]
    pub node: NodeId,
    #[doc = "References the port participating in `InputPortRef`."]
    pub port: String,
}

#[derive(Debug, Clone)]
#[doc = "Configures node."]
pub struct NodeSpec {
    #[doc = "Identifies the id recorded by `NodeSpec`."]
    pub id: NodeId,
    #[doc = "Identifies the type identifier recorded by `NodeSpec`."]
    pub type_id: NodeTypeId,
    #[doc = "Stores the config as a `NodeConfig` value in `NodeSpec`."]
    pub config: NodeConfig,
}

#[derive(Debug, Clone)]
#[doc = "Configures edge."]
pub struct EdgeSpec {
    #[doc = "Identifies the id recorded by `EdgeSpec`."]
    pub id: EdgeId,
    #[doc = "Identifies the origin represented by `EdgeSpec`."]
    pub from: OutputPortRef,
    #[doc = "Identifies the destination represented by `EdgeSpec`."]
    pub to: InputPortRef,
    #[doc = "Stores the requested component of `EdgeSpec`."]
    pub requested: Option<EdgeContract>, // None = compiler negotiates from port caps (Wave 4)
}

#[derive(Debug, Clone, Default)]
#[doc = "Configures graph."]
pub struct GraphSpec {
    #[doc = "References the nodes participating in `GraphSpec`."]
    pub nodes: Vec<NodeSpec>,
    #[doc = "References the edges participating in `GraphSpec`."]
    pub edges: Vec<EdgeSpec>,
}

impl GraphSpec {
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the node count held by `GraphSpec`."]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the edge count held by `GraphSpec`."]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the node held by `GraphSpec`."]
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

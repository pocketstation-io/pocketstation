//! Typed intermediate representation the compiler produces from a `GraphSpec`.
//! Resolution attaches each node's descriptor; the verification passes fill the
//! negotiated media, edge contracts, and topological order in place.

use crate::graph::node::NodeDescriptor;
use crate::graph::ports::{EdgeContract, MediaCaps};
use crate::graph::spec::{EdgeSpec, NodeId, NodeSpec};

#[derive(Debug, Clone)]
#[doc = "Represents the executable graph node for resolved."]
pub struct ResolvedNode {
    #[doc = "Stores the spec as a `NodeSpec` value in `ResolvedNode`."]
    pub spec: NodeSpec,
    #[doc = "Stores the descriptor as a `NodeDescriptor` value in `ResolvedNode`."]
    pub descriptor: NodeDescriptor,
}

impl ResolvedNode {
    #[doc = "Returns the id held by `ResolvedNode`."]
    pub fn id(&self) -> NodeId {
        self.spec.id
    }
    #[doc = "Returns the type str held by `ResolvedNode`."]
    pub fn type_str(&self) -> &str {
        self.spec.type_id.as_str()
    }
}

#[derive(Debug, Clone)]
#[doc = "Binds one compiled graph edge to its resolved source, destination, and contract."]
pub struct ResolvedEdge {
    #[doc = "Stores the spec as a `EdgeSpec` value in `ResolvedEdge`."]
    pub spec: EdgeSpec,
    #[doc = "Records the media selected for `ResolvedEdge`."]
    pub media: MediaCaps, // negotiated by NegotiateCapsPass; MediaCaps::Any until then
    #[doc = "Stores the contract component of `ResolvedEdge`."]
    pub contract: Option<EdgeContract>, // None until NegotiateCapsPass records one
}

#[derive(Debug, Clone)]
#[doc = "Contains the resolved nodes, edges, and topological order consumed by runtime planning."]
pub struct GraphIr {
    #[doc = "References the nodes participating in `GraphIr`."]
    pub nodes: Vec<ResolvedNode>,
    #[doc = "References the edges participating in `GraphIr`."]
    pub edges: Vec<ResolvedEdge>,
    #[doc = "Lists graph nodes in the validated topological execution order for `GraphIr`."]
    pub topo_order: Vec<NodeId>, // empty until CycleDetectionPass fills a valid order
}

impl GraphIr {
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the node count held by `GraphIr`."]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    #[doc = "Returns the edge count held by `GraphIr`."]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    #[doc = "Returns the node held by `GraphIr`."]
    pub fn node(&self, id: NodeId) -> Option<&ResolvedNode> {
        self.nodes.iter().find(|node| node.spec.id == id)
    }

    #[doc = "Returns the topo order held by `GraphIr`."]
    pub fn topo_order(&self) -> &[NodeId] {
        &self.topo_order
    }
}

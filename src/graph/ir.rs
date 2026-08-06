//! Typed intermediate representation the compiler produces from a `GraphSpec`.
//! Resolution attaches each node's descriptor; the verification passes fill the
//! negotiated media, edge contracts, and topological order in place.

use crate::graph::contracts::{EdgeContract, MediaCaps};
use crate::graph::node::NodeDescriptor;
use crate::graph::spec::{EdgeSpec, NodeId, NodeSpec};

#[derive(Debug, Clone)]
pub struct ResolvedNode {
    pub spec: NodeSpec,
    pub descriptor: NodeDescriptor,
}

impl ResolvedNode {
    pub fn id(&self) -> NodeId {
        self.spec.id
    }
    pub fn type_str(&self) -> &str {
        self.spec.type_id.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedEdge {
    pub spec: EdgeSpec,
    pub media: MediaCaps, // negotiated by NegotiateCapsPass; MediaCaps::Any until then
    pub contract: Option<EdgeContract>, // None until NegotiateCapsPass records one
}

#[derive(Debug, Clone)]
pub struct GraphIr {
    pub nodes: Vec<ResolvedNode>,
    pub edges: Vec<ResolvedEdge>,
    pub topo_order: Vec<NodeId>, // empty until CycleDetectionPass fills a valid order
}

impl GraphIr {
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn node(&self, id: NodeId) -> Option<&ResolvedNode> {
        self.nodes.iter().find(|node| node.spec.id == id)
    }

    pub fn topo_order(&self) -> &[NodeId] {
        &self.topo_order
    }
}

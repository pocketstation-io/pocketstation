//! Ergonomic builder frontend. It only assembles a GraphSpec — it never executes.
//! Compilation (Wave 4) and execution (Wave 6) consume the spec downstream.

use crate::graph::node::{NodeConfig, NodeTypeId};
use crate::graph::ports::EdgeContract;
use crate::graph::spec::{
    EdgeId, EdgeSpec, GraphSpec, InputPortRef, NodeId, NodeSpec, OutputPortRef,
};

#[doc = "Owns bounded access to node."]
pub struct NodeHandle {
    id: NodeId,
}

impl NodeHandle {
    #[doc = "Returns the id held by `NodeHandle`."]
    pub fn id(&self) -> NodeId {
        self.id
    }
    #[doc = "Selects a named output port from `NodeHandle`."]
    pub fn out(&self, port: &str) -> OutputPortRef {
        OutputPortRef {
            node: self.id,
            port: port.to_owned(),
        }
    }
    #[doc = "Selects a named input port from `NodeHandle`."]
    pub fn in_(&self, port: &str) -> InputPortRef {
        InputPortRef {
            node: self.id,
            port: port.to_owned(),
        }
    }
}

#[derive(Default)]
#[doc = "Builds typed operator connections on a Session while preserving port and signal contracts."]
pub struct Pipeline {
    spec: GraphSpec,
    next_node: u32,
    next_edge: u32,
}

impl Pipeline {
    #[doc = "Creates a new `Pipeline`."]
    pub fn new() -> Self {
        Self::default()
    }

    #[doc = "Adds node for `Pipeline`."]
    pub fn add_node(&mut self, type_id: impl Into<NodeTypeId>, config: NodeConfig) -> NodeHandle {
        let id = NodeId(self.next_node);
        self.next_node += 1;
        self.spec.nodes.push(NodeSpec {
            id,
            type_id: type_id.into(),
            config,
        });
        NodeHandle { id }
    }

    #[doc = "Connects the requested ports through `Pipeline`."]
    pub fn connect(&mut self, from: OutputPortRef, to: InputPortRef) -> EdgeId {
        self.push_edge(from, to, None)
    }

    #[doc = "Connects pipeline ports using the supplied edge contract on `Pipeline`."]
    pub fn connect_with(
        &mut self,
        from: OutputPortRef,
        to: InputPortRef,
        contract: EdgeContract,
    ) -> EdgeId {
        self.push_edge(from, to, Some(contract))
    }

    fn push_edge(
        &mut self,
        from: OutputPortRef,
        to: InputPortRef,
        requested: Option<EdgeContract>,
    ) -> EdgeId {
        let id = EdgeId(self.next_edge);
        self.next_edge += 1;
        self.spec.edges.push(EdgeSpec {
            id,
            from,
            to,
            requested,
        });
        id
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the spec held by `Pipeline`."]
    pub fn spec(&self) -> &GraphSpec {
        &self.spec
    }

    #[doc = "Converts `Pipeline` into spec."]
    pub fn into_spec(self) -> GraphSpec {
        self.spec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_two_added_nodes_when_into_spec_then_specs_are_preserved_with_distinct_ids() {
        let mut graph = Pipeline::new();
        let mic = graph.add_node("source.mic", NodeConfig::new());
        let gain = graph.add_node("gain", NodeConfig::new().with("gain_db", "-6.0"));
        assert_ne!(mic.id(), gain.id());

        let spec = graph.into_spec();
        assert_eq!(spec.node_count(), 2);
        assert_eq!(spec.nodes[0].type_id.as_str(), "source.mic");
        assert_eq!(spec.nodes[1].type_id.as_str(), "gain");
        assert_eq!(spec.nodes[1].config.get("gain_db"), Some("-6.0"));
    }

    #[test]
    fn given_connected_nodes_when_into_spec_then_edge_records_endpoints() {
        let mut graph = Pipeline::new();
        let mic = graph.add_node("source.mic", NodeConfig::new());
        let gain = graph.add_node("gain", NodeConfig::new().with("gain_db", "0.0"));
        let edge = graph.connect(mic.out("audio"), gain.in_("audio"));
        assert_eq!(edge.index(), 0);

        let spec = graph.into_spec();
        assert_eq!(spec.edge_count(), 1);
        assert_eq!(spec.edges[0].from.node, mic.id());
        assert_eq!(spec.edges[0].to.node, gain.id());
        assert!(spec.edges[0].requested.is_none());
    }

    #[test]
    fn given_connect_with_contract_when_into_spec_then_edge_carries_requested_contract() {
        let mut graph = Pipeline::new();
        let a = graph.add_node("source.mic", NodeConfig::new());
        let b = graph.add_node("sink.browser", NodeConfig::new());
        graph.connect_with(
            a.out("audio"),
            b.in_("audio"),
            EdgeContract::realtime_audio(),
        );

        let spec = graph.into_spec();
        assert!(spec.edges[0].requested.is_some());
    }
}

//! Typed authority connecting compiled graph nodes to Session declarations.
//!
//! Extension-owned `NodeConfig` never carries these identities. Runtime
//! preparation consumes this table directly, so IDs are not serialized to
//! strings and parsed back later.

use std::collections::HashMap;

use crate::frame::ConnectorId;
use crate::graph::NodeId;
use crate::session::{
    EndpointId, OperatorInstanceId, RouteId, SourceInstanceId, StemId, StreamOrigin,
};

#[derive(Debug)]
pub(crate) struct CompiledSessionBindings {
    nodes: HashMap<NodeId, CompiledNodeBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CompiledNodeBinding {
    StemSource {
        stem_id: StemId,
    },
    ExternalSource {
        source_instance_id: SourceInstanceId,
    },
    ExternalAudioIngress {
        source_instance_id: SourceInstanceId,
        output_port: String,
    },
    Operator {
        operator_instance_id: OperatorInstanceId,
    },
    Endpoint {
        route_id: RouteId,
        endpoint_id: EndpointId,
        connector_id: Option<ConnectorId>,
        origin: StreamOrigin,
    },
    GeneratedAudioIngress {
        stem_id: StemId,
    },
    GeneratedAudioBridge {
        stem_id: StemId,
        operator_instance_id: OperatorInstanceId,
    },
}

impl CompiledSessionBindings {
    pub(crate) fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub(crate) fn node(&self, node_id: NodeId) -> Option<&CompiledNodeBinding> {
        self.nodes.get(&node_id)
    }

    #[cfg(test)]
    pub(crate) fn node_mut(&mut self, node_id: NodeId) -> Option<&mut CompiledNodeBinding> {
        self.nodes.get_mut(&node_id)
    }

    pub(crate) fn insert_node(&mut self, node_id: NodeId, binding: CompiledNodeBinding) {
        let replaced = self.nodes.insert(node_id, binding);
        debug_assert!(replaced.is_none(), "compiled node binding replaced");
    }
}

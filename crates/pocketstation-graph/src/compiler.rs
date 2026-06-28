//! Verification-pass pipeline that turns a `GraphSpec` into a checked `GraphIr`.
//! Resolution binds descriptors; ordered passes validate ids, ports, media,
//! realtime boundaries, and acyclicity. Lowering/emit passes arrive in Wave 5.

use std::collections::{BTreeMap, BTreeSet};

use pocketstation_caps::{BackpressurePolicy, EdgeContract, MediaCaps, PortDirection, PortSpec};

use crate::ir::{GraphIr, ResolvedEdge, ResolvedNode};
use crate::node::{ExecutionClass, NodeDescriptor};
use crate::registry::NodeRegistry;
use crate::spec::{GraphSpec, NodeId};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CompileError {
    #[error("unknown node type: {0}")]
    UnknownNodeType(String),
    #[error("node {type_id} has invalid config: {reason}")]
    InvalidConfig { type_id: String, reason: String },
    #[error("edge references unknown node {0}")]
    UnknownNode(u32),
    #[error("node {node}: unknown port {port}")]
    UnknownPort { node: u32, port: String },
    #[error("node {node}: port {port} connected against its declared direction")]
    WrongPortDirection { node: u32, port: String },
    #[error("edge {edge}: media {from} incompatible with {to}")]
    MediaMismatch { edge: u32, from: String, to: String },
    #[error("edge {edge}: invalid realtime boundary: {reason}")]
    InvalidRealtimeEdge { edge: u32, reason: String },
    #[error("graph contains a cycle")]
    CycleDetected,
}

pub struct CompileContext<'a> {
    pub registry: &'a NodeRegistry,
}

impl<'a> CompileContext<'a> {
    pub fn new(registry: &'a NodeRegistry) -> Self {
        Self { registry }
    }
}

pub trait GraphPass {
    fn name(&self) -> &'static str;
    fn run(&self, ir: &mut GraphIr, cx: &CompileContext) -> Result<(), CompileError>;
}

fn find_node(nodes: &[ResolvedNode], id: NodeId) -> Result<&ResolvedNode, CompileError> {
    nodes
        .iter()
        .find(|node| node.id() == id)
        .ok_or(CompileError::UnknownNode(id.index()))
}

fn ensure_port(
    descriptor: &NodeDescriptor,
    node_index: u32,
    port: &str,
    want: PortDirection,
) -> Result<(), CompileError> {
    let (wanted, other) = match want {
        PortDirection::Output => (&descriptor.outputs, &descriptor.inputs),
        PortDirection::Input => (&descriptor.inputs, &descriptor.outputs),
    };
    if wanted.iter().any(|p| p.name == port && p.direction == want) {
        return Ok(());
    }
    if other.iter().any(|p| p.name == port) {
        return Err(CompileError::WrongPortDirection {
            node: node_index,
            port: port.to_owned(),
        });
    }
    Err(CompileError::UnknownPort {
        node: node_index,
        port: port.to_owned(),
    })
}

fn port_media(ports: &[PortSpec], name: &str) -> Option<MediaCaps> {
    ports.iter().find(|p| p.name == name).map(|p| p.media)
}

pub struct ValidateNodeIdsPass;

impl GraphPass for ValidateNodeIdsPass {
    fn name(&self) -> &'static str {
        "ValidateNodeIds"
    }

    fn run(&self, ir: &mut GraphIr, _cx: &CompileContext) -> Result<(), CompileError> {
        for edge in &ir.edges {
            for node_id in [edge.spec.from.node, edge.spec.to.node] {
                if ir.node(node_id).is_none() {
                    return Err(CompileError::UnknownNode(node_id.index()));
                }
            }
        }
        Ok(())
    }
}

pub struct ValidatePortsPass;

impl GraphPass for ValidatePortsPass {
    fn name(&self) -> &'static str {
        "ValidatePorts"
    }

    fn run(&self, ir: &mut GraphIr, _cx: &CompileContext) -> Result<(), CompileError> {
        for edge in &ir.edges {
            let from = find_node(&ir.nodes, edge.spec.from.node)?;
            ensure_port(
                &from.descriptor,
                edge.spec.from.node.index(),
                &edge.spec.from.port,
                PortDirection::Output,
            )?;
            let to = find_node(&ir.nodes, edge.spec.to.node)?;
            ensure_port(
                &to.descriptor,
                edge.spec.to.node.index(),
                &edge.spec.to.port,
                PortDirection::Input,
            )?;
        }
        Ok(())
    }
}

pub struct NegotiateCapsPass;

impl GraphPass for NegotiateCapsPass {
    fn name(&self) -> &'static str {
        "NegotiateCaps"
    }

    fn run(&self, ir: &mut GraphIr, _cx: &CompileContext) -> Result<(), CompileError> {
        let nodes = &ir.nodes;
        for edge in &mut ir.edges {
            let edge_index = edge.spec.id.index();
            let from = find_node(nodes, edge.spec.from.node)?;
            let to = find_node(nodes, edge.spec.to.node)?;
            let from_media = port_media(&from.descriptor.outputs, &edge.spec.from.port).ok_or(
                CompileError::UnknownPort {
                    node: edge.spec.from.node.index(),
                    port: edge.spec.from.port.clone(),
                },
            )?;
            let to_media = port_media(&to.descriptor.inputs, &edge.spec.to.port).ok_or(
                CompileError::UnknownPort {
                    node: edge.spec.to.node.index(),
                    port: edge.spec.to.port.clone(),
                },
            )?;
            if let Some(requested) = edge.spec.requested.as_ref() {
                if !requested.media.is_compatible_with(&from_media)
                    || !requested.media.is_compatible_with(&to_media)
                {
                    return Err(CompileError::MediaMismatch {
                        edge: edge_index,
                        from: format!("{from_media:?}"),
                        to: format!("{to_media:?}"),
                    });
                }
            }
            let negotiated =
                from_media
                    .negotiate(&to_media)
                    .ok_or(CompileError::MediaMismatch {
                        edge: edge_index,
                        from: format!("{from_media:?}"),
                        to: format!("{to_media:?}"),
                    })?;
            edge.media = negotiated;
            // Wave 5 derives a full contract from node policies; here we keep the
            // requested contract or fall back to a voice-safe default.
            edge.contract = Some(
                edge.spec
                    .requested
                    .unwrap_or_else(EdgeContract::voice_default),
            );
        }
        Ok(())
    }
}

pub struct ValidateRealtimeBoundariesPass;

impl GraphPass for ValidateRealtimeBoundariesPass {
    fn name(&self) -> &'static str {
        "ValidateRealtimeBoundaries"
    }

    fn run(&self, ir: &mut GraphIr, _cx: &CompileContext) -> Result<(), CompileError> {
        for edge in &ir.edges {
            let from = find_node(&ir.nodes, edge.spec.from.node)?;
            let to = find_node(&ir.nodes, edge.spec.to.node)?;
            let producer_async = matches!(
                from.descriptor.execution,
                ExecutionClass::AsyncIo | ExecutionClass::Network | ExecutionClass::ModelRemote
            );
            if producer_async && to.descriptor.execution.is_realtime() {
                let non_blocking = matches!(
                    edge.contract.map(|contract| contract.backpressure),
                    Some(BackpressurePolicy::DropNewest) | Some(BackpressurePolicy::DropOldest)
                );
                if !non_blocking {
                    return Err(CompileError::InvalidRealtimeEdge {
                        edge: edge.spec.id.index(),
                        reason: "async producer into realtime consumer requires a non-blocking \
                                 (DropNewest/DropOldest) bounded edge"
                            .to_owned(),
                    });
                }
            }
        }
        Ok(())
    }
}

pub struct CycleDetectionPass;

impl GraphPass for CycleDetectionPass {
    fn name(&self) -> &'static str {
        "CycleDetection"
    }

    fn run(&self, ir: &mut GraphIr, _cx: &CompileContext) -> Result<(), CompileError> {
        let mut in_degree: BTreeMap<NodeId, usize> =
            ir.nodes.iter().map(|node| (node.id(), 0)).collect();
        let mut adjacency: BTreeMap<NodeId, Vec<NodeId>> = ir
            .nodes
            .iter()
            .map(|node| (node.id(), Vec::new()))
            .collect();
        for edge in &ir.edges {
            adjacency
                .entry(edge.spec.from.node)
                .or_default()
                .push(edge.spec.to.node);
            *in_degree.entry(edge.spec.to.node).or_default() += 1;
        }
        let mut ready: BTreeSet<NodeId> = in_degree
            .iter()
            .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
            .collect();
        let mut order: Vec<NodeId> = Vec::with_capacity(ir.nodes.len());
        while let Some(id) = ready.pop_first() {
            order.push(id);
            let neighbors = adjacency.get(&id).cloned().unwrap_or_default();
            for neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        ready.insert(neighbor);
                    }
                }
            }
        }
        if order.len() != ir.nodes.len() {
            return Err(CompileError::CycleDetected);
        }
        ir.topo_order = order;
        Ok(())
    }
}

pub struct Compiler {
    passes: Vec<Box<dyn GraphPass>>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            passes: vec![
                Box::new(ValidateNodeIdsPass),
                Box::new(ValidatePortsPass),
                Box::new(NegotiateCapsPass),
                // deferred: InsertAdapterNodes (Wave 5 lowering)
                Box::new(ValidateRealtimeBoundariesPass),
                Box::new(CycleDetectionPass),
            ],
        }
    }

    pub fn compile(
        &self,
        spec: GraphSpec,
        registry: &NodeRegistry,
    ) -> Result<GraphIr, CompileError> {
        let mut ir = Self::resolve(spec, registry)?;
        let cx = CompileContext::new(registry);
        for pass in &self.passes {
            pass.run(&mut ir, &cx)?;
        }
        Ok(ir)
    }

    fn resolve(spec: GraphSpec, registry: &NodeRegistry) -> Result<GraphIr, CompileError> {
        let mut nodes = Vec::with_capacity(spec.nodes.len());
        for node_spec in spec.nodes {
            let factory = registry.get(&node_spec.type_id).ok_or_else(|| {
                CompileError::UnknownNodeType(node_spec.type_id.as_str().to_owned())
            })?;
            factory.validate_config(&node_spec.config).map_err(|err| {
                CompileError::InvalidConfig {
                    type_id: node_spec.type_id.as_str().to_owned(),
                    reason: err.to_string(),
                }
            })?;
            let descriptor = factory.descriptor();
            nodes.push(ResolvedNode {
                spec: node_spec,
                descriptor,
            });
        }
        let edges = spec
            .edges
            .into_iter()
            .map(|spec| ResolvedEdge {
                spec,
                media: MediaCaps::Any,
                contract: None,
            })
            .collect();
        Ok(GraphIr {
            nodes,
            edges,
            topo_order: Vec::new(),
        })
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use pocketstation_caps::{AudioCaps, ChannelLayout, Multiplicity};
    use pocketstation_frame::SampleFormat;
    use proptest::prelude::*;

    use crate::builtins::PassthroughNode;
    use crate::dsl::AudioGraph;
    use crate::node::{ConfigError, NodeConfig, NodeKind, NodeTypeId, PrepareContext};
    use crate::registry::NodeFactory;
    use crate::runtime_node::RuntimeNode;

    fn audio_media() -> MediaCaps {
        MediaCaps::Audio(AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: ChannelLayout::Any,
            format: SampleFormat::F32Interleaved,
        })
    }

    fn port(name: &str, direction: PortDirection, media: MediaCaps) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
            media,
            multiplicity: Multiplicity::One,
            required: true,
        }
    }

    fn test_descriptor(
        type_id: &'static str,
        kind: NodeKind,
        inputs: Vec<PortSpec>,
        outputs: Vec<PortSpec>,
        execution: ExecutionClass,
    ) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(type_id),
            display_name: "test",
            kind,
            inputs,
            outputs,
            execution,
            realtime_safe: execution.is_realtime(),
            stateful: false,
        }
    }

    fn unused_node() -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
        Ok(Box::new(PassthroughNode))
    }

    struct SourceFactory;
    impl NodeFactory for SourceFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "source",
                NodeKind::Source,
                Vec::new(),
                vec![port("audio", PortDirection::Output, audio_media())],
                ExecutionClass::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    struct TransformFactory;
    impl NodeFactory for TransformFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "transform",
                NodeKind::Transform,
                vec![port("audio", PortDirection::Input, audio_media())],
                vec![port("audio", PortDirection::Output, audio_media())],
                ExecutionClass::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    struct SinkFactory;
    impl NodeFactory for SinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "sink",
                NodeKind::Sink,
                vec![port("audio", PortDirection::Input, audio_media())],
                Vec::new(),
                ExecutionClass::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    struct AsyncModelFactory;
    impl NodeFactory for AsyncModelFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "model.async",
                NodeKind::Model,
                vec![port("audio", PortDirection::Input, audio_media())],
                vec![port("audio", PortDirection::Output, audio_media())],
                ExecutionClass::ModelRemote,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    struct TextSinkFactory;
    impl NodeFactory for TextSinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "sink.text",
                NodeKind::Sink,
                vec![port("text", PortDirection::Input, MediaCaps::Text)],
                Vec::new(),
                ExecutionClass::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    struct RejectingFactory;
    impl NodeFactory for RejectingFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "reject",
                NodeKind::Transform,
                vec![port("audio", PortDirection::Input, audio_media())],
                vec![port("audio", PortDirection::Output, audio_media())],
                ExecutionClass::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Err(ConfigError::Invalid {
                key: "test".to_owned(),
                reason: "always rejects".to_owned(),
            })
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    fn test_registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        registry.register(Arc::new(SourceFactory));
        registry.register(Arc::new(TransformFactory));
        registry.register(Arc::new(SinkFactory));
        registry.register(Arc::new(AsyncModelFactory));
        registry.register(Arc::new(TextSinkFactory));
        registry.register(Arc::new(RejectingFactory));
        registry
    }

    #[test]
    fn given_linear_graph_when_compiled_then_topo_orders_source_before_sink() {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        let source = graph.add_node("source", NodeConfig::new());
        let transform = graph.add_node("transform", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        graph.connect(source.out("audio"), transform.in_("audio"));
        graph.connect(transform.out("audio"), sink.in_("audio"));

        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();

        assert_eq!(ir.topo_order().len(), ir.node_count());
        let order: Vec<u32> = ir.topo_order().iter().map(|id| id.index()).collect();
        let position = |index: u32| order.iter().position(|&value| value == index).unwrap();
        assert!(position(source.id().index()) < position(sink.id().index()));
        assert!(ir.node(source.id()).is_some());
    }

    #[test]
    fn given_unregistered_type_when_compiled_then_unknown_node_type() {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        graph.add_node("does.not.exist", NodeConfig::new());

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert_eq!(
            error,
            CompileError::UnknownNodeType("does.not.exist".to_owned())
        );
    }

    #[test]
    fn given_node_with_rejecting_config_when_compiled_then_invalid_config() {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        graph.add_node("reject", NodeConfig::new());

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::InvalidConfig { .. }));
    }

    #[test]
    fn given_edge_to_missing_port_when_compiled_then_unknown_port() {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        let source = graph.add_node("source", NodeConfig::new());
        let transform = graph.add_node("transform", NodeConfig::new());
        graph.connect(source.out("audio"), transform.in_("missing"));

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::UnknownPort { .. }));
    }

    #[test]
    fn given_edge_into_output_port_when_compiled_then_wrong_port_direction() {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        let source = graph.add_node("source", NodeConfig::new());
        let transform = graph.add_node("transform", NodeConfig::new());
        graph.connect(transform.out("audio"), source.in_("audio"));

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::WrongPortDirection { .. }));
    }

    #[test]
    fn given_audio_output_into_text_input_when_compiled_then_media_mismatch() {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        let source = graph.add_node("source", NodeConfig::new());
        let text_sink = graph.add_node("sink.text", NodeConfig::new());
        graph.connect(source.out("audio"), text_sink.in_("text"));

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::MediaMismatch { .. }));
    }

    #[test]
    fn given_async_producer_into_realtime_consumer_with_bounded_edge_when_compiled_then_invalid_realtime_edge(
    ) {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        let model = graph.add_node("model.async", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        let bounded = EdgeContract {
            backpressure: BackpressurePolicy::BoundedQueue,
            ..EdgeContract::voice_default()
        };
        graph.connect_with(model.out("audio"), sink.in_("audio"), bounded);

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::InvalidRealtimeEdge { .. }));
    }

    #[test]
    fn given_async_producer_into_realtime_consumer_with_drop_newest_edge_when_compiled_then_ok() {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        let model = graph.add_node("model.async", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        graph.connect_with(
            model.out("audio"),
            sink.in_("audio"),
            EdgeContract::voice_default(), // backpressure == DropNewest
        );

        assert!(Compiler::new()
            .compile(graph.into_spec(), &registry)
            .is_ok());
    }

    #[test]
    fn given_two_node_cycle_when_compiled_then_cycle_detected() {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        let first = graph.add_node("transform", NodeConfig::new());
        let second = graph.add_node("transform", NodeConfig::new());
        graph.connect(first.out("audio"), second.in_("audio"));
        graph.connect(second.out("audio"), first.in_("audio"));

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert_eq!(error, CompileError::CycleDetected);
    }

    #[test]
    fn given_fixed_graph_when_compiled_then_topo_order_matches_golden_snapshot() {
        let registry = test_registry();
        let mut graph = AudioGraph::new();
        let source = graph.add_node("source", NodeConfig::new());
        let transform = graph.add_node("transform", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        graph.connect(source.out("audio"), transform.in_("audio"));
        graph.connect(transform.out("audio"), sink.in_("audio"));

        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();

        let order: Vec<u32> = ir.topo_order().iter().map(|id| id.index()).collect();
        assert_eq!(order, vec![0, 1, 2]);
    }

    proptest! {
        #[test]
        fn given_linear_chain_when_compiled_then_topo_order_equals_insertion_order(
            chain_len in 2usize..=8,
        ) {
            let registry = test_registry();
            let mut graph = AudioGraph::new();
            let mut handles = Vec::with_capacity(chain_len);
            for _ in 0..chain_len {
                handles.push(graph.add_node("transform", NodeConfig::new()));
            }
            for pair in handles.windows(2) {
                graph.connect(pair[0].out("audio"), pair[1].in_("audio"));
            }

            let ir = Compiler::new().compile(graph.into_spec(), &registry).unwrap();

            let order: Vec<u32> = ir.topo_order().iter().map(|id| id.index()).collect();
            let expected: Vec<u32> = (0..chain_len as u32).collect();
            prop_assert_eq!(order, expected);
        }
    }
}

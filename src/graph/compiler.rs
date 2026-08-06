//! Verification-pass pipeline that turns a `GraphSpec` into a checked `GraphIr`.
//! Resolution binds descriptors; ordered passes validate ids, ports, media,
//! clock domains, realtime boundaries, and acyclicity. Lowering into a
//! `RuntimePlan` lives in `planner::RuntimePlanner` (Wave 5).

use std::collections::{BTreeMap, BTreeSet};

use crate::graph::contracts::{
    BackpressurePolicy, ChannelLayout, ClockDomain, EdgeContract, MediaCaps, PortDirection,
    PortSpec,
};

use crate::graph::ir::{GraphIr, ResolvedEdge, ResolvedNode};
use crate::graph::node::{NodeConfig, NodeDescriptor, NodeTypeId};
use crate::graph::registry::NodeRegistry;
use crate::graph::spec::{
    EdgeId, EdgeSpec, GraphSpec, InputPortRef, NodeId, NodeSpec, OutputPortRef,
};

/// Canonical built-in adapter that downmixes a stereo output into a mono-only
/// input. Lives in `pocketstation-nodes`; referenced here by stable type id so
/// the compiler can auto-insert it (cf. GStreamer `audioconvert` autoplugging).
const MONO_DOWNMIX_ADAPTER_TYPE: &str = "transform.mono_mix";

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
    #[error("node {node}: input port {port} fans in clock domains {expected:?} and {found:?} without a clock adapter")]
    ClockDomainMismatch {
        node: u32,
        port: String,
        expected: ClockDomain,
        found: ClockDomain,
    },
    #[error("edge {edge}: media {from} incompatible with {to}")]
    MediaMismatch { edge: u32, from: String, to: String },
    #[error("edge {edge}: signal {from} incompatible with {to}")]
    SignalMismatch { edge: u32, from: String, to: String },
    #[error(
        "node {node} ({type_id}) safety {safety:?} is incompatible with partition {execution:?}"
    )]
    InvalidSafetyContract {
        node: u32,
        type_id: String,
        execution: crate::graph::partition::ExecutionPartition,
        safety: crate::graph::partition::SafetyContract,
    },
    #[error("edge {edge}: invalid realtime boundary: {reason}")]
    InvalidRealtimeEdge { edge: u32, reason: String },
    #[error("graph contains a cycle")]
    CycleDetected,
    #[error("edge {edge} needs adapter {type_id} but it is not registered")]
    AdapterUnavailable { edge: u32, type_id: String },
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

fn port_spec<'a>(ports: &'a [PortSpec], name: &str) -> Option<&'a PortSpec> {
    ports.iter().find(|port| port.name == name)
}

fn audio_layout(media: MediaCaps) -> Option<ChannelLayout> {
    match media {
        MediaCaps::Audio(caps) => Some(caps.channel_layout),
        _ => None,
    }
}

pub struct InsertAdapterNodesPass;

impl InsertAdapterNodesPass {
    /// An edge needs a mono downmix only when a concrete stereo output feeds a
    /// mono-only input. Stereo survives to `Any`/stereo consumers untouched
    /// (PocketStation Corrected Audit §4.4).
    fn needs_mono_downmix(from: ChannelLayout, to: ChannelLayout) -> bool {
        from == ChannelLayout::Stereo && to == ChannelLayout::Mono
    }
}

impl GraphPass for InsertAdapterNodesPass {
    fn name(&self) -> &'static str {
        "InsertAdapterNodes"
    }

    fn run(&self, ir: &mut GraphIr, cx: &CompileContext) -> Result<(), CompileError> {
        let mut targets: Vec<usize> = Vec::new();
        for (index, edge) in ir.edges.iter().enumerate() {
            let from = find_node(&ir.nodes, edge.spec.from.node)?;
            let to = find_node(&ir.nodes, edge.spec.to.node)?;
            let from_layout =
                port_media(&from.descriptor.outputs, &edge.spec.from.port).and_then(audio_layout);
            let to_layout =
                port_media(&to.descriptor.inputs, &edge.spec.to.port).and_then(audio_layout);
            if let (Some(from_layout), Some(to_layout)) = (from_layout, to_layout) {
                if Self::needs_mono_downmix(from_layout, to_layout) {
                    targets.push(index);
                }
            }
        }
        if targets.is_empty() {
            return Ok(());
        }

        let adapter_type = NodeTypeId::from(MONO_DOWNMIX_ADAPTER_TYPE);
        let factory =
            cx.registry
                .get(&adapter_type)
                .ok_or_else(|| CompileError::AdapterUnavailable {
                    edge: ir.edges[targets[0]].spec.id.index(),
                    type_id: MONO_DOWNMIX_ADAPTER_TYPE.to_owned(),
                })?;
        let descriptor = factory.descriptor();
        let in_port = descriptor.inputs[0].name.clone();
        let out_port = descriptor.outputs[0].name.clone();

        // Each target inserts exactly one adapter node + one edge, so the new
        // ids are the next free index plus the target's offset.
        let node_base = ir.nodes.iter().map(|n| n.id().index()).max().unwrap_or(0) + 1;
        let edge_base = ir
            .edges
            .iter()
            .map(|e| e.spec.id.index())
            .max()
            .unwrap_or(0)
            + 1;
        let mut added_edges: Vec<ResolvedEdge> = Vec::new();

        for (offset, &index) in targets.iter().enumerate() {
            let adapter_id = NodeId(node_base + offset as u32);
            ir.nodes.push(ResolvedNode {
                spec: NodeSpec {
                    id: adapter_id,
                    type_id: adapter_type.clone(),
                    config: NodeConfig::new(),
                },
                descriptor: descriptor.clone(),
            });

            // Reroute source → adapter.in, then adapter.out → original dest.
            let original_to = ir.edges[index].spec.to.clone();
            ir.edges[index].spec.to = InputPortRef {
                node: adapter_id,
                port: in_port.clone(),
            };
            added_edges.push(ResolvedEdge {
                spec: EdgeSpec {
                    id: EdgeId(edge_base + offset as u32),
                    from: OutputPortRef {
                        node: adapter_id,
                        port: out_port.clone(),
                    },
                    to: original_to,
                    requested: None,
                },
                media: MediaCaps::Any,
                contract: None,
            });
        }
        ir.edges.extend(added_edges);
        Ok(())
    }
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

pub struct ValidateSafetyContractsPass;

impl GraphPass for ValidateSafetyContractsPass {
    fn name(&self) -> &'static str {
        "ValidateSafetyContracts"
    }

    fn run(&self, ir: &mut GraphIr, _cx: &CompileContext) -> Result<(), CompileError> {
        for node in &ir.nodes {
            if !node
                .descriptor
                .safety
                .is_valid_for(node.descriptor.execution)
            {
                return Err(CompileError::InvalidSafetyContract {
                    node: node.id().index(),
                    type_id: node.type_str().to_owned(),
                    execution: node.descriptor.execution,
                    safety: node.descriptor.safety,
                });
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

pub struct ValidateClockDomainsPass;

impl GraphPass for ValidateClockDomainsPass {
    fn name(&self) -> &'static str {
        "ValidateClockDomains"
    }

    fn run(&self, ir: &mut GraphIr, _cx: &CompileContext) -> Result<(), CompileError> {
        // Every edge feeding one input port must share a clock domain: blindly
        // mixing sources from different clocks (e.g. Capture + Network) without a
        // resampling adapter drifts and glitches (ADR-006). The adapter that
        // bridges clocks is inserted in Wave 10 alongside MonoMix/StreamProfile.
        let mut seen: Vec<(InputPortRef, ClockDomain)> = Vec::new();
        for edge in &ir.edges {
            let clock = edge
                .contract
                .map_or(ClockDomain::Capture, |contract| contract.clock);
            match seen.iter().find(|(port, _)| *port == edge.spec.to) {
                Some((_, expected)) if *expected != clock => {
                    return Err(CompileError::ClockDomainMismatch {
                        node: edge.spec.to.node.index(),
                        port: edge.spec.to.port.clone(),
                        expected: *expected,
                        found: clock,
                    });
                }
                Some(_) => {}
                None => seen.push((edge.spec.to.clone(), clock)),
            }
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
            let from_port = port_spec(&from.descriptor.outputs, &edge.spec.from.port).ok_or(
                CompileError::UnknownPort {
                    node: edge.spec.from.node.index(),
                    port: edge.spec.from.port.clone(),
                },
            )?;
            let to_port = port_spec(&to.descriptor.inputs, &edge.spec.to.port).ok_or(
                CompileError::UnknownPort {
                    node: edge.spec.to.node.index(),
                    port: edge.spec.to.port.clone(),
                },
            )?;
            if !from_port.signal.is_compatible_with(&to_port.signal) {
                return Err(CompileError::SignalMismatch {
                    edge: edge_index,
                    from: format!("{:?}", from_port.signal),
                    to: format!("{:?}", to_port.signal),
                });
            }
            let from_media = from_port.media;
            let to_media = to_port.media;
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
            let producer_async = !from.descriptor.execution.requires_realtime_safety();
            if producer_async && to.descriptor.execution.requires_realtime_safety() {
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
                Box::new(InsertAdapterNodesPass),
                Box::new(ValidateSafetyContractsPass),
                Box::new(NegotiateCapsPass),
                Box::new(ValidateClockDomainsPass),
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
            let definition = registry.definition(&node_spec.type_id).ok_or_else(|| {
                CompileError::UnknownNodeType(node_spec.type_id.as_str().to_owned())
            })?;
            definition
                .validate_config(&node_spec.config)
                .map_err(|err| CompileError::InvalidConfig {
                    type_id: node_spec.type_id.as_str().to_owned(),
                    reason: err.to_string(),
                })?;
            let descriptor = definition.descriptor();
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

    use crate::frame::SampleFormat;
    use proptest::prelude::*;

    use crate::graph::builtins::PassthroughNode;
    use crate::graph::contracts::{AudioCaps, ChannelLayout, Multiplicity};
    use crate::graph::dsl::Pipeline;
    use crate::graph::node::{ConfigError, NodeConfig, NodeTypeId, PrepareContext};
    use crate::graph::partition::{ExecutionPartition, SafetyContract};
    use crate::graph::registry::NodeFactory;
    use crate::graph::runtime_node::RuntimeNode;

    fn audio_media() -> MediaCaps {
        audio_media_with(ChannelLayout::Any)
    }

    fn audio_media_with(layout: ChannelLayout) -> MediaCaps {
        MediaCaps::Audio(AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: layout,
            format: SampleFormat::F32Interleaved,
        })
    }

    fn port(name: &str, direction: PortDirection, media: MediaCaps) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
            signal: match media {
                MediaCaps::Audio(_) => crate::graph::SignalSpec::audio(),
                MediaCaps::Text => crate::graph::SignalSpec::text(crate::graph::TextFormat::Utf8),
                MediaCaps::Event => {
                    crate::graph::SignalSpec::event(crate::graph::EventFormat::Json)
                }
                MediaCaps::Control => crate::graph::SignalSpec::control(),
                MediaCaps::Any => crate::graph::SignalSpec::any(),
            },
            media,
            multiplicity: Multiplicity::One,
            required: true,
        }
    }

    fn test_descriptor(
        type_id: &'static str,
        inputs: Vec<PortSpec>,
        outputs: Vec<PortSpec>,
        execution: ExecutionPartition,
    ) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(type_id),
            display_name: "test",
            inputs,
            outputs,
            safety: if execution.requires_realtime_safety() {
                SafetyContract::RealtimeSafe
            } else {
                SafetyContract::ExternalService
            },
            execution,
            stateful: false,
        }
    }

    fn unused_node() -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
        Ok(Box::new(PassthroughNode))
    }

    struct SourceFactory;
    impl NodeFactory for SourceFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "source",
                Vec::new(),
                vec![port("audio", PortDirection::Output, audio_media())],
                ExecutionPartition::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct TransformFactory;
    impl NodeFactory for TransformFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "transform",
                vec![port("audio", PortDirection::Input, audio_media())],
                vec![port("audio", PortDirection::Output, audio_media())],
                ExecutionPartition::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct SinkFactory;
    impl NodeFactory for SinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "sink",
                vec![port("audio", PortDirection::Input, audio_media())],
                Vec::new(),
                ExecutionPartition::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct MixerFactory;
    impl NodeFactory for MixerFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "mixer",
                vec![PortSpec {
                    name: "audio".to_owned(),
                    direction: PortDirection::Input,
                    signal: crate::graph::SignalSpec::audio(),
                    media: audio_media(),
                    multiplicity: Multiplicity::Many,
                    required: true,
                }],
                vec![port("audio", PortDirection::Output, audio_media())],
                ExecutionPartition::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct AsyncModelFactory;
    impl NodeFactory for AsyncModelFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "model.async",
                vec![port("audio", PortDirection::Input, audio_media())],
                vec![port("audio", PortDirection::Output, audio_media())],
                ExecutionPartition::External,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct TextSinkFactory;
    impl NodeFactory for TextSinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "sink.text",
                vec![PortSpec {
                    name: "text".to_owned(),
                    direction: PortDirection::Input,
                    signal: crate::graph::SignalSpec::audio(),
                    media: MediaCaps::Text,
                    multiplicity: Multiplicity::One,
                    required: true,
                }],
                Vec::new(),
                ExecutionPartition::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct SignalTextSinkFactory;
    impl NodeFactory for SignalTextSinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "sink.signal_text",
                vec![PortSpec {
                    name: "text".to_owned(),
                    direction: PortDirection::Input,
                    signal: crate::graph::SignalSpec::text(crate::graph::TextFormat::Utf8),
                    media: audio_media(),
                    multiplicity: Multiplicity::One,
                    required: true,
                }],
                Vec::new(),
                ExecutionPartition::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct UnsafeRealtimeFactory;
    impl NodeFactory for UnsafeRealtimeFactory {
        fn descriptor(&self) -> NodeDescriptor {
            NodeDescriptor {
                type_id: NodeTypeId::from("unsafe.realtime"),
                display_name: "unsafe realtime test node",
                inputs: Vec::new(),
                outputs: Vec::new(),
                execution: ExecutionPartition::RealtimeCpu,
                safety: SafetyContract::AllocationAllowed,
                stateful: false,
            }
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct RejectingFactory;
    impl NodeFactory for RejectingFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "reject",
                vec![port("audio", PortDirection::Input, audio_media())],
                vec![port("audio", PortDirection::Output, audio_media())],
                ExecutionPartition::RealtimeCpu,
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
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct StereoSourceFactory;
    impl NodeFactory for StereoSourceFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "source.stereo",
                Vec::new(),
                vec![port(
                    "audio",
                    PortDirection::Output,
                    audio_media_with(ChannelLayout::Stereo),
                )],
                ExecutionPartition::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct MonoOnlySinkFactory;
    impl NodeFactory for MonoOnlySinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "sink.mono_only",
                vec![port(
                    "audio",
                    PortDirection::Input,
                    audio_media_with(ChannelLayout::Mono),
                )],
                Vec::new(),
                ExecutionPartition::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    struct StereoSinkFactory;
    impl NodeFactory for StereoSinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "sink.stereo",
                vec![port(
                    "audio",
                    PortDirection::Input,
                    audio_media_with(ChannelLayout::Stereo),
                )],
                Vec::new(),
                ExecutionPartition::RealtimeCpu,
            )
        }
        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }
        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, crate::graph::node::NodeError> {
            unused_node()
        }
    }

    fn test_registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        registry.register(Arc::new(SourceFactory)).unwrap();
        registry.register(Arc::new(TransformFactory)).unwrap();
        registry.register(Arc::new(SinkFactory)).unwrap();
        registry.register(Arc::new(MixerFactory)).unwrap();
        registry.register(Arc::new(AsyncModelFactory)).unwrap();
        registry.register(Arc::new(TextSinkFactory)).unwrap();
        registry.register(Arc::new(SignalTextSinkFactory)).unwrap();
        registry.register(Arc::new(UnsafeRealtimeFactory)).unwrap();
        registry.register(Arc::new(RejectingFactory)).unwrap();
        registry.register(Arc::new(StereoSourceFactory)).unwrap();
        registry.register(Arc::new(MonoOnlySinkFactory)).unwrap();
        registry.register(Arc::new(StereoSinkFactory)).unwrap();
        registry
            .register(Arc::new(crate::graph::builtins::MonoMixFactory))
            .unwrap();
        registry
    }

    #[test]
    fn given_linear_graph_when_compiled_then_topo_orders_source_before_sink() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
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
        let mut graph = Pipeline::new();
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
        let mut graph = Pipeline::new();
        graph.add_node("reject", NodeConfig::new());

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::InvalidConfig { .. }));
    }

    #[test]
    fn given_edge_to_missing_port_when_compiled_then_unknown_port() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
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
        let mut graph = Pipeline::new();
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
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let text_sink = graph.add_node("sink.text", NodeConfig::new());
        graph.connect(source.out("audio"), text_sink.in_("text"));

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::MediaMismatch { .. }));
    }

    #[test]
    fn given_audio_signal_into_text_signal_when_compiled_then_signal_mismatch() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let text_sink = graph.add_node("sink.signal_text", NodeConfig::new());
        graph.connect(source.out("audio"), text_sink.in_("text"));

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::SignalMismatch { .. }));
    }

    #[test]
    fn given_realtime_node_with_allocating_safety_when_compiled_then_safety_is_rejected() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        graph.add_node("unsafe.realtime", NodeConfig::new());

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::InvalidSafetyContract { .. }));
    }

    #[test]
    fn given_async_producer_into_realtime_consumer_with_bounded_edge_when_compiled_then_invalid_realtime_edge(
    ) {
        let registry = test_registry();
        let mut graph = Pipeline::new();
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
        let mut graph = Pipeline::new();
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
        let mut graph = Pipeline::new();
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
    fn given_fan_in_with_mismatched_clock_domains_when_compiled_then_clock_domain_mismatch() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let capture = graph.add_node("source", NodeConfig::new());
        let network = graph.add_node("source", NodeConfig::new());
        let mixer = graph.add_node("mixer", NodeConfig::new());
        graph.connect(capture.out("audio"), mixer.in_("audio")); // voice_default → Capture
        let network_edge = EdgeContract {
            clock: ClockDomain::Network,
            ..EdgeContract::voice_default()
        };
        graph.connect_with(network.out("audio"), mixer.in_("audio"), network_edge);

        let error = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap_err();

        assert!(matches!(error, CompileError::ClockDomainMismatch { .. }));
    }

    #[test]
    fn given_fan_in_with_consistent_clock_domains_when_compiled_then_ok() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let first = graph.add_node("source", NodeConfig::new());
        let second = graph.add_node("source", NodeConfig::new());
        let mixer = graph.add_node("mixer", NodeConfig::new());
        graph.connect(first.out("audio"), mixer.in_("audio")); // Capture
        graph.connect(second.out("audio"), mixer.in_("audio")); // Capture

        assert!(Compiler::new()
            .compile(graph.into_spec(), &registry)
            .is_ok());
    }

    #[test]
    fn given_stereo_source_into_mono_only_sink_when_compiled_then_mono_mix_adapter_inserted() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source.stereo", NodeConfig::new());
        let sink = graph.add_node("sink.mono_only", NodeConfig::new());
        graph.connect(source.out("audio"), sink.in_("audio"));

        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();

        // source + inserted adapter + sink; original edge split into two.
        assert_eq!(ir.node_count(), 3);
        assert_eq!(ir.edge_count(), 2);
        assert!(ir
            .nodes
            .iter()
            .any(|node| node.type_str() == MONO_DOWNMIX_ADAPTER_TYPE));
    }

    #[test]
    fn given_stereo_source_into_stereo_sink_when_compiled_then_stereo_survives_no_adapter() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source.stereo", NodeConfig::new());
        let sink = graph.add_node("sink.stereo", NodeConfig::new());
        graph.connect(source.out("audio"), sink.in_("audio"));

        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();

        assert_eq!(ir.node_count(), 2);
        assert_eq!(ir.edge_count(), 1);
        assert!(!ir
            .nodes
            .iter()
            .any(|node| node.type_str() == MONO_DOWNMIX_ADAPTER_TYPE));
    }

    #[test]
    fn given_fixed_graph_when_compiled_then_topo_order_matches_golden_snapshot() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
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
            let mut graph = Pipeline::new();
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

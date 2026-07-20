//! RuntimePlanner — lowers a verified `GraphIr` into a `RuntimePlan`.
//! The pipeline runs real lowering steps in order: fan-out grouping, fan-in
//! mix grouping (with multiplicity validation), execution-domain partitioning,
//! memory sizing, edge instrumentation, then emit.

use crate::ir::GraphIr;
use crate::partition::ExecutionPartition;
use crate::plan::*;
use crate::spec::{EdgeId, InputPortRef, NodeId, OutputPortRef};
use pks_caps::{CopyPolicy, EdgeContract, MediaCaps, Multiplicity};

pub struct RuntimePlanner;

impl RuntimePlanner {
    pub fn new() -> Self {
        Self
    }

    /// Lower a verified IR into an execution-ready plan.
    ///
    /// Adapter insertion (resample/remix) and linear-kernel fusion are deferred
    /// (see AUDIO-026); they are not run here. Every edge reaching this stage is
    /// already caps-compatible because Wave 4's `NegotiateCapsPass` rejected any
    /// incompatible edge, so the planner assumes compatible negotiated media.
    pub fn plan(&self, ir: &GraphIr) -> Result<RuntimePlan, PlanError> {
        let fan_out = self.lower_fan_out(ir);
        let fan_in = self.lower_fan_in_mix(ir)?;
        let partitions = self.partition_execution_domains(ir);
        let memory_plan = self.plan_memory(ir);
        self.validate_fan_out_ownership(&fan_out, &memory_plan)?;
        let edge_metrics = self.instrument_edges(ir);
        let node_order: Vec<NodeId> = ir.topo_order().to_vec();
        Ok(RuntimePlan {
            node_order,
            partitions,
            memory_plan,
            edge_metrics,
            fan_out,
            fan_in,
            edge_count: ir.edge_count(),
        })
    }

    fn lower_fan_out(&self, ir: &GraphIr) -> Vec<FanOutGroup> {
        let mut groups: Vec<(OutputPortRef, Vec<EdgeId>)> = Vec::new();
        for edge in &ir.edges {
            match groups.iter_mut().find(|entry| entry.0 == edge.spec.from) {
                Some(entry) => entry.1.push(edge.spec.id),
                None => groups.push((edge.spec.from.clone(), vec![edge.spec.id])),
            }
        }
        groups
            .into_iter()
            .filter(|(_, targets)| targets.len() > 1)
            .map(|(from, targets)| FanOutGroup { from, targets })
            .collect()
    }

    fn lower_fan_in_mix(&self, ir: &GraphIr) -> Result<Vec<FanInGroup>, PlanError> {
        let mut groups: Vec<(InputPortRef, Vec<EdgeId>)> = Vec::new();
        for edge in &ir.edges {
            match groups.iter_mut().find(|entry| entry.0 == edge.spec.to) {
                Some(entry) => entry.1.push(edge.spec.id),
                None => groups.push((edge.spec.to.clone(), vec![edge.spec.id])),
            }
        }
        let mut fan_in = Vec::new();
        for (into, sources) in groups {
            if sources.len() <= 1 {
                continue;
            }
            let multiplicity = ir
                .node(into.node)
                .and_then(|node| {
                    node.descriptor
                        .inputs
                        .iter()
                        .find(|port| port.name == into.port)
                })
                .map(|port| port.multiplicity);
            if multiplicity == Some(Multiplicity::One) {
                return Err(PlanError::FanInOnSinglePort {
                    node: into.node.index(),
                    port: into.port.clone(),
                });
            }
            fan_in.push(FanInGroup { into, sources });
        }
        Ok(fan_in)
    }

    fn partition_execution_domains(&self, ir: &GraphIr) -> Vec<PartitionGroup> {
        let topo = ir.topo_order();
        let mut seen: Vec<ExecutionPartition> = Vec::new();
        for id in topo {
            if let Some(node) = ir.node(*id) {
                let ep = node.descriptor.execution;
                if !seen.contains(&ep) {
                    seen.push(ep);
                }
            }
        }
        seen.sort_by_key(|ep| ep.rank());
        seen.into_iter()
            .map(|ep| {
                let nodes: Vec<NodeId> = topo
                    .iter()
                    .copied()
                    .filter(|id| {
                        ir.node(*id)
                            .is_some_and(|node| node.descriptor.execution == ep)
                    })
                    .collect();
                PartitionGroup {
                    execution: ep,
                    nodes,
                }
            })
            .collect()
    }

    fn plan_memory(&self, ir: &GraphIr) -> MemoryPlan {
        let mut edge_buffers = Vec::with_capacity(ir.edge_count());
        let mut realtime_pool_bytes = 0usize;
        let mut branch_copy_pool_bytes = 0usize;
        for edge in &ir.edges {
            let consumer_realtime = ir
                .node(edge.spec.to.node)
                .is_some_and(|node| node.descriptor.execution.requires_realtime_safety());
            let outgoing_count = ir
                .edges
                .iter()
                .filter(|candidate| candidate.spec.from == edge.spec.from)
                .count();
            let copy_policy = edge.spec.requested.map_or_else(
                || {
                    if !consumer_realtime {
                        CopyPolicy::CopyToBranchPool
                    } else if outgoing_count == 1 {
                        CopyPolicy::MoveExclusive
                    } else {
                        CopyPolicy::CopyToBranchPool
                    }
                },
                |contract| contract.copy_policy,
            );
            let buffer = EdgeBufferPlan {
                edge: edge.spec.id,
                capacity_frames: Self::capacity_frames(&edge.media, edge.spec.requested.as_ref()),
                bytes_per_frame: Self::bytes_per_frame(&edge.media),
                copy_policy,
            };
            if consumer_realtime {
                realtime_pool_bytes += buffer.total_bytes();
            }
            if copy_policy == CopyPolicy::CopyToBranchPool {
                branch_copy_pool_bytes += buffer.total_bytes();
            }
            edge_buffers.push(buffer);
        }
        MemoryPlan {
            realtime_pool_bytes,
            branch_copy_pool_bytes,
            edge_buffers,
        }
    }

    fn capacity_frames(media: &MediaCaps, requested: Option<&EdgeContract>) -> usize {
        let Some(jitter_budget_ms) = requested.and_then(|contract| contract.jitter_budget_ms)
        else {
            return EDGE_RING_CAPACITY_FRAMES;
        };
        let sizing_media = requested.map_or(media, |contract| &contract.media);
        let MediaCaps::Audio(audio) = sizing_media else {
            return EDGE_RING_CAPACITY_FRAMES;
        };
        let (Some(sample_rate_hz), Some(frame_samples)) =
            (audio.sample_rate_hz, audio.frame_samples)
        else {
            return EDGE_RING_CAPACITY_FRAMES;
        };
        if sample_rate_hz == 0 || frame_samples == 0 {
            return EDGE_RING_CAPACITY_FRAMES;
        }
        let budget_samples = u64::from(jitter_budget_ms).saturating_mul(u64::from(sample_rate_hz));
        let frame_samples = u64::try_from(frame_samples).unwrap_or(u64::MAX);
        let frames = budget_samples
            .div_ceil(frame_samples.saturating_mul(1_000))
            .max(1);
        usize::try_from(frames)
            .unwrap_or(MAX_EDGE_RING_CAPACITY_FRAMES)
            .min(MAX_EDGE_RING_CAPACITY_FRAMES)
    }

    fn validate_fan_out_ownership(
        &self,
        fan_out: &[FanOutGroup],
        memory_plan: &MemoryPlan,
    ) -> Result<(), PlanError> {
        for group in fan_out {
            let has_exclusive_target = group.targets.iter().any(|edge_id| {
                memory_plan
                    .edge_buffer(*edge_id)
                    .is_some_and(|buffer| buffer.copy_policy == CopyPolicy::MoveExclusive)
            });
            if has_exclusive_target {
                return Err(PlanError::MoveExclusiveFanOut {
                    node: group.from.node.index(),
                    port: group.from.port.clone(),
                });
            }
        }
        Ok(())
    }

    fn instrument_edges(&self, ir: &GraphIr) -> Vec<(EdgeId, EdgeMetricId)> {
        ir.edges
            .iter()
            .map(|edge| (edge.spec.id, EdgeMetricId(edge.spec.id.index())))
            .collect()
    }

    fn bytes_per_frame(media: &MediaCaps) -> usize {
        match media {
            MediaCaps::Audio(caps) => match caps.channel_layout.channel_count() {
                Some(channels) => channels as usize * FRAME_BYTES_MONO_48K,
                None => FRAME_BYTES_MONO_48K,
            },
            _ => FRAME_BYTES_MONO_48K,
        }
    }
}

impl Default for RuntimePlanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use pks_caps::{AudioCaps, ChannelLayout, EdgeContract, PortDirection, PortSpec};
    use pks_frame::SampleFormat;
    use proptest::prelude::*;

    use crate::builtins::PassthroughNode;
    use crate::compiler::Compiler;
    use crate::dsl::Pipeline;
    use crate::node::{ConfigError, NodeConfig, NodeDescriptor, NodeTypeId, PrepareContext};
    use crate::partition::ExecutionPartition;
    use crate::registry::{NodeFactory, NodeRegistry};
    use crate::runtime_node::RuntimeNode;

    fn audio_media() -> MediaCaps {
        MediaCaps::Audio(AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: ChannelLayout::Any,
            format: SampleFormat::F32Interleaved,
        })
    }

    fn port(name: &str, direction: PortDirection, multiplicity: Multiplicity) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
            media: audio_media(),
            multiplicity,
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
            realtime_safe: execution.requires_realtime_safety(),
            execution,
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
                Vec::new(),
                vec![port("audio", PortDirection::Output, Multiplicity::One)],
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
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    struct TransformFactory;
    impl NodeFactory for TransformFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "transform",
                vec![port("audio", PortDirection::Input, Multiplicity::One)],
                vec![port("audio", PortDirection::Output, Multiplicity::One)],
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
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    struct SinkFactory;
    impl NodeFactory for SinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "sink",
                vec![port("audio", PortDirection::Input, Multiplicity::One)],
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
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    struct MixerFactory;
    impl NodeFactory for MixerFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "mixer",
                vec![port("audio", PortDirection::Input, Multiplicity::Many)],
                vec![port("audio", PortDirection::Output, Multiplicity::One)],
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
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    struct AsyncModelFactory;
    impl NodeFactory for AsyncModelFactory {
        fn descriptor(&self) -> NodeDescriptor {
            test_descriptor(
                "model.async",
                vec![port("audio", PortDirection::Input, Multiplicity::One)],
                vec![port("audio", PortDirection::Output, Multiplicity::One)],
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
        ) -> Result<Box<dyn RuntimeNode>, crate::node::NodeError> {
            unused_node()
        }
    }

    fn test_registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        registry.register(Arc::new(SourceFactory));
        registry.register(Arc::new(TransformFactory));
        registry.register(Arc::new(SinkFactory));
        registry.register(Arc::new(MixerFactory));
        registry.register(Arc::new(AsyncModelFactory));
        registry
    }

    fn compile(graph: Pipeline, registry: &NodeRegistry) -> GraphIr {
        Compiler::new()
            .compile(graph.into_spec(), registry)
            .unwrap()
    }

    #[test]
    fn given_linear_realtime_graph_when_planned_then_single_partition_in_topo_order() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let transform = graph.add_node("transform", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        graph.connect(source.out("audio"), transform.in_("audio"));
        graph.connect(transform.out("audio"), sink.in_("audio"));
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        assert_eq!(plan.partitions.len(), 1);
        assert_eq!(
            plan.partitions[0].execution,
            ExecutionPartition::RealtimeCpu
        );
        assert_eq!(plan.node_order, ir.topo_order().to_vec());
    }

    #[test]
    fn given_realtime_and_model_remote_nodes_when_planned_then_two_partitions_ordered_by_rank() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let model = graph.add_node("model.async", NodeConfig::new());
        graph.connect(source.out("audio"), model.in_("audio"));
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        assert_eq!(plan.partitions.len(), 2);
        assert_eq!(
            plan.partitions[0].execution,
            ExecutionPartition::RealtimeCpu
        );
        assert_eq!(plan.partitions[1].execution, ExecutionPartition::External);
        assert!(plan.partitions[0].execution.rank() < plan.partitions[1].execution.rank());
        assert_eq!(
            plan.partition(ExecutionPartition::RealtimeCpu)
                .unwrap()
                .nodes,
            vec![source.id()]
        );
        assert_eq!(
            plan.partition(ExecutionPartition::External).unwrap().nodes,
            vec![model.id()]
        );
    }

    #[test]
    fn given_realtime_to_external_edge_when_planned_then_branch_pool_isolated_from_capture_pool() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let model = graph.add_node("model.async", NodeConfig::new());
        let edge_id = graph.connect(source.out("audio"), model.in_("audio"));
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        let edge = plan.memory_plan.edge_buffer(edge_id).unwrap();
        assert_eq!(edge.copy_policy, CopyPolicy::CopyToBranchPool);
        assert_eq!(plan.memory_plan.branch_copy_pool_bytes, edge.total_bytes());
    }

    #[test]
    fn given_output_feeding_two_edges_when_planned_then_one_fan_out_group_with_two_targets() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let left = graph.add_node("transform", NodeConfig::new());
        let right = graph.add_node("transform", NodeConfig::new());
        graph.connect(source.out("audio"), left.in_("audio"));
        graph.connect(source.out("audio"), right.in_("audio"));
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        assert_eq!(plan.fan_out.len(), 1);
        assert_eq!(plan.fan_out[0].from.node, source.id());
        assert_eq!(plan.fan_out[0].from.port, "audio");
        assert_eq!(plan.fan_out[0].targets.len(), 2);
        for edge_id in &plan.fan_out[0].targets {
            assert_eq!(
                plan.memory_plan.edge_buffer(*edge_id).unwrap().copy_policy,
                CopyPolicy::CopyToBranchPool
            );
        }
        assert_eq!(
            plan.memory_plan.branch_copy_pool_bytes,
            2 * EDGE_RING_CAPACITY_FRAMES * FRAME_BYTES_MONO_48K
        );
    }

    #[test]
    fn given_move_exclusive_edge_in_fan_out_when_planned_then_ownership_is_rejected() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let left = graph.add_node("transform", NodeConfig::new());
        let right = graph.add_node("transform", NodeConfig::new());
        let mut exclusive = EdgeContract::voice_default();
        exclusive.copy_policy = CopyPolicy::MoveExclusive;
        graph.connect_with(source.out("audio"), left.in_("audio"), exclusive);
        graph.connect(source.out("audio"), right.in_("audio"));
        let ir = compile(graph, &registry);

        let error = RuntimePlanner::new().plan(&ir).unwrap_err();

        assert_eq!(
            error,
            PlanError::MoveExclusiveFanOut {
                node: source.id().index(),
                port: "audio".to_owned(),
            }
        );
    }

    #[test]
    fn given_many_input_port_with_multiple_sources_when_planned_then_one_fan_in_group() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let first = graph.add_node("source", NodeConfig::new());
        let second = graph.add_node("source", NodeConfig::new());
        let mixer = graph.add_node("mixer", NodeConfig::new());
        graph.connect(first.out("audio"), mixer.in_("audio"));
        graph.connect(second.out("audio"), mixer.in_("audio"));
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        assert_eq!(plan.fan_in.len(), 1);
        assert_eq!(plan.fan_in[0].into.node, mixer.id());
        assert_eq!(plan.fan_in[0].into.port, "audio");
        assert_eq!(plan.fan_in[0].sources.len(), 2);
    }

    #[test]
    fn given_single_input_port_with_multiple_sources_when_planned_then_fan_in_on_single_port_error()
    {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let first = graph.add_node("source", NodeConfig::new());
        let second = graph.add_node("source", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        graph.connect(first.out("audio"), sink.in_("audio"));
        graph.connect(second.out("audio"), sink.in_("audio"));
        let ir = compile(graph, &registry);

        let error = RuntimePlanner::new().plan(&ir).unwrap_err();

        assert_eq!(
            error,
            PlanError::FanInOnSinglePort {
                node: sink.id().index(),
                port: "audio".to_owned(),
            }
        );
    }

    #[test]
    fn given_realtime_consumers_when_planned_then_every_edge_buffered_and_pool_positive() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let transform = graph.add_node("transform", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        let first = graph.connect(source.out("audio"), transform.in_("audio"));
        let second = graph.connect(transform.out("audio"), sink.in_("audio"));
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        let expected_total_bytes = EDGE_RING_CAPACITY_FRAMES * FRAME_BYTES_MONO_48K;
        for edge in [first, second] {
            let buffer = plan.memory_plan.edge_buffer(edge).unwrap();
            assert_eq!(buffer.total_bytes(), expected_total_bytes);
            assert_eq!(buffer.copy_policy, CopyPolicy::MoveExclusive);
        }
        assert_eq!(plan.memory_plan.edge_buffers.len(), 2);
        assert_eq!(
            plan.memory_plan.realtime_pool_bytes,
            2 * expected_total_bytes
        );
        assert!(plan.memory_plan.realtime_pool_bytes > 0);
        assert_eq!(plan.memory_plan.branch_copy_pool_bytes, 0);
    }

    #[test]
    fn given_copy_to_branch_pool_edge_when_planned_then_copy_pool_memory_is_reserved() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        let mut copy_contract = EdgeContract::voice_default();
        copy_contract.copy_policy = CopyPolicy::CopyToBranchPool;
        let edge_id = graph.connect_with(source.out("audio"), sink.in_("audio"), copy_contract);
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        let edge = plan.memory_plan.edge_buffer(edge_id).unwrap();
        assert_eq!(edge.copy_policy, CopyPolicy::CopyToBranchPool);
        assert_eq!(plan.memory_plan.branch_copy_pool_bytes, edge.total_bytes());
    }

    #[test]
    fn given_explicit_jitter_budget_when_planned_then_bounded_capacity_is_derived_from_frame_time()
    {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        let mut contract = EdgeContract::voice_default();
        contract.jitter_budget_ms = Some(1_000);
        contract.copy_policy = CopyPolicy::CopyToBranchPool;
        let edge_id = graph.connect_with(source.out("audio"), sink.in_("audio"), contract);
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        assert_eq!(
            plan.memory_plan
                .edge_buffer(edge_id)
                .unwrap()
                .capacity_frames,
            50
        );
    }

    #[test]
    fn given_compiled_graph_when_instrumented_then_metric_ids_are_stable_and_distinct() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let transform = graph.add_node("transform", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        let first = graph.connect(source.out("audio"), transform.in_("audio"));
        let second = graph.connect(transform.out("audio"), sink.in_("audio"));
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        assert_eq!(plan.metric_id(first), Some(EdgeMetricId(first.index())));
        assert_eq!(plan.metric_id(second), Some(EdgeMetricId(second.index())));
        assert_ne!(plan.metric_id(first), plan.metric_id(second));
    }

    #[test]
    fn given_fixed_graph_when_planned_then_runtime_plan_matches_golden_snapshot() {
        let registry = test_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("source", NodeConfig::new());
        let transform = graph.add_node("transform", NodeConfig::new());
        let sink = graph.add_node("sink", NodeConfig::new());
        graph.connect(source.out("audio"), transform.in_("audio"));
        graph.connect(transform.out("audio"), sink.in_("audio"));
        let ir = compile(graph, &registry);

        let plan = RuntimePlanner::new().plan(&ir).unwrap();

        let node_order: Vec<u32> = plan.node_order.iter().map(|id| id.index()).collect();
        assert_eq!(node_order, vec![0, 1, 2]);
        assert_eq!(plan.partitions.len(), 1);
        assert_eq!(
            plan.partitions[0].execution,
            ExecutionPartition::RealtimeCpu
        );
        let partition_nodes: Vec<u32> = plan.partitions[0]
            .nodes
            .iter()
            .map(|id| id.index())
            .collect();
        assert_eq!(partition_nodes, vec![0, 1, 2]);
        assert_eq!(plan.edge_count, 2);
    }

    proptest! {
        #[test]
        fn given_linear_realtime_chain_when_planned_then_single_partition_and_topo_order(
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

            let plan = RuntimePlanner::new().plan(&ir).unwrap();

            let order: Vec<u32> = plan.node_order.iter().map(|id| id.index()).collect();
            let expected: Vec<u32> = (0..chain_len as u32).collect();
            prop_assert_eq!(plan.partitions.len(), 1);
            prop_assert_eq!(order, expected);
            prop_assert_eq!(plan.edge_count, chain_len - 1);
        }
    }
}

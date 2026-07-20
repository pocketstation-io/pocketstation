//! Builds executable realtime nodes and bounded partition crossings from a
//! compiled `RuntimePlan`, its `GraphIr`, and a `NodeRegistry`.

use pks_graph::ir::GraphIr;
use pks_graph::node::PrepareContext;
use pks_graph::partition::ExecutionPartition;
use pks_graph::plan::RuntimePlan;
use pks_graph::registry::NodeRegistry;
use pks_graph::runtime_node::RuntimeNode;
use pks_graph::spec::{EdgeId, NodeId};

use crate::executor::{ExecError, RealtimeExecutor};
use crate::plan_executor::RealtimePlanExecutor;
use crate::plan_router::PlanEdgeReceiver;

pub struct PlanScheduler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartitionBridgeSpec {
    pub edge: EdgeId,
    pub from: ExecutionPartition,
    pub to: ExecutionPartition,
    pub capacity_frames: usize,
}

impl PlanScheduler {
    pub fn build_plan_executor(
        plan: &RuntimePlan,
        ir: &GraphIr,
        registry: &NodeRegistry,
        cx: &PrepareContext,
    ) -> Result<(RealtimePlanExecutor, Vec<PlanEdgeReceiver>), ExecError> {
        RealtimePlanExecutor::new(plan, ir, registry, cx)
    }

    pub fn build_realtime_executor(
        plan: &RuntimePlan,
        ir: &GraphIr,
        registry: &NodeRegistry,
        cx: &PrepareContext,
    ) -> Result<RealtimeExecutor, ExecError> {
        let realtime: Vec<NodeId> = plan
            .partitions
            .iter()
            .filter(|partition| partition.execution.requires_realtime_safety())
            .flat_map(|partition| partition.nodes.iter().copied())
            .collect();
        let ordered: Vec<NodeId> = plan
            .node_order
            .iter()
            .copied()
            .filter(|id| realtime.contains(id))
            .collect();

        let mut nodes: Vec<Box<dyn RuntimeNode>> = Vec::with_capacity(ordered.len());
        for id in &ordered {
            let resolved = ir
                .node(*id)
                .ok_or_else(|| ExecError::Node(format!("node {} absent from IR", id.index())))?;
            let factory = registry.get(&resolved.spec.type_id).ok_or_else(|| {
                ExecError::Node(format!(
                    "no factory registered for type '{}'",
                    resolved.spec.type_id.as_str()
                ))
            })?;
            let node = factory
                .instantiate(cx, &resolved.spec.config)
                .map_err(ExecError::from_node)?;
            nodes.push(node);
        }

        let mut executor = RealtimeExecutor::new(ordered, nodes)?;
        executor.prepare(cx)?;
        Ok(executor)
    }

    pub fn build_partition_bridges(
        plan: &RuntimePlan,
        ir: &GraphIr,
    ) -> Result<Vec<PartitionBridgeSpec>, ExecError> {
        let mut bridges = Vec::new();
        for edge in &ir.edges {
            let from = ir
                .node(edge.spec.from.node)
                .ok_or_else(|| {
                    ExecError::Node(format!(
                        "edge {} source node {} absent from IR",
                        edge.spec.id.index(),
                        edge.spec.from.node.index()
                    ))
                })?
                .descriptor
                .execution;
            let to = ir
                .node(edge.spec.to.node)
                .ok_or_else(|| {
                    ExecError::Node(format!(
                        "edge {} target node {} absent from IR",
                        edge.spec.id.index(),
                        edge.spec.to.node.index()
                    ))
                })?
                .descriptor
                .execution;
            if from.needs_bridge_to(to) {
                let capacity_frames = plan
                    .memory_plan
                    .edge_buffer(edge.spec.id)
                    .map_or(pks_graph::plan::EDGE_RING_CAPACITY_FRAMES, |buffer| {
                        buffer.capacity_frames
                    });
                bridges.push(PartitionBridgeSpec {
                    edge: edge.spec.id,
                    from,
                    to,
                    capacity_frames,
                });
            }
        }
        Ok(bridges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
    use pks_frame::AudioFrame;
    use pks_frame::{AudioBufferPool, SampleFormat, SampleSpec, SourceId, StreamId};
    use pks_graph::compiler::Compiler;
    use pks_graph::dsl::Pipeline;
    use pks_graph::node::{ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId};
    use pks_graph::planner::RuntimePlanner;
    use pks_graph::register_builtins;
    use pks_graph::registry::{NodeFactory, NodeRegistry};

    const GAIN_DB_KEY: &str = "gain_db";

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn frame_with_samples(samples: &[f32]) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples.len());
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(samples);
        AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle)
    }

    fn built_registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry);
        registry
    }

    fn audio_port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
            media: MediaCaps::Audio(AudioCaps {
                sample_rate_hz: Some(48_000),
                frame_samples: Some(960),
                channel_layout: ChannelLayout::Mono,
                format: SampleFormat::F32Interleaved,
            }),
            multiplicity: Multiplicity::One,
            required: true,
        }
    }

    struct ExternalFactory;

    impl NodeFactory for ExternalFactory {
        fn descriptor(&self) -> NodeDescriptor {
            NodeDescriptor {
                type_id: NodeTypeId::from("model.external"),
                display_name: "External Model",
                inputs: vec![audio_port("in", PortDirection::Input)],
                outputs: vec![audio_port("out", PortDirection::Output)],
                execution: ExecutionPartition::External,
                realtime_safe: false,
                stateful: true,
            }
        }

        fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
            Ok(())
        }

        fn instantiate(
            &self,
            _cx: &PrepareContext,
            _config: &NodeConfig,
        ) -> Result<Box<dyn RuntimeNode>, NodeError> {
            Err(NodeError::Prepare(
                "external test factory is not instantiated by realtime scheduler".to_owned(),
            ))
        }
    }

    fn passthrough_gain_passthrough(gain_db: &str) -> (RuntimePlan, GraphIr) {
        let registry = built_registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let gain = graph.add_node("gain", NodeConfig::new().with(GAIN_DB_KEY, gain_db));
        let sink = graph.add_node("passthrough", NodeConfig::new());
        graph.connect(source.out("out"), gain.in_("in"));
        graph.connect(gain.out("out"), sink.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        (plan, ir)
    }

    fn passthrough_external() -> (RuntimePlan, GraphIr) {
        let mut registry = built_registry();
        registry.register(std::sync::Arc::new(ExternalFactory));
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let external = graph.add_node("model.external", NodeConfig::new());
        graph.connect(source.out("out"), external.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        (plan, ir)
    }

    #[test]
    fn given_realtime_plan_when_built_then_executor_holds_every_realtime_node() {
        let (plan, ir) = passthrough_gain_passthrough("0.0");
        let registry = built_registry();
        let executor =
            PlanScheduler::build_realtime_executor(&plan, &ir, &registry, &prepare_cx()).unwrap();
        assert_eq!(executor.node_count(), 3);
        assert_eq!(executor.node_order(), ir.topo_order());
    }

    #[test]
    fn given_gain_in_chain_when_run_frame_then_gain_doubles_amplitude() {
        let doubling_gain_db = "6.020599913"; // 20·log10(2) → linear gain ≈ 2.0
        let (plan, ir) = passthrough_gain_passthrough(doubling_gain_db);
        let registry = built_registry();
        let mut executor =
            PlanScheduler::build_realtime_executor(&plan, &ir, &registry, &prepare_cx()).unwrap();

        let input = [0.25, -0.1, 0.4];
        let output = executor
            .run_frame(frame_with_samples(&input))
            .unwrap()
            .unwrap();
        for (got, raw) in output.buffer.as_slice().iter().zip(input.iter()) {
            assert!(
                (got - raw * 2.0).abs() < 1e-4,
                "got {got}, want {}",
                raw * 2.0
            );
        }
    }

    #[test]
    fn given_built_executor_when_run_frames_then_frames_in_equals_frames_out() {
        let (plan, ir) = passthrough_gain_passthrough("3.0");
        let registry = built_registry();
        let mut executor =
            PlanScheduler::build_realtime_executor(&plan, &ir, &registry, &prepare_cx()).unwrap();

        let frames = vec![
            frame_with_samples(&[0.1, 0.2]),
            frame_with_samples(&[0.3, 0.4]),
            frame_with_samples(&[0.5, 0.6]),
            frame_with_samples(&[0.7, 0.8]),
        ];
        let metrics = executor.run_frames(frames).unwrap();
        assert_eq!(metrics.frames_in(), 4);
        assert_eq!(metrics.frames_out(), 4);
        assert_eq!(metrics.frames_dropped(), 0);
    }

    #[test]
    fn given_realtime_only_plan_when_build_partition_bridges_then_no_bridges() {
        let (plan, ir) = passthrough_gain_passthrough("0.0");
        let bridges = PlanScheduler::build_partition_bridges(&plan, &ir).unwrap();
        assert!(bridges.is_empty());
    }

    #[test]
    fn given_realtime_to_external_edge_when_build_partition_bridges_then_bridge_is_planned() {
        let (plan, ir) = passthrough_external();
        let bridges = PlanScheduler::build_partition_bridges(&plan, &ir).unwrap();
        assert_eq!(bridges.len(), 1);
        assert_eq!(bridges[0].edge.index(), 0);
        assert_eq!(bridges[0].from, ExecutionPartition::RealtimeCpu);
        assert_eq!(bridges[0].to, ExecutionPartition::External);
        assert_eq!(
            bridges[0].capacity_frames,
            pks_graph::plan::EDGE_RING_CAPACITY_FRAMES
        );
    }
}

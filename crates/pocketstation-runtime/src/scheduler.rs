//! Builds a `RealtimeExecutor` from a compiled `RuntimePlan` plus the `GraphIr`
//! it was lowered from and a `NodeRegistry` to instantiate nodes.
//!
//! Only the realtime partitions (`ExecutionClass::is_realtime()`) are wired into
//! the executor. Async/model/network/recorder partitions are left for a later
//! wave; they will be driven over bounded `EdgeChannel`s, not here.

use pocketstation_graph::ir::GraphIr;
use pocketstation_graph::node::PrepareContext;
use pocketstation_graph::plan::RuntimePlan;
use pocketstation_graph::registry::NodeRegistry;
use pocketstation_graph::runtime_node::RuntimeNode;
use pocketstation_graph::spec::NodeId;

use crate::executor::{ExecError, RealtimeExecutor};

pub struct PlanScheduler;

impl PlanScheduler {
    pub fn build_realtime_executor(
        plan: &RuntimePlan,
        ir: &GraphIr,
        registry: &NodeRegistry,
        cx: &PrepareContext,
    ) -> Result<RealtimeExecutor, ExecError> {
        let realtime: Vec<NodeId> = plan
            .partitions
            .iter()
            .filter(|partition| partition.class.is_realtime())
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::AudioFrame;
    use pocketstation_frame::{AudioBufferPool, SampleFormat, SampleSpec, SourceId, StreamId};
    use pocketstation_graph::compiler::Compiler;
    use pocketstation_graph::dsl::AudioGraph;
    use pocketstation_graph::node::NodeConfig;
    use pocketstation_graph::planner::RuntimePlanner;
    use pocketstation_graph::register_builtins;
    use pocketstation_graph::registry::NodeRegistry;

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

    fn passthrough_gain_passthrough(gain_db: &str) -> (RuntimePlan, GraphIr) {
        let registry = built_registry();
        let mut graph = AudioGraph::new();
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
}

//! Realtime node execution connected by the bounded edges of a `RuntimePlan`.

use pks_frame::AudioFrame;
use pks_graph::ir::GraphIr;
use pks_graph::node::PrepareContext;
use pks_graph::plan::RuntimePlan;
use pks_graph::registry::NodeRegistry;
use pks_graph::runtime_node::RuntimeNode;
use pks_graph::spec::{EdgeId, NodeId};

use crate::executor::ExecError;
use crate::plan_router::{
    DispatchSummary, EdgeObservations, PlanEdgeFrame, PlanEdgeReceiver, PlanEdgeRouter,
};

struct RealtimeNodeSlot {
    output_port: Option<String>,
    node: Box<dyn RuntimeNode>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlanExecutionSummary {
    pub nodes_executed: u64,
    pub edges_attempted: u64,
    pub edges_enqueued: u64,
    pub edges_dropped: u64,
}

impl PlanExecutionSummary {
    fn observe_dispatch(&mut self, dispatch: DispatchSummary) {
        self.edges_attempted = self
            .edges_attempted
            .saturating_add(dispatch.attempted_edges);
        self.edges_enqueued = self.edges_enqueued.saturating_add(dispatch.enqueued_edges);
        self.edges_dropped = self.edges_dropped.saturating_add(dispatch.dropped_edges);
    }
}

pub struct RealtimePlanExecutor {
    node_order: Vec<NodeId>,
    nodes: Vec<Option<RealtimeNodeSlot>>,
    router: PlanEdgeRouter,
    realtime_receivers: Vec<PlanEdgeReceiver>,
}

impl RealtimePlanExecutor {
    pub fn new(
        plan: &RuntimePlan,
        ir: &GraphIr,
        registry: &NodeRegistry,
        prepare_context: &PrepareContext,
    ) -> Result<(Self, Vec<PlanEdgeReceiver>), ExecError> {
        let max_node_index = plan
            .node_order
            .iter()
            .map(|node_id| node_id.index() as usize)
            .max()
            .unwrap_or(0);
        let mut nodes: Vec<Option<RealtimeNodeSlot>> = (0..=max_node_index).map(|_| None).collect();

        for node_id in &plan.node_order {
            let resolved = ir.node(*node_id).ok_or_else(|| {
                ExecError::Node(format!("node {} absent from IR", node_id.index()))
            })?;
            if !resolved.descriptor.execution.requires_realtime_safety() {
                continue;
            }
            if resolved.descriptor.outputs.len() > 1 {
                return Err(ExecError::Node(format!(
                    "realtime node {} exposes multiple outputs; port-aware node output is required",
                    node_id.index()
                )));
            }
            let factory = registry.get(&resolved.spec.type_id).ok_or_else(|| {
                ExecError::Node(format!(
                    "no factory registered for type '{}'",
                    resolved.spec.type_id.as_str()
                ))
            })?;
            let mut node = factory
                .instantiate(prepare_context, &resolved.spec.config)
                .map_err(ExecError::from_node)?;
            node.prepare(prepare_context)
                .map_err(ExecError::from_node)?;
            nodes[node_id.index() as usize] = Some(RealtimeNodeSlot {
                output_port: resolved
                    .descriptor
                    .outputs
                    .first()
                    .map(|port| port.name.clone()),
                node,
            });
        }

        let (router, receivers) =
            PlanEdgeRouter::new(plan, ir).map_err(|error| ExecError::Node(error.to_string()))?;
        let mut realtime_receivers = Vec::new();
        let mut worker_receivers = Vec::new();
        for receiver in receivers {
            let target = ir.node(receiver.to().node).ok_or_else(|| {
                ExecError::Node(format!(
                    "edge {} target node {} absent from IR",
                    receiver.edge_id().index(),
                    receiver.to().node.index()
                ))
            })?;
            if target.descriptor.execution.requires_realtime_safety() {
                if realtime_receivers
                    .iter()
                    .any(|existing: &PlanEdgeReceiver| existing.to().node == receiver.to().node)
                {
                    return Err(ExecError::Node(format!(
                        "realtime fan-in for node {} requires an explicit mixer",
                        receiver.to().node.index()
                    )));
                }
                realtime_receivers.push(receiver);
            } else {
                worker_receivers.push(receiver);
            }
        }

        Ok((
            Self {
                node_order: plan.node_order.clone(),
                nodes,
                router,
                realtime_receivers,
            },
            worker_receivers,
        ))
    }

    pub fn execute_from(
        &mut self,
        source_node_id: NodeId,
        frame: AudioFrame,
        now_ns: u64,
    ) -> Result<PlanExecutionSummary, ExecError> {
        let mut summary = PlanExecutionSummary::default();
        self.process_and_dispatch(source_node_id, frame, now_ns, &mut summary)?;

        for order_index in 0..self.node_order.len() {
            let node_id = self.node_order[order_index];
            if node_id == source_node_id {
                continue;
            }
            let mut incoming = None;
            for receiver in &mut self.realtime_receivers {
                if receiver.to().node == node_id {
                    incoming = receiver.recv_at(now_ns);
                    break;
                }
            }
            let Some(incoming) = incoming else {
                continue;
            };
            let frame = match incoming {
                PlanEdgeFrame::Exclusive(frame) => frame,
                PlanEdgeFrame::Shared(_frame) => {
                    return Err(ExecError::Node(format!(
                        "realtime node {} received an immutable frame without CopyToBranchPool",
                        node_id.index()
                    )))
                }
            };
            self.process_and_dispatch(node_id, frame, now_ns, &mut summary)?;
        }
        Ok(summary)
    }

    pub fn observations(&self, edge_id: EdgeId) -> Option<EdgeObservations> {
        self.router.observations(edge_id)
    }

    fn process_and_dispatch(
        &mut self,
        node_id: NodeId,
        frame: AudioFrame,
        now_ns: u64,
        summary: &mut PlanExecutionSummary,
    ) -> Result<(), ExecError> {
        let Some(slot) = self
            .nodes
            .get_mut(node_id.index() as usize)
            .and_then(Option::as_mut)
        else {
            return Err(ExecError::Node(format!(
                "node {} is not a realtime executable node",
                node_id.index()
            )));
        };
        let output = slot.node.process(frame).map_err(ExecError::from_node)?;
        summary.nodes_executed = summary.nodes_executed.saturating_add(1);
        if let (Some(output), Some(output_port)) = (output, slot.output_port.as_deref()) {
            let dispatch = self
                .router
                .dispatch_from(node_id, output_port, output, now_ns);
            summary.observe_dispatch(dispatch);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
    use pks_frame::{AudioBufferPool, SampleFormat, SampleSpec, SourceId, StreamId};
    use pks_graph::compiler::Compiler;
    use pks_graph::dsl::Pipeline;
    use pks_graph::node::{ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId};
    use pks_graph::partition::ExecutionPartition;
    use pks_graph::planner::RuntimePlanner;
    use pks_graph::register_builtins;
    use pks_graph::registry::{NodeFactory, NodeRegistry};

    use super::*;

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

    struct WorkerSinkFactory;

    impl NodeFactory for WorkerSinkFactory {
        fn descriptor(&self) -> NodeDescriptor {
            NodeDescriptor {
                type_id: NodeTypeId::from("test.worker_sink"),
                display_name: "Worker sink",
                inputs: vec![audio_port("in", PortDirection::Input)],
                outputs: Vec::new(),
                execution: ExecutionPartition::AsyncWorker,
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
                "worker sink must not be instantiated on realtime executor".to_owned(),
            ))
        }
    }

    fn registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry);
        registry.register(Arc::new(WorkerSinkFactory));
        registry
    }

    fn context() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn frame(samples: &[f32]) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples.len());
        let mut buffer = pool.acquire().unwrap();
        buffer.copy_from_slice(samples);
        AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer)
    }

    #[test]
    fn given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output(
    ) {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let gain = graph.add_node("gain", NodeConfig::new().with("gain_db", "6.020599913"));
        let unconnected = graph.add_node("gain", NodeConfig::new().with("gain_db", "20"));
        let worker = graph.add_node("test.worker_sink", NodeConfig::new());
        graph.connect(source.out("out"), gain.in_("in"));
        graph.connect(gain.out("out"), worker.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut executor, mut workers) =
            RealtimePlanExecutor::new(&plan, &ir, &registry, &context()).unwrap();

        // When
        let summary = executor
            .execute_from(source.id(), frame(&[0.25, -0.1]), 10)
            .unwrap();
        let delivered = workers[0].recv_at(20).unwrap();

        // Then
        assert_eq!(summary.nodes_executed, 2);
        assert_eq!(summary.edges_enqueued, 2);
        assert_ne!(unconnected.id(), source.id());
        match delivered {
            PlanEdgeFrame::Exclusive(frame) => {
                assert!((frame.buffer.as_slice()[0] - 0.5).abs() < 1e-4);
                assert!((frame.buffer.as_slice()[1] + 0.2).abs() < 1e-4);
            }
            PlanEdgeFrame::Shared(_) => panic!("worker edge should own an isolated branch copy"),
        }
    }

    #[test]
    fn given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy() {
        // Given
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let left_gain = graph.add_node("gain", NodeConfig::new().with("gain_db", "6.020599913"));
        let right_gain = graph.add_node("gain", NodeConfig::new().with("gain_db", "-6.020599913"));
        let left_worker = graph.add_node("test.worker_sink", NodeConfig::new());
        let right_worker = graph.add_node("test.worker_sink", NodeConfig::new());
        graph.connect(source.out("out"), left_gain.in_("in"));
        graph.connect(source.out("out"), right_gain.in_("in"));
        graph.connect(left_gain.out("out"), left_worker.in_("in"));
        graph.connect(right_gain.out("out"), right_worker.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut executor, mut workers) =
            RealtimePlanExecutor::new(&plan, &ir, &registry, &context()).unwrap();

        // When
        let summary = executor
            .execute_from(source.id(), frame(&[0.5]), 10)
            .unwrap();
        let mut outputs = workers
            .iter_mut()
            .filter_map(|worker| worker.recv_at(20))
            .map(|frame| match frame {
                PlanEdgeFrame::Shared(frame) => frame.buffer.as_slice()[0],
                PlanEdgeFrame::Exclusive(frame) => frame.buffer.as_slice()[0],
            })
            .collect::<Vec<_>>();
        outputs.sort_by(f32::total_cmp);

        // Then
        assert_eq!(summary.nodes_executed, 3);
        assert_eq!(outputs.len(), 2);
        assert!((outputs[0] - 0.25).abs() < 1e-4);
        assert!((outputs[1] - 1.0).abs() < 1e-4);
    }
}

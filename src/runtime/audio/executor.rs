//! Realtime node execution connected by the bounded edges of a `RuntimePlan`.

#[cfg(test)]
use crate::frame::AudioFrame;
use crate::frame::LineagedAudioFrame;
use crate::graph::ir::GraphIr;
use crate::graph::node::PrepareContext;
use crate::graph::plan::RuntimePlan;
use crate::graph::registry::NodeRegistry;
use crate::graph::runtime_node::RuntimeNode;
#[cfg(any(test, feature = "internal-testing"))]
use crate::graph::spec::EdgeId;
use crate::graph::spec::NodeId;

#[cfg(any(test, feature = "internal-testing"))]
use crate::runtime::audio::EdgeObservations;
use crate::runtime::audio::{DispatchSummary, PlanEdgeFrame, PlanEdgeReceiver, PlanEdgeRouter};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[doc = "Classifies failures surfaced by exec operations."]
pub enum ExecError {
    #[error("realtime plan execution failed: {0}")]
    #[doc = "Reports that no de is available."]
    Node(String),
}

impl ExecError {
    #[doc = "Creates `ExecError` from node."]
    pub fn from_node(error: crate::graph::node::NodeError) -> Self {
        Self::Node(error.to_string())
    }
}

struct RealtimeNodeSlot {
    output_port: Option<String>,
    node: Box<dyn RuntimeNode>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[doc = "Reports the counters and terminal facts collected for plan execution."]
pub struct PlanExecutionSummary {
    #[doc = "References the nodes executed participating in `PlanExecutionSummary`."]
    pub nodes_executed: u64,
    #[doc = "References the edges attempted participating in `PlanExecutionSummary`."]
    pub edges_attempted: u64,
    #[doc = "References the edges enqueued participating in `PlanExecutionSummary`."]
    pub edges_enqueued: u64,
    #[doc = "References the edges dropped participating in `PlanExecutionSummary`."]
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

#[doc = "Executes realtime plan according to its compiled plan and cancellation contract."]
pub struct RealtimePlanExecutor {
    node_order: Vec<NodeId>,
    nodes: Vec<Option<RealtimeNodeSlot>>,
    router: PlanEdgeRouter,
    realtime_receivers: Vec<PlanEdgeReceiver>,
}

impl RealtimePlanExecutor {
    #[doc = "Creates a new `RealtimePlanExecutor`."]
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

    #[doc = "Executes one lineaged frame from the named source node through `RealtimePlanExecutor`."]
    pub fn execute_from(
        &mut self,
        source_node_id: NodeId,
        frame: LineagedAudioFrame,
        now_ns: u64,
    ) -> Result<PlanExecutionSummary, ExecError> {
        let mut summary = PlanExecutionSummary::default();
        self.process_and_dispatch_lineaged(source_node_id, frame, now_ns, &mut summary)?;

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
                        "realtime node {} received an immutable lineaged frame without CopyToBranchPool",
                        node_id.index()
                    )))
                }
            };
            self.process_and_dispatch_lineaged(node_id, frame, now_ns, &mut summary)?;
        }
        Ok(summary)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the observations exposed by `RealtimePlanExecutor`."]
    pub fn observations(&self, edge_id: EdgeId) -> Option<EdgeObservations> {
        self.router.observations(edge_id)
    }

    fn process_and_dispatch_lineaged(
        &mut self,
        node_id: NodeId,
        frame: LineagedAudioFrame,
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
        let (frame, lineage) = frame.into_parts();
        let output = slot.node.process(frame).map_err(ExecError::from_node)?;
        summary.nodes_executed = summary.nodes_executed.saturating_add(1);
        if let (Some(output), Some(output_port)) = (output, slot.output_port.as_deref()) {
            let output = LineagedAudioFrame::new(output, lineage).map_err(|error| {
                ExecError::Node(format!(
                    "realtime node {} changed lineage-authoritative frame identity: {error}",
                    node_id.index()
                ))
            })?;
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

    use crate::frame::{
        AudioBufferPool, ClockDomainId, FrameLineage, SampleFormat, SampleSpec, SessionId,
        SourceId, StemId, StreamId,
    };
    use crate::graph::compile::Compiler;
    use crate::graph::compile::RuntimePlanner;
    use crate::graph::dsl::Pipeline;
    use crate::graph::node::{ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId};
    use crate::graph::partition::ExecutionPartition;
    use crate::graph::register_builtins;
    use crate::graph::registry::{NodeFactory, NodeRegistry};
    use crate::graph::{
        AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec, SafetyContract,
        SignalSpec,
    };

    use super::*;

    fn audio_port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
            signal: SignalSpec::audio(),
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
                safety: SafetyContract::AllocationAllowed,
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
        register_builtins(&mut registry).unwrap();
        registry.register(Arc::new(WorkerSinkFactory)).unwrap();
        registry
    }

    fn context() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn audio_frame(samples: &[f32]) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples.len());
        let mut buffer = pool.acquire().unwrap();
        buffer.try_copy_from_slice(samples).unwrap();
        AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer)
    }

    fn lineaged_frame(samples: &[f32]) -> LineagedAudioFrame {
        LineagedAudioFrame::new(
            audio_frame(samples),
            FrameLineage {
                session_id: SessionId(5),
                source_id: SourceId(2),
                stem_id: StemId(6),
                clock_id: ClockDomainId(7),
                sequence_num: 3,
                timestamp_start_ns: 4,
                duration_ns: 20_000_000,
                source_generation: 8,
                discontinuity_epoch: 9,
                permission_epoch: 10,
            },
        )
        .unwrap()
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
            .execute_from(source.id(), lineaged_frame(&[0.25, -0.1]), 10)
            .unwrap();
        let delivered = workers[0].recv_at(20).unwrap();

        // Then
        assert_eq!(summary.nodes_executed, 2);
        assert_eq!(summary.edges_enqueued, 2);
        assert_ne!(unconnected.id(), source.id());
        match delivered {
            PlanEdgeFrame::Exclusive(frame) => {
                assert!((frame.frame().buffer.as_slice()[0] - 0.5).abs() < 1e-4);
                assert!((frame.frame().buffer.as_slice()[1] + 0.2).abs() < 1e-4);
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
            .execute_from(source.id(), lineaged_frame(&[0.5]), 10)
            .unwrap();
        let mut outputs = workers
            .iter_mut()
            .filter_map(|worker| worker.recv_at(20))
            .map(|frame| match frame {
                PlanEdgeFrame::Shared(frame) => frame.frame().buffer.as_slice()[0],
                PlanEdgeFrame::Exclusive(frame) => frame.frame().buffer.as_slice()[0],
            })
            .collect::<Vec<_>>();
        outputs.sort_by(f32::total_cmp);

        // Then
        assert_eq!(summary.nodes_executed, 3);
        assert_eq!(outputs.len(), 2);
        assert!((outputs[0] - 0.25).abs() < 1e-4);
        assert!((outputs[1] - 1.0).abs() < 1e-4);
    }

    #[test]
    fn given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs() {
        let registry = registry();
        let mut graph = Pipeline::new();
        let source = graph.add_node("passthrough", NodeConfig::new());
        let gain = graph.add_node("gain", NodeConfig::new().with("gain_db", "6.020599913"));
        let worker = graph.add_node("test.worker_sink", NodeConfig::new());
        graph.connect(source.out("out"), gain.in_("in"));
        graph.connect(gain.out("out"), worker.in_("in"));
        let ir = Compiler::new()
            .compile(graph.into_spec(), &registry)
            .unwrap();
        let plan = RuntimePlanner::new().plan(&ir).unwrap();
        let (mut executor, mut workers) =
            RealtimePlanExecutor::new(&plan, &ir, &registry, &context()).unwrap();
        let input = lineaged_frame(&[0.25]);
        let expected_lineage = input.lineage();

        let summary = executor.execute_from(source.id(), input, 10).unwrap();
        let delivered = workers[0].recv_at(20).unwrap();

        assert_eq!(summary.nodes_executed, 2);
        assert_eq!(delivered.lineage(), expected_lineage);
        match delivered {
            PlanEdgeFrame::Exclusive(frame) => {
                assert!((frame.frame().buffer.as_slice()[0] - 0.5).abs() < 1e-4);
            }
            PlanEdgeFrame::Shared(_) => {
                panic!("worker edge should own an isolated lineaged branch copy")
            }
        }
    }
}

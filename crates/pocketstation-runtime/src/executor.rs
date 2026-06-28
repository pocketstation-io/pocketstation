//! Realtime executor for a `RuntimePlan`. It owns the instantiated `RuntimeNode`s
//! of the plan's realtime partitions and drives a frame through them in topo order.
//!
//! Scope: only nodes whose `ExecutionClass::is_realtime()` is true run here
//! (`AudioCallback`, `RealtimeCpu`). The async, model, network, recorder, and
//! control-plane partitions are NOT executed on this path; they communicate
//! across bounded `EdgeChannel`s and run on their own threads. Wiring those
//! partitions is a later wave and is intentionally not stubbed here.
//!
//! `run_frame` is the steady-state hot path: alloc-free, lock-free, log-free
//! (LAW 15). Allocation appears only on the error path, which is not steady state.

use pocketstation_frame::AudioFrame;
use pocketstation_graph::node::{NodeError, PrepareContext};
use pocketstation_graph::runtime_node::RuntimeNode;
use pocketstation_graph::spec::NodeId;

#[derive(Debug, PartialEq, Eq)]
pub enum ExecError {
    NodeCountMismatch { order: usize, nodes: usize },
    Node(String),
    NotPrepared,
}

impl ExecError {
    pub fn from_node(error: NodeError) -> Self {
        Self::Node(error.to_string())
    }
}

impl std::fmt::Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeCountMismatch { order, nodes } => write!(
                f,
                "realtime node count {nodes} does not match node order length {order}"
            ),
            Self::Node(reason) => write!(f, "realtime node failed: {reason}"),
            Self::NotPrepared => write!(f, "executor run before prepare() was called"),
        }
    }
}

impl std::error::Error for ExecError {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RunMetrics {
    frames_in: u64,
    frames_out: u64,
    frames_dropped: u64, // frames a node gated to None (e.g. VAD silence)
}

impl RunMetrics {
    pub fn frames_in(&self) -> u64 {
        self.frames_in
    }
    pub fn frames_out(&self) -> u64 {
        self.frames_out
    }
    pub fn frames_dropped(&self) -> u64 {
        self.frames_dropped
    }
}

pub struct RealtimeExecutor {
    node_order: Vec<NodeId>,
    nodes: Vec<Box<dyn RuntimeNode>>,
    metrics: RunMetrics,
    prepared: bool,
}

impl RealtimeExecutor {
    pub fn new(
        node_order: Vec<NodeId>,
        nodes: Vec<Box<dyn RuntimeNode>>,
    ) -> Result<Self, ExecError> {
        if node_order.len() != nodes.len() {
            return Err(ExecError::NodeCountMismatch {
                order: node_order.len(),
                nodes: nodes.len(),
            });
        }
        Ok(Self {
            node_order,
            nodes,
            metrics: RunMetrics::default(),
            prepared: false,
        })
    }

    pub fn node_order(&self) -> &[NodeId] {
        &self.node_order
    }
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn metrics(&self) -> RunMetrics {
        self.metrics
    }

    pub fn prepare(&mut self, cx: &PrepareContext) -> Result<(), ExecError> {
        for node in &mut self.nodes {
            node.prepare(cx).map_err(ExecError::from_node)?;
        }
        self.prepared = true;
        Ok(())
    }

    pub fn run_frame(&mut self, input: AudioFrame) -> Result<Option<AudioFrame>, ExecError> {
        if !self.prepared {
            return Err(ExecError::NotPrepared);
        }
        let mut current = input;
        for node in &mut self.nodes {
            match node.process(current).map_err(ExecError::from_node)? {
                Some(frame) => current = frame,
                None => return Ok(None),
            }
        }
        Ok(Some(current))
    }

    pub fn run_frames(&mut self, frames: Vec<AudioFrame>) -> Result<RunMetrics, ExecError> {
        if !self.prepared {
            return Err(ExecError::NotPrepared);
        }
        for frame in frames {
            self.metrics.frames_in += 1;
            match self.run_frame(frame)? {
                Some(_output) => self.metrics.frames_out += 1,
                None => self.metrics.frames_dropped += 1,
            }
        }
        Ok(self.metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, SampleFormat, SampleSpec, SourceId, StreamId};
    use pocketstation_graph::builtins::{GainFactory, PassthroughNode};
    use pocketstation_graph::dsl::AudioGraph;
    use pocketstation_graph::node::NodeConfig;
    use pocketstation_graph::registry::NodeFactory;

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

    fn node_ids(count: usize) -> Vec<NodeId> {
        let mut graph = AudioGraph::new();
        (0..count)
            .map(|_| graph.add_node("passthrough", NodeConfig::new()).id())
            .collect()
    }

    fn gain_node(gain_db: f32) -> Box<dyn RuntimeNode> {
        let config = NodeConfig::new().with(GAIN_DB_KEY, &gain_db.to_string());
        GainFactory.instantiate(&prepare_cx(), &config).unwrap()
    }

    struct GatingNode;
    impl RuntimeNode for GatingNode {
        fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
            Ok(())
        }
        fn process(&mut self, _frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
            Ok(None)
        }
    }

    #[test]
    fn given_mismatched_lengths_when_new_then_node_count_mismatch_error() {
        let result = RealtimeExecutor::new(node_ids(1), Vec::new());
        assert_eq!(
            result.err(),
            Some(ExecError::NodeCountMismatch { order: 1, nodes: 0 })
        );
    }

    #[test]
    fn given_unprepared_executor_when_run_frame_then_not_prepared_error() {
        let mut executor = RealtimeExecutor::new(Vec::new(), Vec::new()).unwrap();
        let result = executor.run_frame(frame_with_samples(&[0.5]));
        assert_eq!(result.err(), Some(ExecError::NotPrepared));
    }

    #[test]
    fn given_gain_node_when_run_frame_then_samples_scaled_by_linear_gain() {
        let nodes: Vec<Box<dyn RuntimeNode>> = vec![gain_node(6.0)];
        let mut executor = RealtimeExecutor::new(node_ids(1), nodes).unwrap();
        executor.prepare(&prepare_cx()).unwrap();

        let input = [0.5, -0.25, 1.0];
        let expected_linear = 10f32.powf(6.0 / 20.0);
        let output = executor
            .run_frame(frame_with_samples(&input))
            .unwrap()
            .unwrap();
        for (got, raw) in output.buffer.as_slice().iter().zip(input.iter()) {
            assert!((got - raw * expected_linear).abs() < 1e-5);
        }
    }

    #[test]
    fn given_gating_node_when_run_frame_then_short_circuits_to_none() {
        let nodes: Vec<Box<dyn RuntimeNode>> = vec![Box::new(GatingNode)];
        let mut executor = RealtimeExecutor::new(node_ids(1), nodes).unwrap();
        executor.prepare(&prepare_cx()).unwrap();

        let output = executor.run_frame(frame_with_samples(&[0.5])).unwrap();
        assert!(output.is_none());
    }

    #[test]
    fn given_passthrough_chain_when_run_frames_then_frames_in_equals_frames_out() {
        let nodes: Vec<Box<dyn RuntimeNode>> =
            vec![Box::new(PassthroughNode), Box::new(PassthroughNode)];
        let mut executor = RealtimeExecutor::new(node_ids(2), nodes).unwrap();
        executor.prepare(&prepare_cx()).unwrap();

        let frames = vec![
            frame_with_samples(&[0.1]),
            frame_with_samples(&[0.2]),
            frame_with_samples(&[0.3]),
        ];
        let metrics = executor.run_frames(frames).unwrap();
        assert_eq!(metrics.frames_in(), 3);
        assert_eq!(metrics.frames_out(), 3);
        assert_eq!(metrics.frames_dropped(), 0);
    }

    #[test]
    fn given_gating_node_when_run_frames_then_all_frames_counted_dropped() {
        let nodes: Vec<Box<dyn RuntimeNode>> = vec![Box::new(GatingNode)];
        let mut executor = RealtimeExecutor::new(node_ids(1), nodes).unwrap();
        executor.prepare(&prepare_cx()).unwrap();

        let frames = vec![frame_with_samples(&[0.1]), frame_with_samples(&[0.2])];
        let metrics = executor.run_frames(frames).unwrap();
        assert_eq!(metrics.frames_in(), 2);
        assert_eq!(metrics.frames_out(), 0);
        assert_eq!(metrics.frames_dropped(), 2);
    }
}

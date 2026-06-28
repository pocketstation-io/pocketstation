// pocketstation-graph — AudioGraph public API.
// compile() performs Kahn's cycle detection + topological sort.
// run() dispatches frames in topological order via GraphProcessor implementations.

// Wave 2 node model (declaration + registry + lifecycle). The legacy AudioGraph
// below is the frozen prototype; it is replaced by the DSL→GraphSpec rewrite in Wave 3.
pub mod builtins;
pub mod node;
pub mod registry;
pub mod runtime_node;

use std::collections::{HashMap, VecDeque};

// --- identity ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub(crate) u32);

impl NodeId {
    /// Returns the numeric identifier (opaque; for diagnostics and logging only).
    pub fn id(self) -> u32 {
        self.0
    }
}

// --- port references ---

#[derive(Debug, Clone)]
pub struct OutputPortRef {
    pub(crate) node_id: NodeId,
    #[allow(dead_code)] // port name used for documentation; routing by node_id in Phase 1
    pub(crate) port: String,
}

#[derive(Debug, Clone)]
pub struct InputPortRef {
    pub(crate) node_id: NodeId,
    #[allow(dead_code)]
    pub(crate) port: String,
}

// --- connection spec for single and fan-in wiring ---

pub enum ConnectionSpec {
    Single(OutputPortRef),
    FanIn(Vec<OutputPortRef>),
}

impl From<OutputPortRef> for ConnectionSpec {
    fn from(p: OutputPortRef) -> Self {
        Self::Single(p)
    }
}

impl<const N: usize> From<[OutputPortRef; N]> for ConnectionSpec {
    fn from(arr: [OutputPortRef; N]) -> Self {
        Self::FanIn(arr.into_iter().collect())
    }
}

// --- node selector (for policy cross-references) ---

#[allow(dead_code)] // inner NodeId is stored for Phase 1 routing; not yet read internally
#[derive(Debug, Clone, Copy)]
pub struct NodeSelector(pub(crate) NodeId);

// --- node handle ---

pub struct NodeHandle {
    pub(crate) id: NodeId,
}

impl NodeHandle {
    pub fn out(&self, port: &str) -> OutputPortRef {
        OutputPortRef {
            node_id: self.id,
            port: port.to_owned(),
        }
    }
    pub fn in_(&self, port: &str) -> InputPortRef {
        InputPortRef {
            node_id: self.id,
            port: port.to_owned(),
        }
    }
    pub fn selector(&self) -> NodeSelector {
        NodeSelector(self.id)
    }
}

// --- source nodes ---

pub enum SourceNode {
    Mic,
    App(String),
    SystemOutput, // DesktopSystemLoopback per capability model (DOCS-003)
    File(String),
    Synthetic, // sine wave, test only
}

// --- transform nodes ---

pub enum TransformNode {
    VAD, // speech detection; must run on processing thread, not callback (LAW 15)
    NoiseSuppressor,
    EchoCancel,
    GainControl,
}

// --- model providers ---

pub struct ModelProvider {
    name: &'static str,
}

impl ModelProvider {
    pub fn name(&self) -> &str {
        self.name
    }
}

/// Factory for the Deepgram STT provider.
pub fn deepgram() -> ModelProvider {
    ModelProvider { name: "deepgram" }
}
/// Factory for the OpenAI Realtime voice agent provider.
pub fn openai_realtime() -> ModelProvider {
    ModelProvider {
        name: "openai-realtime",
    }
}
/// Factory for a locally-hosted model provider.
pub fn local_model() -> ModelProvider {
    ModelProvider { name: "local" }
}

// --- model nodes ---

pub enum ModelNode {
    Transcribe(ModelProvider),     // STT — speech to text
    SpeechToSpeech(ModelProvider), // voice agent with audio I/O
    EmotionDetect(ModelProvider),  // prosody/emotion analysis
    TextToSpeech(ModelProvider),   // TTS — text to audio output
}

// --- policy nodes ---

pub enum PolicyNode {
    Duck {
        target: NodeSelector,
        gain_db: f32, // duck depth, negative = reduce; e.g. -12.0 (LAW 1)
        attack_ms: u32,
        release_ms: u32,
    },
    Gate {
        threshold_dbfs: f32,
    }, // opens above this level (LAW 1)
    Failover,
}

// --- transport nodes ---

pub enum TransportNode {
    Relay(String), // WebRTC/RTP relay — Phase 1 wires real pion connection
    WebSocket(String),
    LocalPipe,
}

// --- sink nodes ---

pub enum SinkNode {
    Browser,
    Speaker,
    MultiStemRecording(String), // named recording session
    VirtualMic,
}

// --- error ---

#[derive(Debug)]
pub enum GraphError {
    DuplicateEdge { to_port: String },
    UnknownNode(NodeId),
    CycleDetected,
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateEdge { to_port } => write!(f, "duplicate edge to port {to_port}"),
            Self::UnknownNode(id) => write!(f, "unknown node {}", id.0),
            Self::CycleDetected => write!(f, "cycle detected in audio graph"),
        }
    }
}

impl std::error::Error for GraphError {}

// --- GraphProcessor trait ---

/// Per-node audio frame processor. Attach to a node via AudioGraph::attach().
///
/// Called once per frame tick in topological order by AudioGraph::run().
///
/// LAW 15 contract: process() MUST be alloc-free, lock-free, blocking-free, and
/// log-free. All working state must be pre-allocated in the struct. Violating
/// this propagates latency jitter from the processing thread into the frame pipeline.
pub trait GraphProcessor: Send {
    /// Process one 20 ms frame.
    ///
    /// `input` is the sample-wise sum of all upstream node outputs (zero-filled
    /// for source nodes with no predecessors). Write processed samples into `output`.
    /// Both slices have the same length (FRAME_LEN_SAMPLES).
    fn process(&mut self, input: &[f32], output: &mut [f32]);
}

// --- compile output ---

/// Number of f32 samples in one 20 ms mono frame at 48 kHz.
pub const FRAME_LEN_SAMPLES: usize = 960;

/// Validated, topologically ordered execution plan produced by AudioGraph::compile().
///
/// Holds pre-allocated per-node output buffers and a mix scratch buffer so that
/// AudioGraph::run() performs zero heap allocation during frame traversal.
pub struct GraphPlan {
    topo_order: Vec<NodeId>,
    node_count: usize,
    edge_count: usize,
    // Flat buffer: frame_buffers[node_id.0 * FRAME_LEN_SAMPLES..(+1)*FRAME_LEN_SAMPLES]
    frame_buffers: Vec<f32>,
    mix_scratch: Vec<f32>,
}

impl GraphPlan {
    pub fn node_count(&self) -> usize {
        self.node_count
    }
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
    /// Topological processing order (sources first, sinks last).
    pub fn topo_order(&self) -> &[NodeId] {
        &self.topo_order
    }
}

// --- internal storage ---

struct GraphEdge {
    from: OutputPortRef,
    to: InputPortRef,
}

#[allow(dead_code)]
enum NodeKind {
    Source,
    Transform,
    Model,
    Policy,
    Transport,
    Sink,
}

#[allow(dead_code)]
struct GraphNode {
    id: NodeId,
    kind: NodeKind,
}

// --- Kahn's topological sort with cycle detection ---

fn kahn_topo_sort(nodes: &[GraphNode], edges: &[GraphEdge]) -> Result<Vec<NodeId>, GraphError> {
    let mut in_degree: HashMap<NodeId, usize> = nodes.iter().map(|n| (n.id, 0)).collect();
    let mut adj: HashMap<NodeId, Vec<NodeId>> = nodes.iter().map(|n| (n.id, vec![])).collect();

    for edge in edges {
        *in_degree.entry(edge.to.node_id).or_insert(0) += 1;
        adj.entry(edge.from.node_id)
            .or_default()
            .push(edge.to.node_id);
    }

    let mut queue: VecDeque<NodeId> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&id, _)| id)
        .collect();
    // Sort for deterministic ordering (sources before transforms before sinks when possible)
    let mut queue_vec: Vec<NodeId> = queue.drain(..).collect();
    queue_vec.sort_by_key(|id| id.0);
    queue.extend(queue_vec);

    let mut order = Vec::with_capacity(nodes.len());
    while let Some(id) = queue.pop_front() {
        order.push(id);
        let mut neighbors = adj.remove(&id).unwrap_or_default();
        neighbors.sort_by_key(|id| id.0);
        for neighbor in neighbors {
            let deg = in_degree.get_mut(&neighbor).unwrap();
            *deg -= 1;
            if *deg == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    if order.len() != nodes.len() {
        return Err(GraphError::CycleDetected);
    }
    Ok(order)
}

// --- AudioGraph ---

pub struct AudioGraph {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
    next_id: u32,
    processors: HashMap<u32, Box<dyn GraphProcessor>>,
}

impl AudioGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            next_id: 0,
            processors: HashMap::new(),
        }
    }

    fn alloc_node(&mut self, kind: NodeKind) -> NodeHandle {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        self.nodes.push(GraphNode { id, kind });
        NodeHandle { id }
    }

    pub fn source(&mut self, _spec: SourceNode) -> NodeHandle {
        self.alloc_node(NodeKind::Source)
    }
    pub fn transform(&mut self, _spec: TransformNode) -> NodeHandle {
        self.alloc_node(NodeKind::Transform)
    }
    pub fn model(&mut self, _spec: ModelNode) -> NodeHandle {
        self.alloc_node(NodeKind::Model)
    }
    pub fn policy(&mut self, _spec: PolicyNode) -> NodeHandle {
        self.alloc_node(NodeKind::Policy)
    }
    pub fn transport(&mut self, _spec: TransportNode) -> NodeHandle {
        self.alloc_node(NodeKind::Transport)
    }
    pub fn sink(&mut self, _spec: SinkNode) -> NodeHandle {
        self.alloc_node(NodeKind::Sink)
    }

    /// Attach a processor to a node. The processor's process() is called by run().
    /// Nodes without an attached processor act as passthrough (output = mixed input).
    pub fn attach(&mut self, handle: &NodeHandle, processor: Box<dyn GraphProcessor>) {
        self.processors.insert(handle.id.0, processor);
    }

    pub fn connect(
        &mut self,
        from: impl Into<ConnectionSpec>,
        to: InputPortRef,
    ) -> Result<(), GraphError> {
        match from.into() {
            ConnectionSpec::Single(out) => {
                self.edges.push(GraphEdge { from: out, to });
            }
            ConnectionSpec::FanIn(outs) => {
                for out in outs {
                    self.edges.push(GraphEdge {
                        from: out,
                        to: to.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Validate the graph topology (cycle detection) and produce an execution plan.
    ///
    /// Runs Kahn's BFS topological sort. Returns CycleDetected if the graph has cycles.
    /// Pre-allocates all per-node frame buffers so run() is allocation-free.
    pub fn compile(&self) -> Result<GraphPlan, GraphError> {
        let topo_order = kahn_topo_sort(&self.nodes, &self.edges)?;
        let node_count = self.nodes.len();
        let edge_count = self.edges.len();
        // Pre-allocate: one FRAME_LEN_SAMPLES slot per node + one mix scratch slot
        let frame_buffers = vec![0.0f32; node_count * FRAME_LEN_SAMPLES];
        let mix_scratch = vec![0.0f32; FRAME_LEN_SAMPLES];
        Ok(GraphPlan {
            topo_order,
            node_count,
            edge_count,
            frame_buffers,
            mix_scratch,
        })
    }

    /// Execute one frame tick: traverse nodes in topological order, mix upstream
    /// outputs, and dispatch to registered processors.
    ///
    /// The plan's pre-allocated buffers are reused — no heap allocation during traversal.
    /// Nodes with no attached processor act as passthrough (output = mixed input).
    pub fn run(&mut self, plan: &mut GraphPlan) -> Result<(), GraphError> {
        for i in 0..plan.topo_order.len() {
            let node_id = plan.topo_order[i];
            let idx = node_id.0 as usize;

            // Mix all upstream outputs into mix_scratch
            plan.mix_scratch.fill(0.0);
            for edge in &self.edges {
                if edge.to.node_id == node_id {
                    let up = edge.from.node_id.0 as usize;
                    let base = up * FRAME_LEN_SAMPLES;
                    for (s, &u) in plan
                        .mix_scratch
                        .iter_mut()
                        .zip(&plan.frame_buffers[base..base + FRAME_LEN_SAMPLES])
                    {
                        *s += u;
                    }
                }
            }

            let out_base = idx * FRAME_LEN_SAMPLES;
            let out_end = out_base + FRAME_LEN_SAMPLES;

            if let Some(proc) = self.processors.get_mut(&node_id.0) {
                // Stack copy of mix_scratch (3840 bytes) so we can separately
                // borrow frame_buffers[out_base..out_end] as mut. Stack allocation
                // only — no heap (LAW 15: no alloc in processing path).
                let mut mix_copy = [0.0f32; FRAME_LEN_SAMPLES];
                mix_copy.copy_from_slice(&plan.mix_scratch);
                proc.process(&mix_copy, &mut plan.frame_buffers[out_base..out_end]);
            } else {
                // Passthrough: mix_scratch and frame_buffers are distinct Vec fields;
                // NLL field-splitting allows simultaneous & and &mut here.
                plan.frame_buffers[out_base..out_end].copy_from_slice(&plan.mix_scratch);
            }
        }
        Ok(())
    }

    /// Read the output of a node after run() completes. Returns a zero slice if the
    /// node_id is out of range.
    pub fn output<'p>(&self, plan: &'p GraphPlan, handle: &NodeHandle) -> &'p [f32] {
        let idx = handle.id.0 as usize;
        let base = idx * FRAME_LEN_SAMPLES;
        let end = base + FRAME_LEN_SAMPLES;
        if end <= plan.frame_buffers.len() {
            &plan.frame_buffers[base..end]
        } else {
            &[]
        }
    }
}

impl Default for AudioGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct GainProc(f32);
    impl GraphProcessor for GainProc {
        fn process(&mut self, input: &[f32], output: &mut [f32]) {
            for (o, &i) in output.iter_mut().zip(input) {
                *o = i * self.0;
            }
        }
    }

    struct FillProc(f32);
    impl GraphProcessor for FillProc {
        fn process(&mut self, _input: &[f32], output: &mut [f32]) {
            output.fill(self.0);
        }
    }

    #[test]
    fn given_empty_graph_when_compile_then_plan_counts_are_zero() {
        let graph = AudioGraph::new();
        let plan = graph.compile().unwrap();
        assert_eq!(plan.node_count(), 0);
        assert_eq!(plan.edge_count(), 0);
    }

    #[test]
    fn given_source_and_sink_when_connect_then_plan_records_one_edge() {
        let mut graph = AudioGraph::new();
        let src = graph.source(SourceNode::Mic);
        let sink = graph.sink(SinkNode::Browser);
        graph.connect(src.out("voice"), sink.in_("audio")).unwrap();
        let plan = graph.compile().unwrap();
        assert_eq!(plan.node_count(), 2);
        assert_eq!(plan.edge_count(), 1);
    }

    #[test]
    fn given_two_sources_when_fan_in_connect_then_plan_records_two_edges() {
        let mut graph = AudioGraph::new();
        let mic = graph.source(SourceNode::Mic);
        let discord = graph.source(SourceNode::App("Discord".to_owned()));
        let sink = graph.sink(SinkNode::Browser);
        graph
            .connect([mic.out("voice"), discord.out("audio")], sink.in_("audio"))
            .unwrap();
        let plan = graph.compile().unwrap();
        assert_eq!(plan.node_count(), 3);
        assert_eq!(plan.edge_count(), 2);
    }

    #[test]
    fn given_cycle_in_graph_when_compile_then_cycle_detected_error() {
        let mut graph = AudioGraph::new();
        let a = graph.transform(TransformNode::VAD);
        let b = graph.transform(TransformNode::GainControl);
        // a → b → a forms a cycle
        graph.connect(a.out("out"), b.in_("in")).unwrap();
        graph.connect(b.out("out"), a.in_("in")).unwrap();
        assert!(matches!(graph.compile(), Err(GraphError::CycleDetected)));
    }

    #[test]
    fn given_source_and_gain_and_sink_when_run_then_output_is_amplified() {
        let mut graph = AudioGraph::new();
        let src = graph.source(SourceNode::Mic);
        let gain = graph.transform(TransformNode::GainControl);
        let sink = graph.sink(SinkNode::Speaker);

        graph.connect(src.out("out"), gain.in_("in")).unwrap();
        graph.connect(gain.out("out"), sink.in_("in")).unwrap();

        // Source fills with 0.5; gain doubles to 1.0
        graph.attach(&src, Box::new(FillProc(0.5)));
        graph.attach(&gain, Box::new(GainProc(2.0)));

        let mut plan = graph.compile().unwrap();
        graph.run(&mut plan).unwrap();

        let out = graph.output(&plan, &sink);
        assert!(
            out.iter().all(|&s| (s - 1.0).abs() < 1e-6),
            "expected 1.0 after gain×2 on 0.5 input, got {:?}",
            &out[..4]
        );
    }

    #[test]
    fn given_duck_policy_when_connect_sidechain_then_plan_records_edges() {
        let mut graph = AudioGraph::new();
        let mic = graph.source(SourceNode::Mic);
        let spotify = graph.source(SourceNode::App("Spotify".to_owned()));
        let relay = graph.transport(TransportNode::Relay("room".to_owned()));
        let duck = graph.policy(PolicyNode::Duck {
            target: spotify.selector(),
            gain_db: -12.0,
            attack_ms: 40,
            release_ms: 400,
        });
        graph
            .connect(mic.out("voice"), duck.in_("sidechain"))
            .unwrap();
        graph
            .connect(spotify.out("music"), duck.in_("program"))
            .unwrap();
        graph
            .connect(duck.out("audio"), relay.in_("music"))
            .unwrap();
        let plan = graph.compile().unwrap();
        assert_eq!(plan.node_count(), 4);
        assert_eq!(plan.edge_count(), 3);
    }

    #[test]
    fn given_fan_in_two_sources_when_run_then_outputs_are_summed() {
        let mut graph = AudioGraph::new();
        let src_a = graph.source(SourceNode::Mic);
        let src_b = graph.source(SourceNode::App("Discord".to_owned()));
        let sink = graph.sink(SinkNode::Browser);

        graph
            .connect([src_a.out("out"), src_b.out("out")], sink.in_("audio"))
            .unwrap();

        graph.attach(&src_a, Box::new(FillProc(0.3)));
        graph.attach(&src_b, Box::new(FillProc(0.4)));

        let mut plan = graph.compile().unwrap();
        graph.run(&mut plan).unwrap();

        let out = graph.output(&plan, &sink);
        assert!(
            out.iter().all(|&s| (s - 0.7).abs() < 1e-5),
            "expected 0.7 (sum of 0.3+0.4), got {:?}",
            &out[..4]
        );
    }
}

// Phase 0: public AudioGraph API — node, edge, and connection types.
// Enables AudioGraph::new().source().connect().compile().run() to compile.
// Execution is a scaffold; see FAKE_SCAFFOLD_INVENTORY S-graph-run-01.

// --- identity ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u32);

// --- port references ---

// Phase 0: fields stored for Phase 1 compile/routing; not yet read internally.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OutputPortRef {
    node_id: NodeId,
    port: String,
}

// Phase 0: fields stored for Phase 1 compile/routing; not yet read internally.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InputPortRef {
    node_id: NodeId,
    port: String,
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

// Phase 0: inner NodeId stored for Phase 1 routing; not yet read internally.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct NodeSelector(NodeId);

// --- node handle ---

pub struct NodeHandle {
    id: NodeId,
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
    NotImplemented(&'static str), // Phase 0 scaffold; real errors added in Phase 1
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateEdge { to_port } => write!(f, "duplicate edge to port {to_port}"),
            Self::UnknownNode(id) => write!(f, "unknown node {}", id.0),
            Self::CycleDetected => write!(f, "cycle detected in audio graph"),
            Self::NotImplemented(msg) => write!(f, "not implemented: {msg}"),
        }
    }
}

impl std::error::Error for GraphError {}

// --- compile output (opaque) ---

pub struct GraphPlan {
    node_count: usize,
    edge_count: usize,
}

impl GraphPlan {
    pub fn node_count(&self) -> usize {
        self.node_count
    }
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}

// --- internal storage --- Phase 0: fields stored for Phase 1 scheduler; not yet read.

#[allow(dead_code)]
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

// --- AudioGraph ---

pub struct AudioGraph {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
    next_id: u32,
}

impl AudioGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            next_id: 0,
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

    pub fn compile(&self) -> Result<GraphPlan, GraphError> {
        // Phase 0: accepts any topology; Phase 1 adds cycle detection + adapter insertion
        Ok(GraphPlan {
            node_count: self.nodes.len(),
            edge_count: self.edges.len(),
        })
    }

    // Phase 0 scaffold: returns immediately; Phase 1 wires real scheduling loop.
    // See FAKE_SCAFFOLD_INVENTORY S-graph-run-01.
    pub fn run(&mut self, _plan: GraphPlan) -> Result<(), GraphError> {
        Ok(())
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
    fn given_connected_graph_when_run_then_returns_ok() {
        let mut graph = AudioGraph::new();
        let src = graph.source(SourceNode::Mic);
        let sink = graph.sink(SinkNode::Browser);
        graph.connect(src.out("voice"), sink.in_("audio")).unwrap();
        let plan = graph.compile().unwrap();
        assert!(graph.run(plan).is_ok());
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
}

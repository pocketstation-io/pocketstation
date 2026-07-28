# PocketStation
## Program Document v3.0

**Date:** 2026-06-26
**Status:** Historical vision and design record. This document is not the
current product contract, repository map, milestone plan, or implementation
status.
**Supersedes:** v2.3 (Universal Audio Fabric / mobile-first SDK + relay positioning)

> **Canonical current contracts — 2026-07-28:** This file intentionally
> preserves the earlier broad AudioGraph vision. It must not be copied into a
> current status report or used as an implementation requirement. The binding
> product and deadline contract is the factory-root
> `PRODUCT_OPERATING_CONTRACT.md`; current repository and implementation truth
> is `PROJECT_STATE.md`; execution order and acceptance are in
> `CURRENT_PLAN.md` and `docs/PocketStation-BuildGuide.md`; current crate
> boundaries are maintained locally in `CRATE_OWNERSHIP.md` and accepted ADRs.
> Where this historical document disagrees with any of those sources, those
> canonical contracts win.

---

## Changelog v2.3 → v3.0

| v2.3 | v3.0 | Why |
|---|---|---|
| "Universal Audio Fabric" as vision phrase | "Realtime Audio Graph Infrastructure" | "Fabric" is poetic; "graph" is executable, observable, and defensible |
| Source → route → output | SourceNode → Edge → Bus → Policy/Model/Transport → Sink | The product must support branching, model triggers, policies, and per-edge observability |
| Relay as primary infra artifact | Relay as one TransportNode implementation | A relay alone can be commoditized; a graph runtime with source identity and policies cannot |
| Mobile-first SDK framing | Capture-first, graph-first, cross-platform | Existing desktop CLI/capture work is a strong wedge; treating it as secondary was wrong |
| Creator station as first product | Developer graph API first; creator station is one graph template | Creators are a wedge, not the full market |
| Voice-agent backend as a route kind | ModelNode as a first-class graph node | AI-era workloads need model switching, fan-out, fallback, semantic triggers, and model observability |
| Room = source + listeners | Room = distributed GraphSession | Allows multi-source, multi-bus, model nodes, private enterprise sessions, and graph replay |
| ProcessorGraph as internal implementation detail | AudioGraph as the public product center | The graph is not inside PocketStation — PocketStation IS the graph runtime |
| `start_broadcast()` as primary API | `AudioGraph::new().connect().run()` as primary API | The graph wiring is the product; `start_broadcast()` becomes a convenience template |
| Observability = metrics | Observability = per-edge, per-session, per-model product surface | Enterprise voice systems need debuggability across the entire path |

The non-changing core algorithm (frame pool, SPSC ring, codec, clock sync, hot-path rules) is preserved exactly. The graph abstraction sits above it. The hot path is unchanged.

---

## Table of Contents

1. [Vision and Thesis](#1-vision-and-thesis)
2. [What PocketStation Is and Is Not](#2-what-pocketstation-is-and-is-not)
3. [The AudioGraph — Product Center](#3-the-audiograph--product-center)
4. [The Non-Changing Core Algorithm](#4-the-non-changing-core-algorithm)
5. [Source Capability Model](#5-source-capability-model)
6. [Architecture Decision Records](#6-architecture-decision-records)
7. [Platform Adapter Specifications](#7-platform-adapter-specifications)
8. [Relay Architecture — GraphSession](#8-relay-architecture--graphsession)
9. [Security Model](#9-security-model)
10. [Model Nodes and ML Processing Layer](#10-model-nodes-and-ml-processing-layer)
11. [Observability](#11-observability)
12. [Full Tech Stack](#12-full-tech-stack)
13. [Developer API](#13-developer-api)
14. [Repository Structure](#14-repository-structure)
15. [Build Phase Plan](#15-build-phase-plan)
16. [Market Reality](#16-market-reality)
17. [Target Markets — Ranked and Honest](#17-target-markets--ranked-and-honest)
18. [Business Model](#18-business-model)
19. [Infrastructure Cost Strategy](#19-infrastructure-cost-strategy)
20. [Funding Strategy](#20-funding-strategy)
21. [Research Path](#21-research-path)
22. [Competitive Landscape](#22-competitive-landscape)
23. [Threat Analysis](#23-threat-analysis)
24. [Kill Criteria](#24-kill-criteria)
25. [Strategic Positioning](#25-strategic-positioning)
26. [Open Engineering Questions](#26-open-engineering-questions)

---

## 1. Vision and Thesis

### 1.1 The Permanent Vision

> **Any audio → any graph → any human, app, device, room, model, or agent.**

The v2.3 line "Any audio → any route → any output" remains valid. v3.0 makes it precise: the route is not a pipe. It is a graph with typed edges, semantic sources, policy nodes, model nodes, and per-edge observability. Every source is a node. Every connection is a contract.

### 1.2 What "PocketStation" Means

**Pocket:** a fast, discrete, self-contained node of any signal type — audio, metadata, transcript events, control signals. Not "fits in your pocket." A pocket of signal in motion. Compact by design, not by limitation.

**Station:** a directed orchestration point. Not a room people join. A station receives, processes, and dispatches with intent — like a relay station, a base station, a switching station that knows the semantics of what it handles and routes accordingly.

Together: a fast, programmable orchestration node for any signal type. Audio now. JSON events, video, telemetry later. The name survives every product evolution.

### 1.3 The One-Sentence Thesis

> PocketStation is audio-native realtime graph infrastructure: capture any audio source, give it semantic identity, route it through programmable local/remote/model graph nodes, and expose quality and latency telemetry across every edge.

### 1.4 The Technical Claim

> We are not building video-call rooms. We are building realtime audio graphs that cross devices, apps, browsers, phones, and AI models.

### 1.5 The Strategic Position

> LiveKit owns realtime rooms. Vapi owns voice-agent deployment. OpenAI, Deepgram, and ElevenLabs own models. PocketStation owns the programmable audio I/O graph between operating systems, apps, humans, agents, and model pipelines.

### 1.6 The Distinction

**Routing alone = thin.** Capture app audio and send it to a remote listener. Useful, not venture-scale.

**Audio graph infrastructure = the full thesis:**

```
OS audio capture
  → per-source identity and capability discovery
  → graph wiring with typed edge contracts
  → local transform nodes (VAD, noise suppression, gain, mix)
  → policy nodes (ducking, routing decisions, model switching)
  → model nodes (STT, LLM, TTS, translation, diarization)
  → transport nodes (relay, WebRTC, RTP, QUIC, local pipe)
  → sink nodes (browser, mobile, recording, AI backend, virtual mic)
  → per-edge observability (latency, loss, jitter, drift, cost)
```

That is the company.

---

## 2. What PocketStation Is and Is Not

### Is

```
A realtime audio graph runtime for developers.
A cross-platform audio source capture and identity layer.
A distributed audio room/relay infrastructure built around semantic sources and buses.
A model-routing fabric for STT, LLM, TTS, translation, diarization, and audio enhancement.
A developer SDK for voice AI, creator broadcast, accessibility, education, game audio, and enterprise audio.
A local + cloud audio observability system.
A future virtual endpoint layer for OS-level audio integration.
```

### Is Not

```
A generic LiveKit clone.
A generic Vapi clone.
A model company competing with OpenAI, Deepgram, or ElevenLabs.
Only a podcasting or creator app.
Only a desktop audio router.
A promise to silently capture restricted OS audio that platforms do not allow.
A Spotify redistribution tool.
A video-conferencing platform.
```

### Positioning Statements

**Developer:**
> Build realtime audio graphs across apps, devices, browsers, and AI models.

**AI infrastructure:**
> PocketStation gives voice AI systems programmable audio I/O, routing, model switching, and latency observability.

**Creator:**
> Route any app, mic, music, or device into a live station, recording, remote listener, or AI tool.

**Enterprise:**
> Private realtime audio graph infrastructure with observability, policy, fallback, and model-provider control.

---

## 3. The AudioGraph — Product Center

### 3.1 The Graph Is the Product

v2.3 had the right internal primitives: `ProcessorGraph`, `AudioProcessorNode`, `RoutePlan`, `SourceCapability`, observability. Those primitives were framed as implementation details beneath the product. The v3.0 change: they are the product.

The relay is not the company. The relay is `TransportNode::Relay`. The ML layer is not a feature. It is `ModelNode` and `TransformNode` instances in the graph. Platform adapters are `SourceNode` adapter implementations. Creator station is a graph template. The iOS SDK is the Swift wrapper for `SourceNode` adapters on iOS. Everything composes through the graph.

### 3.2 Core Vocabulary

**Node** — a typed unit of audio or data work:

```rust
pub trait AudioGraphNode: Send + Sync {
    fn id(&self) -> NodeId;
    fn kind(&self) -> NodeKind;
    fn inputs(&self) -> Vec<PortSpec>;
    fn outputs(&self) -> Vec<PortSpec>;
    fn constraints(&self) -> NodeConstraints;
}
```

**Port** — a named, typed stream input or output:

```rust
pub struct PortSpec {
    pub name:               String,
    pub media_type:         MediaType,
    pub sample_rate:        Option<u32>,
    pub channels:           Option<ChannelLayout>,
    pub frame_duration_ms:  Option<u16>,
    pub semantic_role:      Option<SemanticRole>,
}
```

**Edge** — a typed streaming contract connecting one output port to one input port:

```rust
pub struct EdgeSpec {
    pub from:           PortRef,
    pub to:             PortRef,
    pub contract:       EdgeContract,
    pub qos:            QosPolicy,
    pub observability:  EdgeObservabilityPolicy,
}
```

**Graph** — the compiled execution plan:

```rust
pub struct AudioGraph {
    nodes:    Graph<NodeId, Box<dyn AudioGraphNode>>,
    edges:    Vec<EdgeSpec>,
    policies: Vec<GraphPolicy>,
}
```

#### Semantic control values

Domain alternatives cross component boundaries as named types, never as
boolean conventions. Causes, policies, modes, directions, roles, ownership
rules, and telemetry outcomes use enums or newtypes. Booleans remain valid only
for irreducibly binary state whose name reads as a yes/no question.

For example, an edge reports `EdgeDropReason::QueueFull` or
`EdgeDropReason::BranchPoolExhausted`; it does not derive a boolean from
`CopyPolicy` and ask telemetry to reinterpret it. A WHIP/WHEP handler accepts a
typed publish/subscribe direction; callers do not pass `true` or `false` with a
comment explaining the meaning. This preserves exhaustiveness, keeps telemetry
independent from routing policy, and lets contracts evolve without boolean
flag combinations.

### 3.3 Node Taxonomy

#### SourceNode

```rust
pub enum SourceNode {
    Mic,
    SystemOutput,
    App(String),
    Device(DeviceId),
    BrowserTab(TabId),
    File(PathBuf),
    NetworkStream(StreamUrl),
    VirtualInput(VirtualDeviceId),
    ModelOutput(ModelNodeId),     // TTS or agent output as a source
    SyntheticSine,                // testing/Phase 0
}
```

Every source has semantic identity:

```rust
pub struct SourceIdentity {
    pub source_id:          SourceId,
    pub display_name:       String,
    pub kind:               SourceKind,
    pub platform:           PlatformId,
    pub app_bundle_id:      Option<String>,
    pub device_id:          Option<String>,
    pub human_owner:        Option<UserId>,
    pub agent_owner:        Option<AgentId>,
    pub model_owner:        Option<ModelProviderId>,
    pub capture_capability: SourceCapability,
    pub clock_domain:       ClockDomainId,
    pub privacy_class:      PrivacyClass,
}
```

Generic WebRTC has tracks. PocketStation has **meaningful sources**. This is a durable moat.

Source nodes emit typed output ports: `audio.raw`, `audio.voice`, `audio.music`, `audio.system`, `metadata.source_state`, `metrics.capture`.

#### TransformNode

Deterministic signal-processing nodes. All run on the Rust processing thread, never inside platform audio callbacks.

```rust
pub enum TransformNode {
    Gain { db: f32 },
    Resample { sample_rate: u32 },
    MonoMix,
    StereoUpmix,
    NoiseSuppress,
    EchoCancel,
    VAD,
    SourceSeparate,
    LoudnessNormalize { target_lufs: f32 },
    Encode { codec: Codec },
    Decode { codec: Codec },
    Duck { target: NodeSelector, db: f32, attack_ms: u32, release_ms: u32 },
    Gate { threshold_dbfs: f32 },
    Limiter { ceiling_dbfs: f32 },
    Compressor { ratio: f32, threshold_dbfs: f32 },
    Watermark,
}
```

Built-in (Phase 0): `PassthroughNode`, `GainNode`, `ResampleNode`, `MonoMixNode`.
Optional (Phase 4+, feature flags): `VadNode`, `NoiseSuppressorNode`, `EchoCancelNode`, `SourceSeparationNode`.

All nodes declare `accepted_channels()`. Graph auto-inserts `MonoMixNode` upstream of any mono-only node. Insertions are zero-allocation passes using the existing pool.

#### Policy Operators

Policy behavior is implemented as open operators with manifests, not as a
closed `PolicyNode` enum. Core only sees `OperatorId`, declared ports,
`SignalSpec`, `ExecutionPartition`, `SafetyContract`, and typed config.

Examples such as duck, gate, route-if, failover, cost-cap, model-switch, and
recording trigger are concrete operator implementations registered by first
party audio crates, examples, or user/community code. Adding a new policy must
not require editing `pks-graph` or `pks-runtime`.

#### Model Operators

Model processing makes PocketStation AI-native without becoming a model
company. Model behavior is an open operator contract, not a closed `ModelNode`
enum.

Transcribe, translate, TTS, speech-to-speech, emotion, diarization, intent, and
agent operators are registered by examples, extension crates, or user code.
Whisper, Ollama, Deepgram, OpenAI, ElevenLabs, and future providers do not live
as enum variants in core.

Model operators expose latency, cost, and quality telemetry:

```rust
pub struct ModelNodeMetrics {
    pub provider:           ModelProviderId,
    pub model_name:         String,
    pub input_audio_ms:     u64,
    pub first_token_ms:     Option<u64>,
    pub first_audio_ms:     Option<u64>,
    pub total_response_ms:  Option<u64>,
    pub error_rate:         f32,
    pub cost_estimate_usd:  Option<f64>,
}
```

Model output (TTS, agent speech) can be fed back into the graph as a `SourceNode::ModelOutput` — completing the full loop.

#### TransportNode

```rust
pub enum TransportNode {
    LocalBus,
    Relay(RoomId),
    WebRTC(PeerConfig),
    RTP(RtpConfig),
    QUIC(QuicConfig),
    WebSocket(WsConfig),
    FileSegment(SegmentConfig),
    VirtualDevice(VirtualDeviceId),
    SIP(SipConfig),
}
```

Transport nodes move graph buses across process, device, network, or storage boundaries. Phase 1 uses `TransportNode::Relay` over Pion v4 WebRTC. The abstraction lets Phase 5+ add QUIC or SIP without changing the graph API.

#### SinkNode

```rust
pub enum SinkNode {
    Speaker,
    Browser,
    MobileApp,
    DesktopApp,
    VirtualMic,
    MultiStemRecording(RecordingId),
    TranscriptLog(LogId),
    MetricsExport(ExporterId),
    AgentInput(AgentId),
    Webhook(WebhookConfig),
}
```

### 3.4 Edge Contract

An edge is not `connect(A, B)`. It is a typed streaming contract:

```rust
pub struct EdgeContract {
    pub media_type:         MediaType,
    pub clock_domain:       ClockDomain,
    pub ordering:           OrderingPolicy,
    pub backpressure:       BackpressurePolicy,
    pub latency_budget_ms:  u32,
    pub jitter_budget_ms:   u32,
    pub loss_policy:        LossPolicy,
    pub conversion_policy:  ConversionPolicy,
    pub encryption:         EncryptionMode,
}
```

Recommended defaults:

```rust
BackpressurePolicy::DropNewest
OrderingPolicy::MonotonicPerStream
ConversionPolicy::AutoInsertAdapters
LossPolicy::ConcealForAudio_DropForMetadata
EncryptionMode::TransportOnly
```

For model edges:

```rust
BackpressurePolicy::CoalesceMetadata
LossPolicy::NeverDropFinalTranscript
```

For music/broadcast:

```rust
BackpressurePolicy::IncreaseBufferUntilLimitThenDropNewest
LossPolicy::NeverResampleWithoutDeclaration
```

### 3.5 Compile-Time Graph Validation

Before runtime, the graph validates:

1. Media type compatibility across connected ports
2. Channel / sample-rate compatibility
3. Auto-insertion of required adapters (ResampleNode, MonoMixNode)
4. Cycle detection (unless explicitly permitted)
5. Clock-domain crossing verification
6. Privacy / encryption policy enforcement
7. Latency budget estimation
8. Model cost estimation

```rust
let plan = graph.compile()?;
println!("{} nodes, {} edges", plan.node_count(), plan.edge_count());
println!("estimated transport p95: {}ms", plan.estimated_p95_ms());
println!("estimated model cost: ${:.4}/hr", plan.estimated_cost_usd_per_hour());
```

### 3.6 The Layered Architecture

```
┌─────────────────────────────────────────────────────────┐
│ Product Surfaces                                         │
│ CLI / SDK / Creator App / Web Receiver / Enterprise UI   │
└────────────────────────────┬────────────────────────────┘
                             ↓
┌─────────────────────────────────────────────────────────┐
│ Graph Control Plane                                      │
│ Graph definitions, sessions, policies, routes, auth      │
└────────────────────────────┬────────────────────────────┘
                             ↓
┌─────────────────────────────────────────────────────────┐
│ Graph Runtime                                            │
│ SourceNode, TransformNode, PolicyNode, ModelNode,        │
│ TransportNode, SinkNode, EdgeContract, EdgeMetrics        │
└────────────────────────────┬────────────────────────────┘
                             ↓
┌─────────────────────────────────────────────────────────┐
│ Audio Core (§4 — unchanged from v2.3)                   │
│ Frame pool · ring bus · clock sync · mixer · codec       │
└────────────────────────────┬────────────────────────────┘
                             ↓
┌─────────────────────────────────────────────────────────┐
│ Platform Source Adapters (§7 — unchanged from v2.3)      │
│ macOS · Windows · Linux · iOS · Android · browser        │
└────────────────────────────┬────────────────────────────┘
                             ↓
┌─────────────────────────────────────────────────────────┐
│ Transport Layer                                          │
│ WebRTC / RTP / QUIC / WebSocket / local pipe / virt dev  │
└─────────────────────────────────────────────────────────┘
```

The Audio Core and Platform Source Adapters are not changed. The Graph Runtime is the new public product surface above them.

### 3.7 The Holy-Shit Demo

```rust
let graph = AudioGraph::new();

let mic     = graph.source(SourceNode::Mic);
let discord = graph.source(SourceNode::App("Discord"));
let spotify = graph.source(SourceNode::App("Spotify"));

let vad     = graph.transform(TransformNode::VAD);
let stt     = graph.model(ModelNode::Transcribe(deepgram()));
let agent   = graph.model(ModelNode::SpeechToSpeech(openai_realtime()));
let emotion = graph.model(ModelNode::EmotionDetect(local_model()));
let duck    = graph.policy(PolicyNode::Duck {
    target: spotify.selector(),
    db: -12.0,
    attack_ms: 40,
    release_ms: 400,
});

let relay   = graph.transport(TransportNode::Relay("room-demo"));
let rec     = graph.sink(SinkNode::MultiStemRecording("demo-session"));
let browser = graph.sink(SinkNode::Browser);

// Mic → VAD → STT + agent.
graph.connect(mic.out("voice"), vad.in_("audio"))?;
graph.connect(vad.out("speech"), stt.in_("audio"))?;
graph.connect(vad.out("speech"), agent.in_("audio"))?;
graph.connect(stt.out("transcript"), relay.in_("events"))?;

// Discord → emotion detector.
graph.connect(discord.out("audio"), emotion.in_("audio"))?;
graph.connect(emotion.out("stress_signal"), relay.in_("events"))?;

// Spotify ducks when mic or Discord is active.
graph.connect([mic.out("voice"), discord.out("audio")], duck.in_("sidechain"))?;
graph.connect(spotify.out("music"), duck.in_("program"))?;
graph.connect(duck.out("audio"), relay.in_("music"))?;

// Everything recorded as separate stems.
graph.connect([mic.out("voice"), discord.out("audio"), spotify.out("music")], rec.in_("stems"))?;

// Agent speech goes to browser listener.
graph.connect(agent.out("audio"), relay.in_("agent_voice"))?;
graph.connect(relay.out("mix"), browser.in_("audio"))?;

let plan = graph.compile()?;
graph.run(plan).await?;
```

This is no longer "phone audio to browser." This is why PocketStation is not a relay clone.

### 3.8 Compatibility Layer

The old broadcast API remains as a convenience wrapper that compiles to a graph template internally. It is never removed — it is just sugar:

```rust
// This:
station.start_broadcast(source).await?;

// Compiles to:
// SourceNode → TransformNode::Encode → TransportNode::Relay → SinkNode::Browser
```

Simple users get the simple API. Advanced users get the graph. Both are valid. Neither breaks the other.

---

## 4. The Non-Changing Core Algorithm

This is what executes inside every graph edge. It does not change when a new node type is added. It does not change when a new platform adapter is added. It does not change when a new transport is added.

```
InputNode → NormalizeNode → FrameBus → ClockSync → RingBuffer →
ProcessorGraph → Mixer → Encoder → Transport →
Receiver/JitterBuffer → OutputNode
```

Pipeline shape is invariant across platform, OS, codec, source, and node type.

### 4.1 AudioFrame — Pool-Backed

```rust
/// Pool-owned memory handle. No heap allocation after pool init.
pub struct AudioBufferHandle {
    pool:  Arc<AudioBufferPool>,
    index: u32,
    len:   u32,
}

impl AudioBufferHandle {
    pub fn as_slice(&self) -> &[f32] {
        self.pool.slot(self.index, self.len)
    }
}

/// Drop contract — load-bearing part of the design. Must remain forever:
///   - wait-free (one atomic op against free_mask)
///   - panic-free (no .expect, no .unwrap, no array OOB)
///   - allocation-free (no Vec, String, Box, format!, etc.)
///   - non-logging (no log::*, no tracing::*, no eprintln!)
/// Debug builds assert no double-release (free_mask bit must be 0 before set).
impl Drop for AudioBufferHandle {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        debug_assert!(self.pool.is_in_use(self.index),
            "double-release of slot {}", self.index);
        self.pool.release(self.index); // single atomic fetch_or
    }
}

pub struct AudioFrame {
    pub stream_id:       StreamId,
    pub source_id:       SourceId,       // graph node identity
    pub bus_id:          Option<BusId>,  // semantic bus identity (v3.0)
    pub sample_rate:     u32,            // 48_000 internally (DOCS-013)
    pub channels:        u8,             // 1 voice / 2 music (DOCS-013)
    pub format:          SampleFormat,   // F32LE interleaved (DOCS-013)
    pub timestamp_ns:    u64,            // monotonic, per-node, never wall clock
    pub sequence_number: u64,            // monotonic per stream
    pub buffer:          AudioBufferHandle,
}
```

```rust
/// Phase 0 pool — 64-slot ceiling imposed by AtomicU64 bitset.
/// At 20ms frame duration (DOCS-012): 64 × 20ms = 1.28s of headroom.
pub struct AudioBufferPool {
    slots:     Box<[f32]>,    // contiguous block, allocated once at session start
    slot_size: usize,         // samples per slot — 960 at 48kHz/20ms (DOCS-012)
    free_mask: AtomicU64,     // bitset of free slots; 64-slot cap
}

impl AudioBufferPool {
    /// Wait-free. Returns None on overrun. Backpressure: see DOCS-004.
    pub fn acquire(&self) -> Option<AudioBufferHandle> { /* ... */ }
}
```

### 4.2 Internal Sample Format and Channel Layout (DOCS-013)

**Internal sample format:** interleaved f32, little-endian, normalized [-1.0, 1.0].
**Internal sample rate:** 48000 Hz. Resample at adapter boundary if platform delivers otherwise.

```
AudioMode::Voice      → 1 channel (mono), 48kHz
AudioMode::Music      → 2 channels (stereo), 48kHz
AudioMode::Broadcast  → configurable, default stereo
```

`AudioGraphNode::accepted_channels()` declares what each node can ingest. Graph builder checks the chain and inserts adapter nodes at the right positions. All adapter insertions are zero-allocation passes using the existing pool.

### 4.3 Opus Frame Duration (DOCS-012)

Frame duration cascades into pool slot size, packet rate, jitter buffer sizing, CPU per second, bitrate overhead, and perceived latency. This is explicit and must not be left implicit.

```
20ms (default)     50 pkt/sec · standard · balanced overhead/latency
10ms (voice-agent) 100 pkt/sec · lowest latency · higher CPU
40/60ms            16-25 pkt/sec · broadcast-only, VAD off
```

Recommended: 20ms default. 10ms enabled for voice-agent mode after Phase 1 latency benchmarks confirm CPU/overhead tradeoff is acceptable.

Knock-on effects:
- Pool slot size: 20ms × 48kHz = 960 samples; 10ms = 480; 60ms = 2880
- Ring buffer headroom: 8 frames × 20ms = 160ms; with 10ms frames, ring may need growth

### 4.4 Hot-Path Rules — Enforced, Not Aspirational

These rules apply to the execution layer beneath every graph edge.

```
No heap allocation on the hot path (verified by DHAT gate in CI)
No locks (SPSC ring buffer + atomic pool bitset)
No blocking (audio callback returns immediately)
No logging on the hot path (metrics are atomic counters)
No async/.await in the audio callback
No ObjC/Swift method calls on callback thread (iOS)
No JNI calls per audio frame (Android)
No Rust panic across any FFI boundary
No ML inference on the callback thread (ML nodes run on processing thread)
```

### 4.5 ProcessorGraph

```rust
pub struct ProcessorGraph {
    nodes: Vec<Box<dyn AudioProcessorNode>>,
}

pub trait AudioProcessorNode: Send {
    /// Process one frame. Some passes downstream, None gates.
    /// Must be allocation-free and wait-free.
    /// Runs on the Rust processing thread, never inside platform audio callbacks.
    fn process(&mut self, frame: AudioFrame) -> Option<AudioFrame>;

    /// Channel layout this node accepts. Graph auto-inserts MonoMixNode
    /// or appropriate adapter when upstream layout differs (DOCS-013).
    fn accepted_channels(&self) -> ChannelLayout {
        ChannelLayout::Either
    }
}
```

Built-in Phase 0: `PassthroughNode`, `GainNode`, `ResampleNode`, `MonoMixNode`.
Optional Phase 4+, feature flags: `VadNode`, `NoiseSuppressorNode`, `EchoCancelNode`, `SpeakerEmbedNode`.

---

## 5. Source Capability Model

The system asks "which source capabilities are available right now on this platform?" — not "can we capture everything?" This is the foundation of `SourceNode` adapter discovery.

### 5.1 SourceCapability

```rust
/// Each variant maps 1:1 to a real platform mechanism.
/// No ambiguous cross-platform names.
#[derive(Debug, Clone, PartialEq)]
pub enum SourceCapability {
    Microphone,
    OwnAppAudio,
    DesktopSystemLoopback,       // Windows WASAPI, macOS SCKit, Linux PipeWire
    EligibleAppPlayback,         // Android AudioPlaybackCapture (policy-gated)
    ScreenProjectionMix,         // Android MediaProjection
    PluginHostAudio,             // iOS AUv3
    BroadcastExtensionAudio,     // iOS ReplayKit
    ExternalRouteInput,          // iOS AirPlay-style receiver, experimental
    VirtualDeviceInput,          // Windows SysVAD, macOS AudioDriverKit
    NetworkStreamInput,
    FileOrBuffer,
    HardwareInput,
}

pub struct AudioSourceDescriptor {
    pub id:                   SourceId,
    pub name:                 String,
    pub platform:             PlatformId,
    pub capability:           SourceCapability,
    pub latency_class:        LatencyClass,
    pub reliability_class:    ReliabilityClass,
    pub requires_user_action: bool,
    pub available_now:        bool,
    pub policy_notes:         Option<String>,
}
```

### 5.2 Platform Source Availability

**iOS:**

```json
[
  {"capability": "Microphone",              "available_now": true,  "reliability": "UserPermission"},
  {"capability": "OwnAppAudio",             "available_now": true,  "reliability": "AlwaysAvailable"},
  {"capability": "PluginHostAudio",         "available_now": true,  "reliability": "UserAction"},
  {"capability": "BroadcastExtensionAudio", "available_now": true,  "reliability": "UserAction"},
  {"capability": "ExternalRouteInput",      "available_now": false, "reliability": "Experimental"}
]
```

iOS has no `DesktopSystemLoopback`. If Apple ships a future system-level capture API it becomes a new enum variant.

**Android:**

```json
[
  {"capability": "Microphone",          "available_now": true, "reliability": "UserPermission"},
  {"capability": "OwnAppAudio",         "available_now": true, "reliability": "AlwaysAvailable"},
  {"capability": "EligibleAppPlayback", "available_now": true, "reliability": "PolicyGated",
   "policy_notes": "Android 10+ AudioPlaybackCapture. Capturable audio limited by per-app capture policy. Most restrictive policy wins."},
  {"capability": "ScreenProjectionMix", "available_now": true, "reliability": "UserAction"}
]
```

**Desktop (macOS / Windows / Linux):**

```json
[
  {"capability": "Microphone",            "available_now": true,  "reliability": "UserPermission"},
  {"capability": "OwnAppAudio",           "available_now": true,  "reliability": "AlwaysAvailable"},
  {"capability": "DesktopSystemLoopback", "available_now": true,  "reliability": "AlwaysAvailable",
   "policy_notes": "Windows WASAPI loopback / macOS screencapturekit / Linux PipeWire native"},
  {"capability": "HardwareInput",         "available_now": true,  "reliability": "UserAction"},
  {"capability": "VirtualDeviceInput",    "available_now": false, "reliability": "FutureAPI"}
]
```

### 5.3 Platform Adapter Trait

```rust
pub trait PlatformAdapter: Send + Sync {
    fn platform(&self) -> PlatformId;
    fn source_capabilities(&self) -> Vec<AudioSourceDescriptor>;
    fn output_capabilities(&self) -> Vec<AudioOutputDescriptor>;
    fn open_source(&self, request: SourceRequest)
        -> Result<Box<dyn AudioSourceStream>, AdapterError>;
    fn open_output(&self, request: OutputRequest)
        -> Result<Box<dyn AudioOutputSink>, AdapterError>;
}
```

---

## 6. Architecture Decision Records

v2.3 ADRs (DOCS-001 through DOCS-013) are fully preserved. v3.0 adds ADR-014 through ADR-022. All open questions are tracked in §26.

### DOCS-001: FFI/JNI Boundary Ownership

**iOS: Platform owns the audio callback thread.**

```
AVAudioEngine installTap fires (Apple realtime thread, priority 47)
  ↓
Swift writes f32 samples into AudioBufferPool slot
  ↓  one memcpy of f32 data — unavoidable, accepted
Swift writes AudioFrame header + buffer handle into SPSC ring
  ↓
Rust reads ring on its own processing thread
  ↓
ProcessorGraph → Encoder → TransportNode
```

Rules:
- Swift callback never allocates, never blocks, never calls into Rust synchronously
- Pool slot acquisition happens before the callback (pre-allocated)
- Ring write is wait-free (SPSC, single atomic store as commit)
- Ring sized for 8 frames (160ms headroom at 20ms frames)
- Rust never dereferences `AVAudioPCMBuffer` memory

**Android: Rust owns the audio thread for mic capture.**

```rust
let stream = AudioStreamBuilder::new()
    .input()
    .callback(Box::new(PocketStationCallback { bus: bus.clone() }))
    .sample_rate(48000)
    .format(AudioFormat::F32)
    .performance_mode(PerformanceMode::LowLatency)
    .open_stream()?;
```

For `EligibleAppPlayback`: Kotlin writes to a pre-allocated `ByteBuffer.allocateDirect()`. Rust reads from the raw pointer. JNI is called once at session init to pass the pointer, never per frame.

Lifetime contract: ByteBuffer lives for session duration. Rust reads only within `on_capture_ready()`. Kotlin signals teardown via `AtomicBoolean`; Rust acknowledges before Kotlin frees.

**Desktop: CPAL. Rust owns the callback. Zero FFI.**

```rust
device.build_input_stream(&config, move |data: &[f32], info| {
    let handle = pool.acquire().expect("pool exhausted");
    handle.copy_from_slice(data);
    let frame = AudioFrame::new(handle, info.timestamp_ns(), source_id, bus_id);
    let _ = bus.push(frame);
}, |err| tracing::error!("{err}"), None)
```

### DOCS-002: Star Topology — No Relay Chains

All audio flows `source → cloud relay → listeners`. No device-to-device chains. Relay chains stack latency, require intermediate decode/encode, and provide no capability that star topology does not. WebRTC ICE handles LAN-direct paths automatically.

### DOCS-003: Custom Go Relay — Graph-Aware, Not LiveKit

Custom relay using Pion v4. The relay control plane speaks graph language (GraphSession, source_id, bus_id, route_table) while WebRTC/Pion handles the media plane internally. LiveKit's architecture is built for video conferencing; its subscription model is not designed for semantic audio buses or per-source routing policies.

**Pion version policy:** pin to `github.com/pion/webrtc/v4`. Track v5 release notes; upgrade only after Phase 3 SDK packaging is stable.

### DOCS-004: Backpressure Policy on Pool/Ring Exhaustion

When `AudioBufferPool::acquire()` returns `None` or the SPSC ring is full:

```
A. Drop newest — stable latency, source-side audio loss visible
B. Drop oldest — keeps fresh audio, encoder glitch
C. Block producer — violates no-blocking rule, non-starter
```

**Decision:** A. Drop newest. Documented as DOCS-004.

### DOCS-005: Relay Forward-Loop Locking

Phase 1 uses `sync.RWMutex` per packet. Bottlenecks above ~200 listeners/room. Phase 2 switches to copy-on-write atomic pointer:

```go
type RelaySession struct {
    buses atomic.Pointer[map[BusID]*AudioBus]
}
```

### DOCS-006: Clock Sync / Async Sample Rate Conversion

```
A. Fixed-rate + drop/duplicate — voice-acceptable with VAD
B. PI-controlled linear interpolation — ~100 lines, voice default
C. Variable-rate SRC (libsoxr / rubato) — music quality, ~5x CPU
```

**Decision:** B for voice, hook for C in music/broadcast mode.

### DOCS-007: Capability Negotiation on Partial Match

```
A. Fail with CapabilityMismatch
B. Auto-insert ResampleNode + MonoMixNode
C. Return descriptor delta, caller decides
```

**Decision:** B with explicit `negotiated: NegotiatedCapability` on the stream.

### DOCS-008: Workspace Release Sequencing

```
pocketstation-frame    (no deps)
pocketstation-bus      (deps: frame)
pocketstation-graph    (deps: frame)
pocketstation-codec    (deps: frame, bus)
pocketstation-route    (deps: frame)
pocketstation-metrics  (deps: frame, bus)
pocketstation-audio    (re-export, deps: all above)
```

Tooling: `cargo-release --workspace` with sequenced publish. Git tag: single `v0.X.Y` at workspace root. First publish: after Phase 1 demo validates the API surface, not at Phase 0 exit.

### DOCS-009: Pion WriteRTP Allocation Profile

Before Phase 1 ships:

1. Does `TrackLocalStaticRTP.WriteRTP` mutate `pkt` (header, SSRC, sequence)?
2. If yes, need `pkt.Clone()` per bus subscriber?
3. GC pressure at 50 pkt/sec × N subscribers?
4. Use shared payload with per-subscriber header rewriting (SFU pattern)?

### DOCS-010: JitterBuffer Algorithm

Owns 60ms of the 170ms transport-P95 budget.

```
A. Fixed-delay buffer — simple; poor for two-way voice
B. Adaptive (NetEQ-class) — WebRTC standard; ~500 lines
C. RTT-variance-driven with PLC — wraps webrtc-audio-processing
```

**Decision:** B for Phase 1, optional upgrade to C in Phase 4.

### DOCS-011: SPSC Ring Buffer Crate Choice

**Default candidate:** `rtrb` — fixed-capacity allocation at construction, wait-free reads/writes, cache-line padding, maintained. Verify against criteria in Phase 0 prototype.

### DOCS-012: Opus Frame Duration

See §4.3. This ADR is resolved: 20ms default, 10ms optional for voice-agent mode post-Phase-1 benchmarks.

### DOCS-013: Internal Sample Format and Channel Layout

See §4.2. This ADR is resolved: interleaved f32, 48kHz, mode-dependent channel count.

---

## 7. Platform Adapter Specifications

### 7.1 iOS Adapter — SourceNode Adapter Priority Order

```
Priority 1: AVAudioEngine own-app (OwnAppAudio)
  Any audio PocketStation plays is captured via installTap.
  Zero restriction, always works, no user action.
  Reliability: AlwaysAvailable

Priority 2: Microphone
  Direct mic capture via AVAudioEngine input node.
  User permission required.
  Reliability: UserPermission

Priority 3: AUv3 effect plugin (PluginHostAudio)
  PocketStation loads as an audio effect in any AUv3 host.
  GarageBand, Logic iPad, Cubasis, AUM, 580+ compatible apps.
  Reliability: UserAction

Priority 4: ReplayKit broadcast extension (BroadcastExtensionAudio)
  User-approved screen + app audio broadcast.
  IPC via App Group container to main app → graph source.
  Reliability: UserAction

Priority 5: AirPlay-style receiver (ExternalRouteInput)
  PocketStation appears as audio output destination.
  Not a normal App Store SDK path.
  Reliability: Experimental
```

iOS does not have silent global capture of all apps' audio. PocketStation does not claim it.

`.voiceChat` enables Apple's built-in AEC but restricts AirPlay routing. Use iOS native AEC only in `AudioMode::Voice`; use `webrtc-audio-processing` elsewhere.

### 7.2 Android Adapter — SourceNode Adapter Priority Order

```
Priority 1: Microphone + own-app audio (AAudio/oboe, Rust-owned thread)
Priority 2: EligibleAppPlayback (Android 10+ AudioPlaybackCapture, policy-gated)
Priority 3: ScreenProjectionMix (MediaProjection screen + audio, user grants per session)
Priority 4: Own-app playback capture (always works, no permission)
```

`EligibleAppPlayback` is meaningfully stronger than iOS equivalents because AudioPlaybackCapture is an official API. Source apps can opt out (`ALLOW_CAPTURE_BY_NONE`), and audio with `USAGE_VOICE_COMMUNICATION` is not capturable.

### 7.3 Desktop Adapter

CPAL handles device I/O. Loopback capture of other apps and virtual device creation require native APIs per platform.

```
Windows:
  Phase 1: WASAPI loopback via windows-rs (DesktopSystemLoopback)
  Phase 2: CPAL for own-device I/O
  Phase 6: SysVAD virtual speaker driver (VirtualDeviceInput, C++/WDK)

macOS:
  Phase 1: screencapturekit-rs (DesktopSystemLoopback, ~1.9% CPU on Apple Silicon)
  Phase 2: CPAL CoreAudio for own-device I/O
  Phase 6: AudioDriverKit virtual device (C++/DriverKit)

Linux:
  Phase 1: PipeWire native via pipewire-rs (DesktopSystemLoopback + graph nodes)
  Phase 2: CPAL for fallback ALSA/PulseAudio on non-PipeWire systems
```

Linux PipeWire is the deepest desktop platform — it exposes the full audio graph natively. Build and validate the SourceNode adapter model on Linux first.

---

## 8. Relay Architecture — GraphSession

### 8.1 Two Product Modes

**Mode A — Broadcast**
```
One source → relay → N listeners
Star topology, one-to-many
Source publishes a named bus (e.g. monitor_mix)
Listeners subscribe to that bus by bus_id
No decode, no re-encode at relay (Phase 1)
```

**Mode B — Voice Agent**
```
Client graph → relay → AI backend (bidirectional)
Multiple buses: uplink voice, downlink agent speech, events
Relay handles ICE, TURN, DTLS for all edges
GraphSession metadata: session_id, graph_id, source_ids, latency budget
Webhook events: session_started, utterance_detected, model_response, session_ended
```

### 8.2 GraphSession Replaces Room

Old relay model:

```go
type Room struct {
    id        string
    source    *webrtc.TrackRemote
    listeners []*webrtc.TrackLocalStaticRTP
}
```

New relay model:

```go
type RelaySession struct {
    ID          string
    GraphID     string
    Sources     map[SourceID]*SourceSession
    Buses       map[BusID]*AudioBus
    Subscribers map[SubscriberID]*BusSubscription
    Routes      atomic.Pointer[RouteTable]
    Policies    []PolicyBinding
    Metrics     *RelaySessionMetrics
}

type AudioBus struct {
    ID          string
    SourceID    string
    Role        BusRole   // voice, music, monitor, mix, stem, agent_output, events
    Codec       Codec
    Clock       ClockDomain
    Subscribers atomic.Pointer[[]Subscriber]
    Metrics     BusMetrics
}
```

The relay still uses Pion v4 WebRTC internally for the media plane. The control plane speaks graph semantics. Transport can still forward raw RTP without decoding.

### 8.3 Core Relay — Phase 1 MVP (~1200–1500 lines total)

```go
func (r *RelaySession) forwardBus(bus *AudioBus) {
    for {
        pkt, _, err := bus.Source.ReadRTP()
        if err != nil { return }
        bus.Metrics.PacketCount.Add(1)
        bus.Metrics.ByteCount.Add(uint64(len(pkt.Payload)))
        subs := bus.Subscribers.Load()
        for _, s := range *subs {
            _ = s.Track.WriteRTP(pkt) // see DOCS-009 for mutation/clone ADR
        }
    }
}
```

**Phase 1 relay includes:** RTP forwarding by bus_id, GraphSession lifecycle, JWT auth, QR/room codes, TURN/STUN config, SSE presence, source identity propagation, per-bus metrics.

**Production relay grows to (~3000–5000 lines):** reconnect logic, regional routing, load balancing, per-bus SLO enforcement, webhook events, recording trigger, live route-table updates, multi-source mixing, graceful shutdown with session migration.

### 8.4 Signaling Protocol

```
Client → Server (WebSocket JSON):
  PUBLISH:   graph_id, session_id, bus_id, token, SDP offer
  SUBSCRIBE: session_id, bus_id, SDP offer
  ICE:       candidate
  ROUTE:     live route-table update
  LEAVE:     session_id

Server → Client:
  SDP_ANSWER:   SDP answer
  ICE:          candidate
  SESSION_STATE: sources[], buses[], routes[], metrics
  BUS_EVENT:    type, bus_id, source_id, payload
  ERROR:        code, message
```

### 8.5 Control Plane API

```
POST   /v1/graphs                        Create graph session → {graph_id, session_id}
GET    /v1/sessions/{id}                 Session state → {sources, buses, routes, metrics}
DELETE /v1/sessions/{id}                 Close session
POST   /v1/sessions/{id}/sources         Register source → {source_id, token}
POST   /v1/sessions/{id}/buses           Register bus → {bus_id}
POST   /v1/sessions/{id}/routes          Update route table
POST   /v1/sessions/{id}/subscribe       Get listener token + ICE config
GET    /v1/sessions/{id}/events          SSE event stream
GET    /v1/sessions/{id}/metrics         Per-bus latency, loss, jitter
GET    /v1/apps/{id}/usage               Graph-session minutes this billing period
```

---

## 9. Security Model

### Phase 1 — Transport Security

```
WebRTC DTLS/SRTP: transport encrypted between client and relay
Relay can read Opus payloads in Phase 1 — accepted and disclosed
Session access controlled by JWT tokens (short-lived, session-scoped)
No public session listing — sessions are ephemeral by default
HTTPS everywhere for control plane
```

Say: "Encrypted transport. Session access requires a token."
Do not say: "End-to-end encrypted." Not true until Phase 3.

### Phase 2 — Access Controls

```
Short-lived source tokens (15-minute expiry, renewable)
Subscriber tokens with max-subscriber-count enforcement
Session expiry (auto-close after N hours of inactivity)
Abuse rate limiting (max sessions per IP, max subscribers per session)
Source privacy classes: Public / UserConscented / EnterprisePrivate / PrivacyRedacted
```

### Phase 3 — True E2EE (SFrame, RFC 9605)

SFrame defines frame-level encryption where the relay forwards media with metadata visible but payload encrypted.

```rust
pub enum EncryptionMode {
    TransportOnly,        // Phase 1
    SFrameE2EE,           // Phase 3: relay is routing-blind to audio
    EnterpriseKeyManager, // Phase 5
}
```

### Phase 5 — Enterprise

```
Private relay deployment (customer VPC)
Audit logs
Source privacy enforcement (block source from reaching cloud model)
Model allowlist/denylist per session
Data retention controls
HIPAA BAA (when operationally ready)
SOC 2 Type II (12–18 month process)
```

---

## 10. Model Nodes and ML Processing Layer

All ML runs as graph nodes. All models run on-device by default. Raw audio is never sent to external ML APIs unless the developer explicitly routes to a model node.

**Threading rule (load-bearing):** Model nodes run on the Rust processing thread, never inside platform audio callbacks. VAD inference is often fast enough that the distinction seems academic — until denoise or AEC pushes 30ms inference into the callback path and the audio system glitches. The boundary is enforced architecturally: callbacks write to the SPSC ring and return; the processing thread drains the ring and runs all `AudioProcessorNode::process()` calls including model inference.

### 10.1 VAD

Silero VAD — MIT licensed, 1.8MB ONNX, <1ms inference on CPU.

```rust
pub struct VadNode {
    model:     OrtSession,
    state:     VadState,
    threshold: f32,
}

impl AudioProcessorNode for VadNode {
    fn process(&mut self, frame: AudioFrame) -> Option<AudioFrame> {
        let prob = self.model.infer(frame.buffer.as_slice());
        self.state = VadState::from_prob(prob, self.threshold);
        match self.state {
            VadState::Speech | VadState::Onset => Some(frame),
            VadState::Silence | VadState::Offset => None,
        }
    }

    fn accepted_channels(&self) -> ChannelLayout {
        ChannelLayout::MonoOnly
    }
}
```

VAD gating saves 40–60% relay bandwidth in typical voice sessions. For voice agents, VAD drives end-of-utterance detection without external API calls.

### 10.2 Noise Suppression

DTLN-rs — open source Rust, WASM-compatible, processes 1s audio in 33ms on M1. Accepts mono.

### 10.3 Echo Cancellation

`webrtc-audio-processing` (libwebrtc AEC3) or iOS native AEC via `.voiceChat` mode. See §7.1 for iOS AEC routing constraint.

### 10.4 Bandwidth-Adaptive Codec Control

Relay measures per-bus RTCP RR. Source adjusts Opus settings:

```
Loss < 1%, RTT < 100ms:    bitrate=96kbps, complexity=10, fec=false
Loss 1-5%, RTT < 200ms:    bitrate=64kbps, complexity=5,  fec=true
Loss > 5%, any RTT:        bitrate=32kbps, complexity=3,  fec=true, dtx=true
Loss > 15%:                trigger ICE restart, fallback TURN relay
```

### 10.5 Future Model Nodes

```
SpeakerDiarizationNode     TitaNet/Sortformer
EmotionCueNode             distress, stress, sarcasm detection
SourceSeparationNode       overlapping speaker isolation
AudioEnhancementNode       bandwidth extension
RealtimeTranslationNode    Whisper + MT (500ms+, broadcast only)
AudioWatermarkNode         EU AI Act compliance
LocalWhisperNode           on-device STT fallback
```

---

## 11. Observability

Observability ships with Phase 0. Not Phase 5.

### 11.1 Per-Edge Metrics

```rust
pub struct EdgeMetrics {
    pub frames_in:            Counter,
    pub frames_out:           Counter,
    pub frames_dropped:       Counter,
    pub queue_depth:          Gauge,
    pub p50_latency_ms:       Histogram,
    pub p95_latency_ms:       Histogram,
    pub p99_latency_ms:       Histogram,
    pub jitter_ms:            Histogram,
    pub drift_ppm:            Gauge,
    pub clipping_events:      Counter,
    pub rms_dbfs:             Gauge,
    pub loudness_lufs:        Gauge,
    pub packet_loss_pct:      Gauge,
    pub model_first_token_ms: Option<Histogram>,
    pub model_first_audio_ms: Option<Histogram>,
    pub model_cost_usd:       Option<Counter>,
}
```

Developer API:

```rust
graph.observe(edge_id)
    .on_latency_p95(|ms| warn!("edge {edge_id} latency p95 = {ms}"))
    .on_drop(|count| warn!("edge {edge_id} dropped {count} frames"))
    .on_model_cost(|usd| if usd > budget { graph.trigger(PolicyNode::CostCap) });
```

### 11.2 Per-Session Bus Metrics

```go
type BusMetrics struct {
    PacketsForwarded    prometheus.CounterVec
    BytesForwarded      prometheus.CounterVec
    SubscriberCount     prometheus.GaugeVec
    ForwardLatencyNs    prometheus.HistogramVec
    SubscriberLossRate  prometheus.GaugeVec
    SessionDurationSec  prometheus.HistogramVec
    ModelLatencyMs      prometheus.HistogramVec
    ModelCostUsd        prometheus.CounterVec
}
```

### 11.3 Latency Budget — Transport-Only Targets

PocketStation owns the **transport segment**: capture → encode → relay → decode → playback. It does not own STT, LLM, or TTS latency.

```
Source capture → FrameBus:           ≤ 5ms
FrameBus → Opus encoder:             ≤ 2ms
Opus → WebRTC send:                  ≤ 1ms
WebRTC → relay (network, P95):       ≤ 50ms
Relay → subscriber (network, P95):   ≤ 50ms
Subscriber WebRTC → JitterBuffer:    ≤ 2ms
JitterBuffer → output (adaptive):    ≤ 60ms

Target transport P95:                ≤ 170ms
Target transport P99:                ≤ 250ms

Voice agent transport-segment P95:   ≤ 185ms
  Leaves remaining budget for STT/LLM/TTS to land the full
  conversational loop near the ≤500–700ms perceptual ceiling.
  PocketStation does not promise the full loop — only its segment.
```

### 11.4 Developer-Facing Latency Breakdown

```json
{
  "session_id": "...",
  "graph_id": "...",
  "edge_id": "mic:voice→relay:uplink",
  "source_id": "...",
  "bus_id": "clean_mic",
  "capture_ms": 3.2,
  "encode_ms": 1.1,
  "relay_rtt_ms": 44.0,
  "jitter_buffer_ms": 55.0,
  "decode_ms": 0.8,
  "transport_e2e_ms": 104.1,
  "model_first_token_ms": 210.0,
  "model_first_audio_ms": 380.0,
  "packet_loss_pct": 0.3,
  "clock_drift_ppm": 12
}
```

### 11.5 SLI / SLO Definitions

```
SLI: Session completion
  Sessions where both source and last subscriber disconnect cleanly,
  or session duration ≥ 30 minutes with no fatal media-plane error.
  SLO target: 99.9%

SLI: Transport latency
  capture_ms + encode_ms + relay_rtt_ms + jitter_buffer_ms + decode_ms
  Measurement: per-session P95 from subscriber client
  SLO target: 95% of sessions ≤ 250ms

SLI: Source publish success
  Source token validated → WebRTC negotiated → first RTP packet forwarded
  SLO target: 99.5% within 3 seconds of token presentation
```

### 11.6 CLI Observability

```bash
pks sources                            # list available source nodes
pks session inspect {session_id}         # graph topology + node states
pks session edges {session_id}           # all edges + latency per edge
pks session trace {session_id} \
  --edge mic:voice→openai:audio        # per-edge packet trace
pks session record {session_id} \
  --stems mic,discord,spotify,agent    # start multi-stem recording
```

---

## 12. Full Tech Stack

| Layer | Language / Library | Rationale |
|---|---|---|
| Audio graph runtime | **Rust** | Type-safe node contracts, zero-cost graph execution |
| Audio engine core | **Rust** | Memory safety, zero-cost abstractions, lock-free |
| iOS adapter shell | **Swift** | Required for AVAudioEngine, AUv3, AVAudioSession |
| Android adapter shell | **Kotlin** | Required for AudioPlaybackCapture, MediaProjection, AAudio JNI |
| Desktop device I/O | **Rust (CPAL)** | Fallback path for standard device I/O |
| Windows system loopback | **Rust (windows-rs WASAPI)** | CPAL does not expose loopback |
| macOS system loopback | **Rust (screencapturekit-rs)** | Native CoreMedia capture, ~1.9% CPU |
| Linux audio graph | **Rust (pipewire-rs)** | Graph-level access, virtual nodes |
| Windows virtual driver | **C++/WDK** | OS forced |
| macOS AudioDriverKit | **C++** | DriverKit requires C++ |
| Cloud relay | **Go + Pion v4** | Current stable; goroutines for N subscribers |
| Control plane API | **Go** | Shared codebase with relay |
| Protocol (Phase 2+) | **Protobuf** | Graph/session/bus/source wire types |
| Web receiver | **TypeScript + WebRTC** | Browser-native; no framework for a subscriber |
| ML inference nodes | **Rust + ONNX Runtime** | WASM-compatible |
| Python SDK | **PyO3 bindings** | Voice AI developers write Python |
| Research tooling | **Python** | Latency measurement, DSP experiments |

**Never:** Python in any production audio path or relay. Never Python for the relay.

---

## 13. Developer API

### 13.1 The AudioGraph API (Primary)

See §3.7 for the full holy-shit demo. The core wiring pattern:

```rust
let graph = AudioGraph::new();

// Source discovery
let sources = graph.discover_sources().await?;

// Build graph
let mic   = graph.source(SourceNode::Mic);
let relay = graph.transport(TransportNode::Relay("session-abc"));
let sink  = graph.sink(SinkNode::Browser);

graph.connect(mic.out("voice"), relay.in_("voice"))?;
graph.connect(relay.out("mix"), sink.in_("audio"))?;

// Compile validates types, inserts adapters, estimates latency/cost
let plan = graph.compile()?;

// Observe edges before running
graph.observe(plan.edge("mic:voice→relay:voice"))
    .on_latency_p95(|ms| tracing::warn!("high latency: {ms}ms"));

graph.run(plan).await?;
```

### 13.2 Convenience API (Sugar — Compiles to Graph)

```rust
// Simple broadcast: compiles to SourceNode → Encode → TransportNode::Relay → Browser
let station = PocketStation::builder()
    .relay_url("wss://relay.pocketstation.io")
    .session_id("abc123")
    .mode(AudioMode::Voice)
    .opus_frame_duration_ms(20)
    .add_processor(VadNode::default())
    .add_processor(NoiseSuppressorNode::default())
    .on_subscriber_count(|n| println!("{n} listening"))
    .build()
    .await?;
let source = station.open_best_source(SourcePreference::Voice).await?;
station.start_broadcast(source).await?;
```

### 13.3 Platform SDKs

```swift
// iOS
let station = try await PocketStation(sessionID: "abc123", role: .source)
station.onSubscriberCount = { n in label.text = "\(n) listening" }
try await station.start()
```

```kotlin
// Android
val station = PocketStation.builder(context)
    .sessionId("abc123")
    .role(Role.SOURCE)
    .build()
lifecycleScope.launch {
    station.subscriberCount.collect { label.text = "$it listening" }
}
station.start()
```

```python
# Python (voice AI developers)
import asyncio
from pocketstation import AudioGraph, SourceNode, ModelNode, SinkNode

async def main():
    graph = AudioGraph()
    mic   = graph.source(SourceNode.Mic)
    stt   = graph.model(ModelNode.Transcribe("deepgram"))
    agent = graph.model(ModelNode.SpeechToSpeech("openai-realtime"))
    out   = graph.sink(SinkNode.Browser)

    graph.connect(mic.out("voice"), stt.in_("audio"))
    graph.connect(stt.out("transcript"), agent.in_("context"))
    graph.connect(agent.out("audio"), out.in_("audio"))

    await graph.run()

asyncio.run(main())
```

---

## 14. Repository Structure

### 14.1 Why Separate Repos

One independently releasable unit = one repository. An iOS developer adding the Swift SDK should not download the Go relay, Windows WDK driver, Python bindings, and research notebooks. The iOS Swift package cannot be published to Swift Package Index from inside a Rust workspace root.

Exception: Cargo workspace where internal crates are tightly coupled, share types, and always release together.

### 14.2 GitHub Organization

```
github.com/pocketstation-io/       core infrastructure, SDKs, services, apps
github.com/pocketstation-examples/ standalone examples, one per use case
```

### 14.3 The Full Repo Map

#### Tier 0 — Core Rust (Cargo workspace)

```
pocketstation-io/pocketstation
  crates/
    pocketstation-frame/     AudioFrame, AudioBufferPool, SampleFormat
    pocketstation-bus/       FrameBus, SpscRingBuffer, ClockSync
    pocketstation-graph/     AudioGraphNode trait, ProcessorGraph, EdgeMetrics
    pocketstation-codec/     OpusEncoder, OpusDecoder, JitterBuffer
    pocketstation-route/     SourceCapability, SourceIdentity, RoutePlan
    pocketstation-metrics/   BusMetrics, OTEL integration
    pocketstation-audio/     re-exports all above as single entry point
  benches/
  tests/
  crates/pks-codec-c/        retained Opus C ABI + checked `pks_codec.h`
```

#### Tier 1 — Graph Runtime (now crates inside the `pocketstation` workspace)

```
pocketstation-io/audio-graph
  crates/
    pocketstation-graph-api/    AudioGraph, AudioGraphNode, PortSpec, EdgeSpec
    pocketstation-graph-nodes/  SourceNode, TransformNode, PolicyNode, ModelNode, TransportNode, SinkNode
    pocketstation-graph-runtime/ Graph compile, validation, execution planner
    pocketstation-graph-observe/ EdgeMetrics, GraphMetrics, tracing integration
  examples/
```

#### Tier 2 — Protocol (created Phase 2)

```
pocketstation-io/protocol
  Language:   Protobuf + generated Go, Rust, TypeScript, Swift, Kotlin
  Contains:   GraphSession, SourceIdentity, BusDescriptor, EdgeDescriptor,
              signaling messages, control plane types, metric schemas
  Published:  Generated code vendored into each SDK repo
```

#### Tier 3 — Client SDKs

```
pocketstation-io/sdk-ios        Swift Package Index   PocketStation
pocketstation-io/sdk-android    Maven Central         io.pocketstation:android
pocketstation-io/sdk-js         npm                   @pocketstation/client
pocketstation-io/sdk-rust       crates.io             pocketstation-client
pocketstation-io/sdk-python     PyPI                  pocketstation
```

#### Tier 4 — Server Services

```
pocketstation-io/relay          Go + Pion v4 (GraphSession, AudioBus)
pocketstation-io/control-plane     Go (control plane, graph/session/metrics APIs)
```

#### Tier 5 — DSP and legacy model workspaces

Active bounded VAD, denoise, AEC, and watermark primitives are in `pks-dsp`.
The archived external `audio-ml` workspace below is not first-party provider
scope for the current product slice.

```
pocketstation-io/audio-ml
  crates/
    pocketstation-vad/          Silero VAD
    pocketstation-denoise/      DTLN-rs
    pocketstation-aec/          webrtc-audio-processing
  models/                       ONNX model files, git-lfs
```

#### Tier 6 — Connector Examples

Model provider integrations are **not** first-party core repos. There is no
official `pocketstation-io/model-connectors` repo and no first-party `pks-ai-*`
crate family. Connector examples live under examples or community-owned repos.

```
pocketstation/examples/
  meeting_whisper_ollama.rs
  deepgram_agent.rs

github.com/pocketstation-examples/
  connector-whisper
  connector-deepgram
  connector-openai-realtime
  connector-elevenlabs
```

#### Tier 7 — OS Driver Extensions

```
pocketstation-io/driver-windows     C++/WDK
pocketstation-io/driver-macos       C++/DriverKit
```

#### Tier 8 — Applications

```
pocketstation-io/app-creator        React Native (iOS + Android)
pocketstation-io/web-receiver   Static TS, no framework
pocketstation-io/app-desktop        Tauri (Rust + web frontend)
pocketstation-io/pks                pks command
pocketstation-io/docs               docs.pocketstation.io
```

#### Tier 9 — Examples

```
github.com/pocketstation-examples/
  voice-agent-openai
  voice-agent-android
  graph-multi-source
  creator-station-ios
  latency-benchmark
  model-routing-fallback
  desktop-loopback-linux
  enterprise-private-graph
```

### 14.4 Release Strategy

```
pocketstation: SemVer. cargo-release --workspace after product API proof.
sdk-ios:       SemVer aligned with PocketStation major. SPM tag.
sdk-android:   SemVer aligned with PocketStation major. Maven Central.
sdk-python:    SemVer independent. PyPI.
relay:         SemVer independent. Docker image on tag.
protocol:      SemVer independent. Vendored into SDKs.
```

---

## 15. Build Phase Plan

Phases as milestones. No week numbers. The sequence is the contract; the dates are not.

### Phase 0 — Core DNA

Prove the hot path. Prove the graph vocabulary. No OS audio APIs. No UI.

Build:
```
AudioBufferPool (zero per-frame allocation, 64-slot cap documented)
AudioFrame with pool handles, interleaved f32 48kHz (DOCS-013)
SPSC ring buffer (proptest-verified invariants; crate: rtrb)
FrameBus
ProcessorGraph (PassthroughNode, GainNode) with accepted_channels routing
ClockSync (PI-controlled linear interpolation, DOCS-006)
Opus encoder/decoder at 20ms default (DOCS-012)
Fake sine-wave source
File output sink
Full per-frame BusMetrics
Backpressure policy decided (DOCS-004)
AudioGraph API sketch: node types, port names, edge contracts (design only)
SourceIdentity type (v3.0 addition)
BusDescriptor type (v3.0 addition)
GraphManifest JSON draft (v3.0 addition)
```

Exit criteria:
```
sine_wave → ProcessorGraph → Opus encode → decode → WAV file
Zero heap allocation on hot path (DHAT in CI)
Ring buffer handles 10,000 frames/sec under 1-hour soak, zero overruns
BusMetrics P50/P95/P99 print correctly after 60-second run
Old simple route API compiles to a graph template internally
DOCS-004 through DOCS-013 written and merged
ADR-014 through ADR-016 drafted
PocketStation crates are component-tested but not product-proof/publish-ready
```

### Phase 1 — First Graph Route

Turn existing relay + web receiver + CLI capture into the first real graph demo.

Build:
```
Desktop source adapters: macOS (screencapturekit-rs), Linux (PipeWire), Windows (WASAPI)
Go relay MVP with GraphSession, source_id, bus_id, route_table (~1200-1500 lines)
Pion v4 WriteRTP allocation profile resolved (DOCS-009)
JitterBuffer algorithm chosen and benchmarked (DOCS-010)
Cloudflare Workers control plane
QR code generation
WebRTC signaling (types live in relay repo)
Browser receiver: shows source name, bus name, latency metrics
E2E latency measurement dashboard
First crates.io publish of PocketStation runtime crates
CLI: pks sources, pks session create, pks route, pks run
```

Exit:
```
Desktop source → relay → browser on different network (30-minute stable session)
P95 transport latency ≤ 250ms measured
Source identity visible in browser receiver
Latency dashboard shows per-edge breakdown (capture, encode, relay, jitter, decode)
PocketStation runtime crates published on crates.io
CLI works on macOS and Linux
```

### Phase 2 — Graph v0: Multi-Source, Multi-Bus, Policies

Prove PocketStation is not forwarding one track.

Build:
```
Multiple sources in one GraphSession
Named buses and stems
Local mixer node
Remote bus subscription by bus_id
Per-bus recording
Ducking policy node (live)
Simple trigger policy node
Graph manifest save/replay
Android mic source (AAudio/oboe, Rust-owned thread)
Android EligibleAppPlayback (Kotlin + direct ByteBuffer)
JNI bridge finalized (zero per-frame JNI calls)
Relay: copy-on-write subscriber slice (DOCS-005), reconnect, rate limiting, session expiry
SLO instrumentation (§11.5)
sdk-ios first SPM publish
protocol repo created with provisional proto definitions
```

Exit:
```
At least 3 concurrent sources in one session
At least 2 simultaneous bus outputs
At least 1 policy action changes routing or gain live
At least 1 multi-stem recording works
Android source → browser: same performance targets
Relay survives source disconnect + reconnect without losing subscribers
1-hour memory soak: zero RSS growth
```

### Phase 3 — Model Nodes and AI Pipeline Routing

Make the AI-era wedge real.

Build:
```
ModelNode interface
OpenAI Realtime connector
Deepgram STT connector
ElevenLabs TTS connector
Local Whisper connector
ModelRouter policy (latency/cost/privacy decision)
Streaming transcript events in GraphSession
TTS return stream as SourceNode::ModelOutput
Cost/latency metrics per model node
sdk-android first Maven Central publish
SDK API finalized (Rust crate API stable)
Python SDK (PyO3 bindings)
Documentation: 3 quickstart guides, graph API reference
3 demo apps (voice-agent, podcast, remote-monitor)
NLnet grant application submitted
```

Exit:
```
Mic → OpenAI Realtime → agent speech → browser: working end-to-end
Model latency metrics visible per edge
Model output is treated as a SourceNode in the graph
At least one routing policy uses model/transcript event as trigger
External developer follows iOS quickstart in ≤ 2 hours
```

### Phase 4 — Creator Station + Developer SDK Polish

Make it usable by people who are not the author.

Build:
```
iOS app: Choose sources → Graph runs → QR → subscriber count → health meter
Android app: same UX
Web receiver: stem selector, latency display, event stream
Desktop app (Tauri): same session pairing, WASAPI/SCKit source
Graph template system (voice-agent, podcast, meeting-transcription, remote-monitor)
CLI: pks session inspect, pks session trace, pks session record
```

Exit:
```
Non-technical person starts station, friend listens in ≤ 1 minute
Station survives 30-minute session on 4G
At least 10 external users run real sessions
At least 3 design partners give feedback on graph API
```

### Phase 5 — Deep Source Expansion + E2EE

Build:
```
iOS AUv3 effect plugin (PluginHostAudio)
iOS ReplayKit broadcast extension (BroadcastExtensionAudio)
macOS screencapturekit-rs full integration
Linux PipeWire virtual sink + node
SFrame E2EE (Phase 3 security model)
Enterprise: private relay deployment, source privacy classes, model allowlist
Observability: per-edge cost tracking, fallback reason logs
```

Exit:
```
AUv3 loads in 3 major iOS host apps without crash
SFrame E2EE: relay cannot distinguish audio content from silence
Enterprise design partner can run private relay
Graph metrics can diagnose a failed voice agent session
```

### Phase 6 — Virtual Endpoints and Scale

Build:
```
Windows SysVAD virtual speaker driver (C++/WDK)
macOS AudioDriverKit virtual audio device (C++/DriverKit)
Linux PipeWire virtual mic/sink
Any app can select PocketStation virtual mic or speaker
Virtual endpoint maps to a graph bus
Multi-region relay (Fly.io edge: EU/US/APAC)
Horizontal relay scaling
Live route-table updates without session drop
```

### Phase 7 — Ecosystem and Moat

Build:
```
Node plugin SDK (external developers build graph nodes)
Model connector marketplace
Graph template library
Enterprise policy packs
Benchmark suite + certification tests for low-latency nodes
Research publications
```

---

## 16. Market Reality

Three signals:

1. LiveKit raised $100M at $1B valuation building realtime voice/AI infrastructure and reportedly powers ChatGPT Voice Mode. The benchmark and the acquisition signal.

2. Deepgram raised $130M at $1.3B in January 2026. ElevenLabs raised $500M at $11B in February 2026. Voice AI infrastructure is venture-scale and actively funded.

3. A 2026 Salesforce AI Research tutorial on enterprise voice agents found that "realtime" performance in production still depends primarily on streaming and pipelining across cascaded STT → LLM → TTS components, not on any single magic model. Measured P50 time-to-first-audio: 947ms with a well-tuned cascaded pipeline. The battle is pipeline, routing, latency, capture, and control — not only model quality.

Grand View Research estimates: WebRTC at **$8.71B in 2024, 45.7% CAGR through 2030**. Call-center AI at **$1.99B in 2024, projected $7.08B by 2030**. Podcasting at **$30.72B in 2024, projected $131.13B by 2030**.

That is the entire market thesis. PocketStation sits at the intersection of all of them.

---

## 17. Target Markets — Ranked and Honest

### Market 1 — Voice AI Developer Infrastructure (Start Here)

**Who:** Teams building voice agents, AI meeting tools, voice-enabled mobile apps, accessibility tools.

**Pain:** No major AI voice API ships mobile audio capture or a cross-platform audio graph. Developers re-implement ~1,000 lines of AVAudioSession lifecycle per project. Model routing, fallback, and observability are always custom.

**PocketStation removes:** the entire audio I/O and pipeline plumbing that every voice AI team builds themselves.

**Reach:** crates.io, PyPI, npm, Show HN, direct outreach to voice AI startups.

### Market 2 — Creator Broadcast

**Who:** DJs, podcasters, mobile creators, radio-style streamers, live event hosts.

**Pain:** Broadcasting from phone to listeners requires OBS + audio interface + port forwarding + Discord. No one-tap cross-app solution.

**PocketStation gives:** start session → share QR → route any source → listeners hear it → stems recorded.

### Market 3 — Enterprise Private Audio Infrastructure

**Who:** Companies building contact-center AI, meeting intelligence, compliance-sensitive voice workflows.

**Pain:** Need private relay, model provider control, source privacy enforcement, audit logs, SLO dashboards.

**PocketStation gives:** private graph infrastructure with observability, policy, and compliance path.

### Market 4 — Consumer Audio Sharing

**Reality:** Consumer default is Bluetooth, AirPlay, Discord, FaceTime. PocketStation wins only when demonstrably simpler or solving a moment none of those reach. Do not chase. Let it come organically from creator station.

---

## 18. Business Model

### Phase 0–2 — Credibility Before Revenue

```
Open source core, adapters, relay source
Docs, benchmarks, demo videos
Free hosted relay (time-limited during development)
No paywall for first 1,000 developers
```

### Phase 3 — Developer Revenue

```
Free tier:    10,000 participant-minutes/month
Usage-based:  $0.0004/participant-minute (matches LiveKit)
Developer:    $49/month — 100K minutes + graph events + per-edge analytics
```

### Phase 4 — Enterprise Revenue

```
Scale plan:   $299/month + overage — 1M minutes + SLA 99.9%
Enterprise:   Contract pricing — private relay, model allowlist,
              HIPAA BAA (when ready), SOC 2, custom SLA, source license
```

### Pricing Units (v3.0 Expansion)

```
graph-session minutes
participant minutes
model-routing minutes (per model node in graph)
recording stem storage
private relay deployment
observability retention days
enterprise policy packs
```

### North Star Metric

**Weekly Active Sessions (WAS):** unique graph sessions with at least one source and one subscriber exchanging audio in the past 7 days.

```
100 WAS    → NLnet grant
1,000 WAS  → YC viable
10,000 WAS → seed round
100,000 WAS → Series A or acquisition
```

---

## 19. Infrastructure Cost Strategy

### Stack

```
Audio relay origin      Hetzner CX23 (€3.49/mo, 20TB included)
Edge relay nodes        Fly.io (30+ cities, pay per second)
Control plane           Cloudflare Workers (free 100K req/day)
Session state           Cloudflare Durable Objects
TURN relay              Cloudflare Calls ($0.05/GB)
Recording storage       Cloudflare R2 (zero egress)
Metrics                 Grafana Cloud (free tier 10K series)
```

### Bandwidth Math

```
Opus 64kbps stereo = 8KB/s = 28.8MB/hour per subscriber stream
Hetzner CX23 included: 20TB/month → 694,444 subscriber-hours included
```

### Cost by Phase

```
Phase 0–1 (0 users):        ~€5/month
Phase 2–3 (≤100 users):     ~€20/month
Phase 4 (100–1K users):     ~€50/month
Phase 5 (1K–10K users):     ~€130/month
Phase 6 (10K–100K):         ~€430/month
Scale (100K+):               Revenue required; at €430/month cost,
                             $14K MRR covers it with 97% margin
```

---

## 20. Funding Strategy

### Phase 0–2 — No Equity, No VC

**NLnet Foundation NGI Zero Commons Fund** — €5K–€50K, rolling open calls, ~3 months to decision.

Grant pitch:
> PocketStation is open-source realtime audio graph infrastructure for mobile devices, AI voice applications, accessibility tools, and creator broadcast. It eliminates the audio I/O plumbing every voice AI team builds independently.

**Mitacs Accelerate** (if enrolled): $15K CAD / 4-month unit.
**NSERC Discovery** (if enrolled): $20–50K CAD/year.
**GitHub Sponsors:** when crates.io downloads are real.

### Phase 3–5 — After Real Users

**YC** at 1,000+ WAU or 10 developers with SDK in production.

Pitch:
> Open source audio graph runtime every voice AI company building on mobile or desktop needs. LiveKit is the comparable infrastructure — $100M at $1B. We are audio-native, graph-first, Rust-core, with model routing and per-edge observability.

**Sequoia / a16z Seed ($5–10M):** at $50K+ MRR or major AI company as design partner.

---

## 21. Research Path

IEEE, ACM, Elsevier, Springer accept independent researchers. Use "Independent Researcher" affiliation. Most venues are double-blind. Path: implement → measure → arXiv preprint → conference/journal.

```
Paper 1   PocketStation: A Graph Runtime for Realtime Audio Across
          Devices, Apps, and AI Models                            (ACM MM / ICASSP)

Paper 2   Memory-Safe Realtime Audio Graphs: Rust Type System as
          a Callback-Thread Safety Proof                          (USENIX ATC / EuroSys)

Paper 3   Latency Decomposition in Cascaded Voice Agent Pipelines:
          Capture, Transport, STT, LLM, and TTS                  (ACM IMC / Interspeech)

Paper 4   Cross-Platform Mobile Audio Capture for Voice AI:
          Constraints, Capabilities, and a Unified Abstraction   (IEEE SP Magazine / ACM CSUR)

Paper 5   Policy-Based Model Routing for Realtime Audio Agents:
          Cost, Latency, and Quality Tradeoffs                   (ICASSP / IEEE SLT)

Paper 6   Multi-Stem Realtime Recording for AI Voice Agent Debugging:
          A Measurement Study                                     (ACM SIGCOMM / IMC)
```

---

## 22. Competitive Landscape

### LiveKit — The Benchmark

LiveKit ships full realtime platform (video + audio + data), Go/Pion, agents SDK, $100M raised. **It has iOS and Android SDKs and powers OpenAI Voice Mode.**

PocketStation's wedge is not "better generic LiveKit." It is a different shape:

```
Audio-only, leaner, smaller binary and API surface
Capture-first: SourceNode identity and adapter layering
Graph-first: programmable nodes, typed edges, policies, model routing
Observability-first: per-edge latency, cost, and drift as first-class API
Self-hostable relay: readable Go codebase, graph-aware control plane
Audio-only pricing: cleaner economics for audio-only workloads
Rust core: measurable CPU and battery advantage on mobile
```

Pitch: "LiveKit is full-stack realtime. PocketStation is the programmable audio graph layer for when you only need audio and you need to own the graph."

### Vapi / Retell / Bland — Voice-Agent Deployment

These own agent deployment: phone numbers, assistant config, workflows, provider integrations, analytics. That is not PocketStation's lane unless the goal is to become a full agent SaaS.

**Better:** Vapi owns "agent deployment." PocketStation owns "audio I/O and graph control for agents." They integrate; they do not necessarily compete.

### OpenAI / Deepgram / ElevenLabs — Model Providers

These own models. PocketStation should not try to out-model them. They are graph nodes.

> Route to the right model at the right time with the right audio source, latency budget, privacy policy, and fallback behavior.

That is PocketStation's territory.

### Desktop Audio Tools

Rogue Amoeba Loopback, VoiceMeeter, BlackHole, JACK, PipeWire, Dante — prove demand for local routing. Most are platform-specific, local-first, manual, or pro-AV-oriented.

PocketStation differentiation: cross-platform, remote by default, programmable, SDK/API-first, AI model nodes, graph observability, source identity, enterprise private relay path.

---

## 23. Threat Analysis

### T1 — Apple Policy Change (High Impact, Medium Probability)

Apple restricts AUv3 sandbox, limits installTap, or changes AVAudioSession in a way that breaks a primary iOS SourceNode adapter.

Mitigation:
- Android-first MVP ensures iOS is never the only source adapter path
- 5 iOS insertion points are independent adapters
- Architecture plugs in new adapters without changing the graph runtime

### T2 — LiveKit Ships Audio-Only Graph SDK (Medium Probability)

Mitigation:
- Ship the graph API before LiveKit does
- Rust core is a measurable performance moat (battery, CPU on mobile)
- Audio-only pricing structure is cleaner than video-first economics
- Self-hostable relay with readable source is a trust moat

### T3 — OpenAI Ships End-to-End Audio Graph Infrastructure (Medium Probability)

OpenAI's Realtime API handles WebRTC, voice sessions, translation, and STT. It does not yet solve per-app desktop capture, virtual drivers, creator routing, low-latency multi-device monitoring, source separation, cross-model routing, or private audio rooms.

Mitigation: PocketStation is the routing layer between OpenAI and everything else. If OpenAI adds routing, PocketStation adds deeper OS integration and enterprise policy that OpenAI cannot own.

### T4 — No Distribution (High Probability If Not Addressed)

First public artifacts:
```
1. Show HN: pocketstation-audio — zero-allocation Rust audio graph runtime
2. Blog: Why every voice AI app rebuilds the same audio capture code
3. Demo video: multi-source graph → relay → browser, measured per-edge latency
4. Benchmark: mobile capture latency vs. raw platform APIs
5. Direct outreach: RustAudio community, iOS music dev community, voice AI startups
```

### T5 — FFI Boundary Crashes in Production (Medium Probability)

Mitigation: DOCS-001 defines the complete boundary contract before code. `proptest` for ring buffer invariants. DHAT in CI verifies zero allocation on hot path. Debug assertion: callback thread identity verified at session start.

### T6 — Graph API Too Complex for External Developers (New, High Probability)

Mitigation: Compatibility layer (`start_broadcast()` sugar, §3.8) keeps simple workflows simple. Graph templates give developers a starting point. Phase 4 exit criterion: external developer builds a graph in under 2 hours.

---

## 24. Kill Criteria

### Technical Kill / Pivot Criteria

```
1. P95 transport latency consistently > 500ms after optimization
2. Battery drain > 15% per hour for a broadcast session on modern iPhone
3. iOS App Store rejection for AVAudioSession misuse that cannot be resolved
4. Relay cannot maintain 99% session completion rate for 30-minute sessions
5. Graph runtime becomes too complex for external developers to understand in < 1 day
6. Cannot maintain stable source capture on at least 2 desktop OSes
7. Model routing adds too much latency to be useful in voice-agent contexts
8. Observability cannot accurately explain route or model failures for debugging
```

### Market Kill / Pivot Criteria

```
1. After 3 public demos and direct outreach, zero external developer integration requests
2. AI voice developers say source capture / routing / model switching is not painful
3. LiveKit or another platform adds equivalent source-aware graph routing before PocketStation gets traction
4. No design partner cares about graph observability
5. Developers only want a full agent SaaS, not graph infrastructure
6. No traction at 100 WAU after sustained public development
```

### Good Pivot Directions (If Needed)

```
Pivot A: Desktop/app audio capture SDK for voice AI only.
Pivot B: Realtime audio observability layer for voice agents.
Pivot C: Local creator routing + remote receiver product.
Pivot D: Private model audio gateway for enterprise voice AI.
```

---

## 25. Strategic Positioning

### 25.1 The One Competitor That Matters

LiveKit. Every decision answers: "Is this better than LiveKit for a developer who needs audio-only, graph-first, cross-platform infrastructure?" The answer should always be yes in at least three specific dimensions: smaller API surface, mobile capture depth, per-edge observability.

### 25.2 The First Thing to Ship

```
pocketstation (central Rust workspace; publish only after the narrow product API is proven)
+ desktop source adapters (macOS/Linux first)
+ audio-graph API (v0, typed nodes and edges)
+ Go relay as GraphSession
+ README with one working demo: multi-source graph → relay → browser, measured per-edge latency

Ship this. Measure who uses it and why.
Everything else follows from what those users tell you.
```

### 25.3 The Permanent Architecture Principle

The graph runtime commits to a node interface contract, not a fixed set of node types. Every new source, transform, policy, model, transport, or sink is a new adapter. The graph runtime never changes when a new adapter is added.

### 25.4 The Question That Decides Every Feature

> Does this feature strengthen PocketStation as the programmable realtime audio graph layer?

If yes, build. If no, defer.

### 25.5 The Phrase That Must Be True Everywhere

> Any audio → any graph → any human, app, device, room, model, or agent.

Not a tagline. An architectural commitment. If a proposed node type or edge contract breaks this sentence, the design is wrong.

---

## 26. Open Engineering Questions

Each open question blocks a phase exit. Each gets an ADR before code lands.

### DOCS-004: Backpressure Policy — RESOLVED

Drop newest. §4.

### DOCS-005: Relay Forward-Loop Locking — Blocks Phase 2

Phase 1: `sync.RWMutex` per packet. Phase 2: `atomic.Pointer` on subscriber slice. See §8.

### DOCS-006: Clock Sync — RESOLVED

PI-controlled linear interpolation for voice; hook for variable-rate SRC in music/broadcast mode.

### DOCS-007: Capability Negotiation — RESOLVED

Auto-insert adapters with `negotiated: NegotiatedCapability` on the stream.

### DOCS-008: Workspace Release Sequencing — Blocks Phase 1 First Publish

See §14.4. `cargo-release --workspace` with sequenced publish and per-crate retry.

### DOCS-009: Pion WriteRTP Allocation Profile — Blocks Phase 1

Does `TrackLocalStaticRTP.WriteRTP` mutate `pkt`? Need `pkt.Clone()` per bus subscriber? GC pressure at N subscribers × 50 pkt/sec? Resolve before Phase 1 ships.

### DOCS-010: JitterBuffer Algorithm — Blocks Phase 1

Adaptive (NetEQ-class) for Phase 1. Optional upgrade to RTT-variance-driven + PLC in Phase 4.

### DOCS-011: SPSC Ring Buffer Crate — Blocks Phase 0

Default candidate: `rtrb`. Verify against criteria in Phase 0 prototype.

### DOCS-012: Opus Frame Duration — RESOLVED

20ms default, 10ms optional for voice-agent mode after Phase 1 latency benchmarks.

### DOCS-013: Internal Sample Format — RESOLVED

Interleaved f32, 48kHz, mode-dependent channel count. See §4.2.

### ADR-014: Graph Manifest Format — Blocks Phase 1

```
JSON first (Phase 0/1 readability)
Protobuf by Phase 2 (when SDKs multiply)
Hybrid: JSON for developer authoring, protobuf on the wire
```

**Recommended:** JSON manifest for Phase 0/1. Protobuf protocol by Phase 2 when protocol repo is created.

### ADR-015: Node and Edge Stable IDs — Blocks Phase 1

Every node, source, bus, edge, and policy needs stable deterministic identity for telemetry and session replay. Decision needed: UUID v4 at runtime, or developer-assigned names, or both?

### ADR-016: Clock Domains Across Graph — Blocks Phase 2

A graph spanning multiple devices and model providers has multiple clock domains. Each `AudioFrame` carries `clock_domain`. Decision needed: how are clock-domain crossings detected at graph compile time, and what adapter is inserted?

### ADR-017: Bus vs Transport Track Boundary — Blocks Phase 2

A semantic bus (named, typed, policy-governed) becomes a transport track (RTP SSRC, WebRTC MediaStreamTrack) at the TransportNode boundary. Decision needed: exact mapping rules, and how bus_id is carried in RTCP metadata.

### ADR-018: Model Node Privacy Contract — Blocks Phase 3

Which model nodes can send raw audio to external providers? Which require explicit developer opt-in? How does `PrivacyClass` on a SourceIdentity block a route to a cloud ModelNode?

### ADR-019: Graph-Aware Relay Metadata — Blocks Phase 2

The relay should route by bus_id, enforce policies, and export per-bus metrics — without decoding audio payloads. Decision needed: what metadata travels in-band (RTP header extension? RTCP SDES? WebSocket control channel?) and what stays on the control plane.

### ADR-020: Multi-Stem Recording Timeline — Blocks Phase 2

Independent stems can have gaps, clock drift, and different start times. Decision needed: container format (multi-track Opus in MKV? WAV stems in a zip? custom format?), gap representation, and how replay aligns stems for debugging.

### ADR-021: Policy Execution Safety — Blocks Phase 2

Which policies run in the realtime processing path (GainNode, GateNode)? Which run on the control plane (ModelSwitch, StartRecording, TriggerWebhook)? Which require async side effects (LatencyFallback, CostCap)? A policy node must declare its execution class.

### ADR-022: Model Fallback Contract — Blocks Phase 3

When a model node fails or exceeds latency budget: does the graph pause, reroute to fallback, emit silence, or drop frames? Decision needed per node type, per error class, with a declared fallback contract on `ModelNode::constraints()`.

---

*Document version 3.0 — green-light version. AudioGraph is the product center. v2.3 core algorithm, hot-path rules, platform specs, and FFI boundary contracts (DOCS-001 through DOCS-013) are fully preserved. New open questions: ADR-014 through ADR-022. Next revision trigger: Phase 0 exit criteria met and first crates.io publish at Phase 1 exit.*

*Kill criteria reviewed: 2026-06-26.*

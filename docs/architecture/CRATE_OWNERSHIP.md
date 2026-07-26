# CRATE_OWNERSHIP.md — PocketStation crate ownership contract

**Status:** Binding. Violations are BLOCKED in review. CI checks enforce this.
**Source:** Architecture constraints from `dd` + `dd2` (2026-07-07).

The highest-leverage action against architecture drift is defining what each crate is allowed to own.
If it is not listed under "Owns", it does not belong here.

---

## The rule in one sentence

> PocketStation owns contracts, execution, timing, capability truth, and compilation.
> PocketStation does not own provider meaning, product meaning, or future operator taxonomy.

---

## Public vs internal vocabulary

These two vocabularies must not bleed into each other.

**Public (developer-facing):**
```
Session, Source, Stream, Route, Bus, Pipeline, Stem, Mix
```

**Internal (compiler/runtime):**
```
GraphSpec, OperatorManifest, EndpointManifest, PortSpec, SignalSpec,
EdgeContract, RuntimePlan, ExecutionPartition, Bridge
```

---

## Core five — the real foundation

These five crates define the architecture. Everything else builds on them.

| Crate | Owns | Must NOT own |
|---|---|---|
| `pks-frame` | `AudioBufferPool`, exclusive/shared frame handles, `AudioFrame`, `SharedAudioFrame`, frame/stream/source/session IDs, sample specifications, timestamps, sequence numbers, deadlines, clock IDs, payload format metadata | Whisper, Ollama, Transcribe, Summarize, Ducking, Meeting recording, Deepgram, OpenAI, any product concept |
| `pks-timing` | ClockDriftEstimator, ClockCorrectionController, clock-domain timing snapshots; compiled experimental SegmentGate storage until a generated-audio endpoint exists | Generic date/time utilities, WebRTC/NetEQ playout, provider APIs, product claims for experimental controls |
| `pks-graph` | `GraphSpec`, registry-backed node declarations, `PortSpec`, **SignalSpec**, edge contracts, graph IR, `Compiler`, `RuntimePlanner`, `RuntimePlan`, memory/fan-out plans, **ExecutionPartition**, and **SafetyContract** | Concrete Whisper/Ollama/Deepgram providers, product UI, worker lifecycle, capture ownership, endpoint finalization |
| `pks-runtime` | Execution of a compiled `RuntimePlan`: `PlanScheduler`, `RealtimePlanExecutor`, `PlanEdgeRouter`, independent bounded edge receivers, and edge/queue/discontinuity observations | Graph compilation or `RuntimePlan` definitions, complete product Session lifecycle, product UI, operator meaning, provider integrations, platform capture |
| `pks-caps` | Capability, CapabilitySet, CapabilityRequirement, PlatformProfile, PermissionRequirement, UnsupportedReason | `SignalSpec` (lives in `pks-graph`), concrete implementations, provider meanings, product logic |

**Boundary note:** `pks-caps` owns platform/runtime availability truth (can this capability run here?). `pks-graph` owns signal/edge contracts (what flows between ports). Related but distinct.

## Endpoint lifecycle boundary

`pks-endpoint` is the acyclic contract between bounded runtime edges, concrete
destination implementations, and Session transaction ownership. It is not a
provider catalog or a second scheduler.

| Crate | Owns | Must NOT own |
|---|---|---|
| `pks-endpoint` | Open `OperatorId`, exact `OperatorId` + `NodeTypeId` driver registration, endpoint prepare/cancel/start-gate/running/stop/join-finalize state contracts, and authoritative endpoint observation/outcome records | Concrete connector/provider/relay/recording algorithms, Session transaction policy, graph execution, worker-thread creation, native capture, product workflows, or production no-op drivers |

`pks-endpoint` depends downward on `pks-runtime` for `PlanEdgeReceiver` and on
`pks-graph`/`pks-frame` for stable identities and setup context.
`pks-runtime` never depends on `pks-endpoint`. Both `pks-session` and concrete
endpoint packages such as `pks-nodes` may depend on `pks-endpoint`, preventing a
`pks-nodes -> pks-session` cycle.

---

## Session engine and language boundary

`pks-session` is the one public Session engine owner. It is deliberately not a
sixth foundation crate: it orchestrates the contracts and implementations owned
by the lower layers without redefining them.

| Crate | Owns | Must NOT own |
|---|---|---|
| `pks-session` | Safe Rust `SessionSpec`, public selectors/descriptors, declaration freeze, exact-source resolution orchestration, transactional startup/rollback, ownership of a running Session's capture/runtime/endpoint resources, cancellation, drain/join/finalization coordination, safe event/metric projections, and stable semantic errors | Graph compilation algorithms, plan scheduling/routing algorithms, buffer-pool implementation, native capture implementation, codec or recorder implementation, provider connectors, C ABI records/handle tables, language-SDK ergonomics, UI, or process-helper IPC |
| `pks-session-c` | Versioned C records and status codes, ABI/capability negotiation, engine-scoped generational foreign handles, marshalling, bounded event/metric polling, bounded immutable audio-batch leases, panic containment, reproducible headers, and C conformance fixtures | Session lifecycle semantics, graph/runtime algorithms, capture or endpoint implementations, provider code, language-owned APIs, or a second scheduler |

`pks-session-c` depends on `pks-session`; `pks-session` never depends on the
adapter. Python and Node may use direct PyO3 and Node-API adapters over
`pks-session` instead of being forced through C. Swift and Kotlin may use the C
projection or platform/generated wrappers. Every adapter preserves one Session
engine and the same lifecycle, error, backpressure, lineage, and observation
semantics.

The embedded-versus-helper decision and portable ABI constraints are binding in
[AUDIO-029](../adr/AUDIO-029-embedded-session-engine-boundary.md).
`pks-audio` may temporarily re-export Session types for source compatibility,
but it must not own a parallel Session runtime.

---

## Signal contract — `SignalSpec` (pks-graph, Phase 2 prerequisite)

**Do not use `SignalType`.** The original `SignalType` enum had two problems:
1. `Transcript`, `Tokens`, `VadEvents` are semantic roles, not fundamental signal classes — semantic creep through the back door
2. `Custom(TypeId)` is a Rust `std::any::TypeId` — breaks cross-language (Python, Swift, Kotlin, Protobuf, remote operators, config manifests)

The correct model is two-level: **class** (what kind of data) + **role** (what it means) + **optional schema**:

```rust
pub struct SignalSpec {
    pub class:  SignalClass,
    pub role:   Option<SemanticRole>,  // e.g. "transcript.partial", "vad.boundary"
    pub schema: Option<SchemaRef>,
}

pub enum SignalClass {
    PcmAudio(AudioFormat),
    EncodedAudio(Codec),
    Text(TextFormat),      // class for transcript, summary, LLM output, captions
    Event(EventFormat),    // class for VAD events, emotion labels, keyword hits
    Metrics,
    Control,
    Binary(BinaryFormat),
    Custom(SignalId),       // stable string — survives cross-language and manifests
}

pub struct SignalId(pub Cow<'static, str>);       // "com.acme.sentiment.label.v1"
pub struct SemanticRole(pub Cow<'static, str>);   // "transcript.partial"
pub struct SchemaRef(pub Cow<'static, str>);
```

Port wiring examples:
```text
PcmAudio + role "voice"                  — mic audio
PcmAudio + role "music"                  — system audio
Text     + role "transcript.partial"     — STT rolling output
Text     + role "summary.final"          — LLM summary result
Text     + role "llm.token"              — streaming LLM token
Event    + role "vad.boundary"           — speech start/stop
Event    + role "emotion.stress"         — emotion classifier output
Control  + role "route.patch"            — graph mutation
Metrics  + role "edge.latency"           — observability
Custom("com.acme.sentiment.label.v1")    — third-party extension
```

The compiler validates **class** compatibility. `SemanticRole` is for humans, tooling, and port-matching hints — not for the core to enumerate forever.

`PortSpec` uses `SignalSpec`. Both `Operator` and `Endpoint` expose `Vec<PortSpec>`:
```rust
pub struct PortSpec {
    pub name:   PortName,
    pub signal: SignalSpec,
    pub arity:  PortArity,
    pub timing: TimingContract,
}

// Endpoints expose multiple ports — a relay, multi-stem recorder, browser sink
// can have many inputs/outputs. Single-signal() is wrong.
pub trait Endpoint: Send + Sync {
    fn id(&self) -> EndpointId;
    fn inputs(&self) -> Vec<PortSpec>;
    fn outputs(&self) -> Vec<PortSpec>;
    fn capabilities(&self) -> CapabilityRequirements;
}
```

---

## Execution partitions and safety contracts (`pks-graph`)

`ExecutionPartition` (WHERE code runs) and `SafetyContract` (WHAT it guarantees) are separate types. Do not conflate them.

```rust
pub enum ExecutionPartition {
    AudioCallback,   // platform OS audio thread — strictest
    RealtimeCpu,     // dedicated RT thread — no alloc/locks/blocking
    AsyncWorker,     // Tokio task — can allocate, await, network
    BlockingWorker,  // spawn_blocking — disk, database, CPU-heavy
    External,        // remote service — always async, network required
}

pub enum SafetyContract {
    RealtimeSafe,       // no alloc, no locks, no blocking — AudioCallback / RealtimeCpu only
    AllocationAllowed,  // may heap-allocate
    BlockingAllowed,    // may block the thread
    NetworkAllowed,     // may make network calls
    ExternalService,    // backed by a remote provider
}
```

The compiler enforces: `SafetyContract::RealtimeSafe` → only `AudioCallback` or `RealtimeCpu` partition. Any edge crossing partitions gets a compiler-inserted `Bridge`. No cross-partition calls on the hot path.

---

## Capability examples (pks-caps)

`pks-caps` owns platform/runtime availability truth — can this run on this platform right now?

```text
capture.microphone
capture.system_audio
capture.app_audio
sink.virtual_mic
sink.speaker
record.multistem
transport.webrtc
codec.opus
ml.local_vad
```

---

## Platform and processing crates

| Crate | Owns | Must NOT own |
|---|---|---|
| `pks-capture` | Platform-neutral discovery/selectors, source identity/generation and permission evidence, runtime source events, bounded captured-frame streams, capture/output traits, and adapter contracts | Platform-specific implementations or Session orchestration |
| `pks-capture-macos` | CoreAudio tap / ScreenCaptureKit / device capture | Non-macOS paths |
| `pks-capture-windows` | WASAPI / process loopback | Non-Windows paths |
| `pks-capture-linux` | PipeWire/Pulse implementation | Non-Linux paths |
| `pks-audio` | Current compatibility façade: selected lower-crate re-exports, legacy Opus C exports, temporary Session declaration, and small demo/test helpers while ownership is migrated | `AudioBufferPool` or frame ownership, compiled graph/runtime ownership, a second Session engine, provider integrations, product workflows |
| `pks-codec` | Opus encode/decode, packet format, RTP timestamp metadata, jitter-adjacent helpers | WebRTC signaling, product logic |

---

## Operator and pipeline crates

| Crate | Owns | Must NOT own |
|---|---|---|
| `pks-nodes` | Current first-party factories/endpoints: synthetic, microphone, and system-output source factories; mono mixing; Bridge sink; VAD/denoise/AEC/watermark adapters; and explicit multistem recording lifecycle/finalization | Provider connectors, API-key fields, meeting/product workflows, or unimplemented operators presented as available |
| `pks-ml` | Current bounded local DSP implementations: VAD, denoise, echo cancellation, and audio watermarking with slice-based hot-path cores and `RuntimeNode` integration | OpenAI chat, Ollama summarize, Deepgram cloud STT, ElevenLabs TTS, agent orchestration, meeting notes product logic, or an unimplemented generic model runtime |
| `pks-pipeline` | Current legacy compatibility path: ring-backed `frame_bus`, linear `ProcessorGraph`, basic processors, and timing correction re-export until callers migrate to the compiled graph/runtime path | Product recipes falsely presented as implemented, compiled graph/runtime semantics, Session lifecycle, provider or model implementations |

---

## Dependency-direction rules

These rules are binding. Violating the dependency direction breaks the architecture invariant.

```text
pks-frame
  may depend on:      std / core / alloc only; small no_std-safe utilities
  must not depend on: tokio, serde_json, tracing, platform crates, providers

pks-graph
  may depend on:      pks-frame, pks-caps, serde (manifest serialization)
  must not depend on: pks-runtime, pks-nodes, pks-ml, any provider SDK

pks-timing
  may depend on:      std / core / alloc only
  must not depend on: pks-runtime, pks-graph, codecs, transports, provider SDKs

pks-runtime
  may depend on:      pks-frame, pks-graph, pks-metrics, pks-timing
  must not depend on: OpenAI/Deepgram/Ollama SDKs, app UI, platform capture implementations

pks-endpoint
  may depend on:      pks-frame, pks-graph, pks-runtime, serde
  must not depend on: pks-session, pks-nodes, platform capture implementations,
                      provider SDKs, concrete relay/recorder algorithms

pks-caps
  may depend on:      pks-frame (types only)
  must not depend on: pks-capture-macos/windows/linux concrete implementations

pks-capture
  may depend on:      pks-frame, pks-timing
  must not depend on: target capture implementations, pks-runtime,
                      pks-session, provider SDKs

pks-session
  may depend on:      pks-frame, pks-caps, pks-graph, pks-runtime, pks-endpoint,
                      pks-capture and target-selected capture adapters,
                      pks-nodes, pks-metrics
  must not depend on: provider SDKs, app UI, language SDK packages,
                      helper-process IPC, or language-runtime types in its
                      public contract

pks-session-c
  may depend on:      pks-session and minimal ABI-support crates
  must not depend on: platform capture implementations directly, pks CLI,
                      provider SDKs, Python/Node/Swift/Kotlin runtime packages,
                      or graph/runtime internals bypassing pks-session

pks-nodes
  may depend on:      pks-frame, pks-caps, pks-graph, pks-runtime, pks-endpoint,
                      pks-timing, pks-ml
  must not contain:   provider connector clients, API-key fields, product workflows

pks-ml
  may depend on:      pks-frame, pks-graph, audio-ml/* algorithm crates
  must not contain:   cloud APIs, LLM orchestration, agent logic, meeting products

examples/
  may depend on:      anything
```

`pks-pipeline` currently depends on `pks-frame`, `pks-timing`, and `pks-codec`.
It may compose timing correction primitives but must not own a second clock
estimator/controller implementation. It is a legacy compatibility crate, not
evidence that product recipes or the compiled Pipeline API already exist. A
frame timestamp is one clock-domain observation; it is never itself a measured
inter-clock offset.

---

## Plugin tier model

Not all operators are equal. The execution partition determines what is allowed.

```text
Tier 1 — Built-in / static Rust operators
  compiled with the binary
  can be RT-safe (RealtimeCpu / AudioCallback) if they obey SafetyContract
  examples: Duck, Mixer, VAD, Gain

Tier 2 — Dynamic external operators
  loaded at runtime via C ABI, abi_stable, or WASM Component Model
  async/worker partition only by default — never in AudioCallback
  must pass explicit RT certification to be promoted to RealtimeCpu
  examples: community plugin, WASM calculator

Tier 3 — Remote connectors
  external service-backed; always External partition
  async only; require explicit network capability declaration
  examples: Deepgram, OpenAI realtime, ElevenLabs
```

The future "plugin marketplace" cannot attach to the RT engine by default. It lives in Tier 2 or Tier 3. RT promotion requires explicit `SafetyContract::RealtimeCertified`.

---

## Forbidden first-party crates

Do not create:
- `pks-ai`
- `pks-ai-whisper`
- `pks-ai-ollama`
- `pks-ai-openai`
- `pks-ai-deepgram`
- `pks-model-connectors` / `model-connectors`

Provider integrations live in:
1. `examples/` in the `pocketstation` workspace — reference implementations
2. `docs/content/specs/recipes/` — runnable recipe docs
3. Community crates (optional connectors, only after real demand)
4. User application code

**Never in:** `pks-frame`, `pks-graph`, `pks-runtime`, `pks-caps`, `pks-nodes`, `pks-ml`.

---

## OperatorId convention

First-party operators use the `pks.*` namespace:
```text
pks.audio.duck.v1
pks.audio.mix.v1
pks.audio.gain.v1
pks.record.multistem_wav.v1
pks.transport.packetizer.v1
pks.ml.vad.silero.v1
```

External/community operators use reverse-domain notation:
```text
local.whisper.transcribe.v1
local.ollama.summarize.v1
vendor.deepgram.streaming_stt.v1
vendor.openai.realtime.v1
vendor.elevenlabs.tts.v1
```

The core does not own the meaning of external operator IDs. It only validates signal class, port arity, and execution partition compatibility.

---

## CI enforcement checks

"Binding" without enforcement is a slogan. Add this script to CI:

```bash
#!/usr/bin/env bash
# scripts/lint/check-architecture-constraints.sh
set -euo pipefail

# 1. Forbid first-party AI connector crate directories
if find . -maxdepth 4 -type d | grep -qE 'pks-ai(-|$)|model-connectors'; then
  echo "FAIL: Forbidden first-party AI connector crate directory found" >&2
  exit 1
fi

# 2. Forbid closed graph enums in core crates
if grep -rq "enum ModelNode\|enum PolicyNode\|enum SinkNode" \
    crates/pks-graph crates/pks-runtime 2>/dev/null; then
  echo "FAIL: Closed graph enum forbidden in pks-graph / pks-runtime" >&2
  exit 1
fi

# 3. Forbid provider names leaking into core
if grep -rq "Whisper\|Ollama\|Deepgram\|ElevenLabs\|OpenAI" \
    crates/pks-frame \
    crates/pks-graph \
    crates/pks-runtime 2>/dev/null; then
  echo "FAIL: Provider name leaked into pks-frame / pks-graph / pks-runtime" >&2
  exit 1
fi

# 4. Forbid Rust TypeId in pks-graph (breaks cross-language contract stability)
if grep -rq "std::any::TypeId\|core::any::TypeId" \
    crates/pks-graph 2>/dev/null; then
  echo "FAIL: TypeId in pks-graph breaks cross-language contract stability" >&2
  exit 1
fi

echo "Architecture constraint checks: PASS"
```

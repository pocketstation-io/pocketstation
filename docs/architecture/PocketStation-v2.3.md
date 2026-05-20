# PocketStation
## Project Document v2.3

**Date:** 2026-05-19
**Status:** Pre-phase-0, architecture direction locked, implementation details subject to Phase 0 profiler data
**Supersedes:** v2.2 (which superseded v2.1)

---

## Changelog v2.2 → v2.3

| v2.2 Claim | v2.3 Correction | Why |
|---|---|---|
| `SourceCapability::SystemOutput` (cross-platform) | Removed. Split into `DesktopSystemLoopback` (Windows/macOS/Linux), `EligibleAppPlayback` (Android), `ScreenProjectionMix` (Android). iOS no longer carries a `SystemOutput` placeholder. | "SystemOutput" hid platform-specific semantics behind one name; the enum now maps 1:1 to actual platform mechanisms |
| `SourceCapability::AppOutput` (Android-only meaning, generic name) | Renamed to `EligibleAppPlayback` | Name matches the Android policy reality |
| `SourceCapability::AirPlayRouteInput` (iOS-only meaning, generic name) | Renamed to `ExternalRouteInput` | Name describes the capability shape, not a single Apple feature |
| "Go + Pion v3" | Go + Pion v4 (current stable as of 2026-05-19); v5 in development | Pion v3 → v4 ship'd; using v3 on a new project is a deliberate downgrade |
| Phase 0 exit: "Crate published on crates.io with docs.rs documentation" | "Crate is publish-ready: docs build cleanly, `cargo publish --dry-run` succeeds, public API reviewed by at least one external Rust reviewer. Actual publish happens after Phase 1 demo validates the surface." | Publishing locks names and creates SemVer pressure before the API has met one real route |
| Opus frame duration left implicit (`960 samples at 48kHz`) | ADR-012 added: 20ms default; 10ms option for voice-agent mode after benchmarks | Frame duration cascades into pool sizing, packet rate, jitter buffer, CPU, bitrate overhead — must be explicit |
| Internal sample format + channel layout policy unstated | ADR-013 added: interleaved f32 internal; mono 48kHz voice mode; stereo 48kHz music/broadcast; `AudioProcessorNode` declares accepted channel layout, graph auto-inserts `MonoMixNode` | Every encoder, VAD, STT, and ML node makes assumptions; lock the policy or pay for surprises |

This is the green-light version. No further structural rewrites planned.

---

## Table of Contents

1. [Vision and Thesis](#1-vision-and-thesis)
2. [What PocketStation Is and Is Not](#2-what-pocketstation-is-and-is-not)
3. [The Universal Audio Fabric Architecture](#3-the-universal-audio-fabric-architecture)
4. [The Non-Changing Core Algorithm](#4-the-non-changing-core-algorithm)
5. [Capability Model — The Right Abstraction](#5-capability-model--the-right-abstraction)
6. [Architecture Decision Records](#6-architecture-decision-records)
7. [Platform Adapter Specifications](#7-platform-adapter-specifications)
8. [Relay Architecture](#8-relay-architecture)
9. [Security Model — Staged, Honest](#9-security-model--staged-honest)
10. [ML Audio Processing Layer](#10-ml-audio-processing-layer)
11. [Full Tech Stack](#11-full-tech-stack)
12. [API Design](#12-api-design)
13. [Observability](#13-observability)
14. [Repository Structure](#14-repository-structure)
15. [Build Phase Plan](#15-build-phase-plan)
16. [Market Reality](#16-market-reality)
17. [Target Markets — Ranked and Honest](#17-target-markets--ranked-and-honest)
18. [Business Model](#18-business-model)
19. [Infrastructure Cost Strategy](#19-infrastructure-cost-strategy)
20. [Funding Strategy](#20-funding-strategy)
21. [Research Path and Papers](#21-research-path-and-papers)
22. [Competitive Landscape](#22-competitive-landscape)
23. [Threat Analysis](#23-threat-analysis)
24. [Kill Criteria](#24-kill-criteria)
25. [Strategic Positioning](#25-strategic-positioning)
26. [Open Engineering Questions](#26-open-engineering-questions)

---

## 1. Vision and Thesis

### 1.1 The Vision

> **Any audio → any route → any output.**

Permanent north star. Not reduced. Not apologized for.

```
Every platform gives us different audio doors.
Some doors are open today.
Some doors are partial today.
Some doors require a plugin.
Some doors require a virtual device.
Some doors require user action.
Some doors require hardware.
Some doors will open later as platforms evolve.

PocketStation wins by having one permanent core
that accepts all doors as they become available.
```

### 1.2 The Thesis

PocketStation is an open source, mobile-first realtime audio infrastructure project.

**Phase wedge:**
```
pocketstation-audio       Rust core audio engine crate
pocketstation-ios         iOS Swift adapter
pocketstation-android     Android Kotlin adapter
Hosted cloud relay        minimal, production-grade
One killer demo           iPhone audio → relay → browser/AI backend with latency metrics
```

**Long-term system:**
```
Universal audio fabric:
  any SourceCapability → normalized FrameBus → RoutePlan → any OutputTarget
```

**The distinction:**
```
Vision:              Any audio anywhere.
Architecture:        Universal audio fabric.
First wedge:         Mobile audio SDK + relay.
First demo:          Phone audio → relay → browser/AI backend.
First product:       Creator station.
Social/consumer:     Only after one user community proves repeat usage.
```

### 1.3 What v2.2 Got Wrong (Fixed Here)

| v2.2 Claim | v2.3 Correction |
|---|---|
| `SystemOutput` as cross-platform enum variant | Split into platform-honest variants |
| Generic `AppOutput`, `AirPlayRouteInput` names | Renamed to capability-shape names |
| Pion v3 hardcoded | Pion v4 stable, current; v5 in flight |
| "Published on crates.io" as Phase 0 exit | "Publish-ready"; first publish lands after Phase 1 demo |
| Opus frame duration implicit | ADR-012 makes it explicit |
| Channel layout / sample format policy implicit | ADR-013 makes it explicit |

---

## 2. What PocketStation Is and Is Not

### Is

```
A Rust realtime audio core publishable as a crate
A mobile audio capture SDK (iOS + Android) for developers building voice features
A cloud audio relay for broadcast and AI voice workflows
A creator broadcast app built on top of the infrastructure
The missing layer between native device audio and realtime cloud applications
```

### Is Not

```
A secret iOS global audio interceptor
A Spotify redistribution tool
A social audio app first
A replacement for LiveKit across all use cases
A replacement for Rogue Amoeba on Mac desktop
A replacement for SonoBus for musician P2P jams
A hardware-free promise that ignores what each OS allows
```

### Positioning

**Internal mission:**
> Any audio → any route → any output.

**Developer:**
> PocketStation is the open-source mobile audio SDK and relay for realtime voice, creator broadcast, and low-latency audio apps.

**Product:**
> Start a live audio station from your phone. Share one link. Anyone can listen.

---

## 3. The Universal Audio Fabric Architecture

```
               ┌──────────────────────────────────┐
               │       CAPABILITY DISCOVERY        │
               │  What can this device do right now? │
               └──────────────────┬───────────────┘
                                  ↓
┌─────────────────────────────────────────────────────────────┐
│                       SOURCE LAYER                          │
│  mic / own-app / loopback / playback / plugin / file / ...  │
│  Every source produces normalized AudioFrame objects        │
└──────────────────────────────┬──────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────┐
│                       CORE BUS                              │
│  Normalize → Pool → FrameBus → Clock → ProcessorGraph       │
│  Mix → Encode → (branch to multiple sinks)                  │
└──────────────────────────────┬──────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────┐
│                       ROUTE LAYER                           │
│  cloud relay / LAN direct / P2P / voice-agent / recorder   │
│  RoutePlan selects and manages transport                    │
└──────────────────────────────┬──────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────┐
│                      OUTPUT LAYER                           │
│  phone / browser / AI backend / speaker / room / file       │
│  Every output accepts normalized AudioFrame objects         │
└─────────────────────────────────────────────────────────────┘
```

Adding a new source does not touch the route layer. Adding a new output does not touch the core bus. The capability model mediates between platform allowance and pipeline input.

### 3.1 The Source × Route × Output Matrix

**Sources:**
```
Microphone                    available everywhere, always
Own-app audio                 iOS AVAudioEngine own-app, Android own-app
Desktop system loopback       Windows WASAPI, macOS SCKit, Linux PipeWire
Eligible app playback         Android AudioPlaybackCapture (policy-gated)
Screen projection mix         Android MediaProjection screen+audio
Plugin host audio             iOS AUv3
Broadcast extension audio     iOS ReplayKit
External route input          iOS AirPlay-style receiver
Virtual device input          Windows SysVAD, macOS AudioDriverKit
Network stream input          Receive and re-route an existing stream
File or buffer playback       Offline source, testing, research
Hardware input                External audio interface
```

**Routes:**
```
Local playback
LAN direct (ICE local)
Cloud relay (default)
Voice agent backend (bidirectional)
Recorder
Virtual microphone
Peer-to-peer (with TURN fallback)
```

**Outputs:**
```
Local speaker
Browser tab
Mobile app
Desktop app
AI backend / STT endpoint
TTS return stream
Recording file
Virtual microphone (for other apps)
Public broadcast channel (later phases)
Hardware accessory
```

### 3.2 Why This Shape Preserves the Vision

The architecture commits to an interface contract, not a fixed set of sources or outputs. Every platform adapter is a plugin. The core never changes when a new adapter is added.

---

## 4. The Non-Changing Core Algorithm

```
InputNode → NormalizeNode → FrameBus → ClockSync → RingBuffer → ProcessorGraph → Mixer → Encoder → Transport → Receiver/JitterBuffer → OutputNode
```

Pipeline shape is invariant across platform, OS, codec, source. The platform provides I/O. The pipeline processes it.

ProcessorGraph is permanent from day one. Transformations (noise suppression, VAD gating, gain, resampling, AEC, diarization) are nodes in the graph. The graph starts empty. Nodes are added per use case. The pipeline shape never changes.

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

/// Drop contract — load-bearing part of the design.
/// drop() must remain forever:
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
    pub source_id:       SourceId,
    pub sample_rate:     u32,         // 48_000 internally (ADR-013)
    pub channels:        u8,          // 1 voice / 2 music (ADR-013)
    pub format:          SampleFormat, // F32LE interleaved (ADR-013)
    pub timestamp_ns:    u64,         // monotonic, per-node, never wall clock
    pub sequence_number: u64,         // monotonic per stream
    pub buffer:          AudioBufferHandle,
}
```

```rust
/// Phase 0 pool — 64-slot ceiling imposed by AtomicU64 bitset.
/// At 20ms frame duration (ADR-012): 64 × 20ms = 1.28s of headroom.
/// Expansion path if needed: segmented bitset (Vec<AtomicU64>) or lock-free
/// freelist (treiber stack of indices). Decision deferred until Phase 0
/// profiler data shows actual pool pressure.
pub struct AudioBufferPool {
    slots:     Box<[f32]>,          // contiguous block, allocated once at session start
    slot_size: usize,               // samples per slot — 960 at 48kHz/20ms (ADR-012)
    free_mask: AtomicU64,           // bitset of free slots; one bit per slot, 64-slot cap
}

impl AudioBufferPool {
    /// Wait-free. Returns None on overrun.
    pub fn acquire(&self) -> Option<AudioBufferHandle> { /* ... */ }
}
```

**Hot path rules — enforced, not aspirational:**
```
No heap allocation (verified by DHAT profiler in CI)
No locks (SPSC ring buffer + atomic pool bitset)
No blocking (callback returns immediately)
No logging (metrics are atomic counters, not log calls)
No async/.await (callback is synchronous)
No ObjC/Swift method calls on callback thread
No JNI calls per audio frame
No Rust panic across any FFI boundary
No ML inference on the callback thread (see §10)
```

Backpressure on pool/ring exhaustion: see §26.1.
SPSC ring buffer crate choice: see §26.8.

### 4.2 The ProcessorGraph

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
    /// or appropriate adapter when upstream layout differs. See ADR-013.
    fn accepted_channels(&self) -> ChannelLayout {
        ChannelLayout::Either
    }
}
```

Built-in (phase 0): PassthroughNode, GainNode, ResampleNode, MonoMixNode.
Optional (later phases, feature flags): VadNode, NoiseSuppressorNode, EchoControlNode, SpeakerEmbedNode.

---

## 5. Capability Model — The Right Abstraction

The system asks "which source capabilities are available right now on this platform?" — not "can we capture everything?"

### 5.1 Source Capabilities

```rust
/// Each variant maps 1:1 to a real platform mechanism.
/// No ambiguous cross-platform names; if Apple ships a future
/// system-level capture API, it gets its own variant rather than
/// overloading an existing one.
#[derive(Debug, Clone, PartialEq)]
pub enum SourceCapability {
    Microphone,                  // everywhere
    OwnAppAudio,                 // iOS AVAudioEngine, Android own-app, desktop own-app
    DesktopSystemLoopback,       // Windows WASAPI, macOS SCKit, Linux PipeWire
    EligibleAppPlayback,         // Android AudioPlaybackCapture (policy-gated)
    ScreenProjectionMix,         // Android MediaProjection screen+audio
    PluginHostAudio,             // iOS AUv3 (plugin in a host app)
    BroadcastExtensionAudio,     // iOS ReplayKit broadcast extension
    ExternalRouteInput,          // iOS AirPlay-style receiver, experimental
    VirtualDeviceInput,          // Windows SysVAD, macOS AudioDriverKit
    NetworkStreamInput,          // receive and re-route an existing stream
    FileOrBuffer,                // offline, testing
    HardwareInput,               // external audio interface
}

#[derive(Debug, Clone)]
pub struct AudioSourceDescriptor {
    pub id:                   SourceId,
    pub name:                 String,
    pub platform:             PlatformId,
    pub capability:           SourceCapability,
    pub latency_class:        LatencyClass,    // Realtime / LowLatency / Buffered
    pub reliability_class:    ReliabilityClass, // AlwaysAvailable / UserPermission / UserAction / Experimental / PolicyGated / FutureAPI
    pub requires_user_action: bool,
    pub available_now:        bool,
    pub policy_notes:         Option<String>,
}
```

**iOS source list:**
```json
[
  {"capability": "Microphone",              "available_now": true,  "requires_user_action": true,  "reliability": "UserPermission"},
  {"capability": "OwnAppAudio",             "available_now": true,  "requires_user_action": false, "reliability": "AlwaysAvailable"},
  {"capability": "PluginHostAudio",         "available_now": true,  "requires_user_action": true,  "reliability": "UserAction"},
  {"capability": "BroadcastExtensionAudio", "available_now": true,  "requires_user_action": true,  "reliability": "UserAction"},
  {"capability": "ExternalRouteInput",      "available_now": false, "requires_user_action": true,  "reliability": "Experimental",
   "policy_notes": "AirPlay-style receiver; not a normal App Store SDK path"}
]
```

iOS has no `DesktopSystemLoopback`. If a future iOS API exposes a streaming system capture, it becomes a new descriptor; it does not silently inherit a placeholder slot.

**Android source list:**
```json
[
  {"capability": "Microphone",          "available_now": true,  "requires_user_action": true,  "reliability": "UserPermission"},
  {"capability": "OwnAppAudio",         "available_now": true,  "requires_user_action": false, "reliability": "AlwaysAvailable"},
  {"capability": "EligibleAppPlayback", "available_now": true,  "requires_user_action": true,  "reliability": "PolicyGated",
   "policy_notes": "Android 10+ AudioPlaybackCapture. Requires MediaProjection permission. Capturable audio is limited by per-app capture policy (ALLOW_CAPTURE_BY_ALL / BY_SYSTEM / NONE) and AudioAttributes usage flags. Most restrictive policy wins."},
  {"capability": "ScreenProjectionMix", "available_now": true,  "requires_user_action": true,  "reliability": "UserAction",
   "policy_notes": "MediaProjection screen-capture flow includes audio mix where playback policy permits."}
]
```

**Desktop source list (Linux / Windows / macOS):**
```json
[
  {"capability": "Microphone",            "available_now": true, "reliability": "UserPermission"},
  {"capability": "OwnAppAudio",           "available_now": true, "reliability": "AlwaysAvailable"},
  {"capability": "DesktopSystemLoopback", "available_now": true, "reliability": "AlwaysAvailable",
   "policy_notes": "Windows WASAPI loopback / macOS screencapturekit / Linux PipeWire native"},
  {"capability": "HardwareInput",         "available_now": true, "reliability": "UserAction"},
  {"capability": "VirtualDeviceInput",    "available_now": false, "reliability": "FutureAPI",
   "policy_notes": "Requires SysVAD or AudioDriverKit installation; later phases"}
]
```

The core never hardcodes which platform's source to use. It asks the platform adapter, ranks by latency + reliability + policy, opens the best available.

### 5.2 Route Capabilities

```rust
#[derive(Debug, Clone)]
pub enum RouteKind {
    LocalPlayback,
    LanDirect,
    CloudRelay,
    VoiceAgentBackend,
    RecordingFile,
    VirtualMicrophone,
    PeerToPeer,
    HardwareBridge,
    PublicChannel,
    PrivateRoom,
}

#[derive(Debug, Clone)]
pub struct RoutePlan {
    pub source:            SourceId,
    pub outputs:           Vec<OutputTarget>,
    pub transport:         TransportKind,
    pub encryption:        EncryptionMode,
    pub latency_budget_ms: u32,
    pub fallback_routes:   Vec<RouteKind>,
}
```

### 5.3 Output Capabilities

```rust
#[derive(Debug, Clone)]
pub enum OutputTarget {
    LocalSpeaker,
    BluetoothDevice,
    WebListener,
    MobileApp,
    DesktopApp,
    VoiceAgentBackend,
    RecordingFile,
    VirtualMicrophone,
    PublicRoom,
    PrivateRoom,
    HardwareAccessory,
}
```

### 5.4 The Platform Adapter Trait

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

Capability negotiation on partial match: see §26.4.

---

## 6. Architecture Decision Records

### ADR-001: FFI/JNI Boundary Ownership

**iOS: Platform owns the audio callback thread.**

```
AVAudioEngine installTap fires (Apple's real-time thread, priority 47)
    ↓
Swift writes f32 samples into AudioBufferPool slot
    ↓  one memcpy of f32 data — unavoidable, accepted
Swift writes AudioFrame header fields + buffer handle into SPSC ring
    ↓
Rust reads ring on its own processing thread
    ↓
ProcessorGraph → Encoder → Transport
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

For `EligibleAppPlayback` (AudioPlaybackCapture), Kotlin writes to a pre-allocated `ByteBuffer.allocateDirect()`. Rust reads from the raw pointer. JNI is called once at session init to pass the pointer, never per frame.

Lifetime contract: ByteBuffer lives for session duration. Rust reads only within `on_capture_ready()`. Kotlin signals teardown via `AtomicBoolean`; Rust acknowledges before Kotlin frees.

**Desktop: CPAL. Rust owns the callback. Zero FFI.**

```rust
device.build_input_stream(&config, move |data: &[f32], info| {
    let handle = pool.acquire().expect("pool exhausted"); // §26.1 for production policy
    handle.copy_from_slice(data);
    let frame = AudioFrame::new(handle, info.timestamp_ns());
    let _ = bus.push(frame);
}, |err| tracing::error!("{err}"), None)
```

### ADR-002: Star Topology — No Relay Chains

All audio flows `source → cloud relay → listeners`. No device-to-device chains.

Relay chains stack latency (each hop: 20-80ms minimum), require each intermediate node to encode/decode, and provide no capability that star topology does not. WebRTC ICE handles LAN-direct paths automatically.

### ADR-003: Custom Go Relay — No LiveKit

Custom MVP relay using Pion v4. Production relay grows as a real subsystem.

LiveKit's architecture is built for video conferencing: simulcast, layer selection, active speaker detection, bandwidth adaptation per subscriber. Audio-only rooms are undefined behavior in their subscription model. Replacing LiveKit post-launch with paying customers is a full rewrite. Own the routing from day one.

A realistic MVP — RTP forwarding, room lifecycle, JWT, QR/room codes, TURN/STUN config, SSE presence — lands closer to 1200-1500 lines including error handling and integration tests. Treat 400 lines as the *core forwarding logic*, not the whole service.

```
Phase 1 relay (~1200-1500 lines total):
  RTP forwarding (~400 lines core)
  Basic room lifecycle
  JWT auth
  QR/room codes
  TURN/STUN config
  SSE presence
  Tests

Production relay (grows to ~3000-5000 lines):
  Reconnect logic (ICE restart, source reconnect, listener reconnect)
  Abuse controls (rate limiting, max listeners, room expiry)
  Regional routing
  Load balancing across relay instances
  SLO enforcement (definition in §13.5)
  Webhook events
  Recording trigger
  Multi-source mixing
  Graceful shutdown with session migration
  Full observability
```

---

## 7. Platform Adapter Specifications

### 7.1 iOS Adapter — Honest Priority Order

```
Priority 1: AVAudioEngine own-app (OwnAppAudio)
  Any audio PocketStation plays is captured via installTap
  Zero restriction, always works, no user action
  Reliability: AlwaysAvailable

Priority 2: Microphone
  Direct mic capture via AVAudioEngine input node
  User permission required
  Reliability: UserPermission

Priority 3: AUv3 effect plugin (PluginHostAudio)
  PocketStation loads as an audio effect in any AUv3 host
  GarageBand, Logic iPad, Cubasis, AUM, 580+ compatible apps
  User loads plugin once in host
  Reliability: UserAction

Priority 4: ReplayKit broadcast extension (BroadcastExtensionAudio)
  User-approved screen + app audio broadcast
  Sample buffers delivered to broadcast extension process
  IPC via App Group container to main app → relay
  Reliability: UserAction

Priority 5: AirPlay-style receiver (ExternalRouteInput)
  PocketStation appears as audio output destination
  User selects from Control Center / Now Playing
  Not a normal App Store SDK path
  Reliability: Experimental
```

iOS does not have silent global capture of all apps' audio. PocketStation does not claim it.

### 7.2 Android Adapter — Honest Priority Order

```
Priority 1: Microphone + own-app audio (AAudio/oboe, Rust-owned thread)
Priority 2: EligibleAppPlayback (Android 10+ AudioPlaybackCapture, policy-gated per source app)
Priority 3: ScreenProjectionMix (MediaProjection screen + audio, user grants per session)
Priority 4: Own-app playback capture (always works, no permission)
```

`EligibleAppPlayback` is meaningfully stronger than iOS's equivalents because AudioPlaybackCapture is an official API, but **it is not universal system output**. Source apps can opt out (`ALLOW_CAPTURE_BY_NONE`), and audio with usage flags like `USAGE_VOICE_COMMUNICATION` is not capturable.

Android OEM/HAL paths are not on the roadmap. That belongs in a separate document if ever pursued.

### 7.3 Desktop Adapter

CPAL handles *device I/O*. It does not handle graph-level integration, loopback capture of other apps, or virtual device creation. Those require native APIs per platform.

```
Windows:
  Phase 1: WASAPI loopback capture via windows-rs (DesktopSystemLoopback)
  Phase 2: CPAL for own-device I/O
  Later:   SysVAD virtual speaker driver (VirtualDeviceInput, C++/WDK)

macOS:
  Phase 1: screencapturekit-rs (DesktopSystemLoopback, ~1.9% CPU on Apple Silicon)
  Phase 2: CPAL CoreAudio for own-device I/O
  Later:   AudioDriverKit virtual device (VirtualDeviceInput, C++/DriverKit)

Linux:
  Phase 1: PipeWire native integration via pipewire-rs (DesktopSystemLoopback + graph nodes)
  Phase 2: CPAL for fallback ALSA/PulseAudio I/O on non-PipeWire systems
```

Linux PipeWire is the deepest desktop platform — it exposes the full audio graph natively. Build and validate the capability model on Linux first, then port iOS/Android constraints into the same model.

---

## 8. Relay Architecture

### 8.1 Two Product Modes

**Mode A — Broadcast**
```
One source → relay → N listeners
Star topology, one-to-many
Source publishes: one WebRTC track
Listeners subscribe: same track forwarded as identical RTP packets
No decode, no re-encode at relay (early phases)
No mixing (until multi-source rooms ship)
```

**Mode B — Voice Agent**
```
Client mic → relay → AI backend (bidirectional)
Two streams: uplink + downlink
Relay handles ICE, TURN, DTLS for both legs
Session metadata: room_id, session_id, latency budget
Webhook events: session_started, utterance_detected, session_ended
Observability: per-utterance latency, loss, jitter
```

### 8.2 Core Relay Implementation (Phase 1 MVP)

Built on Pion v4 (`github.com/pion/webrtc/v4`).

```go
package relay

import (
    "sync"
    "sync/atomic"
    "github.com/pion/webrtc/v4"
)

type Room struct {
    id          string
    mu          sync.RWMutex
    source      *webrtc.TrackRemote
    listeners   []*webrtc.TrackLocalStaticRTP
    packetCount atomic.Uint64
    byteCount   atomic.Uint64
}

func (r *Room) SetSource(track *webrtc.TrackRemote) {
    r.mu.Lock()
    r.source = track
    r.mu.Unlock()
    go r.forwardLoop()
}

func (r *Room) AddListener(track *webrtc.TrackLocalStaticRTP) {
    r.mu.Lock()
    r.listeners = append(r.listeners, track)
    r.mu.Unlock()
}

func (r *Room) forwardLoop() {
    for {
        pkt, _, err := r.source.ReadRTP()
        if err != nil { return }
        r.packetCount.Add(1)
        r.byteCount.Add(uint64(len(pkt.Payload)))
        r.mu.RLock()
        for _, l := range r.listeners {
            _ = l.WriteRTP(pkt) // see §26.6 for mutation/clone ADR
        }
        r.mu.RUnlock()
    }
}
```

**Performance notes for production:**
- `RLock` per packet across all listeners is acceptable at MVP scale but becomes a bottleneck above a few hundred listeners per room. Phase 2 switches to a copy-on-write listener slice (§26.2).
- Re-using the same `*rtp.Packet` across `WriteRTP` calls is a correctness question that must be resolved before Phase 1 ships. See §26.6.

### 8.3 Signaling Protocol

```
Client → Server (WebSocket JSON):
  PUBLISH:   room_id, token, SDP offer
  SUBSCRIBE: room_id, SDP offer
  ICE:       candidate
  LEAVE:     room_id

Server → Client:
  SDP_ANSWER: SDP answer
  ICE:        candidate
  ROOM_STATE: source_active, listener_count, codec
  ERROR:      code, message
```

These message types live inside the relay repo until Phase 2. Once a second SDK consumer needs them, they get extracted into the `protocol` repo with generated bindings.

---

## 9. Security Model — Staged, Honest

### Phase 1 — Transport Security

```
WebRTC DTLS/SRTP: transport encrypted between client and relay
The relay can read Opus payloads — accepted and disclosed
Room access controlled by JWT tokens (short-lived, room-scoped)
No public room listing — rooms are ephemeral by default
HTTPS everywhere for control plane
```

Say: "Encrypted transport. Room access requires a token."
Do not say: "End-to-end encrypted." Not true yet.

### Phase 2 — Access Controls

```
Short-lived source tokens (15-minute expiry, renewable)
Listener tokens with max-listener-count enforcement
Room expiry (auto-close after N hours of inactivity)
Abuse rate limiting (max rooms per IP, max listeners per room)
No recording by default — explicitly opt-in per session
```

### Phase 3 — True E2EE (SFrame, RFC 9605)

SFrame defines frame-level encryption where the SFU/relay forwards media with metadata visible but payload encrypted.

```rust
pub enum EncryptionMode {
    TransportOnly,        // Phase 1
    SFrameE2EE,           // Phase 3: relay is routing-blind to audio
    EnterpriseKeyManager, // Phase 4
}
```

**Implementation path (abstract, ADR-locked per platform):**

```
SFrame is applied at the encoded-frame layer before the relay
can inspect media payloads.

Per-platform insertion point — ADR pending:
  Browser:     WebRTC encoded-transform API where available
  iOS/Android: pre-packetization hook in the native WebRTC pipeline
  Server:      no transformation; relay forwards opaque payloads
```

```
Key exchange: control plane WebSocket, not the media path
Listener side: decrypts using room key
Key rotation: per session, optional per-minute for long sessions
```

### Phase 4 — Enterprise

```
Private relay deployment (customer VPC)
Audit logs
Data retention controls
HIPAA Business Associate Agreement (only when operationally ready)
SOC 2 Type II (12-18 month process)
Audio watermarking for EU AI Act compliance (SFrame payload hook)
```

---

## 10. ML Audio Processing Layer

All ML runs as ProcessorGraph nodes. All models run on-device. Raw audio is never sent to external ML APIs unless the developer explicitly routes to a voice agent backend.

**Threading rule (load-bearing):** ML nodes run only on the Rust processing thread, never inside platform audio callbacks. VAD inference is often quick enough that this distinction looks academic — until denoise or AEC pushes a model load or a 30ms inference into the callback path and the audio system glitches. The boundary is enforced architecturally: callbacks write to the SPSC ring and return; the processing thread drains the ring and runs all `AudioProcessorNode::process()` calls, including ML.

**Channel layout rule (ADR-013):** ML nodes declare `accepted_channels()`. Most voice ML expects mono. The graph inserts a `MonoMixNode` upstream of any mono-only node when fed stereo, transparently and at the cost of one extra allocation-free pass.

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

VAD gating saves 40-60% relay bandwidth in typical voice sessions. For AI voice agents, VAD drives end-of-utterance detection without external calls.

### 10.2 Noise Suppression

DTLN-rs — open source Rust, WASM-compatible, processes 1s audio in 33ms on M1. Accepts mono.

### 10.3 Echo Cancellation

`webrtc-audio-processing` (libwebrtc AEC3) or iOS native AEC via `AVAudioSession` mode `.voiceChat`.

`.voiceChat` enables Apple's built-in AEC but restricts AirPlay routing. Use iOS native AEC only in voice agent mode; use webrtc-audio-processing elsewhere.

### 10.4 Bandwidth-Adaptive Codec Control

Relay measures per-listener RTCP RR. Source adjusts Opus settings:

```
Loss < 1%, RTT < 100ms:    bitrate=96kbps, complexity=10, fec=false
Loss 1-5%, RTT < 200ms:    bitrate=64kbps, complexity=5,  fec=true
Loss > 5%, any RTT:        bitrate=32kbps, complexity=3,  fec=true, dtx=true
Loss > 15%:                trigger ICE restart, fallback TURN relay
```

### 10.5 Future ML Nodes

```
SpeakerDiarizationNode   TitaNet/Sortformer
AudioEnhancementNode     bandwidth extension
RealtimeTranslationNode  Whisper + MT (500ms+, broadcast only)
AudioWatermarkNode       EU AI Act compliance
```

---

## 11. Full Tech Stack

| Layer | Language / Library | Rationale |
|---|---|---|
| Audio engine core | **Rust** | Memory safety + zero-cost abstractions + cross-platform + lock-free |
| iOS adapter shell | **Swift** | Required for AVAudioEngine, AUv3, AVAudioSession |
| Android adapter shell | **Kotlin** | Required for AudioPlaybackCapture, MediaProjection, AAudio JNI bridge |
| Desktop device I/O (Windows/macOS/Linux) | **Rust (CPAL)** | Fallback path for standard device I/O |
| Windows system loopback | **Rust (windows-rs WASAPI loopback)** | CPAL does not expose loopback |
| macOS system loopback | **Rust (screencapturekit-rs)** | Native CoreMedia capture |
| Linux audio graph | **Rust (pipewire-rs)** | Graph-level access, virtual nodes |
| Windows virtual driver | **C++/WDK** | OS forced |
| macOS AudioDriverKit | **C++** | DriverKit requires C++ |
| Cloud relay | **Go + Pion v4** | Current stable Pion major (v4.0.0 released 2025; v5 in flight). Best WebRTC ecosystem; goroutines for N listeners. |
| Control plane API | **Go** | Shared codebase with relay |
| Web receiver | **TypeScript + WebRTC** | Browser-native; no framework for a listener |
| ML inference nodes | **Rust + ONNX Runtime** | WASM-compatible |
| Research tooling | **Python** | Latency measurement, DSP experiments |

**Pion version policy:** pin to `github.com/pion/webrtc/v4` at Phase 1 lock; track v5 release notes, defer upgrade until after Phase 3 SDK packaging is stable.

**Never:** Python in any production audio path. Never Python for the relay.

---

## 12. API Design

### 12.1 Developer SDK

```rust
let station = PocketStation::builder()
    .relay_url("wss://relay.pocketstation.io")
    .room_id("abc123")
    .mode(AudioMode::Voice)                    // mono 48kHz internal (ADR-013)
    .opus_frame_duration_ms(20)                // default; 10 for voice-agent (ADR-012)
    .add_processor(VadNode::default())
    .add_processor(NoiseSuppressorNode::default())
    .on_listener_count(|n| println!("{n} listening"))
    .on_packet_loss(|r| if r > 0.05 { /* adapt */ })
    .build()
    .await?;

let source = station.open_best_source(SourcePreference::Voice).await?;
station.start_broadcast(source).await?;
```

```swift
let station = try await PocketStation(roomID: "abc123", role: .source)
station.onListenerCount = { n in
    DispatchQueue.main.async { self.label.text = "\(n) listening" }
}
try await station.start()
```

```kotlin
val station = PocketStation.builder(context)
    .roomId("abc123")
    .role(Role.SOURCE)
    .build()

lifecycleScope.launch {
    station.listenerCount.collect { label.text = "$it listening" }
}
station.start()
```

```python
import asyncio
from pocketstation import PocketStation, AudioMode

async def main():
    station = PocketStation(room_id="abc123", mode=AudioMode.VOICE_AGENT)
    async for frame in station.listen():
        transcript = await stt.transcribe(frame.pcm)
        response   = await llm.respond(transcript)
        audio      = await tts.synthesize(response)
        await station.broadcast(audio)

asyncio.run(main())
```

### 12.2 REST API (Control Plane)

```
POST   /v1/rooms              Create room → {room_id, source_token, qr_url}
GET    /v1/rooms/{id}         Room state → {source_active, listener_count, codec}
DELETE /v1/rooms/{id}         Close room (source auth required)
POST   /v1/rooms/{id}/join    Get listener token + ICE config
GET    /v1/rooms/{id}/events  SSE presence stream

POST   /v1/apps               Register app → {api_key}
GET    /v1/apps/{id}/usage    Relay minutes this billing period
GET    /v1/apps/{id}/metrics  Aggregate latency, loss, sessions
```

---

## 13. Observability

Ships with Phase 0. Not Phase 4.

### 13.1 Per-Frame Metrics

```rust
pub struct BusMetrics {
    capture_to_bus_ns:   Histogram,   // P50/P95/P99/P999
    ring_utilization:    Gauge,       // 0.0-1.0
    overruns_total:      Counter,     // frames dropped (ring full)
    pool_exhaustion:     Counter,     // frames dropped (pool full)
    frames_total:        Counter,
}
```

### 13.2 Per-Session Relay Metrics

```go
type SessionMetrics struct {
    PacketsForwarded    prometheus.CounterVec
    BytesForwarded      prometheus.CounterVec
    ListenerCount       prometheus.GaugeVec
    ForwardLatencyNs    prometheus.HistogramVec
    ListenerLossRate    prometheus.GaugeVec
    SessionDurationSec  prometheus.HistogramVec
}
```

### 13.3 Latency Budget — Transport-Only Targets

PocketStation owns the **transport segment**: capture → encode → relay → decode → playback. It does not own STT, LLM, or TTS latency.

```
PocketStation segment (what we control):
  Source capture → FrameBus:           ≤ 5ms
  FrameBus → Opus encoder:             ≤ 2ms
  Opus → WebRTC send:                  ≤ 1ms
  WebRTC → relay (network, P95):       ≤ 50ms
  Relay → listener (network, P95):     ≤ 50ms
  Listener WebRTC → JitterBuffer:      ≤ 2ms
  JitterBuffer → output (adaptive):    ≤ 60ms

Target end-to-end transport P95:       ≤ 170ms
Target end-to-end transport P99:       ≤ 250ms

Voice agent transport-segment P95:     ≤ 185ms
  Leaves remaining budget for STT/LLM/TTS to land the full
  conversational loop near the ≤500-700ms perceptual ceiling.
  PocketStation does not promise the full loop — only its segment.
```

Targets, not guarantees. Phase 1 measures actual baselines. Architecture adjusts under profiler data.

### 13.4 Developer-Facing Latency Breakdown

```json
{
  "session_id": "...",
  "capture_ms": 3.2,
  "encode_ms": 1.1,
  "relay_rtt_ms": 44.0,
  "jitter_buffer_ms": 55.0,
  "decode_ms": 0.8,
  "transport_e2e_ms": 104.1,
  "packet_loss_pct": 0.3,
  "clock_drift_ppm": 12
}
```

### 13.5 SLI / SLO Definitions

```
SLI: Session completion
  Definition: Both source and last listener disconnect cleanly (CLOSE frame
              received) OR session duration ≥ 30 minutes with no fatal
              media-plane error.
  Measurement window: 28 days rolling
  SLO target (production): 99.9%
  Error budget: 43 minutes downtime / 28 days

SLI: Transport latency
  Definition: capture_ms + encode_ms + relay_rtt_ms + jitter_buffer_ms + decode_ms
  Measurement: per-session P95 reported by listener client
  SLO target: 95% of sessions show transport_p95 ≤ 250ms

SLI: Source publish success
  Definition: source token validated → WebRTC negotiated → first RTP packet forwarded
  SLO target: 99.5% within 3 seconds of token presentation
```

---

## 14. Repository Structure

### 14.1 Why a Monorepo Is Wrong

A monorepo would mean an iOS developer adding the Swift SDK also downloads the Go relay, Android JNI code, Python bindings, Windows WDK driver, and research notebooks. An ops engineer deploying the relay clones 800MB of Swift and Kotlin. The iOS Swift package cannot be published to Swift Package Index from inside a Rust workspace root.

```
One independently releasable unit = one repository.

Exception: Cargo workspace where internal crates are tightly coupled,
share types, and always release together. Used for the Rust audio core
and ML processing nodes — not the entire project.
```

### 14.2 GitHub Organizations

```
github.com/pocketstation-io/       core infrastructure, SDKs, services, apps
github.com/pocketstation-examples/ standalone example repos, one per use case
```

### 14.3 The Full Repo Map

#### Tier 0 — Protocol (created in Phase 2, not Phase 0)

Until a second SDK consumer needs stable generated wire types, signaling message definitions live inside the `relay` repo.

```
pocketstation-io/protocol  (created Phase 2)
  Language:   Protobuf + generated Go, Rust, TypeScript, Swift, Kotlin
  Contains:   Signaling message types, room API types, session metadata, error codes
  Published:  Generated code vendored into each SDK repo
  Releases:   Versioned independently; SDKs pin a protocol version
```

#### Tier 1 — Core Rust Engine (Cargo workspace)

```
pocketstation-io/audio-core
  audio-core/
  ├── Cargo.toml
  ├── crates/
  │   ├── pocketstation-frame/   AudioFrame, AudioBufferPool, SampleFormat
  │   ├── pocketstation-bus/     FrameBus, SpscRingBuffer, ClockSync
  │   ├── pocketstation-graph/   ProcessorGraph, AudioProcessorNode trait
  │   ├── pocketstation-codec/   OpusEncoder, OpusDecoder, JitterBuffer
  │   ├── pocketstation-route/   RoutePlan, CapabilityModel, SourceDescriptor
  │   ├── pocketstation-metrics/ BusMetrics, OTEL integration
  │   └── pocketstation-audio/   re-exports above as single entry point
  ├── benches/
  ├── tests/
  └── ffi/                       cbindgen → C headers for Swift/Kotlin

Release tooling: cargo-release with --workspace, single version across all crates.
                 See §26.5 for release sequence concern.
```

#### Tier 2 — Client SDKs

```
pocketstation-io/sdk-ios      Swift Package Index   PocketStation
pocketstation-io/sdk-android  Maven Central         io.pocketstation:pocketstation-android
pocketstation-io/sdk-js       npm                   @pocketstation/client
pocketstation-io/sdk-rust     crates.io             pocketstation-client
pocketstation-io/sdk-python   PyPI                  pocketstation
```

#### Tier 3 — Server Services

```
pocketstation-io/relay        Go + Pion v4 (signaling types live here until Phase 2)
pocketstation-io/api-server   Go (control plane)
```

#### Tier 4 — ML Processing Nodes (Cargo workspace, separate from audio-core)

```
pocketstation-io/audio-ml
  crates/
    pocketstation-vad/        Silero VAD
    pocketstation-denoise/    DTLN-rs
    pocketstation-aec/        webrtc-audio-processing
  models/                     ONNX model files, git-lfs

Separate from audio-core: ML deps (onnxruntime) are large and optional.
```

#### Tier 5 — OS Driver Extensions

```
pocketstation-io/driver-windows  C++/WDK
pocketstation-io/driver-macos    C++/DriverKit
```

#### Tier 6 — Applications

```
pocketstation-io/app-creator       React Native (iOS + Android)
pocketstation-io/app-web-receiver  Static TS, no framework
pocketstation-io/app-desktop       Tauri (Rust + web frontend)
```

#### Tier 7 — Developer Tools

```
pocketstation-io/cli   `ps` command — room create, source sine/file, listen, latency, relay status
pocketstation-io/docs  docs.pocketstation.io
```

#### Tier 8 — Examples Org

```
github.com/pocketstation-examples/
  voice-agent-openai
  voice-agent-android
  creator-station-ios
  creator-station-web
  latency-benchmark
  android-playback-capture
  desktop-loopback-linux
  ai-agent-latency-test
```

### 14.4 Dependency Graph

```
audio-core
    ↓
    ├── sdk-ios       (XCFramework)
    ├── sdk-android   (.so via JNI)
    └── sdk-rust      (crates.io dep)

audio-ml
    ↓ implements AudioProcessorNode from audio-core/pocketstation-graph

relay + api-server
    → deployed independently; SDKs connect over WebRTC/WebSocket
    → host signaling message definitions until Phase 2

protocol (Phase 2+)
    ↓
    ├── relay
    ├── api-server
    ├── sdk-ios
    ├── sdk-android
    ├── sdk-js
    ├── sdk-rust
    └── sdk-python
```

### 14.5 Release Strategy

```
audio-core:    SemVer. cargo publish for each crate, single workspace version.
               CI: cargo test + DHAT alloc check + criterion benchmarks.
               Release tooling: cargo-release --workspace. See §26.5.
               First publish: after Phase 1 demo validates the surface, not at Phase 0 exit.

sdk-ios:       SemVer aligned with audio-core major.
               CI: xcodebuild test on iOS simulator matrix.
               Publish: tag triggers SPM resolution.

sdk-android:   SemVer aligned with audio-core major.
               CI: Gradle test on emulator matrix.
               Publish: tag triggers Maven Central upload.

sdk-js:        SemVer independent.
               CI: jest + Playwright.

relay:         SemVer independent.
               CI: go test + race detector + integration test against sdk-js.
               Publish: tag builds + pushes Docker image.

apps:          No package registry.
               Tags trigger App Store / Play Store / GitHub Releases builds.
```

### 14.6 What Changes at Each Phase

```
Phase 0:  Create audio-core. Crate is publish-ready (not yet published).
Phase 1:  Create relay + api-server. Create app-web-receiver. Signaling types in relay.
          First crates.io publish of audio-core happens here, after demo works.
Phase 2:  Create sdk-ios. First SPM publish. Extract protocol repo when sdk-js or
          sdk-android needs stable generated types.
Phase 3:  Create sdk-android. First Maven Central publish. Create cli.
Phase 4:  Create app-creator.
Phase 5:  Create audio-ml. Create sdk-js, sdk-rust, sdk-python as demand warrants.
Phase 6:  Create driver-windows, driver-macos. Create pocketstation-examples org.
```

---

## 15. Build Phase Plan

Phases as milestones. No week numbers. Calendar pressure on real-time audio systems causes corner-cutting; the sequence is the contract, the dates are not.

### Phase 0 — Core DNA

Prove every future audio source can enter one core. No OS audio APIs. No UI.

Build:
```
AudioBufferPool (zero per-frame allocation, 64-slot cap documented)
AudioFrame with pool handles, interleaved f32 48kHz (ADR-013)
SPSC ring buffer (proptest-verified invariants; crate choice per §26.8)
FrameBus
ProcessorGraph (empty graph, PassthroughNode) with accepted_channels routing (ADR-013)
ClockSync (algorithm decision per §26.3)
Opus encoder/decoder at 20ms default frame duration (ADR-012)
Fake sine-wave source
File output sink
Full metrics
Backpressure policy decided (ADR-004, §26.1)
```

Exit criteria:
```
sine_wave → FrameBus → Opus encode → decode → WAV file
Zero heap allocation on hot path (DHAT in CI)
Ring buffer handles 10,000 frames/sec under 1-hour soak, zero overruns
BusMetrics P50/P95/P99 print correctly after 60-second run
Crate is publish-ready: docs build cleanly, `cargo publish --dry-run` succeeds,
  public API reviewed by at least one external Rust reviewer.
  Actual crates.io publish happens at Phase 1 exit, not here.
ADR-004 through ADR-013 written and merged.
```

### Phase 1 — First Real Route

Build:
```
iOS mic/own-app source (Swift FFI boundary locked in writing before code)
Go relay MVP (~1200-1500 lines including tests, Pion v4)
Pion WriteRTP allocation profile resolved (§26.6)
JitterBuffer algorithm chosen and benchmarked (§26.7)
Cloudflare Workers control plane
QR code generation
WebRTC signaling (types live in relay repo)
Browser receiver (static HTML)
E2E latency measurement + dashboard
First crates.io publish of audio-core
```

Exit:
```
iPhone mic → relay → browser listener on different network
P95 transport latency ≤ 250ms measured
Room creation → audio playing in ≤ 10 seconds
30-minute session stable
Latency dashboard shows per-component breakdown
audio-core v0.1.0 published on crates.io with docs.rs
```

### Phase 2 — Android + Relay Hardening + Protocol Extraction

Build:
```
Android mic source (AAudio/oboe, Rust-owned thread)
Android EligibleAppPlayback (Kotlin + direct ByteBuffer)
JNI bridge finalized (zero per-frame JNI calls)
Relay: copy-on-write listener slice (§26.2), reconnect logic, rate limiting,
       room expiry, graceful shutdown
SLO instrumentation (§13.5)
sdk-ios first SPM publish
Extract protocol repo when sdk-android or sdk-js is started
```

Exit:
```
Android source → browser listener, same performance targets
1-hour memory test: zero RSS growth
Relay survives source disconnect + reconnect without losing listeners
Relay handles 50 listeners joining/leaving rapidly without crash
SLO dashboard live
```

### Phase 3 — SDK Packaging + Developer Experience

External developer integrates mobile audio streaming in one afternoon.

Build:
```
sdk-android first Maven Central publish
Rust crate API finalized
Python SDK (PyO3 bindings)
Documentation: 3 quickstart guides, architecture explanation, API reference
3 demo apps
CLI (`ps` command)
```

Exit:
```
Developer unfamiliar with project follows iOS quickstart in ≤ 2 hours
Developer unfamiliar with project follows Android quickstart in ≤ 2 hours
NLnet grant application submitted
```

### Phase 4 — Creator Station App

Build:
```
iOS app: Start Station → QR → listener count → health meter
Android app: same UX
Web receiver: static page
Desktop app (Tauri): same room pairing, WASAPI/SCKit source
Room state, presence, reconnect UX
```

Exit:
```
Non-technical person starts station, friend listens in ≤ 1 minute
Station survives 30-minute session on 4G
Web receiver works in Safari, Chrome, Firefox
```

### Phase 5 — Deep Source Expansion

Build:
```
iOS AUv3 effect plugin (PluginHostAudio)
iOS ReplayKit broadcast extension (BroadcastExtensionAudio)
macOS screencapturekit-rs full integration
Linux PipeWire virtual sink + node
Desktop WASAPI loopback improvements
VAD node (Silero, ONNX)
Noise suppression node (DTLN-rs)
SFrame E2EE
```

Exit:
```
AUv3 loads in 3 major iOS host apps without crash
Audio from host app appears in broadcast session
macOS: any app's audio capturable at ≤ 2% CPU overhead
SFrame E2EE: relay cannot distinguish audio content from silence
```

### Phase 6 — Route Intelligence + Virtual Endpoints + Scale

Build:
```
Windows SysVAD virtual speaker driver (VirtualDeviceInput, C++/WDK)
macOS AudioDriverKit virtual audio device (VirtualDeviceInput, C++/DriverKit)
Capability probing: automatic best-source selection
Adaptive bitrate: relay-driven Opus parameter adjustment
Multi-region relay (Fly.io edge nodes, EU/US/APAC)
Horizontal scaling
Public broadcast channels
```

---

## 16. Market Reality

Three signals matter:

```
1. LiveKit raised $100M at $1B valuation building real-time voice/AI infrastructure.
   This is the benchmark and the acquisition signal.

2. SonoBus has been free, open source, and cross-platform for years, with a
   live user community. Cross-platform realtime audio demand is proven.

3. Every major AI voice API (OpenAI Realtime, Google Dialogflow, Amazon Lex V2)
   explicitly excludes audio capture utilities for iOS/Android. Mobile developers
   re-implement the same ~1000 lines of AVAudioSession lifecycle in every project.
```

That is the entire market thesis.

---

## 17. Target Markets — Ranked and Honest

### Market 1 — Developer Infrastructure (Start Here)

**Who:** AI voice app developers, mobile developers adding voice features, game developers, accessibility tool builders, education apps.

**Pain:** No major AI voice API ships mobile audio capture. Developers re-implement it independently, in every project.

**PocketStation removes:** ~1,000 lines of boilerplate per project × every project that integrates mobile voice.

**Reach:** crates.io, Swift Package Index, GitHub, Show HN.

### Market 2 — Creator Broadcast

**Who:** DJs, podcasters, mobile creators, radio-style streamers, small live event hosts.

**Pain:** Broadcast audio from phone to listeners requires OBS + audio interface + port forwarding + Discord routing or hardware. No one-tap solution.

**PocketStation gives:** Create room → share QR/link → listeners hear it.

### Market 3 — Consumer Audio Sharing

**Reality:** Consumers default to Bluetooth, AirPlay, Discord, FaceTime, Instagram Live. PocketStation wins only when demonstrably simpler or solving a moment none of those reach.

**Strategy:** Don't chase. Let it come to the creator station product organically.

### Market 4 — Social Broadcast Platform (Conditional)

Channels and rooms become viable only after one specific community shows repeat usage. Build infrastructure. Watch where repeat usage clusters. Build social features for that community specifically.

---

## 18. Business Model

### Phase 1 — Credibility Before Revenue

```
Open source core, adapters, relay source
Docs + benchmarks + blog posts
Free hosted relay (time-limited during development)
No paywall for first 1,000 developers
```

### Phase 2 — Developer Revenue

```
Free tier:     10,000 participant-minutes/month
Usage-based:   $0.0004/participant-minute (matches LiveKit)
Developer:     $49/month — 100K minutes + webhook events + analytics
```

### Phase 3 — Enterprise Revenue

```
Scale plan:    $299/month + overage — 1M minutes + SLA 99.9%
Enterprise:    Contract pricing — private relay, HIPAA BAA (when ready),
               SOC 2, custom SLA, source license
```

### North Star Metric

**Weekly Active Sessions (WAS):** unique rooms with at least one source and one listener exchanging audio in the past 7 days.

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
Room state              Cloudflare Durable Objects
TURN relay              Cloudflare Calls ($0.05/GB)
Recording storage       Cloudflare R2 (zero egress)
Metrics                 Grafana Cloud (free tier 10K series)
```

### Bandwidth Math

```
Opus 64kbps stereo = 8KB/s = 28.8MB/hour per listener stream
Hetzner CX23 included: 20TB/month → 694,444 listener-hours included

Cost formula:
  Monthly egress ≈ avg_listeners × avg_session_min × sessions/month
                  × bitrate_MB_per_min × overhead_factor(1.15-1.30)
```

### Cost By Phase

```
Phase 0-1 (0 users):         ~€5/month
Phase 2-3 (≤100 users):      ~€20/month
Phase 4 (100-1K users):      ~€50/month
Phase 5 (1K-10K users):      ~€130/month
Phase 6 (10K-100K):          ~€430/month
Scale (100K+):                Revenue required; at €430/month cost,
                              $14K MRR covers it with 97% margin.
```

---

## 20. Funding Strategy

### Phase 0-2 (No Equity, No VC)

**NLnet Foundation NGI Zero Commons Fund** — €5K–€50K, rolling open calls, ~3 months to decision.

**Grant pitch:**
> PocketStation is open-source realtime audio infrastructure for mobile devices, AI voice applications, accessibility tools, and creator broadcast. It eliminates the need for developers to re-implement cross-platform mobile audio capture for every voice AI application.

**Mitacs Accelerate** (if McGill enrollment): $15K CAD / 4-month unit, 50% government funded.

**NSERC Discovery** (if McGill enrollment): $20-50K CAD/year, 5 years.

**GitHub Sponsors:** when crates.io package has real downloads.

### Phase 3-5 (After Real Users)

**YC** when 1,000+ WAU or 10 developers with SDK in production.

**Sequoia / a16z Seed ($5-10M):** apply when $50K+ MRR or major AI company as design partner.

Pitch: "Open source mobile audio SDK every voice AI company building on mobile needs. LiveKit is the infrastructure comparable — $100M at $1B. We are audio-only, mobile-primary, Rust-core."

---

## 21. Research Path and Papers

IEEE, ACM, Elsevier, Springer accept independent researchers. Use "Independent Researcher" as affiliation. Most venues are double-blind. APC costs covered by NLnet grant.

Path: implement → measure → arXiv preprint → conference/journal.

```
Paper 1   PocketStation: A Mobile-Primary Cross-Platform Audio Routing
          Architecture with Formal Latency Analysis            (ACM MM / ICASSP)
Paper 2   Memory-Safe Real-Time Audio Pipelines: Rust Type System as
          a Callback-Thread Safety Proof                       (USENIX ATC / EuroSys)
Paper 3   P2P vs Star Topology for Consumer Audio Relay: A Measurement
          Study Across Real-World Network Conditions           (ACM IMC)
Paper 4   Cross-Platform Mobile Audio Capture for Voice AI: A Systematic
          Survey of iOS and Android Platform Constraints       (IEEE SP Magazine / ACM CSUR)
Paper 5   Energy-Efficient Mobile Audio Relay via VAD Gating: A
          Deployment Measurement Study                         (IEEE SP Letters)
```

---

## 22. Competitive Landscape

### 22.1 SonoBus — The Proof of Concept

| Dimension | SonoBus | PocketStation |
|---|---|---|
| Topology | P2P (breaks on enterprise/hotel networks) | Star / cloud relay default |
| Encryption | None | DTLS/SRTP Phase 1, SFrame Phase 3 |
| Mobile UX | Complex (IP entry) | QR code, 10 seconds |
| Primary audience | Musicians, jams | Developers, creators, AI voice |
| Cloud relay | No | Yes, default |
| SDK/API | No | Yes, primary product |
| AI voice integration | No | Yes, explicit design target |

### 22.2 LiveKit — The Benchmark

LiveKit ships full realtime platform (video + audio + data), Go server, Pion WebRTC, $100M raised, 200,000+ developers, OpenAI + Meta + Spotify as customers. **LiveKit has iOS and Android SDKs.**

PocketStation's wedge is shape, not existence:

```
Audio-only SDK — leaner, smaller binary, smaller API surface.
Capture-first — centers source capability discovery and adapter layering.
Observability-first — full latency breakdown as first-class API (§13.4).
Self-hostable relay — single Go service with a readable codebase.
Audio-only pricing — targets audio-only workloads cleanly.
Rust core — measurable CPU and battery advantage on mobile.
```

Pitch: "LiveKit is full-stack realtime; PocketStation is the right tool when you only need audio and you're shipping on mobile."

### 22.3 Rogue Amoeba / VB-Cable / BlackHole — Desktop Specialists

Don't compete on desktop local routing. PocketStation's desktop adapters are source plugins, not products.

### 22.4 CPaaS Giants

|  | Twilio | Agora | SignalWire |
|---|---|---|---|
| Primary model | CPaaS (telephony) | Enterprise SDK | Python voice AI SDK |
| Audio routing | Telephony-first | Video-first | AI orchestration |
| Mobile SDK | Yes (WebRTC) | Yes | No native mobile |
| Self-hostable | No | No | No |
| Pricing | Expensive | Expensive, opaque | Usage-based |
| Open source | No | No | Partial |

PocketStation advantage: open source + self-hostable + Rust + mobile-primary + audio-only pricing.

---

## 23. Threat Analysis

### T1 — Apple Policy Change (High Impact, Medium Probability)

Apple restricts AUv3 sandbox, limits installTap, or changes AVAudioSession in a way that breaks a primary iOS path.

Mitigation:
- Android-first MVP ensures iOS is never the only path
- 5 iOS insertion points are independent
- Architecture plugs in new insertion points without core changes

### T2 — LiveKit Ships Audio-Only Mobile-First SDK (Medium Probability)

Mitigation:
- Ship before LiveKit does — first-mover in open source has switching costs
- Rust performance moat: measurable battery and CPU advantage on mobile
- Audio-only pricing structure: cleaner economics than video-first
- Relay ownership: readable, forkable, auditable

### T3 — SonoBus Adds Cloud Relay and Encryption (Low Probability)

Mitigation:
- SonoBus is single-maintainer; cloud relay is a different scale
- Their explicit deprioritization of encryption is a product philosophy
- PocketStation ships before any SonoBus architectural change deploys

### T4 — No Distribution (High Probability if Not Addressed)

First public artifacts:
```
1. Show HN: pocketstation-audio — zero-allocation Rust audio routing primitives
2. Blog: Why every mobile voice AI app rebuilds the same audio capture code
3. Demo video: iPhone → relay → browser, measured latency, 30 lines of code
4. Benchmark: mobile capture latency across iOS/Android vs. raw platform APIs
5. Direct outreach to RustAudio, iOS music dev community, AI voice startups
```

### T5 — FFI Boundary Crashes in Production (Medium Probability if Not Designed Properly)

Mitigation:
- ADR-001 defines the complete boundary contract before code
- `proptest` for ring buffer invariants
- DHAT profiler in CI verifies zero allocation on hot path
- Debug assertion: callback thread identity verified at session start
- **Correctness enforced by tests + profiling + production crash telemetry.** App Store review is not a correctness oracle.

---

## 24. Kill Criteria

```
KILL OR PIVOT IF:

Technical:
  1. P95 transport latency consistently > 500ms after optimization
  2. Battery drain > 15% per hour for broadcast session on modern iPhone
  3. iOS App Store rejection for AVAudioSession misuse that cannot be resolved
  4. Relay cannot maintain 99% session completion rate for 30-minute rooms
  5. No developer can integrate the SDK after reading docs in < 1 working day

Market:
  6. After 3 public demos and direct outreach, zero external developer integration requests
  7. AI voice developers say LiveKit or another platform fully solves their mobile audio pain
  8. No traction at 100 WAU after sustained public development

Product:
  9. Creator station cannot achieve < 1 minute from zero to listening on first use
  10. Session reliability < 95% completion for 30-minute creator sessions
```

---

## 25. Strategic Positioning

### 25.1 The One Competitor That Matters

LiveKit. Every decision answers: "Is this better than LiveKit for an audio-only mobile use case?" The answer should always be yes, in at least three specific dimensions: smaller API surface, lower cost, mobile capture depth.

### 25.2 The First Thing to Ship

```
pocketstation-audio (Rust crate, publish-ready at Phase 0 exit,
                     published at Phase 1 exit)
+ pocketstation-ios (Swift package)
+ README with one working demo: iPhone mic → relay → browser

Ship this. Measure who uses it and why.
Everything else follows from what those users tell you.
```

### 25.3 The Permanent Architecture Principle

The architecture commits to an interface contract, not a fixed set of sources or outputs. Every platform adapter is a plugin. The core never changes when a new source or output is added.

```
Phase 1 opens: Microphone, OwnAppAudio (iOS), Microphone (Android), FileOrBuffer
Phase 2 opens: EligibleAppPlayback (Android), DesktopSystemLoopback (Windows)
Phase 3 opens: DesktopSystemLoopback (macOS, Linux)
Phase 4 opens: PluginHostAudio (iOS), BroadcastExtensionAudio (iOS)
Phase 5 opens: VirtualDeviceInput (Windows, macOS)
Phase 6 opens: Whatever platform APIs exist then
```

Each phase opens another door. The core never changes. The vision advances.

### 25.4 The Phrase to Use Internally

> Any audio anywhere is not one feature. It is the result of accumulating source adapters, route planners, and output adapters around one permanent audio core.

---

## 26. Open Engineering Questions

Each blocks a phase exit. Each gets an ADR before code lands.

### 26.1 Backpressure Policy on Pool / Ring Exhaustion — Blocks Phase 0

When `AudioBufferPool::acquire()` returns `None` or the SPSC ring is full:

```
A. Drop newest — stable latency, source-side audio loss visible
B. Drop oldest — keeps fresh audio flowing, encoder glitch
C. Block producer — violates no-blocking rule, non-starter
```

**Recommended:** A. Documented as ADR-004.

### 26.2 Relay Forward-Loop Locking — Blocks Phase 2

Phase 1 uses `sync.RWMutex` per packet. Bottlenecks above ~200 listeners/room. Phase 2 switches to copy-on-write atomic pointer.

```go
type Room struct {
    listeners atomic.Pointer[[]*webrtc.TrackLocalStaticRTP]
}
```

Documented as ADR-005.

### 26.3 Clock Sync / Async Sample Rate Conversion — Blocks Phase 0

```
A. Fixed-rate + drop/duplicate — voice-acceptable with VAD
B. PI-controlled linear interpolation — ~100 lines, voice default
C. Variable-rate SRC (libsoxr / rubato) — music quality, ~5x CPU
```

**Recommended:** B for voice, hook for C in music-mode. Documented as ADR-006.

### 26.4 Capability Negotiation on Partial Match — Blocks Phase 1

```
A. Fail with CapabilityMismatch
B. Auto-insert ResampleNode + MonoMixNode
C. Return descriptor delta, caller decides
```

**Recommended:** B with explicit `negotiated: NegotiatedCapability` on the stream. Documented as ADR-007.

### 26.5 Workspace Release Sequencing — Blocks Phase 1 First Publish

`cargo publish --workspace` fails mid-flight due to crates.io propagation. Need rigid dependency order with retry/backoff:

```
pocketstation-frame    (no deps)
pocketstation-bus      (deps: frame)
pocketstation-graph    (deps: frame)
pocketstation-codec    (deps: frame, bus)
pocketstation-route    (deps: frame)
pocketstation-metrics  (deps: frame, bus)
pocketstation-audio    (re-export, deps: all above)
```

Tooling: `cargo-release` with sequenced publish and per-crate retry. Git tag strategy: single `v0.X.Y` at workspace root, all crates published at the same version. Documented as ADR-008.

### 26.6 Pion `WriteRTP` Allocation Profile — Blocks Phase 1

```
1. Does pion/v4 TrackLocalStaticRTP.WriteRTP mutate pkt (header, SSRC, sequence)?
2. If yes, need pkt.Clone() per listener?
3. Per-listener allocation cost — header copy? payload copy? both?
4. Use shared payload with per-listener header rewriting (SFU pattern)?
5. GC pressure at 50 pkt/sec × 200 listeners = 10,000 calls/sec/room?
```

"No re-encode" does not mean "no allocation." Documented as ADR-009.

### 26.7 JitterBuffer Algorithm — Blocks Phase 1

Owns 60ms of the 170ms transport-P95 budget.

```
A. Fixed-delay buffer — simple; bad for two-way voice
B. Adaptive (NetEQ-class) — WebRTC standard; ~500 lines
C. RTT-variance-driven with PLC — wraps webrtc-audio-processing
```

**Recommended:** B for Phase 1, optional swap to C in Phase 5. Documented as ADR-010.

### 26.8 SPSC Ring Buffer Crate Choice — Blocks Phase 0

Criteria: wait-free both ends, no alloc after construction, bounded capacity, cache-line padding, maintained, ideally no_std.

**Default candidate:** `rtrb` — fixed-capacity allocation at construction, no allocation afterward, lock-free wait-free reads/writes. Verified against criteria; confirm in Phase 0 prototype. Documented as ADR-011.

### 26.9 Opus Frame Duration — Blocks Phase 0/1

Frame duration cascades into pool slot size, packet rate, jitter buffer sizing, CPU per second, bitrate overhead, and perceived latency. Must be explicit.

```
A. 10ms frames
   100 pkt/sec; lowest latency; ~2x header overhead vs 20ms; higher CPU
   Voice agent mode candidate: shaves ~10ms off transport budget

B. 20ms frames  (WebRTC default)
   50 pkt/sec; standard; balanced overhead and latency
   Default for broadcast and most voice paths

C. 40 or 60ms frames
   25 or 16.7 pkt/sec; lowest overhead; worst latency
   Acceptable only for one-way music/broadcast with VAD off
```

**Recommended:** 20ms default. Make it a runtime parameter on `PocketStation::builder()`. After Phase 1 latency measurements, enable 10ms for voice-agent mode if benchmarks justify the CPU/overhead tradeoff. Documented as ADR-012.

Knock-on effects to verify:
- Pool slot size: 20ms × 48kHz = 960 samples; 10ms = 480 samples; 60ms = 2880 samples
- Ring buffer headroom: 8 frames × 20ms = 160ms; with 10ms frames, 8 × 10 = 80ms (may need to grow ring)
- JitterBuffer target: minimum 1 frame of buffer, scales with frame duration

### 26.10 Internal Sample Format and Channel Layout — Blocks Phase 0

Every encoder, VAD, STT, ML node, and resampler assumes a format. Lock the policy.

**Internal sample format:** interleaved f32, little-endian, normalized [-1.0, 1.0]. Reasoning: most platform callbacks (CPAL, AAudio, AVAudioEngine with f32 buses) deliver interleaved by default; planar adds split/recombine cost at every boundary; interleaved matches Opus encoder input directly.

**Internal sample rate:** 48000Hz. Resample at adapter boundary if platform delivers something else.

**Channel modes:**

```
AudioMode::Voice
  Channels: 1 (mono)
  Use case: AI voice, voice agents, voice chat, podcasts
  ML nodes: most expect mono; no MonoMixNode needed

AudioMode::Music
  Channels: 2 (stereo)
  Use case: creator broadcast, DJ, live music
  ML nodes that require mono: graph auto-inserts MonoMixNode upstream
  Encoder: Opus stereo

AudioMode::Broadcast (default)
  Channels: configurable per session; defaults to stereo
```

**`AudioProcessorNode::accepted_channels()`** lets each node declare what it can ingest. Graph builder checks the chain and inserts adapter nodes (`MonoMixNode`, `StereoBroadcastNode`) at the right positions. Insertions are zero-allocation passes using the existing pool. Documented as ADR-013.

---

*Document version 2.3 — green-light version. No further structural rewrites planned. Phase 0 begins when ADR-004 through ADR-013 are written and merged.*
*Kill criteria reviewed: 2026-05-19.*
*Next revision trigger: Phase 0 exit criteria met and measured; first crates.io publish at Phase 1 exit.*

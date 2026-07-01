# MediaClock — Standalone Timing Engine

## Purpose

MediaClock is a codec-agnostic, zero-pks-dependency timing engine that owns
media timing where PocketStation controls the media path. It does NOT replace
WebRTC/NetEQ for standard human playout.

Three AI-specific problems standard jitter buffers cannot solve:

1. **Asymmetric latency** -- AI ingest needs earliest-possible frames with no
   jitter buffer. NetEQ always buffers for playout smoothness.
2. **LLM burst handling** -- TTS generates 500ms+ of audio at once. Pacing it
   into 20ms RTP frames requires egress timing that no receiver-side buffer provides.
3. **Barge-in/interruption** -- Cutting off stale AI speech mid-segment requires
   Gate semantics (segment_id, played_ms, truncation) that no jitter buffer owns.

## Components

### Ingress

Messy packets -> usable frames.

- Sequence unwrap (16-bit RTP wraparound -> extended u64)
- Reorder buffer (BTreeMap by extended sequence)
- Late/duplicate/missing detection
- Contract-driven capacity and target delay
- Adaptive delay estimation (EWMA)
- Codec-agnostic: operates on `Packet`, not Opus frames

### Egress

Generated/bursty audio -> clocked packets.

- Paced release of generated audio at codec frame rate (20ms/10ms intervals)
- Segment-tagged queue with monotonic timestamps
- Gate integration for interrupt/flush
- NOT a jitter buffer in reverse

### Gate

Interruption/truncation/stale speech cutoff.

- Owns segment_id and interrupt boundary
- Records played_ms at interruption point
- Truncates queued audio at audio_end_ms
- Clears stale speech from Egress queue
- Emits trace events for every state change

### Drift

Long-running clock alignment.

- Estimates producer/consumer clock drift in ppm
- Tracks accumulated timing error
- Phase 1: linear regression estimator only
- Phase 2 (deferred): WSOLA/time-stretch correction

### Trace

Timing/recovery/pacing proof.

- IngressTrace: packets_in, late, duplicate, dropped, missing, buffer_depth
- EgressTrace: queue_depth, pacing_delay_ns, frames_emitted, underruns
- GateTrace: flush_events, interruption_count, last_played_ms
- DriftTrace: drift_ppm, accumulated_error_ns

## API shape

```rust
use media_clock::{MediaClock, Contract};

let mut ingress = MediaClock::ingress(Contract::direct());
let mut egress  = MediaClock::egress(Contract::interactive());
```

The parent `MediaClock` type stays centered. Users do not import
`Ingress`, `Egress`, `Gate` directly.

## Crate structure

```
crates/media-clock/           # standalone, zero pks-* deps
crates/media-clock-opus/      # optional Opus PLC/FEC
crates/media-clock-testkit/   # impairment simulation lab
```

## What MediaClock does NOT do

- Does not replace WebRTC/NetEQ for human playout (Contract::Conversational delegates)
- Does not decode or encode any codec (codec-agnostic)
- Does not run in the relay (relay stays thin Go forwarder)
- Does not depend on any pks-* crate

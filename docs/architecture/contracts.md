# Contracts — Timing Behavior Classes

## Purpose

A Contract declares the timing behavior for a Route or Link. MediaClock uses
the Contract to configure Ingress buffer depth, Egress pacing, Gate behavior,
and Trace thresholds. Contracts are not labels -- they drive behavioral parameters.

## Contract classes

### Direct

Earliest usable frames, minimal buffering, AI/model ingest.

- **Use case:** feeding audio to STT, VAD, or AI model inference
- **Ingress:** target_delay_ms=0, max_buffer_depth=1 (pop immediately)
- **Egress:** no pacing (frames emitted as available)
- **Gate:** disabled (AI consumers handle their own flow)
- **PLC/FEC:** disabled (model tolerates gaps better than concealment artifacts)

### Interactive

Low latency, interruption-aware generated speech.

- **Use case:** TTS output from LLM, voice agent responses
- **Ingress:** target_delay_ms=20, max_buffer_depth=4
- **Egress:** pacing enabled (20ms frame intervals)
- **Gate:** enabled (barge-in support, segment tracking, played_ms)
- **PLC/FEC:** enabled

### Conversational

Normal human voice playout, WebRTC/NetEQ where appropriate.

- **Use case:** human-to-human voice, standard WebRTC calls
- **Ingress:** target_delay_ms=60, max_buffer_depth=8
- **Egress:** pacing enabled
- **Gate:** disabled (standard jitter buffer handles flow)
- **PLC/FEC:** enabled
- **Note:** for browser paths, this delegates to the platform's NetEQ

### Fidelity

Source-preserving, stereo/music-safe.

- **Use case:** music streaming, high-quality audio routing
- **Ingress:** target_delay_ms=40, max_buffer_depth=6
- **Egress:** pacing enabled
- **Gate:** disabled
- **PLC/FEC:** enabled (preserve continuity)

### Continuity

Broadcast/session smoothness, higher buffer allowed.

- **Use case:** live broadcast, radio-style streaming, long sessions
- **Ingress:** target_delay_ms=80, max_buffer_depth=12
- **Egress:** pacing enabled
- **Gate:** disabled
- **PLC/FEC:** enabled

## Contract vs EdgeContract

These serve different layers:

- **Contract** (MediaClock): transport timing behavior between endpoints
- **EdgeContract** (pks-caps): graph-edge contract between processing nodes

Both are needed. The audio-core adapter maps StreamProfile + EdgeContract
to the appropriate Contract for MediaClock.

## CLI shape (future)

```
pks route mic to openai/realtime --contract direct
pks route agent:voice to user --contract interactive
pks route app:Spotify to room/music --contract fidelity
pks route show:mix to broadcast --contract continuity
```

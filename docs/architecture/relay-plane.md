# Relay Plane — Forwarding and Control

## Boundary

The relay is a pure SFU (Selective Forwarding Unit). It forwards RTP packets
between sources and subscribers. It does not own media timing.

## What the relay does

- Forwards raw RTP packets from source to subscribers (byte-for-byte, except padding fix)
- Counts packets and bytes per AudioBus (atomic counters)
- Detects media stalls via last-RTP timestamp
- Reads RTCP Receiver Reports for CODEC_HINT and ICE_RESTART
- Forwards SFrame KEY_EXCHANGE opaquely (never reads key material)
- Optionally wraps Opus in RFC 2198 RED on the subscriber leg

## What the relay does NOT do

- Does not decode or encode audio
- Does not run a jitter buffer
- Does not run MediaClock
- Does not own graph timing
- Does not own AI interruption semantics
- Does not reorder or pace packets
- Does not mix audio (BusMix is subscription fan-out, not audio mixing)

## Technology

- Language: Go
- WebRTC: Pion v4
- Signaling: WebSocket + WHIP/WHEP (RFC 9725)
- Deployment: Fly.io

## MediaClock relationship

MediaClock lives entirely client-side (audio-core, CLI, SDKs).
The relay never instantiates or runs MediaClock components.
Contract selection happens at the client, not the relay.

## Future optimizations (tracked as issues, not code)

- SO_REUSEPORT for kernel-level UDP demux
- Opaque forwarding (skip all header inspection)
- Preallocated packet buffers
- Minimal packet inspection mode
- ICE ufrag routing (later)
- Geo routing (later)

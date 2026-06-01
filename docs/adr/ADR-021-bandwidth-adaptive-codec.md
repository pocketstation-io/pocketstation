# ADR-021-bandwidth-adaptive-codec — Bandwidth-Adaptive Codec Control via RTCP

## Status

Accepted for v2.3. Phase 5. Critical for Phase 4 exit.

## Context

PocketStation's architecture doc §10.4 describes the adaptive codec control policy but it is not implemented. The relay reads RTCP Receiver Reports (RR) per listener which contain loss fraction and round-trip time. This data enables the relay to instruct the source to adjust Opus encoder settings — bitrate, complexity, FEC, DTX — in response to network conditions. This is what Google's WebRTC ADM does internally but does not expose as a developer API. PocketStation can expose it as a first-class, documented mechanism. Critical for Phase 4 exit: "30-minute session stable on 4G."

## Decision

Parse RTCP RR in the relay per-listener. Add CODEC_HINT signaling message (relay to source via existing WebSocket). Three operating points (from architecture §10.4): (loss < 1%, RTT < 100ms) -> 96kbps/complexity=10/fec=false; (loss 1-5%, RTT < 200ms) -> 64kbps/complexity=5/fec=true; (loss > 5%, any RTT) -> 32kbps/complexity=3/fec=true/dtx=true. ICE restart triggered at loss > 15%. CODEC_HINT is best-effort: relay logs WARN on send failure, never blocks the forward loop. Source SDK applies hint to Opus encoder on next encode call (no flush required).

## Options considered

A) Fixed bitrate forever — degrades on lossy networks with no adaptation
B) Relay-driven RTCP feedback (this decision) — global view, coordinated adaptation
C) Client-side detection only — no relay coordination, per-client divergence

Chosen: B

## Consequences

- Relay parses RTCP RR packets from Pion (already available via track.SSRC callbacks)
- New signaling message type: CODEC_HINT {bitrate_kbps: u32, complexity: u8, fec: bool, dtx: bool}
- Source SDK processes CODEC_HINT and applies to the Opus encoder within 100ms
- ICE restart at loss > 15% (already partially implemented — formalized here)
- CODEC_HINT does not change the RTP stream format; listeners are unaffected

## Test / measurement plan

- Simulated packet loss via tc netem (Linux) or synthetic RTP drop in CI
- CODEC_HINT sent within 2 RTT cycles of entering a loss tier
- Source encoder parameters confirmed via relay metrics after hint applied
- ICE restart triggered at 15% loss and session continues
- Regression: existing relay tests pass unmodified

## Reversal trigger

RTCP feedback introduces measurable jitter (> 10ms P95 increase) over 1000 test sessions, AND client-side detection achieves equivalent adaptation without relay coordination.

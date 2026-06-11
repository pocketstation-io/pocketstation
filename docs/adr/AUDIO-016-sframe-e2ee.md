# AUDIO-016-sframe-e2ee — SFrame E2EE

## Status

Accepted for v2.3. Blocks Phase 5 exit.

## Context

RFC 9605 (SFrame, finalized August 2024) defines frame-level encryption where the relay forwards media with metadata visible but payload encrypted. EU AI Act Article 50 compliance deadline August 1 2026 requires encrypted and watermarked AI audio. PocketStation's relay can currently read Opus payloads (transport-only DTLS/SRTP). SFrame makes the relay routing-blind to audio content. This was listed as "Phase 3" in the original architecture but was blocked on RFC finalization; RFC 9605 finalized August 2024.

## Decision

Implement SFrame at the encoded-frame layer, after Opus encode, before relay forward. Key exchange over control-plane WebSocket (new KEY_EXCHANGE signaling message). Per-platform insertion: browser via WebRTC encoded-transform API; iOS/Android via pre-packetization hook in native WebRTC pipeline; relay forwards opaque SFrame packets without decrypting. Use the `sframe` Rust crate for encryption primitives in audio-core; Go `sframe` library in relay for validation.

## Options considered

A) Transport-only DTLS/SRTP (current) — relay reads payloads, no E2EE
B) SFrame RFC 9605 at frame layer — relay routing-blind, E2EE provable
C) Custom symmetric encryption — not interoperable with RFC standard

Chosen: B

## Consequences

- Relay receives and forwards SFrame-encrypted frames; cannot decode audio content
- Key rotation is a control-plane concern, not media-plane
- Adds ~22 bytes per frame overhead (~3% at 20ms Opus)
- EU AI Act Article 50 compliance path requires SFrame + AUDIO-017 watermarking together
- All platform SDKs must implement the pre-packetization insertion point

## Test / measurement plan

- Relay receives SFrame session, attempts decryption, verifies it fails
- Browser interop test: browser encrypts via encoded-transform API, relay forwards, second browser decrypts
- Key rotation test: rotate key mid-session, audio continues without audible gap
- Overhead test: SFrame adds < 5% latency vs non-encrypted baseline

## Reversal trigger

SFrame adoption below 10% of sessions AND no EU/US regulatory requirement by 2027.

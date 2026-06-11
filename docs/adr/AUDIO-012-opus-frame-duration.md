# AUDIO-012-opus-frame-duration — Opus Frame Duration

## Status
Accepted. Phase 0, Phase 1, and Phase 3 complete; no reversal triggered.

## Phase 3 implementation note (2026-05-23)

Real libopus bindings landed in `pocketstation-codec` via `OpusEncoder` / `OpusDecoder`.
- Dependency: `opus = "0.3"` (wraps libopus 1.x via `audiopus_sys`). Chosen as the de-facto Rust binding; approved in commit message.
- Encoder: 48 000 Hz, mono, `Application::Voip`, 20 ms / 960 samples per AUDIO-012.
- Decoder: 48 000 Hz, mono.
- f32 ↔ i16 conversion via multiply/divide by `i16::MAX` (32 767).
- Hot path: stack-allocated `[i16; 960]` for both encode and decode; no per-frame heap allocation beyond what libopus itself requires after init.
- `MockOpusEncoder` / `MockOpusDecoder` are retained as legacy aliases; removed from Phase 5.

## Context
PocketStation v2.3 requires this ADR before implementation lands. See `docs/architecture/pocketstation-v2.3.md`.

## Decision
20ms default. 10ms optional for voice-agent mode after benchmarks justify CPU/overhead tradeoff.

## Options considered

See v2.3 §26 for the complete option list.

## Consequences

- Agents must follow this decision until a new ADR supersedes it.
- Tests/benchmarks must verify the decision in the relevant phase.

## Test / measurement plan

- Add unit tests for correctness.
- Add benchmark where performance matters.
- Add soak/load tests where reliability matters.

## Reversal trigger

Measured Phase 0/1 data shows this decision breaks latency, reliability, safety, or developer usability targets.

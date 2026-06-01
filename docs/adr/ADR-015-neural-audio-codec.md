# ADR-015-neural-audio-codec — Neural Audio Codec / Mimi

## Status

Accepted for v2.3. Research track Phase 5, activates Phase 6.

## Context

Meta's EnCodec and Kyutai's Mimi codec (MIT-licensed, used in Moshi real-time conversational AI) encode audio as discrete tokens at 12.5 tokens/second. These tokens are directly compatible with LLM token streams, eliminating the STT step for LLM-native voice pipelines. Current PocketStation uses Opus (waveform codec requiring separate STT). Mimi is MIT-licensed. This ADR decides whether and when to add a neural codec track alongside Opus in audio-ml.

## Decision

Add `pocketstation-neural-codec` crate in audio-ml workspace as an opt-in feature-flagged ProcessorGraph node (--features neural-codec). Output type: AudioTokenFrame(Vec<u32>) carrying discrete codec tokens. Opus remains the default and only production codec for Phase 0-5. Neural codec activates in Phase 6. The Rust core bus is codec-agnostic; swap point is only at encode/decode boundary. Model weights via Git LFS (~85MB).

## Options considered

A) Opus only forever — no LLM-native token path
B) Neural codec replaces Opus — breaks all existing integrations
C) Neural codec as opt-in parallel track alongside Opus — additive, no regression

Chosen: C

## Consequences

- New AudioTokenFrame type added to pocketstation-frame crate
- Relay gains a second track type (token stream alongside RTP)
- LLM-native pipelines can receive audio tokens directly without STT
- Model weights are 85MB — always via Git LFS, never bundled in SDK binary
- Not available in Phase 0-5; feature flag prevents accidental use

## Test / measurement plan

- Encode 20ms frame to Mimi tokens, decode, SNR > 25dB (acceptable quality)
- Encode latency benchmark: < 20ms per 20ms frame on Apple M3 or Snapdragon 8 Gen 3
- LLM token stream interop test: tokens fed to language model context, valid continuation produced
- Feature flag test: --no-default-features build does not include neural codec

## Reversal trigger

Mimi inference latency exceeds 20ms per frame on reference mobile hardware AND no hardware-accelerated alternative exists by Phase 6 start.

# ADR-019-wasm-plugin-api — WASM Plugin API for DSP Nodes

## Status

Accepted for v2.3. Phase 6.

## Context

OBS Studio's plugin ecosystem (thousands of plugins, 10-year track record) demonstrates the value of extensible audio processing. OBS is desktop-only and non-real-time. PocketStation has the opportunity to define the first cross-platform (iOS/Android/browser/desktop/edge) plugin API for real-time audio relay graph nodes. A WASM-sandboxed plugin ABI means third-party ML models or DSP algorithms can be loaded as AudioProcessorNode instances without recompiling PocketStation — enabling voice conversion, custom EQ, acoustic models, and AI insert effects as drop-in plugins. LiveKit has no plugin API (GitHub issue #1049 open, unresolved).

## Decision

Define a stable C ABI for AudioProcessorNode that compiles to WASM. ABI: `ps_plugin_init(config_json, len) -> handle`, `ps_plugin_process(handle, in_ptr, out_ptr, frames) -> i32`, `ps_plugin_teardown(handle)`, `ps_plugin_accepted_channels() -> u8`. Host uses wasmtime (or wasm3 for embedded) for sandboxing. Plugin manifest (JSON) declares: accepted_channels, max_latency_ms, requires_gpu. Reference plugin: ps-plugin-gain (simple gain node, ~100 lines). Plugin registry is optional.

## Options considered

A) Rust trait only — no external plugins, no ecosystem
B) Dynamic library (.so/.dll) — no sandboxing, platform-dependent, crash risk
C) WASM plugin API — sandboxed, cross-platform, crash-isolated

Chosen: C

## Consequences

- Third-party DSP/ML plugins load without recompiling PocketStation
- Plugin crash does not crash the audio pipeline (WASM sandbox isolation)
- Defined stable ABI enables an ecosystem of compatible plugins
- wasmtime cold-start ~10ms per plugin load (one-time, not per-frame)
- Target overhead: < 0.5ms per 20ms frame after JIT warmup

## Test / measurement plan

- Reference plugin (gain) loads and processes audio correctly
- Plugin crash test: plugin calls abort(), host catches, removes node, continues
- Latency overhead: WASM sandbox adds < 0.5ms per 20ms frame after JIT warmup
- Cross-platform: same .wasm file runs on iOS simulator, Android emulator, macOS

## Reversal trigger

WASM JIT overhead exceeds 2ms per frame on mobile reference hardware AND no AOT compilation path reduces it below 0.5ms.

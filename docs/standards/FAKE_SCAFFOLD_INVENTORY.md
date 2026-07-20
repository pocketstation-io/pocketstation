# Fake / Scaffold Inventory

This file lists every component in this repo that is currently mocked, stubbed, hardcoded, deferred, or otherwise not production-grade.

**It is a living document.** Every PR that adds a scaffold appends a row. Every PR that replaces a scaffold burns the row down (delete the row in the same PR that replaces it).

**Rule:** if a component is fake but not in this file, the PR that added it failed the production bar. Reviewers block PRs that introduce un-inventoried fakes.

---

## Status column meaning

```
SCAFFOLD    Empty placeholder, returns Default::default() or similar
MOCK        Functional fake — tests pass against it, real impl absent
STUB        Throws unimplemented! or returns hardcoded value
PARTIAL     Real implementation, missing significant behavior
DEFERRED    Intentionally postponed; ADR or phase plan justifies it
```

---

## Active inventory

| Component | Status | Repo / File | What's missing | Replace by | Blocked on |
|---|---|---|---|---|---|
| `pks-audio` Session execution | PARTIAL | pocketstation / `crates/pks-audio/src/session.rs` | W1 validates the declarative Session and then returns typed `RuntimeNotIntegrated`; no route is claimed to execute. Replace with real capture-once bounded fan-out through connector, browser/remote, and multistem recording. | Phase 2 W6 | AUDIO-027 frame ownership, RuntimePlan edges/Bridges, AUDIO-028 recording, real endpoint integration |
| pocketstation-ml NoiseSuppressor | PARTIAL | pocketstation / pocketstation-ml / src/denoise.rs | Per-band RMS Wiener gate; missing full FFT-domain spectral subtraction (RNNoise/WebRTC NS quality) | Phase 5 | Phase 5: ONNX/RNNoise model |
| pocketstation-ml EchoCanceller | PARTIAL | pocketstation / pocketstation-ml / src/aec.rs | NLMS 512-tap adaptive filter; missing delay estimation, double-talk detector, frequency-domain partitioned-block (WebRTC AEC3 quality) | Phase 5 | Phase 5: production AEC |
| pocketstation-capture-macos (Application mode) | PARTIAL | pocketstation / pocketstation-capture-macos / src/macos_tap.rs | `CaptureMode::Application` uses CoreAudio tap; per-app routing works but sub-5 ms requires ASP plugin (Wave D). Public claim: macOS 14.4+ until 14.2/14.3 tested on device. | Phase 3 Wave D | libASPL submodule + HAL plugin deployment |
| pocketstation-capture-macos (ASP plugin) | DEFERRED | pocketstation / pocketstation-capture-macos / asp/ | `pks_asp_is_installed()` returns 0 (stub); real plugin requires libASPL vendor submodule + signed deployment | Phase 3 Wave D | AUDIO-022; `vendor/libASPL` submodule (human operator step) |
| pocketstation-capture-linux | PARTIAL | pocketstation / pocketstation-capture-linux / src/linux.rs | PipeWire path + snd-aloop fallback implemented (AUDIO-024); no CI runner with real PipeWire daemon; PipeWire graph model (node serial, ports, links) still first-pass | Phase 3 Wave C | Linux CI runner with PipeWire |
| pocketstation-capture-windows | PARTIAL | pocketstation / pocketstation-capture-windows / src/windows.rs | WASAPI system-wide loopback + process loopback implemented (Wave B); WASAPI session enumeration weak; no Windows CI runner | Phase 3 Wave B | Windows CI runner; wasapi crate |
| pocketstation-capture (non-macOS/Linux/Windows) | STUB | pocketstation / pocketstation-capture / src/lib.rs | `capture_system_audio()` stub returns `Err(CaptureError::NotSupported)` on unsupported platforms | Never (no target) | — |
| JitterBuffer | PARTIAL | pocketstation / pocketstation-codec | EWMA adaptive depth implemented; missing: PLC (caller gets GapDetected but no concealment audio); adaptation gains not tuned on real network jitter; no real-device validation | Phase 5 | Real network jitter measurement |
| DHAT allocation check | RESOLVED | pocketstation | Replaced with assert_no_alloc = "1.1" in `crates/pocketstation-audio/tests/alloc_check.rs`; `tools/pocketstation-alloccheck` deleted (was broken: workspace-inheritance without workspace member). | — | — |
| TURN configuration | DEFERRED | relay | Production TURN credentials; STUN-only works on most networks | Phase 2 | TURN provider decision |
| SFrame E2EE | DEFERRED | relay + SDKs | Frame-layer encryption per RFC 9605 | Phase 3 | ADR for per-platform insertion point |
| Clock-domain (ASRC) adapter insertion | DEFERRED | pocketstation / pocketstation-graph / src/compiler.rs | Channel adapter is done (Wave 10 `InsertAdapterNodesPass` auto-inserts `transform.mono_mix` on stereo→mono-only edges). The clock-domain ASRC/resampler that would *bridge* mismatched clocks is still not inserted — `ValidateClockDomainsPass` continues to *reject* cross-clock fan-in (industry-standard async sample-rate conversion, cf. WebRTC audio mixer) | Wave 11+ | ASRC node + sample-rate adapter insertion |

---

## Phase 3 burns (completed 2026-05-23)

| Component | Status | Repo / File | Resolution |
|---|---|---|---|
| Opus encoder/decoder | resolved | pocketstation / pocketstation-codec | Real libopus bindings via `opus = "0.3"`. `OpusEncoder` / `OpusDecoder` wrap libopus at 48 kHz / mono / VOIP per AUDIO-012. `MockOpusEncoder` / `MockOpusDecoder` are retained as legacy aliases that delegate to the real types. |
| ClockSync | resolved | pocketstation / pocketstation-bus | PI controller implemented per AUDIO-006. `kp = 0.1`, `ki = 0.001`. Output clamped to ±10 ms. Gains will be tuned in Phase 5 with real-world measurements. |

---

## Phase 1 burns (completed 2026-05-20)

These rows were resolved at Phase 1 exit. Work landed in the `relay` repo, not in `pocketstation`.

| Component | Status | Repo / File | Resolution |
|---|---|---|---|
| Fake-source publisher | resolved | relay / cmd/fake-source | Implemented as P1-PROD-003. Binary publishes synthetic 0xAB RTP; it is a development tool, not an pocketstation integration. |
| Token authority | resolved | relay (relay owns issuance) | Implemented as P1-PROD-002. relay issues and validates JWTs; control-plane integration deferred to Phase 2. |
| Browser metrics | resolved | web-receiver | Completed as P1-PROD-006. Real RTCStats.getStats() values wired. |

## Permanent (intentional) scaffolds

These never become production — they exist for testing and development. They are listed here so they're not confused with production-track components.

| Component | Repo / File | Purpose |
|---|---|---|
| Sine wave source | pocketstation / examples/sine_to_wav | Phase 0 smoke test, latency measurement |
| Synthetic source node | pocketstation / pocketstation-nodes / src/source.rs | Registered `source.synthetic` NodeFactory; steady sine tone for graph smoke tests + latency/observability measurement |
| File output sink | pocketstation / pocketstation-route | Test recording, offline verification |
| In-memory token store | control-plane | Phase 1 only; Phase 2+ uses real persistence |

---

## Phase 5 additions (added 2026-05-23)

| Artifact | Status | Location | Replace by | Notes |
|---|---|---|---|---|
| AudioFrame.speaker_id field | PENDING | crates/pocketstation-frame/src/lib.rs | Phase 5 | AUDIO-018 prerequisite; Option<u32>; None = no diarization |
| AudioFrame.source_tag field | PENDING | crates/pocketstation-frame/src/lib.rs | Phase 5 | AUDIO-017 prerequisite; AudioSourceTag enum (Captured, AiTts) |
| AudioFrame.encryption_mode field | PENDING | crates/pocketstation-frame/src/lib.rs | Phase 5 | AUDIO-014 prerequisite; EncryptionMode enum |
| DHAT alloc CI gate | PENDING | .github/workflows/ci.yml | Phase 3 follow-up | Verify zero-alloc on hot path in CI (currently code-only claim) |
| WASM plugin host (wasmtime) | PENDING | crates/pocketstation-audio/ | Phase 6 | AUDIO-019; ps_plugin_* C ABI; sandbox via wasmtime |
| AudioTokenFrame type | PENDING | crates/pocketstation-frame/src/lib.rs | Phase 6 | AUDIO-015 prerequisite; Vec<u32> neural codec tokens |

## How to use this file in a PR

When introducing a scaffold:
1. Add the row before the code lands.
2. Be specific about "what's missing" — "real implementation" is not enough.
3. Pick a "replace by" phase. If it's unknown, mark `DEFERRED` and link the ADR or issue tracking the decision.

When replacing a scaffold:
1. Delete the row in the same PR that lands the real implementation.
2. The PR description references the row being removed.

When reviewing:
1. Block any PR that introduces a fake component without adding to the table.
2. Block any PR that claims to "complete" a scaffold but doesn't burn down the row.
3. Block phase exit if the table has rows whose "replace by" matches the current phase.

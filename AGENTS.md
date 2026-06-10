# AGENTS.md — pocketstation-io/audio-core

## Source of truth

Before editing, read:

1. `docs/architecture/PocketStation-v2.3.md`
2. `docs/REPO_CONTRACT.md`
3. Relevant ADRs in `docs/adr/` (AUDIO-004 through AUDIO-014)
4. The assigned GitHub issue

## Phase gate

This repo covers **Phase 0** (complete) and is the intake for **Phase 2**.

**Phase 0 — COMPLETE as of 2026-05-20**

- 26 tests pass across 6 crates (`cargo test --workspace`).
- Race detector clean.
- All exit criteria met per v2.3 §15 Phase 0 section.
- AUDIO-004 through AUDIO-013 written and merged.

**Phase 1 — audio-core is NOT involved**

audio-core does not connect directly to the relay in Phase 1. The relay's
`fake-source` binary (`relay/cmd/fake-source`) publishes synthetic 0xAB RTP
packets — it does not use audio-core output. Real audio-core output enters the
relay pipeline only when sdk-ios (Phase 2) wraps audio-core via FFI and
establishes a WebRTC PUBLISH connection.

If the current project phase is earlier than Phase 2 for this repo, do not
implement code here unless the issue has `phase-exception-approved`.

## Phase 2 intake

The following items are the gate for this repo entering Phase 2:

1. **First crates.io publish of `pocketstation-audio` v0.1.0** — per v2.3 §14.5
   and §15 Phase 1 exit criteria. Publish happens after the Phase 1 demo
   validates the API surface. Use `cargo-release --workspace` with the
   dependency order specified in AUDIO-008.
2. **Opus real bindings (libopus-sys)** — burn the Opus MOCK row in
   `docs/standards/FAKE_SCAFFOLD_INVENTORY.md`. Gated by `real-opus` feature
   flag; requires libopus-sys dep approval and CI job with libopus installed.
3. **ClockSync PI controller (AUDIO-006 resolution)** — replace the Phase 0
   exponential-smoother placeholder with the dual-stage anti-windup PI
   controller specified in AUDIO-006.
4. **DHAT CI integration** — replace `pocketstation-alloccheck` binary with
   a real `#[global_allocator]` shim or DHAT gate that proves zero per-frame
   heap allocation on the hot path.

sdk-ios depends on audio-core v0.1.0 being published to crates.io. sdk-ios
cannot be created until the Phase 2 publish lands. See AUDIO-014.

## Rules

- One issue = one branch = one PR.
- Do not edit unrelated repos.
- Do not create `pocketstation-io/protocol` before Phase 2.
- Do not change v2.3 architecture unless explicitly assigned.
- Do not add dependencies without approval.
- Do not bypass CI.


## Engineering Standards

Before code changes, every agent must read:

- `docs/standards/STAFF_ENGINEERING_BAR.md`
- `docs/standards/STRUCTURE_NAMING_STYLE_THINKING.md`
- `docs/standards/PRODUCTION_ENGINEERING_BAR.md`
- `docs/REPO_CONTRACT.md`
- relevant ADRs
- current phase progress file
- `docs/standards/FAKE_SCAFFOLD_INVENTORY.md`

All code follows the structure, naming, documentation, test naming,
comment style, and thinking process defined there.

Every non-trivial implementation documents:
- invariant
- ownership model
- failure behavior
- test coverage
- phase scope
- what is intentionally not implemented

Every PR that introduces a fake/mock/scaffold adds a row to
`docs/standards/FAKE_SCAFFOLD_INVENTORY.md`. Every PR that replaces one burns
the row down.

## Hot-path rules — non-negotiable

These rules apply to all code that runs inside an audio callback or on the
audio processing thread. They are enforced, not aspirational.

- No heap allocation (verify with DHAT in CI — Phase 2 gate).
- No locks (use SPSC ring buffer and atomic pool bitset).
- No blocking (callback returns immediately).
- No logging (use atomic counters; see `pocketstation-metrics`).
- No `async`/`.await` (callback is synchronous).
- No ObjC/Swift method calls on the callback thread.
- No JNI calls per audio frame.
- No Rust panic across any FFI boundary.
- No ML inference on the callback thread (ML nodes run on the processing
  thread, which drains the SPSC ring).

## Phase 5 intake gates

- [ ] AudioFrame: add speaker_id (Option<u32>), source_tag (AudioSourceTag), encryption_mode (EncryptionMode) fields — required by AUDIO-014, AUDIO-017, AUDIO-018
- [ ] DHAT alloc CI gate: add DHAT profiler step to CI; zero-alloc on hot path verified automatically (currently only code-structure claim)
- [ ] Relay echo timestamp mechanism (AUDIO-020): embed send_timestamp_ns in Opus payload for benchmark tool

## Phase 6 intake gates

- [ ] WASM plugin host: wasmtime integration in pocketstation-audio; ps_plugin_* C ABI (AUDIO-019)
- [ ] AudioTokenFrame type: Vec<u32> in pocketstation-frame for neural codec output (AUDIO-015)
- [ ] cbindgen C header: generate headers for Swift/Kotlin FFI (AUDIO-011 — long-standing deferral)

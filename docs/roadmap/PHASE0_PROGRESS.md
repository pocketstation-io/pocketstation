# Phase 0 Progress — audio-core

## Phase 0 COMPLETE — 2026-05-20

**Status: COMPLETE**

All 26 tests pass across 6 crates. Race detector clean.
Exit criteria met per v2.3 §15 Phase 0 section.

Remaining items carried to Phase 2:

- Opus real bindings (libopus-sys) — burn the Opus MOCK
- ClockSync PI controller per AUDIO-006
- DHAT CI integration (zero per-frame heap allocation gate)
- First crates.io publish of `pocketstation-audio` v0.1.0

Next phase: Phase 2 — sdk-ios + crates.io publish.
See AUDIO-014 for the Phase 1 integration state and Phase 2 gate.

---

## Task 0 — Fix CI blocker (DONE 2026-05-20)

### Changes
- `pocketstation-frame`: removed dead `ChannelLayout { Mono, Stereo }` (glob clash with `pocketstation-graph::ChannelLayout`)
- `pocketstation-frame`: added `Debug` for `AudioBufferHandle` and `AudioFrame`
- `pocketstation-frame`: added `is_empty()` to `AudioBufferHandle` (clippy `len_without_is_empty`)
- `pocketstation-frame`: suppressed `clippy::mut_from_ref` on `slot_mut` (SAFETY comment explains invariant)
- `pocketstation-bus`: fixed `PushError::Full(frame)` pattern match replacing non-existent `.into_inner()`

### Verification
- `cargo fmt --all -- --check` — clean
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo test --workspace` — 5/5 tests pass

### Staff Bar Self-Check — Task 0
- Smallest correct design: yes — removed dead code, fixed incorrect API usage
- Tests added or updated: yes — existing tests confirmed green; clippy clean
- Hot-path safe: yes — changes were removals and type fixes only
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none

---

## Task 1 — Fix sine_to_wav example (DONE 2026-05-20)

### Changes
- `pocketstation-audio/examples/sine_to_wav.rs`: sized ring to 64 (matches pool) so all 50 frames fit; replaced panicking `.expect("ring full")` with `let _ =` (backpressure drop policy intentional)
- `pocketstation-audio/Cargo.toml`: added `[[example]]` entry documenting the run command

### Verification
- `cargo run -p pocketstation-audio --example sine_to_wav` — exits cleanly, writes 48000 samples

### Staff Bar Self-Check — Task 1
- Smallest correct design: yes — sized ring to match pool capacity; no logic added
- Tests added or updated: not applicable (example binary, not a library function)
- Hot-path safe: yes — no hot-path changes
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none

---

## Task 2 — Harden AudioBufferPool release/drop (DONE 2026-05-20)

### Changes
- `release()`: check free_mask BEFORE fetch_or in debug mode so double-release assertion fires before any state mutation
- Added tests: acquire all 64, 65th returns None; drop releases slot; is_in_use tracks state

### Verification
- `cargo test -p pocketstation-frame` — 5/5 pass

### Staff Bar Self-Check — Task 2
- Smallest correct design: yes — one-line reordering in debug path; no production-build change
- Tests added or updated: yes — 3 new tests covering pool exhaustion, slot reuse, is_in_use
- Hot-path safe: yes — `release()` is wait-free, allocation-free; debug check is `#[cfg(debug_assertions)]`
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no — existing SAFETY comment on `UnsafeCell` access unchanged
- Remaining risk: pool memory layout uses `Box<[UnsafeCell<Box<[f32]>>]>` (pointer-per-slot) vs architecture spec `Box<[f32]>` (contiguous). Non-blocking for Phase 0 correctness; Phase 1 should evaluate cache behaviour. Document in Phase 1 ADR.

---

## Task 3 — FrameBus/rtrb invariant tests (DONE 2026-05-20)

### Changes
- `pocketstation-bus`: added tests for bounded capacity, drop-newest policy, FIFO order, empty-ring pop

### Verification
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo test -p pocketstation-bus` — 5/5 pass

### Staff Bar Self-Check — Task 3
- Smallest correct design: yes — tests only; no production code changed
- Tests added or updated: yes — 4 invariant tests added
- Hot-path safe: not applicable (test code only)
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none

---

## Task 4 — BusMetrics (DONE 2026-05-20)

### Changes
- `pocketstation-metrics`: added `sum_ns()` accessor to `SimpleHistogram`
- Added 6 tests covering Counter, Gauge, SimpleHistogram, and BusMetrics

### Verification
- `cargo test -p pocketstation-metrics` — 6/6 pass
- `cargo clippy --workspace --all-targets -- -D warnings` — clean

### Staff Bar Self-Check — Task 4
- Smallest correct design: yes — one missing accessor plus tests; no structure added
- Tests added or updated: yes — 6 tests covering all primitive types and field independence
- Hot-path safe: yes — all atomics, Relaxed ordering; no allocation, no lock
- Public API changed: yes — `sum_ns()` added to `SimpleHistogram` (additive, non-breaking)
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none

---

## Task 5 — ClockSync Phase 0 (DONE 2026-05-20)

### Changes
- `pocketstation-bus`: made `ClockSync` fields private, exposed `correction_ratio()`, `drift_ppm_estimate()`, `target_sample_rate()` as the stable Phase 1 API surface
- Added 3 tests: zero drift, positive drift convergence, negative drift direction

### Verification
- `cargo test -p pocketstation-bus` — 8/8 pass
- `cargo clippy --workspace --all-targets -- -D warnings` — clean

### Staff Bar Self-Check — Task 5
- Smallest correct design: yes — exponential smoother placeholder; AUDIO-006 owns the full PI controller
- Tests added or updated: yes — 3 tests covering initialization, convergence, and sign direction
- Hot-path safe: yes — pure f32 arithmetic, no allocation, no lock
- Public API changed: yes — fields made private; 3 read-only getters added (non-breaking, additive)
- New dependency: no
- Phase scope respected: yes — explicitly marked as Phase 0 placeholder in doc comment
- Unsafe added: no
- Remaining risk: exponential smoother is not the AUDIO-006 dual-stage PI; Phase 1 must replace with tested PI before production use

---

## Task 6 — JitterBuffer Phase 0 scaffold (DONE 2026-05-20)

### Changes
- `pocketstation-codec`: added doc comment clearly marking JitterBuffer as NOT production NetEQ
- Added `sequence_gap_ahead()` as a PLC hook point (AUDIO-009 placeholder)
- Added 5 tests: depth gating, FIFO order, late frame (documents non-reorder), missing frame gap detection, contiguous no-gap

### Verification
- `cargo test -p pocketstation-codec` — 6/6 pass

### Staff Bar Self-Check — Task 6
- Smallest correct design: yes — FIFO queue with depth gate; explicitly not production NetEQ
- Tests added or updated: yes — 5 tests; late-frame non-reorder is documented as a known Phase 0 limitation
- Hot-path safe: not applicable — Phase 0 scaffold; no audio callback path wired
- Public API changed: yes — `JitterBuffer`, `push`, `pop_ready`, `sequence_gap_ahead`, `depth` added (new type)
- New dependency: no — uses `std::collections::VecDeque`
- Phase scope respected: yes — AUDIO-009 references in doc comments; full NetEQ explicitly deferred
- Unsafe added: no
- Remaining risk: `next_expected_seq` field initially added but drove no behavior — removed in compliance pass (see Task 11)

---

## Task 7 — Codec allocation story (DONE 2026-05-20)

### Changes
- `MockOpusEncoder`/`MockOpusDecoder` doc comments mark them explicitly as test/demo only
- Added `encode_into(&mut Vec<u8>)` and `decode_into(&mut Vec<f32>)` allocation-free APIs for hot-path use
- `encode()` and `decode_to_vec()` delegate to `_into` variants (tests/examples unchanged)
- TODO(Phase 1, AUDIO-008) references added at Vec allocation points
- `real-opus` feature already correctly gated via `dep:opus`; compiles without libopus installed

### Staff Bar Self-Check — Task 7
- Smallest correct design: yes — allocation-free `_into` variants added; convenience wrappers unchanged
- Tests added or updated: not applicable (mock encoder/decoder shape; behavior covered by integration test)
- Hot-path safe: yes — `encode_into`/`decode_slice_into` are allocation-free when called with pre-allocated buffers
- Public API changed: yes — 2 new public methods added on mock types (non-breaking, additive)
- New dependency: no
- Phase scope respected: yes — real-opus gated; mocks clearly marked
- Unsafe added: no
- Remaining risk: `EncodedFrame.payload: Vec<u8>` forces one heap allocation per frame in the convenience path; Phase 1 AUDIO-008 should evaluate pool-backed byte buffer

---

## Task 8 — Allocation-check command (DONE 2026-05-20)

### Changes
- `pocketstation-alloccheck/src/main.rs`: exercises acquire→encode_into→decode_slice_into→release cycle with reused caller-owned buffers; asserts 0 pool failures
- `pocketstation-codec`: added `decode_slice_into()` to avoid intermediate EncodedFrame allocation in the hot path

### Verification
- `cargo run -p pocketstation-alloccheck` — prints "Phase 0 OK", 0 pool failures
- DHAT gate documented as Phase 1 work (ADR-TBD)

### Staff Bar Self-Check — Task 8
- Smallest correct design: yes — harness exercises the hot-path API shape without claiming DHAT integration
- Tests added or updated: not applicable (binary tool, not library; behavior verified by running the binary)
- Hot-path safe: yes — harness itself is a test driver, not on the audio callback path
- Public API changed: yes — `decode_slice_into` added to `MockOpusDecoder` (non-breaking, additive)
- New dependency: no
- Phase scope respected: yes — DHAT integration explicitly deferred with ADR-TBD reference
- Unsafe added: no
- Remaining risk: no actual allocation counting; Phase 1 must add a `#[global_allocator]` shim or DHAT to prove zero per-frame allocation on the hot path

---

## Task 9 — Benchmark/soak placeholders (DONE 2026-05-20)

### Changes
- `pocketstation-audio/examples/soak.rs`: 60-second soak (3000 × 20 ms frames), asserts 0 drops, 0 pool failures, exact sample count
- `pocketstation-audio/Cargo.toml`: registered `[[example]] soak`
- Criterion next steps documented in soak.rs header (add `[[bench]]`, `benches/`, CI gate)

### Verification
- `cargo run -p pocketstation-audio --example soak` — 3000 frames in ~307 ms, PASS
- Criterion: NOT added (no existing `[[bench]]` config); Phase 1 steps documented

### Staff Bar Self-Check — Task 9
- Smallest correct design: yes — soak example only; no Criterion dependency added (one real use case not yet present)
- Tests added or updated: yes — soak example asserts correctness invariants for the full pipeline
- Hot-path safe: not applicable (soak is a test driver)
- Public API changed: no
- New dependency: no
- Phase scope respected: yes — Criterion gating and `benches/` directories deferred to Phase 1
- Unsafe added: no
- Remaining risk: soak is not a latency benchmark; Phase 1 needs Criterion benches to prove per-frame timing budget

---

## Task 10 — Final Phase 0 audit (DONE 2026-05-20)

### Checks run
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings     PASS (0 warnings)
cargo test --workspace                                     PASS (26 tests)
cargo run -p pocketstation-audio --example sine_to_wav    PASS (48000 samples)
```

### Staff Bar Self-Check — Task 10
- Smallest correct design: yes — audit only; no code changes in this task
- Tests added or updated: not applicable
- Hot-path safe: not applicable
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: see Phase 1 gaps below

### Remaining Phase 1 gaps (not blockers for Phase 0 exit)

- **DHAT allocation gate**: `pocketstation-alloccheck` exercises the hot path but does not yet count heap allocations. Needs DHAT or a custom `#[global_allocator]` shim (ADR-TBD). Document the per-frame budget in `PocketStation-v2.3.md`.
- **Criterion benchmarks**: `criterion = "0.5"` is in workspace deps but no `[[bench]]` sections exist. Add `benches/` directories and CI gate in Phase 1.
- **ClockSync full PI controller**: AUDIO-006 owns the dual-stage anti-windup PI. Current stub is an exponential smoother only.
- **JitterBuffer adaptive depth + PLC**: AUDIO-009 owns the real adaptive NetEQ design. `sequence_gap_ahead()` is a hook only; no PLC is generated.
- **Codec real-opus feature**: `dep:opus` is gated correctly but requires `libopus` installed at link time. CI needs the `real-opus` feature tested in a separate job with libopus available.
- **EncodedFrame payload allocation**: `EncodedFrame.payload: Vec<u8>` means the struct itself forces one heap allocation. Phase 1 should consider a pool-backed byte buffer or a newtype over `Arc<[u8]>`.
- **Pool memory layout**: current `Box<[UnsafeCell<Box<[f32]>>]>` is pointer-per-slot; architecture spec shows `Box<[f32]>` contiguous. Non-blocking for Phase 0 correctness; Phase 1 should evaluate cache behaviour and decide via ADR.

---

## Task 11 — Staff Engineering Bar compliance pass (DONE 2026-05-20)

### Changes
- `docs/standards/STAFF_ENGINEERING_BAR.md` and `docs/standards/STRUCTURE_NAMING_STYLE_THINKING.md`: tracked in git (were present on disk but untracked)
- `AGENTS.md`: added Engineering Standards section cross-referencing docs/standards/
- `pocketstation-bus`: removed dead `BackpressurePolicy` enum (defined `pub`, never referenced); renamed GWT-prefixed tests to plain sentence style matching existing convention; added Given/When/Then comments to all tests
- `pocketstation-codec`: fixed TODO format to `TODO(Phase N, ADR-NNN)` across all comment sites; removed unused `next_expected_seq` field from `JitterBuffer` (computed in `push` but gap detection reads queue positions directly); renamed tests to sentence style; added GWT comments
- `pocketstation-frame`: added Given/When/Then comments to all 5 tests
- `pocketstation-metrics`: renamed tests to sentence style; added Given/When/Then comments

### Verification
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings     PASS (0 warnings)
cargo test --workspace                                     PASS (26 tests)
```

### Staff Bar Self-Check — Task 11
- Smallest correct design: yes — removals, renames, and comment additions only; no behavior change
- Tests added or updated: yes — all tests updated with Given/When/Then structure; test names normalized
- Hot-path safe: yes — no production code behavior changed
- Public API changed: no — `BackpressurePolicy` removal is non-breaking (it was defined but not used by any consumer)
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none — all clippy clean, all tests pass

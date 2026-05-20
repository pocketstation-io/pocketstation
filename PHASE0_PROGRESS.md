# Phase 0 Progress — audio-core

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

---

## Task 1 — Fix sine_to_wav example (DONE 2026-05-20)

### Changes
- `pocketstation-audio/examples/sine_to_wav.rs`: sized ring to 64 (matches pool) so all 50 frames fit; replaced panicking `.expect("ring full")` with `let _ =` (backpressure drop policy intentional)
- `pocketstation-audio/Cargo.toml`: added `[[example]]` entry documenting the run command

### Verification
- `cargo run -p pocketstation-audio --example sine_to_wav` — exits cleanly, writes 48000 samples

---

## Task 2 — Harden AudioBufferPool release/drop (DONE 2026-05-20)

### Changes
- `release()`: check free_mask BEFORE fetch_or in debug mode so double-release assertion fires before any state mutation
- Added tests: acquire all 64, 65th returns None; drop releases slot; is_in_use tracks state

### Verification
- `cargo test -p pocketstation-frame` — 5/5 pass

---

## Remaining gaps

- Task 1: `sine_to_wav` panics at runtime (ring capacity 8, pushes 50 frames)
- Task 2: `AudioBufferPool` double-release only protected in debug; needs more tests
- Task 3: No invariant tests for `FrameBus` bounded/FIFO/drop-newest behavior
- Task 4: `BusMetrics` fields exist but no tests
- Task 5: `ClockSync` is a stub; needs clean Phase 1 API
- Task 6: `JitterBuffer` has no tests for late/missing frames
- Task 7: Mock codec is not clearly marked test-only; hot-path `Vec` allocation present
- Task 8: `pocketstation-alloccheck` is a placeholder binary
- Task 9: No benchmarks or soak mode

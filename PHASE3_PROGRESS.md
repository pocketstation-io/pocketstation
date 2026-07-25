# Phase 3 Progress — pocketstation

## Phase 3 COMPLETE — 2026-05-23

**Status: COMPLETE**

38 tests pass across all crates. 0 clippy warnings. sine_to_wav exits cleanly.

---

## Task 1 — Real libopus encoder/decoder (ADR-012)

### Changes

- `crates/pocketstation-codec/Cargo.toml`: made `opus` a non-optional workspace dependency; added `recording` feature (alongside legacy `real-opus`). Dependency approved: `opus = "0.3"` wraps libopus via `audiopus_sys`; chosen as the de-facto Rust binding.
- `crates/pocketstation-codec/src/lib.rs`: replaced `MockOpusEncoder` / `MockOpusDecoder` unit-struct mocks with:
  - `OpusEncoder`: wraps `opus::Encoder`, 48 kHz, mono, `Application::Voip`, 20 ms (960 samples).
  - `OpusDecoder`: wraps `opus::Decoder`, same params.
  - `encode_into(&[f32], &mut Vec<u8>)`: converts f32 → i16 via stack `[i16; 960]`, calls `encoder.encode()`. No per-frame heap allocation beyond `Vec` growth on first call.
  - `decode_into(&[u8], &mut Vec<f32>)`: calls `decoder.decode()` into stack `[i16; 960]`, converts i16 → f32.
  - `MockOpusEncoder` / `MockOpusDecoder` retained as legacy aliases that delegate to real types; removed in Phase 5.
- `crates/pocketstation-audio/examples/sine_to_wav.rs`: updated to `MockOpusEncoder::default()` / `MockOpusDecoder::default()`.
- `crates/pocketstation-audio/examples/soak.rs`: same update.
- `tools/pocketstation-alloccheck/src/main.rs`: same update.
- `docs/standards/FAKE_SCAFFOLD_INVENTORY.md`: Opus row burned.
- `docs/adr/ADR-012-opus-frame-duration.md`: status updated, implementation note added.

### New tests (pocketstation-codec)

- `opus_encoder_encodes_960_sample_frame_to_non_empty_packet`
- `opus_decoder_decodes_encoded_packet_to_960_samples`
- `opus_round_trip_sine_preserves_approximate_magnitude`
- `mock_encoder_and_decoder_round_trip_via_legacy_api`

### Hot-path allocation note

`encode_into` and `decode_into` use stack-allocated `[i16; 960]` buffers. After the first call, the caller-supplied `Vec` does not grow. libopus itself performs no per-frame heap allocation after `Encoder::new()` / `Decoder::new()`. The `encode()` / `decode_to_vec()` convenience paths each allocate one `Vec` per call and are for tests/examples only.

### Verification

```
cargo fmt --all                                            PASS (1 file reformatted)
cargo clippy --workspace --all-targets -- -D warnings     PASS (0 warnings)
cargo test --workspace                                     PASS (38 tests)
cargo run -p pocketstation-audio --example sine_to_wav    PASS (48000 samples)
```

### Staff Bar Self-Check — Task 1

- Smallest correct design: yes — real encoder wraps libopus directly; legacy aliases avoid churn at all call sites
- Tests added or updated: yes — 4 new codec tests; 5 legacy jitter-buffer tests retained
- Hot-path safe: yes — stack-allocated i16 buffer; no per-frame heap allocation after init
- Public API changed: yes — `OpusEncoder`, `OpusDecoder`, `OPUS_FRAME_SAMPLES`, `OPUS_MAX_PACKET_BYTES`, `OPUS_SAMPLE_RATE` added (additive); `MockOpusEncoder` / `MockOpusDecoder` now wrap real types (compatible)
- New dependency: yes — `opus = "0.3"` approved; wraps `libopus` via `audiopus_sys`
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: `EncodedFrame.payload: Vec<u8>` still forces one allocation per frame in the convenience path; Phase 5 can evaluate pool-backed payloads

---

## Task 2 — ClockSync PI controller (ADR-006)

### Changes

- `crates/pocketstation-bus/src/lib.rs`: replaced exponential-smoother stub with a PI controller:
  - `struct ClockSync { kp: f64, ki: f64, integral: f64, last_offset_ns: i64 }`
  - `ClockSync::new(kp, ki)` and `ClockSync::default()` (kp=0.1, ki=0.001).
  - `tick(measured_offset_ns: i64) -> i64`: proportional + integral, output clamped to ±10 ms (10_000_000 ns).
  - `last_offset_ns()` and `integral()` accessors for diagnostics.
- Old API (`update_pi`, `correction_ratio`, `drift_ppm_estimate`, `target_sample_rate`) removed; tests updated.
- `docs/standards/FAKE_SCAFFOLD_INVENTORY.md`: ClockSync row burned.
- `docs/adr/ADR-006-clock-sync-src.md`: status updated, implementation note added.

### New tests (pocketstation-bus)

- `clock_sync_zero_offset_produces_zero_correction`
- `clock_sync_positive_offset_produces_positive_correction`
- `clock_sync_negative_offset_produces_negative_correction`
- `clock_sync_correction_is_clamped_to_ten_milliseconds`
- `clock_sync_integral_accumulates_across_ticks`

### Verification

```
cargo fmt --all                                            PASS
cargo clippy --workspace --all-targets -- -D warnings     PASS (0 warnings)
cargo test --workspace                                     PASS (38 tests)
```

### Staff Bar Self-Check — Task 2

- Smallest correct design: yes — standard PI; clamp prevents windup on startup
- Tests added or updated: yes — 5 new tests replacing 3 old smoother tests
- Hot-path safe: yes — pure f64 arithmetic, no allocation, no lock
- Public API changed: yes — `tick()`, `new(kp, ki)`, `last_offset_ns()`, `integral()` replace old API (breaking; only internal callers)
- New dependency: no
- Phase scope respected: yes — gains deferred to Phase 5 per ADR-006
- Unsafe added: no
- Remaining risk: gains not validated against real hardware; Phase 5 must measure and retune

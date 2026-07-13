# Phase 2 Progress - PocketStation Runtime

## Repository and timing partition - 2026-07-16

- Renamed the local central workspace from `pocketstation` to `pocketstation`; the
  workspace is the product center while `pks-*` crates keep narrow ownership.
- Removed `media-clock` from the central workspace dependency graph.
- Removed the unrelated `StreamProfile -> media_clock::Contract` mapping from
  `pks-codec`; codec profiles now own codec configuration only.
- Kept drift/correction and the compiled, tested experimental `SegmentGate` in
  `pks-timing`.
- Confirmed network pacing, RTP sequence/timestamp continuity, repair, and RTCP
  clock lineage remain relay media-plane responsibilities; no `pks-playout`
  crate was added.
- Added allocation-stable Opus PLC decoding to `pks-codec` so benchmark and
  receiver code do not need a separate codec wrapper.
- Acceptance: `cargo test -p pks-codec -p pks-timing` passes (32 tests total).
- The neutral benchmark's final `media-clock` dependency was removed: benchmark
  drift uses `pks-timing`, Opus PLC uses `pks-codec`, and reproducibility-only
  reorder/holdback stays private to the harness. Product docs now mark the old
  workspace archived rather than compatibility-active; the remote archive was
  verified 2026-07-16.
- Linux capture now uses the canonical `sample_rate_hz` field on `AudioFrame`
  and `CaptureSource`, closing the cross-platform strict-clippy failure without
  changing capture behavior.
- CI benchmark compilation and the allocation-free integration gate now target
  the canonical `pks-audio` package name instead of the retired
  `pocketstation-audio` name.

## Runtime timing ownership - 2026-07-16

- Added `pks-timing` as the single owner of clock drift estimation and PI clock
  correction.
- Replaced `pks-pipeline`'s duplicate `ClockSync` implementation with the
  runtime-owned controller while retaining a compatibility alias.
- Stopped treating an absolute frame timestamp as a measured clock offset in
  `ResampleNode`; correction now requires an explicit inter-clock observation.
- Preserved the future voice-output interruption state machine as compiled,
  tested `pks_timing::experimental::SegmentGate` code without exposing it as a
  current product feature.
- `media-clock` compatibility wrappers delegate to the new owner; the live
  CLI/codec path has since been decoupled from that workspace.
## Local Whisper connector example - 2026-07-13

- Added `examples/whisper-transcribe` as an example-owned `AsyncNode`; no provider dependency entered first-party crates.
- Binary WAV input crosses the async boundary and text output preserves sequence/timestamp lineage.
- Missing process/model and subprocess crashes fail visibly.
- Real whisper.cpp tiny English E2E passed in CPU mode with a 3.84-second spoken fixture; measured wall time was 1.08 seconds.
- GPU remains explicit opt-in because Homebrew whisper.cpp 1.9.1 crashed in the Metal backend on this machine.
## Bounded captured-frame stream - 2026-07-13

- Added a stable `FnMut(AudioFrame)` capture callback contract across the platform adapters.
- Added a bounded, non-blocking SPSC `CapturedFrameStream` with explicit delivered/drop counters and no hidden runtime.
- Unit tests pass for delivery, overflow, closure, callback adaptation, and invalid capacity.
- Real macOS exact-process capture passed with 281 consumed frames, 287,744 samples, RMS 0.141005, and zero dropped frames.
- All 112 CLI tests pass against the updated capture API.
- The capture-stream example is target-gated so Linux and Windows all-targets
  checks compile without pretending the macOS system-loopback endpoint exists.
- Linux capture tests explicitly reject the stream-capacity setup error so the
  cross-platform `CaptureError` contract remains exhaustively checked.

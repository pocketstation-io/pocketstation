# AUDIO-006-clock-sync-src — Clock Sync / Async Sample Rate Conversion

## Status
Accepted. Runtime timing ownership corrected in Phase 2; product-level tuning and
long-session validation remain open.

## Phase 3 implementation note (2026-05-23)

The PI controller was originally implemented as `pks_pipeline::ClockSync`.
- `tick(measured_offset_ns: i64) -> i64`: computes `kp * error + ki * integral`, clamps to ±10 ms.
- Default gains: `kp = 0.1`, `ki = 0.001` (conservative; tuned in Phase 5 with real measurements).
- The Phase 0 exponential-smoother stub (`update_pi` / `correction_ratio`) has been replaced.

## Phase 2 ownership correction (2026-07-16)

- `pks-timing::ClockCorrectionController` now owns the PI controller.
- `pks-timing::ClockDriftEstimator` owns fixed-window drift measurement using
  source/runtime timestamp pairs normalized against their first observation.
- `pks-pipeline` retains `ClockSync` only as a compatibility alias and no longer
  owns a duplicate controller implementation.
- `ResampleNode` no longer treats an absolute frame timestamp as an inter-clock
  offset. Runtime code must call `observe_clock_offset()` after comparing two
  clock domains.
- The former voice-output Gate is preserved and tested as
  `pks_timing::experimental::SegmentGate`; it is not current product API.

## Context
Independent application and microphone stems use different source clocks. The
runtime needs explicit drift measurement and bounded correction to keep those
stems synchronized without confusing a timestamp with a clock offset.

## Decision
Use PI-controlled linear interpolation for voice default. Keep a hook for higher-quality SRC in music mode.

## Options considered

The historical v2.3 architecture document contains the original option list.

## Consequences

- Agents must follow this decision until a new ADR supersedes it.
- Tests/benchmarks must verify the decision in the relevant phase.

## Test / measurement plan

- Add unit tests for correctness.
- Add benchmark where performance matters.
- Add soak/load tests where reliability matters.

## Reversal trigger

Measured Phase 0/1 data shows this decision breaks latency, reliability, safety, or developer usability targets.

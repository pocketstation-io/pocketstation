# AUDIO-006-clock-sync-src — Clock Sync / Async Sample Rate Conversion

## Status
Accepted. Phase 0, Phase 1, and Phase 3 complete; no reversal triggered.

## Phase 3 implementation note (2026-05-23)

PI controller implemented in `pocketstation-bus::ClockSync`.
- `tick(measured_offset_ns: i64) -> i64`: computes `kp * error + ki * integral`, clamps to ±10 ms.
- Default gains: `kp = 0.1`, `ki = 0.001` (conservative; tuned in Phase 5 with real measurements).
- The Phase 0 exponential-smoother stub (`update_pi` / `correction_ratio`) has been replaced.

## Context
PocketStation v2.3 requires this ADR before implementation lands. See `docs/architecture/pocketstation-v2.3.md`.

## Decision
Use PI-controlled linear interpolation for voice default. Keep a hook for higher-quality SRC in music mode.

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

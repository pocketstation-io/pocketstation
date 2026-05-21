# ADR-006-clock-sync-src — Clock Sync / Async Sample Rate Conversion

## Status
Accepted. Phase 0 and Phase 1 complete; no reversal triggered. Active for Phase 2.

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

# Timing and clocks

<!-- claims: CLM-DOC-013-CAP-001,CLM-DOC-013-SOURCE-001 -->

## What it is

A clock domain identifies the clock that produced a timestamp. `TimelineMapping` correlates that source time with the Session's monotonic timeline, while drift components describe bounded correction.

## Why it exists

Capture devices and processing components do not necessarily advance from the same origin or at the same rate. Explicit domains prevent unrelated timestamps from being compared as if they shared one clock.

## Relationships

- Frame lineage carries the source clock-domain ID and timestamp.
- The Session timeline provides a common correlation origin.
- Drift observations and correction operate on mappings without replacing lineage.

## Invariants and guarantees

- A wall-clock timestamp is not substituted for a clock-domain mapping.
- Unrepresentable timestamp arithmetic returns an absent or typed result.
- Discontinuities advance evidence rather than silently smoothing history.

## When you encounter it

You encounter timing and clocks through its declaration and runtime APIs.

## Use it

- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)
- [Timestamps diverge or discontinuities appear](/docs/troubleshooting/timing.md)

## Scope

- **Map clocks and correct drift.** Map source timestamps into a Session timeline and estimate or correct clock drift.

The scope of **Timing and clocks** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::timing::clock_correction::ClockCorrectionController` | struct | Applies bounded proportional corrections from measured clock offsets without changing lineage. | `src/timing/clock_correction.rs:4` |
| `pocketstation::timing::clock_drift::ClockDriftEstimator` | struct | Estimates source-clock drift from accumulated source and Session timing observations. | `src/timing/clock_drift.rs:10` |
| `pocketstation::timing::clock_drift::ClockDriftSnapshot` | struct | Reports the clock drift snapshot collected at an observation boundary. | `src/timing/clock_drift.rs:4` |
| `pocketstation::timing::timeline_mapping::TimelineMapping` | struct | Correlates the prepared identities and runtime resources for timeline. | `src/timing/timeline_mapping.rs:2` |
| `accumulated_error_ns` | function | Returns the accumulated error nanoseconds held by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:62` |
| `default` | function | Returns the default `ClockCorrectionController` value. | `src/timing/clock_correction.rs:52` |
| `default` | function | Returns the default `ClockDriftEstimator` value. | `src/timing/clock_drift.rs:115` |
| `drift_ppm` | function | Returns the drift ppm associated with `ClockDriftEstimator`. | `src/timing/clock_drift.rs:59` |
| `integral_error_ns` | function | Returns the integral error nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:42` |
| `integral_ns` | function | Returns the integral nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:46` |

## Executable evidence

Executable evidence selected for **Timing and clocks** is limited to each test's recorded setup and assertions:

- `given_aligned_clocks_when_observed_then_drift_is_near_zero` — given aligned clocks when observed then drift is near zero (`src/timing/clock_drift.rs:122`; `test-5c4146a598e6cdc11175`).
- `given_faster_runtime_clock_when_observed_then_drift_is_positive` — given faster runtime clock when observed then drift is positive (`src/timing/clock_drift.rs:132`; `test-24a29769eb9c240f93a1`).
- `given_large_absolute_timestamps_when_observed_then_relative_drift_stays_precise` — given large absolute timestamps when observed then relative drift stays precise (`src/timing/clock_drift.rs:150`; `test-62316896388e623801b8`).
- `given_observations_when_snapshotted_then_lineage_metrics_are_reported` — given observations when snapshotted then lineage metrics are reported (`src/timing/clock_drift.rs:163`; `test-da38283fb00d196f31c4`).
- `given_slower_runtime_clock_when_observed_then_drift_is_negative` — given slower runtime clock when observed then drift is negative (`src/timing/clock_drift.rs:141`; `test-eff93c107acb8107fb7d`).
- `given_earlier_source_timestamp_when_normalized_then_session_delta_is_preserved` — given earlier source timestamp when normalized then session delta is preserved (`src/timing/timeline_mapping.rs:35`; `test-c3b7c9068ca6ad167eb7`).
- `given_later_source_timestamp_when_normalized_then_session_delta_is_preserved` — given later source timestamp when normalized then session delta is preserved (`src/timing/timeline_mapping.rs:28`; `test-f2c4e3765204ee0ffc2e`).
- `given_unrepresentable_timestamp_when_normalized_then_none_is_returned` — given unrepresentable timestamp when normalized then none is returned (`src/timing/timeline_mapping.rs:42`; `test-015542ca1efbea673a8b`).
- `given_large_offset_when_corrected_then_correction_is_clamped` — given large offset when corrected then correction is clamped (`src/timing/clock_correction.rs:78`; `test-dd8301421f8682ad1051`).
- `given_negative_offset_when_corrected_then_correction_is_negative` — given negative offset when corrected then correction is negative (`src/timing/clock_correction.rs:72`; `test-76bb72f1a9ba30dce241`).
- `given_positive_offset_when_corrected_then_correction_is_positive` — given positive offset when corrected then correction is positive (`src/timing/clock_correction.rs:66`; `test-e2db7a2b5cf88c25549c`).
- `given_repeated_offset_when_corrected_then_integral_accumulates` — given repeated offset when corrected then integral accumulates (`src/timing/clock_correction.rs:84`; `test-a6f888162e30bbbe5f60`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Timing API](/docs/reference/timing.md)
- [Timestamps diverge or discontinuities appear](/docs/troubleshooting/timing.md)

## Evidence boundary

The claims on **Timing and clocks** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/timing/timeline_mapping.rs:1-51` (`DIRECT`)
- `src/timing/clock_drift.rs:1-175` (`DIRECT`)

For **Timing and clocks**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

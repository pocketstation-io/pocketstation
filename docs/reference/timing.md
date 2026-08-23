# Timing API

<!-- claims: CLM-REF-005-CAP-001,CLM-REF-005-SOURCE-001 -->

## Scope

- **Map clocks and correct drift.** Map source timestamps into a Session timeline and estimate or correct clock drift.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `pocketstation::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Clock-domain identity for timestamps produced by PocketStation's shared process-wide monotonic clock. | `src/timing/mod.rs:20` |
| `accumulated_error_ns` | function | Returns the accumulated error nanoseconds associated with `ClockDriftEstimator`. | `src/timing/clock_drift.rs:62` |
| `default` | function | Returns the default `ClockCorrectionController` value. | `src/timing/clock_correction.rs:52` |
| `default` | function | Returns the default `ClockDriftEstimator` value. | `src/timing/clock_drift.rs:115` |
| `drift_ppm` | function | Returns the drift ppm associated with `ClockDriftEstimator`. | `src/timing/clock_drift.rs:59` |
| `integral_error_ns` | function | Returns the integral error nanoseconds associated with `ClockCorrectionController`. | `src/timing/clock_correction.rs:42` |
| `integral_ns` | function | Returns the integral nanoseconds associated with `ClockCorrectionController`. | `src/timing/clock_correction.rs:46` |
| `last_correction_ns` | function | Returns the last correction nanoseconds associated with `ClockCorrectionController`. | `src/timing/clock_correction.rs:39` |
| `last_offset_ns` | function | Returns the last offset nanoseconds associated with `ClockCorrectionController`. | `src/timing/clock_correction.rs:36` |
| `new` | function | Creates a new `ClockCorrectionController`. | `src/timing/clock_correction.rs:13` |
| `new` | function | Creates a new `ClockDriftEstimator`. | `src/timing/clock_drift.rs:22` |
| `new` | function | Creates a new `TimelineMapping`. | `src/timing/timeline_mapping.rs:8` |
| `normalize_timestamp_ns` | function | Returns the normalize timestamp nanoseconds associated with `TimelineMapping`. | `src/timing/timeline_mapping.rs:15` |
| `observe` | function | Returns the current observation exposed by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:35` |
| `pocketstation::timing::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain shared by capture, routing, and destination workers. | `src/timing/mod.rs:28` |
| `snapshot` | function | Returns a point-in-time snapshot of `ClockDriftEstimator`. | `src/timing/clock_drift.rs:66` |
| `tick` | function | Applies one measured clock offset to `ClockCorrectionController` and returns the bounded correction. | `src/timing/clock_correction.rs:23` |
| `pocketstation::timing` | module | Timing primitives owned by the PocketStation runtime. | `src/timing/mod.rs:1` |
| `pocketstation::timing::clock_correction::ClockCorrectionController` | struct | Represents clock correction controller in the PocketStation API. | `src/timing/clock_correction.rs:4` |
| `pocketstation::timing::clock_drift::ClockDriftEstimator` | struct | Represents clock drift estimator in the PocketStation API. | `src/timing/clock_drift.rs:10` |
| `pocketstation::timing::clock_drift::ClockDriftSnapshot` | struct | Reports the clock drift snapshot collected at an observation boundary. | `src/timing/clock_drift.rs:4` |
| `pocketstation::timing::timeline_mapping::TimelineMapping` | struct | Represents timeline mapping in the PocketStation API. | `src/timing/timeline_mapping.rs:2` |
| `ClockDriftSnapshot::accumulated_error_ns` | struct_field | Stores the accumulated error value for `ClockDriftSnapshot`, in nanoseconds. | `src/timing/clock_drift.rs:6` |
| `ClockDriftSnapshot::drift_ppm` | struct_field | Stores the drift ppm associated with `ClockDriftSnapshot`. | `src/timing/clock_drift.rs:5` |
| `ClockDriftSnapshot::observed_samples_count` | struct_field | Stores the number of observed samples represented by `ClockDriftSnapshot`. | `src/timing/clock_drift.rs:7` |
| `TimelineMapping::session_origin_ns` | struct_field | Stores the session origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:4` |
| `TimelineMapping::source_origin_ns` | struct_field | Stores the source origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:3` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Timing and clocks](/docs/concepts/timing-and-clocks.md)
- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Timestamps diverge or discontinuities appear](/docs/troubleshooting/timing.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/timing/mod.rs:1-49` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

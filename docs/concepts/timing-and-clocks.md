# Timing and clocks

<!-- claims: CLM-DOC-013-SCOPE-001,CLM-DOC-013-TEXT-001,CLM-DOC-013-TEXT-002,CLM-DOC-013-TEXT-003,CLM-DOC-013-TEXT-004,CLM-DOC-013-TEXT-005,CLM-DOC-013-TEXT-006,CLM-DOC-013-SOURCE-001 -->

## What it is

A clock domain identifies the authority and origin that produced a timestamp. `describe_clock_domain` distinguishes unspecified, PocketStation process-monotonic, and provider-defined domains; `TimelineMapping` correlates source time with the Session timeline while drift components describe bounded correction.

## Why it exists

Capture devices and processing components do not necessarily advance from the same origin or at the same rate. Explicit domains prevent unrelated timestamps from being compared as if they shared one clock.

## Relationships

- Frame lineage carries the source clock-domain ID and timestamp.
- `SessionTimelineOrigin` and monotonic nanosecond observations provide the common Session correlation origin.
- Route enqueue, route receive, endpoint enqueue, and application poll timestamps expose distinct delivery boundaries.
- Drift observations and correction operate on mappings without replacing lineage.

## Invariants and guarantees

- Clock-domain ID zero declares no clock authority; ID one is PocketStation's process-monotonic nanosecond clock; other IDs preserve provider-defined origin rather than inventing an epoch.
- A wall-clock timestamp is not substituted for a clock-domain mapping.
- Delivery-boundary timestamps are observations, not a universal end-to-end latency guarantee.
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

- `given_aligned_clocks_when_observed_then_drift_is_near_zero` — given aligned clocks when observed then drift is near zero (`src/timing/clock_drift.rs:122`; `test-6157a8f188bed0df7cf2`).
- `given_faster_runtime_clock_when_observed_then_drift_is_positive` — given faster runtime clock when observed then drift is positive (`src/timing/clock_drift.rs:132`; `test-369a4cb1f110b73815ed`).
- `given_large_absolute_timestamps_when_observed_then_relative_drift_stays_precise` — given large absolute timestamps when observed then relative drift stays precise (`src/timing/clock_drift.rs:150`; `test-7eab6b2397d7f488801e`).
- `given_observations_when_snapshotted_then_lineage_metrics_are_reported` — given observations when snapshotted then lineage metrics are reported (`src/timing/clock_drift.rs:163`; `test-ddeaa98b011c7a32a19f`).
- `given_slower_runtime_clock_when_observed_then_drift_is_negative` — given slower runtime clock when observed then drift is negative (`src/timing/clock_drift.rs:141`; `test-b6b0a57d7df5bfbceeee`).
- `given_earlier_source_timestamp_when_normalized_then_session_delta_is_preserved` — given earlier source timestamp when normalized then session delta is preserved (`src/timing/timeline_mapping.rs:35`; `test-8e4d29c9c42209f5f7a1`).
- `given_later_source_timestamp_when_normalized_then_session_delta_is_preserved` — given later source timestamp when normalized then session delta is preserved (`src/timing/timeline_mapping.rs:28`; `test-07659b10df7191d01261`).
- `given_unrepresentable_timestamp_when_normalized_then_none_is_returned` — given unrepresentable timestamp when normalized then none is returned (`src/timing/timeline_mapping.rs:42`; `test-fb2b6038745ee53f69bc`).
- `given_known_and_provider_clocks_when_described_then_authority_is_not_inferred` — given known and provider clocks when described then authority is not inferred (`src/timing/domain.rs:79`; `test-cd56ec4c57a1c9296423`).
- `given_large_offset_when_corrected_then_correction_is_clamped` — given large offset when corrected then correction is clamped (`src/timing/clock_correction.rs:78`; `test-850f66e4215d912e2b1c`).
- `given_negative_offset_when_corrected_then_correction_is_negative` — given negative offset when corrected then correction is negative (`src/timing/clock_correction.rs:72`; `test-47889c770a30d6bc439a`).
- `given_positive_offset_when_corrected_then_correction_is_positive` — given positive offset when corrected then correction is positive (`src/timing/clock_correction.rs:66`; `test-ae66022ac6af9b851bff`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Timing API](/docs/reference/timing.md)
- [Timestamps diverge or discontinuities appear](/docs/troubleshooting/timing.md)

## Evidence boundary

The claims on **Timing and clocks** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/timing/timeline_mapping.rs:1-1` (`DIRECT`)
- `src/timing/timeline_mapping.rs:1-1` (`DIRECT`)
- `src/timing/timeline_mapping.rs:1-1` (`DIRECT`)
- `src/timing/timeline_mapping.rs:2-5` (`DIRECT`)
- `src/timing/timeline_mapping.rs:3-3` (`DIRECT`)
- `src/timing/timeline_mapping.rs:4-4` (`DIRECT`)
- `src/timing/timeline_mapping.rs:8-13` (`DIRECT`)
- `src/timing/timeline_mapping.rs:15-23` (`DIRECT`)
- `src/timing/clock_drift.rs:1-1` (`DIRECT`)
- `src/timing/clock_drift.rs:3-3` (`DIRECT`)
- `src/timing/clock_drift.rs:3-3` (`DIRECT`)
- `src/timing/clock_drift.rs:3-3` (`DIRECT`)
- `src/timing/clock_drift.rs:3-3` (`DIRECT`)
- `src/timing/clock_drift.rs:4-8` (`DIRECT`)
- `src/timing/clock_drift.rs:5-5` (`DIRECT`)
- `src/timing/clock_drift.rs:6-6` (`DIRECT`)
- `src/timing/clock_drift.rs:7-7` (`DIRECT`)
- `src/timing/clock_drift.rs:10-19` (`DIRECT`)
- `src/timing/clock_drift.rs:11-11` (`DIRECT`)
- `src/timing/clock_drift.rs:12-12` (`DIRECT`)
- `src/timing/clock_drift.rs:13-13` (`DIRECT`)
- `src/timing/clock_drift.rs:14-14` (`DIRECT`)
- `src/timing/clock_drift.rs:15-15` (`DIRECT`)
- `src/timing/clock_drift.rs:16-16` (`DIRECT`)
- `src/timing/clock_drift.rs:17-17` (`DIRECT`)
- `src/timing/clock_drift.rs:18-18` (`DIRECT`)
- `src/timing/clock_drift.rs:22-33` (`DIRECT`)
- `src/timing/clock_drift.rs:35-57` (`DIRECT`)
- `src/timing/clock_drift.rs:59-61` (`DIRECT`)
- `src/timing/clock_drift.rs:62-64` (`DIRECT`)
- `src/timing/clock_drift.rs:66-72` (`DIRECT`)
- `src/timing/clock_drift.rs:74-111` (`DIRECT`)

For **Timing and clocks**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

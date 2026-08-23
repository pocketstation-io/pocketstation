# Timing API

<!-- claims: CLM-REF-005-CAP-001,CLM-REF-005-SOURCE-001 -->

## Scope

- **Map clocks and correct drift.** Map source timestamps into a Session timeline and estimate or correct clock drift.

The scope of **Timing API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Timing API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-8c9708e9a20bf5143276 | `pocketstation::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Clock-domain identity for timestamps produced by PocketStation's shared process-wide monotonic clock. | `src/timing/mod.rs:20` |
| sym-2ba2c6e834e0e56d1b87 | `accumulated_error_ns` | function | Returns the accumulated error nanoseconds held by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:62` |
| sym-0b26dc8e32052d6f3db2 | `default` | function | Returns the default `ClockCorrectionController` value. | `src/timing/clock_correction.rs:52` |
| sym-7a31f0974065a9809fd1 | `default` | function | Returns the default `ClockDriftEstimator` value. | `src/timing/clock_drift.rs:115` |
| sym-28a178cc9d8d140d32f1 | `drift_ppm` | function | Returns the drift ppm associated with `ClockDriftEstimator`. | `src/timing/clock_drift.rs:59` |
| sym-79881e8872c141336a40 | `integral_error_ns` | function | Returns the integral error nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:42` |
| sym-4dda4bf993c35495f761 | `integral_ns` | function | Returns the integral nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:46` |
| sym-e748db1c1f6bdf4274bb | `last_correction_ns` | function | Returns the last correction nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:39` |
| sym-f1c643a525e98a619677 | `last_offset_ns` | function | Returns the last offset nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:36` |
| sym-73e2ad6f978a471dbe7c | `new` | function | Creates a new `ClockCorrectionController`. | `src/timing/clock_correction.rs:13` |
| sym-2a3ac821bd95a8e595a5 | `new` | function | Creates a new `ClockDriftEstimator`. | `src/timing/clock_drift.rs:22` |
| sym-467efaaa4788a0962c0d | `new` | function | Creates a new `TimelineMapping`. | `src/timing/timeline_mapping.rs:8` |
| sym-50b92d4050c599fafa35 | `normalize_timestamp_ns` | function | Returns the normalize timestamp nanoseconds held by `TimelineMapping`. | `src/timing/timeline_mapping.rs:15` |
| sym-c0a379cb4e9dac135996 | `observe` | function | Returns the current observation exposed by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:35` |
| sym-b4e20b5cb9711c373847 | `pocketstation::timing::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain shared by capture, routing, and destination workers. | `src/timing/mod.rs:28` |
| sym-4ab916f93bbc4bf11a2b | `snapshot` | function | Returns a point-in-time snapshot of `ClockDriftEstimator`. | `src/timing/clock_drift.rs:66` |
| sym-191791f5a9e29a495b2d | `tick` | function | Applies one measured clock offset to `ClockCorrectionController` and returns the bounded correction. | `src/timing/clock_correction.rs:23` |
| sym-2d75d52c1a60afe46936 | `pocketstation::timing` | module | Timing primitives owned by the PocketStation runtime. | `src/timing/mod.rs:1` |
| sym-bd7efbabb0ea9370bb5c | `pocketstation::timing::clock_correction::ClockCorrectionController` | struct | Applies bounded proportional corrections from measured clock offsets without changing lineage. | `src/timing/clock_correction.rs:4` |
| sym-35fb3cce537ca1a2170d | `pocketstation::timing::clock_drift::ClockDriftEstimator` | struct | Estimates source-clock drift from accumulated source and Session timing observations. | `src/timing/clock_drift.rs:10` |
| sym-9313cfa91ee45d8cb05e | `pocketstation::timing::clock_drift::ClockDriftSnapshot` | struct | Reports the clock drift snapshot collected at an observation boundary. | `src/timing/clock_drift.rs:4` |
| sym-9fcb4fc3036c0fad6c1c | `pocketstation::timing::timeline_mapping::TimelineMapping` | struct | Correlates the prepared identities and runtime resources for timeline. | `src/timing/timeline_mapping.rs:2` |
| sym-ad76e0a22e34ff04feb5 | `ClockDriftSnapshot::accumulated_error_ns` | struct_field | Stores the accumulated error value for `ClockDriftSnapshot`, in nanoseconds. | `src/timing/clock_drift.rs:6` |
| sym-75e5bfb2fadc319c5ebd | `ClockDriftSnapshot::drift_ppm` | struct_field | Stores the drift ppm used by `ClockDriftSnapshot`. | `src/timing/clock_drift.rs:5` |
| sym-01ef1ec0849935f2decc | `ClockDriftSnapshot::observed_samples_count` | struct_field | Stores the number of observed samples represented by `ClockDriftSnapshot`. | `src/timing/clock_drift.rs:7` |
| sym-3da084bf16fb7475c2eb | `TimelineMapping::session_origin_ns` | struct_field | Stores the session origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:4` |
| sym-b074cd38dd530f8d721c | `TimelineMapping::source_origin_ns` | struct_field | Stores the source origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:3` |

## Interpretation

The **Timing API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Timing and clocks](/docs/concepts/timing-and-clocks.md)
- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Timestamps diverge or discontinuities appear](/docs/troubleshooting/timing.md)

## Evidence boundary

The claims on **Timing API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/timing/mod.rs:1-49` (`DIRECT`)

For **Timing API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

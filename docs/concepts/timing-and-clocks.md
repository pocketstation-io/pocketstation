# Timing and clocks

<!-- claims: CLM-DOC-013-CAP-001,CLM-DOC-013-SOURCE-001 -->

Map source timestamps into a Session timeline and estimate or correct clock drift.

## Scope

- **Map clocks and correct drift.** Map source timestamps into a Session timeline and estimate or correct clock drift.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::timing` | module | Timing primitives owned by the PocketStation runtime. | `src/timing/mod.rs:1` |
| `pocketstation::timing::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain shared by capture, routing, and destination workers. | `src/timing/mod.rs:28` |
| `pocketstation::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Clock-domain identity for timestamps produced by PocketStation's shared process-wide monotonic clock. | `src/timing/mod.rs:20` |
| `pocketstation::timing::clock_correction::ClockCorrectionController` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_correction.rs:4` |
| `pocketstation::timing::clock_drift::ClockDriftEstimator` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_drift.rs:10` |
| `pocketstation::timing::clock_drift::ClockDriftSnapshot` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_drift.rs:4` |
| `pocketstation::timing::timeline_mapping::TimelineMapping` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/timeline_mapping.rs:2` |
| `accumulated_error_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_drift.rs:62` |
| `default` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_correction.rs:52` |
| `default` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_drift.rs:115` |
| `drift_ppm` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_drift.rs:59` |
| `integral_error_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_correction.rs:42` |
| `integral_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_correction.rs:46` |
| `last_correction_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_correction.rs:39` |
| `last_offset_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_correction.rs:36` |
| `new` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_correction.rs:13` |
| `new` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_drift.rs:22` |
| `new` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/timeline_mapping.rs:8` |
| `normalize_timestamp_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/timeline_mapping.rs:15` |
| `observe` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/clock_drift.rs:35` |

## Where you encounter it

The current capability model has no separate end-to-end journey for this concept.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_aligned_clocks_when_observed_then_drift_is_near_zero` — given aligned clocks when observed then drift is near zero (`src/timing/clock_drift.rs:122`; `test-5c4146a598e6cdc11175`).
- `given_large_offset_when_corrected_then_correction_is_clamped` — given large offset when corrected then correction is clamped (`src/timing/clock_correction.rs:78`; `test-dd8301421f8682ad1051`).
- `given_negative_offset_when_corrected_then_correction_is_negative` — given negative offset when corrected then correction is negative (`src/timing/clock_correction.rs:72`; `test-76bb72f1a9ba30dce241`).
- `given_positive_offset_when_corrected_then_correction_is_positive` — given positive offset when corrected then correction is positive (`src/timing/clock_correction.rs:66`; `test-e2db7a2b5cf88c25549c`).
- `given_repeated_offset_when_corrected_then_integral_accumulates` — given repeated offset when corrected then integral accumulates (`src/timing/clock_correction.rs:84`; `test-a6f888162e30bbbe5f60`).
- `given_zero_offset_when_corrected_then_correction_is_zero` — given zero offset when corrected then correction is zero (`src/timing/clock_correction.rs:59`; `test-3d8bfc2db8ae855b33a9`).
- `given_faster_runtime_clock_when_observed_then_drift_is_positive` — given faster runtime clock when observed then drift is positive (`src/timing/clock_drift.rs:132`; `test-24a29769eb9c240f93a1`).
- `given_large_absolute_timestamps_when_observed_then_relative_drift_stays_precise` — given large absolute timestamps when observed then relative drift stays precise (`src/timing/clock_drift.rs:150`; `test-62316896388e623801b8`).
- `given_observations_when_snapshotted_then_lineage_metrics_are_reported` — given observations when snapshotted then lineage metrics are reported (`src/timing/clock_drift.rs:163`; `test-da38283fb00d196f31c4`).
- `given_slower_runtime_clock_when_observed_then_drift_is_negative` — given slower runtime clock when observed then drift is negative (`src/timing/clock_drift.rs:141`; `test-eff93c107acb8107fb7d`).
- `given_shared_monotonic_clock_when_sampled_then_value_never_moves_backwards` — given shared monotonic clock when sampled then value never moves backwards (`src/timing/mod.rs:38`; `test-57367813fb8f3e9a505c`).
- `given_earlier_source_timestamp_when_normalized_then_session_delta_is_preserved` — given earlier source timestamp when normalized then session delta is preserved (`src/timing/timeline_mapping.rs:35`; `test-c3b7c9068ca6ad167eb7`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Timing API](/docs/reference/timing.md)
- [Timestamps diverge or discontinuities appear](/docs/troubleshooting/timing.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/timing/timeline_mapping.rs:1-51` (`DIRECT`)
- `src/timing/clock_drift.rs:1-175` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

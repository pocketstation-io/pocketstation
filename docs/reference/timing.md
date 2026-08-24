# Timing API

<!-- claims: CLM-REF-005-SCOPE-001,CLM-REF-005-TEXT-001,CLM-REF-005-TEXT-002,CLM-REF-005-SOURCE-001 -->

## Scope

- **Map clocks and correct drift.** Map source timestamps into a Session timeline and estimate or correct clock drift.

The scope of **Timing API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Timing API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
| sym-7e140bdb0b67db980cf5 | `pocketstation::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Clock-domain identity for timestamps produced by PocketStation's shared process-wide monotonic clock. | `src/timing/mod.rs:24` |
| sym-b3e6577a839ba5232766 | `pocketstation::timing::domain::ClockDomainKind` | enum | The authority that defines timestamps carried by one clock-domain ID. | `src/timing/domain.rs:7` |
| sym-f73c46c82e8ef2c1787d | `pocketstation::timing::domain::ClockDomainOrigin` | enum | The origin against which timestamps in one clock domain are measured. | `src/timing/domain.rs:15` |
| sym-e9bc18581c3e02587015 | `accumulated_error_ns` | function | Returns the accumulated error nanoseconds held by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:62` |
| sym-db7c11406c22fe9249af | `default` | function | Returns the default `ClockCorrectionController` value. | `src/timing/clock_correction.rs:52` |
| sym-afa922b78c896e4ffb57 | `default` | function | Returns the default `ClockDriftEstimator` value. | `src/timing/clock_drift.rs:115` |
| sym-e2f1dc5f386c4947fe93 | `drift_ppm` | function | Returns the drift ppm associated with `ClockDriftEstimator`. | `src/timing/clock_drift.rs:59` |
| sym-311cba5892f020a3e297 | `id` | function | Returns the id held by `ClockDomainDescriptor`. | `src/timing/domain.rs:31` |
| sym-b5be27424500bdeaf8ac | `integral_error_ns` | function | Returns the integral error nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:42` |
| sym-4d54a78d3c364c587267 | `integral_ns` | function | Returns the integral nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:46` |
| sym-c7bb8ee02a040b352a18 | `kind` | function | Returns the kind represented by `ClockDomainDescriptor`. | `src/timing/domain.rs:35` |
| sym-4b74678cd492f3180b81 | `last_correction_ns` | function | Returns the last correction nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:39` |
| sym-7047cde0c35ee9a4eebd | `last_offset_ns` | function | Returns the last offset nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:36` |
| sym-2059252abf3a743c704d | `new` | function | Creates a new `ClockCorrectionController`. | `src/timing/clock_correction.rs:13` |
| sym-433c1f9402adccdef4f0 | `new` | function | Creates a new `ClockDriftEstimator`. | `src/timing/clock_drift.rs:22` |
| sym-0d08ae9c5f3506a6addd | `new` | function | Creates a new `TimelineMapping`. | `src/timing/timeline_mapping.rs:8` |
| sym-97f49aec5a4257d6a8a5 | `normalize_timestamp_ns` | function | Returns the normalize timestamp nanoseconds held by `TimelineMapping`. | `src/timing/timeline_mapping.rs:15` |
| sym-5e431e856805d2224f11 | `observe` | function | Returns the current observation exposed by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:35` |
| sym-e8cf98b0bcffc28ac74f | `origin` | function | Returns the origin held by `ClockDomainDescriptor`. | `src/timing/domain.rs:39` |
| sym-f0bb36b243ff1edb3d1e | `pocketstation::timing::domain::describe_clock_domain` | function | Describes the stable semantics Core can assert for a clock-domain ID. | `src/timing/domain.rs:54` |
| sym-50908e7003d52b4da731 | `pocketstation::timing::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain shared by capture, routing, and destination workers. | `src/timing/mod.rs:32` |
| sym-650fc92c17cdabab7bd9 | `snapshot` | function | Returns a point-in-time snapshot of `ClockDriftEstimator`. | `src/timing/clock_drift.rs:66` |
| sym-9def9300f09440ab219b | `tick` | function | Applies one measured clock offset to `ClockCorrectionController` and returns the bounded correction. | `src/timing/clock_correction.rs:23` |
| sym-7d275195264aacbf43e2 | `tick_rate_hz` | function | Returns the tick rate hertz held by `ClockDomainDescriptor`. | `src/timing/domain.rs:43` |
| sym-46f35620ed864442b2a0 | `pocketstation::timing` | module | Timing primitives owned by the PocketStation runtime. | `src/timing/mod.rs:1` |
| sym-ae405074cfb9d4d5b291 | `pocketstation::timing::clock_correction::ClockCorrectionController` | struct | Applies bounded proportional corrections from measured clock offsets without changing lineage. | `src/timing/clock_correction.rs:4` |
| sym-64a18bac4bce29e0a26c | `pocketstation::timing::clock_drift::ClockDriftEstimator` | struct | Estimates source-clock drift from accumulated source and Session timing observations. | `src/timing/clock_drift.rs:10` |
| sym-9982b0b272411ab60fb6 | `pocketstation::timing::clock_drift::ClockDriftSnapshot` | struct | Reports the clock drift snapshot collected at an observation boundary. | `src/timing/clock_drift.rs:4` |
| sym-d78fbc7a6425539ac041 | `pocketstation::timing::domain::ClockDomainDescriptor` | struct | Finite description of a clock identity carried by frame and signal lineage. | `src/timing/domain.rs:23` |
| sym-8cb95ccfe2d318c37c6b | `pocketstation::timing::timeline_mapping::TimelineMapping` | struct | Correlates the prepared identities and runtime resources for timeline. | `src/timing/timeline_mapping.rs:2` |
| sym-a04ca7a73c6d365d0091 | `ClockDriftSnapshot::accumulated_error_ns` | struct_field | Stores the accumulated error value for `ClockDriftSnapshot`, in nanoseconds. | `src/timing/clock_drift.rs:6` |
| sym-3302deaa63c3bb51ac21 | `ClockDriftSnapshot::drift_ppm` | struct_field | Reports the estimated clock drift for `ClockDriftSnapshot`, in parts per million. | `src/timing/clock_drift.rs:5` |
| sym-50189778197fe2717210 | `ClockDriftSnapshot::observed_samples_count` | struct_field | Stores the number of observed samples represented by `ClockDriftSnapshot`. | `src/timing/clock_drift.rs:7` |
| sym-03e0e3607e9d368df095 | `TimelineMapping::session_origin_ns` | struct_field | Stores the session origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:4` |
| sym-addb257b22be852d57aa | `TimelineMapping::source_origin_ns` | struct_field | Stores the source origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:3` |
| sym-99b3c55887e45a315fed | `pocketstation::timing::domain::ClockDomainKind::ProcessMonotonic` | variant | Identifies timestamps as belonging to the process monotonic clock domain. | `src/timing/domain.rs:9` |
| sym-90786f825231e7b95285 | `pocketstation::timing::domain::ClockDomainKind::ProviderDefined` | variant | Identifies timestamps as belonging to the provider defined clock domain. | `src/timing/domain.rs:10` |
| sym-d9dd4700e264a084f4c8 | `pocketstation::timing::domain::ClockDomainKind::Unspecified` | variant | Identifies timestamps as belonging to the unspecified clock domain. | `src/timing/domain.rs:8` |
| sym-ea93bf173163a9225a06 | `pocketstation::timing::domain::ClockDomainOrigin::ProcessStart` | variant | Represents the process start alternative defined by `ClockDomainOrigin`. | `src/timing/domain.rs:17` |
| sym-4d8df7f26a599285aa84 | `pocketstation::timing::domain::ClockDomainOrigin::ProviderDefined` | variant | Represents the provider defined alternative defined by `ClockDomainOrigin`. | `src/timing/domain.rs:18` |
| sym-f9596f17e9b3277c3bec | `pocketstation::timing::domain::ClockDomainOrigin::Unspecified` | variant | Represents the unspecified alternative defined by `ClockDomainOrigin`. | `src/timing/domain.rs:16` |

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

The claims on **Timing API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/timing/mod.rs:1-3` (`DECLARED`)

For **Timing API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

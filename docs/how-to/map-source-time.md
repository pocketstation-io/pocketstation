# Map source time into the Session timeline

<!-- claims: CLM-GUIDE-029-CAP-001,CLM-GUIDE-029-CAP-002,CLM-GUIDE-029-SOURCE-001 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Map clocks and correct drift.** Map source timestamps into a Session timeline and estimate or correct clock drift.

The scope of **Map source time into the Session timeline** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A stable source clock-domain ID, source timestamp, Session timestamp, and discontinuity policy.

## Procedure

1. Retain source clock-domain identity and timestamp.
2. Update TimelineMapping with observed source and Session time.
3. Map into the Session domain.
4. Observe drift and discontinuity without rewriting lineage.
5. Apply correction only through evidenced controller bounds.

## Important consequence

Preserve original lineage even when a controller applies a bounded correction.

## Verify the outcome

Mapped timestamps remain representable and drift observations stay within the correction behavior you selected.

Executable evidence selected for **Map source time into the Session timeline** is limited to each test's recorded setup and assertions:

- `given_earlier_source_timestamp_when_normalized_then_session_delta_is_preserved` — given earlier source timestamp when normalized then session delta is preserved (`src/timing/timeline_mapping.rs:35`; `test-8e4d29c9c42209f5f7a1`).
- `given_later_source_timestamp_when_normalized_then_session_delta_is_preserved` — given later source timestamp when normalized then session delta is preserved (`src/timing/timeline_mapping.rs:28`; `test-07659b10df7191d01261`).
- `given_unrepresentable_timestamp_when_normalized_then_none_is_returned` — given unrepresentable timestamp when normalized then none is returned (`src/timing/timeline_mapping.rs:42`; `test-fb2b6038745ee53f69bc`).
- `given_frame_lineage_when_timestamp_end_requested_then_duration_is_saturating` — given frame lineage when timestamp end requested then duration is saturating (`src/frame/audio.rs:458`; `test-ae12619209044ccec1dc`).
- `given_faster_runtime_clock_when_observed_then_drift_is_positive` — given faster runtime clock when observed then drift is positive (`src/timing/clock_drift.rs:132`; `test-369a4cb1f110b73815ed`).
- `given_large_absolute_timestamps_when_observed_then_relative_drift_stays_precise` — given large absolute timestamps when observed then relative drift stays precise (`src/timing/clock_drift.rs:150`; `test-7eab6b2397d7f488801e`).
- `given_slower_runtime_clock_when_observed_then_drift_is_negative` — given slower runtime clock when observed then drift is negative (`src/timing/clock_drift.rs:141`; `test-b6b0a57d7df5bfbceeee`).
- `given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960` — given 48khz mono spec when frame samples for 20ms then returns 960 (`src/frame/audio.rs:434`; `test-bc27ef56fe4e052f18d1`).
- `given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920` — given 48khz stereo spec when frame samples for 20ms then returns 1920 (`src/frame/audio.rs:446`; `test-aaeb53c9b93d7d667a32`).
- `given_acquired_handle_when_copy_from_slice_then_length_matches_data` — given acquired handle when copy from slice then length matches data (`src/frame/audio.rs:378`; `test-49f5dd0943c01f09e6af`).
- `given_exhausted_pool_when_handle_dropped_then_reacquire_succeeds` — given exhausted pool when handle dropped then reacquire succeeds (`src/frame/audio.rs:407`; `test-7d64ba1deb4442d9cd6d`).
- `given_frozen_buffer_with_many_consumers_when_final_handle_drops_then_slot_is_reused` — given frozen buffer with many consumers when final handle drops then slot is reused (`src/frame/audio.rs:536`; `test-eee9fd42fc6e945a499c`).

## Failure signals

- `pocketstation::frame::audio::FrameLineageError` / `Source` — `error-c220db0d4b0d7f6df540`
- `pocketstation::frame::audio::FrameLineageError` / `Timestamp` — `error-5999f78167ca350ab77a`
- `pocketstation::frame::lineage::FrameLineageBuildError` / `TimestampOverflow` — `error-797d18adab9af15852b8`
- `pocketstation::frame::lineage::FrameLineageBuildError` / `ZeroSourceGeneration` — `error-8bad0cabba73294a3557`

## API reference

- [Timing And Clocks](/docs/concepts/timing-and-clocks.md)
- [Timing](/docs/reference/timing.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `TimelineMapping::session_origin_ns` | struct_field | Stores the session origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:4` |
| `TimelineMapping::source_origin_ns` | struct_field | Stores the source origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:3` |
| `pocketstation::timing::timeline_mapping::TimelineMapping` | struct | Correlates the prepared identities and runtime resources for timeline. | `src/timing/timeline_mapping.rs:2` |
| `pocketstation::frame::identity::SessionId` | struct | Uniquely identifies session within its PocketStation ownership scope. | `src/frame/identity.rs:22` |
| `pocketstation::frame::identity::SourceId` | struct | Uniquely identifies source within its PocketStation ownership scope. | `src/frame/identity.rs:21` |
| `into_parts` | function | Consumes `LineagedAudioFrame` and returns its component values. | `src/frame/audio.rs:285` |
| `normalize_timestamp_ns` | function | Returns the normalize timestamp nanoseconds held by `TimelineMapping`. | `src/timing/timeline_mapping.rs:15` |
| `session_id` | function | Returns the session identifier held by `FrameLineage`. | `src/frame/lineage.rs:56` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Timestamps diverge or discontinuities appear](/docs/troubleshooting/timing.md)
- [Frame identity and lineage](/docs/concepts/frame-lineage.md)
- [Memory ownership and buffer pools](/docs/internals/memory-ownership.md)
- [Realtime audio lane](/docs/internals/realtime-audio-lane.md)

## Evidence boundary

The claims on **Map source time into the Session timeline** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/timing/timeline_mapping.rs:1-51` (`DIRECT`)

For **Map source time into the Session timeline**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

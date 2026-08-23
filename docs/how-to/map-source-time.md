# Map source time into the Session timeline

<!-- claims: CLM-GUIDE-029-CAP-001,CLM-GUIDE-029-CAP-002,CLM-GUIDE-029-SOURCE-001 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Map clocks and correct drift.** Map source timestamps into a Session timeline and estimate or correct clock drift.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Retain source clock-domain identity and timestamp.
2. Update TimelineMapping with observed source and Session time.
3. Map into the Session domain.
4. Observe drift and discontinuity without rewriting lineage.
5. Apply correction only through evidenced controller bounds.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `TimelineMapping::session_origin_ns` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/timeline_mapping.rs:4` |
| `TimelineMapping::source_origin_ns` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/timeline_mapping.rs:3` |
| `pocketstation::timing::timeline_mapping::TimelineMapping` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/timeline_mapping.rs:2` |
| `pocketstation::timing::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain shared by capture, routing, and destination workers. | `src/timing/mod.rs:28` |
| `pocketstation::frame::identity::SessionId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:22` |
| `pocketstation::frame::identity::SourceId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:21` |
| `normalize_timestamp_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/timing/timeline_mapping.rs:15` |
| `session_id` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:56` |
| `source_generation` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:77` |
| `source_id` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:122` |
| `source_id` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:192` |
| `source_id` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:59` |
| `timestamp_end_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:87` |
| `timestamp_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:138` |
| `timestamp_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:208` |
| `timestamp_start_ns` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:71` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_earlier_source_timestamp_when_normalized_then_session_delta_is_preserved` — given earlier source timestamp when normalized then session delta is preserved (`src/timing/timeline_mapping.rs:35`; `test-c3b7c9068ca6ad167eb7`).
- `given_later_source_timestamp_when_normalized_then_session_delta_is_preserved` — given later source timestamp when normalized then session delta is preserved (`src/timing/timeline_mapping.rs:28`; `test-f2c4e3765204ee0ffc2e`).
- `given_frame_lineage_when_timestamp_end_requested_then_duration_is_saturating` — given frame lineage when timestamp end requested then duration is saturating (`src/frame/audio.rs:458`; `test-f691cb19b7818f0469d6`).
- `given_faster_runtime_clock_when_observed_then_drift_is_positive` — given faster runtime clock when observed then drift is positive (`src/timing/clock_drift.rs:132`; `test-24a29769eb9c240f93a1`).
- `given_large_absolute_timestamps_when_observed_then_relative_drift_stays_precise` — given large absolute timestamps when observed then relative drift stays precise (`src/timing/clock_drift.rs:150`; `test-62316896388e623801b8`).
- `given_slower_runtime_clock_when_observed_then_drift_is_negative` — given slower runtime clock when observed then drift is negative (`src/timing/clock_drift.rs:141`; `test-eff93c107acb8107fb7d`).
- `given_unrepresentable_timestamp_when_normalized_then_none_is_returned` — given unrepresentable timestamp when normalized then none is returned (`src/timing/timeline_mapping.rs:42`; `test-015542ca1efbea673a8b`).
- `given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960` — given 48khz mono spec when frame samples for 20ms then returns 960 (`src/frame/audio.rs:434`; `test-bf95ac4b2316d447ed6b`).
- `given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920` — given 48khz stereo spec when frame samples for 20ms then returns 1920 (`src/frame/audio.rs:446`; `test-27034bea6e0bcfc0b91b`).
- `given_acquired_handle_when_copy_from_slice_then_length_matches_data` — given acquired handle when copy from slice then length matches data (`src/frame/audio.rs:378`; `test-dde445e05c14558c788a`).
- `given_exhausted_pool_when_handle_dropped_then_reacquire_succeeds` — given exhausted pool when handle dropped then reacquire succeeds (`src/frame/audio.rs:407`; `test-931c3c8a724375d8c6e5`).
- `given_frozen_buffer_with_many_consumers_when_final_handle_drops_then_slot_is_reused` — given frozen buffer with many consumers when final handle drops then slot is reused (`src/frame/audio.rs:536`; `test-5df473b160e87b1a3092`).

## Failure signals

- `pocketstation::frame::pool::AudioBufferWriteError` / `CapacityExceeded` — `error-2317926ecc3df1fe0485`
- `pocketstation::frame::lineage::FrameLineageBuildError` / `ZeroSourceGeneration` — `error-2333fb8ed9ffc64dfe3d`
- `pocketstation::frame::lineage::FrameLineageBuildError` / `ZeroDuration` — `error-36112cc71bb577df5cc6`
- `pocketstation::frame::audio::AudioFrameBuildError` / `ZeroSampleRate` — `error-3d530ffcc82f2ae60152`
- `pocketstation::frame::pool::AudioBufferWriteError` — `error-44d619f15116bb8d5f0e`
- `pocketstation::frame::audio::AudioFrameBuildError` — `error-47bd33a1cf3d0c5fa264`
- `pocketstation::frame::lineage::FrameLineageBuildError` — `error-886f021bf510039ccdbb`
- `pocketstation::frame::lineage::FrameLineageBuildError` / `TimestampOverflow` — `error-bd9d2580f5c500ca2920`
- `pocketstation::frame::audio::AudioFrameBuildError` / `ZeroChannels` — `error-ec0790bb6edfcc3d5058`
- `pocketstation::frame::audio::AudioFrameBuildError` / `MisalignedSamples` — `error-fd6606b3c0707d21bb0f`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/timing/timeline_mapping.rs:1-51` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

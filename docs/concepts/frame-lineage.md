# Frame identity and lineage

<!-- claims: CLM-DOC-012-CAP-001,CLM-DOC-012-SOURCE-001 -->

Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::frame::identity::ClockDomainId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:29` |
| `pocketstation::frame::identity::ConnectorId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:25` |
| `pocketstation::frame::identity::EndpointId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:24` |
| `pocketstation::frame::identity::RouteId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:26` |
| `pocketstation::frame::identity::SessionId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:22` |
| `pocketstation::frame::identity::SourceId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:21` |
| `pocketstation::frame::identity::StemId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:23` |
| `pocketstation::frame::identity::StreamId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:20` |
| `pocketstation::frame::lineage::FrameLineage` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:6` |
| `pocketstation::frame::lineage::FrameLineageBuildError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:93` |
| `pocketstation::frame::lineage::FrameLineageBuildError::TimestampOverflow` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:99` |
| `pocketstation::frame::lineage::FrameLineageBuildError::ZeroDuration` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:95` |
| `pocketstation::frame::lineage::FrameLineageBuildError::ZeroSourceGeneration` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/lineage.rs:97` |
| `lineage` | module | Compact source-aware lineage carried on realtime audio frames. | `src/frame/lineage.rs:1` |
| `pocketstation::frame::audio::AudioFrame` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:39` |
| `pocketstation::frame::audio::SampleSpec` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:18` |
| `pocketstation::frame::audio::SharedAudioFrame` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:176` |
| `pocketstation::frame::pool::AudioBufferHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:198` |
| `pocketstation::frame::pool::AudioBufferPool` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:24` |
| `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:281` |

## Where you encounter it

- **Select a durable source** — Discover and resolve a source selector while preserving identity and source-generation changes.
- **Inject external PCM** — Acquire bounded buffers, write PCM, and observe source runtime outcomes.
- **Encode and decode a stream** — Configure Opus state and convert audio frames to packets and back.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_frame_lineage_when_timestamp_end_requested_then_duration_is_saturating` — given frame lineage when timestamp end requested then duration is saturating (`src/frame/audio.rs:458`; `test-f691cb19b7818f0469d6`).
- `given_matching_frame_and_lineage_when_frozen_then_epochs_survive_fanout` — given matching frame and lineage when frozen then epochs survive fanout (`src/frame/audio.rs:478`; `test-dcfc2ba5c5cd222ed81e`).
- `given_mismatched_dynamic_frame_identity_when_enveloped_then_rejected` — given mismatched dynamic frame identity when enveloped then rejected (`src/frame/audio.rs:510`; `test-7607ea6bdc1ab707af10`).
- `given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960` — given 48khz mono spec when frame samples for 20ms then returns 960 (`src/frame/audio.rs:434`; `test-bf95ac4b2316d447ed6b`).
- `given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920` — given 48khz stereo spec when frame samples for 20ms then returns 1920 (`src/frame/audio.rs:446`; `test-27034bea6e0bcfc0b91b`).
- `given_shared_frame_when_copied_to_branch_pool_then_samples_are_independent` — given shared frame when copied to branch pool then samples are independent (`src/frame/audio.rs:587`; `test-4f61f7c1e0c9b8c6c3ca`).
- `given_acquired_handle_when_copy_from_slice_then_length_matches_data` — given acquired handle when copy from slice then length matches data (`src/frame/audio.rs:378`; `test-dde445e05c14558c788a`).
- `given_exhausted_pool_when_handle_dropped_then_reacquire_succeeds` — given exhausted pool when handle dropped then reacquire succeeds (`src/frame/audio.rs:407`; `test-931c3c8a724375d8c6e5`).
- `given_frozen_buffer_with_many_consumers_when_final_handle_drops_then_slot_is_reused` — given frozen buffer with many consumers when final handle drops then slot is reused (`src/frame/audio.rs:536`; `test-5df473b160e87b1a3092`).
- `given_full_64_slot_pool_when_acquire_then_returns_none_and_increments_failures` — given full 64 slot pool when acquire then returns none and increments failures (`src/frame/audio.rs:392`; `test-eab0bbace82ed1b49614`).
- `given_pool_acquisition_and_release_when_observed_then_available_slots_are_exact` — given pool acquisition and release when observed then available slots are exact (`src/frame/audio.rs:366`; `test-2058f9d84350161875c9`).
- `given_pool_when_acquire_and_release_then_in_use_flag_tracks_state` — given pool when acquire and release then in use flag tracks state (`src/frame/audio.rs:421`; `test-e1e8072dd531e72a3bbb`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [Memory ownership and buffer pools](/docs/internals/memory-ownership.md)
- [PocketStation](/README.md)
- [Realtime audio lane](/docs/internals/realtime-audio-lane.md)
- [Capture application and microphone stems](/docs/how-to/capture-app-and-mic.md)
- [Encode and decode Opus](/docs/how-to/opus-roundtrip.md)
- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/frame/lineage.rs:1-101` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

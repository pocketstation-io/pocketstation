# Frame identity and lineage

<!-- claims: CLM-DOC-012-CAP-001,CLM-DOC-012-SOURCE-001 -->

## What it is

Frame lineage is immutable metadata that identifies the Session, source, stem, stream, route, clock domain, sequence, generation, and derivation associated with audio.

## Why it exists

PCM alone cannot explain which source produced it, which route delivered it, or whether a discontinuity occurred. Lineage makes those answers available to consumers and diagnostics.

## Relationships

- `AudioFrame` combines sample storage, sample specification, and lineage.
- Clock mapping correlates lineage timestamps with the Session timeline.
- Signals use corresponding lineage so operator output remains attributable.

## Invariants and guarantees

- A frame's identity is not rewritten when it crosses a route.
- Derived output records its upstream derivation rather than pretending to be an original source.
- Sequence, generation, and discontinuity changes remain observable.

## When you encounter it

- **Select a durable source** — Discover and resolve a source selector while preserving identity and source-generation changes.
- **Inject external PCM** — Acquire bounded buffers, write PCM, and observe source runtime outcomes.
- **Encode and decode a stream** — Configure Opus state and convert audio frames to packets and back.

## Use it

- [Map source time](/docs/how-to/map-source-time.md)
- [Preserve source identity](/docs/best-practices/source-identity.md)
- [Investigate timestamp divergence](/docs/troubleshooting/timing.md)

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.

The scope of **Frame identity and lineage** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::frame::audio::LineagedAudioFrame` | struct | An exclusive audio frame and the immutable lineage snapshot captured before the frame crosses a bounded edge. | `src/frame/audio.rs:266` |
| `pocketstation::frame::audio::SharedLineagedAudioFrame` | struct | Carries one shared lineaged audio payload together with its declared metadata. | `src/frame/audio.rs:298` |
| `pocketstation::frame::identity::ClockDomainId` | struct | Uniquely identifies clock domain within its PocketStation ownership scope. | `src/frame/identity.rs:29` |
| `pocketstation::frame::identity::ConnectorId` | struct | Uniquely identifies connector within its PocketStation ownership scope. | `src/frame/identity.rs:25` |
| `pocketstation::frame::identity::EndpointId` | struct | Uniquely identifies endpoint within its PocketStation ownership scope. | `src/frame/identity.rs:24` |
| `pocketstation::frame::identity::RouteId` | struct | Uniquely identifies route within its PocketStation ownership scope. | `src/frame/identity.rs:26` |
| `pocketstation::frame::identity::SessionId` | struct | Uniquely identifies session within its PocketStation ownership scope. | `src/frame/identity.rs:22` |
| `pocketstation::frame::identity::SourceId` | struct | Uniquely identifies source within its PocketStation ownership scope. | `src/frame/identity.rs:21` |
| `pocketstation::frame::identity::StemId` | struct | Uniquely identifies stem within its PocketStation ownership scope. | `src/frame/identity.rs:23` |
| `pocketstation::frame::identity::StreamId` | struct | Uniquely identifies stream within its PocketStation ownership scope. | `src/frame/identity.rs:20` |

## Executable evidence

Executable evidence selected for **Frame identity and lineage** is limited to each test's recorded setup and assertions:

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

The claims on **Frame identity and lineage** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/frame/lineage.rs:1-101` (`DIRECT`)

For **Frame identity and lineage**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

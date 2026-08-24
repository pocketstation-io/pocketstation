# Frame identity and lineage

<!-- claims: CLM-DOC-012-SCOPE-001,CLM-DOC-012-TEXT-001,CLM-DOC-012-TEXT-002,CLM-DOC-012-TEXT-003,CLM-DOC-012-TEXT-004,CLM-DOC-012-TEXT-005,CLM-DOC-012-TEXT-006,CLM-DOC-012-SOURCE-001 -->

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

- `given_frame_lineage_when_timestamp_end_requested_then_duration_is_saturating` — given frame lineage when timestamp end requested then duration is saturating (`src/frame/audio.rs:458`; `test-ae12619209044ccec1dc`).
- `given_matching_frame_and_lineage_when_frozen_then_epochs_survive_fanout` — given matching frame and lineage when frozen then epochs survive fanout (`src/frame/audio.rs:478`; `test-ea9e9e188acbacaa93e3`).
- `given_mismatched_dynamic_frame_identity_when_enveloped_then_rejected` — given mismatched dynamic frame identity when enveloped then rejected (`src/frame/audio.rs:510`; `test-cf209e0baa5c521e52ae`).
- `given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960` — given 48khz mono spec when frame samples for 20ms then returns 960 (`src/frame/audio.rs:434`; `test-bc27ef56fe4e052f18d1`).
- `given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920` — given 48khz stereo spec when frame samples for 20ms then returns 1920 (`src/frame/audio.rs:446`; `test-aaeb53c9b93d7d667a32`).
- `given_shared_frame_when_copied_to_branch_pool_then_samples_are_independent` — given shared frame when copied to branch pool then samples are independent (`src/frame/audio.rs:587`; `test-7ce6e852670026e2b412`).
- `given_acquired_handle_when_copy_from_slice_then_length_matches_data` — given acquired handle when copy from slice then length matches data (`src/frame/audio.rs:378`; `test-49f5dd0943c01f09e6af`).
- `given_exhausted_pool_when_handle_dropped_then_reacquire_succeeds` — given exhausted pool when handle dropped then reacquire succeeds (`src/frame/audio.rs:407`; `test-7d64ba1deb4442d9cd6d`).
- `given_frozen_buffer_with_many_consumers_when_final_handle_drops_then_slot_is_reused` — given frozen buffer with many consumers when final handle drops then slot is reused (`src/frame/audio.rs:536`; `test-eee9fd42fc6e945a499c`).
- `given_full_64_slot_pool_when_acquire_then_returns_none_and_increments_failures` — given full 64 slot pool when acquire then returns none and increments failures (`src/frame/audio.rs:392`; `test-ce69bc761c4ca8ea48fa`).
- `given_pool_acquisition_and_release_when_observed_then_available_slots_are_exact` — given pool acquisition and release when observed then available slots are exact (`src/frame/audio.rs:366`; `test-f0315e7a9cc5ce3afe08`).
- `given_pool_when_acquire_and_release_then_in_use_flag_tracks_state` — given pool when acquire and release then in use flag tracks state (`src/frame/audio.rs:421`; `test-6600d4d28322982cd512`).

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

The claims on **Frame identity and lineage** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/frame/lineage.rs:1-1` (`DECLARED`)

For **Frame identity and lineage**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

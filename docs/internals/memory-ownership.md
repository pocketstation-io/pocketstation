# Memory ownership and buffer pools

<!-- claims: CLM-DOC-052-CAP-001,CLM-DOC-052-CAP-002,CLM-DOC-052-SOURCE-001 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

The scope of **Memory ownership and buffer pools** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership map

- `src/frame/audio.rs` owns part of this boundary.
- `src/frame/pool.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::frame::pool::AudioBufferHandle` | struct | Owns bounded access to audio buffer. | `src/frame/pool.rs:198` |
| `pocketstation::frame::pool::AudioBufferPool` | struct | Owns fixed-capacity reusable audio slots and reports acquisition pressure without allocating per frame. | `src/frame/pool.rs:24` |
| `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | Owns bounded access to shared audio buffer. | `src/frame/pool.rs:281` |
| `pocketstation::frame::pool::AudioBufferWriteError` | enum | Classifies failures reported as audio buffer write error. | `src/frame/pool.rs:14` |
| `pocketstation::frame::pool::AudioBufferWriteError::CapacityExceeded` | variant | Reported when the owning operation encounters capacity exceeded. | `src/frame/pool.rs:18` |
| `pocketstation::runtime::audio::router::PlanRouterError::MissingMemoryPlan` | variant | Reported when the owning operation encounters missing memory plan. | `src/runtime/audio/router.rs:19` |
| `PlanRouterError::MissingMemoryPlan::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingMemoryPlan`. | `src/runtime/audio/router.rs:19` |
| `pool::AudioBufferWriteError::CapacityExceeded::capacity_samples` | struct_field | Sets the capacity samples available to `CapacityExceeded`. | `src/frame/pool.rs:20` |
| `pool::AudioBufferWriteError::CapacityExceeded::requested_samples` | struct_field | Stores the requested samples used by `CapacityExceeded`. | `src/frame/pool.rs:19` |
| `audio` | module | Types and operations for audio. | `src/frame/audio.rs:1` |
| `lineage` | module | Compact source-aware lineage carried on realtime audio frames. | `src/frame/lineage.rs:1` |
| `pool` | module | Fixed-capacity realtime audio storage and ownership handles. | `src/frame/pool.rs:1` |
| `pocketstation::frame::audio::AudioFrame` | struct | Carries one audio payload together with its declared metadata. | `src/frame/audio.rs:39` |
| `pocketstation::frame::audio::LineagedAudioFrame` | struct | An exclusive audio frame and the immutable lineage snapshot captured before the frame crosses a bounded edge. | `src/frame/audio.rs:266` |
| `pocketstation::frame::audio::SampleSpec` | struct | Configures sample behavior at its owning API boundary. | `src/frame/audio.rs:18` |
| `pocketstation::frame::audio::SharedAudioFrame` | struct | Carries one shared audio payload together with its declared metadata. | `src/frame/audio.rs:176` |
| `pocketstation::frame::audio::SharedLineagedAudioFrame` | struct | Carries one shared lineaged audio payload together with its declared metadata. | `src/frame/audio.rs:298` |
| `pocketstation::frame::identity::ClockDomainId` | struct | Uniquely identifies clock domain within its PocketStation ownership scope. | `src/frame/identity.rs:29` |
| `pocketstation::frame::identity::ConnectorId` | struct | Uniquely identifies connector within its PocketStation ownership scope. | `src/frame/identity.rs:25` |
| `pocketstation::frame::identity::EndpointId` | struct | Uniquely identifies endpoint within its PocketStation ownership scope. | `src/frame/identity.rs:24` |
| `pocketstation::frame::identity::RouteId` | struct | Uniquely identifies route within its PocketStation ownership scope. | `src/frame/identity.rs:26` |
| `pocketstation::frame::identity::SessionId` | struct | Uniquely identifies session within its PocketStation ownership scope. | `src/frame/identity.rs:22` |
| `pocketstation::frame::identity::SourceId` | struct | Uniquely identifies source within its PocketStation ownership scope. | `src/frame/identity.rs:21` |
| `pocketstation::frame::identity::StemId` | struct | Uniquely identifies stem within its PocketStation ownership scope. | `src/frame/identity.rs:23` |

## Observed implementation patterns

- `typed_error` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/frame/pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/signal/edge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/frame/lineage.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/frame/lineage.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/audio_buffer_pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/bridge/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/signal/io.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/bridge/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/runtime/signal/edge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `tests/runtime_plan_router_alloc.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/frame/pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

Executable evidence selected for **Memory ownership and buffer pools** is limited to each test's recorded setup and assertions:

- `given_frozen_buffer_with_many_consumers_when_final_handle_drops_then_slot_is_reused` — given frozen buffer with many consumers when final handle drops then slot is reused (`src/frame/audio.rs:536`; `test-eee9fd42fc6e945a499c`).
- `given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960` — given 48khz mono spec when frame samples for 20ms then returns 960 (`src/frame/audio.rs:434`; `test-bc27ef56fe4e052f18d1`).
- `given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920` — given 48khz stereo spec when frame samples for 20ms then returns 1920 (`src/frame/audio.rs:446`; `test-aaeb53c9b93d7d667a32`).
- `given_acquired_handle_when_copy_from_slice_then_length_matches_data` — given acquired handle when copy from slice then length matches data (`src/frame/audio.rs:378`; `test-49f5dd0943c01f09e6af`).
- `given_exhausted_pool_when_handle_dropped_then_reacquire_succeeds` — given exhausted pool when handle dropped then reacquire succeeds (`src/frame/audio.rs:407`; `test-7d64ba1deb4442d9cd6d`).
- `given_frame_lineage_when_timestamp_end_requested_then_duration_is_saturating` — given frame lineage when timestamp end requested then duration is saturating (`src/frame/audio.rs:458`; `test-ae12619209044ccec1dc`).
- `given_full_64_slot_pool_when_acquire_then_returns_none_and_increments_failures` — given full 64 slot pool when acquire then returns none and increments failures (`src/frame/audio.rs:392`; `test-ce69bc761c4ca8ea48fa`).
- `given_matching_frame_and_lineage_when_frozen_then_epochs_survive_fanout` — given matching frame and lineage when frozen then epochs survive fanout (`src/frame/audio.rs:478`; `test-ea9e9e188acbacaa93e3`).
- `given_mismatched_dynamic_frame_identity_when_enveloped_then_rejected` — given mismatched dynamic frame identity when enveloped then rejected (`src/frame/audio.rs:510`; `test-cf209e0baa5c521e52ae`).
- `given_pool_acquisition_and_release_when_observed_then_available_slots_are_exact` — given pool acquisition and release when observed then available slots are exact (`src/frame/audio.rs:366`; `test-f0315e7a9cc5ce3afe08`).
- `given_pool_when_acquire_and_release_then_in_use_flag_tracks_state` — given pool when acquire and release then in use flag tracks state (`src/frame/audio.rs:421`; `test-6600d4d28322982cd512`).
- `given_pool_with_4_slots_when_all_acquired_then_next_acquire_returns_none` — given pool with 4 slots when all acquired then next acquire returns none (`src/frame/audio.rs:346`; `test-a86f964d5b2458067ad3`).

## Stability boundary

**Memory ownership and buffer pools** describes internal ownership. Its private module layout is not a compatibility promise; compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Realtime audio lane](/docs/internals/realtime-audio-lane.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Keep realtime callbacks bounded](/docs/best-practices/realtime-boundaries.md)
- [Architecture overview](/docs/architecture/overview.md)
- [Frame identity and lineage](/docs/concepts/frame-lineage.md)

## Evidence boundary

The claims on **Memory ownership and buffer pools** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/frame/audio.rs:1-636` (`DIRECT`)
- `src/frame/pool.rs:1-336` (`DIRECT`)

For **Memory ownership and buffer pools**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

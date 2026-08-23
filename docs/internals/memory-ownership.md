# Memory ownership and buffer pools

<!-- claims: CLM-DOC-052-CAP-001,CLM-DOC-052-CAP-002,CLM-DOC-052-SOURCE-001 -->

## Scope

- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership map

- `src/frame/audio.rs` owns part of this boundary.
- `src/frame/pool.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::frame::pool::AudioBufferHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:198` |
| `pocketstation::frame::pool::AudioBufferPool` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:24` |
| `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:281` |
| `pocketstation::frame::pool::AudioBufferWriteError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:14` |
| `pocketstation::frame::pool::AudioBufferWriteError::CapacityExceeded` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:18` |
| `SignalEdgeObservations::maximum_buffered_payload_bytes` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/signal/edge.rs:34` |
| `pool::AudioBufferWriteError::CapacityExceeded::capacity_samples` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:20` |
| `pool::AudioBufferWriteError::CapacityExceeded::requested_samples` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/pool.rs:19` |
| `audio` | module | Allocation-free realtime audio execution lane. | `src/runtime/audio/mod.rs:1` |
| `lifecycle` | module | Non-realtime runtime ownership and process-protocol lifecycle. | `src/runtime/lifecycle/mod.rs:1` |
| `lineage` | module | Compact source-aware lineage carried on realtime audio frames. | `src/frame/lineage.rs:1` |
| `pool` | module | Fixed-capacity realtime audio storage and ownership handles. | `src/frame/pool.rs:1` |
| `signal` | module | Bounded asynchronous signal execution lane. | `src/runtime/signal/mod.rs:1` |
| `try_copy_from_slice` | function | Copies samples into this fixed-capacity slot without panicking. | `src/frame/pool.rs:240` |
| `try_set_len` | function | Changes the visible sample length without panicking. | `src/frame/pool.rs:228` |
| `SignalEdgeObservations::delivered_total` | struct_field | Compatibility alias for `enqueued_total`. | `src/runtime/signal/edge.rs:44` |
| `audio` | module | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:1` |
| `pocketstation::frame::audio::AudioFrame` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:39` |
| `pocketstation::frame::audio::SampleSpec` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:18` |
| `pocketstation::frame::audio::SharedAudioFrame` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/audio.rs:176` |
| `pocketstation::frame::identity::ClockDomainId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:29` |
| `pocketstation::frame::identity::ConnectorId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:25` |
| `pocketstation::frame::identity::EndpointId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:24` |
| `pocketstation::frame::identity::RouteId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/frame/identity.rs:26` |

## Observed implementation patterns

- `typed_error` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/frame/pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `benches/runtime_plan.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/executor.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/async_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/frame/lineage.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/io.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/edge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `tests/runtime_plan_router_alloc.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/runtime/audio/router.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/nodes.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/signal/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/audio_buffer_pool.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/frame/audio.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/lifecycle/sidecar_host.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/runtime/lifecycle/sidecar_protocol.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/runtime/signal/operator.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `src/runtime/audio/runner.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

The following test bodies are evidence only for their recorded setup:

- `given_frozen_buffer_with_many_consumers_when_final_handle_drops_then_slot_is_reused` — given frozen buffer with many consumers when final handle drops then slot is reused (`src/frame/audio.rs:536`; `test-5df473b160e87b1a3092`).
- `given_one_branch_when_signal_published_then_receiver_has_exclusive_ownership` — given one branch when signal published then receiver has exclusive ownership (`src/runtime/signal/edge.rs:626`; `test-9674b8f8edebf8590582`).
- `given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960` — given 48khz mono spec when frame samples for 20ms then returns 960 (`src/frame/audio.rs:434`; `test-bf95ac4b2316d447ed6b`).
- `given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920` — given 48khz stereo spec when frame samples for 20ms then returns 1920 (`src/frame/audio.rs:446`; `test-27034bea6e0bcfc0b91b`).
- `given_acquired_handle_when_copy_from_slice_then_length_matches_data` — given acquired handle when copy from slice then length matches data (`src/frame/audio.rs:378`; `test-dde445e05c14558c788a`).
- `given_exhausted_pool_when_handle_dropped_then_reacquire_succeeds` — given exhausted pool when handle dropped then reacquire succeeds (`src/frame/audio.rs:407`; `test-931c3c8a724375d8c6e5`).
- `given_frame_lineage_when_timestamp_end_requested_then_duration_is_saturating` — given frame lineage when timestamp end requested then duration is saturating (`src/frame/audio.rs:458`; `test-f691cb19b7818f0469d6`).
- `given_full_64_slot_pool_when_acquire_then_returns_none_and_increments_failures` — given full 64 slot pool when acquire then returns none and increments failures (`src/frame/audio.rs:392`; `test-eab0bbace82ed1b49614`).
- `given_matching_frame_and_lineage_when_frozen_then_epochs_survive_fanout` — given matching frame and lineage when frozen then epochs survive fanout (`src/frame/audio.rs:478`; `test-dcfc2ba5c5cd222ed81e`).
- `given_mismatched_dynamic_frame_identity_when_enveloped_then_rejected` — given mismatched dynamic frame identity when enveloped then rejected (`src/frame/audio.rs:510`; `test-7607ea6bdc1ab707af10`).
- `given_pool_acquisition_and_release_when_observed_then_available_slots_are_exact` — given pool acquisition and release when observed then available slots are exact (`src/frame/audio.rs:366`; `test-2058f9d84350161875c9`).
- `given_pool_when_acquire_and_release_then_in_use_flag_tracks_state` — given pool when acquire and release then in use flag tracks state (`src/frame/audio.rs:421`; `test-e1e8072dd531e72a3bbb`).

## Stability boundary

This page explains internals. Public compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts—not private module layout.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/frame/audio.rs:1-636` (`DIRECT`)
- `src/frame/pool.rs:1-336` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

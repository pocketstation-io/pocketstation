# Keep realtime callbacks bounded

<!-- claims: CLM-BEST-002-CAP-001,CLM-BEST-002-CAP-002,CLM-BEST-002-SOURCE-001 -->

## Recommendation

Keep allocation, blocking, locks, async scheduling, logging, and application callbacks outside the realtime callback path where architecture checks enforce that boundary.

## Why

The repository makes capacity, ownership, identity, lifecycle, and evidence boundaries explicit so failures remain attributable. Bypassing them removes observations and typed outcomes needed for diagnosis.

## Tradeoff

The recommendation requires explicit configuration and result handling. It does not promise that one capacity, retry budget, selector, or shutdown policy fits every workload. Measure within the API's stated scope.

## When it does not apply

Do not apply a realtime, connector, capture, or extension rule to another lane or boundary unless it exposes the same contract. An internal pattern is not automatically a public recommendation.

## Repository evidence

- `typed_error` at `src/runtime/audio/runner.rs` (`pattern-007b832d1ee35a325c5f`).
- `clock_correlation` at `src/frame/audio.rs` (`pattern-0311869ce2fb5212a274`).
- `buffer_pool` at `src/runtime/nodes.rs` (`pattern-06b6cb48eb37e9925506`).
- `sidecar_isolation` at `src/runtime/lifecycle/async_host.rs` (`pattern-0b62d1010395e9d56174`).
- `buffer_pool` at `benches/runtime_plan.rs` (`pattern-0bed8aaed06c69250bca`).
- `typed_error` at `src/frame/pool.rs` (`pattern-145c92021b15c7666c35`).
- `sidecar_isolation` at `benches/runtime_plan.rs` (`pattern-190078991e6fb29b089b`).
- `buffer_pool` at `src/runtime/audio/executor.rs` (`pattern-3caaa68fb904e3dab03b`).
- `sidecar_isolation` at `src/runtime/signal/operator.rs` (`pattern-3da02ff64109c6786946`).
- `typed_error` at `src/runtime/lifecycle/async_host.rs` (`pattern-4b4827ead247761ef305`).
- `clock_correlation` at `src/frame/lineage.rs` (`pattern-503fc1d95fe068d3ac6f`).
- `sidecar_isolation` at `src/runtime/signal/io.rs` (`pattern-520a0bbdfc42436280d2`).
- `sidecar_isolation` at `src/runtime/audio/router.rs` (`pattern-545a8ffd6d5188242445`).
- `buffer_pool` at `src/runtime/audio/runner.rs` (`pattern-550a0db4c916e2cf76fd`).
- `sidecar_isolation` at `src/runtime/signal/edge.rs` (`pattern-5b4a5379bfca29277451`).
- `bounded_queue` at `src/runtime/nodes.rs` (`pattern-63aef9d26f2a98d76eea`).
- `sidecar_isolation` at `src/runtime/nodes.rs` (`pattern-67ea0547a35e7f54b3c5`).
- `clock_correlation` at `src/runtime/audio/router.rs` (`pattern-69313a14552fd2f67df1`).
- `sidecar_isolation` at `tests/runtime_plan_router_alloc.rs` (`pattern-6cd1a46f8aad41f0b460`).
- `bounded_queue` at `src/runtime/audio/router.rs` (`pattern-704b068457aff5e7ae16`).

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-14905efc2e19f82a8cb2`).
- `given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy` — given realtime fan out when executed then each mutating branch gets independent copy (`src/runtime/audio/executor.rs:369`; `test-c0c81ff42570a02c1eb9`).
- `given_queued_sources_when_cancelled_with_budget_then_drain_is_bounded_and_rest_discards` — given queued sources when cancelled with budget then drain is bounded and rest discards (`src/runtime/audio/runner.rs:636`; `test-31632b8eb3f0b3c90934`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-0ae60369d5962ff55b0f`).
- `given_full_input_branch_when_sent_then_overflow_is_counted_and_join_is_bounded` — given full input branch when sent then overflow is counted and join is bounded (`src/runtime/signal/operator.rs:2049`; `test-95905f4e18a52f786a53`).
- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` — given operator composition with three external operators then derived output crosses each bounded edge (`src/runtime/signal/operator.rs:1859`; `test-9ec51c75cedb5ffaef0f`).
- `given_prepared_realtime_plan_when_connected_nodes_execute_then_no_heap_allocation_occurs` — given prepared realtime plan when connected nodes execute then no heap allocation occurs (`tests/runtime_plan_router_alloc.rs:89`; `test-8f6b3c13a8c5d31e5914`).
- `given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960` — given 48khz mono spec when frame samples for 20ms then returns 960 (`src/frame/audio.rs:434`; `test-bf95ac4b2316d447ed6b`).
- `given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920` — given 48khz stereo spec when frame samples for 20ms then returns 1920 (`src/frame/audio.rs:446`; `test-27034bea6e0bcfc0b91b`).
- `given_acquired_handle_when_copy_from_slice_then_length_matches_data` — given acquired handle when copy from slice then length matches data (`src/frame/audio.rs:378`; `test-dde445e05c14558c788a`).
- `given_exhausted_pool_when_handle_dropped_then_reacquire_succeeds` — given exhausted pool when handle dropped then reacquire succeeds (`src/frame/audio.rs:407`; `test-931c3c8a724375d8c6e5`).
- `given_frame_lineage_when_timestamp_end_requested_then_duration_is_saturating` — given frame lineage when timestamp end requested then duration is saturating (`src/frame/audio.rs:458`; `test-f691cb19b7818f0469d6`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [Memory ownership and buffer pools](/docs/internals/memory-ownership.md)
- [PocketStation](/README.md)
- [Realtime audio lane](/docs/internals/realtime-audio-lane.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Architecture overview](/docs/architecture/overview.md)
- [Frame identity and lineage](/docs/concepts/frame-lineage.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/audio/runner.rs:1-746` (`DIRECT`)
- `scripts/lint/check-architecture-constraints.sh:1-81` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

# Keep realtime callbacks bounded

<!-- claims: CLM-BEST-002-CAP-001,CLM-BEST-002-CAP-002,CLM-BEST-002-SOURCE-001 -->

## Problem

Allocation, blocking, locks, async scheduling, logging, or application callbacks in the realtime lane can violate its bounded-work contract.

## Recommendation

Keep realtime callbacks limited to the operations accepted by the declared safety contract, and move asynchronous or blocking work to its explicit partition.

## Reason

The repository architecture checks and fixed-capacity routes rely on a bounded realtime path.

## Tradeoff

Crossing to another partition introduces queue capacity, ownership transfer, and saturation that you must observe.

## When it does not apply

The restriction does not apply to an async or blocking worker whose contract explicitly permits that work.

## Repository evidence

- `buffer_pool` at `src/runtime/audio/runner.rs` (`pattern-550a0db4c916e2cf76fd`).

## Executable evidence

Executable evidence selected for **Keep realtime callbacks bounded** is limited to each test's recorded setup and assertions:

- `given_queued_sources_when_cancelled_with_budget_then_drain_is_bounded_and_rest_discards` — given queued sources when cancelled with budget then drain is bounded and rest discards (`src/runtime/audio/runner.rs:636`; `test-f9e6bcf6752e622096af`).
- `given_full_source_input_when_more_frames_arrive_then_newest_rejects_and_counts` — given full source input when more frames arrive then newest rejects and counts (`src/runtime/audio/runner.rs:704`; `test-3f76115476879df63de7`).
- `given_queued_sources_when_cancelled_with_discard_then_no_frame_executes` — given queued sources when cancelled with discard then no frame executes (`src/runtime/audio/runner.rs:682`; `test-4eb110fe913e6a3021f7`).
- `given_two_ready_sources_when_processed_then_each_source_dispatches_independently` — given two ready sources when processed then each source dispatches independently (`src/runtime/audio/runner.rs:595`; `test-c7a541844f8163662960`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-aee462488aef78361374`).
- `given_realtime_fan_out_when_executed_then_each_mutating_branch_gets_independent_copy` — given realtime fan out when executed then each mutating branch gets independent copy (`src/runtime/audio/executor.rs:369`; `test-8b303620bdafeb3aa260`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-1dfdf14fa335d99dccdc`).
- `given_full_input_branch_when_sent_then_overflow_is_counted_and_join_is_bounded` — given full input branch when sent then overflow is counted and join is bounded (`src/runtime/signal/operator.rs:2049`; `test-4ea56b723e02f622ad3d`).
- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` — given operator composition with three external operators then derived output crosses each bounded edge (`src/runtime/signal/operator.rs:1859`; `test-eab3e581e210e0e82882`).
- `given_prepared_realtime_plan_when_connected_nodes_execute_then_no_heap_allocation_occurs` — given prepared realtime plan when connected nodes execute then no heap allocation occurs (`tests/runtime_plan_router_alloc.rs:89`; `test-7300dec9e191c3915ad1`).
- `given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960` — given 48khz mono spec when frame samples for 20ms then returns 960 (`src/frame/audio.rs:434`; `test-bc27ef56fe4e052f18d1`).
- `given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920` — given 48khz stereo spec when frame samples for 20ms then returns 1920 (`src/frame/audio.rs:446`; `test-aaeb53c9b93d7d667a32`).

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

The claims on **Keep realtime callbacks bounded** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/audio/runner.rs:1-746` (`DIRECT`)
- `scripts/lint/check-architecture-constraints.sh:1-81` (`DIRECT`)

For **Keep realtime callbacks bounded**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

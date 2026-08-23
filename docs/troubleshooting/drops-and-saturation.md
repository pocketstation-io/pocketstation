# Frames or signals are dropped

<!-- claims: CLM-TRBL-006-CAP-001,CLM-TRBL-006-CAP-002,CLM-TRBL-006-CAP-003,CLM-TRBL-006-CAP-004,CLM-TRBL-006-CAP-005,CLM-TRBL-006-CAP-006,CLM-TRBL-006-CAP-007,CLM-TRBL-006-SOURCE-001 -->

## Symptom

Route observations report drops, saturation, full queues, or rejected signals.

## Evidenced causes

- A consumer is slower than its producer at the configured capacity.
- A polled-audio lease remains outstanding and consumes receipt capacity.
- A signal payload exceeds its branch limit.
- The selected loss or delivery policy rejects work at saturation.

## Distinguish the causes

Compare capacity, current depth, peak depth, drop counters, lease counters, and branch identity. Identify one constrained route before changing global settings.

## Diagnostic signals

- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `CapacityTooLarge` (`error-74e52b74897467add979`)
- `pocketstation::runtime::audio::router::PlanRouterError` (`error-fb8f2ed6471ac52a3017`)
- `pocketstation::runtime::audio::router::PlanRouterError` / `InvalidFrameBytes` (`error-852f32645f2215a240b9`)
- `pocketstation::runtime::audio::router::PlanRouterError` / `MissingMemoryPlan` (`error-2bcbacd4cc3a2d562e0e`)
- `pocketstation::runtime::audio::router::PlanRouterError` / `ZeroCapacity` (`error-feb909d9c49412065cd0`)
- `pocketstation::runtime::signal::edge::SignalEdgeSendError` (`error-ade659033eed214c635f`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` (`error-b6dcc6b3baf2420480a7`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `MissingPayloadLimit` (`error-02c550e51f66d125b84d`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `NoBranches` (`error-d382f0afba4a0003ec4b`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `PayloadLimitTooLarge` (`error-ad2b2d68045f1967b941`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `ZeroCapacity` (`error-62954981d20879dae3fd`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `ZeroPayloadLimit` (`error-e6b96455ab48d58c3af3`)

## Executable evidence

- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` exercises given enqueued and dropped frames when observed then drop rate uses all attempts under its recorded setup (`test-81f2a37c65fc1321fb4b`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` exercises given lineaged source fan out when branch frames are copied then exact lineage is preserved under its recorded setup (`test-8407a37a5b957f010ddf`).
- `given_shutdown_with_queued_shared_frames_when_receivers_drop_then_pool_slots_are_released` exercises given shutdown with queued shared frames when receivers drop then pool slots are released under its recorded setup (`test-4d0ca8ad81eff9595233`).
- `given_slow_full_branch_when_more_frames_dispatched_then_other_branch_continues` exercises given slow full branch when more frames dispatched then other branch continues under its recorded setup (`test-14f54e95ea511804cb86`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` exercises given compiled text edge when router builds then only audio edge gets audio receiver under its recorded setup (`test-687c08c4ebc7699d891b`).
- `given_failed_branch_when_receiver_drops_then_unrelated_branch_continues` exercises given failed branch when receiver drops then unrelated branch continues under its recorded setup (`test-e79727ff2a1d9faecc74`).
- `given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated` exercises given foreign clock timestamp when delivered then source latency is not fabricated under its recorded setup (`test-2dfdf77222ba4754d494`).
- `given_lineage_discontinuity_epoch_change_when_received_then_declared_discontinuity_is_counted` exercises given lineage discontinuity epoch change when received then declared discontinuity is counted under its recorded setup (`test-6161d7a8c36359a8e55e`).
- `given_observation_handle_when_consumer_detects_gap_then_live_discontinuity_is_visible` exercises given observation handle when consumer detects gap then live discontinuity is visible under its recorded setup (`test-b11b6230db89e523d9d4`).
- `given_observation_handle_when_producer_fills_edge_then_live_queue_and_drop_are_visible` exercises given observation handle when producer fills edge then live queue and drop are visible under its recorded setup (`test-26474ba2232fc8e073d2`).
- `given_observation_handle_when_receiver_drops_then_shutdown_snapshot_remains_available` exercises given observation handle when receiver drops then shutdown snapshot remains available under its recorded setup (`test-84a0ed752ce8338a3166`).
- `given_one_source_with_three_edges_when_dispatched_then_every_edge_receives_identified_frame` exercises given one source with three edges when dispatched then every edge receives identified frame under its recorded setup (`test-d96ed9959cf0c6f31b59`).
- `given_queued_frame_when_clocked_receive_runs_then_clock_is_sampled_after_pop` exercises given queued frame when clocked receive runs then clock is sampled after pop under its recorded setup (`test-b7f788e10424241552b8`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` exercises given receive before enqueue when observed then latency sample is rejected under its recorded setup (`test-0f111134a8c8ddb61a69`).
- `given_receiver_holds_popped_frame_when_queue_has_room_then_next_copy_is_enqueued` exercises given receiver holds popped frame when queue has room then next copy is enqueued under its recorded setup (`test-899501604678e931ee8b`).

## Corrective action

Release leases, fix the slow consumer, reduce work, or deliberately choose a different capacity and edge policy.

## Retry and incomplete state

A dropped item cannot be recovered by retry unless the producer and contract support replay. Other routes can remain complete or become independently partial.

## Related reference

- [Realtime Routing](/docs/concepts/realtime-routing.md)
- [Route Sizing](/docs/best-practices/route-sizing.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Graph and signal failures](/docs/errors/graph-and-signals.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Size bounded routes from observations](/docs/best-practices/route-sizing.md)

## Evidence boundary

The claims on **Frames or signals are dropped** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/audio/router.rs:1-1646` (`DIRECT`)
- `src/runtime/signal/edge.rs:1-651` (`DIRECT`)

For **Frames or signals are dropped**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

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

- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `CapacityTooLarge` (`error-bcd6542f96923bf14c04`)
- `pocketstation::runtime::audio::router::PlanRouterError` (`error-05bda3230590ed4ebdc0`)
- `pocketstation::runtime::audio::router::PlanRouterError` / `InvalidFrameBytes` (`error-10b07438d146ff25f5ff`)
- `pocketstation::runtime::audio::router::PlanRouterError` / `MissingMemoryPlan` (`error-94555f2e6e2802978bfc`)
- `pocketstation::runtime::audio::router::PlanRouterError` / `ZeroCapacity` (`error-7255bf1a56077c9e285a`)
- `pocketstation::runtime::signal::edge::SignalEdgeSendError` (`error-188081bb302d41d8b38b`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` (`error-0aced45f2e76e7a1963a`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `MissingPayloadLimit` (`error-6596343e02ed441de62a`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `NoBranches` (`error-9444b35743da5ffb7cde`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `PayloadLimitTooLarge` (`error-a5fa26e0cedca7c83187`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `ZeroCapacity` (`error-5b084c27791c642d425c`)
- `pocketstation::runtime::signal::edge::TypedEdgeBuildError` / `ZeroPayloadLimit` (`error-e0e2a1789be560383b81`)

## Executable evidence

- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` exercises given enqueued and dropped frames when observed then drop rate uses all attempts under its recorded setup (`test-9a0bb689d2371b66a92f`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` exercises given lineaged source fan out when branch frames are copied then exact lineage is preserved under its recorded setup (`test-d798548d6c8b059ba1a8`).
- `given_shutdown_with_queued_shared_frames_when_receivers_drop_then_pool_slots_are_released` exercises given shutdown with queued shared frames when receivers drop then pool slots are released under its recorded setup (`test-f8946231a23a1c5d14de`).
- `given_slow_full_branch_when_more_frames_dispatched_then_other_branch_continues` exercises given slow full branch when more frames dispatched then other branch continues under its recorded setup (`test-98b1ad304d1d5b646f6a`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` exercises given compiled text edge when router builds then only audio edge gets audio receiver under its recorded setup (`test-c5f24b62056cfa546c3a`).
- `given_failed_branch_when_receiver_drops_then_unrelated_branch_continues` exercises given failed branch when receiver drops then unrelated branch continues under its recorded setup (`test-b5854f13d50d15dfdbe3`).
- `given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated` exercises given foreign clock timestamp when delivered then source latency is not fabricated under its recorded setup (`test-133d3a4b4c11520b3884`).
- `given_lineage_discontinuity_epoch_change_when_received_then_declared_discontinuity_is_counted` exercises given lineage discontinuity epoch change when received then declared discontinuity is counted under its recorded setup (`test-fceb86228ea42976addb`).
- `given_observation_handle_when_consumer_detects_gap_then_live_discontinuity_is_visible` exercises given observation handle when consumer detects gap then live discontinuity is visible under its recorded setup (`test-225f3db0b8f734fb6907`).
- `given_observation_handle_when_producer_fills_edge_then_live_queue_and_drop_are_visible` exercises given observation handle when producer fills edge then live queue and drop are visible under its recorded setup (`test-7acd587c9b13ea33929e`).
- `given_observation_handle_when_receiver_drops_then_shutdown_snapshot_remains_available` exercises given observation handle when receiver drops then shutdown snapshot remains available under its recorded setup (`test-b85189f2f3734f3dad88`).
- `given_one_source_with_three_edges_when_dispatched_then_every_edge_receives_identified_frame` exercises given one source with three edges when dispatched then every edge receives identified frame under its recorded setup (`test-413eb70a225c14f5ec09`).
- `given_queued_frame_when_clocked_receive_runs_then_clock_is_sampled_after_pop` exercises given queued frame when clocked receive runs then clock is sampled after pop under its recorded setup (`test-f09950594dbe438e24cb`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` exercises given receive before enqueue when observed then latency sample is rejected under its recorded setup (`test-904f86f2ad1e1369647c`).
- `given_receiver_holds_popped_frame_when_queue_has_room_then_next_copy_is_enqueued` exercises given receiver holds popped frame when queue has room then next copy is enqueued under its recorded setup (`test-e0585ce3a8b66eeaa5f4`).

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

The claims on **Frames or signals are dropped** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/audio/router.rs:1-1615` (`DIRECT`)
- `src/runtime/signal/edge.rs:1-651` (`DIRECT`)

For **Frames or signals are dropped**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

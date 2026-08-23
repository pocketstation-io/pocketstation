# Size bounded routes from observations

<!-- claims: CLM-BEST-001-CAP-001,CLM-BEST-001-CAP-002,CLM-BEST-001-CAP-003,CLM-BEST-001-SOURCE-001 -->

## Problem

A guessed route capacity can either saturate under load or reserve more memory than the route needs.

## Recommendation

Measure queue depth, peak depth, saturation, drops, and latency for each route before changing its finite capacity.

## Reason

Per-route observations identify the constrained consumer and keep one branch from hiding another branch's behavior.

## Tradeoff

Measurement adds instrumentation and a representative workload; a larger capacity can trade memory and latency for fewer immediate rejections.

## When it does not apply

Do not reuse one measured capacity for a route with a different producer, consumer, frame size, or loss policy.

## Repository evidence

- `buffer_pool` at `src/runtime/audio/router.rs` (`pattern-bb441f171fed3994bd2a`).

## Executable evidence

Executable evidence selected for **Size bounded routes from observations** is limited to each test's recorded setup and assertions:

- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:983`; `test-c5f24b62056cfa546c3a`).
- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` — given enqueued and dropped frames when observed then drop rate uses all attempts (`src/runtime/audio/router.rs:1241`; `test-9a0bb689d2371b66a92f`).
- `given_failed_branch_when_receiver_drops_then_unrelated_branch_continues` — given failed branch when receiver drops then unrelated branch continues (`src/runtime/audio/router.rs:1518`; `test-b5854f13d50d15dfdbe3`).
- `given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated` — given foreign clock timestamp when delivered then source latency is not fabricated (`src/runtime/audio/router.rs:1182`; `test-133d3a4b4c11520b3884`).
- `given_lineage_discontinuity_epoch_change_when_received_then_declared_discontinuity_is_counted` — given lineage discontinuity epoch change when received then declared discontinuity is counted (`src/runtime/audio/router.rs:1117`; `test-fceb86228ea42976addb`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` — given lineaged source fan out when branch frames are copied then exact lineage is preserved (`src/runtime/audio/router.rs:1076`; `test-d798548d6c8b059ba1a8`).
- `given_observation_handle_when_consumer_detects_gap_then_live_discontinuity_is_visible` — given observation handle when consumer detects gap then live discontinuity is visible (`src/runtime/audio/router.rs:1410`; `test-225f3db0b8f734fb6907`).
- `given_observation_handle_when_producer_fills_edge_then_live_queue_and_drop_are_visible` — given observation handle when producer fills edge then live queue and drop are visible (`src/runtime/audio/router.rs:1374`; `test-7acd587c9b13ea33929e`).
- `given_observation_handle_when_receiver_drops_then_shutdown_snapshot_remains_available` — given observation handle when receiver drops then shutdown snapshot remains available (`src/runtime/audio/router.rs:1439`; `test-b85189f2f3734f3dad88`).
- `given_one_source_with_three_edges_when_dispatched_then_every_edge_receives_identified_frame` — given one source with three edges when dispatched then every edge receives identified frame (`src/runtime/audio/router.rs:1044`; `test-413eb70a225c14f5ec09`).
- `given_queued_frame_when_clocked_receive_runs_then_clock_is_sampled_after_pop` — given queued frame when clocked receive runs then clock is sampled after pop (`src/runtime/audio/router.rs:1217`; `test-f09950594dbe438e24cb`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` — given receive before enqueue when observed then latency sample is rejected (`src/runtime/audio/router.rs:1200`; `test-904f86f2ad1e1369647c`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Choose route capacity and loss policy](/docs/how-to/configure-route-policy.md)
- [Fan out one source](/docs/how-to/fan-out-source.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)

## Evidence boundary

The claims on **Size bounded routes from observations** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/audio/router.rs:1-1615` (`DIRECT`)
- `src/session/lifecycle/observations.rs:1-636` (`DIRECT`)

For **Size bounded routes from observations**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

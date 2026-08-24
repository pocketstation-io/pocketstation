# Size bounded routes from observations

<!-- claims: CLM-BEST-001-SCOPE-001,CLM-BEST-001-TEXT-001,CLM-BEST-001-TEXT-002,CLM-BEST-001-TEXT-003,CLM-BEST-001-TEXT-004,CLM-BEST-001-TEXT-005,CLM-BEST-001-SOURCE-001 -->

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

- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:1012`; `test-687c08c4ebc7699d891b`).
- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` — given enqueued and dropped frames when observed then drop rate uses all attempts (`src/runtime/audio/router.rs:1272`; `test-81f2a37c65fc1321fb4b`).
- `given_failed_branch_when_receiver_drops_then_unrelated_branch_continues` — given failed branch when receiver drops then unrelated branch continues (`src/runtime/audio/router.rs:1549`; `test-e79727ff2a1d9faecc74`).
- `given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated` — given foreign clock timestamp when delivered then source latency is not fabricated (`src/runtime/audio/router.rs:1211`; `test-2dfdf77222ba4754d494`).
- `given_lineage_discontinuity_epoch_change_when_received_then_declared_discontinuity_is_counted` — given lineage discontinuity epoch change when received then declared discontinuity is counted (`src/runtime/audio/router.rs:1146`; `test-6161d7a8c36359a8e55e`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` — given lineaged source fan out when branch frames are copied then exact lineage is preserved (`src/runtime/audio/router.rs:1105`; `test-8407a37a5b957f010ddf`).
- `given_observation_handle_when_consumer_detects_gap_then_live_discontinuity_is_visible` — given observation handle when consumer detects gap then live discontinuity is visible (`src/runtime/audio/router.rs:1441`; `test-b11b6230db89e523d9d4`).
- `given_observation_handle_when_producer_fills_edge_then_live_queue_and_drop_are_visible` — given observation handle when producer fills edge then live queue and drop are visible (`src/runtime/audio/router.rs:1405`; `test-26474ba2232fc8e073d2`).
- `given_observation_handle_when_receiver_drops_then_shutdown_snapshot_remains_available` — given observation handle when receiver drops then shutdown snapshot remains available (`src/runtime/audio/router.rs:1470`; `test-84a0ed752ce8338a3166`).
- `given_one_source_with_three_edges_when_dispatched_then_every_edge_receives_identified_frame` — given one source with three edges when dispatched then every edge receives identified frame (`src/runtime/audio/router.rs:1073`; `test-d96ed9959cf0c6f31b59`).
- `given_queued_frame_when_clocked_receive_runs_then_clock_is_sampled_after_pop` — given queued frame when clocked receive runs then clock is sampled after pop (`src/runtime/audio/router.rs:1246`; `test-b7f788e10424241552b8`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` — given receive before enqueue when observed then latency sample is rejected (`src/runtime/audio/router.rs:1229`; `test-0f111134a8c8ddb61a69`).

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

The claims on **Size bounded routes from observations** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/audio/router.rs:1-1` (`DECLARED`)
- `src/session/lifecycle/observations.rs:16-16` (`DIRECT`)
- `src/session/lifecycle/observations.rs:16-16` (`DIRECT`)
- `src/session/lifecycle/observations.rs:16-16` (`DIRECT`)
- `src/session/lifecycle/observations.rs:17-29` (`DIRECT`)
- `src/session/lifecycle/observations.rs:18-18` (`DIRECT`)
- `src/session/lifecycle/observations.rs:19-19` (`DIRECT`)
- `src/session/lifecycle/observations.rs:20-20` (`DIRECT`)
- `src/session/lifecycle/observations.rs:21-21` (`DIRECT`)
- `src/session/lifecycle/observations.rs:22-22` (`DIRECT`)
- `src/session/lifecycle/observations.rs:23-23` (`DIRECT`)
- `src/session/lifecycle/observations.rs:24-24` (`DIRECT`)
- `src/session/lifecycle/observations.rs:25-25` (`DIRECT`)
- `src/session/lifecycle/observations.rs:26-26` (`DIRECT`)
- `src/session/lifecycle/observations.rs:27-27` (`DIRECT`)
- `src/session/lifecycle/observations.rs:28-28` (`DIRECT`)
- `src/session/lifecycle/observations.rs:35-35` (`DIRECT`)
- `src/session/lifecycle/observations.rs:35-35` (`DIRECT`)
- `src/session/lifecycle/observations.rs:35-35` (`DIRECT`)
- `src/session/lifecycle/observations.rs:36-44` (`DIRECT`)
- `src/session/lifecycle/observations.rs:37-37` (`DIRECT`)
- `src/session/lifecycle/observations.rs:38-38` (`DIRECT`)
- `src/session/lifecycle/observations.rs:39-39` (`DIRECT`)
- `src/session/lifecycle/observations.rs:40-40` (`DIRECT`)
- `src/session/lifecycle/observations.rs:41-41` (`DIRECT`)

For **Size bounded routes from observations**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

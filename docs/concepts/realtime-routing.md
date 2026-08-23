# Realtime routing

<!-- claims: CLM-DOC-017-CAP-001,CLM-DOC-017-SOURCE-001 -->

## What it is

`RunningSession` owns the active Session engine, capture workers, routes, endpoints, observation handles, and finalization responsibilities until stop completes.

## Why it exists

Active resources need one visible lifetime owner. Retaining the running handle prevents stop, recording finalization, and component failures from being reduced to implicit drop behavior.

## Relationships

- Start transfers a prepared Session into `RunningSession`.
- Observation handles expose running metrics and events without taking ownership.
- Stop returns Session, component, recording, and trace outcomes.

## Invariants and guarantees

- Keep the running owner alive while consuming frames or observations.
- Stop and finalization are structured outcomes, not a Boolean success flag.
- Drop behavior does not create an undocumented retry or drain guarantee.

## When you encounter it

- **Return generated audio** — Bridge asynchronous PCM output back into the bounded audio lane.
- **Inject external PCM** — Acquire bounded buffers, write PCM, and observe source runtime outcomes.

## Use it

- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Running ownership](/docs/lifecycle/running.md)
- [Session stop reports failures](/docs/troubleshooting/session-stop.md)

## Scope

- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

The scope of **Realtime routing** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::runtime::audio::runner::RealtimePlanRunner` | struct | Executes realtime plan according to its compiled plan and cancellation contract. | `src/runtime/audio/runner.rs:305` |
| `is_realtime` | function | Returns whether realtime applies to `ClockDomain`. | `src/graph/ports.rs:259` |
| `realtime_audio` | function | Generic realtime PCM edge. Concrete sample rate, frame size, and channel layout are negotiated from connected ports. | `src/graph/ports.rs:391` |
| `pocketstation::graph::ports::DeliverySemantics::BestEffortRealtime` | variant | Identifies the best effort realtime state or stage represented by `DeliverySemantics`. | `src/graph/ports.rs:274` |
| `pocketstation::graph::ports::DeliverySemantics::ExactlyOnceNotRealtime` | variant | Identifies the exactly once not realtime state or stage represented by `DeliverySemantics`. | `src/graph/ports.rs:276` |
| `pocketstation::graph::ports::AudioCaps` | struct | Declares the sample formats, channel layouts, and rates accepted by an audio port. | `src/graph/ports.rs:48` |
| `pocketstation::graph::ports::EdgeContract` | struct | Declares the validated constraints applied to edge. | `src/graph/ports.rs:311` |
| `pocketstation::graph::ports::PortSpec` | struct | Configures port behavior at its owning API boundary. | `src/graph/ports.rs:175` |
| `pocketstation::runtime::audio::router::DispatchSummary` | struct | Reports the counters and terminal facts collected for dispatch. | `src/runtime/audio/router.rs:696` |
| `pocketstation::runtime::audio::router::EdgeObservations` | struct | Reports the edge observations collected at an observation boundary. | `src/runtime/audio/router.rs:142` |

## Executable evidence

Executable evidence selected for **Realtime routing** is limited to each test's recorded setup and assertions:

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

- [Architecture overview](/docs/architecture/overview.md)
- [Glossary](/docs/glossary.md)
- [Memory ownership and buffer pools](/docs/internals/memory-ownership.md)
- [PocketStation](/README.md)
- [Realtime audio lane](/docs/internals/realtime-audio-lane.md)
- [Choose route capacity and loss policy](/docs/how-to/configure-route-policy.md)
- [Fan out one source](/docs/how-to/fan-out-source.md)
- [Poll audio without unbounded buffering](/docs/how-to/poll-audio.md)

## Evidence boundary

The claims on **Realtime routing** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/audio/router.rs:1-1646` (`DIRECT`)

For **Realtime routing**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

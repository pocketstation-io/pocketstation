# Fan out one source

<!-- claims: CLM-GUIDE-007-CAP-001,CLM-GUIDE-007-CAP-002,CLM-GUIDE-007-CAP-003,CLM-GUIDE-007-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.
- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.

The scope of **Fan out one source** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

One declared source and two independently configured destinations.

## Procedure

1. Declare the source once.
2. Create each consumer endpoint independently.
3. Connect the same source output to each endpoint.
4. Set explicit edge policy where the default is unsuitable.
5. Observe each route separately so saturation remains attributable.

## Important consequence

Fan-out does not combine saturation: one constrained branch must remain attributable to that branch.

## Verify the outcome

The compiled plan contains distinct routes and each destination exposes its own delivery observations.

Executable evidence selected for **Fan out one source** is limited to each test's recorded setup and assertions:

- `given_foreign_clock_timestamp_when_delivered_then_source_latency_is_not_fabricated` — given foreign clock timestamp when delivered then source latency is not fabricated (`src/runtime/audio/router.rs:1211`; `test-2dfdf77222ba4754d494`).
- `given_lineaged_source_fan_out_when_branch_frames_are_copied_then_exact_lineage_is_preserved` — given lineaged source fan out when branch frames are copied then exact lineage is preserved (`src/runtime/audio/router.rs:1105`; `test-8407a37a5b957f010ddf`).
- `given_one_source_with_three_edges_when_dispatched_then_every_edge_receives_identified_frame` — given one source with three edges when dispatched then every edge receives identified frame (`src/runtime/audio/router.rs:1073`; `test-d96ed9959cf0c6f31b59`).
- `given_two_sources_with_six_edges_when_dispatched_then_source_identity_stays_separate` — given two sources with six edges when dispatched then source identity stays separate (`src/runtime/audio/router.rs:1312`; `test-a60ef29f8af2493db18f`).
- `given_compiled_text_edge_when_router_builds_then_only_audio_edge_gets_audio_receiver` — given compiled text edge when router builds then only audio edge gets audio receiver (`src/runtime/audio/router.rs:1012`; `test-687c08c4ebc7699d891b`).
- `given_enqueued_and_dropped_frames_when_observed_then_drop_rate_uses_all_attempts` — given enqueued and dropped frames when observed then drop rate uses all attempts (`src/runtime/audio/router.rs:1272`; `test-81f2a37c65fc1321fb4b`).
- `given_failed_branch_when_receiver_drops_then_unrelated_branch_continues` — given failed branch when receiver drops then unrelated branch continues (`src/runtime/audio/router.rs:1549`; `test-e79727ff2a1d9faecc74`).
- `given_lineage_discontinuity_epoch_change_when_received_then_declared_discontinuity_is_counted` — given lineage discontinuity epoch change when received then declared discontinuity is counted (`src/runtime/audio/router.rs:1146`; `test-6161d7a8c36359a8e55e`).
- `given_observation_handle_when_consumer_detects_gap_then_live_discontinuity_is_visible` — given observation handle when consumer detects gap then live discontinuity is visible (`src/runtime/audio/router.rs:1441`; `test-b11b6230db89e523d9d4`).
- `given_observation_handle_when_producer_fills_edge_then_live_queue_and_drop_are_visible` — given observation handle when producer fills edge then live queue and drop are visible (`src/runtime/audio/router.rs:1405`; `test-26474ba2232fc8e073d2`).
- `given_observation_handle_when_receiver_drops_then_shutdown_snapshot_remains_available` — given observation handle when receiver drops then shutdown snapshot remains available (`src/runtime/audio/router.rs:1470`; `test-84a0ed752ce8338a3166`).
- `given_queued_frame_when_clocked_receive_runs_then_clock_is_sampled_after_pop` — given queued frame when clocked receive runs then clock is sampled after pop (`src/runtime/audio/router.rs:1246`; `test-b7f788e10424241552b8`).

## Failure signals

- `pocketstation::runtime::audio::router::PlanRouterError` — `error-fb8f2ed6471ac52a3017`
- `pocketstation::runtime::audio::router::PlanRouterError` / `InvalidFrameBytes` — `error-852f32645f2215a240b9`
- `pocketstation::runtime::audio::router::PlanRouterError` / `MissingMemoryPlan` — `error-2bcbacd4cc3a2d562e0e`
- `pocketstation::runtime::audio::router::PlanRouterError` / `ZeroCapacity` — `error-feb909d9c49412065cd0`
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `GenerationRegressed` — `error-bbe408f369f895d713fa`
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `MissingLineage` — `error-ccd7392a345005050393`
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `RecoveryWithoutDiscontinuity` — `error-94952fdbb1e05f4d9023`
- `pocketstation::graph::signal::envelope::SignalEnvelopeError` / `SourceMismatch` — `error-bc8773e51233f2cfe9c6`
- `pocketstation::graph::signal::lineage::SignalLineageError` / `ZeroSourceGeneration` — `error-71921121e392143f921b`
- `pocketstation::runtime::audio::runner::PlanRunnerError` / `DuplicateSource` — `error-d8b900117b2d90e18ba5`

## API reference

- [Realtime Routing](/docs/concepts/realtime-routing.md)
- [Graph](/docs/reference/graph.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::runtime::audio::runner::PlanSourceInput` | struct | Carries typed input for plan source. | `src/runtime/audio/runner.rs:188` |
| `pocketstation::runtime::audio::runner::PlanSourceInputObservations` | struct | Reports the plan source input observations collected at an observation boundary. | `src/runtime/audio/runner.rs:22` |
| `pocketstation::runtime::audio::runner::PlanSourceObservationHandle` | struct | Owns bounded access to plan source observation. | `src/runtime/audio/runner.rs:138` |
| `pocketstation::runtime::audio::runner::PlanSourceSender` | struct | Sends plan source values across its declared ownership boundary. | `src/runtime/audio/runner.rs:131` |
| `pocketstation::session::declaration::draft::SourceInstanceHandle` | struct | Owns bounded access to source instance. | `src/session/declaration/draft.rs:834` |
| `pocketstation::session::declaration::draft::SourceOutputHandle` | struct | Owns bounded access to source output. | `src/session/declaration/draft.rs:910` |
| `pocketstation::session::declaration::spec::SourceInstanceId` | struct | Uniquely identifies source instance within its PocketStation ownership scope. | `src/session/declaration/spec.rs:14` |
| `pocketstation::session::declaration::spec::SourceInstanceSpec` | struct | Configures source instance behavior at its owning API boundary. | `src/session/declaration/spec.rs:68` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Choose route capacity and loss policy](/docs/how-to/configure-route-policy.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)

## Evidence boundary

The claims on **Fan out one source** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/declaration/draft.rs:1-1405` (`DIRECT`)
- `src/runtime/audio/router.rs:1-1646` (`DIRECT`)

For **Fan out one source**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

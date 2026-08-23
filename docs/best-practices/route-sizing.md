# Size bounded routes from observations

<!-- claims: CLM-BEST-001-CAP-001,CLM-BEST-001-CAP-002,CLM-BEST-001-CAP-003,CLM-BEST-001-SOURCE-001 -->

## Recommendation

Measure each bounded route's queue, saturation, drop, and latency observations before changing capacity.

## Why

The repository makes capacity, ownership, identity, lifecycle, and evidence boundaries explicit so failures remain attributable. Bypassing them removes observations and typed outcomes needed for diagnosis.

## Tradeoff

The recommendation requires explicit configuration and result handling. It does not promise that one capacity, retry budget, selector, or shutdown policy fits every workload. Measure within the API's stated scope.

## When it does not apply

Do not apply a realtime, connector, capture, or extension rule to another lane or boundary unless it exposes the same contract. An internal pattern is not automatically a public recommendation.

## Repository evidence

- `typed_error` at `src/runtime/audio/runner.rs` (`pattern-007b832d1ee35a325c5f`).
- `buffer_pool` at `src/runtime/nodes.rs` (`pattern-06b6cb48eb37e9925506`).
- `sidecar_isolation` at `src/runtime/lifecycle/async_host.rs` (`pattern-0b62d1010395e9d56174`).
- `buffer_pool` at `benches/runtime_plan.rs` (`pattern-0bed8aaed06c69250bca`).
- `bounded_queue` at `src/graph/ports.rs` (`pattern-0e9aafce5b5917888ef1`).
- `sidecar_isolation` at `benches/runtime_plan.rs` (`pattern-190078991e6fb29b089b`).
- `buffer_pool` at `src/graph/signal/envelope.rs` (`pattern-2c19d9af46f0136c5084`).
- `bounded_queue` at `src/graph/compile/plan.rs` (`pattern-2d4589efbc6a28e1e690`).
- `typed_error` at `src/graph/named_ports.rs` (`pattern-2eceeaaee3e6da04704d`).
- `buffer_pool` at `src/runtime/audio/executor.rs` (`pattern-3caaa68fb904e3dab03b`).
- `buffer_pool` at `src/graph/plan.rs` (`pattern-3cba3102bace53b5f813`).
- `sidecar_isolation` at `src/runtime/signal/operator.rs` (`pattern-3da02ff64109c6786946`).
- `typed_error` at `examples/operator-consumer/src/lib.rs` (`pattern-3dd96e814fdbb6001ead`).
- `typed_error` at `src/runtime/lifecycle/async_host.rs` (`pattern-4b4827ead247761ef305`).
- `typed_error` at `src/graph/runtime_node.rs` (`pattern-4b495789777872e04c34`).
- `sidecar_isolation` at `src/runtime/signal/io.rs` (`pattern-520a0bbdfc42436280d2`).
- `sidecar_isolation` at `src/runtime/audio/router.rs` (`pattern-545a8ffd6d5188242445`).
- `buffer_pool` at `src/runtime/audio/runner.rs` (`pattern-550a0db4c916e2cf76fd`).
- `buffer_pool` at `src/graph/compile/plan.rs` (`pattern-5961667d513f1fd57f10`).
- `sidecar_isolation` at `src/runtime/signal/edge.rs` (`pattern-5b4a5379bfca29277451`).

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_explicit_jitter_budget_when_planned_then_bounded_capacity_is_derived_from_frame_time` — given explicit jitter budget when planned then bounded capacity is derived from frame time (`src/graph/compile/plan.rs:755`; `test-28b5b52333fa1672705d`).
- `given_realtime_to_external_edge_when_planned_then_branch_pool_isolated_from_capture_pool` — given realtime to external edge when planned then branch pool isolated from capture pool (`src/graph/compile/plan.rs:591`; `test-ab3f88fbe7eaddfa92c8`).
- `given_async_producer_into_realtime_consumer_with_bounded_edge_when_compiled_then_invalid_realtime_edge` — given async producer into realtime consumer with bounded edge when compiled then invalid realtime edge (`src/graph/compile/resolve.rs:1122`; `test-f6fe68993af8cd133a10`).
- `given_unspecified_binary_edge_when_compiled_then_bounded_async_contract_is_derived` — given unspecified binary edge when compiled then bounded async contract is derived (`src/graph/compile/resolve.rs:994`; `test-8c765cd365efcd62766c`).
- `given_first_party_configuration_keys_when_audited_then_each_is_bounded_snake_case` — given first party configuration keys when audited then each is bounded snake case (`src/graph/identifier.rs:168`; `test-9d098b7cf4d784a630d2`).
- `given_bounded_async_when_built_then_contains_no_payload_or_clock_origin_assumption` — given bounded async when built then contains no payload or clock origin assumption (`src/graph/ports.rs:537`; `test-764d8a62597c3f9220c7`).
- `given_audio_and_typed_root_outputs_when_planned_then_each_branch_has_bounded_authority` — given audio and typed root outputs when planned then each branch has bounded authority (`src/graph/source.rs:51`; `test-d7ac5b66a6a72ddc6e51`).
- `observations` — observations (`src/runtime/audio/executor.rs:185`; `test-8e5dda8471ef4129edb9`).
- `from` — from (`src/runtime/audio/router.rs:510`; `test-bd1711e374cc4ec84e26`).
- `observations` — observations (`src/runtime/audio/router.rs:849`; `test-75f0a25930a60efd39e9`).
- `given_queued_sources_when_cancelled_with_budget_then_drain_is_bounded_and_rest_discards` — given queued sources when cancelled with budget then drain is bounded and rest discards (`src/runtime/audio/runner.rs:636`; `test-31632b8eb3f0b3c90934`).
- `observations` — observations (`src/runtime/audio/runner.rs:174`; `test-b1965f6e40d10be0df1e`).

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/audio/router.rs:1-1615` (`DIRECT`)
- `src/session/lifecycle/observations.rs:1-636` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

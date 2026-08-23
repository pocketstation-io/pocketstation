# Realtime routing

<!-- claims: CLM-DOC-017-CAP-001,CLM-DOC-017-SOURCE-001 -->

Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

## Scope

- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `realtime_audio` | function | Generic realtime PCM edge. Concrete sample rate, frame size, and channel layout are negotiated from connected ports. | `src/graph/ports.rs:391` |
| `requires_realtime_safety` | function | Returns `true` if the partition requires strict real-time safety. | `src/graph/partition.rs:55` |
| `pocketstation::graph::partition::ExecutionPartition::RealtimeCpu` | variant | Dedicated real-time processing thread. | `src/graph/partition.rs:30` |
| `pocketstation::graph::partition::SafetyContract::RealtimeSafe` | variant | No heap allocation, no locking, no blocking, no logging. | `src/graph/partition.rs:87` |
| `is_realtime` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:259` |
| `pocketstation::graph::compile::resolve::CompileError::InvalidRealtimeEdge` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/compile/resolve.rs:58` |
| `pocketstation::graph::ports::DeliverySemantics::BestEffortRealtime` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:274` |
| `pocketstation::graph::ports::DeliverySemantics::ExactlyOnceNotRealtime` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/ports.rs:276` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifestError::RealtimePartition` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:333` |
| `CompileError::InvalidRealtimeEdge::edge` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/compile/resolve.rs:58` |
| `CompileError::InvalidRealtimeEdge::reason` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/compile/resolve.rs:58` |
| `audio` | module | Allocation-free realtime audio execution lane. | `src/runtime/audio/mod.rs:1` |
| `lifecycle` | module | Non-realtime runtime ownership and process-protocol lifecycle. | `src/runtime/lifecycle/mod.rs:1` |
| `pocketstation::graph` | module | Stable signal, port, capability, partition, and extension contracts. | `src/graph/mod.rs:1` |
| `signal` | module | Bounded asynchronous signal execution lane. | `src/runtime/signal/mod.rs:1` |
| `pocketstation::graph::runtime_node::RuntimeNode` | trait | Realtime invariant: for nodes whose ExecutionClass::is_realtime is true, process() must stay alloc-free, lock-free, log-free, and blocking-free (LAW 15). All working state is sized once in prepare() and reused for the lifetime of the node. | `src/graph/runtime_node.rs:7` |
| `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| `pocketstation::graph::node::PortPrepareContext` | struct | Exact graph-owned contract for one prepared node port. | `src/graph/node.rs:282` |
| `pocketstation::graph::operator::OperatorId` | struct | Open identifier for a registered graph operator implementation. | `src/graph/operator.rs:16` |
| `pocketstation::graph::signal::lineage::SignalDerivation` | struct | Source-independent record of the signal consumed by an operator. | `src/graph/signal/lineage.rs:97` |

## Where you encounter it

- **Return generated audio** — Bridge asynchronous PCM output back into the bounded audio lane.
- **Inject external PCM** — Acquire bounded buffers, write PCM, and observe source runtime outcomes.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_linear_realtime_chain_when_planned_then_single_partition_and_topo_order` — given linear realtime chain when planned then single partition and topo order (`src/graph/compile/plan.rs:834`; `test-9570c9e13e72112e73ef`).
- `given_linear_realtime_graph_when_planned_then_single_partition_in_topo_order` — given linear realtime graph when planned then single partition in topo order (`src/graph/compile/plan.rs:540`; `test-0ad4ea1abd7124d12740`).
- `given_realtime_and_model_remote_nodes_when_planned_then_two_partitions_ordered_by_rank` — given realtime and model remote nodes when planned then two partitions ordered by rank (`src/graph/compile/plan.rs:561`; `test-917d8ff26f7ff8e13525`).
- `given_realtime_consumers_when_planned_then_every_edge_buffered_and_pool_positive` — given realtime consumers when planned then every edge buffered and pool positive (`src/graph/compile/plan.rs:706`; `test-977e6826ff8e82a0a94c`).
- `given_realtime_to_external_edge_when_planned_then_branch_pool_isolated_from_capture_pool` — given realtime to external edge when planned then branch pool isolated from capture pool (`src/graph/compile/plan.rs:591`; `test-ab3f88fbe7eaddfa92c8`).
- `given_async_producer_into_realtime_consumer_with_bounded_edge_when_compiled_then_invalid_realtime_edge` — given async producer into realtime consumer with bounded edge when compiled then invalid realtime edge (`src/graph/compile/resolve.rs:1122`; `test-f6fe68993af8cd133a10`).
- `given_async_producer_into_realtime_consumer_with_drop_newest_edge_when_compiled_then_ok` — given async producer into realtime consumer with drop newest edge when compiled then ok (`src/graph/compile/resolve.rs:1142`; `test-92bc66964b3eeb57cf0b`).
- `given_realtime_node_with_allocating_safety_when_compiled_then_safety_is_rejected` — given realtime node with allocating safety when compiled then safety is rejected (`src/graph/compile/resolve.rs:1109`; `test-3dd54e90aaf4832a8387`).
- `given_audio_callback_partition_when_requires_realtime_safety_then_true` — given audio callback partition when requires realtime safety then true (`src/graph/partition.rs:128`; `test-d54ff707a3d77d445397`).
- `given_non_realtime_partitions_when_requires_realtime_safety_then_false` — given non realtime partitions when requires realtime safety then false (`src/graph/partition.rs:134`; `test-b898c8afe76701619b1a`).
- `given_realtime_safe_contract_when_valid_for_audio_callback_then_true` — given realtime safe contract when valid for audio callback then true (`src/graph/partition.rs:162`; `test-0ed6d5e5aedfb9880e78`).
- `given_realtime_audio_when_built_then_physical_caps_remain_negotiable` — given realtime audio when built then physical caps remain negotiable (`src/graph/ports.rs:520`; `test-31c4ce2508a308db7cb9`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/audio/router.rs:1-1615` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

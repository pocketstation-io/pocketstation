# Graph contracts

<!-- claims: CLM-DOC-020-CAP-001,CLM-DOC-020-SOURCE-001 -->

Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.

## Scope

- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph` | module | Stable signal, port, capability, partition, and extension contracts. | `src/graph/mod.rs:1` |
| `pocketstation::graph::runtime_node::RuntimeNode` | trait | Realtime invariant: for nodes whose ExecutionClass::is_realtime is true, process() must stay alloc-free, lock-free, log-free, and blocking-free (LAW 15). All working state is sized once in prepare() and reused for the lifetime of the node. | `src/graph/runtime_node.rs:7` |
| `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| `pocketstation::graph::node::PortPrepareContext` | struct | Exact graph-owned contract for one prepared node port. | `src/graph/node.rs:282` |
| `pocketstation::graph::operator::OperatorId` | struct | Open identifier for a registered graph operator implementation. | `src/graph/operator.rs:16` |
| `pocketstation::graph::signal::lineage::SignalDerivation` | struct | Source-independent record of the signal consumed by an operator. | `src/graph/signal/lineage.rs:97` |
| `pocketstation::graph::signal::preparation::AsyncOperatorPrepareContext` | struct | Complete graph-owned preparation contract for one asynchronous Operator. | `src/graph/signal/preparation.rs:22` |
| `pocketstation::graph::signal::spec::SchemaRef` | struct | Reference to an external schema document. | `src/graph/signal/spec.rs:87` |
| `pocketstation::graph::signal::spec::SemanticRole` | struct | Semantic role annotation on a port. | `src/graph/signal/spec.rs:57` |
| `pocketstation::graph::signal::spec::SignalId` | struct | Opaque identifier for a custom signal type. | `src/graph/signal/spec.rs:22` |
| `pocketstation::graph::signal::spec::SignalSpec` | struct | Full signal contract for a single port. | `src/graph/signal/spec.rs:205` |
| `pocketstation::graph::partition::ExecutionPartition` | enum | WHERE an operator runs. | `src/graph/partition.rs:18` |
| `pocketstation::graph::partition::SafetyContract` | enum | WHAT an operator guarantees about its runtime behaviour. | `src/graph/partition.rs:82` |
| `pocketstation::graph::signal::spec::BinaryFormat` | enum | Binary encoding hint for `SignalClass::Binary`. | `src/graph/signal/spec.rs:141` |
| `pocketstation::graph::signal::spec::Codec` | enum | Audio encoding format for `SignalClass::EncodedAudio`. | `src/graph/signal/spec.rs:113` |
| `pocketstation::graph::signal::spec::EventFormat` | enum | Event structure hint for `SignalClass::Event`. | `src/graph/signal/spec.rs:132` |
| `pocketstation::graph::signal::spec::SignalClass` | enum | The fundamental class of data flowing through a port. | `src/graph/signal/spec.rs:156` |
| `pocketstation::graph::signal::spec::TextFormat` | enum | Text encoding hint for `SignalClass::Text`. | `src/graph/signal/spec.rs:124` |
| `pocketstation::graph::signal::preparation::AsyncOperatorEdgePrepareContext` | type_alias | Exact bounded graph edge supplied to an asynchronous Operator at prepare time. | `src/graph/signal/preparation.rs:18` |
| `pocketstation::graph::partition::ExecutionPartition::AsyncWorker` | variant | Tokio async task. | `src/graph/partition.rs:36` |

## Where you encounter it

- **Add an asynchronous operator** — Declare typed ports, implement an operator factory, and route its output.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_compiled_graph_when_instrumented_then_metric_ids_are_stable_and_distinct` — given compiled graph when instrumented then metric ids are stable and distinct (`src/graph/compile/plan.rs:785`; `test-1c092b4376cfedf5e86d`).
- `given_fixed_graph_when_planned_then_runtime_plan_matches_golden_snapshot` — given fixed graph when planned then runtime plan matches golden snapshot (`src/graph/compile/plan.rs:803`; `test-79ec9d2ff6b56808169b`).
- `given_linear_realtime_graph_when_planned_then_single_partition_in_topo_order` — given linear realtime graph when planned then single partition in topo order (`src/graph/compile/plan.rs:540`; `test-0ad4ea1abd7124d12740`).
- `given_fixed_graph_when_compiled_then_topo_order_matches_golden_snapshot` — given fixed graph when compiled then topo order matches golden snapshot (`src/graph/compile/resolve.rs:1252`; `test-6cbc98440137003f4368`).
- `given_linear_graph_when_compiled_then_topo_orders_source_before_sink` — given linear graph when compiled then topo orders source before sink (`src/graph/compile/resolve.rs:973`; `test-7ece727a2fa318f311df`).
- `given_default_graph_spec_when_built_then_has_no_nodes_or_edges` — given default graph spec when built then has no nodes or edges (`src/graph/spec.rs:80`; `test-3db527034c49600287e9`).
- `given_node_spec_in_graph_when_looked_up_by_id_then_returns_it` — given node spec in graph when looked up by id then returns it (`src/graph/spec.rs:87`; `test-efb53040e5153777b34b`).
- `given_public_facade_when_typed_delivery_declared_then_internal_graph_is_not_required` — given public facade when typed delivery declared then internal graph is not required (`tests/operator_declaration.rs:4`; `test-ea6acbe3b724189ac6ad`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-ace9b7d11da2036ce899`).
- `given_gain_config_with_non_numeric_gain_db_when_validate_then_invalid_error` — given gain config with non numeric gain db when validate then invalid error (`src/graph/builtins.rs:256`; `test-0e41065f28a838e0deaf`).
- `given_gain_config_with_valid_gain_db_when_validate_then_ok` — given gain config with valid gain db when validate then ok (`src/graph/builtins.rs:264`; `test-c5d54824499f245c4c6c`).
- `given_gain_config_without_gain_db_when_validate_then_missing_error` — given gain config without gain db when validate then missing error (`src/graph/builtins.rs:249`; `test-c2584e0bcdbbb154dfa1`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Choose route capacity and loss policy](/docs/how-to/configure-route-policy.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Fan out one source](/docs/how-to/fan-out-source.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/ports.rs:1-618` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

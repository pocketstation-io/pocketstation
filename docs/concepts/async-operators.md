# Asynchronous operators

<!-- claims: CLM-DOC-021-CAP-001,CLM-DOC-021-SOURCE-001 -->

Register operator factories that consume and emit named typed signals on the asynchronous execution lane.

## Scope

- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `audio` | module | Allocation-free realtime audio execution lane. | `src/runtime/audio/mod.rs:1` |
| `lifecycle` | module | Non-realtime runtime ownership and process-protocol lifecycle. | `src/runtime/lifecycle/mod.rs:1` |
| `pocketstation::graph` | module | Stable signal, port, capability, partition, and extension contracts. | `src/graph/mod.rs:1` |
| `signal` | module | Bounded asynchronous signal execution lane. | `src/runtime/signal/mod.rs:1` |
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

## Where you encounter it

- **Add an asynchronous operator** — Declare typed ports, implement an operator factory, and route its output.
- **Return generated audio** — Bridge asynchronous PCM output back into the bounded audio lane.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` — given operator composition with three external operators then derived output crosses each bounded edge (`src/runtime/signal/operator.rs:1859`; `test-9ec51c75cedb5ffaef0f`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-ace9b7d11da2036ce899`).
- `given_gain_config_with_non_numeric_gain_db_when_validate_then_invalid_error` — given gain config with non numeric gain db when validate then invalid error (`src/graph/builtins.rs:256`; `test-0e41065f28a838e0deaf`).
- `given_gain_config_with_valid_gain_db_when_validate_then_ok` — given gain config with valid gain db when validate then ok (`src/graph/builtins.rs:264`; `test-c5d54824499f245c4c6c`).
- `given_gain_config_without_gain_db_when_validate_then_missing_error` — given gain config without gain db when validate then missing error (`src/graph/builtins.rs:249`; `test-c2584e0bcdbbb154dfa1`).
- `given_mono_frame_when_mono_mixed_then_frame_is_unchanged` — given mono frame when mono mixed then frame is unchanged (`src/graph/builtins.rs:330`; `test-d76ec44bacdca3f6a506`).
- `given_passthrough_node_when_process_then_returns_frame_unchanged` — given passthrough node when process then returns frame unchanged (`src/graph/builtins.rs:300`; `test-681eed046f58e8486db9`).
- `given_six_db_gain_node_when_process_then_samples_scaled_by_linear_gain` — given six db gain node when process then samples scaled by linear gain (`src/graph/builtins.rs:286`; `test-7b9d00604bd1f54f6bfb`).
- `given_stereo_frame_when_mono_mixed_then_channels_and_samples_are_downmixed` — given stereo frame when mono mixed then channels and samples are downmixed (`src/graph/builtins.rs:313`; `test-f022cf78595274582c24`).
- `given_unity_gain_node_when_process_then_samples_unchanged` — given unity gain node when process then samples unchanged (`src/graph/builtins.rs:271`; `test-aac58d9d619b8280715f`).
- `given_compiled_graph_when_instrumented_then_metric_ids_are_stable_and_distinct` — given compiled graph when instrumented then metric ids are stable and distinct (`src/graph/compile/plan.rs:785`; `test-1c092b4376cfedf5e86d`).
- `given_copy_to_branch_pool_edge_when_planned_then_copy_pool_memory_is_reserved` — given copy to branch pool edge when planned then copy pool memory is reserved (`src/graph/compile/plan.rs:734`; `test-1d8783b870fa351a933b`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Asynchronous signal lane](/docs/internals/async-signal-lane.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Return generated PCM through a bridge](/docs/how-to/return-generated-audio.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/signal/operator.rs:1-380` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

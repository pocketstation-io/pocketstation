# Return generated PCM through a bridge

<!-- claims: CLM-GUIDE-013-CAP-001,CLM-GUIDE-013-CAP-002,CLM-GUIDE-013-SOURCE-001 -->

## Scope

- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Declare generated-audio output.
2. Prepare the bounded audio-reentry bridge.
3. Produce PCM matching the target sample specification.
4. Write from the asynchronous lane.
5. Observe accepted, saturated, closed, or cancelled outcomes.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `needs_bridge_to` | function | Returns `true` if crossing from `self` to `other` requires a compiler-inserted Bridge. | `src/graph/partition.rs:71` |
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

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_passthrough_node_when_process_then_returns_frame_unchanged` — given passthrough node when process then returns frame unchanged (`src/graph/builtins.rs:300`; `test-681eed046f58e8486db9`).
- `given_non_numeric_config_when_get_f32_then_returns_none` — given non numeric config when get f32 then returns none (`src/graph/node.rs:384`; `test-e8a88787bf728526e24e`).
- `given_non_numeric_config_when_get_u32_then_returns_none` — given non numeric config when get u32 then returns none (`src/graph/node.rs:396`; `test-822e3a824202ce79818e`).
- `given_different_partitions_when_needs_bridge_then_true` — given different partitions when needs bridge then true (`src/graph/partition.rs:156`; `test-715ab55819026a5e2ee1`).
- `given_same_partition_when_needs_bridge_then_false` — given same partition when needs bridge then false (`src/graph/partition.rs:150`; `test-eff95761245ec9c01147`).
- `given_mono_and_stereo_when_channel_count_then_returns_one_and_two` — given mono and stereo when channel count then returns one and two (`src/graph/ports.rs:441`; `test-8304caec6a9e3b31e801`).
- `given_empty_registry_when_get_unknown_then_returns_none` — given empty registry when get unknown then returns none (`src/graph/registry.rs:196`; `test-b951b0b08bbe3b9fb23a`).
- `given_registered_factory_when_get_then_returns_some` — given registered factory when get then returns some (`src/graph/registry.rs:187`; `test-c3d829c8e4d87e90b4f0`).
- `given_echo_async_node_when_process_after_prepare_then_envelope_is_returned` — given echo async node when process after prepare then envelope is returned (`src/graph/signal/envelope.rs:233`; `test-9d67f3359220613efda8`).
- `given_echo_async_node_when_process_before_prepare_then_error_is_returned` — given echo async node when process before prepare then error is returned (`src/graph/signal/envelope.rs:251`; `test-bfea57e87a139988d3b9`).
- `given_semantic_role_when_as_str_then_returns_inner` — given semantic role when as str then returns inner (`src/graph/signal/spec.rs:428`; `test-e0dd4a7d461ad612034d`).
- `given_signal_id_when_as_str_then_returns_inner` — given signal id when as str then returns inner (`src/graph/signal/spec.rs:422`; `test-3fa05de5ac89880ef9e0`).

## Failure signals

- `pocketstation::graph::node::NodeDescriptorError` / `InvalidSafetyContract` — `error-04b7031025a9b635fdbf`
- `pocketstation::graph::node::ConfigError` — `error-0be8ad81000b2924c24c`
- `pocketstation::graph::compile::resolve::CompileError` — `error-0da3f91a5f274a27ab76`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `ZeroProcessTimeout` — `error-10e3a522fa28fccdfc60`
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `InvalidMagic` — `error-143cce14f0e71f68c4cf`
- `pocketstation::graph::signal::operator::OperatorFailurePolicy` / `StopWorker` — `error-14ca51fa44623142d004`
- `pocketstation::graph::node::NodeError` / `Process` — `error-170066b0b40a26e0e33d`
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `SequenceGapWithoutDiscontinuity` — `error-18565faf820bbf8e2650`
- `pocketstation::graph::compile::resolve::CompileError` / `MediaMismatch` — `error-1877b4a7bdffa5d7ed88`
- `pocketstation::graph::signal::continuity::SignalContinuityError` / `InvalidEnvelope` — `error-1897c7da4711d75eb14d`
- `pocketstation::graph::plan::PlanError` / `MoveExclusiveFanOut` — `error-18d1485abaf31198b6d8`
- `pocketstation::graph::node::NodeDescriptorError` / `EmptyDisplayName` — `error-1981cbd27763ca5ffcbe`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Graph and signal failures](/docs/errors/graph-and-signals.md)
- [Asynchronous operators](/docs/concepts/async-operators.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/bridge/audio.rs:1-529` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

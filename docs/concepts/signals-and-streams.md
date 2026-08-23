# Signals and streams

<!-- claims: CLM-DOC-022-CAP-001,CLM-DOC-022-SOURCE-001 -->

Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.

## Scope

- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::signal::spec::SignalSpec` | struct | Full signal contract for a single port. | `src/graph/signal/spec.rs:205` |
| `pocketstation::graph::signal::spec::SignalSpecError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/spec.rs:351` |
| `capacity_signals` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/node.rs:361` |
| `pocketstation::graph::signal::envelope::SignalEnvelopeError::InvalidSignalSpec` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/envelope.rs:139` |
| `pocketstation::graph::signal::spec::SignalSpecError::EmptyCustomId` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/spec.rs:353` |
| `pocketstation::graph::signal::spec::SignalSpecError::EmptyRole` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/spec.rs:355` |
| `pocketstation::graph::signal::spec::SignalSpecError::EmptySchema` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/spec.rs:357` |
| `SignalEdgeObservations::capacity_signals` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/signal/edge.rs:32` |
| `SignalEdgeObservations::depth_signals` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/signal/edge.rs:35` |
| `SignalEdgeObservations::peak_depth_signals` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/signal/edge.rs:36` |
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

## Where you encounter it

- **Add an asynchronous operator** — Declare typed ports, implement an operator factory, and route its output.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_supported_non_audio_signals_when_checked_then_media_is_symmetric` — given supported non audio signals when checked then media is symmetric (`src/graph/ports.rs:559`; `test-d97a306ad6dc3558e082`).
- `given_contiguous_signals_when_replayed_then_continuity_is_deterministic` — given contiguous signals when replayed then continuity is deterministic (`src/graph/signal/envelope.rs:390`; `test-d0dc80cc2da279b6a618`).
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

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Architecture overview](/docs/architecture/overview.md)
- [Asynchronous signal lane](/docs/internals/async-signal-lane.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Rust API reference](/docs/reference/rust-api.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/graph/signal/envelope.rs:1-444` (`DIRECT`)
- `src/session/declaration/typed_stream.rs:1-215` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

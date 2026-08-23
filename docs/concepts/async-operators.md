# Asynchronous operators

<!-- claims: CLM-DOC-021-CAP-001,CLM-DOC-021-SOURCE-001 -->

## What it is

An asynchronous operator is a registered factory and runtime node that consumes and emits named typed signals outside the realtime callback lane.

## Why it exists

Model calls, network I/O, and asynchronous processing need allocation and awaiting that are not valid in the realtime audio path. The operator boundary moves that work onto an explicit async partition.

## Relationships

- `AsyncOperatorManifest` declares ports and execution requirements.
- A factory prepares a node for one compiled operator instance.
- Generated PCM returns to the audio lane only through the audio-reentry bridge.

## Invariants and guarantees

- Ports used by the declaration must exist in the registered manifest.
- Output signal identity and media must match the declared port.
- Cancellation and final output follow the operator runtime contract.

## When you encounter it

- **Add an asynchronous operator** — Declare typed ports, implement an operator factory, and route its output.
- **Return generated audio** — Bridge asynchronous PCM output back into the bounded audio lane.

## Use it

- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Return generated PCM](/docs/how-to/return-generated-audio.md)

## Scope

- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.

The scope of **Asynchronous operators** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| `pocketstation::graph::signal::operator::AsyncOperatorFactory` | trait | Implement this trait to provide async operator behavior to PocketStation; its methods define the preparation and runtime contract. | `src/graph/signal/operator.rs:368` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifest` | struct | Describes the async operator manifest contract. | `src/graph/signal/operator.rs:127` |
| `pocketstation::graph::signal::operator::OperatorDeadlinePolicy` | struct | Configures operator deadline behavior at its owning API boundary. | `src/graph/signal/operator.rs:52` |
| `pocketstation::graph::signal::operator::OperatorOutputRolePolicy` | struct | Configures operator output role behavior at its owning API boundary. | `src/graph/signal/operator.rs:69` |
| `pocketstation::graph::signal::operator::OperatorPermissionPolicy` | struct | Configures operator permission behavior at its owning API boundary. | `src/graph/signal/operator.rs:46` |
| `pocketstation::runtime::signal::operator::AsyncOperatorWorker` | struct | Owns the asynchronous operator task, typed I/O, cancellation, and terminal join result. | `src/runtime/signal/operator.rs:250` |
| `pocketstation::runtime::signal::operator::CompiledOperatorInputContract` | struct | Declares the validated constraints applied to compiled operator input. | `src/runtime/signal/operator.rs:103` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | enum | Classifies failures reported as async operator manifest error. | `src/graph/signal/operator.rs:321` |
| `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | Selects the operator cancellation policy used by PocketStation. | `src/graph/signal/operator.rs:57` |

## Executable evidence

Executable evidence selected for **Asynchronous operators** is limited to each test's recorded setup and assertions:

- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` — given operator composition with three external operators then derived output crosses each bounded edge (`src/runtime/signal/operator.rs:1859`; `test-eab3e581e210e0e82882`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-f98e0a98874ff7dfbdf8`).
- `given_gain_config_with_non_numeric_gain_db_when_validate_then_invalid_error` — given gain config with non numeric gain db when validate then invalid error (`src/graph/builtins.rs:256`; `test-029e0dcc2de8d59ff561`).
- `given_gain_config_with_valid_gain_db_when_validate_then_ok` — given gain config with valid gain db when validate then ok (`src/graph/builtins.rs:264`; `test-13fb010e32e78037a1e6`).
- `given_gain_config_without_gain_db_when_validate_then_missing_error` — given gain config without gain db when validate then missing error (`src/graph/builtins.rs:249`; `test-8e6d108129a9248596fb`).
- `given_mono_frame_when_mono_mixed_then_frame_is_unchanged` — given mono frame when mono mixed then frame is unchanged (`src/graph/builtins.rs:330`; `test-c4fded8eeabe88603a20`).
- `given_passthrough_node_when_process_then_returns_frame_unchanged` — given passthrough node when process then returns frame unchanged (`src/graph/builtins.rs:300`; `test-746c4f775eb51b59004b`).
- `given_six_db_gain_node_when_process_then_samples_scaled_by_linear_gain` — given six db gain node when process then samples scaled by linear gain (`src/graph/builtins.rs:286`; `test-c5eb65d3dbe60d07022e`).
- `given_stereo_frame_when_mono_mixed_then_channels_and_samples_are_downmixed` — given stereo frame when mono mixed then channels and samples are downmixed (`src/graph/builtins.rs:313`; `test-6ffe4edd1fbdc435fec2`).
- `given_unity_gain_node_when_process_then_samples_unchanged` — given unity gain node when process then samples unchanged (`src/graph/builtins.rs:271`; `test-2721547134c7c2fb6bc4`).
- `given_compiled_graph_when_instrumented_then_metric_ids_are_stable_and_distinct` — given compiled graph when instrumented then metric ids are stable and distinct (`src/graph/compile/plan.rs:785`; `test-aef519dc61d29ebde9b6`).
- `given_copy_to_branch_pool_edge_when_planned_then_copy_pool_memory_is_reserved` — given copy to branch pool edge when planned then copy pool memory is reserved (`src/graph/compile/plan.rs:734`; `test-1b94392ac4d9cdf72c6b`).

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

The claims on **Asynchronous operators** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/graph/signal/operator.rs:1-380` (`DIRECT`)

For **Asynchronous operators**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

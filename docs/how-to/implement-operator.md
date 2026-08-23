# Implement an asynchronous operator

<!-- claims: CLM-GUIDE-011-CAP-001,CLM-GUIDE-011-CAP-002,CLM-GUIDE-011-CAP-003,CLM-GUIDE-011-SOURCE-001 -->

## Scope

- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.

The scope of **Implement an asynchronous operator** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A unique operator and node identity, named typed ports, and an execution partition appropriate for the work.

## Procedure

1. Define named ports in AsyncOperatorManifest.
2. Implement factory preparation.
3. Return an async node that observes cancellation and declared policies.
4. Register before Session compilation.
5. Connect named ports and run the separate consumer example.

## Important consequence

Preparation, processing, timeout, final output, and cancellation failures are separate stages.

## Verify the outcome

The factory registers, the Session compiles, and the runtime receives and emits the declared signal types.

Executable evidence selected for **Implement an asynchronous operator** is limited to each test's recorded setup and assertions:

- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-f98e0a98874ff7dfbdf8`).
- `given_operator_id_when_serialized_then_version_one_is_a_transparent_string` — given operator id when serialized then version one is a transparent string (`src/graph/operator.rs:42`; `test-55db4ac70dd43bb79e41`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-aee462488aef78361374`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-1dfdf14fa335d99dccdc`).
- `given_cancellation_when_operator_has_pending_state_then_no_final_is_fabricated` — given cancellation when operator has pending state then no final is fabricated (`src/runtime/signal/operator.rs:2308`; `test-245c72f8c086e47e1ada`).
- `given_graceful_finish_when_operator_has_pending_state_then_one_final_is_emitted` — given graceful finish when operator has pending state then one final is emitted (`src/runtime/signal/operator.rs:2127`; `test-015530818eeaeba0077f`).
- `given_operator_composition_with_named_multi_input_output_manifest_then_each_declared_port_executes` — given operator composition with named multi input output manifest then each declared port executes (`src/runtime/signal/operator.rs:1916`; `test-bc69e1a774892a686b9f`).
- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` — given operator composition with three external operators then derived output crosses each bounded edge (`src/runtime/signal/operator.rs:1859`; `test-eab3e581e210e0e82882`).
- `given_slow_operator_when_deadline_expires_then_timeout_cancel_and_join_are_observed` — given slow operator when deadline expires then timeout cancel and join are observed (`src/runtime/signal/operator.rs:2489`; `test-d23b7905d713383361c0`).
- `given_public_session_when_named_operator_connected_then_one_instance_owns_both_inputs` — given public session when named operator connected then one instance owns both inputs (`tests/operator_declaration.rs:26`; `test-2076461784bfa8508fc9`).
- `given_gain_config_with_non_numeric_gain_db_when_validate_then_invalid_error` — given gain config with non numeric gain db when validate then invalid error (`src/graph/builtins.rs:256`; `test-029e0dcc2de8d59ff561`).
- `given_gain_config_with_valid_gain_db_when_validate_then_ok` — given gain config with valid gain db when validate then ok (`src/graph/builtins.rs:264`; `test-13fb010e32e78037a1e6`).

## Failure signals

- `pocketstation::graph::registry::NodeRegistrationError` / `DuplicateOperatorId` — `error-39c40c390407fd042e82`
- `pocketstation::graph::signal::lineage::SignalDerivationError` / `EmptyOperatorId` — `error-63c87ba1fc78022d5747`
- `pocketstation::graph::signal::lineage::SignalDerivationError` / `ZeroOperatorVersion` — `error-93ba6d12b46ef29abdd0`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` — `error-2cadbd0ae903fe4aa6b5`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `DuplicateOutputRole` — `error-8c0ef1b5899f1454c03c`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `EmptyOperatorId` — `error-8c504c413b46344e2975`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `EmptyOutputRole` — `error-5dc9640cde26000397ea`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `InputEdgeMediaMismatch` — `error-b5666a48719036a795fc`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `InputSignalMediaMismatch` — `error-127108f928b9312d9d03`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `InvalidInputSignal` — `error-dc219892488d8c72c4d1`

## API reference

- [Async Operators](/docs/concepts/async-operators.md)
- [Graph](/docs/reference/graph.md)

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

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [Graph and signal failures](/docs/errors/graph-and-signals.md)
- [Asynchronous signal lane](/docs/internals/async-signal-lane.md)

## Evidence boundary

The claims on **Implement an asynchronous operator** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `examples/operator-consumer/src/lib.rs:1-159` (`DIRECT`)

For **Implement an asynchronous operator**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

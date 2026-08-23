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

- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-ace9b7d11da2036ce899`).
- `given_operator_id_when_serialized_then_version_one_is_a_transparent_string` — given operator id when serialized then version one is a transparent string (`src/graph/operator.rs:42`; `test-91c6760e9ada9a5d771b`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-14905efc2e19f82a8cb2`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-0ae60369d5962ff55b0f`).
- `given_cancellation_when_operator_has_pending_state_then_no_final_is_fabricated` — given cancellation when operator has pending state then no final is fabricated (`src/runtime/signal/operator.rs:2308`; `test-8f495e7bcc9df9f06f5a`).
- `given_graceful_finish_when_operator_has_pending_state_then_one_final_is_emitted` — given graceful finish when operator has pending state then one final is emitted (`src/runtime/signal/operator.rs:2127`; `test-932daa336ec5d49f804c`).
- `given_operator_composition_with_named_multi_input_output_manifest_then_each_declared_port_executes` — given operator composition with named multi input output manifest then each declared port executes (`src/runtime/signal/operator.rs:1916`; `test-716956fd7ff21d2765ad`).
- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` — given operator composition with three external operators then derived output crosses each bounded edge (`src/runtime/signal/operator.rs:1859`; `test-9ec51c75cedb5ffaef0f`).
- `given_slow_operator_when_deadline_expires_then_timeout_cancel_and_join_are_observed` — given slow operator when deadline expires then timeout cancel and join are observed (`src/runtime/signal/operator.rs:2489`; `test-1bec17e5820e3c9ada80`).
- `given_public_session_when_named_operator_connected_then_one_instance_owns_both_inputs` — given public session when named operator connected then one instance owns both inputs (`tests/operator_declaration.rs:26`; `test-9e128271c40c45bb1c0b`).
- `given_gain_config_with_non_numeric_gain_db_when_validate_then_invalid_error` — given gain config with non numeric gain db when validate then invalid error (`src/graph/builtins.rs:256`; `test-0e41065f28a838e0deaf`).
- `given_gain_config_with_valid_gain_db_when_validate_then_ok` — given gain config with valid gain db when validate then ok (`src/graph/builtins.rs:264`; `test-c5d54824499f245c4c6c`).

## Failure signals

- `pocketstation::graph::registry::NodeRegistrationError` / `DuplicateOperatorId` — `error-237a7fc9fdd749cb3d97`
- `pocketstation::graph::signal::lineage::SignalDerivationError` / `EmptyOperatorId` — `error-174f3629452523371612`
- `pocketstation::graph::signal::lineage::SignalDerivationError` / `ZeroOperatorVersion` — `error-1133115bcece1357b725`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` — `error-dcf070f9ad7e215fb99b`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `DuplicateOutputRole` — `error-6b9ed517797bcbd2e801`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `EmptyOperatorId` — `error-57ab861621cd6b16c77f`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `EmptyOutputRole` — `error-8f1c68100994fda6e553`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `InputEdgeMediaMismatch` — `error-e6fe1486f6655f2d2e05`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `InputSignalMediaMismatch` — `error-a0e7136408cf72f15e15`
- `pocketstation::graph::signal::operator::AsyncOperatorManifestError` / `InvalidInputSignal` — `error-349333071b17fe6a6c02`

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

The claims on **Implement an asynchronous operator** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/operator-consumer/src/lib.rs:1-159` (`DIRECT`)

For **Implement an asynchronous operator**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

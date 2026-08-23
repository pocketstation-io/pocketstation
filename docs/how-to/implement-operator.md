# Implement an asynchronous operator

<!-- claims: CLM-GUIDE-011-CAP-001,CLM-GUIDE-011-CAP-002,CLM-GUIDE-011-CAP-003,CLM-GUIDE-011-SOURCE-001 -->

## Scope

- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Define named ports in AsyncOperatorManifest.
2. Implement factory preparation.
3. Return an async node that observes cancellation and declared policies.
4. Register before Session compilation.
5. Connect named ports and run the separate consumer example.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| `pocketstation::graph::operator::OperatorId` | struct | Open identifier for a registered graph operator implementation. | `src/graph/operator.rs:16` |
| `pocketstation::graph::signal::preparation::AsyncOperatorPrepareContext` | struct | Complete graph-owned preparation contract for one asynchronous Operator. | `src/graph/signal/preparation.rs:22` |
| `pocketstation::graph::signal::preparation::AsyncOperatorEdgePrepareContext` | type_alias | Exact bounded graph edge supplied to an asynchronous Operator at prepare time. | `src/graph/signal/preparation.rs:18` |
| `pocketstation::graph::signal::operator::AsyncOperatorFactory` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:368` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifest` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:127` |
| `pocketstation::graph::signal::operator::OperatorDeadlinePolicy` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:52` |
| `pocketstation::graph::signal::operator::OperatorOutputRolePolicy` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:69` |
| `pocketstation::graph::signal::operator::OperatorPermissionPolicy` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:46` |
| `pocketstation::runtime::signal::observations::AsyncOperatorObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/runtime/signal/observations.rs:29` |
| `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:321` |
| `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:57` |
| `pocketstation::graph::signal::operator::OperatorFailurePolicy` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:63` |
| `AsyncOperatorFactory::create` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:378` |
| `AsyncOperatorFactory::manifest` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:369` |
| `AsyncOperatorFactory::resolve_manifest` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:371` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_operator_id_when_serialized_then_version_one_is_a_transparent_string` — given operator id when serialized then version one is a transparent string (`src/graph/operator.rs:42`; `test-91c6760e9ada9a5d771b`).
- `given_lineaged_frame_when_realtime_operator_executes_then_output_keeps_capture_epochs` — given lineaged frame when realtime operator executes then output keeps capture epochs (`src/runtime/audio/executor.rs:411`; `test-14905efc2e19f82a8cb2`).
- `given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source` — given operator audio when bridged then owned frame enters bounded plan source (`src/runtime/bridge/audio.rs:335`; `test-0ae60369d5962ff55b0f`).
- `given_cancellation_when_operator_has_pending_state_then_no_final_is_fabricated` — given cancellation when operator has pending state then no final is fabricated (`src/runtime/signal/operator.rs:2308`; `test-8f495e7bcc9df9f06f5a`).
- `given_graceful_finish_when_operator_has_pending_state_then_one_final_is_emitted` — given graceful finish when operator has pending state then one final is emitted (`src/runtime/signal/operator.rs:2127`; `test-932daa336ec5d49f804c`).
- `given_operator_composition_with_named_multi_input_output_manifest_then_each_declared_port_executes` — given operator composition with named multi input output manifest then each declared port executes (`src/runtime/signal/operator.rs:1916`; `test-716956fd7ff21d2765ad`).
- `given_operator_composition_with_three_external_operators_then_derived_output_crosses_each_bounded_edge` — given operator composition with three external operators then derived output crosses each bounded edge (`src/runtime/signal/operator.rs:1859`; `test-9ec51c75cedb5ffaef0f`).
- `given_slow_operator_when_deadline_expires_then_timeout_cancel_and_join_are_observed` — given slow operator when deadline expires then timeout cancel and join are observed (`src/runtime/signal/operator.rs:2489`; `test-1bec17e5820e3c9ada80`).
- `given_public_session_when_named_operator_connected_then_one_instance_owns_both_inputs` — given public session when named operator connected then one instance owns both inputs (`tests/operator_declaration.rs:26`; `test-9e128271c40c45bb1c0b`).
- `given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api` — given external consumer when declared then provider and typed endpoint use public api (`examples/operator-consumer/src/lib.rs:120`; `test-ace9b7d11da2036ce899`).
- `given_gain_config_with_non_numeric_gain_db_when_validate_then_invalid_error` — given gain config with non numeric gain db when validate then invalid error (`src/graph/builtins.rs:256`; `test-0e41065f28a838e0deaf`).
- `given_gain_config_with_valid_gain_db_when_validate_then_ok` — given gain config with valid gain db when validate then ok (`src/graph/builtins.rs:264`; `test-c5d54824499f245c4c6c`).

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
- [Asynchronous signal lane](/docs/internals/async-signal-lane.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/operator-consumer/src/lib.rs:1-159` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

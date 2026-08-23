# Connect named operator ports

<!-- claims: CLM-GUIDE-012-CAP-001,CLM-GUIDE-012-CAP-002,CLM-GUIDE-012-CAP-003,CLM-GUIDE-012-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Retain typed output and input declaration handles.
2. Connect handles with compatible signal specifications.
3. Use exact port names from the manifest.
4. Compile and handle unknown, duplicate, or incompatible port errors.
5. Confirm the compiled binding targets the intended instance.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `SessionOperatorMetrics::input_ports` | struct_field | Exact per-port input accounting. `input_edge` is the compatibility aggregate across this slice. | `src/session/lifecycle/observations.rs:392` |
| `pocketstation::session::compile::error::SessionCompileError::DuplicateOperatorInputConnection` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/compile/error.rs:62` |
| `pocketstation::session::declaration::spec::ConnectionTarget::OperatorInput` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/declaration/spec.rs:225` |
| `ConnectionTarget::OperatorInput::input_port` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/declaration/spec.rs:227` |
| `ConnectionTarget::OperatorInput::operator_instance_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/declaration/spec.rs:226` |
| `SessionCompileError::DuplicateOperatorInputConnection::operator_instance_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/compile/error.rs:63` |
| `SessionCompileError::DuplicateOperatorInputConnection::port_name` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/compile/error.rs:64` |
| `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| `pocketstation::graph::operator::OperatorId` | struct | Open identifier for a registered graph operator implementation. | `src/graph/operator.rs:16` |
| `pocketstation::graph::signal::preparation::AsyncOperatorPrepareContext` | struct | Complete graph-owned preparation contract for one asynchronous Operator. | `src/graph/signal/preparation.rs:22` |
| `pocketstation::session::declaration::spec::ConnectionSpec` | struct | The single Session connection declaration used for every stream origin and every operator/endpoint destination. | `src/session/declaration/spec.rs:238` |
| `pocketstation::session::declaration::spec::ConnectionTarget` | enum | Stable destination of a declared Session connection. | `src/session/declaration/spec.rs:224` |
| `pocketstation::graph::signal::preparation::AsyncOperatorEdgePrepareContext` | type_alias | Exact bounded graph edge supplied to an asynchronous Operator at prepare time. | `src/graph/signal/preparation.rs:18` |
| `pocketstation::graph::ports::ClockDomain::Inherited` | variant | Preserve the clock carried by the producer's signal envelope. | `src/graph/ports.rs:254` |
| `SessionOperatorMetrics::input_edge` | struct_field | Sole counter authority for input delivered by the compiled Session plan. | `src/session/lifecycle/observations.rs:389` |
| `pocketstation::graph::signal::operator::AsyncOperatorFactory` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/graph/signal/operator.rs:368` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_declared_operator_when_named_ports_connected_then_one_instance_owns_all_connections` — given declared operator when named ports connected then one instance owns all connections (`src/session/declaration/tests/operator_connections.rs:19`; `test-858f27a0fc9c849c962b`).
- `given_one_named_operator_instance_when_compiled_then_all_named_connections_share_one_node` — given one named operator instance when compiled then all named connections share one node (`src/session/compile/tests.rs:308`; `test-c5a9415d19c5469b6f22`).
- `given_public_session_when_named_operator_connected_then_one_instance_owns_both_inputs` — given public session when named operator connected then one instance owns both inputs (`tests/operator_declaration.rs:26`; `test-9e128271c40c45bb1c0b`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` — given derived stream chain when compiled then operator output feeds next named input (`src/session/compile/tests.rs:587`; `test-e9a24a392741b4dbe6e7`).
- `given_duplicate_named_input_when_connected_then_declaration_fails_immediately` — given duplicate named input when connected then declaration fails immediately (`src/session/declaration/tests/operator_connections.rs:110`; `test-f9a6ec4f71dbaf6d8083`).
- `given_public_session_when_composed_then_three_stages_and_named_ports_run_under_one_owner` — given public session when composed then three stages and named ports run under one owner (`src/session/lifecycle/tests/running.rs:1755`; `test-42bab06a6020bf545d3a`).
- `given_connect_with_contract_when_into_spec_then_edge_carries_requested_contract` — given connect with contract when into spec then edge carries requested contract (`src/graph/dsl.rs:126`; `test-4a39ba3d3efea5bb8559`).
- `given_connected_nodes_when_into_spec_then_edge_records_endpoints` — given connected nodes when into spec then edge records endpoints (`src/graph/dsl.rs:111`; `test-6c7c927742d9d04f37dd`).
- `given_exact_named_inputs_when_compiled_then_one_multi_port_node_is_preserved` — given exact named inputs when compiled then one multi port node is preserved (`src/graph/named_ports.rs:67`; `test-898cd2ae0c6b04c280a5`).
- `given_unknown_named_input_when_compiled_then_failure_precedes_runtime` — given unknown named input when compiled then failure precedes runtime (`src/graph/named_ports.rs:87`; `test-547f292fc53c973e1c33`).
- `given_operator_id_when_serialized_then_version_one_is_a_transparent_string` — given operator id when serialized then version one is a transparent string (`src/graph/operator.rs:42`; `test-91c6760e9ada9a5d771b`).
- `given_operator_node_mismatch_when_compiled_then_typed_error_is_returned` — given operator node mismatch when compiled then typed error is returned (`src/session/compile/tests.rs:755`; `test-f27dda5da849d6322dd8`).

## Failure signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` — `error-00e5716261eba0f8cf3d`
- `pocketstation::session::error::SessionError` / `UnknownStem` — `error-00f6e798d158df66c847`
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` — `error-01d3fc855e2a00319076`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` — `error-023d6ab0b23a50a614ff`
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` — `error-0279b2b6b0cb3b5801bc`
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` — `error-037ddc3e193da74177f8`
- `pocketstation::graph::node::NodeDescriptorError` / `InvalidSafetyContract` — `error-04b7031025a9b635fdbf`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` — `error-05c60389efcb84311921`
- `pocketstation::session::prepare::error::SessionPrepareError` — `error-085082b521c14e5ecd1e`
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` — `error-08a7536094bfb2242b17`
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` — `error-09837185c7fca0f70618`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` — `error-0bc2f7c0b9f9dbf8ddd7`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Fan out one source](/docs/how-to/fan-out-source.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/declaration/tests/operator_connections.rs:1-152` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

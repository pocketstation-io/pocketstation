# Connect named operator ports

<!-- claims: CLM-GUIDE-012-CAP-001,CLM-GUIDE-012-CAP-002,CLM-GUIDE-012-CAP-003,CLM-GUIDE-012-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.

The scope of **Connect named operator ports** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A registered operator manifest and typed declaration handles from the same Session.

## Procedure

1. Retain typed output and input declaration handles.
2. Connect handles with compatible signal specifications.
3. Use exact port names from the manifest.
4. Compile and handle unknown, duplicate, or incompatible port errors.
5. Confirm the compiled binding targets the intended instance.

## Important consequence

Unknown, duplicate, foreign, or incompatible ports fail before runtime preparation.

## Verify the outcome

Compilation binds each named handle to the intended operator instance and compatible port.

Executable evidence selected for **Connect named operator ports** is limited to each test's recorded setup and assertions:

- `given_declared_operator_when_named_ports_connected_then_one_instance_owns_all_connections` — given declared operator when named ports connected then one instance owns all connections (`src/session/declaration/tests/operator_connections.rs:19`; `test-858f27a0fc9c849c962b`).
- `given_duplicate_named_input_when_connected_then_declaration_fails_immediately` — given duplicate named input when connected then declaration fails immediately (`src/session/declaration/tests/operator_connections.rs:110`; `test-f9a6ec4f71dbaf6d8083`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` — given foreign input handle when connected then declaration fails before freeze (`src/session/declaration/tests/operator_connections.rs:133`; `test-766194f5939b3ddb896d`).
- `given_through_sugar_when_frozen_then_canonical_instance_and_connection_records_are_used` — given through sugar when frozen then canonical instance and connection records are used (`src/session/declaration/tests/operator_connections.rs:81`; `test-716d5311ec84f59825a1`).
- `given_one_named_operator_instance_when_compiled_then_all_named_connections_share_one_node` — given one named operator instance when compiled then all named connections share one node (`src/session/compile/tests.rs:308`; `test-c5a9415d19c5469b6f22`).
- `given_public_session_when_named_operator_connected_then_one_instance_owns_both_inputs` — given public session when named operator connected then one instance owns both inputs (`tests/operator_declaration.rs:26`; `test-9e128271c40c45bb1c0b`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` — given derived stream chain when compiled then operator output feeds next named input (`src/session/compile/tests.rs:587`; `test-e9a24a392741b4dbe6e7`).
- `given_public_session_when_composed_then_three_stages_and_named_ports_run_under_one_owner` — given public session when composed then three stages and named ports run under one owner (`src/session/lifecycle/tests/running.rs:1755`; `test-42bab06a6020bf545d3a`).
- `given_connect_with_contract_when_into_spec_then_edge_carries_requested_contract` — given connect with contract when into spec then edge carries requested contract (`src/graph/dsl.rs:126`; `test-4a39ba3d3efea5bb8559`).
- `given_connected_nodes_when_into_spec_then_edge_records_endpoints` — given connected nodes when into spec then edge records endpoints (`src/graph/dsl.rs:111`; `test-6c7c927742d9d04f37dd`).
- `given_exact_named_inputs_when_compiled_then_one_multi_port_node_is_preserved` — given exact named inputs when compiled then one multi port node is preserved (`src/graph/named_ports.rs:67`; `test-898cd2ae0c6b04c280a5`).
- `given_unknown_named_input_when_compiled_then_failure_precedes_runtime` — given unknown named input when compiled then failure precedes runtime (`src/graph/named_ports.rs:87`; `test-547f292fc53c973e1c33`).

## Failure signals

- `pocketstation::session::compile::error::SessionCompileError` / `DuplicateOperatorInputConnection` — `error-967401df89c3468e7bd0`
- `pocketstation::session::compile::error::SessionCompileError` / `MissingRequiredOperatorInput` — `error-88aaa28af18ebdcb8225`
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownOperatorPort` — `error-07fdf39d6cdd4b147cc6`
- `pocketstation::graph::compile::resolve::CompileError` / `WrongPortDirection` — `error-8c5c6abb3ef2886f50f1`
- `pocketstation::graph::node::NodeDescriptorError` / `DuplicatePort` — `error-49f1427276e1627a6bff`
- `pocketstation::graph::ports::PortSpecError` — `error-5601bc96e0f09d517ffa`
- `pocketstation::graph::ports::PortSpecError` / `EmptyName` — `error-de68a6f4314abffa41f2`
- `pocketstation::graph::ports::PortSpecError` / `InvalidSignal` — `error-144a7c7033b72fb3ebe8`
- `pocketstation::graph::ports::PortSpecError` / `SignalMediaMismatch` — `error-41dfb8544f872cc47db6`
- `pocketstation::graph::registry::NodeRegistrationError` / `DuplicateOperatorId` — `error-237a7fc9fdd749cb3d97`

## API reference

- [Graph Contracts](/docs/concepts/graph-contracts.md)
- [Graph And Signals](/docs/errors/graph-and-signals.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::declaration::spec::ConnectionTarget::OperatorInput` | variant | Represents the operator input alternative defined by `ConnectionTarget`. | `src/session/declaration/spec.rs:225` |
| `ConnectionTarget::OperatorInput::input_port` | struct_field | Stores the input port used by `OperatorInput`. | `src/session/declaration/spec.rs:227` |
| `ConnectionTarget::OperatorInput::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `OperatorInput`. | `src/session/declaration/spec.rs:226` |
| `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| `pocketstation::graph::signal::operator::AsyncOperatorFactory` | trait | Implement this trait to provide async operator behavior to PocketStation; its methods define the preparation and runtime contract. | `src/graph/signal/operator.rs:368` |
| `pocketstation::graph::ports::AudioCaps` | struct | Declares the sample formats, channel layouts, and rates accepted by an audio port. | `src/graph/ports.rs:48` |
| `pocketstation::graph::ports::EdgeContract` | struct | Declares the validated constraints applied to edge. | `src/graph/ports.rs:311` |
| `pocketstation::graph::ports::PortSpec` | struct | Configures port behavior at its owning API boundary. | `src/graph/ports.rs:175` |

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

The claims on **Connect named operator ports** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/declaration/tests/operator_connections.rs:1-152` (`DIRECT`)

For **Connect named operator ports**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

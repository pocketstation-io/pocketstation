# Connect named operator ports

<!-- claims: CLM-GUIDE-012-SCOPE-001,CLM-GUIDE-012-TEXT-001,CLM-GUIDE-012-TEXT-002,CLM-GUIDE-012-TEXT-003,CLM-GUIDE-012-TEXT-004,CLM-GUIDE-012-TEXT-005,CLM-GUIDE-012-TEXT-006,CLM-GUIDE-012-SOURCE-001 -->

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
3. Use exact port names from the manifest and preserve each source-aware binding when several stems feed one operator.
4. Compile and inspect SessionCompileDiagnostic for the stage, stable code, and affected component identities.
5. Confirm every compiled binding targets the intended instance.

## Concrete repository example

The executable repository test `given_declared_operator_when_named_ports_connected_then_one_instance_owns_all_connections` (`test-d74d1c0449808ea58c4f`) shows the concrete API sequence and asserted outcome at `src/session/declaration/tests/operator_connections.rs:19`.

```rust
}

#[test]
fn given_declared_operator_when_named_ports_connected_then_one_instance_owns_all_connections() {
    let session = Session::new();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "Meeting App",
        )))
        .expect("application");
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone");
    let declared = session
        .operator(operator("example.operator.named-composition.v1"))
        .expect("one operator instance");

    application
        .connect(declared.input("application").expect("application input"))
        .expect("application connection");
    microphone
        .connect(declared.input("microphone").expect("microphone input"))
        .expect("microphone connection");
    declared
        .output("primary")
        .expect("primary output")
        .send(endpoint(&session, "primary"))
        .expect("primary route");
    declared
        .output("diagnostics")
        .expect("diagnostics output")
        .send(endpoint(&session, "diagnostics"))
        .expect("diagnostics route");

    let spec = session.freeze().expect("named Session specification");
    assert_eq!(spec.operators().len(), 1);
    let operator_inputs = spec
        .connections()
        .iter()
        .filter(|connection| matches!(connection.target(), ConnectionTarget::OperatorInput { .. }))
        .collect::<Vec<_>>();
    let endpoint_inputs = spec
        .connections()
        .iter()
        .filter(|connection| matches!(connection.target(), ConnectionTarget::EndpointInput { .. }))
        .collect::<Vec<_>>();
    assert_eq!(operator_inputs.len(), 2);
    assert_eq!(endpoint_inputs.len(), 2);
    assert_eq!(
        operator_inputs
            .iter()
            .map(|connection| match connection.target() {
                ConnectionTarget::OperatorInput {
                    input_port: Some(input_port),
                    ..
                } => input_port.as_str(),
                _ => panic!("named operator input"),
            })
            .collect::<Vec<_>>(),
        ["application", "microphone"]
    );
    assert!(matches!(operator_inputs[0].origin(), StreamOrigin::Stem(_)));
}
```

```bash
cargo test --all-features given_declared_operator_when_named_ports_connected_then_one_instance_owns_all_connections
```

## Important consequence

Unknown, duplicate, foreign, or incompatible ports fail before runtime preparation.

## Verify the outcome

Compilation binds each named handle to the intended operator instance and compatible port.

Executable evidence selected for **Connect named operator ports** is limited to each test's recorded setup and assertions:

- `given_declared_operator_when_named_ports_connected_then_one_instance_owns_all_connections` — given declared operator when named ports connected then one instance owns all connections (`src/session/declaration/tests/operator_connections.rs:19`; `test-d74d1c0449808ea58c4f`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` — given foreign input handle when connected then declaration fails before freeze (`src/session/declaration/tests/operator_connections.rs:154`; `test-0098e9bf5859cd4840f9`).
- `given_repeated_named_input_when_declared_then_compiler_retains_multiplicity_authority` — given repeated named input when declared then compiler retains multiplicity authority (`src/session/declaration/tests/operator_connections.rs:110`; `test-0920f0be863672d2298e`).
- `given_through_sugar_when_frozen_then_canonical_instance_and_connection_records_are_used` — given through sugar when frozen then canonical instance and connection records are used (`src/session/declaration/tests/operator_connections.rs:81`; `test-f9ee710abe61af2dce08`).
- `given_one_named_operator_instance_when_compiled_then_all_named_connections_share_one_node` — given one named operator instance when compiled then all named connections share one node (`src/session/compile/tests.rs:308`; `test-04616ce7442a986e3b43`).
- `given_public_session_when_named_operator_connected_then_one_instance_owns_both_inputs` — given public session when named operator connected then one instance owns both inputs (`tests/operator_declaration.rs:26`; `test-2076461784bfa8508fc9`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` — given derived stream chain when compiled then operator output feeds next named input (`src/session/compile/tests.rs:587`; `test-081f9254eabd3bfeaad1`).
- `given_public_session_when_composed_then_three_stages_and_named_ports_run_under_one_owner` — given public session when composed then three stages and named ports run under one owner (`src/session/lifecycle/tests/running.rs:1758`; `test-b269c7b61711642b5e6e`).
- `given_connect_with_contract_when_into_spec_then_edge_carries_requested_contract` — given connect with contract when into spec then edge carries requested contract (`src/graph/dsl.rs:126`; `test-465bd31d37efb7892d1b`).
- `given_connected_nodes_when_into_spec_then_edge_records_endpoints` — given connected nodes when into spec then edge records endpoints (`src/graph/dsl.rs:111`; `test-5634782bf1cbb4bdc0c9`).
- `given_exact_named_inputs_when_compiled_then_one_multi_port_node_is_preserved` — given exact named inputs when compiled then one multi port node is preserved (`src/graph/named_ports.rs:67`; `test-20fc88a0719f6a9da67e`).
- `given_unknown_named_input_when_compiled_then_failure_precedes_runtime` — given unknown named input when compiled then failure precedes runtime (`src/graph/named_ports.rs:87`; `test-1a4df0780a30d247ebc4`).

## Failure signals

- `pocketstation::session::compile::error::SessionCompileError` / `DuplicateOperatorInputConnection` — `error-8d82b1ede97797f60e26`
- `pocketstation::session::compile::error::SessionCompileError` / `MissingRequiredOperatorInput` — `error-5903fce05dd27adde84a`
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownOperatorPort` — `error-5934f31fda2b7f05b9ac`
- `pocketstation::graph::compile::resolve::CompileError` / `WrongPortDirection` — `error-921f3a52482730a6cadc`
- `pocketstation::graph::node::NodeDescriptorError` / `DuplicatePort` — `error-61a4806d55aa5a25fa8f`
- `pocketstation::graph::ports::PortSpecError` — `error-632ca0eab915b16bffbe`
- `pocketstation::graph::ports::PortSpecError` / `EmptyName` — `error-365361ddef8f066cfbd9`
- `pocketstation::graph::ports::PortSpecError` / `InvalidSignal` — `error-8baac8353ed3d47bf0b5`
- `pocketstation::graph::ports::PortSpecError` / `SignalMediaMismatch` — `error-a3257596a6ac9f317574`
- `pocketstation::graph::registry::NodeRegistrationError` / `DuplicateOperatorId` — `error-39c40c390407fd042e82`

## API reference

- [Graph Contracts](/docs/concepts/graph-contracts.md)
- [Graph And Signals](/docs/errors/graph-and-signals.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::declaration::spec::ConnectionTarget::OperatorInput` | variant | Represents the operator input alternative defined by `ConnectionTarget`. | `src/session/declaration/spec.rs:225` |
| `ConnectionTarget::OperatorInput::input_port` | struct_field | References the input port participating in `OperatorInput`. | `src/session/declaration/spec.rs:227` |
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

The claims on **Connect named operator ports** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/declaration/tests/operator_connections.rs:154-172` (`TESTED`)
- `src/session/declaration/tests/operator_connections.rs:110-154` (`TESTED`)
- `src/session/declaration/tests/operator_connections.rs:19-81` (`TESTED`)
- `src/session/declaration/tests/operator_connections.rs:81-110` (`TESTED`)

For **Connect named operator ports**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

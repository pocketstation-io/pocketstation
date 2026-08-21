use crate::graph::NodeTypeId;
use crate::session::{
    ApplicationSelector, ConnectionTarget, EndpointDescriptor, Operator, OperatorConfiguration,
    OperatorId, Session, SessionError, Source, StreamOrigin,
};

fn operator(id: &str) -> Operator {
    Operator::new(OperatorId::new(id), OperatorConfiguration::new())
}

fn endpoint(session: &Session, suffix: &str) -> crate::session::EndpointHandle {
    let node_type_id = format!("endpoint.named.{suffix}");
    session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(node_type_id.as_str()),
            OperatorId::new(format!("example.endpoint.named.{suffix}.v1")),
        ))
        .expect("endpoint declaration")
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

#[test]
fn given_through_sugar_when_frozen_then_canonical_instance_and_connection_records_are_used() {
    let session = Session::new();
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone");
    let derived = microphone
        .through(operator("example.operator.sugar.v1"))
        .expect("through sugar");
    derived
        .send(endpoint(&session, "sugar"))
        .expect("derived route");

    let spec = session.freeze().expect("canonical specification");
    assert_eq!(spec.operators().len(), 1);
    let connection = spec
        .connections()
        .iter()
        .find(|connection| matches!(connection.target(), ConnectionTarget::OperatorInput { .. }))
        .expect("operator input connection");
    assert!(matches!(
        connection.target(),
        ConnectionTarget::OperatorInput {
            operator_instance_id,
            ..
        } if *operator_instance_id == spec.operators()[0].instance_id()
    ));
}

#[test]
fn given_repeated_named_input_when_declared_then_compiler_retains_multiplicity_authority() {
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
        .operator(operator("example.operator.duplicate-input.v1"))
        .expect("operator");
    application
        .connect(declared.input("audio").expect("input"))
        .expect("first connection");

    microphone
        .connect(declared.input("audio").expect("same input"))
        .expect("second declaration");
    declared
        .output("transcript")
        .expect("operator output")
        .send(endpoint(&session, "fan-in"))
        .expect("operator destination");

    let spec = session.freeze().expect("uncompiled specification");
    let connections = spec
        .connections()
        .iter()
        .filter(|connection| {
            matches!(
                connection.target(),
                ConnectionTarget::OperatorInput {
                    operator_instance_id,
                    input_port: Some(input_port),
                } if *operator_instance_id == declared.instance_id() && input_port == "audio"
            )
        })
        .count();
    assert_eq!(connections, 2);
}

#[test]
fn given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze() {
    let first = Session::new();
    let second = Session::new();
    let microphone = first
        .capture(Source::microphone_default())
        .expect("microphone");
    let foreign = second
        .operator(operator("example.operator.foreign.v1"))
        .expect("foreign operator")
        .input("audio")
        .expect("foreign input");

    let result = microphone.connect(foreign);

    assert!(matches!(result, Err(SessionError::InvalidOperator { .. })));
}

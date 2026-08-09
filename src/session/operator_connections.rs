use crate::graph::NodeTypeId;
use crate::session::{
    ApplicationSelector, EndpointDescriptor, Operator, OperatorConfiguration, OperatorId,
    OperatorInputOrigin, Session, SessionError, Source,
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
fn given_one_declared_instance_when_named_ports_connected_then_one_instance_owns_all_connections() {
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
    assert_eq!(spec.operator_connections().len(), 2);
    assert_eq!(spec.derived_routes().len(), 2);
    assert_eq!(
        spec.operator_connections()
            .iter()
            .map(|connection| connection.input_port().expect("named input"))
            .collect::<Vec<_>>(),
        ["application", "microphone"]
    );
    assert!(matches!(
        spec.operator_connections()[0].input_origin(),
        OperatorInputOrigin::Stem(_)
    ));
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
    assert_eq!(spec.operator_connections().len(), 1);
    assert_eq!(
        spec.operator_connections()[0].operator_instance_id(),
        spec.operators()[0].instance_id()
    );
}

#[test]
fn given_duplicate_named_input_when_connected_then_declaration_fails_immediately() {
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

    let result = microphone.connect(declared.input("audio").expect("same input"));

    assert!(matches!(result, Err(SessionError::InvalidOperator { .. })));
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

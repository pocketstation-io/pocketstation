use pocketstation::{
    ApplicationSelector, EndpointDescriptor, NodeTypeId, Operator, OperatorConfiguration,
    OperatorId, Session, Source,
};

#[test]
fn given_public_facade_when_typed_delivery_declared_then_internal_graph_is_not_required() {
    let session = Session::new();
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone declaration");
    let terminal = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from("endpoint.text.example"),
            OperatorId::new("example.endpoint.text.v1"),
        ))
        .expect("terminal declaration");
    let transcript = microphone
        .through(Operator::new(
            OperatorId::new("example.operator.stt.v1"),
            OperatorConfiguration::new().with("language", "auto"),
        ))
        .expect("operator declaration");

    transcript.send(terminal).expect("derived terminal route");
}

#[test]
fn given_public_session_when_named_operator_connected_then_one_instance_owns_both_inputs() {
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
        .operator(Operator::new(
            OperatorId::new("example.operator.public-named.v1"),
            OperatorConfiguration::new(),
        ))
        .expect("one operator instance");
    let application_route = application
        .connect(declared.input("application").expect("application input"))
        .expect("application connection");
    let microphone_route = microphone
        .connect(declared.input("microphone").expect("microphone input"))
        .expect("microphone connection");
    let terminal = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from("endpoint.public.named"),
            OperatorId::new("example.endpoint.public-named.v1"),
        ))
        .expect("terminal");
    declared
        .output("result")
        .expect("result output")
        .send(terminal)
        .expect("result route");

    assert_eq!(declared.session_id(), session.id());
    assert_ne!(application_route, microphone_route);
}

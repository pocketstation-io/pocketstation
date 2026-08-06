use pocketstation::{
    EndpointDescriptor, NodeTypeId, Operator, OperatorConfiguration, OperatorId, Session, Source,
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

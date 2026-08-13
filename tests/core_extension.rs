use pocketstation::{
    ApplicationSelector, AsyncOperatorManifest, CopyPolicy, EdgeContract, EndpointConfiguration,
    EndpointDescriptor, ExecutionPartition, MediaCaps, Multiplicity, NodeDescriptor, NodeTypeId,
    Operator, OperatorCancellationPolicy, OperatorConfiguration, OperatorDeadlinePolicy,
    OperatorFailurePolicy, OperatorId, OperatorOutputRolePolicy, OperatorPermissionPolicy,
    PortDirection, PortSpec, SafetyContract, SignalSpec, Source, Stream, StreamSignal,
    TypedOperator,
};

struct CapturedSignal;
struct DerivedSignal;

impl StreamSignal for CapturedSignal {
    fn signal_spec() -> SignalSpec {
        SignalSpec::audio()
    }
}

impl StreamSignal for DerivedSignal {
    fn signal_spec() -> SignalSpec {
        SignalSpec::control().with_schema("urn:pocketstation:test:derived:v1")
    }
}

fn manifest(id: &str) -> AsyncOperatorManifest {
    let input_edge = EdgeContract::realtime_audio().with_copy_policy(CopyPolicy::CopyToBranchPool);
    let output_edge = EdgeContract::bounded_async().with_media(MediaCaps::Control);
    let node = NodeDescriptor::new(
        NodeTypeId::from("dev.pocketstation.test.typed-node.v1"),
        "typed extension conformance",
        vec![PortSpec::new(
            "in",
            PortDirection::Input,
            CapturedSignal::signal_spec(),
            input_edge.media(),
            Multiplicity::One,
            true,
        )
        .expect("input port")],
        vec![PortSpec::new(
            "out",
            PortDirection::Output,
            DerivedSignal::signal_spec(),
            MediaCaps::Control,
            Multiplicity::Many,
            true,
        )
        .expect("output port")],
        ExecutionPartition::AsyncWorker,
        SafetyContract::AllocationAllowed,
        false,
    )
    .expect("node descriptor");
    AsyncOperatorManifest::new(
        OperatorId::new(id),
        1,
        1,
        node,
        input_edge,
        output_edge,
        2,
        OperatorPermissionPolicy {
            network_allowed: false,
            filesystem_allowed: false,
        },
        OperatorDeadlinePolicy {
            process_timeout_ms: 100,
        },
        OperatorCancellationPolicy::DiscardQueued,
        OperatorFailurePolicy::StopWorker,
        OperatorOutputRolePolicy::default(),
    )
    .expect("operator manifest")
}

#[test]
fn given_core_extension_typed_stream_when_composed_then_signal_spec_is_runtime_authority() {
    let operator_id = "dev.pocketstation.test.typed-operator.v1";
    let typed = TypedOperator::<CapturedSignal, DerivedSignal>::new(
        Operator::new(OperatorId::new(operator_id), OperatorConfiguration::new()),
        &manifest(operator_id),
        None,
        None,
    )
    .expect("typed operator");
    let session = pocketstation::Session::new();
    let stem = session
        .capture(Source::application(ApplicationSelector::name("fixture")))
        .expect("stem");
    let endpoint = session
        .endpoint(
            EndpointDescriptor::new(
                NodeTypeId::from("dev.pocketstation.test.endpoint.v1"),
                OperatorId::new("dev.pocketstation.test.endpoint.v1"),
            )
            .with_configuration(EndpointConfiguration::new()),
        )
        .expect("endpoint");
    let output = Stream::<CapturedSignal>::from_stem(stem)
        .expect("typed capture stream")
        .through(typed)
        .expect("typed composition");
    assert_eq!(output.signal_spec(), &DerivedSignal::signal_spec());
    output.send(endpoint).expect("typed route");
}

#[test]
fn given_core_extension_wire_signals_when_read_then_ids_are_language_neutral() {
    assert_eq!(SignalSpec::audio().wire_id(), "pks.signal.pcm-audio.v1");
    assert_eq!(
        DerivedSignal::signal_spec().wire_id(),
        "pks.signal.control.v1"
    );
    assert_eq!(
        SignalSpec::custom("dev.pocketstation.external.v1").wire_id(),
        "dev.pocketstation.external.v1"
    );
}

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
    let mut input_edge = EdgeContract::voice_default();
    input_edge.copy_policy = CopyPolicy::CopyToBranchPool;
    let mut output_edge = EdgeContract::typed_default();
    output_edge.media = MediaCaps::Control;
    AsyncOperatorManifest {
        operator_id: OperatorId::new(id),
        revision: 1,
        generation: 1,
        node: NodeDescriptor {
            type_id: NodeTypeId::from("dev.pocketstation.test.typed-node.v1"),
            display_name: "typed extension conformance",
            inputs: vec![PortSpec {
                name: "in".to_owned(),
                direction: PortDirection::Input,
                signal: CapturedSignal::signal_spec(),
                media: input_edge.media,
                multiplicity: Multiplicity::One,
                required: true,
            }],
            outputs: vec![PortSpec {
                name: "out".to_owned(),
                direction: PortDirection::Output,
                signal: DerivedSignal::signal_spec(),
                media: MediaCaps::Control,
                multiplicity: Multiplicity::Many,
                required: true,
            }],
            execution: ExecutionPartition::AsyncWorker,
            safety: SafetyContract::AllocationAllowed,
            stateful: false,
        },
        input_edge,
        output_edge,
        queue_capacity_frames: 2,
        permission: OperatorPermissionPolicy {
            network_allowed: false,
            filesystem_allowed: false,
        },
        deadline: OperatorDeadlinePolicy {
            process_timeout_ms: 100,
        },
        cancellation: OperatorCancellationPolicy::DiscardQueued,
        failure: OperatorFailurePolicy::StopWorker,
        output_roles: OperatorOutputRolePolicy::default(),
    }
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

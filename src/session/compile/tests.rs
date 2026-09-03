use std::sync::Arc;

use crate::endpoint::{
    EndpointDriverFactory, EndpointDriverRegistry, EndpointFailure, EndpointFailureStage,
    EndpointPortInput, PreparedEndpointDriver,
};
use crate::frame::{SampleFormat, SampleSpec};
use crate::graph::compile::CompileError;
use crate::graph::{
    AsyncNode, AsyncOperatorFactory, AsyncOperatorManifest, AudioCaps, BackpressurePolicy,
    ChannelLayout, ConfigError, CopyPolicy, ExecutionPartition, ExecutionSafety, LossPolicy,
    MediaCaps, Multiplicity, NodeDefinition, NodeDescriptor, NodeError, OperatorCancellationPolicy,
    OperatorDeadlinePolicy, OperatorFailurePolicy, OperatorOutputRolePolicy,
    OperatorPermissionPolicy, PortDirection, PortSpec, RouteObservability, SemanticRole,
    SignalSpec, TextFormat,
};

use super::*;
use crate::session::extensions::builtins::APPLICATION_SOURCE_NODE_TYPE_ID;
use crate::session::{
    register_session_graph_nodes, ApplicationSelector, EndpointConfiguration, EndpointDescriptor,
    Operator, OperatorConfiguration, OperatorId, Session, Source, BROWSER_NODE_TYPE_ID,
    BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID, MICROPHONE_SOURCE_NODE_TYPE_ID,
    RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
};

const TEST_CONNECTOR_OPERATOR_ID: &str = "example.connector.streaming-stt.v1";
const TEST_ASYNC_OPERATOR_ID: &str = "example.operator.streaming-stt.v1";
const TEST_ASYNC_NODE_TYPE_ID: &str = "operator.transcription.test";
const TEST_TRANSFORM_OPERATOR_ID: &str = "example.operator.text-transform.v1";
const TEST_TRANSFORM_NODE_TYPE_ID: &str = "operator.text-transform.test";
const TEST_NAMED_OPERATOR_ID: &str = "example.operator.named-composition.v1";
const TEST_NAMED_NODE_TYPE_ID: &str = "operator.named-composition.test";
const TEST_TEXT_ENDPOINT_OPERATOR_ID: &str = "example.endpoint.text.v1";
const TEST_TEXT_ENDPOINT_NODE_TYPE_ID: &str = "endpoint.text.test";
const TEST_NONTERMINAL_ROLE: &str = "test.output.nonterminal";
const TEST_TERMINAL_ROLE: &str = "test.output.terminal";

struct CompileOnlyEndpointFactory;

struct CompileOnlyAsyncFactory {
    manifest: AsyncOperatorManifest,
}

impl CompileOnlyAsyncFactory {
    fn new() -> Self {
        let mut input_route_settings = RouteSettings::realtime_audio();
        input_route_settings.copy_policy = crate::graph::CopyPolicy::CopyToBranchPool;
        let mut output_route_settings = RouteSettings::bounded_async();
        output_route_settings.media = MediaCaps::Text;
        Self {
            manifest: AsyncOperatorManifest {
                operator_id: OperatorId::new(TEST_ASYNC_OPERATOR_ID),
                revision: 1,
                generation: 1,
                node: NodeDescriptor {
                    type_id: NodeTypeId::from(TEST_ASYNC_NODE_TYPE_ID),
                    display_name: "Compile-only transcription operator",
                    inputs: vec![PortSpec {
                        name: "audio".to_owned(),
                        direction: PortDirection::Input,
                        signal: SignalSpec::audio(),
                        media: MediaCaps::Audio(AudioCaps {
                            sample_rate_hz: Some(48_000),
                            frame_samples: None,
                            channel_layout: ChannelLayout::Any,
                            format: SampleFormat::F32Interleaved,
                        }),
                        multiplicity: Multiplicity::One,
                        required: true,
                    }],
                    outputs: vec![PortSpec {
                        name: "transcript".to_owned(),
                        direction: PortDirection::Output,
                        signal: SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                        media: MediaCaps::Text,
                        multiplicity: Multiplicity::One,
                        required: true,
                    }],
                    execution: ExecutionPartition::AsyncWorker,
                    safety: ExecutionSafety::ExternalService,
                    stateful: true,
                },
                input_route_settings,
                output_route_settings,
                queue_capacity_frames: 32,
                permission: OperatorPermissionPolicy {
                    network_allowed: true,
                    filesystem_allowed: false,
                },
                deadline: OperatorDeadlinePolicy {
                    process_timeout_ms: 1_000,
                },
                cancellation: OperatorCancellationPolicy::DiscardQueued,
                failure: OperatorFailurePolicy::StopWorker,
                output_roles: OperatorOutputRolePolicy {
                    allowed: vec![
                        SemanticRole::new(TEST_NONTERMINAL_ROLE),
                        SemanticRole::new(TEST_TERMINAL_ROLE),
                    ],
                    terminal: vec![SemanticRole::new(TEST_TERMINAL_ROLE)],
                },
            },
        }
    }

    fn text_transform() -> Self {
        let mut factory = Self::new();
        factory.manifest.operator_id = OperatorId::new(TEST_TRANSFORM_OPERATOR_ID);
        factory.manifest.node.type_id = NodeTypeId::from(TEST_TRANSFORM_NODE_TYPE_ID);
        factory.manifest.node.inputs = vec![PortSpec {
            name: "text".to_owned(),
            direction: PortDirection::Input,
            signal: SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
            media: MediaCaps::Text,
            multiplicity: Multiplicity::One,
            required: true,
        }];
        factory.manifest.input_route_settings.media = MediaCaps::Text;
        factory
    }

    fn named_composition() -> Self {
        let mut factory = Self::new();
        factory.manifest.operator_id = OperatorId::new(TEST_NAMED_OPERATOR_ID);
        factory.manifest.node.type_id = NodeTypeId::from(TEST_NAMED_NODE_TYPE_ID);
        let audio = |name: &str| PortSpec {
            name: name.to_owned(),
            direction: PortDirection::Input,
            signal: SignalSpec::audio(),
            media: MediaCaps::Audio(AudioCaps {
                sample_rate_hz: Some(48_000),
                frame_samples: None,
                channel_layout: ChannelLayout::Any,
                format: SampleFormat::F32Interleaved,
            }),
            multiplicity: Multiplicity::One,
            required: true,
        };
        let text = |name: &str| PortSpec {
            name: name.to_owned(),
            direction: PortDirection::Output,
            signal: SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
            media: MediaCaps::Text,
            multiplicity: Multiplicity::One,
            required: true,
        };
        factory.manifest.node.inputs = vec![audio("application"), audio("microphone")];
        factory.manifest.node.outputs = vec![text("primary"), text("diagnostics")];
        factory
    }
}

impl AsyncOperatorFactory for CompileOnlyAsyncFactory {
    fn manifest(&self) -> &AsyncOperatorManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(&self, _configuration: &NodeConfig) -> Result<Box<dyn AsyncNode>, NodeError> {
        Err(NodeError::Prepare(
            "compile-only async factory must not create".to_owned(),
        ))
    }
}

struct CompileOnlyTextEndpointDefinition;

impl NodeDefinition for CompileOnlyTextEndpointDefinition {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            display_name: "Compile-only text terminal",
            inputs: vec![PortSpec {
                name: "transcript".to_owned(),
                direction: PortDirection::Input,
                signal: SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                media: MediaCaps::Text,
                multiplicity: Multiplicity::One,
                required: true,
            }],
            outputs: Vec::new(),
            execution: ExecutionPartition::External,
            safety: ExecutionSafety::ExternalService,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
}

impl EndpointDriverFactory for CompileOnlyEndpointFactory {
    fn prepare(
        &self,
        _inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        Err(EndpointFailure::new(
            EndpointFailureStage::Prepare,
            "compile-only endpoint factory must not prepare",
        ))
    }
}

fn endpoint_registry(connector_node_type_id: Option<&'static str>) -> EndpointDriverRegistry {
    let mut endpoint_registry = EndpointDriverRegistry::new();
    let factory: Arc<dyn EndpointDriverFactory> = Arc::new(CompileOnlyEndpointFactory);
    let registrations = [
        connector_node_type_id.map(|node_type_id| {
            (
                TEST_CONNECTOR_OPERATOR_ID,
                node_type_id,
                Arc::clone(&factory),
            )
        }),
        Some((
            BROWSER_OPERATOR_ID,
            BROWSER_NODE_TYPE_ID,
            Arc::clone(&factory),
        )),
        Some((
            RECORDER_OPERATOR_ID,
            RECORDER_NODE_TYPE_ID,
            Arc::clone(&factory),
        )),
    ];
    for registration in registrations.into_iter().flatten() {
        endpoint_registry
            .register(
                OperatorId::new(registration.0),
                NodeTypeId::from(registration.1),
                registration.2,
            )
            .expect("endpoint registration");
    }
    endpoint_registry
}

#[test]
fn given_endpoint_accepting_any_frame_size_when_edge_is_specialized_then_wildcard_is_preserved() {
    let input_route_settings = RouteSettings::realtime_audio();
    let endpoint_media = MediaCaps::Audio(AudioCaps {
        sample_rate_hz: Some(48_000),
        frame_samples: None,
        channel_layout: ChannelLayout::Any,
        format: SampleFormat::F32Interleaved,
    });

    let specialized = specialize_edge_media(input_route_settings, endpoint_media);

    assert_eq!(specialized.media, endpoint_media);
    assert!(specialized
        .media
        .is_compatible_with(&MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(48_000),
            frame_samples: Some(480),
            channel_layout: ChannelLayout::Stereo,
            format: SampleFormat::F32Interleaved,
        },)));
}

fn registries() -> (NodeRegistry, EndpointDriverRegistry) {
    let mut node_registry = NodeRegistry::new();
    register_session_graph_nodes(&mut node_registry).expect("Session structural registrations");
    for node_type_id in [
        CONNECTOR_NODE_TYPE_ID,
        BROWSER_NODE_TYPE_ID,
        RECORDER_NODE_TYPE_ID,
    ] {
        node_registry
            .register_definition(
                crate::session::extensions::builtins::audio_endpoint_boundary_definition(
                    NodeTypeId::from(node_type_id),
                    SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
                    crate::frame::AudioFrameDuration::default().samples_per_channel(48_000),
                ),
            )
            .expect("audio endpoint definition");
    }

    (
        node_registry,
        endpoint_registry(Some(CONNECTOR_NODE_TYPE_ID)),
    )
}

fn derived_registries() -> (NodeRegistry, EndpointDriverRegistry) {
    let (mut node_registry, mut endpoint_registry) = registries();
    node_registry
        .register_async(Arc::new(CompileOnlyAsyncFactory::new()))
        .expect("async operator registration");
    node_registry
        .register_async(Arc::new(CompileOnlyAsyncFactory::text_transform()))
        .expect("text transform registration");
    node_registry
        .register_async(Arc::new(CompileOnlyAsyncFactory::named_composition()))
        .expect("named composition registration");
    node_registry
        .register_definition(Arc::new(CompileOnlyTextEndpointDefinition))
        .expect("text endpoint node registration");
    endpoint_registry
        .register(
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            Arc::new(CompileOnlyEndpointFactory),
        )
        .expect("text endpoint driver registration");
    (node_registry, endpoint_registry)
}

fn derived_spec() -> SessionSpec {
    let session = Session::new();
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone declaration");
    let terminal = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
        ))
        .expect("text terminal declaration");
    let transcript = microphone
        .through(Operator::new(
            OperatorId::new(TEST_ASYNC_OPERATOR_ID),
            OperatorConfiguration::new().with("language", "auto"),
        ))
        .expect("operator declaration");
    transcript.send(terminal).expect("derived terminal route");
    session.freeze().expect("derived spec")
}

#[test]
fn given_one_named_operator_instance_when_compiled_then_all_named_connections_share_one_node() {
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
            OperatorId::new(TEST_NAMED_OPERATOR_ID),
            OperatorConfiguration::new(),
        ))
        .expect("operator instance");
    application
        .connect(declared.input("application").expect("application input"))
        .expect("application connection");
    microphone
        .connect(declared.input("microphone").expect("microphone input"))
        .expect("microphone connection");
    let terminal = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
        ))
        .expect("terminal");
    declared
        .output("primary")
        .expect("primary output")
        .send(terminal)
        .expect("primary route");
    declared
        .output("diagnostics")
        .expect("diagnostics output")
        .send(terminal)
        .expect("diagnostics route");
    let spec = session.freeze().expect("named spec");
    let (nodes, endpoints) = derived_registries();

    let compiled = SessionCompiler::new(&nodes, &endpoints)
        .compile(spec)
        .expect("named compile");

    let operator_nodes = compiled
        .graph_ir
        .nodes
        .iter()
        .filter(|node| node.spec.type_id.as_str() == TEST_NAMED_NODE_TYPE_ID)
        .collect::<Vec<_>>();
    assert_eq!(operator_nodes.len(), 1);
    let node_id = operator_nodes[0].id();
    assert_eq!(
        compiled
            .graph_ir
            .edges
            .iter()
            .filter(|edge| edge.spec.to.node == node_id)
            .map(|edge| edge.spec.to.port.as_str())
            .collect::<Vec<_>>(),
        ["application", "microphone"]
    );
    assert_eq!(
        compiled
            .graph_ir
            .edges
            .iter()
            .filter(|edge| edge.spec.from.node == node_id)
            .map(|edge| edge.spec.from.port.as_str())
            .collect::<Vec<_>>(),
        ["primary", "diagnostics"]
    );
}

#[test]
fn given_required_named_input_missing_when_compiled_then_failure_precedes_graph_runtime() {
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
            OperatorId::new(TEST_NAMED_OPERATOR_ID),
            OperatorConfiguration::new(),
        ))
        .expect("operator instance");
    application
        .connect(declared.input("application").expect("application input"))
        .expect("application connection");
    let terminal = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
        ))
        .expect("terminal");
    declared
        .output("primary")
        .expect("primary output")
        .send(terminal)
        .expect("primary route");
    microphone
        .send(session.browser("wss://receiver.test").expect("browser"))
        .expect("microphone destination");
    let spec = session.freeze().expect("structurally valid spec");
    let (nodes, endpoints) = derived_registries();

    let result = SessionCompiler::new(&nodes, &endpoints).compile(spec);

    assert!(matches!(
        result,
        Err(SessionCompileError::MissingRequiredOperatorInput { port_name, .. })
            if port_name == "microphone"
    ));
}

fn two_destination_derived_spec() -> SessionSpec {
    let session = Session::new();
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone declaration");
    let first = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
        ))
        .expect("first text terminal declaration");
    let second = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
        ))
        .expect("second text terminal declaration");
    let transcript = microphone
        .through(Operator::new(
            OperatorId::new(TEST_ASYNC_OPERATOR_ID),
            OperatorConfiguration::new(),
        ))
        .expect("operator declaration");
    transcript.send(first).expect("first derived route");
    transcript.send(second).expect("second derived route");
    session.freeze().expect("two-destination derived spec")
}

fn product_spec() -> SessionSpec {
    let session = Session::new();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "Meeting App",
        )))
        .expect("application declaration");
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone declaration");
    let connector = session
        .connector(
            OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
            EndpointConfiguration::new().with("model", "test-only"),
        )
        .expect("connector declaration");
    let browser = session
        .browser("wss://receiver.example.test")
        .expect("browser declaration");

    application
        .send(connector)
        .expect("application connector route");
    application
        .send(browser)
        .expect("application browser route");
    application
        .record("application")
        .expect("application recorder route");
    microphone
        .send(connector)
        .expect("microphone connector route");
    microphone.send(browser).expect("microphone browser route");
    microphone
        .record("microphone")
        .expect("microphone recorder route");
    session.freeze().expect("product spec")
}

#[test]
fn given_product_spec_when_compiled_then_six_independent_edges_are_planned() {
    let spec = product_spec();
    let (node_registry, endpoint_registry) = registries();

    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(spec)
        .expect("compiled product Session");

    assert_eq!(compiled.source_declarations().len(), 2);
    assert_eq!(compiled.endpoint_declarations().len(), 4);
    assert_eq!(compiled.node_count(), 8);
    assert_eq!(compiled.edge_count(), 6);
    assert_eq!(compiled.planned_edge_count(), 6);

    for edge in &compiled.graph_ir.edges {
        let source_type = compiled
            .graph_ir
            .node(edge.spec.from.node)
            .expect("source node")
            .spec
            .type_id
            .as_str();
        let planned = compiled
            .runtime_plan
            .memory_plan
            .edge_buffer(edge.spec.id)
            .expect("planned audio edge");
        match source_type {
            APPLICATION_SOURCE_NODE_TYPE_ID => assert_eq!(planned.bytes_per_frame, 7_680),
            MICROPHONE_SOURCE_NODE_TYPE_ID => assert_eq!(planned.bytes_per_frame, 3_840),
            _ => panic!("unexpected product source node '{source_type}'"),
        }
    }

    let recorder_edges = compiled
        .graph_ir
        .edges
        .iter()
        .filter(|edge| {
            compiled
                .graph_ir
                .node(edge.spec.to.node)
                .is_some_and(|node| node.spec.type_id.as_str() == RECORDER_NODE_TYPE_ID)
        })
        .collect::<Vec<_>>();
    assert_eq!(recorder_edges.len(), 2);
    for edge in recorder_edges {
        let route_settings = edge
            .spec
            .requested
            .expect("explicit recorder route_settings");
        assert_eq!(route_settings.jitter_budget_ms, Some(400));
        assert_eq!(route_settings.backpressure, BackpressurePolicy::DropNewest);
        assert_eq!(route_settings.loss, LossPolicy::DropAllowed);
        assert_eq!(route_settings.copy_policy, CopyPolicy::CopyToBranchPool);
        assert_eq!(route_settings.observability, RouteObservability::Full);
        assert_eq!(
            compiled
                .runtime_plan
                .memory_plan
                .edge_buffer(edge.spec.id)
                .expect("planned recorder buffer")
                .capacity_frames,
            20
        );
    }
}

#[test]
fn given_registered_operator_when_compiled_then_manifest_types_and_edge_policies_are_authority() {
    let spec = derived_spec();
    let (node_registry, endpoint_registry) = derived_registries();

    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(spec)
        .expect("compiled derived Session");

    assert_eq!(compiled.node_count(), 3);
    assert_eq!(compiled.edge_count(), 2);
    assert_eq!(compiled.planned_edge_count(), 2);
    assert_eq!(
        compiled.graph_ir.edges[0]
            .route_settings
            .expect("operator input route_settings")
            .backpressure,
        BackpressurePolicy::DropNewest
    );
    assert_eq!(compiled.graph_ir.edges[1].media, MediaCaps::Text);
    let output_contract = compiled.graph_ir.edges[1]
        .route_settings
        .expect("operator output route_settings");
    assert_eq!(output_contract.media, MediaCaps::Text);
    assert_eq!(
        output_contract.backpressure,
        BackpressurePolicy::BoundedQueue
    );
    let operator_node = compiled
        .graph_ir
        .nodes
        .iter()
        .find(|node| node.spec.type_id.as_str() == TEST_ASYNC_NODE_TYPE_ID)
        .expect("compiled operator node");
    assert_eq!(operator_node.spec.config.get("language"), Some("auto"));
    assert_eq!(operator_node.spec.config.get("operator_id"), None);
    assert!(matches!(
        compiled.bindings.node(operator_node.id()),
        Some(CompiledNodeBinding::Operator {
            operator_instance_id,
        }) if *operator_instance_id == compiled.spec.operators()[0].instance_id()
    ));
}

#[test]
fn given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input() {
    let session = Session::new();
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone");
    let terminal = session
        .endpoint(EndpointDescriptor::new(
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
        ))
        .expect("terminal");
    let first = microphone
        .through(Operator::new(
            OperatorId::new(TEST_ASYNC_OPERATOR_ID),
            OperatorConfiguration::new(),
        ))
        .expect("first");
    let second = first
        .through_ports(
            Operator::new(
                OperatorId::new(TEST_TRANSFORM_OPERATOR_ID),
                OperatorConfiguration::new(),
            ),
            Some("text".to_owned()),
            Some("transcript".to_owned()),
        )
        .expect("second");
    second.send(terminal).expect("route");
    let specification = session.freeze().expect("specification");
    let (node_registry, endpoint_registry) = derived_registries();

    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(specification)
        .expect("compiled chain");

    assert_eq!(compiled.node_count(), 4);
    assert_eq!(compiled.edge_count(), 3);
    assert_eq!(compiled.runtime_plan.typed_edges.len(), 2);
    assert!(compiled.graph_ir.edges.iter().any(|edge| {
        compiled
            .graph_ir
            .node(edge.spec.from.node)
            .is_some_and(|node| node.spec.type_id.as_str() == TEST_ASYNC_NODE_TYPE_ID)
            && compiled
                .graph_ir
                .node(edge.spec.to.node)
                .is_some_and(|node| node.spec.type_id.as_str() == TEST_TRANSFORM_NODE_TYPE_ID)
            && edge.spec.to.port == "text"
    }));
}

#[test]
fn given_unregistered_async_operator_when_compiled_then_typed_error_is_returned() {
    let spec = derived_spec();
    let (node_registry, mut endpoint_registry) = registries();
    endpoint_registry
        .register(
            OperatorId::new(TEST_TEXT_ENDPOINT_OPERATOR_ID),
            NodeTypeId::from(TEST_TEXT_ENDPOINT_NODE_TYPE_ID),
            Arc::new(CompileOnlyEndpointFactory),
        )
        .expect("text endpoint registration");

    let result = SessionCompiler::new(&node_registry, &endpoint_registry).compile(spec);

    assert!(matches!(
        result,
        Err(SessionCompileError::UnknownAsyncOperator { .. })
    ));
}

#[test]
fn given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved() {
    let spec = derived_spec();
    let (node_registry, endpoint_registry) = derived_registries();
    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(spec)
        .expect("compiled derived Session");
    let prepare_context =
        crate::graph::PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));

    let prepared =
        crate::session::prepare_session_runtime(compiled, &node_registry, &prepare_context, 8)
            .expect("prepared derived Session");

    assert!(prepared.worker_mappings().is_empty());
    assert_eq!(prepared.operator_mappings().len(), 1);
    assert_eq!(prepared.operator_mappings()[0].signal_routes().len(), 1);
    assert_eq!(
        prepared.operator_mappings()[0].signal_routes()[0]
            .output_branch
            .route_settings
            .media,
        MediaCaps::Text
    );
}

#[test]
fn given_two_derived_destinations_when_prepared_then_independent_branch_plans_are_preserved() {
    let spec = two_destination_derived_spec();
    let (node_registry, endpoint_registry) = derived_registries();
    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(spec)
        .expect("compiled two-destination Session");
    let prepare_context =
        crate::graph::PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));

    let prepared =
        crate::session::prepare_session_runtime(compiled, &node_registry, &prepare_context, 8)
            .expect("prepared two-destination Session");

    let routes = prepared.operator_mappings()[0].signal_routes();
    assert_eq!(routes.len(), 2);
    assert_ne!(routes[0].route_id(), routes[1].route_id());
    assert_ne!(routes[0].endpoint_id(), routes[1].endpoint_id());
    assert!(routes.iter().all(|route| {
        route.output_branch.route_settings.media == MediaCaps::Text
            && route.output_branch.capacity_signals > 0
    }));
}

#[test]
fn given_text_operator_sent_to_audio_endpoint_when_compiled_then_signal_mismatch_is_typed() {
    let session = Session::new();
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone declaration");
    let browser = session
        .browser("wss://receiver.example.test")
        .expect("browser declaration");
    let transcript = microphone
        .through(Operator::new(
            OperatorId::new(TEST_ASYNC_OPERATOR_ID),
            OperatorConfiguration::new(),
        ))
        .expect("operator declaration");
    transcript.send(browser).expect("derived route declaration");
    let spec = session.freeze().expect("derived spec");
    let (mut node_registry, endpoint_registry) = registries();
    node_registry
        .register_async(Arc::new(CompileOnlyAsyncFactory::new()))
        .expect("async operator registration");

    let result = SessionCompiler::new(&node_registry, &endpoint_registry).compile(spec);

    assert!(matches!(
        result,
        Err(SessionCompileError::GraphCompile(
            CompileError::SignalMismatch { .. }
        ))
    ));
}

#[test]
fn given_unregistered_operator_when_compiled_then_typed_error_is_returned() {
    let spec = product_spec();
    let (node_registry, _) = registries();
    let endpoint_registry = endpoint_registry(None);

    let result = SessionCompiler::new(&node_registry, &endpoint_registry).compile(spec);

    assert!(matches!(
        result,
        Err(SessionCompileError::UnknownOperator { .. })
    ));
}

#[test]
fn given_operator_node_mismatch_when_compiled_then_typed_error_is_returned() {
    let spec = product_spec();
    let (node_registry, _) = registries();
    let endpoint_registry = endpoint_registry(Some(BROWSER_NODE_TYPE_ID));

    let result = SessionCompiler::new(&node_registry, &endpoint_registry).compile(spec);

    assert!(matches!(
        result,
        Err(SessionCompileError::OperatorNodeTypeMismatch { .. })
    ));
}

#[test]
fn given_extension_key_matching_old_metadata_when_compiled_then_value_remains_opaque() {
    let session = Session::new();
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone declaration");
    let connector = session
        .connector(
            OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
            EndpointConfiguration::new().with("session_id", "forged"),
        )
        .expect("connector declaration");
    microphone.send(connector).expect("connector route");
    let spec = session.freeze().expect("valid declaration");
    let (node_registry, endpoint_registry) = registries();

    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(spec)
        .expect("opaque endpoint configuration");
    let endpoint_node = compiled
        .graph_ir
        .nodes
        .iter()
        .find(|node| {
            matches!(
                compiled.bindings.node(node.id()),
                Some(CompiledNodeBinding::Endpoint { .. })
            )
        })
        .expect("compiled endpoint node");

    assert_eq!(endpoint_node.spec.config.get("session_id"), Some("forged"));
}

#[test]
fn given_exact_process_instance_when_lowered_then_typed_declaration_remains_authoritative() {
    let stable_id = crate::capture::StableSourceId::new(
        crate::frame::Platform::Windows,
        crate::capture::SourceKind::Application,
        "wasapi:pid:42:creation-100ns:133801234567890000",
    );
    let session = Session::new();
    let stem = session
        .capture(Source::application(ApplicationSelector::process_instance(
            crate::session::ProcessId::new(42),
            stable_id,
        )))
        .expect("exact process-instance declaration");
    let browser = session
        .browser("wss://receiver.example.test")
        .expect("browser declaration");
    stem.send(browser).expect("browser route");
    let spec = session.freeze().expect("exact process-instance spec");
    assert!(matches!(
        spec.stems()[0].source(),
        Source::Application(ApplicationSelector::ProcessInstance { process_id, stable_id })
            if process_id.get() == 42
                && stable_id.stable_key
                    == "wasapi:pid:42:creation-100ns:133801234567890000"
    ));

    let (node_registry, endpoint_registry) = registries();
    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(spec)
        .expect("typed source declaration");
    let source_node = compiled
        .graph_ir
        .nodes
        .iter()
        .find(|node| {
            matches!(
                compiled.bindings.node(node.id()),
                Some(CompiledNodeBinding::StemSource { .. })
            )
        })
        .expect("compiled source node");

    assert_eq!(source_node.spec.config.iter().count(), 0);
}

#[test]
fn given_unknown_operator_port_when_diagnosed_then_location_is_structured() {
    let error = SessionCompileError::UnknownOperatorPort {
        operator_id: TEST_NAMED_OPERATOR_ID.to_owned(),
        direction: "input",
        port_name: "caller_audio".to_owned(),
    };

    let diagnostic = error.diagnostic();

    assert_eq!(diagnostic.code(), "compile.unknown_operator_port");
    assert_eq!(diagnostic.operator_id(), Some(TEST_NAMED_OPERATOR_ID));
    assert_eq!(diagnostic.direction(), Some("input"));
    assert_eq!(diagnostic.port_name(), Some("caller_audio"));
    assert_eq!(diagnostic.node_index(), None);
    assert_eq!(diagnostic.edge_index(), None);
}

#[test]
fn given_graph_mismatch_when_start_fails_then_diagnostic_is_retained() {
    let engine_error = crate::session::SessionEngineStartError::Compile(
        SessionCompileError::GraphCompile(CompileError::MediaMismatch {
            edge: 7,
            from: "audio/f32/stereo".to_owned(),
            to: "audio/f32/mono".to_owned(),
        }),
    );

    let error = crate::SessionStartError::from(engine_error);
    let diagnostic = error
        .compile_diagnostic()
        .expect("compile error has structured diagnostic");

    assert_eq!(diagnostic.code(), "compile.graph.media_mismatch");
    assert_eq!(diagnostic.edge_index(), Some(7));
    assert_eq!(diagnostic.actual(), Some("audio/f32/stereo"));
    assert_eq!(diagnostic.expected(), Some("audio/f32/mono"));
}

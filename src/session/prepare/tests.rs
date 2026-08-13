use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::endpoint::{
    EndpointDriverFactory, EndpointDriverRegistry, EndpointFailure, EndpointFailureStage,
    EndpointPortInput, PreparedEndpointDriver,
};
use crate::frame::{AudioFrame, RouteId, SampleFormat, SampleSpec};
use crate::graph::{
    AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec, SafetyContract,
    SignalSpec,
};
use crate::graph::{
    ConfigError, ExecutionPartition, NodeConfig, NodeDescriptor, NodeError, NodeFactory,
    RuntimeNode,
};
use crate::runtime::PlanRunnerError;

use super::*;
use crate::session::{
    ApplicationSelector, EndpointConfiguration, OperatorId, Session, SessionCompiler, Source,
    APPLICATION_SOURCE_NODE_TYPE_ID, BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID,
    CONNECTOR_NODE_TYPE_ID, MICROPHONE_SOURCE_NODE_TYPE_ID, RECORDER_NODE_TYPE_ID,
    RECORDER_OPERATOR_ID,
};

const TEST_CONNECTOR_OPERATOR_ID: &str = "example.connector.runtime-prepare.v1";

struct TestFactory {
    descriptor: NodeDescriptor,
    live_nodes: Arc<AtomicUsize>,
}

struct TestNode {
    live_nodes: Arc<AtomicUsize>,
}

struct CompileOnlyEndpointFactory;

impl EndpointDriverFactory for CompileOnlyEndpointFactory {
    fn prepare(
        &self,
        _inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        Err(EndpointFailure::new(
            EndpointFailureStage::Prepare,
            "runtime-prepare compiler fixture must not prepare an endpoint driver",
        ))
    }
}

#[derive(Clone, Copy)]
enum TestNodeRole {
    Source,
    Endpoint,
}

impl NodeFactory for TestFactory {
    fn descriptor(&self) -> NodeDescriptor {
        self.descriptor.clone()
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _context: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        self.live_nodes.fetch_add(1, Ordering::Relaxed);
        Ok(Box::new(TestNode {
            live_nodes: Arc::clone(&self.live_nodes),
        }))
    }
}

impl RuntimeNode for TestNode {
    fn prepare(&mut self, _context: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        Ok(Some(frame))
    }
}

impl Drop for TestNode {
    fn drop(&mut self) {
        self.live_nodes.fetch_sub(1, Ordering::Relaxed);
    }
}

fn audio_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        signal: SignalSpec::audio(),
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(48_000),
            frame_samples: Some(960),
            channel_layout: ChannelLayout::Mono,
            format: SampleFormat::F32Interleaved,
        }),
        multiplicity: Multiplicity::One,
        required: true,
    }
}

fn descriptor(
    node_type_id: &'static str,
    partition: ExecutionPartition,
    role: TestNodeRole,
) -> NodeDescriptor {
    let (inputs, outputs) = match role {
        TestNodeRole::Source => (Vec::new(), vec![audio_port("audio", PortDirection::Output)]),
        TestNodeRole::Endpoint => (vec![audio_port("audio", PortDirection::Input)], Vec::new()),
    };
    NodeDescriptor {
        type_id: NodeTypeId::from(node_type_id),
        display_name: "Explicit runtime preparation test node",
        inputs,
        outputs,
        execution: partition,
        safety: if partition.requires_realtime_safety() {
            SafetyContract::RealtimeSafe
        } else {
            SafetyContract::AllocationAllowed
        },
        stateful: true,
    }
}

fn registries(
    endpoint_partition: ExecutionPartition,
    live_nodes: &Arc<AtomicUsize>,
) -> (NodeRegistry, EndpointDriverRegistry) {
    let mut node_registry = NodeRegistry::new();
    for node_type_id in [
        APPLICATION_SOURCE_NODE_TYPE_ID,
        MICROPHONE_SOURCE_NODE_TYPE_ID,
    ] {
        node_registry
            .register(Arc::new(TestFactory {
                descriptor: descriptor(
                    node_type_id,
                    ExecutionPartition::RealtimeCpu,
                    TestNodeRole::Source,
                ),
                live_nodes: Arc::clone(live_nodes),
            }))
            .unwrap();
    }
    for node_type_id in [
        CONNECTOR_NODE_TYPE_ID,
        BROWSER_NODE_TYPE_ID,
        RECORDER_NODE_TYPE_ID,
    ] {
        node_registry
            .register(Arc::new(TestFactory {
                descriptor: descriptor(node_type_id, endpoint_partition, TestNodeRole::Endpoint),
                live_nodes: Arc::clone(live_nodes),
            }))
            .unwrap();
    }

    let mut endpoint_registry = EndpointDriverRegistry::new();
    let endpoint_factory: Arc<dyn EndpointDriverFactory> = Arc::new(CompileOnlyEndpointFactory);
    for (operator_id, node_type_id) in [
        (TEST_CONNECTOR_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID),
        (BROWSER_OPERATOR_ID, BROWSER_NODE_TYPE_ID),
        (RECORDER_OPERATOR_ID, RECORDER_NODE_TYPE_ID),
    ] {
        endpoint_registry
            .register(
                OperatorId::new(operator_id),
                NodeTypeId::from(node_type_id),
                Arc::clone(&endpoint_factory),
            )
            .expect("test endpoint registration must succeed");
    }
    (node_registry, endpoint_registry)
}

fn product_spec() -> SessionSpec {
    let session = Session::new();
    let application = session
        .capture(Source::application(ApplicationSelector::name(
            "Meeting App",
        )))
        .expect("application declaration must succeed");
    let microphone = session
        .capture(Source::microphone_default())
        .expect("microphone declaration must succeed");
    let connector = session
        .connector(
            OperatorId::new(TEST_CONNECTOR_OPERATOR_ID),
            EndpointConfiguration::new(),
        )
        .expect("connector declaration must succeed");
    let browser = session
        .browser("wss://receiver.example.test")
        .expect("browser declaration must succeed");

    for stem in [&application, &microphone] {
        stem.send(connector).expect("connector route must succeed");
        stem.send(browser).expect("browser route must succeed");
    }
    application
        .record("application")
        .expect("application recording route must succeed");
    microphone
        .record("microphone")
        .expect("microphone recording route must succeed");
    session.freeze().expect("product spec must freeze")
}

fn prepare_context() -> PrepareContext {
    PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
}

#[test]
fn given_product_plan_when_prepared_then_two_sources_and_six_workers_are_owned() {
    let live_nodes = Arc::new(AtomicUsize::new(0));
    let (node_registry, endpoint_registry) =
        registries(ExecutionPartition::AsyncWorker, &live_nodes);
    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(product_spec())
        .expect("product Session must compile");

    let prepared = prepare_session_runtime(compiled, &node_registry, &prepare_context(), 8)
        .expect("runtime preparation must succeed");

    assert_eq!(prepared.source_mappings().len(), 2);
    assert_eq!(prepared.source_input_count(), 2);
    assert_eq!(prepared.worker_mappings().len(), 6);
    assert!(prepared
        .source_mappings()
        .iter()
        .all(|mapping| mapping.sender_observations().queue_capacity_frames == 8));
    assert_eq!(
        prepared
            .source_inputs
            .iter()
            .map(PlanSourceInput::source_node_id)
            .collect::<HashSet<_>>()
            .len(),
        2
    );
    assert_eq!(
        prepared
            .worker_mappings()
            .iter()
            .map(PreparedWorkerMapping::route_id)
            .collect::<HashSet<_>>()
            .len(),
        6
    );
    assert_eq!(
        prepared
            .worker_mappings()
            .iter()
            .map(|mapping| mapping.receiver.edge_id())
            .collect::<HashSet<_>>()
            .len(),
        6
    );
    assert_eq!(live_nodes.load(Ordering::Relaxed), 2);
    drop(prepared);
    assert_eq!(live_nodes.load(Ordering::Relaxed), 0);
}

#[test]
fn given_compiled_endpoint_configuration_when_prepared_then_worker_mapping_preserves_it() {
    let live_nodes = Arc::new(AtomicUsize::new(0));
    let (node_registry, endpoint_registry) =
        registries(ExecutionPartition::AsyncWorker, &live_nodes);
    let mut compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(product_spec())
        .expect("product Session must compile");
    let (endpoint_node_id, route_id) = compiled
        .graph_ir()
        .nodes
        .iter()
        .find_map(|node| match compiled.bindings().node(node.id()) {
            Some(CompiledNodeBinding::Endpoint { route_id, .. }) => {
                Some((node.id(), route_id.to_owned()))
            }
            _ => None,
        })
        .expect("compiled endpoint node must carry typed route identity");
    let endpoint_node = compiled
        .graph_ir_mut()
        .nodes
        .iter_mut()
        .find(|node| node.id() == endpoint_node_id)
        .expect("compiled endpoint node");
    endpoint_node.spec.config = endpoint_node
        .spec
        .config
        .clone()
        .with("compiled_test_marker", "authoritative");

    let prepared = prepare_session_runtime(compiled, &node_registry, &prepare_context(), 8)
        .expect("runtime preparation must succeed");
    let mapping = prepared
        .worker_mappings()
        .iter()
        .find(|mapping| mapping.route_id() == route_id)
        .expect("prepared route mapping");

    assert_eq!(
        mapping.node_configuration().get("compiled_test_marker"),
        Some("authoritative")
    );
}

#[test]
fn given_worker_partition_mismatch_when_prepared_then_error_is_typed_and_nodes_roll_back() {
    let live_nodes = Arc::new(AtomicUsize::new(0));
    let (node_registry, endpoint_registry) =
        registries(ExecutionPartition::RealtimeCpu, &live_nodes);
    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(product_spec())
        .expect("mismatched product Session must still compile");

    let result = prepare_session_runtime(compiled, &node_registry, &prepare_context(), 8);

    assert!(matches!(
        result,
        Err(SessionPrepareError::WorkerTopologyMismatch {
            expected: 6,
            actual: 0,
            expected_operator_inputs: 0,
            actual_operator_inputs: 0,
            expected_signal_endpoints: 0,
            actual_signal_endpoints: 0,
        })
    ));
    assert_eq!(live_nodes.load(Ordering::Relaxed), 0);
}

#[test]
fn given_unknown_worker_route_when_prepared_then_error_is_typed_and_nodes_roll_back() {
    let live_nodes = Arc::new(AtomicUsize::new(0));
    let (node_registry, endpoint_registry) =
        registries(ExecutionPartition::AsyncWorker, &live_nodes);
    let mut compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(product_spec())
        .expect("product Session must compile");
    let endpoint_node_id = compiled
        .graph_ir()
        .nodes
        .iter()
        .find(|node| {
            matches!(
                compiled.bindings().node(node.id()),
                Some(CompiledNodeBinding::Endpoint { .. })
            )
        })
        .map(|node| node.id())
        .expect("compiled endpoint node must carry typed route identity");
    let binding = compiled
        .bindings_mut()
        .node_mut(endpoint_node_id)
        .expect("typed endpoint binding");
    let CompiledNodeBinding::Endpoint { route_id, .. } = binding else {
        panic!("endpoint binding");
    };
    *route_id = RouteId(999_999);

    let result = prepare_session_runtime(compiled, &node_registry, &prepare_context(), 8);

    assert!(matches!(
        result,
        Err(SessionPrepareError::UnknownWorkerRoute {
            route_id: RouteId(999_999),
            ..
        })
    ));
    assert_eq!(live_nodes.load(Ordering::Relaxed), 0);
}

#[test]
fn given_zero_source_capacity_when_prepared_then_error_is_typed_and_nodes_roll_back() {
    let live_nodes = Arc::new(AtomicUsize::new(0));
    let (node_registry, endpoint_registry) =
        registries(ExecutionPartition::AsyncWorker, &live_nodes);
    let compiled = SessionCompiler::new(&node_registry, &endpoint_registry)
        .compile(product_spec())
        .expect("product Session must compile");

    let result = prepare_session_runtime(compiled, &node_registry, &prepare_context(), 0);

    assert!(matches!(
        result,
        Err(SessionPrepareError::SourceChannel(
            PlanRunnerError::ZeroSourceCapacity { .. }
        ))
    ));
    assert_eq!(live_nodes.load(Ordering::Relaxed), 0);
}

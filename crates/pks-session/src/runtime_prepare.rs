use std::collections::HashSet;

use pks_caps::{ChannelLayout, MediaCaps};
use pks_frame::{EndpointId, RouteId, SessionId, StemId};
use pks_frame::{SampleFormat, SampleSpec};
use pks_graph::ir::GraphIr;
use pks_graph::{EdgeId, NodeRegistry, NodeTypeId, PrepareContext};
use pks_runtime::{
    plan_source_channel, ExecError, PlanEdgeReceiver, PlanRunnerCancellation, PlanRunnerError,
    PlanSourceInput, PlanSourceSender, RealtimePlanExecutor,
};

use crate::{
    CompiledSession, SessionSpec, Source, APPLICATION_SOURCE_NODE_TYPE_ID,
    MICROPHONE_SOURCE_NODE_TYPE_ID,
};

#[derive(Debug, thiserror::Error)]
pub enum SessionPrepareError {
    #[error(transparent)]
    Runtime(#[from] ExecError),
    #[error(transparent)]
    SourceChannel(#[from] PlanRunnerError),
    #[error("compiled stem {stem_id:?} has no matching source node")]
    MissingSourceNode { stem_id: StemId },
    #[error("compiled stem {stem_id:?} maps to more than one source node")]
    DuplicateSourceNode { stem_id: StemId },
    #[error(
        "compiled plan produced {actual_receivers} worker receivers for {expected_routes} routes"
    )]
    WorkerReceiverCountMismatch {
        expected_routes: usize,
        actual_receivers: usize,
    },
    #[error("worker edge {edge_id:?} target is absent from the compiled graph")]
    MissingWorkerTarget { edge_id: EdgeId },
    #[error("worker edge {edge_id:?} is absent from the compiled graph")]
    MissingWorkerEdge { edge_id: EdgeId },
    #[error("worker edge {edge_id:?} has no concrete audio sample specification")]
    MissingWorkerSampleSpec { edge_id: EdgeId },
    #[error("worker edge {edge_id:?} target is missing configuration key {key}")]
    MissingWorkerMetadata { edge_id: EdgeId, key: &'static str },
    #[error("worker edge {edge_id:?} target has invalid {key} value {value:?}")]
    InvalidWorkerMetadata {
        edge_id: EdgeId,
        key: &'static str,
        value: String,
    },
    #[error("worker edge {edge_id:?} maps to unknown route {route_id:?}")]
    UnknownWorkerRoute { edge_id: EdgeId, route_id: RouteId },
    #[error("worker route {route_id:?} is mapped more than once")]
    DuplicateWorkerRoute { route_id: RouteId },
    #[error(
        "worker edge {edge_id:?} metadata does not match route {route_id:?}: expected stem {expected_stem_id:?} and endpoint {expected_endpoint_id:?}, got stem {actual_stem_id:?} and endpoint {actual_endpoint_id:?}"
    )]
    WorkerRouteMismatch {
        edge_id: EdgeId,
        route_id: RouteId,
        expected_stem_id: StemId,
        actual_stem_id: StemId,
        expected_endpoint_id: EndpointId,
        actual_endpoint_id: EndpointId,
    },
}

pub struct PreparedSourceMapping {
    pub(crate) stem_id: StemId,
    pub(crate) sender: PlanSourceSender,
}

impl PreparedSourceMapping {
    pub const fn stem_id(&self) -> StemId {
        self.stem_id
    }

    pub fn sender_observations(&self) -> pks_runtime::PlanSourceInputObservations {
        self.sender.observations()
    }
}

pub struct PreparedWorkerMapping {
    pub(crate) route_id: RouteId,
    pub(crate) stem_id: StemId,
    pub(crate) endpoint_id: EndpointId,
    pub(crate) receiver: PlanEdgeReceiver,
    pub(crate) prepare_context: PrepareContext,
}

impl PreparedWorkerMapping {
    pub const fn route_id(&self) -> RouteId {
        self.route_id
    }

    pub const fn stem_id(&self) -> StemId {
        self.stem_id
    }

    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }

    pub fn receiver_observations(&self) -> pks_runtime::EdgeObservations {
        self.receiver.observations()
    }

    pub const fn prepare_context(&self) -> &PrepareContext {
        &self.prepare_context
    }
}

/// Setup-time ownership for one compiled Session.
///
/// Preparation instantiates the realtime plan and allocates only bounded
/// channels. It does not open capture, start endpoint workers, spawn a runtime
/// thread, or publish a `Running` lifecycle state.
pub struct PreparedSession {
    pub(crate) spec: SessionSpec,
    pub(crate) executor: RealtimePlanExecutor,
    pub(crate) source_mappings: Vec<PreparedSourceMapping>,
    pub(crate) source_inputs: Vec<PlanSourceInput>,
    pub(crate) worker_mappings: Vec<PreparedWorkerMapping>,
    pub(crate) cancellation: PlanRunnerCancellation,
}

impl PreparedSession {
    pub const fn session_id(&self) -> SessionId {
        self.spec.session_id()
    }

    pub fn spec(&self) -> &SessionSpec {
        &self.spec
    }

    pub fn source_mappings(&self) -> &[PreparedSourceMapping] {
        &self.source_mappings
    }

    pub fn source_input_count(&self) -> usize {
        self.source_inputs.len()
    }

    pub fn worker_mappings(&self) -> &[PreparedWorkerMapping] {
        &self.worker_mappings
    }

    pub fn route_observations(&self, route_id: RouteId) -> Option<pks_runtime::EdgeObservations> {
        let mapping = self
            .worker_mappings
            .iter()
            .find(|mapping| mapping.route_id == route_id)?;
        self.executor.observations(mapping.receiver.edge_id())
    }

    pub fn cancellation_requested(&self) -> bool {
        self.cancellation.is_requested()
    }
}

pub fn prepare_session_runtime(
    compiled: CompiledSession,
    node_registry: &NodeRegistry,
    prepare_context: &PrepareContext,
    source_queue_capacity_frames: usize,
) -> Result<PreparedSession, SessionPrepareError> {
    let (spec, graph_ir, runtime_plan) = compiled.into_runtime_parts();
    let (executor, worker_receivers) =
        RealtimePlanExecutor::new(&runtime_plan, &graph_ir, node_registry, prepare_context)?;
    let cancellation = PlanRunnerCancellation::new();
    let (source_mappings, source_inputs) = prepare_sources(
        &spec,
        &graph_ir,
        source_queue_capacity_frames,
        &cancellation,
    )?;
    let worker_mappings = map_worker_receivers(&spec, &graph_ir, worker_receivers)?;

    Ok(PreparedSession {
        spec,
        executor,
        source_mappings,
        source_inputs,
        worker_mappings,
        cancellation,
    })
}

fn prepare_sources(
    spec: &SessionSpec,
    graph_ir: &GraphIr,
    source_queue_capacity_frames: usize,
    cancellation: &PlanRunnerCancellation,
) -> Result<(Vec<PreparedSourceMapping>, Vec<PlanSourceInput>), SessionPrepareError> {
    let mut mappings = Vec::with_capacity(spec.stems().len());
    let mut inputs = Vec::with_capacity(spec.stems().len());
    for stem in spec.stems() {
        let expected_type_id = source_node_type_id(stem.source());
        let stem_id = stem.id().0.to_string();
        let mut matches = graph_ir.nodes.iter().filter(|node| {
            node.spec.type_id == expected_type_id
                && node.spec.config.get("stem_id") == Some(stem_id.as_str())
        });
        let node = matches
            .next()
            .ok_or(SessionPrepareError::MissingSourceNode { stem_id: stem.id() })?;
        if matches.next().is_some() {
            return Err(SessionPrepareError::DuplicateSourceNode { stem_id: stem.id() });
        }
        let source_node_id = node.id();
        let (sender, input) = plan_source_channel(
            source_node_id,
            source_queue_capacity_frames,
            cancellation.clone(),
        )?;
        mappings.push(PreparedSourceMapping {
            stem_id: stem.id(),
            sender,
        });
        inputs.push(input);
    }
    Ok((mappings, inputs))
}

fn map_worker_receivers(
    spec: &SessionSpec,
    graph_ir: &GraphIr,
    worker_receivers: Vec<PlanEdgeReceiver>,
) -> Result<Vec<PreparedWorkerMapping>, SessionPrepareError> {
    if worker_receivers.len() != spec.routes().len() {
        return Err(SessionPrepareError::WorkerReceiverCountMismatch {
            expected_routes: spec.routes().len(),
            actual_receivers: worker_receivers.len(),
        });
    }

    let mut mapped_routes = HashSet::with_capacity(worker_receivers.len());
    let mut mappings = Vec::with_capacity(worker_receivers.len());
    for receiver in worker_receivers {
        let edge_id = receiver.edge_id();
        let edge = graph_ir
            .edges
            .iter()
            .find(|edge| edge.spec.id == edge_id)
            .ok_or(SessionPrepareError::MissingWorkerEdge { edge_id })?;
        let prepare_context = prepare_context_for_media(edge.media)
            .ok_or(SessionPrepareError::MissingWorkerSampleSpec { edge_id })?;
        let target = graph_ir
            .node(receiver.to().node)
            .ok_or(SessionPrepareError::MissingWorkerTarget { edge_id })?;
        let route_id = RouteId(parse_metadata(&target.spec.config, edge_id, "route_id")?);
        let stem_id = StemId(parse_metadata(&target.spec.config, edge_id, "stem_id")?);
        let endpoint_id = EndpointId(parse_metadata(&target.spec.config, edge_id, "endpoint_id")?);
        let route = spec
            .routes()
            .iter()
            .find(|route| route.id() == route_id)
            .ok_or(SessionPrepareError::UnknownWorkerRoute { edge_id, route_id })?;
        if route.stem_id() != stem_id || route.endpoint_id() != endpoint_id {
            return Err(SessionPrepareError::WorkerRouteMismatch {
                edge_id,
                route_id,
                expected_stem_id: route.stem_id(),
                actual_stem_id: stem_id,
                expected_endpoint_id: route.endpoint_id(),
                actual_endpoint_id: endpoint_id,
            });
        }
        if !mapped_routes.insert(route_id) {
            return Err(SessionPrepareError::DuplicateWorkerRoute { route_id });
        }
        mappings.push(PreparedWorkerMapping {
            route_id,
            stem_id,
            endpoint_id,
            receiver,
            prepare_context,
        });
    }
    Ok(mappings)
}

fn prepare_context_for_media(media: MediaCaps) -> Option<PrepareContext> {
    let MediaCaps::Audio(audio) = media else {
        return None;
    };
    let channels = match audio.channel_layout {
        ChannelLayout::Mono => 1,
        ChannelLayout::Stereo => 2,
        ChannelLayout::Any => return None,
    };
    Some(PrepareContext::new(SampleSpec::new(
        audio.sample_rate_hz?,
        channels,
        match audio.format {
            SampleFormat::F32Interleaved => SampleFormat::F32Interleaved,
        },
    )))
}

fn parse_metadata(
    config: &pks_graph::NodeConfig,
    edge_id: EdgeId,
    key: &'static str,
) -> Result<u64, SessionPrepareError> {
    let value = config
        .get(key)
        .ok_or(SessionPrepareError::MissingWorkerMetadata { edge_id, key })?;
    value
        .parse()
        .map_err(|_| SessionPrepareError::InvalidWorkerMetadata {
            edge_id,
            key,
            value: value.to_owned(),
        })
}

fn source_node_type_id(source: &Source) -> NodeTypeId {
    match source {
        Source::Application(_) => NodeTypeId::from(APPLICATION_SOURCE_NODE_TYPE_ID),
        Source::Microphone(_) => NodeTypeId::from(MICROPHONE_SOURCE_NODE_TYPE_ID),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
    use pks_frame::{AudioFrame, SampleFormat, SampleSpec};
    use pks_graph::{
        ConfigError, ExecutionPartition, NodeConfig, NodeDescriptor, NodeError, NodeFactory,
        RuntimeNode,
    };

    use super::*;
    use crate::{
        ApplicationSelector, EndpointConfiguration, OperatorId, OperatorRegistry, Session,
        SessionCompiler, BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID,
        RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
    };

    const TEST_CONNECTOR_OPERATOR_ID: &str = "example.connector.runtime-prepare.v1";

    struct TestFactory {
        descriptor: NodeDescriptor,
        live_nodes: Arc<AtomicUsize>,
    }

    struct TestNode {
        live_nodes: Arc<AtomicUsize>,
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
            realtime_safe: partition.requires_realtime_safety(),
            stateful: true,
        }
    }

    fn registries(
        endpoint_partition: ExecutionPartition,
        live_nodes: &Arc<AtomicUsize>,
    ) -> (NodeRegistry, OperatorRegistry) {
        let mut node_registry = NodeRegistry::new();
        for node_type_id in [
            APPLICATION_SOURCE_NODE_TYPE_ID,
            MICROPHONE_SOURCE_NODE_TYPE_ID,
        ] {
            node_registry.register(Arc::new(TestFactory {
                descriptor: descriptor(
                    node_type_id,
                    ExecutionPartition::RealtimeCpu,
                    TestNodeRole::Source,
                ),
                live_nodes: Arc::clone(live_nodes),
            }));
        }
        for node_type_id in [
            CONNECTOR_NODE_TYPE_ID,
            BROWSER_NODE_TYPE_ID,
            RECORDER_NODE_TYPE_ID,
        ] {
            node_registry.register(Arc::new(TestFactory {
                descriptor: descriptor(node_type_id, endpoint_partition, TestNodeRole::Endpoint),
                live_nodes: Arc::clone(live_nodes),
            }));
        }

        let mut operator_registry = OperatorRegistry::new();
        for (operator_id, node_type_id) in [
            (TEST_CONNECTOR_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID),
            (BROWSER_OPERATOR_ID, BROWSER_NODE_TYPE_ID),
            (RECORDER_OPERATOR_ID, RECORDER_NODE_TYPE_ID),
        ] {
            operator_registry
                .register(OperatorId::new(operator_id), NodeTypeId::from(node_type_id))
                .expect("test operator registration must succeed");
        }
        (node_registry, operator_registry)
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
        let (node_registry, operator_registry) =
            registries(ExecutionPartition::AsyncWorker, &live_nodes);
        let compiled = SessionCompiler::new(&node_registry, &operator_registry)
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
    fn given_worker_partition_mismatch_when_prepared_then_error_is_typed_and_nodes_roll_back() {
        let live_nodes = Arc::new(AtomicUsize::new(0));
        let (node_registry, operator_registry) =
            registries(ExecutionPartition::RealtimeCpu, &live_nodes);
        let compiled = SessionCompiler::new(&node_registry, &operator_registry)
            .compile(product_spec())
            .expect("mismatched product Session must still compile");

        let result = prepare_session_runtime(compiled, &node_registry, &prepare_context(), 8);

        assert!(matches!(
            result,
            Err(SessionPrepareError::WorkerReceiverCountMismatch {
                expected_routes: 6,
                actual_receivers: 0,
            })
        ));
        assert_eq!(live_nodes.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn given_unknown_worker_route_when_prepared_then_error_is_typed_and_nodes_roll_back() {
        let live_nodes = Arc::new(AtomicUsize::new(0));
        let (node_registry, operator_registry) =
            registries(ExecutionPartition::AsyncWorker, &live_nodes);
        let mut compiled = SessionCompiler::new(&node_registry, &operator_registry)
            .compile(product_spec())
            .expect("product Session must compile");
        let endpoint_node = compiled
            .graph_ir_mut()
            .nodes
            .iter_mut()
            .find(|node| node.spec.config.get("route_id").is_some())
            .expect("compiled endpoint node must carry route identity");
        endpoint_node.spec.config = endpoint_node.spec.config.clone().with("route_id", "999999");

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
        let (node_registry, operator_registry) =
            registries(ExecutionPartition::AsyncWorker, &live_nodes);
        let compiled = SessionCompiler::new(&node_registry, &operator_registry)
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
}

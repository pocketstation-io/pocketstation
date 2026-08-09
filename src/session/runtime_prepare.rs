use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::frame::{EndpointId, RouteId, SessionId, SourceId, StemId, StreamId};
use crate::frame::{SampleFormat, SampleSpec};
use crate::graph::ir::GraphIr;
use crate::graph::{
    AsyncOperatorFactory, EdgeContract, EdgeId, NodeConfig, NodeId, NodeRegistry, NodeTypeId,
    PrepareContext, SignalSpec,
};
use crate::graph::{ChannelLayout, MediaCaps};
use crate::runtime::{
    plan_source_channel, AsyncOperatorOutputBranchSpec, ExecError, PlanEdgeReceiver,
    PlanRunnerCancellation, PlanRunnerError, PlanSourceInput, PlanSourceSender,
    RealtimePlanExecutor, TypedEdgeBranchSpec,
};

use crate::session::compiler::EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID;
use crate::session::{
    CompiledSession, SessionSpec, Source, SourceConfiguration, SourceInstanceId, SourceTypeId,
    APPLICATION_SOURCE_NODE_TYPE_ID, MICROPHONE_SOURCE_NODE_TYPE_ID,
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
    #[error("compiled external source instance {source_instance_id:?} output '{output_port}' has no matching audio ingress node")]
    MissingExternalAudioIngress {
        source_instance_id: SourceInstanceId,
        output_port: String,
    },
    #[error("external source instance {source_instance_id:?} output '{output_port}' has incompatible or non-concrete PCM media")]
    InvalidExternalAudioMedia {
        source_instance_id: SourceInstanceId,
        output_port: String,
    },
    #[error("compiled external source route {route_id:?} has no matching edge")]
    MissingExternalSourceRouteEdge { route_id: RouteId },
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
    #[error("derived edge {edge_id:?} has no compiled typed-edge plan")]
    MissingTypedEdgePlan { edge_id: EdgeId },
    #[error("compiled operator node {node_id:?} has no registered async factory")]
    MissingAsyncOperatorFactory { node_id: NodeId },
    #[error("compiled operator node {node_id:?} does not match Session operator declaration")]
    OperatorDeclarationMismatch { node_id: NodeId },
    #[error("compiled operator node {node_id:?} has more than one input receiver")]
    DuplicateOperatorInput { node_id: NodeId },
    #[error("derived edge {edge_id:?} has no compiled operator input")]
    MissingDerivedOperatorInput { edge_id: EdgeId },
    #[error("derived route {route_id:?} is mapped more than once")]
    DuplicateDerivedRoute { route_id: RouteId },
    #[error(
        "derived edge {edge_id:?} does not match route {route_id:?} operator/stem declaration"
    )]
    DerivedRouteMismatch { edge_id: EdgeId, route_id: RouteId },
    #[error(
        "compiled plan produced {actual} raw, {actual_operator_inputs} operator-input, and {actual_derived} derived receivers; expected {expected} raw, {expected_operator_inputs} operator-input, and {expected_derived} derived receivers"
    )]
    WorkerTopologyMismatch {
        expected: usize,
        actual: usize,
        expected_operator_inputs: usize,
        actual_operator_inputs: usize,
        expected_derived: usize,
        actual_derived: usize,
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

    pub fn sender_observations(&self) -> crate::runtime::PlanSourceInputObservations {
        self.sender.observations()
    }
}

pub struct PreparedWorkerMapping {
    pub(crate) route_id: RouteId,
    pub(crate) stem_id: StemId,
    pub(crate) endpoint_id: EndpointId,
    pub(crate) node_configuration: NodeConfig,
    pub(crate) receiver: PlanEdgeReceiver,
    pub(crate) prepare_context: PrepareContext,
    pub(crate) origin: PreparedWorkerOrigin,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PreparedWorkerOrigin {
    Stem(StemId),
    ExternalAudio {
        stem_id: StemId,
        source_id: SourceId,
        stream_id: StreamId,
    },
}

impl PreparedWorkerOrigin {
    pub(crate) const fn stem_id(&self) -> StemId {
        match self {
            Self::Stem(stem_id) | Self::ExternalAudio { stem_id, .. } => *stem_id,
        }
    }
}

pub(crate) struct PreparedExternalSourceMapping {
    pub(crate) instance_id: SourceInstanceId,
    pub(crate) source_id: SourceId,
    pub(crate) source_type_id: SourceTypeId,
    pub(crate) configuration: SourceConfiguration,
    pub(crate) branches: Vec<PreparedExternalSourceBranch>,
}

pub(crate) struct PreparedExternalSourceBranch {
    pub(crate) output_port: String,
    pub(crate) stream_id: StreamId,
    pub(crate) branch: TypedEdgeBranchSpec,
    pub(crate) target: PreparedExternalSourceTarget,
}

pub(crate) enum PreparedExternalSourceTarget {
    AudioIngress(PreparedExternalAudioIngress),
    TypedEndpoint(PreparedExternalTypedRoute),
}

pub(crate) struct PreparedExternalAudioIngress {
    pub(crate) stem_id: StemId,
    pub(crate) stream_id: StreamId,
    pub(crate) source_id: SourceId,
    pub(crate) sample_spec: SampleSpec,
    pub(crate) samples_per_frame: usize,
    pub(crate) sender: PlanSourceSender,
}

pub(crate) struct PreparedExternalTypedRoute {
    pub(crate) route_id: RouteId,
    pub(crate) endpoint_id: EndpointId,
    pub(crate) endpoint_operator_id: crate::endpoint::OperatorId,
    pub(crate) endpoint_node_type_id: NodeTypeId,
    pub(crate) stream_id: StreamId,
    pub(crate) source_id: SourceId,
    pub(crate) node_configuration: NodeConfig,
    pub(crate) signal_spec: SignalSpec,
    pub(crate) media: MediaCaps,
    pub(crate) edge_contract: EdgeContract,
}

pub struct PreparedDerivedRouteMapping {
    pub(crate) route_id: RouteId,
    pub(crate) stem_id: StemId,
    pub(crate) endpoint_id: EndpointId,
    pub(crate) node_configuration: NodeConfig,
    pub(crate) prepare_context: PrepareContext,
    pub(crate) signal_spec: SignalSpec,
    pub(crate) output_branch: AsyncOperatorOutputBranchSpec,
}

impl PreparedDerivedRouteMapping {
    pub const fn route_id(&self) -> RouteId {
        self.route_id
    }

    pub const fn stem_id(&self) -> StemId {
        self.stem_id
    }

    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }
}

pub struct PreparedOperatorMapping {
    pub(crate) node_id: NodeId,
    pub(crate) instance_id: crate::session::OperatorInstanceId,
    pub(crate) stem_id: StemId,
    pub(crate) factory: Arc<dyn AsyncOperatorFactory>,
    pub(crate) node_configuration: NodeConfig,
    pub(crate) prepare_context: PrepareContext,
    pub(crate) receiver: PlanEdgeReceiver,
    pub(crate) derived_routes: Vec<PreparedDerivedRouteMapping>,
}

impl PreparedOperatorMapping {
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn derived_routes(&self) -> &[PreparedDerivedRouteMapping] {
        &self.derived_routes
    }
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

    pub const fn node_configuration(&self) -> &NodeConfig {
        &self.node_configuration
    }

    pub fn receiver_observations(&self) -> crate::runtime::EdgeObservations {
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
    pub(crate) operator_mappings: Vec<PreparedOperatorMapping>,
    pub(crate) external_source_mappings: Vec<PreparedExternalSourceMapping>,
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

    pub fn operator_mappings(&self) -> &[PreparedOperatorMapping] {
        &self.operator_mappings
    }

    pub fn route_observations(
        &self,
        route_id: RouteId,
    ) -> Option<crate::runtime::EdgeObservations> {
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
    let (source_mappings, mut source_inputs) = prepare_sources(
        &spec,
        &graph_ir,
        source_queue_capacity_frames,
        &cancellation,
    )?;
    let external_source_mappings = prepare_external_sources(
        &spec,
        &graph_ir,
        &runtime_plan,
        node_registry,
        prepare_context,
        source_queue_capacity_frames,
        &cancellation,
        &mut source_inputs,
    )?;
    let (worker_mappings, operator_mappings) = map_worker_receivers(
        &spec,
        &graph_ir,
        &runtime_plan,
        node_registry,
        prepare_context,
        worker_receivers,
    )?;

    Ok(PreparedSession {
        spec,
        executor,
        source_mappings,
        source_inputs,
        worker_mappings,
        operator_mappings,
        external_source_mappings,
        cancellation,
    })
}

#[allow(clippy::too_many_arguments)]
fn prepare_external_sources(
    spec: &SessionSpec,
    graph_ir: &GraphIr,
    runtime_plan: &crate::graph::plan::RuntimePlan,
    node_registry: &NodeRegistry,
    default_prepare_context: &PrepareContext,
    source_queue_capacity_frames: usize,
    cancellation: &PlanRunnerCancellation,
    source_inputs: &mut Vec<PlanSourceInput>,
) -> Result<Vec<PreparedExternalSourceMapping>, SessionPrepareError> {
    let mut prepared = Vec::with_capacity(spec.source_instances().len());
    for source in spec.source_instances() {
        let descriptor = node_registry
            .definition(&NodeTypeId::from(source.source_type_id().as_str()))
            .expect("compiled external source definition");
        let manifest = descriptor.descriptor();
        let mut branches = Vec::new();
        for port in &manifest.outputs {
            let Some(output) = spec.source_outputs().iter().find(|output| {
                output.source_instance_id() == source.instance_id()
                    && output.output_port() == port.name
            }) else {
                continue;
            };
            if port.signal.class.is_audio() {
                let node = graph_ir
                    .nodes
                    .iter()
                    .find(|node| {
                        node.spec.type_id.as_str() == EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID
                            && node.spec.config.get("source_instance_id")
                                == Some(source.instance_id().value().to_string().as_str())
                            && node.spec.config.get("source_output_port")
                                == Some(output.output_port())
                    })
                    .ok_or_else(|| SessionPrepareError::MissingExternalAudioIngress {
                        source_instance_id: source.instance_id(),
                        output_port: output.output_port().to_owned(),
                    })?;
                let MediaCaps::Audio(audio) = port.media else {
                    return Err(SessionPrepareError::InvalidExternalAudioMedia {
                        source_instance_id: source.instance_id(),
                        output_port: output.output_port().to_owned(),
                    });
                };
                let channels = match audio.channel_layout {
                    ChannelLayout::Mono => 1,
                    ChannelLayout::Stereo => 2,
                    ChannelLayout::Any => default_prepare_context.sample_spec.channels,
                };
                let sample_spec = SampleSpec::new(
                    audio
                        .sample_rate_hz
                        .unwrap_or(default_prepare_context.sample_spec.sample_rate_hz),
                    channels,
                    audio.format,
                );
                if sample_spec != default_prepare_context.sample_spec {
                    return Err(SessionPrepareError::InvalidExternalAudioMedia {
                        source_instance_id: source.instance_id(),
                        output_port: output.output_port().to_owned(),
                    });
                }
                let frame_samples = audio.frame_samples.ok_or_else(|| {
                    SessionPrepareError::InvalidExternalAudioMedia {
                        source_instance_id: source.instance_id(),
                        output_port: output.output_port().to_owned(),
                    }
                })?;
                let (sender, input) = plan_source_channel(
                    node.id(),
                    source_queue_capacity_frames,
                    cancellation.clone(),
                )?;
                source_inputs.push(input);
                let mut edge_contract = EdgeContract::voice_default();
                edge_contract.media = port.media;
                branches.push(PreparedExternalSourceBranch {
                    output_port: output.output_port().to_owned(),
                    stream_id: output.stream_id(),
                    branch: TypedEdgeBranchSpec {
                        capacity_signals: source_queue_capacity_frames,
                        edge_contract,
                    },
                    target: PreparedExternalSourceTarget::AudioIngress(
                        PreparedExternalAudioIngress {
                            stem_id: external_audio_stem_id(spec, output),
                            stream_id: output.stream_id(),
                            source_id: source.source_id(),
                            sample_spec,
                            samples_per_frame: frame_samples.saturating_mul(usize::from(channels)),
                            sender,
                        },
                    ),
                });
                continue;
            }

            for route in spec.source_routes().iter().filter(|route| {
                route.source_instance_id() == source.instance_id()
                    && route.output_port() == output.output_port()
            }) {
                let route_id = route.id().0.to_string();
                let edge = graph_ir
                    .edges
                    .iter()
                    .find(|edge| {
                        graph_ir.node(edge.spec.to.node).is_some_and(|target| {
                            target.spec.config.get("route_id") == Some(route_id.as_str())
                        })
                    })
                    .ok_or(SessionPrepareError::MissingExternalSourceRouteEdge {
                        route_id: route.id(),
                    })?;
                let typed_edge = runtime_plan.typed_edge(edge.spec.id).ok_or(
                    SessionPrepareError::MissingTypedEdgePlan {
                        edge_id: edge.spec.id,
                    },
                )?;
                let target = graph_ir.node(edge.spec.to.node).ok_or(
                    SessionPrepareError::MissingWorkerTarget {
                        edge_id: edge.spec.id,
                    },
                )?;
                let endpoint = spec
                    .endpoints()
                    .iter()
                    .find(|endpoint| endpoint.id() == route.endpoint_id())
                    .ok_or(SessionPrepareError::UnknownWorkerRoute {
                        edge_id: edge.spec.id,
                        route_id: route.id(),
                    })?;
                branches.push(PreparedExternalSourceBranch {
                    output_port: output.output_port().to_owned(),
                    stream_id: output.stream_id(),
                    branch: TypedEdgeBranchSpec {
                        capacity_signals: typed_edge.capacity_signals,
                        edge_contract: typed_edge.contract,
                    },
                    target: PreparedExternalSourceTarget::TypedEndpoint(
                        PreparedExternalTypedRoute {
                            route_id: route.id(),
                            endpoint_id: route.endpoint_id(),
                            endpoint_operator_id: endpoint.operator_id().clone(),
                            endpoint_node_type_id: endpoint.node_type_id().clone(),
                            stream_id: route.stream_id(),
                            source_id: route.source_id(),
                            node_configuration: target.spec.config.clone(),
                            signal_spec: typed_edge.signal.clone(),
                            media: typed_edge.media,
                            edge_contract: typed_edge.contract,
                        },
                    ),
                });
            }
        }
        prepared.push(PreparedExternalSourceMapping {
            instance_id: source.instance_id(),
            source_id: source.source_id(),
            source_type_id: source.source_type_id().clone(),
            configuration: source.configuration().clone(),
            branches,
        });
    }
    Ok(prepared)
}

fn external_audio_stem_id(spec: &SessionSpec, output: &crate::session::SourceOutputSpec) -> StemId {
    let built_in_max = spec
        .stems()
        .iter()
        .map(|stem| stem.id().0)
        .max()
        .unwrap_or(0);
    let output_index = spec
        .source_outputs()
        .iter()
        .position(|candidate| candidate == output)
        .unwrap_or(0) as u64;
    StemId(built_in_max.saturating_add(output_index).saturating_add(1))
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
    runtime_plan: &crate::graph::plan::RuntimePlan,
    node_registry: &NodeRegistry,
    default_prepare_context: &PrepareContext,
    worker_receivers: Vec<PlanEdgeReceiver>,
) -> Result<(Vec<PreparedWorkerMapping>, Vec<PreparedOperatorMapping>), SessionPrepareError> {
    let mut mapped_routes = HashSet::with_capacity(worker_receivers.len());
    let mut mappings = Vec::with_capacity(worker_receivers.len());
    let mut operator_mappings = Vec::with_capacity(spec.operators().len());
    let mut operator_indices = HashMap::with_capacity(spec.operators().len());
    let mut pending_derived = Vec::with_capacity(spec.derived_routes().len());
    let mut derived_edge_ids = HashSet::with_capacity(spec.derived_routes().len());
    let mut mapped_derived_routes = HashSet::with_capacity(spec.derived_routes().len());

    for edge in &graph_ir.edges {
        let source =
            graph_ir
                .node(edge.spec.from.node)
                .ok_or(SessionPrepareError::MissingWorkerEdge {
                    edge_id: edge.spec.id,
                })?;
        let Some(factory) = node_registry.async_factory(&source.spec.type_id) else {
            continue;
        };
        let target =
            graph_ir
                .node(edge.spec.to.node)
                .ok_or(SessionPrepareError::MissingWorkerTarget {
                    edge_id: edge.spec.id,
                })?;
        let route_id = RouteId(parse_metadata(
            &target.spec.config,
            edge.spec.id,
            "route_id",
        )?);
        let stem_id = StemId(parse_metadata(
            &target.spec.config,
            edge.spec.id,
            "stem_id",
        )?);
        let endpoint_id = EndpointId(parse_metadata(
            &target.spec.config,
            edge.spec.id,
            "endpoint_id",
        )?);
        let route = spec
            .derived_routes()
            .iter()
            .find(|route| route.id() == route_id)
            .ok_or(SessionPrepareError::UnknownWorkerRoute {
                edge_id: edge.spec.id,
                route_id,
            })?;
        let operator_instance_id = crate::session::OperatorInstanceId::new(parse_metadata(
            &source.spec.config,
            edge.spec.id,
            "operator_instance_id",
        )?);
        let operator = spec
            .operators()
            .iter()
            .find(|operator| operator.instance_id() == operator_instance_id)
            .ok_or(SessionPrepareError::DerivedRouteMismatch {
                edge_id: edge.spec.id,
                route_id,
            })?;
        if route.endpoint_id() != endpoint_id
            || route.operator_instance_id() != operator_instance_id
            || operator.source_stem_id() != Some(stem_id)
        {
            return Err(SessionPrepareError::DerivedRouteMismatch {
                edge_id: edge.spec.id,
                route_id,
            });
        }
        if !mapped_derived_routes.insert(route_id) {
            return Err(SessionPrepareError::DuplicateDerivedRoute { route_id });
        }
        let signal_spec = factory
            .manifest()
            .output_ports()
            .next()
            .ok_or(SessionPrepareError::DerivedRouteMismatch {
                edge_id: edge.spec.id,
                route_id,
            })?
            .signal
            .clone();
        let typed_edge = runtime_plan.typed_edge(edge.spec.id).ok_or(
            SessionPrepareError::MissingTypedEdgePlan {
                edge_id: edge.spec.id,
            },
        )?;
        derived_edge_ids.insert(edge.spec.id);
        pending_derived.push((
            source.id(),
            edge.spec.id,
            PreparedDerivedRouteMapping {
                route_id,
                stem_id,
                endpoint_id,
                node_configuration: target.spec.config.clone(),
                prepare_context: default_prepare_context.clone(),
                signal_spec,
                output_branch: AsyncOperatorOutputBranchSpec {
                    capacity_signals: typed_edge.capacity_signals,
                    edge_contract: typed_edge.contract,
                },
            },
        ));
    }

    for receiver in worker_receivers {
        let edge_id = receiver.edge_id();
        let edge = graph_ir
            .edges
            .iter()
            .find(|edge| edge.spec.id == edge_id)
            .ok_or(SessionPrepareError::MissingWorkerEdge { edge_id })?;
        let target = graph_ir
            .node(receiver.to().node)
            .ok_or(SessionPrepareError::MissingWorkerTarget { edge_id })?;
        if let Some(factory) = node_registry.async_factory(&target.spec.type_id) {
            let prepare_context = prepare_context_for_media(edge.media)
                .ok_or(SessionPrepareError::MissingWorkerSampleSpec { edge_id })?;
            let instance_id = crate::session::OperatorInstanceId::new(parse_metadata(
                &target.spec.config,
                edge_id,
                "operator_instance_id",
            )?);
            let input_route_id = RouteId(parse_metadata(
                &target.spec.config,
                edge_id,
                "input_route_id",
            )?);
            let stem_id = StemId(parse_metadata(&target.spec.config, edge_id, "stem_id")?);
            let declaration = spec
                .operators()
                .iter()
                .find(|operator| operator.instance_id() == instance_id)
                .ok_or(SessionPrepareError::OperatorDeclarationMismatch {
                    node_id: target.id(),
                })?;
            if declaration.input_route_id() != input_route_id
                || declaration.source_stem_id() != Some(stem_id)
                || declaration.operator_id() != &factory.manifest().operator_id
            {
                return Err(SessionPrepareError::OperatorDeclarationMismatch {
                    node_id: target.id(),
                });
            }
            let mapping_index = operator_mappings.len();
            if operator_indices
                .insert(target.id(), mapping_index)
                .is_some()
            {
                return Err(SessionPrepareError::DuplicateOperatorInput {
                    node_id: target.id(),
                });
            }
            operator_mappings.push(PreparedOperatorMapping {
                node_id: target.id(),
                instance_id,
                stem_id,
                factory: Arc::clone(factory),
                node_configuration: target.spec.config.clone(),
                prepare_context,
                receiver,
                derived_routes: Vec::new(),
            });
            continue;
        }

        if derived_edge_ids.contains(&edge_id) {
            drop(receiver);
            continue;
        }

        let prepare_context = prepare_context_for_media(edge.media)
            .ok_or(SessionPrepareError::MissingWorkerSampleSpec { edge_id })?;
        let route_id = RouteId(parse_metadata(&target.spec.config, edge_id, "route_id")?);
        let endpoint_id = EndpointId(parse_metadata(&target.spec.config, edge_id, "endpoint_id")?);
        if let Some(source_instance_id) = target.spec.config.get("source_instance_id") {
            let source_instance_id =
                SourceInstanceId::new(source_instance_id.parse().map_err(|_| {
                    SessionPrepareError::InvalidWorkerMetadata {
                        edge_id,
                        key: "source_instance_id",
                        value: source_instance_id.to_owned(),
                    }
                })?);
            let route = spec
                .source_routes()
                .iter()
                .find(|route| route.id() == route_id)
                .ok_or(SessionPrepareError::UnknownWorkerRoute { edge_id, route_id })?;
            if route.source_instance_id() != source_instance_id
                || route.endpoint_id() != endpoint_id
            {
                return Err(SessionPrepareError::UnknownWorkerRoute { edge_id, route_id });
            }
            if !mapped_routes.insert(route_id) {
                return Err(SessionPrepareError::DuplicateWorkerRoute { route_id });
            }
            let output = spec
                .source_outputs()
                .iter()
                .find(|output| {
                    output.source_instance_id() == source_instance_id
                        && output.output_port() == route.output_port()
                })
                .ok_or(SessionPrepareError::UnknownWorkerRoute { edge_id, route_id })?;
            let stem_id = external_audio_stem_id(spec, output);
            mappings.push(PreparedWorkerMapping {
                route_id,
                stem_id,
                endpoint_id,
                node_configuration: target.spec.config.clone(),
                receiver,
                prepare_context,
                origin: PreparedWorkerOrigin::ExternalAudio {
                    stem_id,
                    source_id: route.source_id(),
                    stream_id: route.stream_id(),
                },
            });
            continue;
        }
        let stem_id = StemId(parse_metadata(&target.spec.config, edge_id, "stem_id")?);
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
            node_configuration: target.spec.config.clone(),
            receiver,
            prepare_context,
            origin: PreparedWorkerOrigin::Stem(stem_id),
        });
    }
    for (operator_node_id, edge_id, derived) in pending_derived {
        let mapping_index = operator_indices
            .get(&operator_node_id)
            .copied()
            .ok_or(SessionPrepareError::MissingDerivedOperatorInput { edge_id })?;
        operator_mappings[mapping_index]
            .derived_routes
            .push(derived);
    }
    let expected_audio_source_routes = mappings
        .iter()
        .filter(|mapping| matches!(mapping.origin, PreparedWorkerOrigin::ExternalAudio { .. }))
        .count();
    if mappings.len() != spec.routes().len() + expected_audio_source_routes
        || operator_mappings.len() != spec.operators().len()
        || operator_mappings
            .iter()
            .map(|mapping| mapping.derived_routes.len())
            .sum::<usize>()
            != spec.derived_routes().len()
    {
        return Err(SessionPrepareError::WorkerTopologyMismatch {
            expected: spec.routes().len() + expected_audio_source_routes,
            actual: mappings.len(),
            expected_operator_inputs: spec.operators().len(),
            actual_operator_inputs: operator_mappings.len(),
            expected_derived: spec.derived_routes().len(),
            actual_derived: operator_mappings
                .iter()
                .map(|mapping| mapping.derived_routes.len())
                .sum(),
        });
    }
    Ok((mappings, operator_mappings))
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
    config: &crate::graph::NodeConfig,
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

    use crate::endpoint::{
        EndpointDriverFactory, EndpointDriverInput, EndpointDriverRegistry, EndpointFailure,
        EndpointFailureStage, PreparedEndpointDriver,
    };
    use crate::frame::{AudioFrame, SampleFormat, SampleSpec};
    use crate::graph::{
        AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec, SafetyContract,
        SignalSpec,
    };
    use crate::graph::{
        ConfigError, ExecutionPartition, NodeConfig, NodeDescriptor, NodeError, NodeFactory,
        RuntimeNode,
    };

    use super::*;
    use crate::session::{
        ApplicationSelector, EndpointConfiguration, OperatorId, Session, SessionCompiler,
        BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID, RECORDER_NODE_TYPE_ID,
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
            _inputs: Vec<EndpointDriverInput>,
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
                    descriptor: descriptor(
                        node_type_id,
                        endpoint_partition,
                        TestNodeRole::Endpoint,
                    ),
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
        let endpoint_node = compiled
            .graph_ir_mut()
            .nodes
            .iter_mut()
            .find(|node| node.spec.config.get("route_id").is_some())
            .expect("compiled endpoint node must carry route identity");
        let route_id = RouteId(
            endpoint_node
                .spec
                .config
                .get("route_id")
                .expect("compiled route identity")
                .parse()
                .expect("numeric compiled route identity"),
        );
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
                expected_derived: 0,
                actual_derived: 0,
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
}

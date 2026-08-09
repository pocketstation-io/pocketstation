use std::collections::HashMap;

use crate::endpoint::EndpointDriverRegistry;
use crate::graph::compiler::{CompileError, Compiler};
use crate::graph::ir::GraphIr;
use crate::graph::planner::RuntimePlanner;
use crate::graph::{
    BackpressurePolicy, CopyPolicy, EdgeContract, EdgeObservabilityLevel, LossPolicy, MediaCaps,
    NodeConfig, NodeHandle, NodeRegistry, NodeTypeId, OutputPortRef, Pipeline,
};

use crate::session::{
    ApplicationSelector, DeviceSelector, EndpointSpec, OperatorConnectionSpec, OperatorInputOrigin,
    OperatorInstanceId, OperatorInstanceSpec, SessionError, SessionId, SessionSpec, Source,
    SourceInstanceId, SourceInstanceSpec, SourceRegistry, SourceRouteSpec, SourceTypeId, StemSpec,
};

pub const APPLICATION_SOURCE_NODE_TYPE_ID: &str = "source.application";
pub const MICROPHONE_SOURCE_NODE_TYPE_ID: &str = "source.microphone";
pub(crate) const EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID: &str = "source.external-audio-ingress";
pub const CONNECTOR_NODE_TYPE_ID: &str = "endpoint.connector.external";
pub const BROWSER_NODE_TYPE_ID: &str = "endpoint.browser.remote";
pub const BROWSER_OPERATOR_ID: &str = "io.pocketstation.browser.webrtc.v1";
pub const RECORDER_NODE_TYPE_ID: &str = "endpoint.recording.multistem";
pub const RECORDER_OPERATOR_ID: &str = "io.pocketstation.recording.wav-stems.v1";
pub const RECORDING_GROUP_CONFIGURATION_KEY: &str = "recording_group_id";
pub const DEFAULT_MULTISTEM_RECORDING_GROUP_ID: &str = "session.multistem.default.v1";
const AUDIO_OUTPUT_PORT: &str = "audio";
const AUDIO_INPUT_PORT: &str = "audio";
// A recording worker is not a mouth-to-ear destination. Give only recorder
// routes enough bounded scheduler headroom to survive a short desktop/VM
// deschedule without inflating latency-sensitive connector and browser edges.
// With the canonical 20 ms frame this plans 20 slots (400 ms). Overflow is
// still explicit and observable; the capture/realtime producer never blocks.
const RECORDING_SCHEDULER_HEADROOM_MS: u32 = 400;
const RESERVED_ENDPOINT_CONFIGURATION_KEYS: [&str; 10] = [
    "session_id",
    "stem_id",
    "endpoint_id",
    "route_id",
    "operator_id",
    "connector_id",
    "source_instance_id",
    "source_id",
    "stream_id",
    "source_output_port",
];
const RESERVED_OPERATOR_CONFIGURATION_KEYS: [&str; 12] = [
    "session_id",
    "stem_id",
    "input_route_id",
    "operator_instance_id",
    "operator_id",
    "operator_revision",
    "operator_generation",
    "node_type_id",
    "source_instance_id",
    "source_id",
    "stream_id",
    "source_output_port",
];

#[derive(Debug, thiserror::Error)]
pub enum SessionCompileError {
    #[error(transparent)]
    InvalidSpec(#[from] SessionError),
    #[error("operator {operator_id} is not registered")]
    UnknownOperator { operator_id: String },
    #[error(
        "operator {operator_id} resolves to node type {registered_node_type_id}, not {declared_node_type_id}"
    )]
    OperatorNodeTypeMismatch {
        operator_id: String,
        registered_node_type_id: String,
        declared_node_type_id: String,
    },
    #[error("endpoint {endpoint_id:?} configuration uses reserved key {key}")]
    ReservedEndpointConfigurationKey {
        endpoint_id: crate::session::EndpointId,
        key: String,
    },
    #[error("operator instance {operator_instance_id:?} configuration uses reserved key {key}")]
    ReservedOperatorConfigurationKey {
        operator_instance_id: OperatorInstanceId,
        key: String,
    },
    #[error("async operator {operator_id} is not registered")]
    UnknownAsyncOperator { operator_id: String },
    #[error("derived endpoint node type {node_type_id} is not registered")]
    UnknownDerivedEndpointNodeType { node_type_id: String },
    #[error(
        "derived endpoint node type {node_type_id} has {input_ports_total} inputs; send(destination) requires exactly one"
    )]
    AmbiguousDerivedEndpointInput {
        node_type_id: String,
        input_ports_total: usize,
    },
    #[error("operator {operator_id} requires explicit {direction} port selection")]
    AmbiguousOperatorPort {
        operator_id: String,
        direction: &'static str,
    },
    #[error("operator {operator_id} has no {direction} port named '{port_name}'")]
    UnknownOperatorPort {
        operator_id: String,
        direction: &'static str,
        port_name: String,
    },
    #[error("operator instance {operator_instance_id:?} required input port '{port_name}' is not connected")]
    MissingRequiredOperatorInput {
        operator_instance_id: OperatorInstanceId,
        port_name: String,
    },
    #[error("operator instance {operator_instance_id:?} input port '{port_name}' is connected more than once")]
    DuplicateOperatorInputConnection {
        operator_instance_id: OperatorInstanceId,
        port_name: String,
    },
    #[error("required source node type {node_type_id} is not registered")]
    UnknownSourceNodeType { node_type_id: String },
    #[error("external source type {source_type_id} is not registered on SessionEngine")]
    UnknownExternalSource { source_type_id: SourceTypeId },
    #[error(
        "external source type {source_type_id} has no declared output port named '{output_port}'"
    )]
    UnknownExternalSourceOutput {
        source_type_id: SourceTypeId,
        output_port: String,
    },
    #[error("external source type {source_type_id} configuration is invalid: {reason}")]
    InvalidExternalSourceConfiguration {
        source_type_id: SourceTypeId,
        reason: String,
    },
    #[error("endpoint node type {node_type_id} has no input port named '{port_name}'")]
    UnknownEndpointInputPort {
        node_type_id: String,
        port_name: String,
    },
    #[error(transparent)]
    GraphCompile(#[from] CompileError),
    #[error(transparent)]
    RuntimePlan(#[from] crate::graph::plan::PlanError),
}

pub struct CompiledSession {
    spec: SessionSpec,
    graph_ir: GraphIr,
    runtime_plan: crate::graph::plan::RuntimePlan,
}

impl CompiledSession {
    pub const fn session_id(&self) -> SessionId {
        self.spec.session_id()
    }

    pub fn spec(&self) -> &SessionSpec {
        &self.spec
    }

    pub fn source_declarations(&self) -> &[StemSpec] {
        self.spec.stems()
    }

    pub fn external_source_declarations(&self) -> &[SourceInstanceSpec] {
        self.spec.source_instances()
    }

    pub fn endpoint_declarations(&self) -> &[EndpointSpec] {
        self.spec.endpoints()
    }

    pub fn node_count(&self) -> usize {
        self.graph_ir.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph_ir.edge_count()
    }

    pub fn planned_edge_count(&self) -> usize {
        self.runtime_plan.edge_count
    }

    #[cfg(test)]
    pub(crate) fn planned_source_output_count(&self) -> usize {
        self.runtime_plan.source_outputs.len()
    }

    #[cfg(test)]
    pub(crate) fn planned_typed_edge_count(&self) -> usize {
        self.runtime_plan.typed_edges.len()
    }

    #[cfg(test)]
    pub(crate) fn planned_audio_edge_count(&self) -> usize {
        self.runtime_plan.memory_plan.edge_buffers.len()
    }

    pub(crate) fn into_runtime_parts(
        self,
    ) -> (SessionSpec, GraphIr, crate::graph::plan::RuntimePlan) {
        (self.spec, self.graph_ir, self.runtime_plan)
    }

    #[cfg(test)]
    pub(crate) fn graph_ir_mut(&mut self) -> &mut GraphIr {
        &mut self.graph_ir
    }
}

pub struct SessionCompiler<'registry> {
    node_registry: &'registry NodeRegistry,
    endpoint_registry: &'registry EndpointDriverRegistry,
    source_registry: Option<&'registry SourceRegistry>,
}

impl<'registry> SessionCompiler<'registry> {
    pub const fn new(
        node_registry: &'registry NodeRegistry,
        endpoint_registry: &'registry EndpointDriverRegistry,
    ) -> Self {
        Self {
            node_registry,
            endpoint_registry,
            source_registry: None,
        }
    }

    pub const fn with_sources(
        node_registry: &'registry NodeRegistry,
        endpoint_registry: &'registry EndpointDriverRegistry,
        source_registry: &'registry SourceRegistry,
    ) -> Self {
        Self {
            node_registry,
            endpoint_registry,
            source_registry: Some(source_registry),
        }
    }

    pub fn compile(&self, spec: SessionSpec) -> Result<CompiledSession, SessionCompileError> {
        spec.validate()?;
        self.validate_source_node_types(&spec)?;
        self.validate_external_sources(&spec)?;
        self.validate_endpoint_operators(&spec)?;
        self.validate_async_operators(&spec)?;

        let graph_spec = self.lower_graph_spec(&spec)?;
        let graph_ir = Compiler::new().compile(graph_spec, self.node_registry)?;
        let runtime_plan = RuntimePlanner::new().plan(&graph_ir)?;

        Ok(CompiledSession {
            spec,
            graph_ir,
            runtime_plan,
        })
    }

    fn validate_source_node_types(&self, spec: &SessionSpec) -> Result<(), SessionCompileError> {
        for stem in spec.stems() {
            let node_type_id = source_node_type_id(stem.source());
            if !self.node_registry.contains(&node_type_id) {
                return Err(SessionCompileError::UnknownSourceNodeType {
                    node_type_id: node_type_id.as_str().to_owned(),
                });
            }
        }
        Ok(())
    }

    fn validate_external_sources(&self, spec: &SessionSpec) -> Result<(), SessionCompileError> {
        for source in spec.source_instances() {
            let registry =
                self.source_registry
                    .ok_or_else(|| SessionCompileError::UnknownExternalSource {
                        source_type_id: source.source_type_id().clone(),
                    })?;
            let manifest = registry.manifest(source.source_type_id()).ok_or_else(|| {
                SessionCompileError::UnknownExternalSource {
                    source_type_id: source.source_type_id().clone(),
                }
            })?;
            registry
                .validate_config(source.source_type_id(), source.configuration())
                .map_err(
                    |error| SessionCompileError::InvalidExternalSourceConfiguration {
                        source_type_id: source.source_type_id().clone(),
                        reason: error.to_string(),
                    },
                )?;
            for output in spec
                .source_outputs()
                .iter()
                .filter(|output| output.source_instance_id() == source.instance_id())
            {
                if manifest.output_port(output.output_port()).is_none() {
                    return Err(SessionCompileError::UnknownExternalSourceOutput {
                        source_type_id: source.source_type_id().clone(),
                        output_port: output.output_port().to_owned(),
                    });
                }
            }
            let node_type_id = NodeTypeId::from(source.source_type_id().as_str());
            if !self.node_registry.contains(&node_type_id) {
                return Err(SessionCompileError::UnknownSourceNodeType {
                    node_type_id: node_type_id.as_str().to_owned(),
                });
            }
        }
        Ok(())
    }

    fn validate_endpoint_operators(&self, spec: &SessionSpec) -> Result<(), SessionCompileError> {
        for endpoint in spec.endpoints() {
            if let Some((key, _)) = endpoint
                .configuration()
                .iter()
                .find(|(key, _)| RESERVED_ENDPOINT_CONFIGURATION_KEYS.contains(key))
            {
                return Err(SessionCompileError::ReservedEndpointConfigurationKey {
                    endpoint_id: endpoint.id(),
                    key: key.to_owned(),
                });
            }
            let Some(registered_node_type_id) =
                self.endpoint_registry.node_type_id(endpoint.operator_id())
            else {
                return Err(SessionCompileError::UnknownOperator {
                    operator_id: endpoint.operator_id().as_str().to_owned(),
                });
            };
            if registered_node_type_id != endpoint.node_type_id() {
                return Err(SessionCompileError::OperatorNodeTypeMismatch {
                    operator_id: endpoint.operator_id().as_str().to_owned(),
                    registered_node_type_id: registered_node_type_id.as_str().to_owned(),
                    declared_node_type_id: endpoint.node_type_id().as_str().to_owned(),
                });
            }
        }
        Ok(())
    }

    fn validate_async_operators(&self, spec: &SessionSpec) -> Result<(), SessionCompileError> {
        for operator in spec.operators() {
            if let Some(key) = RESERVED_OPERATOR_CONFIGURATION_KEYS
                .iter()
                .find(|key| operator.configuration().get(key).is_some())
            {
                return Err(SessionCompileError::ReservedOperatorConfigurationKey {
                    operator_instance_id: operator.instance_id(),
                    key: (*key).to_owned(),
                });
            }
            if self
                .node_registry
                .async_factory_by_operator(operator.operator_id())
                .is_none()
            {
                return Err(SessionCompileError::UnknownAsyncOperator {
                    operator_id: operator.operator_id().as_str().to_owned(),
                });
            }
        }
        for operator in spec.operators() {
            let factory = self
                .node_registry
                .async_factory_by_operator(operator.operator_id())
                .ok_or_else(|| SessionCompileError::UnknownAsyncOperator {
                    operator_id: operator.operator_id().as_str().to_owned(),
                })?;
            let manifest = factory.manifest();
            let connections = spec
                .operator_connections()
                .iter()
                .filter(|connection| connection.operator_instance_id() == operator.instance_id())
                .collect::<Vec<_>>();
            let mut selected_inputs = HashMap::<String, usize>::new();
            for connection in connections {
                let input = select_operator_port(
                    manifest,
                    crate::graph::PortDirection::Input,
                    connection.input_port(),
                )?;
                let count = selected_inputs.entry(input.name.clone()).or_default();
                *count += 1;
                if *count > 1 && input.multiplicity == crate::graph::Multiplicity::One {
                    return Err(SessionCompileError::DuplicateOperatorInputConnection {
                        operator_instance_id: operator.instance_id(),
                        port_name: input.name.clone(),
                    });
                }
            }
            for input in manifest.input_ports().filter(|input| input.required) {
                if !selected_inputs.contains_key(&input.name) {
                    return Err(SessionCompileError::MissingRequiredOperatorInput {
                        operator_instance_id: operator.instance_id(),
                        port_name: input.name.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    fn lower_graph_spec(
        &self,
        spec: &SessionSpec,
    ) -> Result<crate::graph::GraphSpec, SessionCompileError> {
        let mut pipeline = Pipeline::new();
        let mut source_nodes = HashMap::with_capacity(spec.stems().len());
        let mut external_source_nodes = HashMap::with_capacity(spec.source_instances().len());
        let mut external_audio_ingress_nodes = HashMap::new();

        for stem in spec.stems() {
            let source_node = pipeline.add_node(
                source_node_type_id(stem.source()),
                source_node_config(spec.session_id(), stem),
            );
            source_nodes.insert(stem.id(), source_node);
        }

        for source in spec.source_instances() {
            let source_node = pipeline.add_node(
                NodeTypeId::from(source.source_type_id().as_str()),
                external_source_node_config(spec.session_id(), source),
            );
            external_source_nodes.insert(source.instance_id(), source_node.id());
            let manifest = self
                .source_registry
                .and_then(|registry| registry.manifest(source.source_type_id()))
                .ok_or_else(|| SessionCompileError::UnknownExternalSource {
                    source_type_id: source.source_type_id().clone(),
                })?;
            for output in spec
                .source_outputs()
                .iter()
                .filter(|output| output.source_instance_id() == source.instance_id())
            {
                let port = manifest.output_port(output.output_port()).ok_or_else(|| {
                    SessionCompileError::UnknownExternalSourceOutput {
                        source_type_id: source.source_type_id().clone(),
                        output_port: output.output_port().to_owned(),
                    }
                })?;
                if port.signal.class.is_audio() {
                    let ingress = pipeline.add_node(
                        NodeTypeId::from(EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID),
                        external_audio_ingress_node_config(spec.session_id(), source, output),
                    );
                    external_audio_ingress_nodes.insert(
                        (source.instance_id(), output.output_port().to_owned()),
                        ingress.id(),
                    );
                }
            }
        }

        for route in spec.routes() {
            let stem = spec
                .stems()
                .iter()
                .find(|stem| stem.id() == route.stem_id())
                .ok_or(SessionError::UnknownStem {
                    stem_id: route.stem_id(),
                })?;
            let endpoint = spec
                .endpoints()
                .iter()
                .find(|endpoint| endpoint.id() == route.endpoint_id())
                .ok_or(SessionError::UnknownEndpoint {
                    endpoint_id: route.endpoint_id(),
                })?;
            let source_node = source_nodes
                .get(&stem.id())
                .ok_or(SessionError::UnknownStem { stem_id: stem.id() })?;
            let endpoint_node = pipeline.add_node(
                endpoint.node_type_id().clone(),
                endpoint_node_config(spec.session_id(), stem, endpoint, route.id()),
            );
            if endpoint.operator_id().as_str() == RECORDER_OPERATOR_ID {
                let recording_media = self
                    .node_registry
                    .definition(endpoint.node_type_id())
                    .and_then(|definition| {
                        definition
                            .descriptor()
                            .inputs
                            .into_iter()
                            .find(|port| port.name == AUDIO_INPUT_PORT)
                    })
                    .map(|port| port.media);
                pipeline.connect_with(
                    source_node.out(AUDIO_OUTPUT_PORT),
                    endpoint_node.in_(AUDIO_INPUT_PORT),
                    recording_edge_contract(recording_media),
                );
            } else {
                pipeline.connect(
                    source_node.out(AUDIO_OUTPUT_PORT),
                    endpoint_node.in_(AUDIO_INPUT_PORT),
                );
            }
        }

        for route in spec.source_routes() {
            let source = spec
                .source_instances()
                .iter()
                .find(|source| source.instance_id() == route.source_instance_id())
                .ok_or(SessionError::UnknownSourceInstance {
                    source_instance_id: route.source_instance_id(),
                })?;
            let endpoint = spec
                .endpoints()
                .iter()
                .find(|endpoint| endpoint.id() == route.endpoint_id())
                .ok_or(SessionError::UnknownEndpoint {
                    endpoint_id: route.endpoint_id(),
                })?;
            let source_output = external_source_output_ref(
                source.instance_id(),
                route.output_port(),
                &external_source_nodes,
                &external_audio_ingress_nodes,
            )?;
            let endpoint_descriptor = self
                .node_registry
                .definition(endpoint.node_type_id())
                .ok_or_else(|| SessionCompileError::UnknownDerivedEndpointNodeType {
                    node_type_id: endpoint.node_type_id().as_str().to_owned(),
                })?
                .descriptor();
            let endpoint_input =
                select_endpoint_input(&endpoint_descriptor, route.endpoint_input_port())?;
            let endpoint_node = pipeline.add_node(
                endpoint.node_type_id().clone(),
                source_endpoint_node_config(
                    spec.session_id(),
                    SourceOriginMetadata::from(route),
                    endpoint,
                ),
            );
            if endpoint.operator_id().as_str() == RECORDER_OPERATOR_ID {
                pipeline.connect_with(
                    source_output,
                    endpoint_node.in_(&endpoint_input.name),
                    recording_edge_contract(Some(endpoint_input.media)),
                );
            } else {
                pipeline.connect(source_output, endpoint_node.in_(&endpoint_input.name));
            }
        }

        let mut operator_nodes: HashMap<OperatorInstanceId, LoweredOperator> =
            HashMap::with_capacity(spec.operators().len());
        for operator in spec.operators() {
            let factory = self
                .node_registry
                .async_factory_by_operator(operator.operator_id())
                .ok_or_else(|| SessionCompileError::UnknownAsyncOperator {
                    operator_id: operator.operator_id().as_str().to_owned(),
                })?;
            let manifest = factory.manifest();
            let connections = spec
                .operator_connections()
                .iter()
                .filter(|connection| connection.operator_instance_id() == operator.instance_id())
                .collect::<Vec<_>>();
            let operator_node = pipeline.add_node(
                manifest.node.type_id.clone(),
                operator_node_config(
                    spec.session_id(),
                    operator,
                    manifest,
                    (connections.len() == 1).then_some(connections[0]),
                ),
            );
            operator_nodes.insert(
                operator.instance_id(),
                LoweredOperator {
                    node: operator_node,
                    manifest: manifest.clone(),
                    source_stem_id: operator_source_stem_id(spec, operator.instance_id()),
                },
            );
        }

        for connection in spec.operator_connections() {
            let operator = operator_nodes
                .get(&connection.operator_instance_id())
                .ok_or(SessionError::UnknownOperatorInstance {
                    operator_instance_id: connection.operator_instance_id(),
                })?;
            let input_port = select_operator_port(
                &operator.manifest,
                crate::graph::PortDirection::Input,
                connection.input_port(),
            )?;
            let source_output = match connection.input_origin() {
                OperatorInputOrigin::Stem(stem_id) => source_nodes
                    .get(stem_id)
                    .ok_or(SessionError::UnknownStem { stem_id: *stem_id })?
                    .out(AUDIO_OUTPUT_PORT),
                OperatorInputOrigin::SourceOutput {
                    source_instance_id,
                    output_port,
                    ..
                } => external_source_output_ref(
                    *source_instance_id,
                    output_port,
                    &external_source_nodes,
                    &external_audio_ingress_nodes,
                )?,
                OperatorInputOrigin::OperatorOutput {
                    operator_instance_id,
                    output_port,
                } => {
                    let upstream = operator_nodes.get(operator_instance_id).ok_or(
                        SessionError::UnknownOperatorInstance {
                            operator_instance_id: *operator_instance_id,
                        },
                    )?;
                    let output = select_operator_port(
                        &upstream.manifest,
                        crate::graph::PortDirection::Output,
                        output_port.as_deref(),
                    )?;
                    upstream.node.out(&output.name)
                }
            };
            pipeline.connect_with(
                source_output,
                operator.node.in_(&input_port.name),
                operator.manifest.input_edge,
            );
        }

        for route in spec.derived_routes() {
            let operator = operator_nodes.get(&route.operator_instance_id()).ok_or(
                SessionError::UnknownOperatorInstance {
                    operator_instance_id: route.operator_instance_id(),
                },
            )?;
            let endpoint = spec
                .endpoints()
                .iter()
                .find(|endpoint| endpoint.id() == route.endpoint_id())
                .ok_or(SessionError::UnknownEndpoint {
                    endpoint_id: route.endpoint_id(),
                })?;
            let descriptor = self
                .node_registry
                .definition(endpoint.node_type_id())
                .ok_or_else(|| SessionCompileError::UnknownDerivedEndpointNodeType {
                    node_type_id: endpoint.node_type_id().as_str().to_owned(),
                })?
                .descriptor();
            let endpoint_input = select_endpoint_input(&descriptor, None)?;
            let endpoint_config = if let Some(stem_id) = operator.source_stem_id {
                let stem = spec
                    .stems()
                    .iter()
                    .find(|stem| stem.id() == stem_id)
                    .ok_or(SessionError::UnknownStem { stem_id })?;
                endpoint_node_config(spec.session_id(), stem, endpoint, route.id())
            } else if let Ok(source_origin) =
                source_origin_for_operator(spec, route.operator_instance_id())
            {
                source_endpoint_node_config(spec.session_id(), source_origin, endpoint)
            } else {
                derived_endpoint_node_config(
                    spec.session_id(),
                    route.operator_instance_id(),
                    endpoint,
                    route.id(),
                )
            };
            let endpoint_node = pipeline.add_node(endpoint.node_type_id().clone(), endpoint_config);
            pipeline.connect_with(
                operator.node.out(
                    &select_operator_port(
                        &operator.manifest,
                        crate::graph::PortDirection::Output,
                        route.output_port(),
                    )?
                    .name,
                ),
                endpoint_node.in_(&endpoint_input.name),
                operator.manifest.output_edge,
            );
        }
        Ok(pipeline.into_spec())
    }
}

fn external_source_output_ref(
    source_instance_id: SourceInstanceId,
    output_port: &str,
    source_nodes: &HashMap<SourceInstanceId, crate::graph::NodeId>,
    audio_ingress_nodes: &HashMap<(SourceInstanceId, String), crate::graph::NodeId>,
) -> Result<OutputPortRef, SessionCompileError> {
    if let Some(node) = audio_ingress_nodes.get(&(source_instance_id, output_port.to_owned())) {
        return Ok(OutputPortRef {
            node: *node,
            port: AUDIO_OUTPUT_PORT.to_owned(),
        });
    }
    let node = source_nodes
        .get(&source_instance_id)
        .copied()
        .ok_or(SessionError::UnknownSourceInstance { source_instance_id })?;
    Ok(OutputPortRef {
        node,
        port: output_port.to_owned(),
    })
}

fn recording_edge_contract(media: Option<MediaCaps>) -> EdgeContract {
    let mut contract = EdgeContract {
        latency_budget_ms: None,
        jitter_budget_ms: Some(RECORDING_SCHEDULER_HEADROOM_MS),
        backpressure: BackpressurePolicy::DropNewest,
        loss: LossPolicy::DropAllowed,
        copy_policy: CopyPolicy::CopyToBranchPool,
        observability: EdgeObservabilityLevel::Full,
        ..EdgeContract::voice_default()
    };
    if let Some(MediaCaps::Audio(mut audio)) = media {
        // Session structural ports carry the configured sample rate but accept
        // variable callback sizes. Recording is fed by the canonical 20 ms
        // normalizer, so make that duration explicit for queue planning while
        // retaining the Session's real rate and channel compatibility.
        if audio.frame_samples.is_none() {
            audio.frame_samples = audio
                .sample_rate_hz
                .map(|sample_rate_hz| (sample_rate_hz / 50) as usize);
        }
        contract.media = MediaCaps::Audio(audio);
    }
    contract
}

struct LoweredOperator {
    node: NodeHandle,
    manifest: crate::graph::AsyncOperatorManifest,
    source_stem_id: Option<crate::session::StemId>,
}

fn select_endpoint_input<'a>(
    descriptor: &'a crate::graph::NodeDescriptor,
    selected: Option<&str>,
) -> Result<&'a crate::graph::PortSpec, SessionCompileError> {
    let mut inputs = descriptor
        .inputs
        .iter()
        .filter(|port| port.direction == crate::graph::PortDirection::Input)
        .collect::<Vec<_>>();
    if let Some(selected) = selected {
        return inputs
            .into_iter()
            .find(|port| port.name == selected)
            .ok_or_else(|| SessionCompileError::UnknownEndpointInputPort {
                node_type_id: descriptor.type_id.as_str().to_owned(),
                port_name: selected.to_owned(),
            });
    }
    if inputs.len() != 1 {
        return Err(SessionCompileError::AmbiguousDerivedEndpointInput {
            node_type_id: descriptor.type_id.as_str().to_owned(),
            input_ports_total: inputs.len(),
        });
    }
    Ok(inputs.remove(0))
}

fn select_operator_port<'a>(
    manifest: &'a crate::graph::AsyncOperatorManifest,
    direction: crate::graph::PortDirection,
    selected: Option<&str>,
) -> Result<&'a crate::graph::PortSpec, SessionCompileError> {
    let mut ports = match direction {
        crate::graph::PortDirection::Input => manifest.input_ports().collect::<Vec<_>>(),
        crate::graph::PortDirection::Output => manifest.output_ports().collect::<Vec<_>>(),
    };
    let direction_name = match direction {
        crate::graph::PortDirection::Input => "input",
        crate::graph::PortDirection::Output => "output",
    };
    if let Some(selected) = selected {
        return ports
            .into_iter()
            .find(|port| port.name == selected)
            .ok_or_else(|| SessionCompileError::UnknownOperatorPort {
                operator_id: manifest.operator_id.as_str().to_owned(),
                direction: direction_name,
                port_name: selected.to_owned(),
            });
    }
    if ports.len() != 1 {
        return Err(SessionCompileError::AmbiguousOperatorPort {
            operator_id: manifest.operator_id.as_str().to_owned(),
            direction: direction_name,
        });
    }
    Ok(ports.remove(0))
}

fn source_node_type_id(source: &Source) -> NodeTypeId {
    match source {
        Source::Application(_) => NodeTypeId::from(APPLICATION_SOURCE_NODE_TYPE_ID),
        Source::Microphone(_) => NodeTypeId::from(MICROPHONE_SOURCE_NODE_TYPE_ID),
    }
}

fn source_node_config(session_id: SessionId, stem: &StemSpec) -> NodeConfig {
    let mut config = NodeConfig::new()
        .with("session_id", &session_id.0.to_string())
        .with("stem_id", &stem.id().0.to_string());
    match stem.source() {
        Source::Application(ApplicationSelector::BundleId(bundle_id)) => {
            config = config
                .with("selector_kind", "bundle_id")
                .with("selector_value", bundle_id);
        }
        Source::Application(ApplicationSelector::ProcessId(process_id)) => {
            config = config
                .with("selector_kind", "process_id")
                .with("selector_value", &process_id.get().to_string());
        }
        Source::Application(ApplicationSelector::ProcessInstance {
            process_id,
            stable_id,
        }) => {
            config = config
                .with("selector_kind", "process_instance")
                .with("selector_process_id", &process_id.get().to_string())
                .with("selector_stable_id", &stable_id.stable_key);
        }
        Source::Application(ApplicationSelector::StableId(source_id)) => {
            config = config
                .with("selector_kind", "stable_id")
                .with("selector_value", &source_id.stable_key);
        }
        Source::Application(ApplicationSelector::Name(name)) => {
            config = config
                .with("selector_kind", "name")
                .with("selector_value", name);
        }
        Source::Microphone(DeviceSelector::Default) => {
            config = config.with("selector_kind", "default");
        }
        Source::Microphone(DeviceSelector::Id(device_id)) => {
            config = config
                .with("selector_kind", "device_id")
                .with("selector_value", device_id.as_str());
        }
    }
    config
}

fn operator_node_config(
    session_id: SessionId,
    operator: &OperatorInstanceSpec,
    manifest: &crate::graph::AsyncOperatorManifest,
    sole_connection: Option<&OperatorConnectionSpec>,
) -> NodeConfig {
    let mut config = operator
        .configuration()
        .clone()
        .with("session_id", &session_id.0.to_string())
        .with(
            "operator_instance_id",
            &operator.instance_id().value().to_string(),
        )
        .with("operator_id", manifest.operator_id.as_str())
        .with("operator_revision", &manifest.revision.to_string())
        .with("operator_generation", &manifest.generation.to_string())
        .with("node_type_id", manifest.node.type_id.as_str());
    let Some(connection) = sole_connection else {
        return config;
    };
    config = config.with("input_route_id", &connection.route_id().0.to_string());
    match connection.input_origin() {
        OperatorInputOrigin::Stem(stem_id) => {
            config = config.with("stem_id", &stem_id.0.to_string());
        }
        OperatorInputOrigin::SourceOutput {
            source_instance_id,
            output_port,
            stream_id,
            source_id,
        } => {
            config = config
                .with(
                    "source_instance_id",
                    &source_instance_id.value().to_string(),
                )
                .with("source_id", &source_id.0.to_string())
                .with("stream_id", &stream_id.0.to_string())
                .with("source_output_port", output_port);
        }
        OperatorInputOrigin::OperatorOutput { .. } => {}
    }
    config
}

fn operator_source_stem_id(
    spec: &SessionSpec,
    operator_instance_id: OperatorInstanceId,
) -> Option<crate::session::StemId> {
    fn visit(
        spec: &SessionSpec,
        operator_instance_id: OperatorInstanceId,
        visiting: &mut Vec<OperatorInstanceId>,
        stems: &mut Vec<crate::session::StemId>,
    ) -> bool {
        if visiting.contains(&operator_instance_id) {
            return false;
        }
        visiting.push(operator_instance_id);
        let connections = spec
            .operator_connections()
            .iter()
            .filter(|connection| connection.operator_instance_id() == operator_instance_id)
            .collect::<Vec<_>>();
        if connections.is_empty() {
            return false;
        }
        for connection in connections {
            match connection.input_origin() {
                OperatorInputOrigin::Stem(stem_id) => stems.push(*stem_id),
                OperatorInputOrigin::OperatorOutput {
                    operator_instance_id,
                    ..
                } => {
                    if !visit(spec, *operator_instance_id, visiting, stems) {
                        return false;
                    }
                }
                OperatorInputOrigin::SourceOutput { .. } => return false,
            }
        }
        visiting.pop();
        true
    }

    let mut stems = Vec::new();
    if !visit(spec, operator_instance_id, &mut Vec::new(), &mut stems) {
        return None;
    }
    let first = *stems.first()?;
    stems.iter().all(|stem| *stem == first).then_some(first)
}

fn derived_endpoint_node_config(
    session_id: SessionId,
    operator_instance_id: OperatorInstanceId,
    endpoint: &EndpointSpec,
    route_id: crate::session::RouteId,
) -> NodeConfig {
    let mut config = NodeConfig::new()
        .with("session_id", &session_id.0.to_string())
        .with(
            "operator_instance_id",
            &operator_instance_id.value().to_string(),
        )
        .with("endpoint_id", &endpoint.id().0.to_string())
        .with("route_id", &route_id.0.to_string())
        .with("operator_id", endpoint.operator_id().as_str());
    if let Some(connector_id) = endpoint.connector_id() {
        config = config.with("connector_id", &connector_id.0.to_string());
    }
    for (key, value) in endpoint.configuration().iter() {
        config = config.with(key, value);
    }
    config
}

fn external_source_node_config(session_id: SessionId, source: &SourceInstanceSpec) -> NodeConfig {
    let manifest_prefix = crate::session::source_extension::SOURCE_CONFIGURATION_PREFIX;
    let mut config = NodeConfig::new()
        .with("session_id", &session_id.0.to_string())
        .with(
            "source_instance_id",
            &source.instance_id().value().to_string(),
        )
        .with("source_id", &source.source_id().0.to_string())
        .with("source_type_id", source.source_type_id().as_str());
    for (key, value) in source.configuration().iter() {
        config = config.with(&format!("{manifest_prefix}{key}"), value);
    }
    config
}

fn external_audio_ingress_node_config(
    session_id: SessionId,
    source: &SourceInstanceSpec,
    output: &crate::session::SourceOutputSpec,
) -> NodeConfig {
    NodeConfig::new()
        .with("session_id", &session_id.0.to_string())
        .with(
            "source_instance_id",
            &source.instance_id().value().to_string(),
        )
        .with("source_id", &source.source_id().0.to_string())
        .with("stream_id", &output.stream_id().0.to_string())
        .with("source_output_port", output.output_port())
}

#[derive(Clone)]
struct SourceOriginMetadata {
    route_id: crate::session::RouteId,
    source_instance_id: SourceInstanceId,
    output_port: String,
    stream_id: crate::frame::StreamId,
    source_id: crate::frame::SourceId,
}

impl From<&SourceRouteSpec> for SourceOriginMetadata {
    fn from(route: &SourceRouteSpec) -> Self {
        Self {
            route_id: route.id(),
            source_instance_id: route.source_instance_id(),
            output_port: route.output_port().to_owned(),
            stream_id: route.stream_id(),
            source_id: route.source_id(),
        }
    }
}

fn source_endpoint_node_config(
    session_id: SessionId,
    origin: SourceOriginMetadata,
    endpoint: &EndpointSpec,
) -> NodeConfig {
    let mut config = NodeConfig::new()
        .with("session_id", &session_id.0.to_string())
        .with(
            "source_instance_id",
            &origin.source_instance_id.value().to_string(),
        )
        .with("source_id", &origin.source_id.0.to_string())
        .with("stream_id", &origin.stream_id.0.to_string())
        .with("source_output_port", &origin.output_port)
        .with("endpoint_id", &endpoint.id().0.to_string())
        .with("route_id", &origin.route_id.0.to_string())
        .with("operator_id", endpoint.operator_id().as_str());
    if let Some(connector_id) = endpoint.connector_id() {
        config = config.with("connector_id", &connector_id.0.to_string());
    }
    for (key, value) in endpoint.configuration().iter() {
        config = config.with(key, value);
    }
    config
}

fn source_origin_for_operator(
    spec: &SessionSpec,
    operator_instance_id: OperatorInstanceId,
) -> Result<SourceOriginMetadata, SessionCompileError> {
    let mut current_instance_id = operator_instance_id;
    loop {
        let mut connections = spec
            .operator_connections()
            .iter()
            .filter(|connection| connection.operator_instance_id() == current_instance_id);
        let current = connections
            .next()
            .ok_or(SessionError::UnknownOperatorInstance {
                operator_instance_id: current_instance_id,
            })?;
        if connections.next().is_some() {
            return Err(SessionError::InvalidOperator {
                reason: format!(
                    "operator instance {current_instance_id:?} has multiple source origins"
                ),
            }
            .into());
        }
        match current.input_origin() {
            OperatorInputOrigin::SourceOutput {
                source_instance_id,
                output_port,
                stream_id,
                source_id,
            } => {
                return Ok(SourceOriginMetadata {
                    route_id: current.route_id(),
                    source_instance_id: *source_instance_id,
                    output_port: output_port.clone(),
                    stream_id: *stream_id,
                    source_id: *source_id,
                });
            }
            OperatorInputOrigin::OperatorOutput {
                operator_instance_id,
                ..
            } => {
                current_instance_id = *operator_instance_id;
            }
            OperatorInputOrigin::Stem(stem_id) => {
                return Err(SessionError::UnknownStem { stem_id: *stem_id }.into());
            }
        }
    }
}

pub(crate) fn endpoint_node_config(
    session_id: SessionId,
    stem: &StemSpec,
    endpoint: &EndpointSpec,
    route_id: crate::session::RouteId,
) -> NodeConfig {
    let mut config = NodeConfig::new()
        .with("session_id", &session_id.0.to_string())
        .with("stem_id", &stem.id().0.to_string())
        .with("endpoint_id", &endpoint.id().0.to_string())
        .with("route_id", &route_id.0.to_string())
        .with("operator_id", endpoint.operator_id().as_str());
    if let Some(connector_id) = endpoint.connector_id() {
        config = config.with("connector_id", &connector_id.0.to_string());
    }
    for (key, value) in endpoint.configuration().iter() {
        config = config.with(key, value);
    }
    config
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::endpoint::{
        EndpointDriverFactory, EndpointDriverInput, EndpointDriverRegistry, EndpointFailure,
        EndpointFailureStage, PreparedEndpointDriver,
    };
    use crate::frame::{SampleFormat, SampleSpec};
    use crate::graph::{
        AsyncNode, AsyncOperatorFactory, AsyncOperatorManifest, AudioCaps, BackpressurePolicy,
        ChannelLayout, ConfigError, ExecutionPartition, MediaCaps, Multiplicity, NodeDefinition,
        NodeDescriptor, NodeError, OperatorCancellationPolicy, OperatorDeadlinePolicy,
        OperatorFailurePolicy, OperatorOutputRolePolicy, OperatorPermissionPolicy, PortDirection,
        PortSpec, SafetyContract, SemanticRole, SignalSpec, TextFormat,
    };

    use super::*;
    use crate::session::{
        register_session_structural_nodes, EndpointConfiguration, EndpointDescriptor, Operator,
        OperatorConfiguration, OperatorId, Session,
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
            let mut input_edge = EdgeContract::voice_default();
            input_edge.copy_policy = crate::graph::CopyPolicy::CopyToBranchPool;
            let mut output_edge = EdgeContract::typed_default();
            output_edge.media = MediaCaps::Text;
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
                        safety: SafetyContract::ExternalService,
                        stateful: true,
                    },
                    input_edge,
                    output_edge,
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
            factory.manifest.input_edge.media = MediaCaps::Text;
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
                safety: SafetyContract::ExternalService,
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
            _inputs: Vec<EndpointDriverInput>,
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

    fn registries() -> (NodeRegistry, EndpointDriverRegistry) {
        let mut node_registry = NodeRegistry::new();
        register_session_structural_nodes(&mut node_registry)
            .expect("Session structural registrations");

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
            let contract = edge.spec.requested.expect("explicit recorder contract");
            assert_eq!(
                contract.jitter_budget_ms,
                Some(RECORDING_SCHEDULER_HEADROOM_MS)
            );
            assert_eq!(contract.backpressure, BackpressurePolicy::DropNewest);
            assert_eq!(contract.loss, LossPolicy::DropAllowed);
            assert_eq!(contract.copy_policy, CopyPolicy::CopyToBranchPool);
            assert_eq!(contract.observability, EdgeObservabilityLevel::Full);
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
    fn given_registered_operator_when_compiled_then_manifest_types_and_edge_policies_are_authority()
    {
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
                .contract
                .expect("operator input contract")
                .backpressure,
            BackpressurePolicy::DropNewest
        );
        assert_eq!(compiled.graph_ir.edges[1].media, MediaCaps::Text);
        let output_contract = compiled.graph_ir.edges[1]
            .contract
            .expect("operator output contract");
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
        assert_eq!(
            operator_node.spec.config.get("operator_id"),
            Some(TEST_ASYNC_OPERATOR_ID)
        );
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
        let prepare_context = crate::graph::PrepareContext::new(SampleSpec::new(
            48_000,
            1,
            SampleFormat::F32Interleaved,
        ));

        let prepared =
            crate::session::prepare_session_runtime(compiled, &node_registry, &prepare_context, 8)
                .expect("prepared derived Session");

        assert!(prepared.worker_mappings().is_empty());
        assert_eq!(prepared.operator_mappings().len(), 1);
        assert_eq!(prepared.operator_mappings()[0].derived_routes().len(), 1);
        assert_eq!(
            prepared.operator_mappings()[0].derived_routes()[0]
                .output_branch
                .edge_contract
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
        let prepare_context = crate::graph::PrepareContext::new(SampleSpec::new(
            48_000,
            1,
            SampleFormat::F32Interleaved,
        ));

        let prepared =
            crate::session::prepare_session_runtime(compiled, &node_registry, &prepare_context, 8)
                .expect("prepared two-destination Session");

        let routes = prepared.operator_mappings()[0].derived_routes();
        assert_eq!(routes.len(), 2);
        assert_ne!(routes[0].route_id(), routes[1].route_id());
        assert_ne!(routes[0].endpoint_id(), routes[1].endpoint_id());
        assert!(routes.iter().all(|route| {
            route.output_branch.edge_contract.media == MediaCaps::Text
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
    fn given_reserved_endpoint_key_when_compiled_then_typed_error_is_returned() {
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

        let result = SessionCompiler::new(&node_registry, &endpoint_registry).compile(spec);

        assert!(matches!(
            result,
            Err(SessionCompileError::ReservedEndpointConfigurationKey { .. })
        ));
    }

    #[test]
    fn given_exact_process_instance_when_lowered_then_pid_and_stable_id_are_preserved() {
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
        let config = source_node_config(spec.session_id(), &spec.stems()[0]);

        assert_eq!(config.get("selector_kind"), Some("process_instance"));
        assert_eq!(config.get("selector_process_id"), Some("42"));
        assert_eq!(
            config.get("selector_stable_id"),
            Some("wasapi:pid:42:creation-100ns:133801234567890000")
        );
        assert_eq!(config.get("selector_value"), None);
    }
}

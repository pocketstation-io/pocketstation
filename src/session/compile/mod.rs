use std::collections::HashMap;
use std::sync::Arc;

use crate::endpoint::EndpointDriverRegistry;
use crate::graph::compile::{Compiler, RuntimePlanner};
use crate::graph::{
    EdgeContract, MediaCaps, NodeConfig, NodeHandle, NodeId, NodeRegistry, NodeTypeId,
    OutputPortRef, Pipeline,
};

use crate::session::{
    ConnectionSpec, ConnectionTarget, EndpointSpec, OperatorInstanceId, SessionError, SessionSpec,
    SourceInstanceId, SourceRegistry, StreamOrigin,
};

mod bindings;
mod compiled;
mod error;
#[cfg(any(test, feature = "internal-testing"))]
use crate::session::extensions::builtins::default_session_graph_lowerers;
pub(crate) use bindings::{CompiledNodeBinding, CompiledSessionBindings};
pub use compiled::CompiledSession;
pub use error::{SessionCompileDiagnostic, SessionCompileError};

const AUDIO_OUTPUT_PORT: &str = "audio";
const AUDIO_INPUT_PORT: &str = "audio";

pub(crate) type LoweredStemNodes = HashMap<crate::session::StemId, NodeHandle>;
pub(crate) type LoweredExternalSourceNodes = HashMap<SourceInstanceId, NodeId>;
pub(crate) type LoweredExternalAudioIngressNodes = HashMap<(SourceInstanceId, String), NodeId>;

pub(crate) struct SessionSourceLoweringContext<'lowering> {
    pub(crate) source_registry: Option<&'lowering SourceRegistry>,
    pub(crate) pipeline: &'lowering mut Pipeline,
    pub(crate) source_nodes: &'lowering mut LoweredStemNodes,
    pub(crate) external_source_nodes: &'lowering mut LoweredExternalSourceNodes,
    pub(crate) external_audio_ingress_nodes: &'lowering mut LoweredExternalAudioIngressNodes,
    pub(crate) bindings: &'lowering mut CompiledSessionBindings,
}

pub struct SessionCompiler<'registry> {
    node_registry: &'registry NodeRegistry,
    endpoint_registry: &'registry EndpointDriverRegistry,
    source_registry: Option<&'registry SourceRegistry>,
    graph_lowerers: Vec<Arc<dyn SessionGraphLowerer>>,
}

/// Registration-owned lowering seam for Session components that expand into
/// graph nodes. The generic compiler coordinates stable origins, named ports,
/// and connections; component packages own their node types and metadata.
pub(crate) trait SessionGraphLowerer: Send + Sync {
    fn lower_source_nodes(
        &self,
        spec: &SessionSpec,
        context: &mut SessionSourceLoweringContext<'_>,
    ) -> Result<(), SessionCompileError>;

    fn lower_operator_edges(
        &self,
        spec: &SessionSpec,
        pipeline: &mut Pipeline,
        operator_nodes: &HashMap<OperatorInstanceId, LoweredOperator>,
        bindings: &mut CompiledSessionBindings,
    ) -> Result<(), SessionCompileError>;

    fn endpoint_config(
        &self,
        spec: &SessionSpec,
        stem_id: crate::session::StemId,
        endpoint: &EndpointSpec,
        route_id: crate::session::RouteId,
    ) -> Result<Option<NodeConfig>, SessionCompileError>;
}

impl<'registry> SessionCompiler<'registry> {
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn new(
        node_registry: &'registry NodeRegistry,
        endpoint_registry: &'registry EndpointDriverRegistry,
    ) -> Self {
        Self {
            node_registry,
            endpoint_registry,
            source_registry: None,
            graph_lowerers: default_session_graph_lowerers(),
        }
    }

    pub(crate) fn with_sources(
        node_registry: &'registry NodeRegistry,
        endpoint_registry: &'registry EndpointDriverRegistry,
        source_registry: &'registry SourceRegistry,
        graph_lowerers: &'registry [Arc<dyn SessionGraphLowerer>],
    ) -> Self {
        Self {
            node_registry,
            endpoint_registry,
            source_registry: Some(source_registry),
            graph_lowerers: graph_lowerers.to_vec(),
        }
    }

    pub fn compile(&self, spec: SessionSpec) -> Result<CompiledSession, SessionCompileError> {
        spec.validate()?;
        self.validate_external_sources(&spec)?;
        self.validate_endpoint_operators(&spec)?;
        self.validate_async_operators(&spec)?;

        let (graph_spec, bindings) = self.lower_graph_spec(&spec)?;
        let graph_ir = Compiler::new().compile(graph_spec, self.node_registry)?;
        let runtime_plan = RuntimePlanner::new().plan(&graph_ir)?;

        Ok(CompiledSession {
            spec,
            graph_ir,
            runtime_plan,
            bindings,
        })
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
            let connections =
                operator_input_connections(spec, operator.instance_id()).collect::<Vec<_>>();
            let mut selected_inputs = HashMap::<String, usize>::new();
            for connection in connections {
                let ConnectionTarget::OperatorInput { input_port, .. } = connection.target() else {
                    continue;
                };
                let input = select_operator_port(
                    manifest,
                    crate::graph::PortDirection::Input,
                    input_port.as_deref(),
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
    ) -> Result<(crate::graph::GraphSpec, CompiledSessionBindings), SessionCompileError> {
        let mut pipeline = Pipeline::new();
        let mut bindings = CompiledSessionBindings::new();
        let mut source_nodes = HashMap::with_capacity(spec.stems().len());
        let mut external_source_nodes = HashMap::with_capacity(spec.source_instances().len());
        let mut external_audio_ingress_nodes = HashMap::new();

        for lowerer in &self.graph_lowerers {
            lowerer.lower_source_nodes(
                spec,
                &mut SessionSourceLoweringContext {
                    source_registry: self.source_registry,
                    pipeline: &mut pipeline,
                    source_nodes: &mut source_nodes,
                    external_source_nodes: &mut external_source_nodes,
                    external_audio_ingress_nodes: &mut external_audio_ingress_nodes,
                    bindings: &mut bindings,
                },
            )?;
        }

        for connection in spec.connections() {
            let StreamOrigin::Stem(stem_id) = connection.origin() else {
                continue;
            };
            let ConnectionTarget::EndpointInput {
                endpoint_id,
                input_port,
            } = connection.target()
            else {
                continue;
            };
            let stem = spec.stems().iter().find(|stem| stem.id() == *stem_id);
            if stem.is_none() && !source_nodes.contains_key(stem_id) {
                return Err(SessionError::UnknownStem { stem_id: *stem_id }.into());
            }
            let endpoint = spec
                .endpoints()
                .iter()
                .find(|endpoint| endpoint.id() == *endpoint_id)
                .ok_or(SessionError::UnknownEndpoint {
                    endpoint_id: *endpoint_id,
                })?;
            let source_node = source_nodes
                .get(stem_id)
                .ok_or(SessionError::UnknownStem { stem_id: *stem_id })?;
            let endpoint_config =
                self.endpoint_config_for_stem(spec, *stem_id, endpoint, connection.id())?;
            let endpoint_node = pipeline.add_node(endpoint.node_type_id().clone(), endpoint_config);
            bindings.insert_node(
                endpoint_node.id(),
                CompiledNodeBinding::Endpoint {
                    route_id: connection.id(),
                    endpoint_id: *endpoint_id,
                    connector_id: endpoint.connector_id(),
                    origin: connection.origin().clone(),
                },
            );
            let endpoint_descriptor = self
                .node_registry
                .definition(endpoint.node_type_id())
                .ok_or_else(|| SessionCompileError::UnknownEndpointNodeType {
                    node_type_id: endpoint.node_type_id().as_str().to_owned(),
                })?
                .descriptor();
            let endpoint_input =
                select_endpoint_input(&endpoint_descriptor, input_port.as_deref())?;
            if let Some(input_edge) = endpoint.input_edge() {
                pipeline.connect_with(
                    source_node.out(AUDIO_OUTPUT_PORT),
                    endpoint_node.in_(&endpoint_input.name),
                    specialize_audio_edge(input_edge, self.node_registry, endpoint)?,
                );
            } else {
                pipeline.connect(
                    source_node.out(AUDIO_OUTPUT_PORT),
                    endpoint_node.in_(&endpoint_input.name),
                );
            }
        }

        for connection in spec.connections() {
            let StreamOrigin::SourceOutput {
                source_instance_id,
                output_port,
                ..
            } = connection.origin()
            else {
                continue;
            };
            let ConnectionTarget::EndpointInput {
                endpoint_id,
                input_port,
            } = connection.target()
            else {
                continue;
            };
            let source = spec
                .source_instances()
                .iter()
                .find(|source| source.instance_id() == *source_instance_id)
                .ok_or(SessionError::UnknownSourceInstance {
                    source_instance_id: *source_instance_id,
                })?;
            let endpoint = spec
                .endpoints()
                .iter()
                .find(|endpoint| endpoint.id() == *endpoint_id)
                .ok_or(SessionError::UnknownEndpoint {
                    endpoint_id: *endpoint_id,
                })?;
            let source_output = external_source_output_ref(
                source.instance_id(),
                output_port,
                &external_source_nodes,
                &external_audio_ingress_nodes,
            )?;
            let endpoint_descriptor = self
                .node_registry
                .definition(endpoint.node_type_id())
                .ok_or_else(|| SessionCompileError::UnknownEndpointNodeType {
                    node_type_id: endpoint.node_type_id().as_str().to_owned(),
                })?
                .descriptor();
            let endpoint_input =
                select_endpoint_input(&endpoint_descriptor, input_port.as_deref())?;
            let endpoint_node = pipeline.add_node(
                endpoint.node_type_id().clone(),
                endpoint_node_config(endpoint),
            );
            bindings.insert_node(
                endpoint_node.id(),
                CompiledNodeBinding::Endpoint {
                    route_id: connection.id(),
                    endpoint_id: *endpoint_id,
                    connector_id: endpoint.connector_id(),
                    origin: connection.origin().clone(),
                },
            );
            if let Some(input_edge) = endpoint.input_edge() {
                pipeline.connect_with(
                    source_output,
                    endpoint_node.in_(&endpoint_input.name),
                    specialize_edge_media(input_edge, endpoint_input.media),
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
            let operator_node = pipeline.add_node(
                manifest.node.type_id.clone(),
                operator.configuration().clone(),
            );
            bindings.insert_node(
                operator_node.id(),
                CompiledNodeBinding::Operator {
                    operator_instance_id: operator.instance_id(),
                },
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

        for connection in spec.connections() {
            let ConnectionTarget::OperatorInput {
                operator_instance_id,
                input_port: selected_input_port,
            } = connection.target()
            else {
                continue;
            };
            let operator = operator_nodes.get(operator_instance_id).ok_or(
                SessionError::UnknownOperatorInstance {
                    operator_instance_id: *operator_instance_id,
                },
            )?;
            let input_port = select_operator_port(
                &operator.manifest,
                crate::graph::PortDirection::Input,
                selected_input_port.as_deref(),
            )?;
            let source_output = match connection.origin() {
                StreamOrigin::Stem(stem_id) => source_nodes
                    .get(stem_id)
                    .ok_or(SessionError::UnknownStem { stem_id: *stem_id })?
                    .out(AUDIO_OUTPUT_PORT),
                StreamOrigin::SourceOutput {
                    source_instance_id,
                    output_port,
                    ..
                } => external_source_output_ref(
                    *source_instance_id,
                    output_port,
                    &external_source_nodes,
                    &external_audio_ingress_nodes,
                )?,
                StreamOrigin::OperatorOutput {
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

        for lowerer in &self.graph_lowerers {
            lowerer.lower_operator_edges(spec, &mut pipeline, &operator_nodes, &mut bindings)?;
        }

        for connection in spec.connections() {
            let StreamOrigin::OperatorOutput {
                operator_instance_id,
                output_port,
            } = connection.origin()
            else {
                continue;
            };
            let ConnectionTarget::EndpointInput {
                endpoint_id,
                input_port,
            } = connection.target()
            else {
                continue;
            };
            let operator = operator_nodes.get(operator_instance_id).ok_or(
                SessionError::UnknownOperatorInstance {
                    operator_instance_id: *operator_instance_id,
                },
            )?;
            let endpoint = spec
                .endpoints()
                .iter()
                .find(|endpoint| endpoint.id() == *endpoint_id)
                .ok_or(SessionError::UnknownEndpoint {
                    endpoint_id: *endpoint_id,
                })?;
            let descriptor = self
                .node_registry
                .definition(endpoint.node_type_id())
                .ok_or_else(|| SessionCompileError::UnknownEndpointNodeType {
                    node_type_id: endpoint.node_type_id().as_str().to_owned(),
                })?
                .descriptor();
            let endpoint_input = select_endpoint_input(&descriptor, input_port.as_deref())?;
            let endpoint_config = if let Some(stem_id) = operator.source_stem_id {
                self.endpoint_config_for_stem(spec, stem_id, endpoint, connection.id())?
            } else {
                endpoint_node_config(endpoint)
            };
            let endpoint_node = pipeline.add_node(endpoint.node_type_id().clone(), endpoint_config);
            bindings.insert_node(
                endpoint_node.id(),
                CompiledNodeBinding::Endpoint {
                    route_id: connection.id(),
                    endpoint_id: *endpoint_id,
                    connector_id: endpoint.connector_id(),
                    origin: operator_lineage_origin(spec, *operator_instance_id)
                        .unwrap_or_else(|| connection.origin().clone()),
                },
            );
            pipeline.connect_with(
                operator.node.out(
                    &select_operator_port(
                        &operator.manifest,
                        crate::graph::PortDirection::Output,
                        output_port.as_deref(),
                    )?
                    .name,
                ),
                endpoint_node.in_(&endpoint_input.name),
                operator.manifest.output_edge,
            );
        }
        Ok((pipeline.into_spec(), bindings))
    }

    fn endpoint_config_for_stem(
        &self,
        spec: &SessionSpec,
        stem_id: crate::session::StemId,
        endpoint: &EndpointSpec,
        route_id: crate::session::RouteId,
    ) -> Result<NodeConfig, SessionCompileError> {
        if spec.stems().iter().any(|stem| stem.id() == stem_id) {
            return Ok(endpoint_node_config(endpoint));
        }
        for lowerer in &self.graph_lowerers {
            if let Some(config) = lowerer.endpoint_config(spec, stem_id, endpoint, route_id)? {
                return Ok(config);
            }
        }
        Err(SessionError::UnknownStem { stem_id }.into())
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

fn specialize_audio_edge(
    input_edge: EdgeContract,
    node_registry: &NodeRegistry,
    endpoint: &EndpointSpec,
) -> Result<EdgeContract, SessionCompileError> {
    let media = node_registry
        .definition(endpoint.node_type_id())
        .and_then(|definition| {
            definition
                .descriptor()
                .inputs
                .into_iter()
                .find(|port| port.name == AUDIO_INPUT_PORT)
        })
        .map_or(input_edge.media, |port| port.media);
    Ok(specialize_edge_media(input_edge, media))
}

fn specialize_edge_media(mut input_edge: EdgeContract, media: MediaCaps) -> EdgeContract {
    input_edge.media = media;
    input_edge
}

pub(crate) struct LoweredOperator {
    pub(crate) node: NodeHandle,
    pub(crate) manifest: crate::graph::AsyncOperatorManifest,
    source_stem_id: Option<crate::session::StemId>,
}

fn operator_input_connections(
    spec: &SessionSpec,
    operator_instance_id: OperatorInstanceId,
) -> impl Iterator<Item = &ConnectionSpec> {
    spec.connections().iter().filter(move |connection| {
        matches!(
            connection.target(),
            ConnectionTarget::OperatorInput {
                operator_instance_id: target_instance_id,
                ..
            } if *target_instance_id == operator_instance_id
        )
    })
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
        return Err(SessionCompileError::AmbiguousEndpointInput {
            node_type_id: descriptor.type_id.as_str().to_owned(),
            input_ports_total: inputs.len(),
        });
    }
    Ok(inputs.remove(0))
}

pub(crate) fn select_operator_port<'a>(
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
        let connections =
            operator_input_connections(spec, operator_instance_id).collect::<Vec<_>>();
        if connections.is_empty() {
            return false;
        }
        for connection in connections {
            match connection.origin() {
                StreamOrigin::Stem(stem_id) => stems.push(*stem_id),
                StreamOrigin::OperatorOutput {
                    operator_instance_id,
                    ..
                } => {
                    if !visit(spec, *operator_instance_id, visiting, stems) {
                        return false;
                    }
                }
                StreamOrigin::SourceOutput { .. } => return false,
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

fn operator_lineage_origin(
    spec: &SessionSpec,
    operator_instance_id: OperatorInstanceId,
) -> Option<StreamOrigin> {
    if let Some(stem_id) = operator_source_stem_id(spec, operator_instance_id) {
        return Some(StreamOrigin::Stem(stem_id));
    }
    let mut current_instance_id = operator_instance_id;
    let mut visiting = Vec::new();
    loop {
        if visiting.contains(&current_instance_id) {
            return None;
        }
        visiting.push(current_instance_id);
        let mut connections = operator_input_connections(spec, current_instance_id);
        let current = connections.next()?;
        if connections.next().is_some() {
            return None;
        }
        match current.origin() {
            StreamOrigin::SourceOutput { .. } => return Some(current.origin().clone()),
            StreamOrigin::OperatorOutput {
                operator_instance_id,
                ..
            } => current_instance_id = *operator_instance_id,
            StreamOrigin::Stem(stem_id) => return Some(StreamOrigin::Stem(*stem_id)),
        }
    }
}

pub(crate) fn endpoint_node_config(endpoint: &EndpointSpec) -> NodeConfig {
    let mut config = NodeConfig::new();
    for (key, value) in endpoint.configuration().iter() {
        config = if endpoint.configuration().is_sensitive(key) {
            config.with_sensitive(key, value)
        } else {
            config.with(key, value)
        };
    }
    config
}

#[cfg(test)]
mod tests;

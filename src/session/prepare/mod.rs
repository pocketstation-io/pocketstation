use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::frame::StemId;
use crate::frame::{SampleFormat, SampleSpec};
use crate::graph::ir::GraphIr;
use crate::graph::{ChannelLayout, MediaCaps};
use crate::graph::{EdgeContract, NodeId, NodeRegistry, NodeTypeId, PrepareContext};
use crate::runtime::{
    plan_source_channel, AsyncOperatorOutputBranchSpec, PlanEdgeReceiver, PlanRunnerCancellation,
    PlanSourceInput, RealtimePlanExecutor, TypedEdgeBranchSpec,
};

use crate::session::compile::{CompiledNodeBinding, CompiledSessionBindings};
use crate::session::{CompiledSession, ConnectionTarget, SessionSpec, StreamOrigin};

mod error;
mod mappings;
mod prepared;
pub use error::SessionPrepareError;
pub(crate) use mappings::{
    PreparedExternalAudioIngress, PreparedExternalOperatorInput, PreparedExternalSourceBranch,
    PreparedExternalSourceMapping, PreparedExternalSourceTarget, PreparedExternalTypedRoute,
    PreparedGeneratedAudioIngress, PreparedOperatorInputMapping, PreparedOperatorOutputMapping,
    PreparedOperatorOutputTarget, PreparedTypedInputOrigin, PreparedWorkerOrigin,
};
pub use mappings::{
    PreparedOperatorMapping, PreparedSignalRouteMapping, PreparedSourceMapping,
    PreparedWorkerMapping,
};
pub use prepared::PreparedSession;

pub fn prepare_session_runtime(
    compiled: CompiledSession,
    node_registry: &NodeRegistry,
    prepare_context: &PrepareContext,
    source_queue_capacity_frames: usize,
) -> Result<PreparedSession, SessionPrepareError> {
    let (spec, graph_ir, runtime_plan, bindings) = compiled.into_runtime_parts();
    let (executor, worker_receivers) =
        RealtimePlanExecutor::new(&runtime_plan, &graph_ir, node_registry, prepare_context)?;
    let cancellation = PlanRunnerCancellation::new();
    let (source_mappings, mut source_inputs) = prepare_sources(
        &spec,
        &graph_ir,
        &bindings,
        source_queue_capacity_frames,
        &cancellation,
    )?;
    let external_source_mappings = prepare_external_sources(
        &spec,
        &graph_ir,
        &runtime_plan,
        &bindings,
        node_registry,
        prepare_context,
        source_queue_capacity_frames,
        &cancellation,
        &mut source_inputs,
    )?;
    let mut generated_audio_ingresses = prepare_generated_audio_ingresses(
        &spec,
        &graph_ir,
        &bindings,
        source_queue_capacity_frames,
        &cancellation,
        &mut source_inputs,
    )?;
    let (worker_mappings, operator_mappings) = map_worker_receivers(
        WorkerMappingContext {
            spec: &spec,
            graph_ir: &graph_ir,
            runtime_plan: &runtime_plan,
            bindings: &bindings,
            node_registry,
        },
        worker_receivers,
        &mut generated_audio_ingresses,
    )?;
    debug_assert!(generated_audio_ingresses.is_empty());

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

fn prepare_generated_audio_ingresses(
    spec: &SessionSpec,
    graph_ir: &GraphIr,
    bindings: &CompiledSessionBindings,
    source_queue_capacity_frames: usize,
    cancellation: &PlanRunnerCancellation,
    source_inputs: &mut Vec<PlanSourceInput>,
) -> Result<Vec<PreparedGeneratedAudioIngress>, SessionPrepareError> {
    let mut prepared = Vec::with_capacity(spec.generated_audio_ingresses().len());
    for ingress in spec.generated_audio_ingresses() {
        let source_node = graph_ir
            .nodes
            .iter()
            .find(|node| {
                matches!(
                    bindings.node(node.id()),
                    Some(CompiledNodeBinding::GeneratedAudioIngress { stem_id })
                        if *stem_id == ingress.stem_id()
                )
            })
            .ok_or(SessionPrepareError::MissingGeneratedAudioIngress {
                stem_id: ingress.stem_id(),
            })?;
        let bridge = graph_ir
            .nodes
            .iter()
            .find(|node| {
                matches!(
                    bindings.node(node.id()),
                    Some(CompiledNodeBinding::GeneratedAudioBridge { stem_id, .. })
                        if *stem_id == ingress.stem_id()
                )
            })
            .ok_or(SessionPrepareError::MissingGeneratedAudioBridge {
                stem_id: ingress.stem_id(),
            })?;
        let bridge_edge = graph_ir
            .edges
            .iter()
            .find(|edge| edge.spec.to.node == bridge.id())
            .ok_or(SessionPrepareError::MissingGeneratedAudioBridge {
                stem_id: ingress.stem_id(),
            })?;
        let MediaCaps::Audio(audio) = bridge_edge.media else {
            return Err(SessionPrepareError::InvalidGeneratedAudioMedia {
                stem_id: ingress.stem_id(),
            });
        };
        let channels = match audio.channel_layout {
            ChannelLayout::Mono => 1,
            ChannelLayout::Stereo => 2,
            ChannelLayout::Any => {
                return Err(SessionPrepareError::InvalidGeneratedAudioMedia {
                    stem_id: ingress.stem_id(),
                });
            }
        };
        let (Some(sample_rate_hz), Some(frame_samples)) =
            (audio.sample_rate_hz, audio.frame_samples)
        else {
            return Err(SessionPrepareError::InvalidGeneratedAudioMedia {
                stem_id: ingress.stem_id(),
            });
        };
        let sample_spec = SampleSpec::new(sample_rate_hz, channels, audio.format);
        let (sender, input) = plan_source_channel(
            source_node.id(),
            source_queue_capacity_frames,
            cancellation.clone(),
        )?;
        source_inputs.push(input);
        prepared.push(PreparedGeneratedAudioIngress {
            stem_id: ingress.stem_id(),
            stream_id: ingress.stream_id(),
            source_id: ingress.source_id(),
            sample_spec,
            samples_per_frame: frame_samples.saturating_mul(usize::from(channels)),
            sender,
        });
    }
    Ok(prepared)
}

#[allow(clippy::too_many_arguments)]
fn prepare_external_sources(
    spec: &SessionSpec,
    graph_ir: &GraphIr,
    runtime_plan: &crate::graph::plan::RuntimePlan,
    bindings: &CompiledSessionBindings,
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
            .ok_or_else(|| SessionPrepareError::MissingExternalSourceDefinition {
                source_type_id: source.source_type_id().clone(),
            })?;
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
                        matches!(
                            bindings.node(node.id()),
                            Some(CompiledNodeBinding::ExternalAudioIngress {
                                source_instance_id,
                                output_port,
                            }) if *source_instance_id == source.instance_id()
                                && output_port == output.output_port()
                        )
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
                let mut edge_contract = EdgeContract::realtime_audio();
                edge_contract.media = port.media;
                edge_contract = edge_contract.with_max_payload_bytes(
                    frame_samples
                        .saturating_mul(usize::from(channels))
                        .saturating_mul(std::mem::size_of::<f32>()),
                );
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

            for connection in spec.connections().iter().filter(|connection| {
                matches!(
                    connection.origin(),
                    StreamOrigin::SourceOutput {
                        source_instance_id,
                        output_port,
                        ..
                    } if *source_instance_id == source.instance_id()
                        && output_port == output.output_port()
                ) && matches!(connection.target(), ConnectionTarget::EndpointInput { .. })
            }) {
                let ConnectionTarget::EndpointInput { endpoint_id, .. } = connection.target()
                else {
                    continue;
                };
                let StreamOrigin::SourceOutput {
                    stream_id,
                    source_id,
                    ..
                } = connection.origin()
                else {
                    continue;
                };
                let edge = graph_ir
                    .edges
                    .iter()
                    .find(|edge| {
                        graph_ir.node(edge.spec.to.node).is_some_and(|target| {
                            matches!(
                                bindings.node(target.id()),
                                Some(CompiledNodeBinding::Endpoint { route_id, .. })
                                    if *route_id == connection.id()
                            )
                        })
                    })
                    .ok_or(SessionPrepareError::MissingExternalSourceRouteEdge {
                        route_id: connection.id(),
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
                    .find(|endpoint| endpoint.id() == *endpoint_id)
                    .ok_or(SessionPrepareError::UnknownWorkerRoute {
                        edge_id: edge.spec.id,
                        route_id: connection.id(),
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
                            route_id: connection.id(),
                            endpoint_id: *endpoint_id,
                            connector_id: endpoint.connector_id(),
                            endpoint_operator_id: endpoint.operator_id().clone(),
                            endpoint_node_type_id: endpoint.node_type_id().clone(),
                            stream_id: *stream_id,
                            source_id: *source_id,
                            node_configuration: target.spec.config.clone(),
                            input_port: edge.spec.to.port.clone(),
                            signal_spec: typed_edge.signal.clone(),
                            media: typed_edge.media,
                            edge_contract: typed_edge.contract,
                        },
                    ),
                });
            }
            for connection in spec.connections().iter().filter(|connection| {
                matches!(
                    connection.origin(),
                    StreamOrigin::SourceOutput {
                        source_instance_id,
                        output_port,
                        ..
                    } if *source_instance_id == source.instance_id()
                        && output_port == output.output_port()
                ) && matches!(connection.target(), ConnectionTarget::OperatorInput { .. })
            }) {
                let ConnectionTarget::OperatorInput {
                    operator_instance_id,
                    input_port,
                } = connection.target()
                else {
                    continue;
                };
                let edge = graph_ir
                    .edges
                    .iter()
                    .find(|edge| {
                        graph_ir.node(edge.spec.from.node).is_some_and(|origin| {
                            matches!(
                                bindings.node(origin.id()),
                                Some(CompiledNodeBinding::ExternalSource {
                                    source_instance_id,
                                }) if *source_instance_id == source.instance_id()
                            ) && edge.spec.from.port == output.output_port()
                        }) && graph_ir.node(edge.spec.to.node).is_some_and(|target| {
                            matches!(
                                bindings.node(target.id()),
                                Some(CompiledNodeBinding::Operator {
                                    operator_instance_id: target_instance_id,
                                }) if *target_instance_id == *operator_instance_id
                            ) && input_port
                                .as_deref()
                                .is_none_or(|port| port == edge.spec.to.port)
                        })
                    })
                    .ok_or(SessionPrepareError::OperatorDeclarationMismatch {
                        node_id: NodeId::from_index(u32::MAX),
                    })?;
                let typed_edge = runtime_plan.typed_edge(edge.spec.id).ok_or(
                    SessionPrepareError::MissingTypedEdgePlan {
                        edge_id: edge.spec.id,
                    },
                )?;
                branches.push(PreparedExternalSourceBranch {
                    output_port: output.output_port().to_owned(),
                    stream_id: output.stream_id(),
                    branch: TypedEdgeBranchSpec {
                        capacity_signals: typed_edge.capacity_signals,
                        edge_contract: typed_edge.contract,
                    },
                    target: PreparedExternalSourceTarget::OperatorInput(
                        PreparedExternalOperatorInput {
                            operator_instance_id: *operator_instance_id,
                            input_port: edge.spec.to.port.clone(),
                            edge_id: edge.spec.id,
                            signal_spec: typed_edge.signal.clone(),
                            media: typed_edge.media,
                            edge_contract: typed_edge.contract,
                            capacity_signals: typed_edge.capacity_signals,
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
    let allocated_max = spec
        .stems()
        .iter()
        .map(|stem| stem.id().0)
        .chain(
            spec.generated_audio_ingresses()
                .iter()
                .map(|ingress| ingress.stem_id().0),
        )
        .max()
        .unwrap_or(0);
    let output_index = spec
        .source_outputs()
        .iter()
        .position(|candidate| candidate == output)
        .unwrap_or(0) as u64;
    StemId(allocated_max.saturating_add(output_index).saturating_add(1))
}

fn prepare_sources(
    spec: &SessionSpec,
    graph_ir: &GraphIr,
    bindings: &CompiledSessionBindings,
    source_queue_capacity_frames: usize,
    cancellation: &PlanRunnerCancellation,
) -> Result<(Vec<PreparedSourceMapping>, Vec<PlanSourceInput>), SessionPrepareError> {
    let mut mappings = Vec::with_capacity(spec.stems().len());
    let mut inputs = Vec::with_capacity(spec.stems().len());
    for stem in spec.stems() {
        let mut matches = graph_ir.nodes.iter().filter(|node| {
            matches!(
                bindings.node(node.id()),
                Some(CompiledNodeBinding::StemSource { stem_id }) if *stem_id == stem.id()
            )
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

struct WorkerMappingContext<'prepare> {
    spec: &'prepare SessionSpec,
    graph_ir: &'prepare GraphIr,
    runtime_plan: &'prepare crate::graph::plan::RuntimePlan,
    bindings: &'prepare CompiledSessionBindings,
    node_registry: &'prepare NodeRegistry,
}

fn map_worker_receivers(
    context: WorkerMappingContext<'_>,
    worker_receivers: Vec<PlanEdgeReceiver>,
    generated_audio_ingresses: &mut Vec<PreparedGeneratedAudioIngress>,
) -> Result<(Vec<PreparedWorkerMapping>, Vec<PreparedOperatorMapping>), SessionPrepareError> {
    let WorkerMappingContext {
        spec,
        graph_ir,
        runtime_plan,
        bindings,
        node_registry,
    } = context;
    let mut mapped_routes = HashSet::with_capacity(worker_receivers.len());
    let mut mappings = Vec::with_capacity(worker_receivers.len());
    let mut operator_mappings = Vec::with_capacity(spec.operators().len());
    let mut operator_indices = HashMap::with_capacity(spec.operators().len());
    let mut instance_indices = HashMap::with_capacity(spec.operators().len());
    let signal_endpoint_connections = spec
        .connections()
        .iter()
        .filter(|connection| {
            matches!(connection.origin(), StreamOrigin::OperatorOutput { .. })
                && matches!(connection.target(), ConnectionTarget::EndpointInput { .. })
        })
        .count();
    let operator_input_connections = spec
        .connections()
        .iter()
        .filter(|connection| matches!(connection.target(), ConnectionTarget::OperatorInput { .. }))
        .count();
    let mut signal_edge_ids = HashSet::with_capacity(signal_endpoint_connections);
    let mut mapped_signal_routes = HashSet::with_capacity(signal_endpoint_connections);
    let mut mapped_operator_input_edges = HashSet::with_capacity(operator_input_connections);

    for declaration in spec.operators() {
        let node = graph_ir
            .nodes
            .iter()
            .find(|node| {
                matches!(
                    bindings.node(node.id()),
                    Some(CompiledNodeBinding::Operator {
                        operator_instance_id,
                    }) if *operator_instance_id == declaration.instance_id()
                ) && node_registry
                    .async_factory(&node.spec.type_id)
                    .is_some_and(|factory| {
                        factory.manifest().operator_id == *declaration.operator_id()
                    })
            })
            .ok_or(SessionPrepareError::OperatorDeclarationMismatch {
                node_id: NodeId::from_index(u32::MAX),
            })?;
        let factory = node_registry
            .async_factory(&node.spec.type_id)
            .ok_or(SessionPrepareError::MissingAsyncOperatorFactory { node_id: node.id() })?;
        if declaration.operator_id() != &factory.manifest().operator_id {
            return Err(SessionPrepareError::OperatorDeclarationMismatch { node_id: node.id() });
        }
        let index = operator_mappings.len();
        if operator_indices.insert(node.id(), index).is_some()
            || instance_indices
                .insert(declaration.instance_id(), index)
                .is_some()
        {
            return Err(SessionPrepareError::OperatorDeclarationMismatch { node_id: node.id() });
        }
        operator_mappings.push(PreparedOperatorMapping {
            node_id: node.id(),
            instance_id: declaration.instance_id(),
            factory: Arc::clone(factory),
            node_configuration: node.spec.config.clone(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        });
    }

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
        if node_registry.async_factory(&target.spec.type_id).is_some() {
            let CompiledNodeBinding::Operator {
                operator_instance_id: target_instance,
            } = required_node_binding(bindings, target.id())?
            else {
                return Err(SessionPrepareError::IncompatibleNodeBinding {
                    node_id: target.id(),
                });
            };
            let typed_edge = runtime_plan.typed_edge(edge.spec.id).ok_or(
                SessionPrepareError::MissingTypedEdgePlan {
                    edge_id: edge.spec.id,
                },
            )?;
            let output = factory
                .manifest()
                .output_ports()
                .find(|port| port.name == edge.spec.from.port)
                .ok_or(SessionPrepareError::OperatorDeclarationMismatch {
                    node_id: source.id(),
                })?;
            let source_index = operator_indices[&source.id()];
            operator_mappings[source_index]
                .outputs
                .push(PreparedOperatorOutputMapping {
                    output_port: edge.spec.from.port.clone(),
                    branch: AsyncOperatorOutputBranchSpec {
                        capacity_signals: typed_edge.capacity_signals,
                        edge_contract: typed_edge.contract,
                    },
                    target: PreparedOperatorOutputTarget::OperatorInput {
                        operator_instance_id: *target_instance,
                        input_port: edge.spec.to.port.clone(),
                    },
                });
            if !output.media.is_compatible_with(&edge.media) {
                return Err(SessionPrepareError::OperatorDeclarationMismatch {
                    node_id: source.id(),
                });
            }
            continue;
        }
        if matches!(
            bindings.node(target.id()),
            Some(CompiledNodeBinding::GeneratedAudioBridge { .. })
        ) {
            let CompiledNodeBinding::GeneratedAudioBridge {
                stem_id,
                operator_instance_id,
            } = required_node_binding(bindings, target.id())?
            else {
                return Err(SessionPrepareError::IncompatibleNodeBinding {
                    node_id: target.id(),
                });
            };
            let declaration = spec
                .generated_audio_ingresses()
                .iter()
                .find(|ingress| {
                    ingress.stem_id() == *stem_id
                        && ingress.operator_instance_id() == *operator_instance_id
                        && ingress
                            .output_port()
                            .is_none_or(|port| port == edge.spec.from.port)
                })
                .ok_or(SessionPrepareError::MissingGeneratedAudioBridge { stem_id: *stem_id })?;
            let typed_edge = runtime_plan.typed_edge(edge.spec.id).ok_or(
                SessionPrepareError::MissingTypedEdgePlan {
                    edge_id: edge.spec.id,
                },
            )?;
            if !typed_edge.signal.class.is_audio() {
                return Err(SessionPrepareError::InvalidGeneratedAudioMedia { stem_id: *stem_id });
            }
            let index = generated_audio_ingresses
                .iter()
                .position(|prepared| prepared.stem_id == declaration.stem_id())
                .ok_or(SessionPrepareError::MissingGeneratedAudioIngress { stem_id: *stem_id })?;
            let prepared = generated_audio_ingresses.remove(index);
            signal_edge_ids.insert(edge.spec.id);
            let source_index = operator_indices[&source.id()];
            operator_mappings[source_index]
                .outputs
                .push(PreparedOperatorOutputMapping {
                    output_port: edge.spec.from.port.clone(),
                    branch: AsyncOperatorOutputBranchSpec {
                        capacity_signals: typed_edge.capacity_signals,
                        edge_contract: typed_edge.contract,
                    },
                    target: PreparedOperatorOutputTarget::GeneratedAudio(prepared),
                });
            continue;
        }
        let CompiledNodeBinding::Endpoint {
            route_id,
            endpoint_id,
            connector_id,
            origin,
        } = required_node_binding(bindings, target.id())?
        else {
            return Err(SessionPrepareError::IncompatibleNodeBinding {
                node_id: target.id(),
            });
        };
        let stem_id = match origin {
            StreamOrigin::Stem(stem_id) => Some(*stem_id),
            _ => None,
        };
        let signal_origin = match origin {
            StreamOrigin::SourceOutput {
                source_id,
                stream_id,
                ..
            } => Some((*source_id, *stream_id)),
            _ => None,
        };
        let connection = spec
            .connections()
            .iter()
            .find(|connection| {
                connection.id() == *route_id
                    && matches!(connection.origin(), StreamOrigin::OperatorOutput { .. })
                    && matches!(connection.target(), ConnectionTarget::EndpointInput { .. })
            })
            .ok_or(SessionPrepareError::UnknownWorkerRoute {
                edge_id: edge.spec.id,
                route_id: *route_id,
            })?;
        let CompiledNodeBinding::Operator {
            operator_instance_id,
        } = required_node_binding(bindings, source.id())?
        else {
            return Err(SessionPrepareError::IncompatibleNodeBinding {
                node_id: source.id(),
            });
        };
        let _operator = spec
            .operators()
            .iter()
            .find(|operator| operator.instance_id() == *operator_instance_id)
            .ok_or(SessionPrepareError::SignalRouteMismatch {
                edge_id: edge.spec.id,
                route_id: *route_id,
            })?;
        let matches_connection = matches!(
            connection.origin(),
            StreamOrigin::OperatorOutput {
                operator_instance_id: declared_operator,
                ..
            } if *declared_operator == *operator_instance_id
        ) && matches!(
            connection.target(),
            ConnectionTarget::EndpointInput {
                endpoint_id: declared_endpoint,
                ..
            } if *declared_endpoint == *endpoint_id
        );
        if !matches_connection {
            return Err(SessionPrepareError::SignalRouteMismatch {
                edge_id: edge.spec.id,
                route_id: *route_id,
            });
        }
        if !mapped_signal_routes.insert(*route_id) {
            return Err(SessionPrepareError::DuplicateSignalRoute {
                route_id: *route_id,
            });
        }
        let signal_spec = factory
            .manifest()
            .output_ports()
            .find(|port| port.name == edge.spec.from.port)
            .ok_or(SessionPrepareError::SignalRouteMismatch {
                edge_id: edge.spec.id,
                route_id: *route_id,
            })?
            .signal
            .clone();
        let typed_edge = runtime_plan.typed_edge(edge.spec.id).ok_or(
            SessionPrepareError::MissingTypedEdgePlan {
                edge_id: edge.spec.id,
            },
        )?;
        signal_edge_ids.insert(edge.spec.id);
        let source_index = operator_indices[&source.id()];
        operator_mappings[source_index]
            .outputs
            .push(PreparedOperatorOutputMapping {
                output_port: edge.spec.from.port.clone(),
                branch: AsyncOperatorOutputBranchSpec {
                    capacity_signals: typed_edge.capacity_signals,
                    edge_contract: typed_edge.contract,
                },
                target: PreparedOperatorOutputTarget::SignalEndpoint(Box::new(
                    PreparedSignalRouteMapping {
                        route_id: *route_id,
                        stem_id,
                        signal_origin,
                        endpoint_id: *endpoint_id,
                        connector_id: *connector_id,
                        node_configuration: target.spec.config.clone(),
                        input_port: edge.spec.to.port.clone(),
                        signal_spec,
                        output_branch: AsyncOperatorOutputBranchSpec {
                            capacity_signals: typed_edge.capacity_signals,
                            edge_contract: typed_edge.contract,
                        },
                    },
                )),
            });
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
            let input_port = target
                .descriptor
                .inputs
                .iter()
                .find(|port| port.name == edge.spec.to.port)
                .ok_or_else(|| SessionPrepareError::InvalidOperatorInputPort {
                    edge_id,
                    port_name: edge.spec.to.port.clone(),
                })?;
            if input_port.signal
                != factory
                    .manifest()
                    .input_ports()
                    .find(|port| port.name == edge.spec.to.port)
                    .ok_or_else(|| SessionPrepareError::InvalidOperatorInputPort {
                        edge_id,
                        port_name: edge.spec.to.port.clone(),
                    })?
                    .signal
                || !input_port.media.is_compatible_with(&edge.media)
            {
                return Err(SessionPrepareError::InvalidOperatorInputPort {
                    edge_id,
                    port_name: edge.spec.to.port.clone(),
                });
            }
            let input_edge_contract = edge
                .contract
                .ok_or(SessionPrepareError::MissingWorkerEdgeContract { edge_id })?;
            let input_capacity_signals = runtime_plan
                .typed_edge(edge_id)
                .map(|plan| plan.capacity_signals)
                .or_else(|| {
                    runtime_plan
                        .memory_plan
                        .edge_buffer(edge_id)
                        .map(|plan| plan.capacity_frames)
                })
                .ok_or(SessionPrepareError::MissingWorkerCapacity { edge_id })?;
            let CompiledNodeBinding::Operator {
                operator_instance_id: instance_id,
            } = required_node_binding(bindings, target.id())?
            else {
                return Err(SessionPrepareError::IncompatibleNodeBinding {
                    node_id: target.id(),
                });
            };
            let declaration = spec
                .operators()
                .iter()
                .find(|operator| operator.instance_id() == *instance_id)
                .ok_or(SessionPrepareError::OperatorDeclarationMismatch {
                    node_id: target.id(),
                })?;
            let connection = spec
                .connections()
                .iter()
                .find(|connection| {
                    let ConnectionTarget::OperatorInput {
                        operator_instance_id,
                        input_port,
                    } = connection.target()
                    else {
                        return false;
                    };
                    *operator_instance_id == *instance_id
                        && input_port.as_deref().map_or_else(
                            || {
                                let mut ports = factory.manifest().input_ports();
                                ports
                                    .next()
                                    .is_some_and(|port| port.name == edge.spec.to.port)
                                    && ports.next().is_none()
                            },
                            |port| port == edge.spec.to.port,
                        )
                })
                .ok_or(SessionPrepareError::OperatorDeclarationMismatch {
                    node_id: target.id(),
                })?;
            let stem_id = match connection.origin() {
                StreamOrigin::Stem(stem_id) => *stem_id,
                StreamOrigin::SourceOutput {
                    source_instance_id,
                    output_port,
                    ..
                } => spec
                    .source_outputs()
                    .iter()
                    .find(|output| {
                        output.source_instance_id() == *source_instance_id
                            && output.output_port() == output_port
                    })
                    .map(|output| external_audio_stem_id(spec, output))
                    .ok_or(SessionPrepareError::OperatorDeclarationMismatch {
                        node_id: target.id(),
                    })?,
                StreamOrigin::OperatorOutput { .. } => {
                    return Err(SessionPrepareError::OperatorDeclarationMismatch {
                        node_id: target.id(),
                    });
                }
            };
            if declaration.operator_id() != &factory.manifest().operator_id {
                return Err(SessionPrepareError::OperatorDeclarationMismatch {
                    node_id: target.id(),
                });
            }
            if !mapped_operator_input_edges.insert(edge_id) {
                return Err(SessionPrepareError::DuplicateOperatorInput {
                    node_id: target.id(),
                });
            }
            let mapping_index = operator_indices[&target.id()];
            operator_mappings[mapping_index]
                .inputs
                .push(PreparedOperatorInputMapping::Compiled {
                    stem_id,
                    input_port: edge.spec.to.port.clone(),
                    signal_spec: input_port.signal.clone(),
                    media: edge.media,
                    edge_contract: input_edge_contract,
                    capacity_signals: input_capacity_signals,
                    receiver,
                });
            continue;
        }

        if signal_edge_ids.contains(&edge_id) {
            drop(receiver);
            continue;
        }

        let prepare_context = prepare_context_for_media(edge.media)
            .ok_or(SessionPrepareError::MissingWorkerSampleSpec { edge_id })?;
        let CompiledNodeBinding::Endpoint {
            route_id,
            endpoint_id,
            connector_id,
            origin,
        } = required_node_binding(bindings, target.id())?
        else {
            return Err(SessionPrepareError::IncompatibleNodeBinding {
                node_id: target.id(),
            });
        };
        if !mapped_routes.insert(*route_id) {
            return Err(SessionPrepareError::DuplicateWorkerRoute {
                route_id: *route_id,
            });
        }
        if let StreamOrigin::SourceOutput {
            source_instance_id,
            output_port,
            stream_id,
            source_id,
        } = origin
        {
            let output = spec
                .source_outputs()
                .iter()
                .find(|output| {
                    output.source_instance_id() == *source_instance_id
                        && output.output_port() == output_port
                })
                .ok_or(SessionPrepareError::UnknownWorkerRoute {
                    edge_id,
                    route_id: *route_id,
                })?;
            let stem_id = external_audio_stem_id(spec, output);
            mappings.push(PreparedWorkerMapping {
                route_id: *route_id,
                endpoint_id: *endpoint_id,
                connector_id: *connector_id,
                node_configuration: target.spec.config.clone(),
                receiver,
                prepare_context,
                input_port: edge.spec.to.port.clone(),
                signal_spec: target
                    .descriptor
                    .inputs
                    .iter()
                    .find(|port| port.name == edge.spec.to.port)
                    .map(|port| port.signal.clone())
                    .ok_or_else(|| SessionPrepareError::InvalidOperatorInputPort {
                        edge_id,
                        port_name: edge.spec.to.port.clone(),
                    })?,
                media: edge.media,
                edge_contract: edge
                    .contract
                    .ok_or(SessionPrepareError::MissingWorkerEdgeContract { edge_id })?,
                origin: PreparedWorkerOrigin::SignalIngress {
                    stem_id,
                    source_id: *source_id,
                    stream_id: *stream_id,
                },
            });
            continue;
        }
        let StreamOrigin::Stem(stem_id) = origin else {
            return Err(SessionPrepareError::IncompatibleNodeBinding {
                node_id: target.id(),
            });
        };
        let connection = spec
            .connections()
            .iter()
            .find(|connection| connection.id() == *route_id)
            .ok_or(SessionPrepareError::UnknownWorkerRoute {
                edge_id,
                route_id: *route_id,
            })?;
        let StreamOrigin::Stem(declared_stem_id) = connection.origin() else {
            return Err(SessionPrepareError::UnknownWorkerRoute {
                edge_id,
                route_id: *route_id,
            });
        };
        let ConnectionTarget::EndpointInput {
            endpoint_id: declared_endpoint_id,
            ..
        } = connection.target()
        else {
            return Err(SessionPrepareError::UnknownWorkerRoute {
                edge_id,
                route_id: *route_id,
            });
        };
        if *declared_stem_id != *stem_id || *declared_endpoint_id != *endpoint_id {
            return Err(SessionPrepareError::WorkerRouteMismatch {
                edge_id,
                route_id: *route_id,
                expected_stem_id: *declared_stem_id,
                actual_stem_id: *stem_id,
                expected_endpoint_id: *declared_endpoint_id,
                actual_endpoint_id: *endpoint_id,
            });
        }
        let origin = spec
            .generated_audio_ingresses()
            .iter()
            .find(|ingress| ingress.stem_id() == *stem_id)
            .map_or(PreparedWorkerOrigin::Stem(*stem_id), |ingress| {
                PreparedWorkerOrigin::SignalIngress {
                    stem_id: *stem_id,
                    source_id: ingress.source_id(),
                    stream_id: ingress.stream_id(),
                }
            });
        mappings.push(PreparedWorkerMapping {
            route_id: *route_id,
            endpoint_id: *endpoint_id,
            connector_id: *connector_id,
            node_configuration: target.spec.config.clone(),
            receiver,
            prepare_context,
            input_port: edge.spec.to.port.clone(),
            signal_spec: target
                .descriptor
                .inputs
                .iter()
                .find(|port| port.name == edge.spec.to.port)
                .map(|port| port.signal.clone())
                .ok_or_else(|| SessionPrepareError::InvalidOperatorInputPort {
                    edge_id,
                    port_name: edge.spec.to.port.clone(),
                })?,
            media: edge.media,
            edge_contract: edge
                .contract
                .ok_or(SessionPrepareError::MissingWorkerEdgeContract { edge_id })?,
            origin,
        });
    }
    for edge in &graph_ir.edges {
        if mapped_operator_input_edges.contains(&edge.spec.id) {
            continue;
        }
        let Some(target) = graph_ir.node(edge.spec.to.node) else {
            continue;
        };
        let Some(factory) = node_registry.async_factory(&target.spec.type_id) else {
            continue;
        };
        let CompiledNodeBinding::Operator {
            operator_instance_id: instance_id,
        } = required_node_binding(bindings, target.id())?
        else {
            return Err(SessionPrepareError::IncompatibleNodeBinding {
                node_id: target.id(),
            });
        };
        let connection = spec
            .connections()
            .iter()
            .find(|connection| {
                let ConnectionTarget::OperatorInput {
                    operator_instance_id,
                    input_port,
                } = connection.target()
                else {
                    return false;
                };
                *operator_instance_id == *instance_id
                    && input_port.as_deref().map_or_else(
                        || {
                            let mut ports = factory.manifest().input_ports();
                            ports
                                .next()
                                .is_some_and(|port| port.name == edge.spec.to.port)
                                && ports.next().is_none()
                        },
                        |port| port == edge.spec.to.port,
                    )
            })
            .ok_or(SessionPrepareError::OperatorDeclarationMismatch {
                node_id: target.id(),
            })?;
        let origin = match connection.origin() {
            StreamOrigin::SourceOutput {
                source_instance_id,
                output_port,
                ..
            } => PreparedTypedInputOrigin::SourceOutput {
                source_instance_id: *source_instance_id,
                output_port: output_port.clone(),
            },
            StreamOrigin::OperatorOutput {
                operator_instance_id,
                output_port,
            } => {
                let upstream = instance_indices[operator_instance_id];
                let selected = output_port.clone().or_else(|| {
                    let mut ports = operator_mappings[upstream]
                        .factory
                        .manifest()
                        .output_ports();
                    let port = ports.next()?;
                    ports.next().is_none().then(|| port.name.clone())
                });
                PreparedTypedInputOrigin::OperatorOutput {
                    operator_instance_id: *operator_instance_id,
                    output_port: selected.ok_or(
                        SessionPrepareError::OperatorDeclarationMismatch {
                            node_id: target.id(),
                        },
                    )?,
                }
            }
            StreamOrigin::Stem(_) => {
                return Err(SessionPrepareError::MissingOperatorSignalInput {
                    edge_id: edge.spec.id,
                });
            }
        };
        let input = factory
            .manifest()
            .input_ports()
            .find(|port| port.name == edge.spec.to.port)
            .ok_or_else(|| SessionPrepareError::InvalidOperatorInputPort {
                edge_id: edge.spec.id,
                port_name: edge.spec.to.port.clone(),
            })?;
        let typed_edge = runtime_plan.typed_edge(edge.spec.id).ok_or(
            SessionPrepareError::MissingTypedEdgePlan {
                edge_id: edge.spec.id,
            },
        )?;
        let mapping_index = operator_indices[&target.id()];
        operator_mappings[mapping_index]
            .inputs
            .push(PreparedOperatorInputMapping::Typed {
                edge_id: edge.spec.id,
                input_port: edge.spec.to.port.clone(),
                signal_spec: input.signal.clone(),
                media: edge.media,
                edge_contract: typed_edge.contract,
                capacity_signals: typed_edge.capacity_signals,
                origin,
            });
    }
    let expected_audio_source_routes = mappings
        .iter()
        .filter(|mapping| {
            matches!(mapping.origin, PreparedWorkerOrigin::SignalIngress { .. })
                && !spec
                    .generated_audio_ingresses()
                    .iter()
                    .any(|ingress| ingress.stem_id() == mapping.origin.stem_id())
        })
        .count();
    let expected_stem_routes = spec
        .connections()
        .iter()
        .filter(|connection| {
            matches!(connection.origin(), StreamOrigin::Stem(_))
                && matches!(connection.target(), ConnectionTarget::EndpointInput { .. })
        })
        .count();
    if mappings.len() != expected_stem_routes + expected_audio_source_routes
        || operator_mappings.len() != spec.operators().len()
        || operator_mappings
            .iter()
            .map(|mapping| mapping.inputs.len())
            .sum::<usize>()
            != operator_input_connections
    {
        return Err(SessionPrepareError::WorkerTopologyMismatch {
            expected: expected_stem_routes + expected_audio_source_routes,
            actual: mappings.len(),
            expected_operator_inputs: operator_input_connections,
            actual_operator_inputs: operator_mappings
                .iter()
                .map(|mapping| mapping.inputs.len())
                .sum(),
            expected_signal_endpoints: signal_endpoint_connections,
            actual_signal_endpoints: operator_mappings
                .iter()
                .flat_map(|mapping| &mapping.outputs)
                .filter(|mapping| {
                    matches!(
                        mapping.target,
                        PreparedOperatorOutputTarget::SignalEndpoint(_)
                    )
                })
                .count(),
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

fn required_node_binding(
    bindings: &CompiledSessionBindings,
    node_id: NodeId,
) -> Result<&CompiledNodeBinding, SessionPrepareError> {
    bindings
        .node(node_id)
        .ok_or(SessionPrepareError::MissingNodeBinding { node_id })
}

#[cfg(test)]
mod tests;

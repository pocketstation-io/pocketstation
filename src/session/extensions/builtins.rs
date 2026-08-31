use std::collections::HashMap;
use std::sync::Arc;

#[cfg(any(test, feature = "internal-testing"))]
use crate::frame::SampleFormat;
use crate::frame::{AudioFrame, AudioFrameDuration, SampleSpec};
use crate::graph::{
    AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec, SafetyContract,
    SignalSpec,
};
use crate::graph::{
    ConfigError, ExecutionPartition, NodeConfig, NodeDefinition, NodeDescriptor, NodeError,
    NodeFactory, NodeRegistry, NodeTypeId, Pipeline, PrepareContext, RuntimeNode,
};

use crate::session::compile::{
    select_operator_port, CompiledNodeBinding, CompiledSessionBindings, LoweredOperator,
    SessionCompileError, SessionGraphLowerer, SessionSourceLoweringContext,
};
use crate::session::{EndpointSpec, OperatorInstanceId, SessionSpec, Source, StemId};

const AUDIO_PORT: &str = "audio";
pub const APPLICATION_SOURCE_NODE_TYPE_ID: &str = "source.application";
pub const MICROPHONE_SOURCE_NODE_TYPE_ID: &str = "source.microphone";
pub(crate) const EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID: &str = "source.external_audio_ingress";
pub(crate) const GENERATED_AUDIO_INGRESS_NODE_TYPE_ID: &str = "source.generated_audio_ingress";
pub(crate) const GENERATED_AUDIO_BRIDGE_NODE_TYPE_ID: &str = "bridge.generated_audio";

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SessionGraphRegistrationError {
    #[error("Session structural node type '{node_type_id}' is already registered")]
    DuplicateNodeType { node_type_id: String },
}

#[cfg(any(test, feature = "internal-testing"))]
pub fn register_session_graph_nodes(
    registry: &mut NodeRegistry,
) -> Result<(), SessionGraphRegistrationError> {
    register_session_graph_nodes_with_sample_spec(
        registry,
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        AudioFrameDuration::default(),
    )
    .map(|_| ())
}

pub(crate) fn register_session_graph_nodes_with_sample_spec(
    registry: &mut NodeRegistry,
    sample_spec: SampleSpec,
    audio_frame_duration: AudioFrameDuration,
) -> Result<Vec<Arc<dyn SessionGraphLowerer>>, SessionGraphRegistrationError> {
    let frame_samples_per_channel =
        audio_frame_duration.samples_per_channel(sample_spec.sample_rate_hz);
    let factories: Vec<Arc<dyn NodeFactory>> = vec![
        Arc::new(AudioIngressFactory::new(
            APPLICATION_SOURCE_NODE_TYPE_ID,
            "Application Capture Ingress",
            sample_spec,
            frame_samples_per_channel,
            ChannelLayout::Stereo,
        )),
        Arc::new(AudioIngressFactory::new(
            MICROPHONE_SOURCE_NODE_TYPE_ID,
            "Microphone Capture Ingress",
            sample_spec,
            frame_samples_per_channel,
            ChannelLayout::Mono,
        )),
        Arc::new(AudioIngressFactory::new(
            EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID,
            "External Audio Ingress",
            sample_spec,
            frame_samples_per_channel,
            channel_layout_for(sample_spec),
        )),
        Arc::new(AudioIngressFactory::new(
            GENERATED_AUDIO_INGRESS_NODE_TYPE_ID,
            "Generated Audio Ingress",
            sample_spec,
            frame_samples_per_channel,
            channel_layout_for(sample_spec),
        )),
    ];
    let definitions: Vec<Arc<dyn NodeDefinition>> =
        vec![Arc::new(GeneratedAudioBridgeDefinition {
            sample_spec,
            frame_samples_per_channel,
        })];

    for node_type_id in factories
        .iter()
        .map(|factory| factory.descriptor().type_id)
        .chain(
            definitions
                .iter()
                .map(|definition| definition.descriptor().type_id),
        )
    {
        if registry.contains(&node_type_id) {
            return Err(SessionGraphRegistrationError::DuplicateNodeType {
                node_type_id: node_type_id.as_str().to_owned(),
            });
        }
    }

    for factory in factories {
        let node_type_id = factory.descriptor().type_id.as_str().to_owned();
        registry
            .register(factory)
            .map_err(|_| SessionGraphRegistrationError::DuplicateNodeType { node_type_id })?;
    }
    for definition in definitions {
        let node_type_id = definition.descriptor().type_id.as_str().to_owned();
        registry
            .register_definition(definition)
            .map_err(|_| SessionGraphRegistrationError::DuplicateNodeType { node_type_id })?;
    }
    Ok(default_session_graph_lowerers())
}

pub(crate) fn default_session_graph_lowerers() -> Vec<Arc<dyn SessionGraphLowerer>> {
    vec![
        Arc::new(BuiltinSourceLowerer),
        Arc::new(OperatorAudioLowerer),
    ]
}

struct BuiltinSourceLowerer;

impl SessionGraphLowerer for BuiltinSourceLowerer {
    fn lower_source_nodes(
        &self,
        spec: &SessionSpec,
        context: &mut SessionSourceLoweringContext<'_>,
    ) -> Result<(), SessionCompileError> {
        for stem in spec.stems() {
            let node_type_id = match stem.source() {
                Source::Application(_) => NodeTypeId::from(APPLICATION_SOURCE_NODE_TYPE_ID),
                Source::Microphone(_) => NodeTypeId::from(MICROPHONE_SOURCE_NODE_TYPE_ID),
            };
            let source_node = context.pipeline.add_node(node_type_id, NodeConfig::new());
            context.bindings.insert_node(
                source_node.id(),
                CompiledNodeBinding::StemSource { stem_id: stem.id() },
            );
            context.source_nodes.insert(stem.id(), source_node);
        }

        for source in spec.source_instances() {
            let mut config = NodeConfig::new();
            for (key, value) in source.configuration().iter() {
                config = config.with(key, value);
            }
            let source_node = context
                .pipeline
                .add_node(NodeTypeId::from(source.source_type_id().as_str()), config);
            context.bindings.insert_node(
                source_node.id(),
                CompiledNodeBinding::ExternalSource {
                    source_instance_id: source.instance_id(),
                },
            );
            context
                .external_source_nodes
                .insert(source.instance_id(), source_node.id());
            let manifest = context
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
                    let ingress = context.pipeline.add_node(
                        NodeTypeId::from(EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID),
                        NodeConfig::new(),
                    );
                    context.bindings.insert_node(
                        ingress.id(),
                        CompiledNodeBinding::ExternalAudioIngress {
                            source_instance_id: source.instance_id(),
                            output_port: output.output_port().to_owned(),
                        },
                    );
                    context.external_audio_ingress_nodes.insert(
                        (source.instance_id(), output.output_port().to_owned()),
                        ingress.id(),
                    );
                }
            }
        }
        Ok(())
    }

    fn lower_operator_edges(
        &self,
        _spec: &SessionSpec,
        _pipeline: &mut Pipeline,
        _operator_nodes: &HashMap<
            crate::session::OperatorInstanceId,
            crate::session::compile::LoweredOperator,
        >,
        _bindings: &mut CompiledSessionBindings,
    ) -> Result<(), SessionCompileError> {
        Ok(())
    }

    fn endpoint_config(
        &self,
        _spec: &SessionSpec,
        _stem_id: StemId,
        _endpoint: &crate::session::EndpointSpec,
        _route_id: crate::session::RouteId,
    ) -> Result<Option<NodeConfig>, SessionCompileError> {
        Ok(None)
    }
}

struct OperatorAudioLowerer;

impl SessionGraphLowerer for OperatorAudioLowerer {
    fn lower_source_nodes(
        &self,
        spec: &SessionSpec,
        context: &mut SessionSourceLoweringContext<'_>,
    ) -> Result<(), SessionCompileError> {
        for ingress in spec.generated_audio_ingresses() {
            let node = context.pipeline.add_node(
                NodeTypeId::from(GENERATED_AUDIO_INGRESS_NODE_TYPE_ID),
                NodeConfig::new(),
            );
            context.bindings.insert_node(
                node.id(),
                CompiledNodeBinding::GeneratedAudioIngress {
                    stem_id: ingress.stem_id(),
                },
            );
            context.source_nodes.insert(ingress.stem_id(), node);
        }
        Ok(())
    }

    fn lower_operator_edges(
        &self,
        spec: &SessionSpec,
        pipeline: &mut Pipeline,
        operator_nodes: &HashMap<OperatorInstanceId, LoweredOperator>,
        bindings: &mut CompiledSessionBindings,
    ) -> Result<(), SessionCompileError> {
        for ingress in spec.generated_audio_ingresses() {
            let operator = operator_nodes.get(&ingress.operator_instance_id()).ok_or(
                crate::session::SessionError::UnknownOperatorInstance {
                    operator_instance_id: ingress.operator_instance_id(),
                },
            )?;
            let output = select_operator_port(
                &operator.manifest,
                PortDirection::Output,
                ingress.output_port(),
            )?;
            let concrete_pcm = output.signal.class.is_audio()
                && matches!(
                    output.media,
                    MediaCaps::Audio(audio)
                        if audio.sample_rate_hz.is_some()
                            && audio.frame_samples.is_some()
                            && !matches!(audio.channel_layout, ChannelLayout::Any)
                );
            if !concrete_pcm {
                return Err(SessionCompileError::InvalidAudioBridgeOutput {
                    operator_instance_id: ingress.operator_instance_id(),
                    output_port: output.name.clone(),
                });
            }
            let ordinary_consumers = spec
                .connections()
                .iter()
                .filter(|connection| {
                    let crate::session::StreamOrigin::OperatorOutput {
                        operator_instance_id,
                        output_port,
                    } = connection.origin()
                    else {
                        return false;
                    };
                    if *operator_instance_id != ingress.operator_instance_id() {
                        return false;
                    }
                    select_operator_port(
                        &operator.manifest,
                        PortDirection::Output,
                        output_port.as_deref(),
                    )
                    .is_ok_and(|candidate| candidate.name == output.name)
                })
                .count();
            let reentry_consumers = spec
                .generated_audio_ingresses()
                .iter()
                .filter(|candidate| {
                    if candidate.operator_instance_id() != ingress.operator_instance_id() {
                        return false;
                    }
                    select_operator_port(
                        &operator.manifest,
                        PortDirection::Output,
                        candidate.output_port(),
                    )
                    .is_ok_and(|candidate_port| candidate_port.name == output.name)
                })
                .count();
            if ordinary_consumers != 0 || reentry_consumers != 1 {
                return Err(SessionCompileError::AudioBridgeOutputNotExclusive {
                    operator_instance_id: ingress.operator_instance_id(),
                    output_port: output.name.clone(),
                });
            }
            let bridge = pipeline.add_node(
                NodeTypeId::from(GENERATED_AUDIO_BRIDGE_NODE_TYPE_ID),
                NodeConfig::new(),
            );
            bindings.insert_node(
                bridge.id(),
                CompiledNodeBinding::GeneratedAudioBridge {
                    stem_id: ingress.stem_id(),
                    operator_instance_id: ingress.operator_instance_id(),
                },
            );
            pipeline.connect_with(
                operator.node.out(&output.name),
                bridge.in_(AUDIO_PORT),
                operator.manifest.output_edge,
            );
        }
        Ok(())
    }

    fn endpoint_config(
        &self,
        spec: &SessionSpec,
        stem_id: StemId,
        endpoint: &EndpointSpec,
        _route_id: crate::session::RouteId,
    ) -> Result<Option<NodeConfig>, SessionCompileError> {
        if !spec
            .generated_audio_ingresses()
            .iter()
            .any(|ingress| ingress.stem_id() == stem_id)
        {
            return Ok(None);
        }
        let mut config = NodeConfig::new();
        for (key, value) in endpoint.configuration().iter() {
            config = config.with(key, value);
        }
        Ok(Some(config))
    }
}

struct AudioIngressFactory {
    node_type_id: &'static str,
    display_name: &'static str,
    sample_spec: SampleSpec,
    frame_samples_per_channel: usize,
    channel_layout: ChannelLayout,
}

impl AudioIngressFactory {
    const fn new(
        node_type_id: &'static str,
        display_name: &'static str,
        sample_spec: SampleSpec,
        frame_samples_per_channel: usize,
        channel_layout: ChannelLayout,
    ) -> Self {
        Self {
            node_type_id,
            display_name,
            sample_spec,
            frame_samples_per_channel,
            channel_layout,
        }
    }
}

impl NodeFactory for AudioIngressFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(self.node_type_id),
            display_name: self.display_name,
            inputs: Vec::new(),
            outputs: vec![audio_port(
                PortDirection::Output,
                self.sample_spec,
                self.frame_samples_per_channel,
                self.channel_layout,
            )],
            execution: ExecutionPartition::RealtimeCpu,
            safety: SafetyContract::RealtimeSafe,
            stateful: false,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _context: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        self.validate_config(config)?;
        Ok(Box::new(SourceIngressNode))
    }
}

const fn channel_layout_for(sample_spec: SampleSpec) -> ChannelLayout {
    match sample_spec.channels {
        1 => ChannelLayout::Mono,
        2 => ChannelLayout::Stereo,
        _ => ChannelLayout::Any,
    }
}

struct GeneratedAudioBridgeDefinition {
    sample_spec: SampleSpec,
    frame_samples_per_channel: usize,
}

impl NodeDefinition for GeneratedAudioBridgeDefinition {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(GENERATED_AUDIO_BRIDGE_NODE_TYPE_ID),
            display_name: "Generated Audio Bridge",
            inputs: vec![audio_port(
                PortDirection::Input,
                self.sample_spec,
                self.frame_samples_per_channel,
                ChannelLayout::Any,
            )],
            outputs: Vec::new(),
            execution: ExecutionPartition::BlockingWorker,
            safety: SafetyContract::BlockingAllowed,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
}

struct SourceIngressNode;

impl RuntimeNode for SourceIngressNode {
    fn prepare(&mut self, _context: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        Ok(Some(frame))
    }
}

struct EndpointBoundaryDefinition {
    node_type_id: NodeTypeId,
    sample_spec: SampleSpec,
    frame_samples_per_channel: usize,
}

impl EndpointBoundaryDefinition {
    fn new(
        node_type_id: NodeTypeId,
        sample_spec: SampleSpec,
        frame_samples_per_channel: usize,
    ) -> Self {
        Self {
            node_type_id,
            sample_spec,
            frame_samples_per_channel,
        }
    }
}

pub(crate) fn audio_endpoint_boundary_definition(
    node_type_id: NodeTypeId,
    sample_spec: SampleSpec,
    frame_samples_per_channel: usize,
) -> Arc<dyn NodeDefinition> {
    Arc::new(EndpointBoundaryDefinition::new(
        node_type_id,
        sample_spec,
        frame_samples_per_channel,
    ))
}

impl NodeDefinition for EndpointBoundaryDefinition {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: self.node_type_id.clone(),
            display_name: "Audio Endpoint Boundary",
            inputs: vec![audio_port(
                PortDirection::Input,
                self.sample_spec,
                self.frame_samples_per_channel,
                ChannelLayout::Any,
            )],
            outputs: Vec::new(),
            execution: ExecutionPartition::AsyncWorker,
            safety: SafetyContract::ExternalService,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }
}

fn audio_port(
    direction: PortDirection,
    sample_spec: SampleSpec,
    frame_samples_per_channel: usize,
    channel_layout: ChannelLayout,
) -> PortSpec {
    PortSpec {
        name: AUDIO_PORT.to_owned(),
        direction,
        signal: SignalSpec::audio(),
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(sample_spec.sample_rate_hz),
            frame_samples: Some(frame_samples_per_channel),
            channel_layout,
            format: sample_spec.format,
        }),
        multiplicity: Multiplicity::One,
        required: true,
    }
}

#[cfg(test)]
mod tests {
    use crate::frame::{AudioBufferPool, SampleSpec, SourceId, StreamId};

    use super::*;

    fn prepare_context() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn sample_spec() -> SampleSpec {
        prepare_context().sample_spec
    }

    fn endpoint_config() -> NodeConfig {
        NodeConfig::new()
    }

    fn frame() -> AudioFrame {
        let pool = AudioBufferPool::new(1, 4);
        let buffer = pool.acquire().expect("buffer");
        AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer)
    }

    #[test]
    fn given_empty_registry_when_components_registered_then_descriptors_and_lowerers_exist() {
        let mut registry = NodeRegistry::new();

        let lowerers = register_session_graph_nodes_with_sample_spec(
            &mut registry,
            sample_spec(),
            AudioFrameDuration::default(),
        )
        .expect("registration");

        let expected_node_type_ids = [
            APPLICATION_SOURCE_NODE_TYPE_ID,
            MICROPHONE_SOURCE_NODE_TYPE_ID,
            EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID,
            GENERATED_AUDIO_INGRESS_NODE_TYPE_ID,
            GENERATED_AUDIO_BRIDGE_NODE_TYPE_ID,
        ];
        assert_eq!(registry.len(), expected_node_type_ids.len());
        assert_eq!(lowerers.len(), 2);
        for node_type_id in expected_node_type_ids {
            assert!(registry.contains(&NodeTypeId::from(node_type_id)));
        }
    }

    #[test]
    fn given_duplicate_type_when_session_nodes_registered_then_registry_is_unchanged() {
        let mut registry = NodeRegistry::new();
        registry
            .register(Arc::new(AudioIngressFactory::new(
                APPLICATION_SOURCE_NODE_TYPE_ID,
                "Application Capture Ingress",
                sample_spec(),
                AudioFrameDuration::default().samples_per_channel(48_000),
                ChannelLayout::Stereo,
            )))
            .unwrap();
        let initial_len = registry.len();

        let result = register_session_graph_nodes(&mut registry);

        assert!(matches!(
            result,
            Err(SessionGraphRegistrationError::DuplicateNodeType { .. })
        ));
        assert_eq!(registry.len(), initial_len);
        assert!(!registry.contains(&NodeTypeId::from(MICROPHONE_SOURCE_NODE_TYPE_ID)));
    }

    #[test]
    fn given_application_ingress_when_frame_processed_then_identity_is_forwarded() {
        let factory = AudioIngressFactory::new(
            APPLICATION_SOURCE_NODE_TYPE_ID,
            "Application Capture Ingress",
            sample_spec(),
            AudioFrameDuration::default().samples_per_channel(48_000),
            ChannelLayout::Stereo,
        );
        let mut node = factory
            .instantiate(&prepare_context(), &NodeConfig::new())
            .expect("source ingress");
        let expected = frame();
        let expected_stream_id = expected.stream_id;
        let expected_source_id = expected.source_id;
        let expected_sequence_number = expected.sequence_number;
        let expected_timestamp_ns = expected.timestamp_ns;

        let output = node
            .process(expected)
            .expect("source processing")
            .expect("forwarded frame");

        assert_eq!(output.stream_id, expected_stream_id);
        assert_eq!(output.source_id, expected_source_id);
        assert_eq!(output.sequence_number, expected_sequence_number);
        assert_eq!(output.timestamp_ns, expected_timestamp_ns);
    }

    #[test]
    fn given_audio_endpoint_extension_when_requested_then_definition_is_not_boot_registered() {
        let mut registry = NodeRegistry::new();
        register_session_graph_nodes(&mut registry).expect("registration");
        let node_type_id = NodeTypeId::from("endpoint.test");

        assert!(registry.definition(&node_type_id).is_none());
        registry
            .register_definition(audio_endpoint_boundary_definition(
                node_type_id.clone(),
                sample_spec(),
                AudioFrameDuration::default().samples_per_channel(48_000),
            ))
            .expect("extension definition");

        let definition = registry.definition(&node_type_id).expect("definition");

        assert!(definition.validate_config(&endpoint_config()).is_ok());
        assert!(registry.get(&node_type_id).is_none());
    }

    #[test]
    fn given_structural_ingress_when_validated_then_session_metadata_is_not_required() {
        let factory = AudioIngressFactory::new(
            MICROPHONE_SOURCE_NODE_TYPE_ID,
            "Microphone Capture Ingress",
            sample_spec(),
            AudioFrameDuration::default().samples_per_channel(48_000),
            ChannelLayout::Mono,
        );

        assert!(factory.validate_config(&NodeConfig::new()).is_ok());
    }
}

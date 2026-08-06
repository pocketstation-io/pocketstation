use std::sync::Arc;

use crate::frame::{AudioFrame, SampleFormat, SampleSpec};
use crate::graph::{
    AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec, SafetyContract,
    SignalSpec,
};
use crate::graph::{
    ConfigError, ExecutionPartition, NodeConfig, NodeDefinition, NodeDescriptor, NodeError,
    NodeFactory, NodeRegistry, NodeTypeId, PrepareContext, RuntimeNode,
};

use crate::session::{
    APPLICATION_SOURCE_NODE_TYPE_ID, BROWSER_NODE_TYPE_ID, CONNECTOR_NODE_TYPE_ID,
    MICROPHONE_SOURCE_NODE_TYPE_ID, RECORDER_NODE_TYPE_ID,
};

const AUDIO_PORT: &str = "audio";
const STRUCTURAL_NODE_TYPE_IDS: [&str; 5] = [
    APPLICATION_SOURCE_NODE_TYPE_ID,
    MICROPHONE_SOURCE_NODE_TYPE_ID,
    CONNECTOR_NODE_TYPE_ID,
    BROWSER_NODE_TYPE_ID,
    RECORDER_NODE_TYPE_ID,
];

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SessionStructuralNodeRegistrationError {
    #[error("Session structural node type '{node_type_id}' is already registered")]
    DuplicateNodeType { node_type_id: String },
}

pub fn register_session_structural_nodes(
    registry: &mut NodeRegistry,
) -> Result<(), SessionStructuralNodeRegistrationError> {
    register_session_structural_nodes_with_sample_spec(
        registry,
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
    )
}

pub(crate) fn register_session_structural_nodes_with_sample_spec(
    registry: &mut NodeRegistry,
    sample_spec: SampleSpec,
) -> Result<(), SessionStructuralNodeRegistrationError> {
    for node_type_id in STRUCTURAL_NODE_TYPE_IDS {
        if registry.contains(&NodeTypeId::from(node_type_id)) {
            return Err(SessionStructuralNodeRegistrationError::DuplicateNodeType {
                node_type_id: node_type_id.to_owned(),
            });
        }
    }

    for factory in [
        SourceIngressFactory::application(sample_spec),
        SourceIngressFactory::microphone(sample_spec),
    ] {
        let node_type_id = factory.node_type_id().to_owned();
        registry.register(Arc::new(factory)).map_err(|_| {
            SessionStructuralNodeRegistrationError::DuplicateNodeType { node_type_id }
        })?;
    }
    for definition in [
        ExternalBoundaryFactory::connector(sample_spec),
        ExternalBoundaryFactory::browser(sample_spec),
        ExternalBoundaryFactory::recorder(sample_spec),
    ] {
        let node_type_id = definition.node_type_id.to_owned();
        registry
            .register_definition(Arc::new(definition))
            .map_err(
                |_| SessionStructuralNodeRegistrationError::DuplicateNodeType { node_type_id },
            )?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum SourceIngressKind {
    Application,
    Microphone,
}

struct SourceIngressFactory {
    kind: SourceIngressKind,
    sample_spec: SampleSpec,
}

impl SourceIngressFactory {
    const fn application(sample_spec: SampleSpec) -> Self {
        Self {
            kind: SourceIngressKind::Application,
            sample_spec,
        }
    }

    const fn microphone(sample_spec: SampleSpec) -> Self {
        Self {
            kind: SourceIngressKind::Microphone,
            sample_spec,
        }
    }

    fn node_type_id(&self) -> &'static str {
        match self.kind {
            SourceIngressKind::Application => APPLICATION_SOURCE_NODE_TYPE_ID,
            SourceIngressKind::Microphone => MICROPHONE_SOURCE_NODE_TYPE_ID,
        }
    }

    fn display_name(&self) -> &'static str {
        match self.kind {
            SourceIngressKind::Application => "Application Capture Ingress",
            SourceIngressKind::Microphone => "Microphone Capture Ingress",
        }
    }

    const fn channel_layout(&self) -> ChannelLayout {
        match self.kind {
            SourceIngressKind::Application => ChannelLayout::Stereo,
            SourceIngressKind::Microphone => ChannelLayout::Mono,
        }
    }
}

impl NodeFactory for SourceIngressFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(self.node_type_id()),
            display_name: self.display_name(),
            inputs: Vec::new(),
            outputs: vec![audio_port(
                PortDirection::Output,
                self.sample_spec,
                self.channel_layout(),
            )],
            execution: ExecutionPartition::RealtimeCpu,
            safety: SafetyContract::RealtimeSafe,
            stateful: false,
        }
    }

    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        require_nonempty(config, "session_id")?;
        require_nonempty(config, "stem_id")?;
        let selector_kind = require_nonempty(config, "selector_kind")?;
        match (self.kind, selector_kind) {
            (SourceIngressKind::Application, "bundle_id" | "process_id" | "stable_id" | "name") => {
                require_nonempty(config, "selector_value")?;
                Ok(())
            }
            (SourceIngressKind::Application, "process_instance") => {
                require_nonempty(config, "selector_process_id")?;
                require_nonempty(config, "selector_stable_id")?;
                Ok(())
            }
            (SourceIngressKind::Microphone, "default") => Ok(()),
            (SourceIngressKind::Microphone, "device_id") => {
                require_nonempty(config, "selector_value")?;
                Ok(())
            }
            _ => Err(ConfigError::Invalid {
                key: "selector_kind".to_owned(),
                reason: format!(
                    "selector kind '{selector_kind}' is invalid for {}",
                    self.node_type_id()
                ),
            }),
        }
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

struct SourceIngressNode;

impl RuntimeNode for SourceIngressNode {
    fn prepare(&mut self, _context: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        Ok(Some(frame))
    }
}

struct ExternalBoundaryFactory {
    node_type_id: &'static str,
    display_name: &'static str,
    required_configuration_keys: &'static [&'static str],
    sample_spec: SampleSpec,
}

impl ExternalBoundaryFactory {
    const fn connector(sample_spec: SampleSpec) -> Self {
        Self {
            node_type_id: CONNECTOR_NODE_TYPE_ID,
            display_name: "External Connector Boundary",
            required_configuration_keys: &["connector_id"],
            sample_spec,
        }
    }

    const fn browser(sample_spec: SampleSpec) -> Self {
        Self {
            node_type_id: BROWSER_NODE_TYPE_ID,
            display_name: "Remote Browser Boundary",
            required_configuration_keys: &["receiver_uri"],
            sample_spec,
        }
    }

    const fn recorder(sample_spec: SampleSpec) -> Self {
        Self {
            node_type_id: RECORDER_NODE_TYPE_ID,
            display_name: "Multistem Recording Boundary",
            required_configuration_keys: &["stem_name", "recording_group_id"],
            sample_spec,
        }
    }
}

impl NodeDefinition for ExternalBoundaryFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(self.node_type_id),
            display_name: self.display_name,
            inputs: vec![audio_port(
                PortDirection::Input,
                self.sample_spec,
                ChannelLayout::Any,
            )],
            outputs: Vec::new(),
            execution: ExecutionPartition::AsyncWorker,
            safety: SafetyContract::ExternalService,
            stateful: true,
        }
    }

    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        for key in [
            "session_id",
            "stem_id",
            "endpoint_id",
            "route_id",
            "operator_id",
        ] {
            require_nonempty(config, key)?;
        }
        for key in self.required_configuration_keys {
            require_nonempty(config, key)?;
        }
        Ok(())
    }
}

fn audio_port(
    direction: PortDirection,
    sample_spec: SampleSpec,
    channel_layout: ChannelLayout,
) -> PortSpec {
    PortSpec {
        name: AUDIO_PORT.to_owned(),
        direction,
        signal: SignalSpec::audio(),
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(sample_spec.sample_rate_hz),
            frame_samples: None,
            channel_layout,
            format: sample_spec.format,
        }),
        multiplicity: Multiplicity::One,
        required: true,
    }
}

fn require_nonempty<'config>(
    config: &'config NodeConfig,
    key: &'static str,
) -> Result<&'config str, ConfigError> {
    match config.get(key) {
        Some(value) if !value.trim().is_empty() => Ok(value),
        Some(_) => Err(ConfigError::Invalid {
            key: key.to_owned(),
            reason: "value cannot be empty".to_owned(),
        }),
        None => Err(ConfigError::Missing(key.to_owned())),
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

    fn source_config(selector_kind: &str) -> NodeConfig {
        NodeConfig::new()
            .with("session_id", "1")
            .with("stem_id", "2")
            .with("selector_kind", selector_kind)
            .with("selector_value", "selected-source")
    }

    fn endpoint_config() -> NodeConfig {
        NodeConfig::new()
            .with("session_id", "1")
            .with("stem_id", "2")
            .with("endpoint_id", "3")
            .with("route_id", "4")
            .with("operator_id", "example.connector.v1")
            .with("connector_id", "5")
    }

    fn frame() -> AudioFrame {
        let pool = AudioBufferPool::new(1, 4);
        let buffer = pool.acquire().expect("buffer");
        AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer)
    }

    #[test]
    fn given_empty_registry_when_session_nodes_registered_then_all_fixed_types_exist() {
        let mut registry = NodeRegistry::new();

        register_session_structural_nodes(&mut registry).expect("registration");

        assert_eq!(registry.len(), STRUCTURAL_NODE_TYPE_IDS.len());
        for node_type_id in STRUCTURAL_NODE_TYPE_IDS {
            assert!(registry.contains(&NodeTypeId::from(node_type_id)));
        }
    }

    #[test]
    fn given_duplicate_type_when_session_nodes_registered_then_registry_is_unchanged() {
        let mut registry = NodeRegistry::new();
        registry
            .register(Arc::new(SourceIngressFactory::application(sample_spec())))
            .unwrap();
        let initial_len = registry.len();

        let result = register_session_structural_nodes(&mut registry);

        assert!(matches!(
            result,
            Err(SessionStructuralNodeRegistrationError::DuplicateNodeType { .. })
        ));
        assert_eq!(registry.len(), initial_len);
        assert!(!registry.contains(&NodeTypeId::from(MICROPHONE_SOURCE_NODE_TYPE_ID)));
    }

    #[test]
    fn given_application_ingress_when_frame_processed_then_identity_is_forwarded() {
        let factory = SourceIngressFactory::application(sample_spec());
        let mut node = factory
            .instantiate(&prepare_context(), &source_config("stable_id"))
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
    fn given_external_boundary_when_registered_then_definition_exists_without_runtime_factory() {
        let mut registry = NodeRegistry::new();
        register_session_structural_nodes(&mut registry).expect("registration");
        let node_type_id = NodeTypeId::from(CONNECTOR_NODE_TYPE_ID);

        let definition = registry.definition(&node_type_id).expect("definition");

        assert!(definition.validate_config(&endpoint_config()).is_ok());
        assert!(registry.get(&node_type_id).is_none());
    }

    #[test]
    fn given_invalid_selector_kind_when_source_validated_then_config_error_is_returned() {
        let factory = SourceIngressFactory::microphone(sample_spec());

        let result = factory.validate_config(&source_config("bundle_id"));

        assert!(matches!(result, Err(ConfigError::Invalid { .. })));
    }
}

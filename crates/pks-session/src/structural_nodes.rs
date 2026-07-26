use std::sync::Arc;

use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
use pks_frame::{AudioFrame, SampleFormat};
use pks_graph::{
    ConfigError, ExecutionPartition, NodeConfig, NodeDescriptor, NodeError, NodeFactory,
    NodeRegistry, NodeTypeId, PrepareContext, RuntimeNode,
};

use crate::{
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
    for node_type_id in STRUCTURAL_NODE_TYPE_IDS {
        if registry.contains(&NodeTypeId::from(node_type_id)) {
            return Err(SessionStructuralNodeRegistrationError::DuplicateNodeType {
                node_type_id: node_type_id.to_owned(),
            });
        }
    }

    registry.register(Arc::new(SourceIngressFactory::application()));
    registry.register(Arc::new(SourceIngressFactory::microphone()));
    registry.register(Arc::new(ExternalBoundaryFactory::connector()));
    registry.register(Arc::new(ExternalBoundaryFactory::browser()));
    registry.register(Arc::new(ExternalBoundaryFactory::recorder()));
    Ok(())
}

#[derive(Clone, Copy)]
enum SourceIngressKind {
    Application,
    Microphone,
}

struct SourceIngressFactory {
    kind: SourceIngressKind,
}

impl SourceIngressFactory {
    const fn application() -> Self {
        Self {
            kind: SourceIngressKind::Application,
        }
    }

    const fn microphone() -> Self {
        Self {
            kind: SourceIngressKind::Microphone,
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
            outputs: vec![audio_port(PortDirection::Output, self.channel_layout())],
            execution: ExecutionPartition::RealtimeCpu,
            realtime_safe: true,
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
}

impl ExternalBoundaryFactory {
    const fn connector() -> Self {
        Self {
            node_type_id: CONNECTOR_NODE_TYPE_ID,
            display_name: "External Connector Boundary",
            required_configuration_keys: &["connector_id"],
        }
    }

    const fn browser() -> Self {
        Self {
            node_type_id: BROWSER_NODE_TYPE_ID,
            display_name: "Remote Browser Boundary",
            required_configuration_keys: &["receiver_uri"],
        }
    }

    const fn recorder() -> Self {
        Self {
            node_type_id: RECORDER_NODE_TYPE_ID,
            display_name: "Multistem Recording Boundary",
            required_configuration_keys: &["stem_name", "recording_group_id"],
        }
    }
}

impl NodeFactory for ExternalBoundaryFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(self.node_type_id),
            display_name: self.display_name,
            inputs: vec![audio_port(PortDirection::Input, ChannelLayout::Any)],
            outputs: Vec::new(),
            execution: ExecutionPartition::AsyncWorker,
            realtime_safe: false,
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

    fn instantiate(
        &self,
        _context: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        self.validate_config(config)?;
        Err(NodeError::ExternalBoundaryExecution {
            node_type_id: NodeTypeId::from(self.node_type_id),
        })
    }
}

fn audio_port(direction: PortDirection, channel_layout: ChannelLayout) -> PortSpec {
    PortSpec {
        name: AUDIO_PORT.to_owned(),
        direction,
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(48_000),
            frame_samples: None,
            channel_layout,
            format: SampleFormat::F32Interleaved,
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
    use pks_frame::{AudioBufferPool, SampleSpec, SourceId, StreamId};

    use super::*;

    fn prepare_context() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
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
        registry.register(Arc::new(SourceIngressFactory::application()));
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
        let factory = SourceIngressFactory::application();
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
    fn given_external_boundary_when_runtime_instantiation_attempted_then_typed_error_is_returned() {
        let factory = ExternalBoundaryFactory::connector();

        let result = factory.instantiate(&prepare_context(), &endpoint_config());

        assert!(matches!(
            result,
            Err(NodeError::ExternalBoundaryExecution { node_type_id })
                if node_type_id == NodeTypeId::from(CONNECTOR_NODE_TYPE_ID)
        ));
    }

    #[test]
    fn given_invalid_selector_kind_when_source_validated_then_config_error_is_returned() {
        let factory = SourceIngressFactory::microphone();

        let result = factory.validate_config(&source_config("bundle_id"));

        assert!(matches!(result, Err(ConfigError::Invalid { .. })));
    }
}

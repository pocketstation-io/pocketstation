use std::collections::HashMap;
use std::sync::Arc;

use pocketstation_caps::{EdgeContract, PortSpec};
use pocketstation_frame::SampleSpec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeTypeId(Arc<str>);

impl NodeTypeId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for NodeTypeId {
    fn from(value: &str) -> Self {
        Self(Arc::from(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Source,
    Transform,
    Policy,
    Model,
    Transport,
    Sink,
}

impl NodeKind {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Source | Self::Sink)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionClass {
    AudioCallback, // platform capture/playback callback thread; alloc/lock/log-free (LAW 15)
    RealtimeCpu,   // processing-thread DSP; alloc-free per tick (LAW 15)
    AsyncIo,       // file/socket IO off the realtime path
    Network,       // relay/transport sockets; never on a realtime executor
    ModelLocal,    // on-device inference; processing thread, not the callback
    ModelRemote,   // cloud model adapter; bounded edge channels only
    Recorder,      // disk/stem writers; may block, never realtime
    ControlPlane,  // policy/orchestration; not per-frame
}

impl ExecutionClass {
    pub fn is_realtime(self) -> bool {
        matches!(self, Self::AudioCallback | Self::RealtimeCpu)
    }

    pub fn rank(self) -> u8 {
        match self {
            Self::AudioCallback => 0,
            Self::RealtimeCpu => 1,
            Self::AsyncIo => 2,
            Self::Network => 3,
            Self::ModelLocal => 4,
            Self::ModelRemote => 5,
            Self::Recorder => 6,
            Self::ControlPlane => 7,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NodeConfig {
    values: HashMap<String, String>,
}

impl NodeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, key: &str, value: &str) -> Self {
        self.values.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn get_f32(&self, key: &str) -> Option<f32> {
        self.get(key).and_then(|raw| raw.parse().ok())
    }

    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.get(key).and_then(|raw| raw.parse().ok())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required config key: {0}")]
    Missing(String),
    #[error("invalid config '{key}': {reason}")]
    Invalid { key: String, reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("node prepare failed: {0}")]
    Prepare(String),
    #[error("node process failed: {0}")]
    Process(String),
    #[error(transparent)]
    Config(#[from] ConfigError),
}

#[derive(Debug, Clone)]
pub struct NodeDescriptor {
    pub type_id: NodeTypeId,
    pub display_name: &'static str,
    pub kind: NodeKind,
    pub inputs: Vec<PortSpec>,
    pub outputs: Vec<PortSpec>,
    pub execution: ExecutionClass,
    pub realtime_safe: bool,
    pub stateful: bool,
}

#[derive(Debug, Clone)]
pub struct PrepareContext {
    pub sample_spec: SampleSpec,
    pub input_contracts: Vec<EdgeContract>,
    pub output_contracts: Vec<EdgeContract>,
}

impl PrepareContext {
    pub fn new(sample_spec: SampleSpec) -> Self {
        Self {
            sample_spec,
            input_contracts: Vec::new(),
            output_contracts: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_execution_classes_when_is_realtime_then_only_callback_and_realtime_cpu_are_true() {
        assert!(ExecutionClass::AudioCallback.is_realtime());
        assert!(ExecutionClass::RealtimeCpu.is_realtime());
        assert!(!ExecutionClass::Network.is_realtime());
        assert!(!ExecutionClass::ModelRemote.is_realtime());
    }

    #[test]
    fn given_execution_classes_when_ranked_then_ordered_callback_first_control_plane_last() {
        assert_eq!(ExecutionClass::AudioCallback.rank(), 0);
        assert_eq!(ExecutionClass::RealtimeCpu.rank(), 1);
        assert!(ExecutionClass::RealtimeCpu.rank() < ExecutionClass::AsyncIo.rank());
        assert!(ExecutionClass::Network.rank() < ExecutionClass::ModelLocal.rank());
        assert_eq!(ExecutionClass::ControlPlane.rank(), 7);
    }

    #[test]
    fn given_node_kinds_when_is_terminal_then_only_source_and_sink_are_true() {
        assert!(NodeKind::Source.is_terminal());
        assert!(NodeKind::Sink.is_terminal());
        assert!(!NodeKind::Transform.is_terminal());
        assert!(!NodeKind::Model.is_terminal());
    }

    #[test]
    fn given_config_builder_when_with_then_values_are_retrievable() {
        let config = NodeConfig::new()
            .with("gain_db", "6.0")
            .with("mode", "voice");
        assert_eq!(config.get("gain_db"), Some("6.0"));
        assert_eq!(config.get("mode"), Some("voice"));
        assert_eq!(config.get("absent"), None);
    }

    #[test]
    fn given_numeric_config_when_get_f32_then_parses_value() {
        let config = NodeConfig::new().with("gain_db", "-12.5");
        assert_eq!(config.get_f32("gain_db"), Some(-12.5));
    }

    #[test]
    fn given_non_numeric_config_when_get_f32_then_returns_none() {
        let config = NodeConfig::new().with("gain_db", "loud");
        assert_eq!(config.get_f32("gain_db"), None);
    }

    #[test]
    fn given_numeric_config_when_get_u32_then_parses_value() {
        let config = NodeConfig::new().with("attack_ms", "40");
        assert_eq!(config.get_u32("attack_ms"), Some(40));
    }

    #[test]
    fn given_non_numeric_config_when_get_u32_then_returns_none() {
        let config = NodeConfig::new().with("attack_ms", "fast");
        assert_eq!(config.get_u32("attack_ms"), None);
    }
}

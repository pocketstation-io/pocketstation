use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::frame::SampleSpec;

use crate::graph::contracts::{EdgeContract, PortSpec};
use crate::graph::partition::{ExecutionPartition, SafetyContract};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeTypeId(Arc<str>);

impl NodeTypeId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NodeTypeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<&str> for NodeTypeId {
    fn from(value: &str) -> Self {
        Self(Arc::from(value))
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
    #[error("node process exceeded its {timeout_ms} ms deadline")]
    ProcessTimeout { timeout_ms: u32 },
    #[error(
        "external boundary node type '{node_type_id}' must execute through its endpoint driver"
    )]
    ExternalBoundaryExecution { node_type_id: NodeTypeId },
    #[error(transparent)]
    Config(#[from] ConfigError),
}

#[derive(Debug, Clone)]
pub struct NodeDescriptor {
    pub type_id: NodeTypeId,
    pub display_name: &'static str,
    pub inputs: Vec<PortSpec>,
    pub outputs: Vec<PortSpec>,
    pub execution: ExecutionPartition,
    pub safety: SafetyContract,
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

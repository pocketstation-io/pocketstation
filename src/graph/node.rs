use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::frame::SampleSpec;

use crate::graph::partition::{ExecutionPartition, SafetyContract};
use crate::graph::ports::{EdgeContract, MediaCaps, PortDirection, PortSpec};
use crate::graph::signal::SignalSpec;
use crate::graph::EdgeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[doc = "Uniquely identifies node type."]
pub struct NodeTypeId(Arc<str>);

impl NodeTypeId {
    #[doc = "Returns the stable string representation of `NodeTypeId`."]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Reports whether this value follows the portable node-type syntax.
    ///
    /// Version 1 keeps construction infallible for source compatibility.
    /// Node labels are lowercase and dot-separated; internal structural names
    /// use underscores while externally owned contract labels may use hyphens.
    pub fn is_well_formed(&self) -> bool {
        crate::graph::identifier::is_node_type_id(self.as_str())
    }
}

impl fmt::Display for NodeTypeId {
    #[doc = "Formats `NodeTypeId` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<&str> for NodeTypeId {
    #[doc = "Converts the supplied value into `NodeTypeId`."]
    fn from(value: &str) -> Self {
        Self(Arc::from(value))
    }
}

#[derive(Clone, Default)]
#[doc = "Configures node."]
pub struct NodeConfig {
    values: HashMap<String, NodeConfigValue>,
}

#[derive(Clone)]
struct NodeConfigValue {
    value: String,
    sensitive: bool,
}

impl Drop for NodeConfigValue {
    fn drop(&mut self) {
        if self.sensitive {
            crate::secret::clear_string(&mut self.value);
        }
    }
}

impl NodeConfig {
    #[doc = "Creates a new `NodeConfig`."]
    pub fn new() -> Self {
        Self::default()
    }

    #[doc = "Returns `NodeConfig` with the supplied entry applied."]
    pub fn with(mut self, key: &str, value: &str) -> Self {
        self.values.insert(
            key.to_owned(),
            NodeConfigValue {
                value: value.to_owned(),
                sensitive: false,
            },
        );
        self
    }

    /// Adds a setup-time value whose normal debug representation is redacted.
    ///
    /// The value remains available to the owning non-realtime factory through
    /// [`Self::get`]. Callers must not copy it into errors, logs, or metrics.
    pub fn with_sensitive(mut self, key: &str, value: &str) -> Self {
        self.values.insert(
            key.to_owned(),
            NodeConfigValue {
                value: value.to_owned(),
                sensitive: true,
            },
        );
        self
    }

    #[doc = "Returns the value held by `NodeConfig`."]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|entry| entry.value.as_str())
    }

    #[doc = "Reports whether sensitive is true for `NodeConfig`."]
    pub fn is_sensitive(&self, key: &str) -> bool {
        self.values.get(key).is_some_and(|entry| entry.sensitive)
    }

    #[doc = "Returns the get f32 held by `NodeConfig`."]
    pub fn get_f32(&self, key: &str) -> Option<f32> {
        self.get(key).and_then(|raw| raw.parse().ok())
    }

    #[doc = "Returns the get u32 held by `NodeConfig`."]
    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.get(key).and_then(|raw| raw.parse().ok())
    }

    #[doc = "Iterates over the values held by `NodeConfig`."]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .map(|(key, entry)| (key.as_str(), entry.value.as_str()))
    }
}

impl fmt::Debug for NodeConfig {
    #[doc = "Formats `NodeConfig` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugValue<'a>(&'a NodeConfigValue);

        impl fmt::Debug for DebugValue<'_> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.0.sensitive {
                    formatter.write_str("<redacted>")
                } else {
                    self.0.value.fmt(formatter)
                }
            }
        }

        formatter
            .debug_map()
            .entries(
                self.values
                    .iter()
                    .map(|(key, value)| (key, DebugValue(value))),
            )
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures surfaced by config operations."]
pub enum ConfigError {
    #[error("missing required config key: {0}")]
    #[doc = "Reports that a required value is missing."]
    Missing(String),
    #[error("invalid config '{key}': {reason}")]
    #[doc = "Reports that validation rejected the supplied value."]
    Invalid {
        #[doc = "Stores the key text reported by `Invalid`."]
        key: String,
        #[doc = "Carries the reason reported by `Invalid`."]
        reason: String,
    },
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures surfaced by node operations."]
pub enum NodeError {
    #[error("node prepare failed: {0}")]
    #[doc = "Classifies a failure at the prepare stage or component of `NodeError`."]
    Prepare(String),
    #[error("node process failed: {0}")]
    #[doc = "Classifies a failure at the process stage or component of `NodeError`."]
    Process(String),
    #[error("node process exceeded its {timeout_ms} ms deadline")]
    #[doc = "Reports that process exceeded its deadline."]
    ProcessTimeout {
        #[doc = "Stores the timeout value for `ProcessTimeout`, in milliseconds."]
        timeout_ms: u32,
    },
    #[error(
        "external boundary node type '{node_type_id}' must execute through its endpoint driver"
    )]
    #[doc = "Classifies a failure at the external boundary execution stage or component of `NodeError`."]
    ExternalBoundaryExecution {
        #[doc = "Identifies the node type identifier recorded by `ExternalBoundaryExecution`."]
        node_type_id: NodeTypeId,
    },
    #[error(transparent)]
    #[doc = "Classifies a failure at the config stage or component of `NodeError`."]
    Config(#[from] ConfigError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Declares a graph node's stable type identity, ports, execution partition, and safety contract."]
pub struct NodeDescriptor {
    pub(crate) type_id: NodeTypeId,
    pub(crate) display_name: &'static str,
    pub(crate) inputs: Vec<PortSpec>,
    pub(crate) outputs: Vec<PortSpec>,
    pub(crate) execution: ExecutionPartition,
    pub(crate) safety: SafetyContract,
    pub(crate) stateful: bool,
}

impl NodeDescriptor {
    #[doc = "Creates a new `NodeDescriptor`."]
    pub fn new(
        type_id: NodeTypeId,
        display_name: &'static str,
        inputs: Vec<PortSpec>,
        outputs: Vec<PortSpec>,
        execution: ExecutionPartition,
        safety: SafetyContract,
        stateful: bool,
    ) -> Result<Self, NodeDescriptorError> {
        if type_id.as_str().trim().is_empty() {
            return Err(NodeDescriptorError::EmptyTypeId);
        }
        if display_name.trim().is_empty() {
            return Err(NodeDescriptorError::EmptyDisplayName);
        }
        if !safety.is_valid_for(execution) {
            return Err(NodeDescriptorError::InvalidSafetyContract);
        }
        if inputs
            .iter()
            .any(|port| port.direction() != PortDirection::Input)
            || outputs
                .iter()
                .any(|port| port.direction() != PortDirection::Output)
        {
            return Err(NodeDescriptorError::PortDirectionMismatch);
        }
        let mut names = std::collections::HashSet::new();
        if inputs
            .iter()
            .chain(outputs.iter())
            .any(|port| !names.insert((port.direction(), port.name().to_owned())))
        {
            return Err(NodeDescriptorError::DuplicatePort);
        }
        Ok(Self {
            type_id,
            display_name,
            inputs,
            outputs,
            execution,
            safety,
            stateful,
        })
    }

    #[doc = "Returns the type identifier held by `NodeDescriptor`."]
    pub const fn type_id(&self) -> &NodeTypeId {
        &self.type_id
    }

    #[doc = "Returns the display name held by `NodeDescriptor`."]
    pub const fn display_name(&self) -> &'static str {
        self.display_name
    }

    #[doc = "Returns the inputs held by `NodeDescriptor`."]
    pub fn inputs(&self) -> &[PortSpec] {
        &self.inputs
    }

    #[doc = "Returns the outputs held by `NodeDescriptor`."]
    pub fn outputs(&self) -> &[PortSpec] {
        &self.outputs
    }

    #[doc = "Returns the execution held by `NodeDescriptor`."]
    pub const fn execution(&self) -> ExecutionPartition {
        self.execution
    }

    #[doc = "Returns the safety held by `NodeDescriptor`."]
    pub const fn safety(&self) -> SafetyContract {
        self.safety
    }

    #[doc = "Reports whether stateful is true for `NodeDescriptor`."]
    pub const fn is_stateful(&self) -> bool {
        self.stateful
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures surfaced by node descriptor operations."]
pub enum NodeDescriptorError {
    #[error("node type id cannot be empty")]
    #[doc = "Reports that type identifier is empty."]
    EmptyTypeId,
    #[error("node display name cannot be empty")]
    #[doc = "Reports that display name is empty."]
    EmptyDisplayName,
    #[error("node safety contract does not match its execution partition")]
    #[doc = "Reports that the supplied safety contract is invalid."]
    InvalidSafetyContract,
    #[error("node port is stored under the wrong direction")]
    #[doc = "Reports that port direction does not match the expected contract."]
    PortDirectionMismatch,
    #[error("node has a duplicate named port in one direction")]
    #[doc = "Reports that port duplicates an existing declaration or record."]
    DuplicatePort,
}

#[derive(Debug, Clone)]
#[doc = "Carries the inputs and runtime context required to prepare."]
pub struct PrepareContext {
    #[doc = "Declares the sample rate, channel layout, and format used by `PrepareContext`."]
    pub sample_spec: SampleSpec,
}

impl PrepareContext {
    #[doc = "Creates a new `PrepareContext`."]
    pub fn new(sample_spec: SampleSpec) -> Self {
        Self { sample_spec }
    }
}

/// Exact graph-owned contract for one prepared node port.
///
/// Realtime nodes, asynchronous operators, sources, and endpoints may wrap
/// this value with lifecycle-specific context, but they do not redefine edge
/// identity, signal/media, capacity, or delivery policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortPrepareContext {
    edge_id: Option<EdgeId>,
    port_name: String,
    direction: PortDirection,
    signal: SignalSpec,
    media: MediaCaps,
    edge_contract: EdgeContract,
    capacity_signals: usize,
}

impl PortPrepareContext {
    #[doc = "Creates a new `PortPrepareContext`."]
    pub fn new(
        edge_id: Option<EdgeId>,
        port_name: impl Into<String>,
        direction: PortDirection,
        signal: SignalSpec,
        media: MediaCaps,
        edge_contract: EdgeContract,
        capacity_signals: usize,
    ) -> Result<Self, NodeError> {
        let port_name = port_name.into();
        if port_name.trim().is_empty() {
            return Err(NodeError::Prepare(
                "prepared port name cannot be empty".to_owned(),
            ));
        }
        if capacity_signals == 0 {
            return Err(NodeError::Prepare(format!(
                "prepared port '{port_name}' has zero capacity"
            )));
        }
        signal
            .validate()
            .map_err(|error| NodeError::Prepare(error.to_string()))?;
        if !media.supports_signal(&signal) {
            return Err(NodeError::Prepare(format!(
                "prepared port '{port_name}' has incompatible signal/media"
            )));
        }
        if !edge_contract.media.is_compatible_with(&media) {
            return Err(NodeError::Prepare(format!(
                "prepared port '{port_name}' has incompatible edge media"
            )));
        }
        Ok(Self {
            edge_id,
            port_name,
            direction,
            signal,
            media,
            edge_contract,
            capacity_signals,
        })
    }

    #[doc = "Returns the edge identifier held by `PortPrepareContext`."]
    pub const fn edge_id(&self) -> Option<EdgeId> {
        self.edge_id
    }

    #[doc = "Returns the port name held by `PortPrepareContext`."]
    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    #[doc = "Returns the direction held by `PortPrepareContext`."]
    pub const fn direction(&self) -> PortDirection {
        self.direction
    }

    #[doc = "Returns the signal held by `PortPrepareContext`."]
    pub const fn signal(&self) -> &SignalSpec {
        &self.signal
    }

    #[doc = "Returns the media held by `PortPrepareContext`."]
    pub const fn media(&self) -> MediaCaps {
        self.media
    }

    #[doc = "Returns the edge contract held by `PortPrepareContext`."]
    pub const fn edge_contract(&self) -> EdgeContract {
        self.edge_contract
    }

    #[doc = "Returns the capacity signals held by `PortPrepareContext`."]
    pub const fn capacity_signals(&self) -> usize {
        self.capacity_signals
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

    #[test]
    fn given_provider_owned_configuration_key_when_stored_then_it_remains_opaque() {
        let config = NodeConfig::new().with("provider.api-key", "opaque");

        assert_eq!(config.get("provider.api-key"), Some("opaque"));
    }
}

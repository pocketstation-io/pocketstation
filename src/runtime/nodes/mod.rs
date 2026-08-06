mod bridge_sink;
mod mixer;
mod synthetic_source;
mod system_source;

use std::sync::Arc;

use crate::graph::{register_builtins, NodeRegistrationError, NodeRegistry};

pub use bridge_sink::{BridgeSinkFactory, BridgeSinkNode, BridgeSinkTelemetry};
pub use mixer::{MixerSourceFactory, MixerSourceNode, MixerTelemetry};
pub use synthetic_source::{SyntheticSourceFactory, SyntheticSourceNode};
pub use system_source::{SystemOutputSourceFactory, SystemOutputSourceNode, SystemOutputTelemetry};

pub fn register_runtime_nodes(registry: &mut NodeRegistry) -> Result<(), NodeRegistrationError> {
    register_builtins(registry)?;
    registry.register(Arc::new(SyntheticSourceFactory))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NodeTypeId;

    #[test]
    fn given_empty_registry_when_runtime_nodes_registered_then_required_types_exist() {
        let mut registry = NodeRegistry::new();

        register_runtime_nodes(&mut registry).unwrap();

        for type_id in [
            "passthrough",
            "gain",
            "source.synthetic",
            "transform.mono_mix",
        ] {
            assert!(
                registry.contains(&NodeTypeId::from(type_id)),
                "registry missing {type_id}"
            );
        }
    }
}

use std::collections::HashMap;
use std::sync::Arc;

use crate::node::{ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PrepareContext};
use crate::runtime_node::RuntimeNode;

pub trait NodeFactory: Send + Sync {
    fn descriptor(&self) -> NodeDescriptor;
    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError>;
    fn instantiate(
        &self,
        cx: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError>;
}

#[derive(Default)]
pub struct NodeRegistry {
    factories: HashMap<NodeTypeId, Arc<dyn NodeFactory>>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, factory: Arc<dyn NodeFactory>) {
        let type_id = factory.descriptor().type_id;
        self.factories.insert(type_id, factory);
    }

    pub fn get(&self, type_id: &NodeTypeId) -> Option<&Arc<dyn NodeFactory>> {
        self.factories.get(type_id)
    }

    pub fn contains(&self, type_id: &NodeTypeId) -> bool {
        self.factories.contains_key(type_id)
    }

    pub fn len(&self) -> usize {
        self.factories.len()
    }

    pub fn is_empty(&self) -> bool {
        self.factories.is_empty()
    }

    pub fn type_ids(&self) -> impl Iterator<Item = &NodeTypeId> {
        self.factories.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::{register_builtins, PassthroughFactory};

    #[test]
    fn given_registered_factory_when_get_then_returns_some() {
        let mut registry = NodeRegistry::new();
        registry.register(Arc::new(PassthroughFactory));
        let type_id = NodeTypeId::from("passthrough");
        assert!(registry.get(&type_id).is_some());
        assert!(registry.contains(&type_id));
    }

    #[test]
    fn given_empty_registry_when_get_unknown_then_returns_none() {
        let registry = NodeRegistry::new();
        let type_id = NodeTypeId::from("passthrough");
        assert!(registry.get(&type_id).is_none());
        assert!(!registry.contains(&type_id));
        assert!(registry.is_empty());
    }

    #[test]
    fn given_builtins_registered_when_len_then_counts_each_factory() {
        let mut registry = NodeRegistry::new();
        register_builtins(&mut registry);
        assert_eq!(registry.len(), 2);
        assert!(!registry.is_empty());
        assert_eq!(registry.type_ids().count(), 2);
    }
}

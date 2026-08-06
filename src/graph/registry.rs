use std::collections::HashMap;
use std::sync::Arc;

use crate::graph::async_operator::{AsyncOperatorFactory, AsyncOperatorManifestError};
use crate::graph::node::{
    ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PrepareContext,
};
use crate::graph::runtime_node::RuntimeNode;
use crate::graph::OperatorId;

pub trait NodeFactory: Send + Sync {
    fn descriptor(&self) -> NodeDescriptor;
    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError>;
    fn instantiate(
        &self,
        cx: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError>;
}

pub trait NodeDefinition: Send + Sync {
    fn descriptor(&self) -> NodeDescriptor;
    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError>;
}

enum RegistryEntry {
    Runtime(Arc<dyn NodeFactory>),
    Async(Arc<dyn AsyncOperatorFactory>),
    Definition(Arc<dyn NodeDefinition>),
}

pub enum NodeDefinitionRef<'registry> {
    Runtime(&'registry Arc<dyn NodeFactory>),
    Async(&'registry Arc<dyn AsyncOperatorFactory>),
    Definition(&'registry Arc<dyn NodeDefinition>),
}

impl NodeDefinitionRef<'_> {
    pub fn descriptor(&self) -> NodeDescriptor {
        match self {
            Self::Runtime(factory) => factory.descriptor(),
            Self::Async(factory) => factory.manifest().node.clone(),
            Self::Definition(definition) => definition.descriptor(),
        }
    }

    pub fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        match self {
            Self::Runtime(factory) => factory.validate_config(config),
            Self::Async(factory) => factory.validate_config(config),
            Self::Definition(definition) => definition.validate_config(config),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum NodeRegistrationError {
    #[error(transparent)]
    InvalidAsyncManifest(#[from] AsyncOperatorManifestError),
    #[error("node type is already registered: {node_type_id}")]
    DuplicateNodeType { node_type_id: String },
    #[error("operator id is already registered: {operator_id}")]
    DuplicateOperatorId { operator_id: String },
}

#[derive(Default)]
pub struct NodeRegistry {
    entries: HashMap<NodeTypeId, RegistryEntry>,
    async_operator_types: HashMap<OperatorId, NodeTypeId>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, factory: Arc<dyn NodeFactory>) -> Result<(), NodeRegistrationError> {
        let type_id = factory.descriptor().type_id;
        if self.entries.contains_key(&type_id) {
            return Err(NodeRegistrationError::DuplicateNodeType {
                node_type_id: type_id.as_str().to_owned(),
            });
        }
        self.entries
            .insert(type_id, RegistryEntry::Runtime(factory));
        Ok(())
    }

    pub fn register_async(
        &mut self,
        factory: Arc<dyn AsyncOperatorFactory>,
    ) -> Result<(), NodeRegistrationError> {
        factory.manifest().validate()?;
        let operator_id = factory.manifest().operator_id.clone();
        if self.async_operator_types.contains_key(&operator_id) {
            return Err(NodeRegistrationError::DuplicateOperatorId {
                operator_id: operator_id.as_str().to_owned(),
            });
        }
        let type_id = factory.manifest().node.type_id.clone();
        if self.entries.contains_key(&type_id) {
            return Err(NodeRegistrationError::DuplicateNodeType {
                node_type_id: type_id.as_str().to_owned(),
            });
        }
        self.entries
            .insert(type_id.clone(), RegistryEntry::Async(factory));
        self.async_operator_types.insert(operator_id, type_id);
        Ok(())
    }

    pub fn register_definition(
        &mut self,
        definition: Arc<dyn NodeDefinition>,
    ) -> Result<(), NodeRegistrationError> {
        let type_id = definition.descriptor().type_id;
        if self.entries.contains_key(&type_id) {
            return Err(NodeRegistrationError::DuplicateNodeType {
                node_type_id: type_id.as_str().to_owned(),
            });
        }
        self.entries
            .insert(type_id, RegistryEntry::Definition(definition));
        Ok(())
    }

    pub fn get(&self, type_id: &NodeTypeId) -> Option<&Arc<dyn NodeFactory>> {
        match self.entries.get(type_id) {
            Some(RegistryEntry::Runtime(factory)) => Some(factory),
            _ => None,
        }
    }

    pub fn definition(&self, type_id: &NodeTypeId) -> Option<NodeDefinitionRef<'_>> {
        match self.entries.get(type_id)? {
            RegistryEntry::Runtime(factory) => Some(NodeDefinitionRef::Runtime(factory)),
            RegistryEntry::Async(factory) => Some(NodeDefinitionRef::Async(factory)),
            RegistryEntry::Definition(definition) => {
                Some(NodeDefinitionRef::Definition(definition))
            }
        }
    }

    pub fn async_factory(&self, type_id: &NodeTypeId) -> Option<&Arc<dyn AsyncOperatorFactory>> {
        match self.entries.get(type_id) {
            Some(RegistryEntry::Async(factory)) => Some(factory),
            _ => None,
        }
    }

    pub fn async_factory_by_operator(
        &self,
        operator_id: &OperatorId,
    ) -> Option<&Arc<dyn AsyncOperatorFactory>> {
        let node_type_id = self.async_operator_types.get(operator_id)?;
        self.async_factory(node_type_id)
    }

    pub fn async_node_type_id(&self, operator_id: &OperatorId) -> Option<&NodeTypeId> {
        self.async_operator_types.get(operator_id)
    }

    pub fn contains(&self, type_id: &NodeTypeId) -> bool {
        self.entries.contains_key(type_id)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn type_ids(&self) -> impl Iterator<Item = &NodeTypeId> {
        self.entries.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::builtins::{register_builtins, PassthroughFactory};

    #[test]
    fn given_registered_factory_when_get_then_returns_some() {
        let mut registry = NodeRegistry::new();
        registry.register(Arc::new(PassthroughFactory)).unwrap();
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
        register_builtins(&mut registry).unwrap();
        assert_eq!(registry.len(), 3);
        assert!(!registry.is_empty());
        assert_eq!(registry.type_ids().count(), 3);
    }

    #[test]
    fn given_duplicate_runtime_node_when_registered_then_first_authority_is_preserved() {
        let mut registry = NodeRegistry::new();
        registry.register(Arc::new(PassthroughFactory)).unwrap();

        let result = registry.register(Arc::new(PassthroughFactory));

        assert!(matches!(
            result,
            Err(NodeRegistrationError::DuplicateNodeType { .. })
        ));
        assert_eq!(registry.len(), 1);
        assert!(registry.get(&NodeTypeId::from("passthrough")).is_some());
    }
}

use std::collections::HashMap;
use std::sync::Arc;

use crate::graph::node::{
    ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PrepareContext,
};
use crate::graph::runtime_node::RuntimeNode;
use crate::graph::signal::{AsyncOperatorFactory, AsyncOperatorManifestError};
use crate::graph::OperatorId;

#[doc = "Implement this trait to provide node behavior to PocketStation; its methods define the preparation and runtime contract."]
pub trait NodeFactory: Send + Sync {
    #[doc = "Returns the descriptor held by `NodeFactory`."]
    fn descriptor(&self) -> NodeDescriptor;
    #[doc = "Validates config for `NodeFactory`."]
    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError>;
    #[doc = "Instantiates the runtime node described by `NodeFactory`."]
    fn instantiate(
        &self,
        cx: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError>;
}

#[doc = "Implement this trait to provide node definition behavior to PocketStation; its methods define the preparation and runtime contract."]
pub trait NodeDefinition: Send + Sync {
    #[doc = "Returns the descriptor held by `NodeDefinition`."]
    fn descriptor(&self) -> NodeDescriptor;
    #[doc = "Validates config for `NodeDefinition`."]
    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError>;
}

enum RegistryEntry {
    Runtime(Arc<dyn NodeFactory>),
    Async(Arc<dyn AsyncOperatorFactory>),
    Definition(Arc<dyn NodeDefinition>),
}

#[doc = "Enumerates the supported node definition ref cases."]
pub enum NodeDefinitionRef<'registry> {
    #[doc = "Represents the runtime case of `NodeDefinitionRef`."]
    Runtime(&'registry Arc<dyn NodeFactory>),
    #[doc = "Represents the async case of `NodeDefinitionRef`."]
    Async(&'registry Arc<dyn AsyncOperatorFactory>),
    #[doc = "Represents the definition case of `NodeDefinitionRef`."]
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
#[doc = "Classifies failures reported as node registration error."]
pub enum NodeRegistrationError {
    #[error(transparent)]
    #[doc = "Reports invalid async manifest."]
    InvalidAsyncManifest(#[from] AsyncOperatorManifestError),
    #[error("node type is already registered: {node_type_id}")]
    #[doc = "Reports duplicate node type."]
    DuplicateNodeType {
        #[doc = "Identifies the node type identifier recorded by `DuplicateNodeType`."]
        node_type_id: String,
    },
    #[error("operator id is already registered: {operator_id}")]
    #[doc = "Reports duplicate operator identifier."]
    DuplicateOperatorId {
        #[doc = "Identifies the operator identifier recorded by `DuplicateOperatorId`."]
        operator_id: String,
    },
}

#[derive(Default)]
#[doc = "Indexes registered node implementations by their stable identities."]
pub struct NodeRegistry {
    entries: HashMap<NodeTypeId, RegistryEntry>,
    async_operator_types: HashMap<OperatorId, NodeTypeId>,
}

impl NodeRegistry {
    #[doc = "Creates a new `NodeRegistry`."]
    pub fn new() -> Self {
        Self::default()
    }

    #[doc = "Registers a node definition with `NodeRegistry` while preserving unique identities."]
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

    #[doc = "Registers async for `NodeRegistry`."]
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

    #[doc = "Registers definition for `NodeRegistry`."]
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

    #[doc = "Returns the value held by `NodeRegistry`."]
    pub fn get(&self, type_id: &NodeTypeId) -> Option<&Arc<dyn NodeFactory>> {
        match self.entries.get(type_id) {
            Some(RegistryEntry::Runtime(factory)) => Some(factory),
            _ => None,
        }
    }

    #[doc = "Returns the definition held by `NodeRegistry`."]
    pub fn definition(&self, type_id: &NodeTypeId) -> Option<NodeDefinitionRef<'_>> {
        match self.entries.get(type_id)? {
            RegistryEntry::Runtime(factory) => Some(NodeDefinitionRef::Runtime(factory)),
            RegistryEntry::Async(factory) => Some(NodeDefinitionRef::Async(factory)),
            RegistryEntry::Definition(definition) => {
                Some(NodeDefinitionRef::Definition(definition))
            }
        }
    }

    #[doc = "Returns the async factory held by `NodeRegistry`."]
    pub fn async_factory(&self, type_id: &NodeTypeId) -> Option<&Arc<dyn AsyncOperatorFactory>> {
        match self.entries.get(type_id) {
            Some(RegistryEntry::Async(factory)) => Some(factory),
            _ => None,
        }
    }

    #[doc = "Returns the async factory by operator held by `NodeRegistry`."]
    pub fn async_factory_by_operator(
        &self,
        operator_id: &OperatorId,
    ) -> Option<&Arc<dyn AsyncOperatorFactory>> {
        let node_type_id = self.async_operator_types.get(operator_id)?;
        self.async_factory(node_type_id)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn async_node_type_id(&self, operator_id: &OperatorId) -> Option<&NodeTypeId> {
        self.async_operator_types.get(operator_id)
    }

    #[doc = "Returns whether contains is true for `NodeRegistry`."]
    pub fn contains(&self, type_id: &NodeTypeId) -> bool {
        self.entries.contains_key(type_id)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[cfg(any(test, feature = "internal-testing"))]
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

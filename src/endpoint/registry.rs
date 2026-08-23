use std::collections::HashMap;
use std::sync::Arc;

use crate::endpoint::{
    EndpointDriverFactory, EndpointFailure, EndpointPortInput, EndpointPreparationGroup,
    OperatorId, PreparedEndpoint,
};
use crate::graph::NodeTypeId;

struct RegisteredEndpointDriver {
    node_type_id: NodeTypeId,
    factory: Arc<dyn EndpointDriverFactory>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[doc = "Classifies failures reported as endpoint driver registry error."]
pub enum EndpointDriverRegistryError {
    #[error("endpoint operator id cannot be empty")]
    #[doc = "Reports empty operator identifier."]
    EmptyOperatorId,
    #[error("endpoint node type id cannot be empty")]
    #[doc = "Reports empty node type identifier."]
    EmptyNodeTypeId,
    #[error(
        "endpoint driver already registered for operator '{operator_id}' and node type '{node_type_id}'"
    )]
    #[doc = "Reports duplicate."]
    Duplicate {
        #[doc = "Identifies the operator identifier recorded by `Duplicate`."]
        operator_id: String,
        #[doc = "Identifies the node type identifier recorded by `Duplicate`."]
        node_type_id: String,
    },
    #[error(
        "endpoint operator '{operator_id}' is already mapped to node type '{registered_node_type_id}', not '{requested_node_type_id}'"
    )]
    #[doc = "Reports operator node type conflict."]
    OperatorNodeTypeConflict {
        #[doc = "Identifies the operator identifier recorded by `OperatorNodeTypeConflict`."]
        operator_id: String,
        #[doc = "Identifies the registered node type identifier recorded by `OperatorNodeTypeConflict`."]
        registered_node_type_id: String,
        #[doc = "Identifies the requested node type identifier recorded by `OperatorNodeTypeConflict`."]
        requested_node_type_id: String,
    },
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[doc = "Classifies failures reported as endpoint prepare error."]
pub enum EndpointPrepareError {
    #[error("endpoint driver batch cannot be empty")]
    #[doc = "Reports empty batch."]
    EmptyBatch,
    #[error(
        "no endpoint driver registered for operator '{operator_id}' and node type '{node_type_id}'"
    )]
    #[doc = "Reports not registered."]
    NotRegistered {
        #[doc = "Identifies the operator identifier recorded by `NotRegistered`."]
        operator_id: String,
        #[doc = "Identifies the node type identifier recorded by `NotRegistered`."]
        node_type_id: String,
    },
    #[error(transparent)]
    #[doc = "Reports driver."]
    Driver(#[from] EndpointFailure),
}

#[derive(Default)]
pub struct EndpointDriverRegistry {
    registrations: HashMap<OperatorId, RegisteredEndpointDriver>,
}

impl EndpointDriverRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        operator_id: OperatorId,
        node_type_id: NodeTypeId,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<(), EndpointDriverRegistryError> {
        self.validate_registration(&operator_id, &node_type_id)?;
        self.registrations.insert(
            operator_id,
            RegisteredEndpointDriver {
                node_type_id,
                factory,
            },
        );
        Ok(())
    }

    pub(crate) fn validate_registration(
        &self,
        operator_id: &OperatorId,
        node_type_id: &NodeTypeId,
    ) -> Result<(), EndpointDriverRegistryError> {
        if operator_id.as_str().trim().is_empty() {
            return Err(EndpointDriverRegistryError::EmptyOperatorId);
        }
        if node_type_id.as_str().trim().is_empty() {
            return Err(EndpointDriverRegistryError::EmptyNodeTypeId);
        }
        if let Some(registered) = self.registrations.get(operator_id) {
            if &registered.node_type_id == node_type_id {
                return Err(EndpointDriverRegistryError::Duplicate {
                    operator_id: operator_id.as_str().to_owned(),
                    node_type_id: node_type_id.as_str().to_owned(),
                });
            }
            return Err(EndpointDriverRegistryError::OperatorNodeTypeConflict {
                operator_id: operator_id.as_str().to_owned(),
                registered_node_type_id: registered.node_type_id.as_str().to_owned(),
                requested_node_type_id: node_type_id.as_str().to_owned(),
            });
        }
        Ok(())
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn contains(&self, operator_id: &OperatorId, node_type_id: &NodeTypeId) -> bool {
        self.node_type_id(operator_id) == Some(node_type_id)
    }

    pub fn node_type_id(&self, operator_id: &OperatorId) -> Option<&NodeTypeId> {
        self.registrations
            .get(operator_id)
            .map(|registration| &registration.node_type_id)
    }

    pub fn preparation_group(
        &self,
        operator_id: &OperatorId,
        node_type_id: &NodeTypeId,
        route_id: crate::frame::RouteId,
        configuration: &crate::graph::NodeConfig,
    ) -> Result<EndpointPreparationGroup, EndpointPrepareError> {
        let registration = self.registration(operator_id, node_type_id)?;
        Ok(registration
            .factory
            .preparation_group(route_id, configuration)?)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn prepare(
        &self,
        operator_id: &OperatorId,
        node_type_id: &NodeTypeId,
        input: EndpointPortInput,
    ) -> Result<PreparedEndpoint, EndpointPrepareError> {
        self.prepare_batch(operator_id, node_type_id, vec![input])
    }

    pub fn prepare_batch(
        &self,
        operator_id: &OperatorId,
        node_type_id: &NodeTypeId,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<PreparedEndpoint, EndpointPrepareError> {
        if inputs.is_empty() {
            return Err(EndpointPrepareError::EmptyBatch);
        }
        let registration = self.registration(operator_id, node_type_id)?;
        Ok(PreparedEndpoint {
            driver: registration.factory.prepare(inputs)?,
        })
    }

    fn registration(
        &self,
        operator_id: &OperatorId,
        node_type_id: &NodeTypeId,
    ) -> Result<&RegisteredEndpointDriver, EndpointPrepareError> {
        self.registrations
            .get(operator_id)
            .filter(|registration| &registration.node_type_id == node_type_id)
            .ok_or_else(|| EndpointPrepareError::NotRegistered {
                operator_id: operator_id.as_str().to_owned(),
                node_type_id: node_type_id.as_str().to_owned(),
            })
    }
}

#[cfg(test)]
mod tests;

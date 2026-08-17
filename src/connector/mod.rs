mod configuration;
#[cfg(feature = "conformance-fixtures")]
pub mod conformance;
mod error;
mod manifest;
mod observation;
mod policy;
mod readiness;

use std::sync::Arc;

use crate::graph::{ConfigError, NodeConfig, NodeDefinition};
use crate::{EndpointDescriptor, EndpointDriverFactory, EndpointHandle, Session, SessionError};

pub use configuration::{
    ConnectorConfiguration, ConnectorConfigurationConstraint, ConnectorConfigurationError,
    ConnectorConfigurationErrorCode, ConnectorConfigurationField,
    ConnectorConfigurationRequirement, ConnectorConfigurationSchema, ConnectorConfigurationValue,
    ConnectorConfigurationValueKind, ConnectorSecret, ResolvedConnectorConfiguration,
    MAX_CONNECTOR_CONFIGURATION_FIELDS, MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES,
};
pub use error::{
    ConnectorError, ConnectorErrorBuildError, ConnectorErrorCode, ConnectorErrorCodeError,
    ConnectorErrorStage, ConnectorRetryability, MAX_CONNECTOR_ERROR_CODE_BYTES,
    MAX_CONNECTOR_ERROR_MESSAGE_BYTES,
};
pub use manifest::{
    ConnectorCapability, ConnectorManifest, ConnectorManifestError, ConnectorRequirement,
    CONNECTOR_API_REVISION, MAX_CONNECTOR_MANIFEST_ENTRIES, MAX_CONNECTOR_MANIFEST_TEXT_BYTES,
};
pub use observation::{
    ConnectorObservationError, ConnectorObservationHandle, ConnectorObservations,
};
pub use policy::{
    ConnectorDeliveryPolicy, ConnectorPolicyError, ConnectorReadinessPolicy, ConnectorRetryPolicy,
    MAX_CONNECTOR_ATTEMPTS, MAX_CONNECTOR_READINESS_THRESHOLD, MAX_CONNECTOR_TIMEOUT_MS,
    MAX_CONNECTOR_WORKER_QUEUE_ITEMS,
};
pub use readiness::{ConnectorReadiness, ConnectorReadinessTransitionError};

pub struct Connector {
    manifest: Arc<ConnectorManifest>,
    factory: Arc<dyn EndpointDriverFactory>,
}

impl Connector {
    pub fn new(
        manifest: ConnectorManifest,
        factory: Arc<dyn EndpointDriverFactory>,
    ) -> Result<Self, ConnectorManifestError> {
        manifest.validate()?;
        Ok(Self {
            manifest: Arc::new(manifest),
            factory,
        })
    }

    pub fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }
}

#[derive(Clone)]
pub struct RegisteredConnector {
    session_id: crate::SessionId,
    manifest: Arc<ConnectorManifest>,
}

impl RegisteredConnector {
    pub const fn session_id(&self) -> crate::SessionId {
        self.session_id
    }

    pub fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }

    pub fn declare(
        &self,
        session: &Session,
        configuration: ConnectorConfiguration,
    ) -> Result<EndpointHandle, ConnectorDeclarationError> {
        if session.id() != self.session_id {
            return Err(ConnectorDeclarationError::WrongSession {
                registered: self.session_id,
                requested: session.id(),
            });
        }
        let configuration = self.manifest.configuration().resolve(&configuration)?;
        let descriptor = EndpointDescriptor::new(
            self.manifest.node().type_id().clone(),
            self.manifest.operator_id().clone(),
        )
        .with_configuration(configuration.into_endpoint_configuration())
        .with_input_edge(self.manifest.delivery().input_edge());
        Ok(session.declaration.connector_endpoint(descriptor)?)
    }
}

struct ConnectorDefinition {
    manifest: Arc<ConnectorManifest>,
}

impl NodeDefinition for ConnectorDefinition {
    fn descriptor(&self) -> crate::NodeDescriptor {
        self.manifest.node().clone()
    }

    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        self.manifest
            .configuration()
            .resolve_node_config(config)
            .map(|_| ())
            .map_err(|error| ConfigError::Invalid {
                key: error.field().unwrap_or("<configuration>").to_owned(),
                reason: error.code().as_str().to_owned(),
            })
    }
}

impl Session {
    pub fn register_connector(
        &self,
        connector: Connector,
    ) -> Result<RegisteredConnector, ConnectorRegistrationError> {
        connector.manifest.validate()?;
        let operator_id = connector.manifest.operator_id().clone();
        let node_type_id = connector.manifest.node().type_id().clone();

        let mut extensions = self
            .endpoint_extensions
            .lock()
            .map_err(|_| ConnectorRegistrationError::RegistrationStateUnavailable)?;
        let registrations = self
            .endpoint_registrations
            .lock()
            .map_err(|_| ConnectorRegistrationError::RegistrationStateUnavailable)?;

        if extensions
            .iter()
            .any(|entry| entry.operator_id == operator_id)
            || registrations
                .iter()
                .any(|entry| entry.operator_id == operator_id)
        {
            return Err(ConnectorRegistrationError::DuplicateOperatorId {
                operator_id: operator_id.as_str().to_owned(),
            });
        }
        if extensions
            .iter()
            .any(|entry| entry.definition.descriptor().type_id() == &node_type_id)
            || registrations
                .iter()
                .any(|entry| entry.node_type_id == node_type_id)
        {
            return Err(ConnectorRegistrationError::DuplicateNodeTypeId {
                node_type_id: node_type_id.as_str().to_owned(),
            });
        }
        drop(registrations);

        extensions.push(crate::EndpointExtensionRegistration {
            operator_id,
            definition: Arc::new(ConnectorDefinition {
                manifest: Arc::clone(&connector.manifest),
            }),
            factory: connector.factory,
        });
        Ok(RegisteredConnector {
            session_id: self.id(),
            manifest: connector.manifest,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectorRegistrationError {
    #[error(transparent)]
    InvalidManifest(#[from] ConnectorManifestError),
    #[error("connector registration state is unavailable")]
    RegistrationStateUnavailable,
    #[error("connector operator id '{operator_id}' is already registered")]
    DuplicateOperatorId { operator_id: String },
    #[error("connector node type id '{node_type_id}' is already registered")]
    DuplicateNodeTypeId { node_type_id: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectorDeclarationError {
    #[error("connector is registered to Session {registered:?}, not Session {requested:?}")]
    WrongSession {
        registered: crate::SessionId,
        requested: crate::SessionId,
    },
    #[error(transparent)]
    Configuration(#[from] ConnectorConfigurationError),
    #[error(transparent)]
    Session(#[from] SessionError),
}

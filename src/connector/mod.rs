mod configuration;
#[cfg(feature = "conformance-fixtures")]
pub mod conformance;
mod error;
mod manifest;
mod observations;
mod service_status;
mod worker;

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
pub(crate) use observations::ConnectorObservationStore;
pub use observations::{
    ConnectorObservationError, ConnectorObservationHandle, ConnectorObservations,
    ConnectorRuntimeObservations,
};
pub use service_status::{
    ConnectorDeliveryReadiness, ConnectorHealth, ConnectorReadinessPolicy,
    ConnectorReadinessPolicyError, ConnectorRecovery, ConnectorServiceStatus,
    MAX_CONNECTOR_READINESS_THRESHOLD, MAX_CONNECTOR_READINESS_TIMEOUT,
};
pub use worker::{ConnectorContext, ConnectorFactory, ConnectorRunOutcome, ConnectorWorker};

pub struct Connector {
    manifest: Arc<ConnectorManifest>,
    endpoint_factory: Arc<dyn EndpointDriverFactory>,
    observations: ConnectorObservationStore,
}

impl Connector {
    pub fn new(
        manifest: ConnectorManifest,
        factory: Arc<dyn ConnectorFactory>,
    ) -> Result<Self, ConnectorManifestError> {
        manifest.validate()?;
        let observations = ConnectorObservationStore::new();
        let endpoint_factory =
            worker::connector_endpoint_factory(factory, observations.clone(), manifest.readiness());
        Ok(Self {
            manifest: Arc::new(manifest),
            endpoint_factory,
            observations,
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
    observations: ConnectorObservationStore,
}

impl RegisteredConnector {
    pub const fn session_id(&self) -> crate::SessionId {
        self.session_id
    }

    pub fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }

    pub fn observation(
        &self,
        endpoint: EndpointHandle,
    ) -> Result<Option<ConnectorObservationHandle>, ConnectorObservationLookupError> {
        if endpoint.session_id() != self.session_id {
            return Err(ConnectorObservationLookupError::WrongSession {
                registered: self.session_id,
                requested: endpoint.session_id(),
            });
        }
        Ok(self.observations.observation(endpoint.id()))
    }

    pub fn observations(
        &self,
    ) -> Result<Vec<ConnectorRuntimeObservations>, ConnectorObservationError> {
        self.observations.snapshots()
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
        .with_input_edge(self.manifest.input_edge());
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
        self.register_endpoint(
            connector.manifest.operator_id().clone(),
            Arc::new(ConnectorDefinition {
                manifest: Arc::clone(&connector.manifest),
            }),
            connector.endpoint_factory,
        )?;
        Ok(RegisteredConnector {
            session_id: self.id(),
            manifest: connector.manifest,
            observations: connector.observations,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectorRegistrationError {
    #[error(transparent)]
    InvalidManifest(#[from] ConnectorManifestError),
    #[error(transparent)]
    Session(#[from] crate::SessionEndpointError),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ConnectorObservationLookupError {
    #[error("connector is registered to Session {registered:?}, not Session {requested:?}")]
    WrongSession {
        registered: crate::SessionId,
        requested: crate::SessionId,
    },
}

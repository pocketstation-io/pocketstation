//! Connector declarations, typed configuration, workers, transport records,
//! readiness, observations, and endpoint integration.

mod configuration;
mod error;
mod manifest;
mod observations;
mod readiness;
mod sidecar;
mod status;
mod transport;
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
pub use readiness::{
    ConnectorReadinessPolicy, ConnectorReadinessPolicyError, MAX_CONNECTOR_READINESS_THRESHOLD,
    MAX_CONNECTOR_READINESS_TIMEOUT,
};
pub use sidecar::{
    sidecar_connector_factory, SidecarConnectorDriverFactory, CONNECTOR_AUDIO_RECORD_SCHEMA,
    CONNECTOR_AUDIO_RECORD_SIGNAL_ID,
};
pub use status::{
    ConnectorDeliveryReadiness, ConnectorHealth, ConnectorRecovery, ConnectorServiceStatus,
};
pub use transport::{
    ConnectorAudioMetadata, ConnectorAudioRecord, ConnectorAudioRecordError,
    ConnectorConfigurationRecord, ConnectorConfigurationRecordError, CONNECTOR_AUDIO_RECORD_MAJOR,
    CONNECTOR_AUDIO_RECORD_MINOR, CONNECTOR_CONFIGURATION_RECORD_MAJOR,
    CONNECTOR_CONFIGURATION_RECORD_MINOR, MAX_CONNECTOR_AUDIO_RECORD_PORT_BYTES,
    MAX_CONNECTOR_AUDIO_RECORD_SAMPLES,
};
pub use worker::{
    ConnectorContext, ConnectorDeliveryOutcome, ConnectorDriver, ConnectorDriverFactory,
    ConnectorFactory, ConnectorInputDescriptor, ConnectorItem, ConnectorRunOutcome,
    ConnectorWorker,
};

#[doc = "Declares a connector endpoint and the manifest-backed configuration used to instantiate it."]
pub struct Connector {
    manifest: Arc<ConnectorManifest>,
    endpoint_factory: Arc<dyn EndpointDriverFactory>,
    observations: ConnectorObservationStore,
}

impl Connector {
    #[doc = "Creates a new `Connector`."]
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

    /// Builds a connector whose bounded receiver loop is owned by Core.
    ///
    /// Connector authors implement item delivery and provider-specific
    /// state only. Endpoint remains authoritative for preparation, start-gate,
    /// shutdown, join, rollback, and finalization semantics.
    pub fn with_driver(
        manifest: ConnectorManifest,
        factory: Arc<dyn ConnectorDriverFactory>,
    ) -> Result<Self, ConnectorManifestError> {
        manifest.validate()?;
        let manifest = Arc::new(manifest);
        let observations = ConnectorObservationStore::new();
        let endpoint_factory = worker::connector_driver_endpoint_factory(
            factory,
            observations.clone(),
            Arc::clone(&manifest),
        );
        Ok(Self {
            manifest,
            endpoint_factory,
            observations,
        })
    }

    /// Builds an outbound Connector backed by one bounded sidecar process.
    ///
    /// Resolved typed configuration is encoded during transactional prepare;
    /// PCM and typed signals are then delivered through the existing Core
    /// Connector worker. Provider code never runs on a realtime partition.
    pub fn sidecar(
        manifest: ConnectorManifest,
        process: crate::SidecarProcessSpec,
    ) -> Result<Self, ConnectorManifestError> {
        Self::with_driver(manifest, sidecar_connector_factory(process))
    }

    #[doc = "Returns the manifest held by `Connector`."]
    pub fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }
}

#[derive(Clone)]
#[doc = "Retains a connector declaration after its factory has been registered with the node registry."]
pub struct RegisteredConnector {
    session_id: crate::SessionId,
    manifest: Arc<ConnectorManifest>,
    observations: ConnectorObservationStore,
}

impl RegisteredConnector {
    #[doc = "Returns the session identifier held by `RegisteredConnector`."]
    pub const fn session_id(&self) -> crate::SessionId {
        self.session_id
    }

    #[doc = "Returns the manifest held by `RegisteredConnector`."]
    pub fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }

    #[doc = "Returns the current observation exposed by `RegisteredConnector`."]
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

    #[doc = "Returns the observations exposed by `RegisteredConnector`."]
    pub fn observations(
        &self,
    ) -> Result<Vec<ConnectorRuntimeObservations>, ConnectorObservationError> {
        self.observations.snapshots()
    }

    #[doc = "Adds the declaration represented by `RegisteredConnector` to its Session."]
    pub fn declare(
        &self,
        session: &Session,
        configuration: ConnectorConfiguration,
        input_edge: crate::EdgeContract,
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
        .with_input_edge(input_edge);
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
    #[doc = "Registers connector for `Session`."]
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
#[doc = "Classifies failures reported as connector registration error."]
pub enum ConnectorRegistrationError {
    #[error(transparent)]
    #[doc = "Reports invalid manifest."]
    InvalidManifest(#[from] ConnectorManifestError),
    #[error(transparent)]
    #[doc = "Reports session."]
    Session(#[from] crate::SessionEndpointError),
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as connector declaration error."]
pub enum ConnectorDeclarationError {
    #[error("connector is registered to Session {registered:?}, not Session {requested:?}")]
    #[doc = "Reports wrong session."]
    WrongSession {
        #[doc = "Stores the registered used by `WrongSession`."]
        registered: crate::SessionId,
        #[doc = "Stores the requested used by `WrongSession`."]
        requested: crate::SessionId,
    },
    #[error(transparent)]
    #[doc = "Reports configuration."]
    Configuration(#[from] ConnectorConfigurationError),
    #[error(transparent)]
    #[doc = "Reports session."]
    Session(#[from] SessionError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as connector observation lookup error."]
pub enum ConnectorObservationLookupError {
    #[error("connector is registered to Session {registered:?}, not Session {requested:?}")]
    #[doc = "Reports wrong session."]
    WrongSession {
        #[doc = "Stores the registered used by `WrongSession`."]
        registered: crate::SessionId,
        #[doc = "Stores the requested used by `WrongSession`."]
        requested: crate::SessionId,
    },
}

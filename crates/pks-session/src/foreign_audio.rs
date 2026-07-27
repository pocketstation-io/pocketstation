use std::sync::Arc;

use pks_endpoint::EndpointDriverFactory;
use pks_graph::NodeTypeId;
use pks_nodes::PolledAudioEndpointFactory;

use crate::{
    EndpointConfiguration, EndpointHandle, OperatorId, Session, SessionEngineBuilder,
    SessionEngineRegistrationError, SessionError, CONNECTOR_NODE_TYPE_ID,
};

pub use pks_nodes::{
    PolledAudioBatchLease, PolledAudioEndpointConfig, PolledAudioEndpointConfigError,
    PolledAudioFrame, PolledAudioObservations, PolledAudioPollError, PolledAudioReceipt,
};

pub const POLLED_AUDIO_OPERATOR_ID: &str = "io.pocketstation.endpoint.polled-audio.v1";

/// Safe composition owner for application-polled audio.
///
/// The concrete worker remains in `pks-nodes`; this type only keeps its
/// registration authority and safe receipt together so every language adapter
/// consumes the canonical compiled Session path.
pub struct PolledAudioEndpoint {
    factory: Arc<PolledAudioEndpointFactory>,
    receipt: PolledAudioReceipt,
}

impl PolledAudioEndpoint {
    pub fn new(config: PolledAudioEndpointConfig) -> Result<Self, PolledAudioEndpointConfigError> {
        let (factory, receipt) = PolledAudioEndpointFactory::new(config)?;
        Ok(Self {
            factory: Arc::new(factory),
            receipt,
        })
    }

    pub fn receipt(&self) -> PolledAudioReceipt {
        self.receipt.clone()
    }
}

impl Session {
    pub fn polled_audio(&self) -> Result<EndpointHandle, SessionError> {
        self.connector(
            OperatorId::new(POLLED_AUDIO_OPERATOR_ID),
            EndpointConfiguration::new(),
        )
    }
}

impl SessionEngineBuilder {
    pub fn register_polled_audio_endpoint(
        &mut self,
        endpoint: &PolledAudioEndpoint,
    ) -> Result<&mut Self, SessionEngineRegistrationError> {
        let factory: Arc<dyn EndpointDriverFactory> = endpoint.factory.clone();
        self.register_endpoint_driver(
            OperatorId::new(POLLED_AUDIO_OPERATOR_ID),
            NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
            factory,
        )
    }
}

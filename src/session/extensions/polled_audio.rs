//! Session composition for the bounded polled-audio endpoint.
//!
//! The endpoint module owns the concrete driver and queue. Session owns only
//! declaration and registration through the Endpoint APIs.

use crate::endpoint::{PolledAudioEndpoint, POLLED_AUDIO_OPERATOR_ID};
use crate::graph::{EdgeContract, NodeTypeId, RouteSettings};
use crate::session::{
    EndpointConfiguration, EndpointExtensionRegistrationError, EndpointHandle, OperatorId, Session,
    SessionEngineBuilder, SessionError, CONNECTOR_NODE_TYPE_ID,
};

impl Session {
    pub fn polled_audio(&self) -> Result<EndpointHandle, SessionError> {
        self.connector(
            OperatorId::new(POLLED_AUDIO_OPERATOR_ID),
            EndpointConfiguration::new(),
        )
    }

    /// Declares application-polled audio with one explicit bounded input edge.
    pub fn polled_audio_with_input_edge(
        &self,
        input_edge: EdgeContract,
    ) -> Result<EndpointHandle, SessionError> {
        self.connector_endpoint(
            crate::session::EndpointDescriptor::new(
                NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
                OperatorId::new(POLLED_AUDIO_OPERATOR_ID),
            )
            .with_input_edge(input_edge),
        )
    }

    /// Declares application-polled audio with explicit media and delivery settings.
    pub fn polled_audio_with_route_settings(
        &self,
        settings: RouteSettings,
    ) -> Result<EndpointHandle, SessionError> {
        self.connector_endpoint(
            crate::session::EndpointDescriptor::new(
                NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
                OperatorId::new(POLLED_AUDIO_OPERATOR_ID),
            )
            .with_route_settings(settings),
        )
    }
}

impl SessionEngineBuilder {
    pub fn register_polled_audio_endpoint(
        &mut self,
        endpoint: &PolledAudioEndpoint,
    ) -> Result<&mut Self, EndpointExtensionRegistrationError> {
        self.register_audio_endpoint_driver(
            OperatorId::new(POLLED_AUDIO_OPERATOR_ID),
            NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
            endpoint.factory(),
        )
    }
}

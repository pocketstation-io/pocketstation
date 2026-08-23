use std::collections::BTreeMap;

pub use crate::graph::OperatorId;
use crate::graph::{EdgeContract, NodeTypeId};

use crate::session::SessionError;

pub const CONNECTOR_NODE_TYPE_ID: &str = "endpoint.connector.external";
pub const BROWSER_NODE_TYPE_ID: &str = "endpoint.browser.remote";
pub const BROWSER_OPERATOR_ID: &str = "io.pocketstation.browser.webrtc.v1";
pub(crate) const BROWSER_RECEIVER_URI_CONFIGURATION_KEY: &str = "receiver_uri";

#[derive(Clone, Default, PartialEq, Eq)]
#[doc = "Configures endpoint."]
pub struct EndpointConfiguration {
    values: BTreeMap<String, EndpointConfigurationValue>,
}

#[derive(Clone, PartialEq, Eq)]
struct EndpointConfigurationValue {
    value: String,
    sensitive: bool,
}

impl Drop for EndpointConfigurationValue {
    fn drop(&mut self) {
        if self.sensitive {
            crate::secret::clear_string(&mut self.value);
        }
    }
}

impl EndpointConfiguration {
    #[doc = "Creates a new `EndpointConfiguration`."]
    pub fn new() -> Self {
        Self::default()
    }

    #[doc = "Returns `EndpointConfiguration` with the supplied entry applied."]
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.values.insert(
            key.into(),
            EndpointConfigurationValue {
                value: value.into(),
                sensitive: false,
            },
        );
        self
    }

    /// Adds a setup-time value whose normal debug representation is redacted.
    pub fn with_sensitive(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.values.insert(
            key.into(),
            EndpointConfigurationValue {
                value: value.into(),
                sensitive: true,
            },
        );
        self
    }

    #[doc = "Returns the value held by `EndpointConfiguration`."]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|entry| entry.value.as_str())
    }

    #[doc = "Returns whether sensitive applies to `EndpointConfiguration`."]
    pub fn is_sensitive(&self, key: &str) -> bool {
        self.values.get(key).is_some_and(|entry| entry.sensitive)
    }

    #[doc = "Iterates over the values held by `EndpointConfiguration`."]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .map(|(key, entry)| (key.as_str(), entry.value.as_str()))
    }

    pub(crate) fn validate(&self) -> Result<(), SessionError> {
        if let Some((key, _)) = self.values.iter().find(|(key, _)| key.trim().is_empty()) {
            return Err(SessionError::InvalidEndpoint {
                reason: format!("configuration key {key:?} cannot be empty"),
            });
        }
        Ok(())
    }
}

impl std::fmt::Debug for EndpointConfiguration {
    #[doc = "Formats `EndpointConfiguration` with the requested formatter."]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct DebugValue<'a>(&'a EndpointConfigurationValue);

        impl std::fmt::Debug for DebugValue<'_> {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.0.sensitive {
                    formatter.write_str("<redacted>")
                } else {
                    self.0.value.fmt(formatter)
                }
            }
        }

        formatter
            .debug_map()
            .entries(
                self.values
                    .iter()
                    .map(|(key, value)| (key, DebugValue(value))),
            )
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Describes the endpoint descriptor contract."]
pub struct EndpointDescriptor {
    node_type_id: NodeTypeId,
    operator_id: OperatorId,
    configuration: EndpointConfiguration,
    input_edge: Option<EdgeContract>,
}

impl EndpointDescriptor {
    #[doc = "Creates a new `EndpointDescriptor`."]
    pub fn new(node_type_id: NodeTypeId, operator_id: OperatorId) -> Self {
        Self {
            node_type_id,
            operator_id,
            configuration: EndpointConfiguration::new(),
            input_edge: None,
        }
    }

    #[doc = "Sets the configuration on `EndpointDescriptor` and returns the updated value."]
    pub fn with_configuration(mut self, configuration: EndpointConfiguration) -> Self {
        self.configuration = configuration;
        self
    }

    /// Declares the bounded delivery policy for routes entering this endpoint.
    ///
    /// Endpoint packages own this policy. The Session compiler validates and
    /// lowers it without recognizing provider, recorder, or transport names.
    pub fn with_input_edge(mut self, input_edge: EdgeContract) -> Self {
        self.input_edge = Some(input_edge);
        self
    }

    #[doc = "Returns the node type identifier held by `EndpointDescriptor`."]
    pub fn node_type_id(&self) -> &NodeTypeId {
        &self.node_type_id
    }

    #[doc = "Returns the operator identifier held by `EndpointDescriptor`."]
    pub fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

    #[doc = "Returns the configuration held by `EndpointDescriptor`."]
    pub fn configuration(&self) -> &EndpointConfiguration {
        &self.configuration
    }

    #[doc = "Returns the input edge held by `EndpointDescriptor`."]
    pub const fn input_edge(&self) -> Option<EdgeContract> {
        self.input_edge
    }

    pub(crate) fn validate(&self) -> Result<(), SessionError> {
        if self.node_type_id.as_str().trim().is_empty() {
            return Err(SessionError::InvalidEndpoint {
                reason: "node type id cannot be empty".to_owned(),
            });
        }
        if self.operator_id.as_str().trim().is_empty() {
            return Err(SessionError::InvalidEndpoint {
                reason: "operator id cannot be empty".to_owned(),
            });
        }
        self.configuration.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported(
    ) {
        let operator_id: OperatorId = crate::graph::OperatorId::new("connector.example");

        assert_eq!(operator_id.as_str(), "connector.example");
        assert_eq!(OperatorId::syntax_version(), 1);
    }

    #[test]
    fn given_provider_owned_endpoint_key_when_validated_then_core_keeps_it_open() {
        let descriptor = EndpointDescriptor::new(
            NodeTypeId::from(CONNECTOR_NODE_TYPE_ID),
            OperatorId::new("dev.pocketstation.test.endpoint.v1"),
        )
        .with_configuration(EndpointConfiguration::new().with("provider.api-key", "opaque"));

        assert!(descriptor.validate().is_ok());
        assert_eq!(
            descriptor.configuration().get("provider.api-key"),
            Some("opaque")
        );
    }
}

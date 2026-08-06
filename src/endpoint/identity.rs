pub use crate::graph::{OperatorId, OPERATOR_ID_SYNTAX_VERSION};

/// Explicit Session-scoped grouping key for endpoints that share one lifecycle.
///
/// Grouping still requires an exact `OperatorId` and `NodeTypeId` match. This
/// key must never cause unrelated endpoint kinds or Sessions to share an owner.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct EndpointGroupId(String);

impl EndpointGroupId {
    pub fn new(group_id: impl Into<String>) -> Self {
        Self(group_id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

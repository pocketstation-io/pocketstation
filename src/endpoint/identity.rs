pub(crate) use crate::graph::OperatorId;

/// Explicit Session-scoped grouping key for endpoints that share one lifecycle.
///
/// Grouping still requires an exact `OperatorId` and `NodeTypeId` match. This
/// key must never cause unrelated endpoint kinds or Sessions to share an owner.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct EndpointGroupId(String);

impl EndpointGroupId {
    #[doc = "Creates a new `EndpointGroupId`."]
    pub fn new(group_id: impl Into<String>) -> Self {
        Self(group_id.into())
    }

    #[doc = "Returns the stable string representation of `EndpointGroupId`."]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Factory-owned decision for batching endpoint inputs into one lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EndpointPreparationGroup {
    #[doc = "Represents the route case of `EndpointPreparationGroup`."]
    Route(crate::frame::RouteId),
    #[doc = "Represents the shared case of `EndpointPreparationGroup`."]
    Shared(EndpointGroupId),
}

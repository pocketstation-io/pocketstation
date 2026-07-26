/// Version of the serialized operator-identifier syntax.
///
/// Version 1 serializes `OperatorId` as one UTF-8 string. The enclosing
/// `SessionSpec` owns document-version migration; changing this syntax requires
/// a new value here and an explicit migration at that boundary.
pub const OPERATOR_ID_SYNTAX_VERSION: u16 = 1;

/// Open identifier for an endpoint operator implementation.
///
/// This transparent string is provider-neutral and serializes without a
/// wrapper object. Construction remains infallible for source compatibility;
/// registries and Session validation reject empty identifiers at authority
/// boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct OperatorId(String);

impl OperatorId {
    pub fn new(operator_id: impl Into<String>) -> Self {
        Self(operator_id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub const fn syntax_version() -> u16 {
        OPERATOR_ID_SYNTAX_VERSION
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_operator_id_when_serialized_then_version_one_is_a_transparent_string() {
        let operator_id = OperatorId::new("connector.example");

        let encoded = serde_json::to_string(&operator_id).unwrap();
        let decoded: OperatorId = serde_json::from_str(&encoded).unwrap();

        assert_eq!(OperatorId::syntax_version(), 1);
        assert_eq!(encoded, "\"connector.example\"");
        assert_eq!(decoded, operator_id);
    }
}

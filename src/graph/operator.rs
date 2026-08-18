/// Version of the serialized operator-identifier syntax.
///
/// Version 1 serializes `OperatorId` as one UTF-8 string. The enclosing graph
/// or Session document owns migration; changing this syntax requires a new
/// value here and an explicit migration at those authority boundaries.
pub const OPERATOR_ID_SYNTAX_VERSION: u16 = 1;

/// Open identifier for a registered graph operator implementation.
///
/// This transparent string is provider-neutral and serializes without a
/// wrapper object. Construction remains infallible for source compatibility;
/// registries and graph validation reject empty identifiers at authority
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

    /// Reports whether this value is a portable implementation contract ID.
    ///
    /// Portable IDs use reverse-domain ownership and an explicit final
    /// revision segment, for example `io.example.operator.transcribe.v1`.
    pub fn is_portable(&self) -> bool {
        crate::graph::identifier::is_portable_contract_id(self.as_str())
    }

    pub const fn syntax_version() -> u16 {
        OPERATOR_ID_SYNTAX_VERSION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_operator_id_when_serialized_then_version_one_is_a_transparent_string() {
        let operator_id = OperatorId::new("operator.example");

        let encoded = serde_json::to_string(&operator_id).unwrap();
        let decoded: OperatorId = serde_json::from_str(&encoded).unwrap();

        assert_eq!(OperatorId::syntax_version(), 1);
        assert_eq!(encoded, "\"operator.example\"");
        assert_eq!(decoded, operator_id);
    }
}

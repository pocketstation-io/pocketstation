use crate::frame::{EndpointId, SessionId, StemId};

use crate::session::{OperatorInstanceId, SourceInstanceId};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SessionError {
    #[error("session must contain at least one built-in or external source")]
    NoSources,
    #[error("stem {stem_id:?} has no destination route")]
    NoRoutes { stem_id: StemId },
    #[error("external source instance {source_instance_id:?} has no declared output")]
    NoSourceOutputs {
        source_instance_id: SourceInstanceId,
    },
    #[error(
        "external source instance {source_instance_id:?} output '{output_port}' has no destination route"
    )]
    NoSourceOutputRoutes {
        source_instance_id: SourceInstanceId,
        output_port: String,
    },
    #[error("invalid source selector: {reason}")]
    InvalidSelector { reason: String },
    #[error("endpoint descriptor is invalid: {reason}")]
    InvalidEndpoint { reason: String },
    #[error("operator descriptor is invalid: {reason}")]
    InvalidOperator { reason: String },
    #[error("route descriptor is invalid: {reason}")]
    InvalidRoute { reason: String },
    #[error("endpoint belongs to session {actual:?}, expected {expected:?}")]
    ForeignEndpoint {
        expected: SessionId,
        actual: SessionId,
    },
    #[error("session draft {session_id:?} is already frozen")]
    DraftFrozen { session_id: SessionId },
    #[error("session draft lock is poisoned")]
    DraftPoisoned,
    #[error("session identifier space is exhausted")]
    IdExhausted,
    #[error("unsupported SessionSpec version {major}.{minor}")]
    UnsupportedVersion { major: u16, minor: u16 },
    #[error("route references unknown endpoint {endpoint_id:?}")]
    UnknownEndpoint { endpoint_id: EndpointId },
    #[error("route references unknown stem {stem_id:?}")]
    UnknownStem { stem_id: StemId },
    #[error("route references unknown external source instance {source_instance_id:?}")]
    UnknownSourceInstance {
        source_instance_id: SourceInstanceId,
    },
    #[error(
        "route references unknown output '{output_port}' on external source instance {source_instance_id:?}"
    )]
    UnknownSourceOutput {
        source_instance_id: SourceInstanceId,
        output_port: String,
    },
    #[error("route references unknown operator instance {operator_instance_id:?}")]
    UnknownOperatorInstance {
        operator_instance_id: OperatorInstanceId,
    },
    #[error("operator instance {operator_instance_id:?} has no terminal destination")]
    OperatorHasNoDestination {
        operator_instance_id: OperatorInstanceId,
    },
}

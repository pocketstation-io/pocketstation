use pks_frame::{EndpointId, SessionId, StemId};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SessionError {
    #[error("session must contain at least one capture source")]
    NoSources,
    #[error("stem {stem_id:?} has no destination route")]
    NoRoutes { stem_id: StemId },
    #[error("invalid source selector: {reason}")]
    InvalidSelector { reason: String },
    #[error("endpoint descriptor is invalid: {reason}")]
    InvalidEndpoint { reason: String },
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
}

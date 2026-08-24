use crate::frame::{EndpointId, SessionId, StemId};

use crate::session::{OperatorInstanceId, SourceInstanceId};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures surfaced by session operations."]
pub enum SessionError {
    #[error("session must contain at least one built-in or external source")]
    #[doc = "Reports that no sources is available."]
    NoSources,
    #[error("stem {stem_id:?} has no destination route")]
    #[doc = "Reports that no routes is available."]
    NoRoutes {
        #[doc = "Identifies the stem identifier recorded by `NoRoutes`."]
        stem_id: StemId,
    },
    #[error("external source instance {source_instance_id:?} has no declared output")]
    #[doc = "Reports that no source outputs is available."]
    NoSourceOutputs {
        #[doc = "Identifies the source instance identifier recorded by `NoSourceOutputs`."]
        source_instance_id: SourceInstanceId,
    },
    #[error(
        "external source instance {source_instance_id:?} output '{output_port}' has no destination route"
    )]
    #[doc = "Reports that no source output routes is available."]
    NoSourceOutputRoutes {
        #[doc = "Identifies the source instance identifier recorded by `NoSourceOutputRoutes`."]
        source_instance_id: SourceInstanceId,
        #[doc = "References the output port participating in `NoSourceOutputRoutes`."]
        output_port: String,
    },
    #[error("invalid source selector: {reason}")]
    #[doc = "Reports that the supplied selector is invalid."]
    InvalidSelector {
        #[doc = "Carries the reason reported by `InvalidSelector`."]
        reason: String,
    },
    #[error("endpoint descriptor is invalid: {reason}")]
    #[doc = "Reports that the supplied endpoint is invalid."]
    InvalidEndpoint {
        #[doc = "Carries the reason reported by `InvalidEndpoint`."]
        reason: String,
    },
    #[error("operator descriptor is invalid: {reason}")]
    #[doc = "Reports that the supplied operator is invalid."]
    InvalidOperator {
        #[doc = "Carries the reason reported by `InvalidOperator`."]
        reason: String,
    },
    #[error("route descriptor is invalid: {reason}")]
    #[doc = "Reports that the supplied route is invalid."]
    InvalidRoute {
        #[doc = "Carries the reason reported by `InvalidRoute`."]
        reason: String,
    },
    #[error("endpoint belongs to session {actual:?}, expected {expected:?}")]
    #[doc = "Reports that endpoint belongs to a different owning Session or declaration."]
    ForeignEndpoint {
        #[doc = "Records the value expected by `ForeignEndpoint`."]
        expected: SessionId,
        #[doc = "Records the value observed by `ForeignEndpoint`."]
        actual: SessionId,
    },
    #[error("session draft {session_id:?} is already frozen")]
    #[doc = "Classifies a failure at the draft frozen stage or component of `SessionError`."]
    DraftFrozen {
        #[doc = "Identifies the session identifier recorded by `DraftFrozen`."]
        session_id: SessionId,
    },
    #[error("session draft lock is poisoned")]
    #[doc = "Reports that shared draft became unavailable after a panic while locked."]
    DraftPoisoned,
    #[error("session identifier space is exhausted")]
    #[doc = "Reports that the available id range or capacity is exhausted."]
    IdExhausted,
    #[error("unsupported SessionSpec version {major}.{minor}")]
    #[doc = "Reports that the requested version is unsupported."]
    UnsupportedVersion {
        #[doc = "Stores the major component of `UnsupportedVersion`."]
        major: u16,
        #[doc = "Stores the minor component of `UnsupportedVersion`."]
        minor: u16,
    },
    #[error("route references unknown endpoint {endpoint_id:?}")]
    #[doc = "Reports that the referenced endpoint is not declared or registered."]
    UnknownEndpoint {
        #[doc = "Identifies the endpoint identifier recorded by `UnknownEndpoint`."]
        endpoint_id: EndpointId,
    },
    #[error("route references unknown stem {stem_id:?}")]
    #[doc = "Reports that the referenced stem is not declared or registered."]
    UnknownStem {
        #[doc = "Identifies the stem identifier recorded by `UnknownStem`."]
        stem_id: StemId,
    },
    #[error("route references unknown external source instance {source_instance_id:?}")]
    #[doc = "Reports that the referenced source instance is not declared or registered."]
    UnknownSourceInstance {
        #[doc = "Identifies the source instance identifier recorded by `UnknownSourceInstance`."]
        source_instance_id: SourceInstanceId,
    },
    #[error(
        "route references unknown output '{output_port}' on external source instance {source_instance_id:?}"
    )]
    #[doc = "Reports that the referenced source output is not declared or registered."]
    UnknownSourceOutput {
        #[doc = "Identifies the source instance identifier recorded by `UnknownSourceOutput`."]
        source_instance_id: SourceInstanceId,
        #[doc = "References the output port participating in `UnknownSourceOutput`."]
        output_port: String,
    },
    #[error("route references unknown operator instance {operator_instance_id:?}")]
    #[doc = "Reports that the referenced operator instance is not declared or registered."]
    UnknownOperatorInstance {
        #[doc = "Identifies the operator instance identifier recorded by `UnknownOperatorInstance`."]
        operator_instance_id: OperatorInstanceId,
    },
    #[error("operator instance {operator_instance_id:?} has no terminal destination")]
    #[doc = "Classifies a failure at the operator has no destination stage or component of `SessionError`."]
    OperatorHasNoDestination {
        #[doc = "Identifies the operator instance identifier recorded by `OperatorHasNoDestination`."]
        operator_instance_id: OperatorInstanceId,
    },
}

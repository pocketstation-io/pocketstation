use crate::frame::{EndpointId, SessionId, StemId};

use crate::session::{OperatorInstanceId, SourceInstanceId};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as session error."]
pub enum SessionError {
    #[error("session must contain at least one built-in or external source")]
    #[doc = "Reports no sources."]
    NoSources,
    #[error("stem {stem_id:?} has no destination route")]
    #[doc = "Reports no routes."]
    NoRoutes {
        #[doc = "Identifies the stem associated with `NoRoutes`."]
        stem_id: StemId,
    },
    #[error("external source instance {source_instance_id:?} has no declared output")]
    #[doc = "Reports no source outputs."]
    NoSourceOutputs {
        #[doc = "Identifies the source instance associated with `NoSourceOutputs`."]
        source_instance_id: SourceInstanceId,
    },
    #[error(
        "external source instance {source_instance_id:?} output '{output_port}' has no destination route"
    )]
    #[doc = "Reports no source output routes."]
    NoSourceOutputRoutes {
        #[doc = "Identifies the source instance associated with `NoSourceOutputRoutes`."]
        source_instance_id: SourceInstanceId,
        #[doc = "Stores the output port associated with `NoSourceOutputRoutes`."]
        output_port: String,
    },
    #[error("invalid source selector: {reason}")]
    #[doc = "Reports invalid selector."]
    InvalidSelector {
        #[doc = "Carries the reason reported by `InvalidSelector`."]
        reason: String,
    },
    #[error("endpoint descriptor is invalid: {reason}")]
    #[doc = "Reports invalid endpoint."]
    InvalidEndpoint {
        #[doc = "Carries the reason reported by `InvalidEndpoint`."]
        reason: String,
    },
    #[error("operator descriptor is invalid: {reason}")]
    #[doc = "Reports invalid operator."]
    InvalidOperator {
        #[doc = "Carries the reason reported by `InvalidOperator`."]
        reason: String,
    },
    #[error("route descriptor is invalid: {reason}")]
    #[doc = "Reports invalid route."]
    InvalidRoute {
        #[doc = "Carries the reason reported by `InvalidRoute`."]
        reason: String,
    },
    #[error("endpoint belongs to session {actual:?}, expected {expected:?}")]
    #[doc = "Reports foreign endpoint."]
    ForeignEndpoint {
        #[doc = "Records the value expected by `ForeignEndpoint`."]
        expected: SessionId,
        #[doc = "Records the value observed by `ForeignEndpoint`."]
        actual: SessionId,
    },
    #[error("session draft {session_id:?} is already frozen")]
    #[doc = "Reports draft frozen."]
    DraftFrozen {
        #[doc = "Identifies the session associated with `DraftFrozen`."]
        session_id: SessionId,
    },
    #[error("session draft lock is poisoned")]
    #[doc = "Reports draft poisoned."]
    DraftPoisoned,
    #[error("session identifier space is exhausted")]
    #[doc = "Reports id exhausted."]
    IdExhausted,
    #[error("unsupported SessionSpec version {major}.{minor}")]
    #[doc = "Reports unsupported version."]
    UnsupportedVersion {
        #[doc = "Stores the major associated with `UnsupportedVersion`."]
        major: u16,
        #[doc = "Stores the minor associated with `UnsupportedVersion`."]
        minor: u16,
    },
    #[error("route references unknown endpoint {endpoint_id:?}")]
    #[doc = "Reports unknown endpoint."]
    UnknownEndpoint {
        #[doc = "Identifies the endpoint associated with `UnknownEndpoint`."]
        endpoint_id: EndpointId,
    },
    #[error("route references unknown stem {stem_id:?}")]
    #[doc = "Reports unknown stem."]
    UnknownStem {
        #[doc = "Identifies the stem associated with `UnknownStem`."]
        stem_id: StemId,
    },
    #[error("route references unknown external source instance {source_instance_id:?}")]
    #[doc = "Reports unknown source instance."]
    UnknownSourceInstance {
        #[doc = "Identifies the source instance associated with `UnknownSourceInstance`."]
        source_instance_id: SourceInstanceId,
    },
    #[error(
        "route references unknown output '{output_port}' on external source instance {source_instance_id:?}"
    )]
    #[doc = "Reports unknown source output."]
    UnknownSourceOutput {
        #[doc = "Identifies the source instance associated with `UnknownSourceOutput`."]
        source_instance_id: SourceInstanceId,
        #[doc = "Stores the output port associated with `UnknownSourceOutput`."]
        output_port: String,
    },
    #[error("route references unknown operator instance {operator_instance_id:?}")]
    #[doc = "Reports unknown operator instance."]
    UnknownOperatorInstance {
        #[doc = "Identifies the operator instance associated with `UnknownOperatorInstance`."]
        operator_instance_id: OperatorInstanceId,
    },
    #[error("operator instance {operator_instance_id:?} has no terminal destination")]
    #[doc = "Reports operator has no destination."]
    OperatorHasNoDestination {
        #[doc = "Identifies the operator instance associated with `OperatorHasNoDestination`."]
        operator_instance_id: OperatorInstanceId,
    },
}

//! Stable graph-extension contracts.
//!
//! This namespace exposes signal, media, port, partition, and extension
//! contracts. Declaration builders, registries, compiler stages, IR, runtime
//! plans, and executable nodes are private engine implementation.

#[cfg(any(test, feature = "internal-testing"))]
pub mod builtins;
pub(crate) mod compile;
pub(crate) mod dsl;
pub(crate) mod identifier;
pub(crate) mod ir;
#[cfg(test)]
mod named_ports;
pub(crate) mod node;
pub(crate) mod operator;
pub(crate) mod partition;
pub(crate) mod plan;
pub(crate) mod ports;
pub(crate) mod registry;
pub(crate) mod runtime_node;
pub(crate) mod signal;
#[cfg(test)]
mod source;
pub(crate) mod spec;

#[cfg(any(test, feature = "internal-testing"))]
pub use builtins::register_builtins;
pub use dsl::{NodeHandle, Pipeline};
pub use node::{
    ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PortPrepareContext,
    PrepareContext,
};
pub use operator::OperatorId;
#[cfg(any(test, feature = "internal-testing"))]
pub use operator::OPERATOR_ID_SYNTAX_VERSION;
pub use partition::{ExecutionPartition, SafetyContract};
#[cfg(test)]
pub(crate) use ports::DEFAULT_ASYNC_MAX_PAYLOAD_BYTES;
pub use ports::{
    AudioCaps, BackpressurePolicy, ChannelLayout, ClockDomain, CopyPolicy, DeliverySemantics,
    EdgeContract, EdgeObservabilityLevel, LossPolicy, MediaCaps, MediaKind, Multiplicity,
    PortDirection, PortSpec, MAX_ASYNC_PAYLOAD_BYTES,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use registry::NodeDefinitionRef;
pub use registry::{NodeDefinition, NodeFactory, NodeRegistrationError, NodeRegistry};
pub use runtime_node::RuntimeNode;
pub use signal::{
    AsyncNode, AsyncNodeFuture, AsyncOperatorEdgePrepareContext, AsyncOperatorPrepareContext,
    SignalContinuityError, SignalContinuityObservation, SignalContinuityTracker, SignalDerivation,
    SignalDerivationError, SignalEnvelope, SignalEnvelopeError, SignalLineage, SignalLineageError,
    SignalPayload, SignalTiming, SignalTimingError,
};
pub use signal::{
    AsyncOperatorFactory, AsyncOperatorManifest, AsyncOperatorManifestError,
    OperatorCancellationPolicy, OperatorDeadlinePolicy, OperatorFailurePolicy,
    OperatorOutputRolePolicy, OperatorPermissionPolicy,
};
pub use signal::{
    BinaryFormat, Codec, EventFormat, SchemaRef, SemanticRole, SignalClass, SignalId, SignalSpec,
    SignalSpecError, TextFormat,
};
pub use spec::{EdgeId, GraphSpec, NodeId, OutputPortRef};
#[cfg(any(test, feature = "internal-testing"))]
pub use spec::{EdgeSpec, InputPortRef, NodeSpec};

//! Endpoint-driver lifecycle and registry contracts.
//!
//! This crate owns setup-time endpoint state transitions. It does not implement
//! connectors, relay protocols, recording algorithms, or worker threads.

mod driver;
mod identity;
mod registry;

pub use driver::{
    endpoint_start_gate, EndpointCancellationOutcome, EndpointDriverFinalization,
    EndpointDriverObservations, EndpointFailure, EndpointFailureStage, EndpointFinalizationOutcome,
    EndpointPrepareContext, EndpointRouteContext, EndpointSignalRouteContext, EndpointStartFailure,
    EndpointStartFailureCause, EndpointStartGate, EndpointStartGateController, PreparedEndpoint,
    PreparedEndpointDriver, RunningEndpoint, RunningEndpointDriver, SessionTimelineOrigin,
};
pub use identity::{EndpointGroupId, OperatorId, OPERATOR_ID_SYNTAX_VERSION};
pub use registry::{
    DerivedEndpointDriverInput, EndpointDriverFactory, EndpointDriverInput, EndpointDriverRegistry,
    EndpointDriverRegistryError, EndpointPrepareError,
};

// Public extension contract for external endpoint-driver packages. The
// implementation remains in this module; no separately versioned endpoint
// package exists.
pub use crate::graph::{
    EdgeContract, ExecutionPartition, MediaCaps, Multiplicity, NodeConfig as OperatorConfiguration,
    NodeDefinition, NodeDescriptor, NodeTypeId, PortDirection, PortSpec, SafetyContract,
    SignalEnvelope, SignalSpec, TextFormat,
};
pub use crate::runtime::{AsyncOperatorOutputObservationHandle, AsyncOperatorOutputObservations};

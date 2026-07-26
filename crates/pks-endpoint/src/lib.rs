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
    EndpointPrepareContext, EndpointStartFailure, EndpointStartFailureCause, EndpointStartGate,
    EndpointStartGateController, PreparedEndpoint, PreparedEndpointDriver, RunningEndpoint,
    RunningEndpointDriver,
};
pub use identity::{EndpointGroupId, OperatorId, OPERATOR_ID_SYNTAX_VERSION};
pub use registry::{
    EndpointDriverFactory, EndpointDriverInput, EndpointDriverRegistry,
    EndpointDriverRegistryError, EndpointPrepareError,
};

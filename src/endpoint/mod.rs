//! Endpoint-driver lifecycle and registry APIs.
//!
//! This crate owns setup-time endpoint state transitions. It does not implement
//! connectors, relay protocols, recording algorithms, or worker threads.

mod contract;
mod identity;
mod polled_audio;
mod polled_audio_driver;
mod registry;
mod runtime;

pub use contract::{
    EndpointAudioFrame, EndpointAudioReceiver, EndpointDriverFactory, EndpointPortInput,
    EndpointReceiver, EndpointSignalReceiver,
};
pub(crate) use identity::OperatorId;
pub use identity::{EndpointGroupId, EndpointPreparationGroup};
pub use polled_audio::PolledAudioEndpoint;
pub(crate) use polled_audio::POLLED_AUDIO_OPERATOR_ID;
pub use polled_audio_driver::{
    PolledAudioBatchLease, PolledAudioEndpointConfig, PolledAudioEndpointConfigError,
    PolledAudioFrame, PolledAudioObservations, PolledAudioPollError, PolledAudioReceipt,
};
pub(crate) use registry::{
    EndpointDriverRegistry, EndpointDriverRegistryError, EndpointPrepareError,
};
pub(crate) use runtime::{
    endpoint_start_gate, EndpointDriverObservationHandle, EndpointFinalizationOutcome,
    EndpointStartFailure, PreparedEndpoint, RunningEndpoint,
};
pub use runtime::{
    EndpointCancellationOutcome, EndpointDriverFinalization, EndpointDriverObservations,
    EndpointFailure, EndpointFailureRetryability, EndpointFailureStage, EndpointInputOrigin,
    EndpointPrepareContext, EndpointRouteContext, EndpointShutdownMode, EndpointStartGate,
    PreparedEndpointDriver, RunningEndpointDriver, SessionTimelineOrigin,
};

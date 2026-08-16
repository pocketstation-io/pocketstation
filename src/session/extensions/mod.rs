//! Session extension integration.
//!
//! Registration contracts live here; extension implementation and provider
//! policy remain outside the core Session lifecycle.

pub(crate) mod audio_reentry;
#[cfg(test)]
mod external_source_tests;
mod native_library;
mod polled_audio;
mod recording;
pub(crate) mod source;
#[cfg(test)]
mod source_lifecycle_tests;
#[cfg(test)]
mod source_registration_tests;
pub(crate) mod structural_nodes;

pub use recording::{
    session_recording_outcome_error_code, SessionRecordingErrorCode, SessionRecordingObservations,
    SessionRecordingOutcome, SessionRecordingReceipt, SessionRecordingState,
    SessionRecordingStemOutcome,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use recording::{
    DEFAULT_MULTISTEM_RECORDING_GROUP_ID, RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
    RECORDING_GROUP_CONFIGURATION_KEY,
};
pub use source::{
    PreparedSourceRuntime, SourceCancellation, SourceConfiguration, SourceDriver,
    SourceDriverError, SourceEmission, SourceFactory, SourceManifest, SourceManifestError,
    SourceOutputBranchSpec, SourceOutputIdentity, SourcePrepareContext, SourceRegistrationError,
    SourceRegistry, SourceRuntime, SourceRuntimeObservationHandle, SourceRuntimeObservations,
    SourceSessionContext, SourceTypeId,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use source::{SourceOutputReceiver, SourceRuntimeError};
#[cfg(any(test, feature = "internal-testing"))]
pub use structural_nodes::register_session_structural_nodes;
pub use structural_nodes::SessionStructuralNodeRegistrationError;
#[cfg(any(test, feature = "internal-testing"))]
pub use structural_nodes::{APPLICATION_SOURCE_NODE_TYPE_ID, MICROPHONE_SOURCE_NODE_TYPE_ID};

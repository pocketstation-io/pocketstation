//! Session extension integration.
//!
//! Registration contracts live here; extension implementation and provider
//! policy remain outside the core Session lifecycle.

mod audio_input;
pub(crate) mod builtins;
mod native_library;
mod polled_audio;
mod recording;
pub(crate) mod source;
pub(crate) use audio_input::AudioInputFactory;
pub use audio_input::{
    AudioInput, AudioInputBuffer, AudioInputBufferAcquireError, AudioInputBufferError,
    AudioInputConfig, AudioInputConfigError, AudioInputError, AudioInputObservations,
    AudioInputWriteError, AudioInputWriteErrorKind, AudioInputWriter, AudioOutputWriteError,
    AudioOutputWriteErrorKind, PcmSource, PCM_SOURCE_TYPE_ID,
};
pub use builtins::SessionGraphRegistrationError;
#[cfg(any(test, feature = "internal-testing"))]
pub use builtins::{
    register_session_graph_nodes, APPLICATION_SOURCE_NODE_TYPE_ID, MICROPHONE_SOURCE_NODE_TYPE_ID,
};
pub use recording::{
    session_recording_outcome_error_code, SessionRecordingErrorCode, SessionRecordingObservations,
    SessionRecordingOutcome, SessionRecordingReceipt, SessionRecordingState,
    SessionRecordingStemOutcome, DEFAULT_MULTISTEM_RECORDING_GROUP_ID,
    SESSION_RECORDING_MANIFEST_FILE_NAME, SESSION_RECORDING_MANIFEST_SCHEMA_VERSION,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use recording::{
    RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID, RECORDING_GROUP_CONFIGURATION_KEY,
};
pub use source::{
    PreparedSourceRuntime, SourceCancellation, SourceConfiguration, SourceDriver,
    SourceDriverError, SourceEmission, SourceFactory, SourceManifest, SourceManifestError,
    SourceOutputBranchSpec, SourceOutputIdentity, SourcePrepareContext, SourceRegistrationError,
    SourceRegistry, SourceRuntime, SourceRuntimeObservationHandle, SourceRuntimeObservations,
    SourceSessionContext, SourceTypeId, SourceTypeIdError,
};
#[cfg(any(test, feature = "internal-testing"))]
pub use source::{SourceOutputReceiver, SourceRuntimeError};

#[cfg(test)]
mod tests {
    mod composition;
    mod registry;
    mod runtime;
}

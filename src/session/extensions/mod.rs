//! Session extension integration.
//!
//! Registration contracts live here; extension implementation and provider
//! policy remain outside the core Session lifecycle.

pub(crate) mod builtins;
mod native_library;
mod pcm_source;
mod polled_audio;
mod recording;
pub(crate) mod source;
pub use builtins::SessionGraphRegistrationError;
#[cfg(any(test, feature = "internal-testing"))]
pub use builtins::{
    register_session_graph_nodes, APPLICATION_SOURCE_NODE_TYPE_ID, MICROPHONE_SOURCE_NODE_TYPE_ID,
};
pub(crate) use pcm_source::PcmSourceFactory;
pub use pcm_source::{
    PcmBuffer, PcmBufferAcquireError, PcmBufferError, PcmSource, PcmSourceConfig,
    PcmSourceConfigError, PcmSourceError, PcmSourceObservations, PcmSourceWriter, PcmWriteError,
    PcmWriteErrorKind, PCM_SOURCE_TYPE_ID,
};
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

#[cfg(test)]
mod tests {
    mod composition;
    mod registry;
    mod runtime;
}

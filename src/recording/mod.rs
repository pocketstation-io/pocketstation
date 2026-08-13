//! Multistem recording owned behind the generic Endpoint lifecycle.
//!
//! Session may offer convenience syntax, but recording policy, timeline,
//! persistence, and finalization remain in this module.

mod config;
mod endpoint;
mod error_code;
mod writer;

pub use config::{
    PermissionDecision, PermissionScope, RecorderLineageField, RecorderStemConfig, StemLabel,
};
pub use endpoint::{
    MultistemRecordingReceipt, SessionMultistemEndpointCoordinator,
    MULTISTEM_GROUP_CONFIGURATION_KEY,
};
pub use error_code::{recording_outcome_error_code, RecordingErrorCode};
#[cfg(any(test, feature = "internal-testing"))]
pub use writer::{DiscontinuityKind, DiscontinuityRecord};
pub use writer::{
    MultistemRecording, RecorderError, RecordingObservations, RecordingOutcome, RecordingState,
    RecordingStemOutcome,
};

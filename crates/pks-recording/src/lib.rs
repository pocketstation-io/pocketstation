//! Concrete multistem recording behind the generic endpoint lifecycle.

mod multistem_endpoint;
mod multistem_recorder;

pub use multistem_endpoint::{
    MultistemEndpointCoordinator, MultistemEndpointError, MultistemEndpointStem,
    MultistemRecordingReceipt, SessionMultistemEndpointCoordinator,
};
pub use multistem_recorder::{
    DiscontinuityKind, DiscontinuityRecord, MultistemRecording, PermissionDecision,
    PermissionScope, RecorderError, RecorderLineageField, RecorderStemConfig,
    RecordingObservations, RecordingOutcome, RecordingRollbackFailure,
    RecordingRollbackWorkerFailure, RecordingState, RecordingStemOutcome, StemLabel,
};

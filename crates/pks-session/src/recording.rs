use std::path::PathBuf;
use std::sync::Arc;

use pks_endpoint::{EndpointDriverFactory, EndpointGroupId};
use pks_graph::NodeTypeId;
use pks_recording::{
    MultistemRecordingReceipt, RecordingOutcome, SessionMultistemEndpointCoordinator,
};

use crate::{
    OperatorId, SessionEngineBuilder, SessionEngineRegistrationError,
    DEFAULT_MULTISTEM_RECORDING_GROUP_ID, RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
};

/// Safe receipt for one Session-owned multistem recording group.
#[derive(Clone)]
pub struct SessionRecordingReceipt {
    inner: MultistemRecordingReceipt,
}

impl SessionRecordingReceipt {
    /// Returns the terminal recording outcome after endpoint finalization.
    pub fn outcome(&self) -> Option<&RecordingOutcome> {
        self.inner.result()
    }
}

impl SessionEngineBuilder {
    /// Registers the canonical concrete multistem recorder for this engine.
    ///
    /// The Session declaration supplies exact endpoint/stem/route context.
    /// Capture-owned frame lineage supplies source, clock, generation, and
    /// permission truth. Callers supply only the artifact root.
    pub fn register_multistem_recording(
        &mut self,
        output_root: impl Into<PathBuf>,
    ) -> Result<SessionRecordingReceipt, SessionEngineRegistrationError> {
        let coordinator = Arc::new(SessionMultistemEndpointCoordinator::new(
            output_root,
            EndpointGroupId::new(DEFAULT_MULTISTEM_RECORDING_GROUP_ID),
        ));
        let receipt = SessionRecordingReceipt {
            inner: coordinator.receipt(),
        };
        let factory: Arc<dyn EndpointDriverFactory> = coordinator;
        self.register_endpoint_driver(
            OperatorId::new(RECORDER_OPERATOR_ID),
            NodeTypeId::from(RECORDER_NODE_TYPE_ID),
            factory,
        )?;
        Ok(receipt)
    }
}

pub use pks_recording::{
    recording_outcome_error_code as session_recording_outcome_error_code,
    RecordingErrorCode as SessionRecordingErrorCode,
    RecordingObservations as SessionRecordingObservations,
    RecordingOutcome as SessionRecordingOutcome, RecordingState as SessionRecordingState,
    RecordingStemOutcome as SessionRecordingStemOutcome,
};

//! Session composition for the multistem recording endpoint.
//!
//! Recording owns persistence, timeline, policy, and finalization. Session
//! owns only declaration and registration through the generic Endpoint path.

use std::path::PathBuf;
use std::sync::Arc;

use crate::endpoint::{EndpointDriverFactory, EndpointGroupId};
use crate::graph::NodeTypeId;
use crate::recording::{
    MultistemRecordingReceipt, RecordingOutcome, SessionMultistemEndpointCoordinator,
    MULTISTEM_GROUP_CONFIGURATION_KEY, MULTISTEM_NAME_CONFIGURATION_KEY,
};
use crate::session::{
    EndpointConfiguration, EndpointDescriptor, EndpointExtensionRegistrationError, EndpointHandle,
    OperatorId, Session, SessionEngineBuilder, SessionError, SourceOutputHandle, StemHandle,
};

pub const RECORDER_NODE_TYPE_ID: &str = "endpoint.recording.multistem";
pub const RECORDER_OPERATOR_ID: &str = "io.pocketstation.recording.wav-stems.v1";
#[cfg(any(test, feature = "internal-testing"))]
pub const RECORDING_GROUP_CONFIGURATION_KEY: &str = MULTISTEM_GROUP_CONFIGURATION_KEY;
pub const DEFAULT_MULTISTEM_RECORDING_GROUP_ID: &str = "session.multistem.default.v1";

#[derive(Clone)]
pub struct SessionRecordingReceipt {
    inner: MultistemRecordingReceipt,
}

impl SessionRecordingReceipt {
    pub fn outcome(&self) -> Option<&RecordingOutcome> {
        self.inner.result()
    }
}

impl SessionEngineBuilder {
    pub fn register_multistem_recording(
        &mut self,
        output_root: impl Into<PathBuf>,
    ) -> Result<SessionRecordingReceipt, EndpointExtensionRegistrationError> {
        let coordinator = Arc::new(SessionMultistemEndpointCoordinator::new(
            output_root,
            EndpointGroupId::new(DEFAULT_MULTISTEM_RECORDING_GROUP_ID),
        ));
        let receipt = SessionRecordingReceipt {
            inner: coordinator.receipt(),
        };
        let factory: Arc<dyn EndpointDriverFactory> = coordinator;
        self.register_audio_endpoint_driver(
            OperatorId::new(RECORDER_OPERATOR_ID),
            NodeTypeId::from(RECORDER_NODE_TYPE_ID),
            factory,
        )?;
        Ok(receipt)
    }
}

impl StemHandle {
    #[doc = "Attaches recording output to `StemHandle`."]
    pub fn record(&self, stem_name: impl Into<String>) -> Result<EndpointHandle, SessionError> {
        let descriptor = EndpointDescriptor::new(
            NodeTypeId::from(RECORDER_NODE_TYPE_ID),
            OperatorId::new(RECORDER_OPERATOR_ID),
        )
        .with_input_edge(multistem_recording_edge_contract())
        .with_configuration(
            EndpointConfiguration::new()
                .with(MULTISTEM_NAME_CONFIGURATION_KEY, stem_name)
                .with(
                    MULTISTEM_GROUP_CONFIGURATION_KEY,
                    DEFAULT_MULTISTEM_RECORDING_GROUP_ID,
                ),
        );
        self.declare_endpoint_and_send(descriptor)
    }
}

impl SourceOutputHandle {
    #[doc = "Attaches recording output to `SourceOutputHandle`."]
    pub fn record(&self, stem_name: impl Into<String>) -> Result<EndpointHandle, SessionError> {
        let descriptor = EndpointDescriptor::new(
            NodeTypeId::from(RECORDER_NODE_TYPE_ID),
            OperatorId::new(RECORDER_OPERATOR_ID),
        )
        .with_input_edge(multistem_recording_edge_contract())
        .with_configuration(
            EndpointConfiguration::new()
                .with(MULTISTEM_NAME_CONFIGURATION_KEY, stem_name)
                .with(
                    MULTISTEM_GROUP_CONFIGURATION_KEY,
                    DEFAULT_MULTISTEM_RECORDING_GROUP_ID,
                ),
        );
        self.declare_endpoint_and_send(descriptor)
    }
}

impl Session {
    pub fn declares_multistem_recording(&self) -> Result<bool, SessionError> {
        self.declares_endpoint_operator(&OperatorId::new(RECORDER_OPERATOR_ID))
    }
}

fn multistem_recording_edge_contract() -> crate::graph::EdgeContract {
    crate::graph::EdgeContract {
        jitter_budget_ms: Some(400),
        backpressure: crate::graph::BackpressurePolicy::DropNewest,
        loss: crate::graph::LossPolicy::DropAllowed,
        copy_policy: crate::graph::CopyPolicy::CopyToBranchPool,
        observability: crate::graph::EdgeObservabilityLevel::Full,
        ..crate::graph::EdgeContract::realtime_audio()
    }
}

pub use crate::recording::{
    recording_outcome_error_code as session_recording_outcome_error_code,
    RecordingErrorCode as SessionRecordingErrorCode,
    RecordingObservations as SessionRecordingObservations,
    RecordingOutcome as SessionRecordingOutcome, RecordingState as SessionRecordingState,
    RecordingStemOutcome as SessionRecordingStemOutcome,
};

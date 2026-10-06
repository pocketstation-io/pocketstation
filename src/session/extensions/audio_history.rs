use std::sync::Arc;

use crate::graph::{NodeTypeId, RouteSettings};
use crate::recording::{AudioHistory, AudioHistoryConfig, AudioHistoryError};
use crate::session::{
    EndpointDescriptor, EndpointHandle, OperatorId, SessionError, SourceOutputHandle, StemHandle,
};

const NODE_TYPE_ID: &str = "endpoint.recording.audio-history";
const OPERATOR_ID: &str = "io.pocketstation.recording.audio-history.v1";

impl crate::Session {
    /// Retain only audio explicitly routed with `retain_audio()`. Ordinary
    /// capture has no history allocation or worker until this is declared.
    pub fn audio_history(
        &self,
        config: AudioHistoryConfig,
    ) -> Result<AudioHistory, AudioHistoryDeclarationError> {
        let history = AudioHistory::new(config)?;
        let definition = super::builtins::audio_endpoint_boundary_definition_with_frame_samples(
            NodeTypeId::from(NODE_TYPE_ID),
            self.sample_spec,
            Some(
                self.audio_frame_duration
                    .samples_per_channel(self.sample_spec.sample_rate_hz),
            ),
        );
        self.register_endpoint(
            OperatorId::new(OPERATOR_ID),
            definition,
            Arc::new(crate::recording::HistoryFactory(history.clone())),
        )?;
        Ok(history)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AudioHistoryDeclarationError {
    #[error(transparent)]
    History(#[from] AudioHistoryError),
    #[error(transparent)]
    Session(#[from] crate::SessionEndpointError),
}

fn descriptor() -> EndpointDescriptor {
    EndpointDescriptor::new(NodeTypeId::from(NODE_TYPE_ID), OperatorId::new(OPERATOR_ID))
        .with_route_settings(RouteSettings {
            copy_policy: crate::graph::CopyPolicy::CopyToBranchPool,
            observability: crate::graph::RouteObservability::Full,
            ..RouteSettings::realtime_audio()
        })
}

impl SourceOutputHandle {
    pub fn retain_audio(&self) -> Result<EndpointHandle, SessionError> {
        self.declare_endpoint_and_send(descriptor())
    }
}

impl StemHandle {
    pub fn retain_audio(&self) -> Result<EndpointHandle, SessionError> {
        self.declare_endpoint_and_send(descriptor())
    }
}

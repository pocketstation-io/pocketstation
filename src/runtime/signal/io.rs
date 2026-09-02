//! Bounded Operator inputs and named outputs.

#[cfg(any(test, feature = "internal-testing"))]
use std::sync::atomic::Ordering;
#[cfg(any(test, feature = "internal-testing"))]
use std::sync::Arc;

#[cfg(any(test, feature = "internal-testing"))]
use crate::frame::AudioFrame;
#[cfg(any(test, feature = "internal-testing"))]
use crate::graph::SignalEnvelope;
use crate::graph::{EdgeContract, EdgeId, MediaCaps, SignalSpec};

#[cfg(any(test, feature = "internal-testing"))]
use super::edge::{SignalEdgeSendError, SignalEdgeSender};
use super::edge::{
    TypedEdgeBranchSpec, TypedEdgeObservationHandle, TypedEdgeObservations, TypedEdgeReceiver,
};
#[cfg(any(test, feature = "internal-testing"))]
use super::observations::AsyncOperatorObservationState;

#[cfg(any(test, feature = "internal-testing"))]
pub struct AsyncOperatorInput {
    pub(super) sender: SignalEdgeSender<SignalEnvelope>,
    pub(super) observations: Arc<AsyncOperatorObservationState>,
}

#[cfg(any(test, feature = "internal-testing"))]
#[derive(Debug, thiserror::Error)]
#[error("this async operator consumes a compiled plan edge and has no direct input sender")]
pub struct AsyncOperatorInputAccessError;

#[cfg(any(test, feature = "internal-testing"))]
impl AsyncOperatorInput {
    #[expect(
        clippy::result_large_err,
        reason = "full queues return the rejected envelope inline so saturation never allocates"
    )]
    pub fn send(
        &mut self,
        envelope: SignalEnvelope,
    ) -> Result<(), SignalEdgeSendError<SignalEnvelope>> {
        self.observations
            .input_attempted_total
            .fetch_add(1, Ordering::Relaxed);
        let result = self.sender.send(envelope);
        if result.is_err() {
            self.observations
                .input_dropped_total
                .fetch_add(1, Ordering::Relaxed);
        }
        result
    }

    pub fn send_audio(
        &mut self,
        frame: AudioFrame,
        sequence_number: u64,
        timestamp_ns: u64,
    ) -> Result<(), AudioFrame> {
        self.observations
            .input_attempted_total
            .fetch_add(1, Ordering::Relaxed);
        let result = self.sender.send_audio(frame, sequence_number, timestamp_ns);
        if result.is_err() {
            self.observations
                .input_dropped_total
                .fetch_add(1, Ordering::Relaxed);
        }
        result
    }
}

pub type AsyncOperatorOutput = TypedEdgeReceiver;
pub type AsyncOperatorOutputObservationHandle = TypedEdgeObservationHandle;
pub type AsyncOperatorOutputBranchSpec = TypedEdgeBranchSpec;
pub type AsyncOperatorOutputObservations = TypedEdgeObservations;

pub struct AsyncOperatorTypedInput {
    pub port_name: String,
    pub receiver: TypedEdgeReceiver,
    pub edge_id: Option<EdgeId>,
    pub signal_spec: SignalSpec,
    pub media: MediaCaps,
    pub edge_contract: EdgeContract,
    pub capacity_signals: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct AsyncOperatorNamedOutputBranchSpec<'a> {
    pub output_port: &'a str,
    pub branch: AsyncOperatorOutputBranchSpec,
}

pub struct AsyncOperatorNamedOutput {
    pub output_port: String,
    pub receiver: AsyncOperatorOutput,
}

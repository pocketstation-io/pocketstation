//! Bounded operator ingress and named output contracts.

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
#[doc = "Carries typed input for async operator."]
pub struct AsyncOperatorInput {
    pub(super) sender: SignalEdgeSender<SignalEnvelope>,
    pub(super) observations: Arc<AsyncOperatorObservationState>,
}

#[cfg(any(test, feature = "internal-testing"))]
#[derive(Debug, thiserror::Error)]
#[error("this async operator consumes a compiled plan edge and has no direct input sender")]
#[doc = "Classifies failures surfaced by async operator input access operations."]
pub struct AsyncOperatorInputAccessError;

#[cfg(any(test, feature = "internal-testing"))]
impl AsyncOperatorInput {
    #[expect(
        clippy::result_large_err,
        reason = "full queues return the rejected envelope inline so saturation never allocates"
    )]
    #[doc = "Sends a value through `AsyncOperatorInput`."]
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

    #[doc = "Sends one audio signal through the bounded input owned by `AsyncOperatorInput`."]
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

#[doc = "Exposes `TypedEdgeReceiver` as the public `AsyncOperatorOutput` alias at this API boundary."]
pub type AsyncOperatorOutput = TypedEdgeReceiver;
#[doc = "Exposes `TypedEdgeObservationHandle` as the public `AsyncOperatorOutputObservationHandle` alias at this API boundary."]
pub type AsyncOperatorOutputObservationHandle = TypedEdgeObservationHandle;
#[doc = "Exposes `TypedEdgeBranchSpec` as the public `AsyncOperatorOutputBranchSpec` alias at this API boundary."]
pub type AsyncOperatorOutputBranchSpec = TypedEdgeBranchSpec;
#[doc = "Exposes `TypedEdgeObservations` as the public `AsyncOperatorOutputObservations` alias at this API boundary."]
pub type AsyncOperatorOutputObservations = TypedEdgeObservations;

#[doc = "Carries typed input for async operator typed."]
pub struct AsyncOperatorTypedInput {
    #[doc = "Stores the human-readable port used to identify `AsyncOperatorTypedInput`."]
    pub port_name: String,
    #[doc = "Owns the receiver endpoint through which `AsyncOperatorTypedInput` exchanges values."]
    pub receiver: TypedEdgeReceiver,
    #[doc = "Identifies the edge identifier recorded by `AsyncOperatorTypedInput`."]
    pub edge_id: Option<EdgeId>,
    #[doc = "Declares the signal class and format accepted by `AsyncOperatorTypedInput`."]
    pub signal_spec: SignalSpec,
    #[doc = "Records the media selected for `AsyncOperatorTypedInput`."]
    pub media: MediaCaps,
    #[doc = "References the edge contract participating in `AsyncOperatorTypedInput`."]
    pub edge_contract: EdgeContract,
    #[doc = "Sets the capacity signals available to `AsyncOperatorTypedInput`."]
    pub capacity_signals: usize,
}

#[derive(Debug, Clone, Copy)]
#[doc = "Configures async operator named output branch behavior at its owning API boundary."]
pub struct AsyncOperatorNamedOutputBranchSpec<'a> {
    #[doc = "References the output port participating in `AsyncOperatorNamedOutputBranchSpec`."]
    pub output_port: &'a str,
    #[doc = "References the branch participating in `AsyncOperatorNamedOutputBranchSpec`."]
    pub branch: AsyncOperatorOutputBranchSpec,
}

#[doc = "Carries typed output from async operator named."]
pub struct AsyncOperatorNamedOutput {
    #[doc = "References the output port participating in `AsyncOperatorNamedOutput`."]
    pub output_port: String,
    #[doc = "Owns the receiver endpoint through which `AsyncOperatorNamedOutput` exchanges values."]
    pub receiver: AsyncOperatorOutput,
}

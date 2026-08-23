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
#[doc = "Reports a async operator input access error."]
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

    #[doc = "Sends audio for `AsyncOperatorInput`."]
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

#[doc = "Names the async operator output type used by the public API."]
pub type AsyncOperatorOutput = TypedEdgeReceiver;
#[doc = "Names the async operator output observation handle type used by the public API."]
pub type AsyncOperatorOutputObservationHandle = TypedEdgeObservationHandle;
#[doc = "Names the async operator output branch spec type used by the public API."]
pub type AsyncOperatorOutputBranchSpec = TypedEdgeBranchSpec;
#[doc = "Names the async operator output observations type used by the public API."]
pub type AsyncOperatorOutputObservations = TypedEdgeObservations;

#[doc = "Carries typed input for async operator typed."]
pub struct AsyncOperatorTypedInput {
    #[doc = "Stores the port name used by `AsyncOperatorTypedInput`."]
    pub port_name: String,
    #[doc = "Stores the receiver used by `AsyncOperatorTypedInput`."]
    pub receiver: TypedEdgeReceiver,
    #[doc = "Identifies the edge identifier recorded by `AsyncOperatorTypedInput`."]
    pub edge_id: Option<EdgeId>,
    #[doc = "Stores the signal spec used by `AsyncOperatorTypedInput`."]
    pub signal_spec: SignalSpec,
    #[doc = "Stores the media used by `AsyncOperatorTypedInput`."]
    pub media: MediaCaps,
    #[doc = "Stores the edge contract used by `AsyncOperatorTypedInput`."]
    pub edge_contract: EdgeContract,
    #[doc = "Sets the capacity signals available to `AsyncOperatorTypedInput`."]
    pub capacity_signals: usize,
}

#[derive(Debug, Clone, Copy)]
#[doc = "Configures async operator named output branch behavior at its owning API boundary."]
pub struct AsyncOperatorNamedOutputBranchSpec<'a> {
    #[doc = "Stores the output port used by `AsyncOperatorNamedOutputBranchSpec`."]
    pub output_port: &'a str,
    #[doc = "Stores the branch used by `AsyncOperatorNamedOutputBranchSpec`."]
    pub branch: AsyncOperatorOutputBranchSpec,
}

#[doc = "Carries typed output from async operator named."]
pub struct AsyncOperatorNamedOutput {
    #[doc = "Stores the output port used by `AsyncOperatorNamedOutput`."]
    pub output_port: String,
    #[doc = "Stores the receiver used by `AsyncOperatorNamedOutput`."]
    pub receiver: AsyncOperatorOutput,
}

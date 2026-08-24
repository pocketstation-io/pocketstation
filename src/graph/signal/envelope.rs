use crate::frame::{AudioFrame, FrameLineage, SourceId};
use crate::graph::signal::payload::SignalPayload;
use crate::graph::signal::{SignalDerivation, SignalLineage, SignalSpec, SignalTiming};

#[derive(Debug)]
#[doc = "Carries a typed signal payload together with timing, lineage, continuity, and terminal metadata."]
pub struct SignalEnvelope {
    pub(crate) payload: SignalPayload,
    pub(crate) spec: SignalSpec,
    pub(crate) timing: SignalTiming,
    pub(crate) lineage: Option<SignalLineage>,
    pub(crate) derivation: Option<SignalDerivation>,
}

impl SignalEnvelope {
    /// Creates an envelope for data that has not yet entered a source-aware
    /// Session. Session sources must attach lineage before routing it.
    pub fn untracked(payload: SignalPayload, spec: SignalSpec, observed_timestamp_ns: u64) -> Self {
        Self {
            payload,
            spec,
            timing: SignalTiming::observed(observed_timestamp_ns),
            lineage: None,
            derivation: None,
        }
    }

    #[doc = "Creates `SignalEnvelope` from audio."]
    pub fn from_audio(frame: AudioFrame, lineage: Option<FrameLineage>) -> Self {
        let timestamp_ns = frame.timestamp_ns;
        let frame_stream_id = frame.stream_id;
        let signal_lineage =
            lineage.map(|lineage| SignalLineage::from_frame(frame_stream_id, lineage));
        let timing = lineage.map_or_else(
            || SignalTiming::observed(timestamp_ns),
            |lineage| SignalTiming::from_frame(lineage, timestamp_ns),
        );
        Self {
            payload: SignalPayload::Audio(frame),
            spec: SignalSpec::audio(),
            timing,
            lineage: signal_lineage,
            derivation: None,
        }
    }

    #[doc = "Transforms the payload held by `SignalEnvelope` while preserving envelope metadata."]
    pub fn map_payload(mut self, payload: SignalPayload, spec: SignalSpec) -> Self {
        self.payload = payload;
        self.spec = spec;
        self
    }

    #[doc = "Sets the lineage on `SignalEnvelope` and returns the updated value."]
    pub fn with_lineage(mut self, lineage: SignalLineage, timing: SignalTiming) -> Self {
        self.lineage = Some(lineage);
        self.timing = timing;
        self
    }

    #[doc = "Sets the derivation on `SignalEnvelope` and returns the updated value."]
    pub fn with_derivation(mut self, derivation: SignalDerivation) -> Self {
        self.derivation = Some(derivation);
        self
    }

    #[doc = "Returns the payload held by `SignalEnvelope`."]
    pub const fn payload(&self) -> &SignalPayload {
        &self.payload
    }

    #[doc = "Returns the payload size bytes held by `SignalEnvelope`."]
    pub fn payload_size_bytes(&self) -> usize {
        self.payload.size_bytes()
    }

    #[doc = "Returns the signal spec held by `SignalEnvelope`."]
    pub const fn signal_spec(&self) -> &SignalSpec {
        &self.spec
    }

    #[doc = "Returns the timing held by `SignalEnvelope`."]
    pub const fn timing(&self) -> SignalTiming {
        self.timing
    }

    #[doc = "Returns the frame lineage carried by `SignalEnvelope`."]
    pub const fn lineage(&self) -> Option<SignalLineage> {
        self.lineage
    }

    #[doc = "Returns the derivation held by `SignalEnvelope`."]
    pub const fn derivation(&self) -> Option<&SignalDerivation> {
        self.derivation.as_ref()
    }

    #[doc = "Converts `SignalEnvelope` into payload."]
    pub fn into_payload(self) -> SignalPayload {
        self.payload
    }

    #[doc = "Returns the sequence number held by `SignalEnvelope`."]
    pub fn sequence_number(&self) -> Option<u64> {
        self.lineage.map(|lineage| lineage.sequence_number).or(
            if let SignalPayload::Audio(frame) = &self.payload {
                Some(frame.sequence_number)
            } else {
                None
            },
        )
    }

    #[doc = "Returns the source identifier held by `SignalEnvelope`."]
    pub fn source_id(&self) -> Option<SourceId> {
        self.lineage.map(|lineage| lineage.source_id).or({
            if let SignalPayload::Audio(frame) = &self.payload {
                Some(frame.source_id)
            } else {
                None
            }
        })
    }

    #[doc = "Returns the timestamp nanoseconds held by `SignalEnvelope`."]
    pub fn timestamp_ns(&self) -> u64 {
        self.timing
            .session_timestamp_ns
            .or(self.timing.source_timestamp_ns)
            .unwrap_or(self.timing.observed_timestamp_ns)
    }

    #[doc = "Validates `SignalEnvelope` against its declared contract."]
    pub fn validate(&self) -> Result<(), SignalEnvelopeError> {
        self.spec
            .validate()
            .map_err(|_| SignalEnvelopeError::InvalidSignalSpec)?;
        if !self.payload.supports(&self.spec) {
            return Err(SignalEnvelopeError::PayloadSpecMismatch);
        }
        if let (SignalPayload::Audio(frame), Some(lineage)) = (&self.payload, self.lineage) {
            if frame.sequence_number != lineage.sequence_number {
                return Err(SignalEnvelopeError::SequenceMismatch);
            }
            if frame.source_id != lineage.source_id || frame.stream_id != lineage.stream_id {
                return Err(SignalEnvelopeError::SourceMismatch);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures surfaced by signal envelope operations."]
pub enum SignalEnvelopeError {
    #[error("signal envelope has an invalid SignalSpec")]
    #[doc = "Reports that the supplied signal spec is invalid."]
    InvalidSignalSpec,
    #[error("signal payload does not match its declared SignalSpec")]
    #[doc = "Reports that payload spec does not match the expected contract."]
    PayloadSpecMismatch,
    #[error("signal envelope sequence does not match its lineage")]
    #[doc = "Reports that sequence does not match the expected contract."]
    SequenceMismatch,
    #[error("signal envelope source does not match its lineage")]
    #[doc = "Reports that source does not match the expected contract."]
    SourceMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{AudioBufferPool, ClockDomainId, SessionId, StemId, StreamId};
    use crate::graph::{
        AsyncNode, AsyncNodeFuture, AsyncOperatorEdgePrepareContext, AsyncOperatorPrepareContext,
        BinaryFormat, Codec, EdgeContract, EventFormat, ExecutionPartition, MediaCaps, NodeError,
        PortDirection, SignalContinuityError, SignalContinuityObservation, SignalContinuityTracker,
        SignalId, TextFormat,
    };

    fn prepare_cx() -> AsyncOperatorPrepareContext {
        let mut contract = EdgeContract::bounded_async();
        contract.media = MediaCaps::Control;
        AsyncOperatorPrepareContext::new(
            ExecutionPartition::AsyncWorker,
            vec![
                AsyncOperatorEdgePrepareContext::new(
                    None,
                    "input",
                    PortDirection::Input,
                    SignalSpec::control(),
                    MediaCaps::Control,
                    contract,
                    8,
                )
                .unwrap(),
                AsyncOperatorEdgePrepareContext::new(
                    None,
                    "output",
                    PortDirection::Output,
                    SignalSpec::control(),
                    MediaCaps::Control,
                    contract,
                    8,
                )
                .unwrap(),
            ],
        )
        .unwrap()
    }

    fn block_on_ready<T>(future: AsyncNodeFuture<'_, T>) -> T {
        use std::task::{Context, Poll, Waker};

        let mut cx = Context::from_waker(Waker::noop());
        let mut future = future;
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("test future unexpectedly pending"),
        }
    }

    struct EchoAsyncNode {
        prepared: bool,
    }

    impl AsyncNode for EchoAsyncNode {
        fn prepare<'a>(
            &'a mut self,
            _cx: &'a AsyncOperatorPrepareContext,
        ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
            Box::pin(async move {
                self.prepared = true;
                Ok(())
            })
        }

        fn process<'a>(
            &'a mut self,
            input: SignalEnvelope,
        ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
            Box::pin(async move {
                if !self.prepared {
                    return Err(NodeError::Process("async node not prepared".to_owned()));
                }
                Ok(vec![input])
            })
        }
    }

    #[test]
    fn given_text_storage_when_checked_against_text_spec_then_representation_is_supported() {
        let signal = SignalPayload::Text("text".to_owned());
        assert!(signal.supports(&SignalSpec::text(crate::graph::signal::TextFormat::Utf8)));
    }

    #[test]
    fn given_echo_async_node_when_process_after_prepare_then_envelope_is_returned() {
        let mut node = EchoAsyncNode { prepared: false };
        block_on_ready(node.prepare(&prepare_cx())).unwrap();

        let envelope =
            SignalEnvelope::untracked(SignalPayload::Bytes(Vec::new()), SignalSpec::control(), 9);
        let output = block_on_ready(node.process(envelope))
            .unwrap()
            .pop()
            .unwrap();

        assert_eq!(output.sequence_number(), None);
        assert_eq!(output.timestamp_ns(), 9);
        assert_eq!(output.spec, SignalSpec::control());
        assert!(matches!(output.payload, SignalPayload::Bytes(_)));
    }

    #[test]
    fn given_echo_async_node_when_process_before_prepare_then_error_is_returned() {
        let mut node = EchoAsyncNode { prepared: false };
        let envelope =
            SignalEnvelope::untracked(SignalPayload::Bytes(Vec::new()), SignalSpec::control(), 0);
        let error = block_on_ready(node.process(envelope)).unwrap_err();

        assert!(matches!(error, NodeError::Process(_)));
    }

    #[test]
    fn given_fundamental_payloads_when_enveloped_then_specs_are_symmetric() {
        let payloads = [
            (
                SignalPayload::Bytes(vec![1]),
                SignalSpec::encoded_audio(Codec::Opus),
            ),
            (
                SignalPayload::Text("hello".to_owned()),
                SignalSpec::text(TextFormat::Markdown),
            ),
            (
                SignalPayload::Bytes(vec![2]),
                SignalSpec::event(EventFormat::Cbor),
            ),
            (SignalPayload::Bytes(vec![3]), SignalSpec::metrics()),
            (
                SignalPayload::Bytes(vec![4]),
                SignalSpec::binary(BinaryFormat::Protobuf),
            ),
            (
                SignalPayload::Bytes(vec![5]),
                SignalSpec::custom(SignalId::new("com.acme.signal.v1")),
            ),
        ];

        for (payload, spec) in payloads {
            assert!(SignalEnvelope::untracked(payload, spec, 2)
                .validate()
                .is_ok());
        }
    }

    #[test]
    fn given_payload_and_incompatible_spec_when_validated_then_rejected() {
        let envelope =
            SignalEnvelope::untracked(SignalPayload::Text(String::new()), SignalSpec::control(), 2);

        assert_eq!(
            envelope.validate(),
            Err(SignalEnvelopeError::PayloadSpecMismatch)
        );
    }

    #[test]
    fn given_generic_lineage_when_enveloped_then_no_frame_lineage_is_required() {
        let lineage = SignalLineage {
            session_id: SessionId(1),
            stream_id: StreamId(2),
            source_id: SourceId(3),
            clock_id: ClockDomainId(4),
            sequence_number: 5,
            source_generation: 6,
            discontinuity_epoch: 7,
            policy_epoch: 8,
        };
        let timing = SignalTiming {
            source_timestamp_ns: Some(10),
            observed_timestamp_ns: 12,
            session_timestamp_ns: Some(11),
            duration_ns: Some(20),
        };
        let envelope =
            SignalEnvelope::untracked(SignalPayload::Bytes(vec![]), SignalSpec::metrics(), 0)
                .with_lineage(lineage, timing);

        assert!(envelope.validate().is_ok());
        assert_eq!(envelope.lineage, Some(lineage));
        assert_eq!(envelope.timing.timestamp_end_ns(), Some(30));
    }

    #[test]
    fn given_audio_frame_lineage_when_enveloped_then_generic_lineage_is_projected() {
        let pool = AudioBufferPool::new(1, 4);
        let frame = AudioFrame::new(StreamId(2), SourceId(3), 5, 10, 1, pool.acquire().unwrap());
        let frame_lineage = FrameLineage {
            session_id: SessionId(1),
            source_id: SourceId(3),
            stem_id: StemId(4),
            clock_id: ClockDomainId(5),
            sequence_num: 5,
            timestamp_start_ns: 10,
            duration_ns: 20,
            source_generation: 6,
            discontinuity_epoch: 7,
            permission_epoch: 8,
        };
        let envelope = SignalEnvelope::from_audio(frame, Some(frame_lineage));

        assert!(envelope.validate().is_ok());
        assert_eq!(
            envelope.lineage,
            Some(SignalLineage::from_frame(StreamId(2), frame_lineage))
        );
        assert_eq!(envelope.timing.timestamp_end_ns(), Some(30));
    }

    fn generic_envelope(
        sequence_number: u64,
        timestamp_ns: u64,
        generation: u32,
        discontinuity_epoch: u64,
        policy_epoch: u64,
    ) -> SignalEnvelope {
        SignalEnvelope::untracked(
            SignalPayload::Bytes(vec![]),
            SignalSpec::metrics(),
            timestamp_ns,
        )
        .with_lineage(
            SignalLineage {
                session_id: SessionId(1),
                stream_id: StreamId(2),
                source_id: SourceId(3),
                clock_id: ClockDomainId(4),
                sequence_number,
                source_generation: generation,
                discontinuity_epoch,
                policy_epoch,
            },
            SignalTiming {
                source_timestamp_ns: Some(timestamp_ns),
                observed_timestamp_ns: timestamp_ns,
                session_timestamp_ns: Some(timestamp_ns),
                duration_ns: Some(10),
            },
        )
    }

    #[test]
    fn given_contiguous_signals_when_replayed_then_continuity_is_deterministic() {
        let mut first = SignalContinuityTracker::default();
        let mut second = SignalContinuityTracker::default();
        let envelopes = [
            generic_envelope(0, 10, 1, 0, 1),
            generic_envelope(1, 20, 1, 0, 1),
            generic_envelope(0, 30, 2, 1, 2),
        ];

        let first_result = envelopes
            .iter()
            .map(|envelope| first.observe(envelope))
            .collect::<Vec<_>>();
        let second_result = envelopes
            .iter()
            .map(|envelope| second.observe(envelope))
            .collect::<Vec<_>>();

        assert_eq!(first_result, second_result);
        assert_eq!(
            first_result[2],
            Ok(SignalContinuityObservation {
                discontinuity_observed: true,
                source_recovered: true,
                policy_changed: true,
            })
        );
    }

    #[test]
    fn given_gap_without_discontinuity_when_replayed_then_rejected() {
        let mut tracker = SignalContinuityTracker::default();
        tracker.observe(&generic_envelope(0, 10, 1, 0, 1)).unwrap();

        assert_eq!(
            tracker.observe(&generic_envelope(2, 20, 1, 0, 1)),
            Err(SignalContinuityError::SequenceGapWithoutDiscontinuity)
        );
    }

    #[test]
    fn given_recovery_without_discontinuity_when_replayed_then_rejected() {
        let mut tracker = SignalContinuityTracker::default();
        tracker.observe(&generic_envelope(0, 10, 1, 0, 1)).unwrap();

        assert_eq!(
            tracker.observe(&generic_envelope(1, 20, 2, 0, 1)),
            Err(SignalContinuityError::RecoveryWithoutDiscontinuity)
        );
    }
}

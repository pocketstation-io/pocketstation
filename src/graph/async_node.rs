use std::future::Future;
use std::pin::Pin;

use crate::frame::{AudioFrame, ClockDomainId, FrameLineage, SessionId, SourceId, StreamId};
use crate::graph::node::{NodeError, PrepareContext};
use crate::graph::signal::{
    BinaryFormat, Codec, EventFormat, SignalClass, SignalId, SignalSpec, TextFormat,
};
use crate::graph::DerivedSignalLineage;

pub type AsyncNodeFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Debug)]
pub enum SignalPayload {
    Audio(AudioFrame),
    EncodedAudio {
        codec: Codec,
        bytes: Vec<u8>,
    },
    Text(String),
    FormattedText {
        format: TextFormat,
        text: String,
    },
    Event(Vec<u8>),
    StructuredEvent {
        format: EventFormat,
        bytes: Vec<u8>,
    },
    Metrics(Vec<u8>),
    Binary(Vec<u8>),
    StructuredBinary {
        format: BinaryFormat,
        bytes: Vec<u8>,
    },
    Control(Vec<u8>),
    Custom {
        signal_id: SignalId,
        bytes: Vec<u8>,
    },
}

impl SignalPayload {
    pub fn signal_spec(&self) -> SignalSpec {
        match self {
            Self::Audio(_) => SignalSpec::audio(),
            Self::EncodedAudio { codec, .. } => SignalSpec::encoded_audio(*codec),
            Self::Text(_) => SignalSpec::text(crate::graph::signal::TextFormat::Utf8),
            Self::FormattedText { format, .. } => SignalSpec::text(*format),
            Self::Event(_) => SignalSpec::event(crate::graph::signal::EventFormat::Json),
            Self::StructuredEvent { format, .. } => SignalSpec::event(*format),
            Self::Metrics(_) => SignalSpec::metrics(),
            Self::Binary(_) => {
                SignalSpec::new(SignalClass::Binary(crate::graph::signal::BinaryFormat::Raw))
            }
            Self::StructuredBinary { format, .. } => SignalSpec::binary(*format),
            Self::Control(_) => SignalSpec::control(),
            Self::Custom { signal_id, .. } => SignalSpec::custom(signal_id.clone()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalTiming {
    pub source_timestamp_ns: Option<u64>,
    pub observed_timestamp_ns: u64,
    pub session_timestamp_ns: Option<u64>,
    pub duration_ns: Option<u64>,
}

impl SignalTiming {
    pub const fn observed(observed_timestamp_ns: u64) -> Self {
        Self {
            source_timestamp_ns: None,
            observed_timestamp_ns,
            session_timestamp_ns: None,
            duration_ns: None,
        }
    }

    pub const fn from_frame(lineage: FrameLineage, observed_timestamp_ns: u64) -> Self {
        Self {
            source_timestamp_ns: Some(lineage.timestamp_start_ns),
            observed_timestamp_ns,
            session_timestamp_ns: Some(lineage.timestamp_start_ns),
            duration_ns: Some(lineage.duration_ns),
        }
    }

    pub fn timestamp_end_ns(self) -> Option<u64> {
        self.source_timestamp_ns
            .zip(self.duration_ns)
            .map(|(start, duration)| start.saturating_add(duration))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalLineage {
    pub session_id: SessionId,
    pub stream_id: StreamId,
    pub source_id: SourceId,
    pub clock_id: ClockDomainId,
    pub sequence_number: u64,
    pub source_generation: u32,
    pub discontinuity_epoch: u64,
    pub policy_epoch: u64,
}

impl SignalLineage {
    pub const fn from_frame(stream_id: StreamId, lineage: FrameLineage) -> Self {
        Self {
            session_id: lineage.session_id,
            stream_id,
            source_id: lineage.source_id,
            clock_id: lineage.clock_id,
            sequence_number: lineage.sequence_num,
            source_generation: lineage.source_generation,
            discontinuity_epoch: lineage.discontinuity_epoch,
            policy_epoch: lineage.permission_epoch,
        }
    }
}

#[derive(Debug)]
pub struct SignalEnvelope {
    pub signal: SignalPayload,
    pub signal_spec: SignalSpec,
    pub sequence_number: u64,
    pub timestamp_ns: u64,
    pub source_id: Option<SourceId>,
    pub lineage: Option<FrameLineage>,
    pub derived_lineage: Option<DerivedSignalLineage>,
    pub timing: SignalTiming,
    pub signal_lineage: Option<SignalLineage>,
}

impl SignalEnvelope {
    pub fn new(signal: SignalPayload, sequence_number: u64, timestamp_ns: u64) -> Self {
        let signal_spec = signal.signal_spec();
        Self {
            signal,
            signal_spec,
            sequence_number,
            timestamp_ns,
            source_id: None,
            lineage: None,
            derived_lineage: None,
            timing: SignalTiming::observed(timestamp_ns),
            signal_lineage: None,
        }
    }

    pub fn from_audio(frame: AudioFrame, lineage: Option<FrameLineage>) -> Self {
        let sequence_number = frame.sequence_number;
        let timestamp_ns = frame.timestamp_ns;
        let frame_stream_id = frame.stream_id;
        let source_id = Some(frame.source_id);
        let signal_lineage =
            lineage.map(|lineage| SignalLineage::from_frame(frame_stream_id, lineage));
        let timing = lineage.map_or_else(
            || SignalTiming::observed(timestamp_ns),
            |lineage| SignalTiming::from_frame(lineage, timestamp_ns),
        );
        Self {
            signal: SignalPayload::Audio(frame),
            signal_spec: SignalSpec::audio(),
            sequence_number,
            timestamp_ns,
            source_id,
            lineage,
            derived_lineage: None,
            timing,
            signal_lineage,
        }
    }

    pub fn map_signal(mut self, signal: SignalPayload, signal_spec: SignalSpec) -> Self {
        self.signal = signal;
        self.signal_spec = signal_spec;
        self
    }

    pub fn with_signal_lineage(mut self, lineage: SignalLineage, timing: SignalTiming) -> Self {
        self.sequence_number = lineage.sequence_number;
        self.timestamp_ns = timing
            .session_timestamp_ns
            .or(timing.source_timestamp_ns)
            .unwrap_or(timing.observed_timestamp_ns);
        self.source_id = Some(lineage.source_id);
        self.signal_lineage = Some(lineage);
        self.timing = timing;
        self
    }

    pub fn validate(&self) -> Result<(), SignalEnvelopeError> {
        let payload_spec = self.signal.signal_spec();
        if payload_spec.class != self.signal_spec.class {
            return Err(SignalEnvelopeError::PayloadSpecMismatch);
        }
        if let Some(lineage) = self.signal_lineage {
            if lineage.sequence_number != self.sequence_number {
                return Err(SignalEnvelopeError::SequenceMismatch);
            }
            if self
                .source_id
                .is_some_and(|source_id| source_id != lineage.source_id)
            {
                return Err(SignalEnvelopeError::SourceMismatch);
            }
        }
        if let Some(frame_lineage) = self.lineage {
            if frame_lineage.sequence_num != self.sequence_number {
                return Err(SignalEnvelopeError::SequenceMismatch);
            }
            if self
                .source_id
                .is_some_and(|source_id| source_id != frame_lineage.source_id)
            {
                return Err(SignalEnvelopeError::SourceMismatch);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SignalEnvelopeError {
    #[error("signal payload does not match its declared SignalSpec")]
    PayloadSpecMismatch,
    #[error("signal envelope sequence does not match its lineage")]
    SequenceMismatch,
    #[error("signal envelope source does not match its lineage")]
    SourceMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalContinuityObservation {
    pub discontinuity_observed: bool,
    pub source_recovered: bool,
    pub policy_changed: bool,
}

#[derive(Debug, Default)]
pub struct SignalContinuityTracker {
    previous: Option<(SignalLineage, SignalTiming)>,
}

impl SignalContinuityTracker {
    pub fn observe(
        &mut self,
        envelope: &SignalEnvelope,
    ) -> Result<SignalContinuityObservation, SignalContinuityError> {
        envelope
            .validate()
            .map_err(SignalContinuityError::InvalidEnvelope)?;
        let current = envelope
            .signal_lineage
            .ok_or(SignalContinuityError::MissingLineage)?;
        if let Some((previous, previous_timing)) = self.previous {
            if current.session_id != previous.session_id
                || current.stream_id != previous.stream_id
                || current.source_id != previous.source_id
                || current.clock_id != previous.clock_id
            {
                return Err(SignalContinuityError::IdentityChanged);
            }
            if current.discontinuity_epoch < previous.discontinuity_epoch {
                return Err(SignalContinuityError::DiscontinuityRegressed);
            }
            if current.source_generation < previous.source_generation {
                return Err(SignalContinuityError::GenerationRegressed);
            }
            if current.policy_epoch < previous.policy_epoch {
                return Err(SignalContinuityError::PolicyRegressed);
            }
            let discontinuity_observed = current.discontinuity_epoch > previous.discontinuity_epoch;
            if !discontinuity_observed
                && current.sequence_number != previous.sequence_number.saturating_add(1)
            {
                return Err(SignalContinuityError::SequenceGapWithoutDiscontinuity);
            }
            let source_recovered = current.source_generation > previous.source_generation;
            if source_recovered && !discontinuity_observed {
                return Err(SignalContinuityError::RecoveryWithoutDiscontinuity);
            }
            if timestamp_regressed(previous_timing, envelope.timing) {
                return Err(SignalContinuityError::TimestampRegression);
            }
            let observation = SignalContinuityObservation {
                discontinuity_observed,
                source_recovered,
                policy_changed: current.policy_epoch > previous.policy_epoch,
            };
            self.previous = Some((current, envelope.timing));
            return Ok(observation);
        }
        self.previous = Some((current, envelope.timing));
        Ok(SignalContinuityObservation {
            discontinuity_observed: false,
            source_recovered: false,
            policy_changed: false,
        })
    }
}

fn timestamp_regressed(previous: SignalTiming, current: SignalTiming) -> bool {
    current.observed_timestamp_ns < previous.observed_timestamp_ns
        || previous
            .source_timestamp_ns
            .zip(current.source_timestamp_ns)
            .is_some_and(|(previous, current)| current < previous)
        || previous
            .session_timestamp_ns
            .zip(current.session_timestamp_ns)
            .is_some_and(|(previous, current)| current < previous)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SignalContinuityError {
    #[error("signal envelope is invalid: {0}")]
    InvalidEnvelope(SignalEnvelopeError),
    #[error("signal continuity requires source-independent lineage")]
    MissingLineage,
    #[error("signal identity changed within one continuity tracker")]
    IdentityChanged,
    #[error("signal sequence gap occurred without a discontinuity epoch change")]
    SequenceGapWithoutDiscontinuity,
    #[error("signal timestamp regressed")]
    TimestampRegression,
    #[error("signal discontinuity epoch regressed")]
    DiscontinuityRegressed,
    #[error("signal source generation regressed")]
    GenerationRegressed,
    #[error("signal source recovery occurred without a discontinuity")]
    RecoveryWithoutDiscontinuity,
    #[error("signal policy epoch regressed")]
    PolicyRegressed,
}

/// Async operator contract for model, connector, transport, and control-plane work.
///
/// `AsyncNode` is intentionally separate from `RuntimeNode`: realtime nodes process
/// `AudioFrame` synchronously on alloc-free executors, while async nodes may await,
/// allocate, and perform I/O only after a Bridge has moved data off the hot path.
pub trait AsyncNode: Send {
    fn prepare<'a>(
        &'a mut self,
        cx: &'a PrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>>;

    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>>;

    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async { Ok(Vec::new()) })
    }

    fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }

    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{AudioBufferPool, SampleFormat, SampleSpec, StemId};

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
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
            _cx: &'a PrepareContext,
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
    fn given_text_signal_when_signal_spec_then_text_class_is_returned() {
        let signal = SignalPayload::Text("partial transcript".to_owned());
        assert!(signal
            .signal_spec()
            .class
            .is_compatible_with(&SignalSpec::text(crate::graph::signal::TextFormat::Utf8).class));
    }

    #[test]
    fn given_echo_async_node_when_process_after_prepare_then_envelope_is_returned() {
        let mut node = EchoAsyncNode { prepared: false };
        block_on_ready(node.prepare(&prepare_cx())).unwrap();

        let envelope = SignalEnvelope::new(SignalPayload::Control(Vec::new()), 7, 9);
        let output = block_on_ready(node.process(envelope))
            .unwrap()
            .pop()
            .unwrap();

        assert_eq!(output.sequence_number, 7);
        assert_eq!(output.timestamp_ns, 9);
        assert_eq!(output.signal_spec, SignalSpec::control());
        assert!(matches!(output.signal, SignalPayload::Control(_)));
    }

    #[test]
    fn given_echo_async_node_when_process_before_prepare_then_error_is_returned() {
        let mut node = EchoAsyncNode { prepared: false };
        let envelope = SignalEnvelope::new(SignalPayload::Control(Vec::new()), 0, 0);
        let error = block_on_ready(node.process(envelope)).unwrap_err();

        assert!(matches!(error, NodeError::Process(_)));
    }

    #[test]
    fn given_fundamental_payloads_when_enveloped_then_specs_are_symmetric() {
        let payloads = [
            SignalPayload::EncodedAudio {
                codec: Codec::Opus,
                bytes: vec![1],
            },
            SignalPayload::FormattedText {
                format: TextFormat::Markdown,
                text: "hello".to_owned(),
            },
            SignalPayload::StructuredEvent {
                format: EventFormat::Cbor,
                bytes: vec![2],
            },
            SignalPayload::Metrics(vec![3]),
            SignalPayload::StructuredBinary {
                format: BinaryFormat::Protobuf,
                bytes: vec![4],
            },
            SignalPayload::Custom {
                signal_id: SignalId::new("com.acme.signal.v1"),
                bytes: vec![5],
            },
        ];

        for payload in payloads {
            assert!(SignalEnvelope::new(payload, 1, 2).validate().is_ok());
        }
    }

    #[test]
    fn given_payload_and_incompatible_spec_when_validated_then_rejected() {
        let mut envelope = SignalEnvelope::new(SignalPayload::Metrics(vec![]), 1, 2);
        envelope.signal_spec = SignalSpec::control();

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
        let envelope = SignalEnvelope::new(SignalPayload::Metrics(vec![]), 0, 0)
            .with_signal_lineage(lineage, timing);

        assert!(envelope.validate().is_ok());
        assert_eq!(envelope.signal_lineage, Some(lineage));
        assert!(envelope.lineage.is_none());
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
            envelope.signal_lineage,
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
        SignalEnvelope::new(
            SignalPayload::Metrics(vec![]),
            sequence_number,
            timestamp_ns,
        )
        .with_signal_lineage(
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

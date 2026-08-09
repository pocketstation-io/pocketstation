use std::future::Future;
use std::pin::Pin;

use crate::frame::{
    AudioFrame, ClockDomainId, ConnectorId, FrameLineage, SessionId, SourceId, StreamId,
};
use crate::graph::node::NodeError;
use crate::graph::operator::OperatorId;
use crate::graph::signal::{
    BinaryFormat, Codec, EventFormat, SignalClass, SignalId, SignalSpec, TextFormat,
};
use crate::graph::{EdgeContract, EdgeId, ExecutionPartition, MediaCaps, PortDirection};

pub type AsyncNodeFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Exact bounded graph edge supplied to an asynchronous Operator at prepare time.
///
/// This contract is signal-shaped, not audio-shaped. An audio edge carries
/// audio `MediaCaps`; text, event, metrics, control, binary, and custom edges
/// carry their own media and never receive a fabricated `SampleSpec`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsyncOperatorEdgePrepareContext {
    edge_id: Option<EdgeId>,
    port_name: String,
    direction: PortDirection,
    signal: SignalSpec,
    media: MediaCaps,
    edge_contract: EdgeContract,
    capacity_signals: usize,
}

impl AsyncOperatorEdgePrepareContext {
    pub fn new(
        edge_id: Option<EdgeId>,
        port_name: impl Into<String>,
        direction: PortDirection,
        signal: SignalSpec,
        media: MediaCaps,
        edge_contract: EdgeContract,
        capacity_signals: usize,
    ) -> Result<Self, NodeError> {
        let port_name = port_name.into();
        if port_name.trim().is_empty() {
            return Err(NodeError::Prepare(
                "async operator prepare port name cannot be empty".to_owned(),
            ));
        }
        if capacity_signals == 0 {
            return Err(NodeError::Prepare(format!(
                "async operator prepare port '{port_name}' has zero capacity"
            )));
        }
        signal
            .validate()
            .map_err(|error| NodeError::Prepare(error.to_string()))?;
        if !media.supports_signal(&signal) {
            return Err(NodeError::Prepare(format!(
                "async operator prepare port '{port_name}' has incompatible signal/media"
            )));
        }
        if !edge_contract.media.is_compatible_with(&media) {
            return Err(NodeError::Prepare(format!(
                "async operator prepare port '{port_name}' has incompatible edge media"
            )));
        }
        Ok(Self {
            edge_id,
            port_name,
            direction,
            signal,
            media,
            edge_contract,
            capacity_signals,
        })
    }

    pub const fn edge_id(&self) -> Option<EdgeId> {
        self.edge_id
    }

    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    pub const fn direction(&self) -> PortDirection {
        self.direction
    }

    pub const fn signal(&self) -> &SignalSpec {
        &self.signal
    }

    pub const fn media(&self) -> MediaCaps {
        self.media
    }

    pub const fn edge_contract(&self) -> EdgeContract {
        self.edge_contract
    }

    pub const fn capacity_signals(&self) -> usize {
        self.capacity_signals
    }
}

/// Complete graph-owned preparation contract for one asynchronous Operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsyncOperatorPrepareContext {
    execution_partition: ExecutionPartition,
    inputs: Vec<AsyncOperatorEdgePrepareContext>,
    outputs: Vec<AsyncOperatorEdgePrepareContext>,
}

impl AsyncOperatorPrepareContext {
    pub fn new(
        execution_partition: ExecutionPartition,
        edges: Vec<AsyncOperatorEdgePrepareContext>,
    ) -> Result<Self, NodeError> {
        if execution_partition.requires_realtime_safety() {
            return Err(NodeError::Prepare(
                "async operator cannot prepare in a realtime partition".to_owned(),
            ));
        }
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for edge in edges {
            match edge.direction() {
                PortDirection::Input => inputs.push(edge),
                PortDirection::Output => outputs.push(edge),
            }
        }
        if inputs.is_empty() || outputs.is_empty() {
            return Err(NodeError::Prepare(
                "async operator prepare requires bounded input and output edges".to_owned(),
            ));
        }
        Ok(Self {
            execution_partition,
            inputs,
            outputs,
        })
    }

    pub const fn execution_partition(&self) -> ExecutionPartition {
        self.execution_partition
    }

    pub fn inputs(&self) -> &[AsyncOperatorEdgePrepareContext] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[AsyncOperatorEdgePrepareContext] {
        &self.outputs
    }
}

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

/// Source-independent record of the signal consumed by an operator.
///
/// Derivation deliberately references the upstream typed-signal identity and
/// timing rather than `FrameLineage`. Audio is projected into these generic
/// contracts exactly once at the realtime-to-async boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalDerivation {
    pub upstream_lineage: SignalLineage,
    pub upstream_timing: SignalTiming,
    pub operator_id: OperatorId,
    pub operator_revision: u32,
    pub operator_generation: u32,
    pub connector_id: Option<ConnectorId>,
}

impl SignalDerivation {
    pub fn new(
        upstream_lineage: SignalLineage,
        upstream_timing: SignalTiming,
        operator_id: OperatorId,
        operator_revision: u32,
        operator_generation: u32,
        connector_id: Option<ConnectorId>,
    ) -> Result<Self, SignalDerivationError> {
        if operator_id.as_str().trim().is_empty() {
            return Err(SignalDerivationError::EmptyOperatorId);
        }
        if operator_revision == 0 || operator_generation == 0 {
            return Err(SignalDerivationError::ZeroOperatorVersion);
        }
        if upstream_timing
            .source_timestamp_ns
            .zip(upstream_timing.duration_ns)
            .is_some_and(|(start, duration)| start.checked_add(duration).is_none())
        {
            return Err(SignalDerivationError::InvalidTimestampRange);
        }
        Ok(Self {
            upstream_lineage,
            upstream_timing,
            operator_id,
            operator_revision,
            operator_generation,
            connector_id,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SignalDerivationError {
    #[error("derived signal upstream timing is invalid")]
    InvalidTimestampRange,
    #[error("derived signal operator id is empty")]
    EmptyOperatorId,
    #[error("derived signal operator revision and generation must be non-zero")]
    ZeroOperatorVersion,
}

#[derive(Debug)]
pub struct SignalEnvelope {
    pub payload: SignalPayload,
    pub spec: SignalSpec,
    pub timing: SignalTiming,
    pub lineage: Option<SignalLineage>,
    pub derivation: Option<SignalDerivation>,
}

impl SignalEnvelope {
    /// Creates an envelope for data that has not yet entered a source-aware
    /// Session. Session sources must attach lineage before routing it.
    pub fn untracked(payload: SignalPayload, observed_timestamp_ns: u64) -> Self {
        let spec = payload.signal_spec();
        Self {
            payload,
            spec,
            timing: SignalTiming::observed(observed_timestamp_ns),
            lineage: None,
            derivation: None,
        }
    }

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

    pub fn map_payload(mut self, payload: SignalPayload, spec: SignalSpec) -> Self {
        self.payload = payload;
        self.spec = spec;
        self
    }

    pub fn with_lineage(mut self, lineage: SignalLineage, timing: SignalTiming) -> Self {
        self.lineage = Some(lineage);
        self.timing = timing;
        self
    }

    pub fn with_derivation(mut self, derivation: SignalDerivation) -> Self {
        self.derivation = Some(derivation);
        self
    }

    pub fn sequence_number(&self) -> Option<u64> {
        self.lineage.map(|lineage| lineage.sequence_number).or(
            if let SignalPayload::Audio(frame) = &self.payload {
                Some(frame.sequence_number)
            } else {
                None
            },
        )
    }

    pub fn source_id(&self) -> Option<SourceId> {
        self.lineage.map(|lineage| lineage.source_id).or({
            if let SignalPayload::Audio(frame) = &self.payload {
                Some(frame.source_id)
            } else {
                None
            }
        })
    }

    pub fn timestamp_ns(&self) -> u64 {
        self.timing
            .session_timestamp_ns
            .or(self.timing.source_timestamp_ns)
            .unwrap_or(self.timing.observed_timestamp_ns)
    }

    pub fn validate(&self) -> Result<(), SignalEnvelopeError> {
        self.spec
            .validate()
            .map_err(|_| SignalEnvelopeError::InvalidSignalSpec)?;
        let payload_spec = self.payload.signal_spec();
        if payload_spec.class != self.spec.class {
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
pub enum SignalEnvelopeError {
    #[error("signal envelope has an invalid SignalSpec")]
    InvalidSignalSpec,
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
            .lineage
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
        cx: &'a AsyncOperatorPrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>>;

    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>>;

    /// Port-aware processing entry point used by composed Session operators.
    /// Existing one-input implementations remain source compatible.
    fn process_port<'a>(
        &'a mut self,
        _input_port: &'a str,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        self.process(input)
    }

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
    use crate::frame::{AudioBufferPool, StemId};

    fn prepare_cx() -> AsyncOperatorPrepareContext {
        let mut contract = EdgeContract::typed_default();
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

        let envelope = SignalEnvelope::untracked(SignalPayload::Control(Vec::new()), 9);
        let output = block_on_ready(node.process(envelope))
            .unwrap()
            .pop()
            .unwrap();

        assert_eq!(output.sequence_number(), None);
        assert_eq!(output.timestamp_ns(), 9);
        assert_eq!(output.spec, SignalSpec::control());
        assert!(matches!(output.payload, SignalPayload::Control(_)));
    }

    #[test]
    fn given_echo_async_node_when_process_before_prepare_then_error_is_returned() {
        let mut node = EchoAsyncNode { prepared: false };
        let envelope = SignalEnvelope::untracked(SignalPayload::Control(Vec::new()), 0);
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
            assert!(SignalEnvelope::untracked(payload, 2).validate().is_ok());
        }
    }

    #[test]
    fn given_payload_and_incompatible_spec_when_validated_then_rejected() {
        let mut envelope = SignalEnvelope::untracked(SignalPayload::Metrics(vec![]), 2);
        envelope.spec = SignalSpec::control();

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
        let envelope = SignalEnvelope::untracked(SignalPayload::Metrics(vec![]), 0)
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
        SignalEnvelope::untracked(SignalPayload::Metrics(vec![]), timestamp_ns).with_lineage(
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

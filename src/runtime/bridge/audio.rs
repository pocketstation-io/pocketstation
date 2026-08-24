use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, FrameLineage, LineagedAudioFrame, SampleSpec,
    SessionId, SourceId, StemId, StreamId, POOL_MAX_SLOTS,
};
use crate::graph::{SignalEnvelope, SignalPayload};
use crate::runtime::{
    PlanSourceSendError, PlanSourceSendOutcome, PlanSourceSender, TypedEdgeObservationHandle,
    TypedEdgeObservations, TypedEdgeReceiver,
};

const LOST_WAKEUP_FALLBACK_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Configures generated audio bridge behavior at its owning API boundary."]
pub struct GeneratedAudioBridgeSpec {
    #[doc = "Identifies the session identifier recorded by `GeneratedAudioBridgeSpec`."]
    pub session_id: SessionId,
    #[doc = "Identifies the stem identifier recorded by `GeneratedAudioBridgeSpec`."]
    pub stem_id: StemId,
    #[doc = "Identifies the stream identifier recorded by `GeneratedAudioBridgeSpec`."]
    pub stream_id: StreamId,
    #[doc = "Identifies the source identifier recorded by `GeneratedAudioBridgeSpec`."]
    pub source_id: SourceId,
    #[doc = "Identifies the clock identifier recorded by `GeneratedAudioBridgeSpec`."]
    pub clock_id: ClockDomainId,
    #[doc = "Declares the sample rate, channel layout, and format used by `GeneratedAudioBridgeSpec`."]
    pub sample_spec: SampleSpec,
    #[doc = "Stores the number of samples in each channel of a frame handled by `GeneratedAudioBridgeSpec`."]
    pub samples_per_frame: usize,
    #[doc = "Contains the pool slots owned or reported by `GeneratedAudioBridgeSpec`."]
    pub pool_slots: usize,
}

impl GeneratedAudioBridgeSpec {
    #[doc = "Validates `GeneratedAudioBridgeSpec` against its declared contract."]
    pub fn validate(self) -> Result<(), GeneratedAudioBridgeStartError> {
        if self.sample_spec.sample_rate_hz == 0 || self.sample_spec.channels == 0 {
            return Err(GeneratedAudioBridgeStartError::InvalidSampleSpec);
        }
        if self.samples_per_frame == 0 {
            return Err(GeneratedAudioBridgeStartError::ZeroFrameSamples);
        }
        if !(1..=POOL_MAX_SLOTS).contains(&self.pool_slots) {
            return Err(GeneratedAudioBridgeStartError::InvalidPoolSlots);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures produced during generated audio bridge lifecycle start."]
pub enum GeneratedAudioBridgeStartError {
    #[error("generated-audio bridge sample rate and channel count must be non-zero")]
    #[doc = "Reports that the supplied sample spec is invalid."]
    InvalidSampleSpec,
    #[error("generated-audio bridge samples per frame must be non-zero")]
    #[doc = "Reports that frame samples must be greater than zero."]
    ZeroFrameSamples,
    #[error("generated-audio bridge pool slots must be between 1 and 64")]
    #[doc = "Reports that the supplied pool slots is invalid."]
    InvalidPoolSlots,
    #[error("generated-audio bridge worker thread could not start")]
    #[doc = "Classifies a failure at the thread start stage or component of `GeneratedAudioBridgeStartError`."]
    ThreadStart,
}

#[derive(Default)]
struct GeneratedAudioBridgeObservationState {
    received_total: AtomicU64,
    normalized_total: AtomicU64,
    invalid_total: AtomicU64,
    shared_audio_rejected_total: AtomicU64,
    pool_exhausted_total: AtomicU64,
    ingress_rejected_total: AtomicU64,
    enqueued_total: AtomicU64,
    cancellation_total: AtomicU64,
    joined: AtomicBool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GeneratedAudioBridgeObservations {
    pub(crate) input_edge: TypedEdgeObservations,
    pub(crate) pool_slots: u64,
    pub(crate) frame_capacity_samples: u64,
    pub(crate) maximum_buffered_audio_bytes: u64,
    pub(crate) received_total: u64,
    pub(crate) normalized_total: u64,
    pub(crate) invalid_total: u64,
    pub(crate) shared_audio_rejected_total: u64,
    pub(crate) pool_exhausted_total: u64,
    pub(crate) ingress_rejected_total: u64,
    pub(crate) enqueued_total: u64,
    pub(crate) cancellation_total: u64,
    pub(crate) joined: bool,
}

#[derive(Clone)]
pub(crate) struct GeneratedAudioBridgeObservationHandle {
    state: Arc<GeneratedAudioBridgeObservationState>,
    input_edge: TypedEdgeObservationHandle,
    pool_slots: u64,
    frame_capacity_samples: u64,
}

impl GeneratedAudioBridgeObservationHandle {
    pub fn snapshot(&self) -> GeneratedAudioBridgeObservations {
        let input_edge = self.input_edge.snapshot();
        GeneratedAudioBridgeObservations {
            input_edge,
            pool_slots: self.pool_slots,
            frame_capacity_samples: self.frame_capacity_samples,
            maximum_buffered_audio_bytes: input_edge
                .capacity_signals
                .saturating_add(self.pool_slots)
                .saturating_mul(self.frame_capacity_samples)
                .saturating_mul(std::mem::size_of::<f32>() as u64),
            received_total: self.state.received_total.load(Ordering::Relaxed),
            normalized_total: self.state.normalized_total.load(Ordering::Relaxed),
            invalid_total: self.state.invalid_total.load(Ordering::Relaxed),
            shared_audio_rejected_total: self
                .state
                .shared_audio_rejected_total
                .load(Ordering::Relaxed),
            pool_exhausted_total: self.state.pool_exhausted_total.load(Ordering::Relaxed),
            ingress_rejected_total: self.state.ingress_rejected_total.load(Ordering::Relaxed),
            enqueued_total: self.state.enqueued_total.load(Ordering::Relaxed),
            cancellation_total: self.state.cancellation_total.load(Ordering::Relaxed),
            joined: self.state.joined.load(Ordering::Acquire),
        }
    }
}

#[doc = "Transfers generated audio across the bounded runtime boundary it owns."]
pub struct GeneratedAudioBridge {
    stem_id: StemId,
    cancellation: Arc<AtomicBool>,
    observations: GeneratedAudioBridgeObservationHandle,
    join: Option<JoinHandle<()>>,
}

impl GeneratedAudioBridge {
    #[doc = "Spawns its owned operation for `GeneratedAudioBridge`."]
    pub fn spawn(
        receiver: TypedEdgeReceiver,
        sender: PlanSourceSender,
        specification: GeneratedAudioBridgeSpec,
    ) -> Result<Self, GeneratedAudioBridgeStartError> {
        specification.validate()?;
        let input_edge = receiver.observation_handle();
        let pool = AudioBufferPool::new(specification.pool_slots, specification.samples_per_frame);
        let cancellation = Arc::new(AtomicBool::new(false));
        let state = Arc::new(GeneratedAudioBridgeObservationState::default());
        let worker_cancellation = Arc::clone(&cancellation);
        let worker_state = Arc::clone(&state);
        let join = thread::Builder::new()
            .name("pks-generated-audio".to_owned())
            .spawn(move || {
                run_bridge(
                    receiver,
                    sender,
                    specification,
                    pool,
                    worker_cancellation,
                    Arc::clone(&worker_state),
                );
                worker_state.joined.store(true, Ordering::Release);
            })
            .map_err(|_| GeneratedAudioBridgeStartError::ThreadStart)?;
        Ok(Self {
            stem_id: specification.stem_id,
            cancellation,
            observations: GeneratedAudioBridgeObservationHandle {
                state,
                input_edge,
                pool_slots: specification.pool_slots as u64,
                frame_capacity_samples: specification.samples_per_frame as u64,
            },
            join: Some(join),
        })
    }

    pub(crate) const fn stem_id(&self) -> StemId {
        self.stem_id
    }

    pub(crate) fn observations(&self) -> GeneratedAudioBridgeObservationHandle {
        self.observations.clone()
    }

    #[doc = "Finishes input to `GeneratedAudioBridge`, joins its worker, and returns the terminal result."]
    pub fn finish_and_join(mut self) {
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }

    #[doc = "Cancels and join for `GeneratedAudioBridge`."]
    pub fn cancel_and_join(mut self) {
        self.cancellation.store(true, Ordering::Release);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

impl Drop for GeneratedAudioBridge {
    #[doc = "Releases resources owned by `GeneratedAudioBridge`."]
    fn drop(&mut self) {
        self.cancellation.store(true, Ordering::Release);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

fn run_bridge(
    mut receiver: TypedEdgeReceiver,
    mut sender: PlanSourceSender,
    specification: GeneratedAudioBridgeSpec,
    pool: Arc<AudioBufferPool>,
    cancellation: Arc<AtomicBool>,
    observations: Arc<GeneratedAudioBridgeObservationState>,
) {
    let _ = receiver.register_current_thread();
    loop {
        if cancellation.load(Ordering::Acquire) {
            observations
                .cancellation_total
                .fetch_add(1, Ordering::Relaxed);
            break;
        }
        let shared = match receiver.recv() {
            Some(shared) => shared,
            None if receiver.is_abandoned() => {
                let Some(shared) = receiver.recv() else {
                    break;
                };
                shared
            }
            None => {
                thread::park_timeout(LOST_WAKEUP_FALLBACK_INTERVAL);
                continue;
            }
        };
        observations.received_total.fetch_add(1, Ordering::Relaxed);
        let Ok(envelope) = Arc::try_unwrap(shared) else {
            observations
                .shared_audio_rejected_total
                .fetch_add(1, Ordering::Relaxed);
            continue;
        };
        let SignalEnvelope {
            payload,
            timing,
            lineage: signal_lineage,
            ..
        } = envelope;
        let SignalPayload::Audio(input) = payload else {
            observations.invalid_total.fetch_add(1, Ordering::Relaxed);
            continue;
        };
        let Some(sequence_number) = signal_lineage.map(|lineage| lineage.sequence_number) else {
            observations.invalid_total.fetch_add(1, Ordering::Relaxed);
            continue;
        };
        if input.sample_rate_hz != specification.sample_spec.sample_rate_hz
            || input.channels != specification.sample_spec.channels
            || input.format != specification.sample_spec.format
            || input.buffer.len() != specification.samples_per_frame
        {
            observations.invalid_total.fetch_add(1, Ordering::Relaxed);
            continue;
        }
        let Some(mut buffer) = pool.acquire() else {
            observations
                .pool_exhausted_total
                .fetch_add(1, Ordering::Relaxed);
            continue;
        };
        if buffer.try_copy_from_slice(input.buffer.as_slice()).is_err() {
            observations.invalid_total.fetch_add(1, Ordering::Relaxed);
            continue;
        }
        let timestamp_ns = timing
            .session_timestamp_ns
            .or(timing.source_timestamp_ns)
            .unwrap_or(timing.observed_timestamp_ns);
        let mut normalized = AudioFrame::new(
            specification.stream_id,
            specification.source_id,
            sequence_number,
            timestamp_ns,
            specification.sample_spec.channels,
            buffer,
        );
        normalized.sample_rate_hz = specification.sample_spec.sample_rate_hz;
        normalized.format = specification.sample_spec.format;
        let per_channel_samples = specification
            .samples_per_frame
            .checked_div(usize::from(specification.sample_spec.channels))
            .unwrap_or(0);
        let duration_ns = u64::try_from(per_channel_samples)
            .unwrap_or(u64::MAX)
            .saturating_mul(1_000_000_000)
            / u64::from(specification.sample_spec.sample_rate_hz);
        let lineage = FrameLineage {
            session_id: specification.session_id,
            source_id: specification.source_id,
            stem_id: specification.stem_id,
            clock_id: signal_lineage.map_or(specification.clock_id, |lineage| lineage.clock_id),
            sequence_num: sequence_number,
            timestamp_start_ns: timestamp_ns,
            duration_ns,
            source_generation: signal_lineage.map_or(1, |lineage| lineage.source_generation),
            discontinuity_epoch: signal_lineage.map_or(0, |lineage| lineage.discontinuity_epoch),
            permission_epoch: signal_lineage.map_or(0, |lineage| lineage.policy_epoch),
        };
        let Ok(frame) = LineagedAudioFrame::new(normalized, lineage) else {
            observations.invalid_total.fetch_add(1, Ordering::Relaxed);
            continue;
        };
        observations
            .normalized_total
            .fetch_add(1, Ordering::Relaxed);
        match sender.try_send(frame) {
            PlanSourceSendOutcome::Enqueued => {
                observations.enqueued_total.fetch_add(1, Ordering::Relaxed);
            }
            PlanSourceSendOutcome::Rejected { error, frame } => {
                drop(frame);
                match error {
                    PlanSourceSendError::Cancelled | PlanSourceSendError::Full => {
                        observations
                            .ingress_rejected_total
                            .fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{SampleFormat, SAMPLE_RATE_HZ};
    use crate::graph::{EdgeContract, SignalEnvelope};
    use crate::runtime::{
        plan_source_channel, PlanRunnerCancellation, TypedEdgeBranchSpec, TypedEdgeFanout,
    };

    #[test]
    fn given_operator_audio_when_bridged_then_owned_frame_enters_bounded_plan_source() {
        let cancellation = PlanRunnerCancellation::new();
        let (sender, _input) =
            plan_source_channel(crate::graph::NodeId(1), 2, cancellation).expect("source channel");
        let (mut fanout, mut receivers) = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 1,
            edge_contract: EdgeContract::bounded_async(),
        }])
        .expect("typed edge");
        let specification = GeneratedAudioBridgeSpec {
            session_id: SessionId(1),
            stem_id: StemId(2),
            stream_id: StreamId(3),
            source_id: SourceId(4),
            clock_id: ClockDomainId(5),
            sample_spec: SampleSpec::new(SAMPLE_RATE_HZ, 1, SampleFormat::F32Interleaved),
            samples_per_frame: 960,
            pool_slots: 2,
        };
        let bridge = GeneratedAudioBridge::spawn(receivers.remove(0), sender, specification)
            .expect("bridge");
        let input_pool = AudioBufferPool::new(1, 960);
        let frame = AudioFrame::new(
            StreamId(30),
            SourceId(40),
            7,
            10,
            1,
            input_pool.acquire().expect("input buffer"),
        );
        fanout
            .publish(
                SignalEnvelope::from_audio(
                    frame,
                    Some(FrameLineage {
                        session_id: SessionId(9),
                        source_id: SourceId(40),
                        stem_id: StemId(10),
                        clock_id: ClockDomainId(11),
                        sequence_num: 7,
                        timestamp_start_ns: 10,
                        duration_ns: 20_000_000,
                        source_generation: 1,
                        discontinuity_epoch: 0,
                        permission_epoch: 1,
                    }),
                ),
                false,
            )
            .expect("publish");
        for _ in 0..100 {
            if bridge.observations().snapshot().enqueued_total == 1 {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
        let observations = bridge.observations();
        drop(fanout);
        bridge.finish_and_join();
        let observations = observations.snapshot();
        assert_eq!(observations.received_total, 1);
        assert_eq!(observations.normalized_total, 1);
        assert_eq!(observations.enqueued_total, 1);
        assert_eq!(observations.invalid_total, 0);
        assert_eq!(observations.pool_exhausted_total, 0);
        assert_eq!(observations.input_edge.capacity_signals, 1);
        assert_eq!(observations.input_edge.received_total, 1);
        assert_eq!(observations.maximum_buffered_audio_bytes, 3 * 960 * 4);
        assert_eq!(observations.cancellation_total, 0);
        assert!(observations.joined);
    }

    fn publish_audio(
        fanout: &mut TypedEdgeFanout,
        sequence_number: u64,
        input_pool: &Arc<AudioBufferPool>,
    ) {
        let frame = AudioFrame::new(
            StreamId(30),
            SourceId(40),
            sequence_number,
            sequence_number.saturating_mul(20_000_000),
            1,
            input_pool.acquire().expect("input buffer"),
        );
        fanout
            .publish(
                SignalEnvelope::from_audio(
                    frame,
                    Some(
                        FrameLineage::try_new(
                            SessionId(9),
                            SourceId(40),
                            StemId(10),
                            ClockDomainId(11),
                            sequence_number,
                            sequence_number.saturating_mul(20_000_000),
                            20_000_000,
                            1,
                            0,
                            1,
                        )
                        .expect("lineage"),
                    ),
                ),
                false,
            )
            .expect("publish");
    }

    fn bridge_specification(pool_slots: usize) -> GeneratedAudioBridgeSpec {
        GeneratedAudioBridgeSpec {
            session_id: SessionId(1),
            stem_id: StemId(2),
            stream_id: StreamId(3),
            source_id: SourceId(4),
            clock_id: ClockDomainId(5),
            sample_spec: SampleSpec::new(SAMPLE_RATE_HZ, 1, SampleFormat::F32Interleaved),
            samples_per_frame: 960,
            pool_slots,
        }
    }

    #[test]
    fn given_retained_audio_ingress_when_pool_is_exhausted_then_loss_is_counted_exactly() {
        let cancellation = PlanRunnerCancellation::new();
        let (sender, _retained_input) =
            plan_source_channel(crate::graph::NodeId(1), 4, cancellation).expect("source channel");
        let (mut fanout, mut receivers) = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 2,
            edge_contract: EdgeContract::bounded_async(),
        }])
        .expect("typed edge");
        let bridge =
            GeneratedAudioBridge::spawn(receivers.remove(0), sender, bridge_specification(1))
                .expect("bridge");
        let observations = bridge.observations();
        let input_pool = AudioBufferPool::new(2, 960);
        publish_audio(&mut fanout, 1, &input_pool);
        publish_audio(&mut fanout, 2, &input_pool);
        drop(fanout);

        bridge.finish_and_join();
        let observations = observations.snapshot();
        assert_eq!(
            observations.received_total, 2,
            "unexpected bridge observations: {observations:?}"
        );
        assert_eq!(observations.normalized_total, 1);
        assert_eq!(observations.enqueued_total, 1);
        assert_eq!(observations.pool_exhausted_total, 1);
        assert_eq!(observations.ingress_rejected_total, 0);
        assert!(
            (1..=2).contains(&observations.input_edge.peak_depth_signals),
            "unexpected peak input depth: {}",
            observations.input_edge.peak_depth_signals
        );
        assert_eq!(observations.maximum_buffered_audio_bytes, 3 * 960 * 4);
        assert!(observations.joined);
    }

    #[test]
    fn given_full_audio_ingress_when_bridge_sends_then_rejection_is_counted_exactly() {
        let cancellation = PlanRunnerCancellation::new();
        let (sender, _retained_input) =
            plan_source_channel(crate::graph::NodeId(1), 1, cancellation).expect("source channel");
        let (mut fanout, mut receivers) = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 2,
            edge_contract: EdgeContract::bounded_async(),
        }])
        .expect("typed edge");
        let bridge =
            GeneratedAudioBridge::spawn(receivers.remove(0), sender, bridge_specification(2))
                .expect("bridge");
        let observations = bridge.observations();
        let input_pool = AudioBufferPool::new(2, 960);
        publish_audio(&mut fanout, 1, &input_pool);
        publish_audio(&mut fanout, 2, &input_pool);
        drop(fanout);

        bridge.finish_and_join();
        let observations = observations.snapshot();
        assert_eq!(observations.received_total, 2);
        assert_eq!(observations.normalized_total, 2);
        assert_eq!(observations.enqueued_total, 1);
        assert_eq!(observations.pool_exhausted_total, 0);
        assert_eq!(observations.ingress_rejected_total, 1);
        assert_eq!(observations.maximum_buffered_audio_bytes, 4 * 960 * 4);
        assert!(observations.joined);
    }
}

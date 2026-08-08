use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, FrameLineage, LineagedAudioFrame, SampleSpec,
    SessionId, SourceId, StemId, StreamId, POOL_MAX_SLOTS,
};
use crate::graph::{SignalEnvelope, SignalPayload};
use crate::runtime::{PlanSourceSendOutcome, PlanSourceSender, TypedEdgeReceiver};

const IDLE_POLL_INTERVAL: Duration = Duration::from_millis(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedAudioBridgeSpec {
    pub session_id: SessionId,
    pub stem_id: StemId,
    pub stream_id: StreamId,
    pub source_id: SourceId,
    pub clock_id: ClockDomainId,
    pub sample_spec: SampleSpec,
    pub samples_per_frame: usize,
    pub pool_slots: usize,
}

impl GeneratedAudioBridgeSpec {
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
pub enum GeneratedAudioBridgeStartError {
    #[error("generated-audio bridge sample rate and channel count must be non-zero")]
    InvalidSampleSpec,
    #[error("generated-audio bridge samples per frame must be non-zero")]
    ZeroFrameSamples,
    #[error("generated-audio bridge pool slots must be between 1 and 64")]
    InvalidPoolSlots,
    #[error("generated-audio bridge worker thread could not start")]
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
pub struct GeneratedAudioBridgeObservations {
    pub received_total: u64,
    pub normalized_total: u64,
    pub invalid_total: u64,
    pub shared_audio_rejected_total: u64,
    pub pool_exhausted_total: u64,
    pub ingress_rejected_total: u64,
    pub enqueued_total: u64,
    pub cancellation_total: u64,
    pub joined: bool,
}

#[derive(Clone)]
pub struct GeneratedAudioBridgeObservationHandle {
    state: Arc<GeneratedAudioBridgeObservationState>,
}

impl GeneratedAudioBridgeObservationHandle {
    pub fn snapshot(&self) -> GeneratedAudioBridgeObservations {
        GeneratedAudioBridgeObservations {
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

pub struct GeneratedAudioBridge {
    cancellation: Arc<AtomicBool>,
    observations: GeneratedAudioBridgeObservationHandle,
    join: Option<JoinHandle<()>>,
}

impl GeneratedAudioBridge {
    pub fn spawn(
        receiver: TypedEdgeReceiver,
        sender: PlanSourceSender,
        specification: GeneratedAudioBridgeSpec,
    ) -> Result<Self, GeneratedAudioBridgeStartError> {
        specification.validate()?;
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
            cancellation,
            observations: GeneratedAudioBridgeObservationHandle { state },
            join: Some(join),
        })
    }

    pub fn observations(&self) -> GeneratedAudioBridgeObservationHandle {
        self.observations.clone()
    }

    pub fn cancel_and_join(mut self) {
        self.cancellation.store(true, Ordering::Release);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

impl Drop for GeneratedAudioBridge {
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
    loop {
        if cancellation.load(Ordering::Acquire) {
            observations
                .cancellation_total
                .fetch_add(1, Ordering::Relaxed);
            break;
        }
        let Some(shared) = receiver.recv() else {
            if receiver.is_abandoned() {
                break;
            }
            thread::park_timeout(IDLE_POLL_INTERVAL);
            continue;
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
        buffer.copy_from_slice(input.buffer.as_slice());
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
        normalized.source_tag = input.source_tag;
        normalized.encryption_mode = input.encryption_mode;
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
            clock_id: specification.clock_id,
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
            PlanSourceSendOutcome::Rejected { .. } => {
                observations
                    .ingress_rejected_total
                    .fetch_add(1, Ordering::Relaxed);
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
    fn given_generated_audio_when_bridged_then_owned_frame_enters_bounded_plan_source() {
        let cancellation = PlanRunnerCancellation::new();
        let (sender, _input) =
            plan_source_channel(crate::graph::NodeId(1), 2, cancellation).expect("source channel");
        let (mut fanout, mut receivers) = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 1,
            edge_contract: EdgeContract::typed_default(),
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
        bridge.cancel_and_join();
        let observations = observations.snapshot();
        assert_eq!(observations.received_total, 1);
        assert_eq!(observations.normalized_total, 1);
        assert_eq!(observations.enqueued_total, 1);
        assert_eq!(observations.invalid_total, 0);
        assert_eq!(observations.pool_exhausted_total, 0);
        assert!(observations.joined);
    }
}

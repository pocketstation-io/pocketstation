//! Runtime observation bindings and finalized Session-level snapshots.

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::capture::CaptureObservationReceipt;
use crate::endpoint::EndpointDriverObservations;
use crate::frame::{EndpointId, LineagedAudioFrame, RouteId, StemId};
use crate::runtime::{
    AsyncOperatorObservations, AsyncOperatorOutputObservationHandle, EdgeObservations,
    PlanEdgeObservationHandle, PlanSourceObservationHandle,
};
use crate::session::declaration::OperatorInstanceId;

use super::observations::{
    SessionDerivedRouteMetrics, SessionExternalSourceMetrics, SessionOperatorInputMetrics,
    SessionOperatorMetrics, SessionRouteMetrics, SessionSourceActivityObservations,
    SessionSourceMetrics, SessionSourceSignalAtomicSnapshot, SessionSourceSignalObservations,
};

#[derive(Default)]
struct SourceActivityObservationState {
    session_started_at_ns: u64,
    first_frame_received_at_ns: AtomicU64,
    latest_frame_received_at_ns: AtomicU64,
    frames_received_total: AtomicU64,
}

#[derive(Clone)]
pub(super) struct SourceActivityObservationHandle {
    state: Arc<SourceActivityObservationState>,
}

impl SourceActivityObservationHandle {
    pub(super) fn new(session_started_at_ns: u64) -> Self {
        Self {
            state: Arc::new(SourceActivityObservationState {
                session_started_at_ns,
                ..SourceActivityObservationState::default()
            }),
        }
    }

    /// Records activity on the Session runtime worker after capture dequeue.
    /// The runtime has one writer per built-in Source.
    pub(super) fn observe_frame(&self, received_at_ns: u64) {
        if self.state.frames_received_total.load(Ordering::Relaxed) == 0 {
            self.state
                .first_frame_received_at_ns
                .store(received_at_ns, Ordering::Relaxed);
        }
        self.state
            .latest_frame_received_at_ns
            .store(received_at_ns, Ordering::Relaxed);
        let _ = self.state.frames_received_total.fetch_update(
            Ordering::Release,
            Ordering::Relaxed,
            |current| Some(current.saturating_add(1)),
        );
    }

    pub(super) fn observations(&self, observed_at_ns: u64) -> SessionSourceActivityObservations {
        let frames_received_total = self.state.frames_received_total.load(Ordering::Acquire);
        SessionSourceActivityObservations {
            session_started_at_ns: self.state.session_started_at_ns,
            observed_at_ns,
            first_frame_received_at_ns: (frames_received_total > 0).then(|| {
                self.state
                    .first_frame_received_at_ns
                    .load(Ordering::Relaxed)
            }),
            latest_frame_received_at_ns: (frames_received_total > 0).then(|| {
                self.state
                    .latest_frame_received_at_ns
                    .load(Ordering::Relaxed)
            }),
            frames_received_total,
        }
    }
}

#[derive(Default)]
struct SourceSignalObservationState {
    write_sequence: AtomicU64,
    samples_observed_total: AtomicU64,
    exact_zero_samples_observed_total: AtomicU64,
    nonzero_samples_observed_total: AtomicU64,
    nonfinite_samples_observed_total: AtomicU64,
    window_timestamp_start_ns: AtomicU64,
    window_duration_ns: AtomicU64,
    window_observed_at_ns: AtomicU64,
    window_sequence_number: AtomicU64,
    window_source_generation: AtomicU32,
    window_discontinuity_epoch: AtomicU64,
    window_samples_total: AtomicU64,
    window_exact_zero_samples_total: AtomicU64,
    window_nonzero_samples_total: AtomicU64,
    window_nonfinite_samples_total: AtomicU64,
    window_peak_linear_bits: AtomicU32,
    window_mean_square_linear_bits: AtomicU64,
    consecutive_exact_zero_duration_ns: AtomicU64,
}

#[derive(Clone, Default)]
pub(super) struct SourceSignalObservationHandle {
    state: Arc<SourceSignalObservationState>,
}

impl SourceSignalObservationHandle {
    /// Measures one canonical frame on the Session runtime worker after
    /// capture dequeue. This never runs in a native capture callback.
    pub(super) fn observe_frame(&self, observed_at_ns: u64, frame: &LineagedAudioFrame) {
        let lineage = frame.lineage();
        let samples = frame.frame().samples();
        let mut exact_zero_samples_total = 0u64;
        let mut nonzero_samples_total = 0u64;
        let mut nonfinite_samples_total = 0u64;
        let mut peak_linear = 0.0f32;
        let mut sum_squares = 0.0f64;

        for sample in samples {
            if !sample.is_finite() {
                nonfinite_samples_total = nonfinite_samples_total.saturating_add(1);
                continue;
            }
            if *sample == 0.0 {
                exact_zero_samples_total = exact_zero_samples_total.saturating_add(1);
            } else {
                nonzero_samples_total = nonzero_samples_total.saturating_add(1);
            }
            peak_linear = peak_linear.max(sample.abs());
            sum_squares += f64::from(*sample) * f64::from(*sample);
        }

        let finite_samples_total = exact_zero_samples_total.saturating_add(nonzero_samples_total);
        let mean_square_linear = if finite_samples_total == 0 {
            0.0
        } else {
            sum_squares / finite_samples_total as f64
        };
        let sample_count = u64::try_from(samples.len()).unwrap_or(u64::MAX);
        let previous_samples_total = self.state.samples_observed_total.load(Ordering::Relaxed);
        let previous_generation = self.state.window_source_generation.load(Ordering::Relaxed);
        let previous_discontinuity = self
            .state
            .window_discontinuity_epoch
            .load(Ordering::Relaxed);
        let same_continuity = previous_samples_total > 0
            && previous_generation == lineage.source_generation
            && previous_discontinuity == lineage.discontinuity_epoch;
        let frame_is_exact_zero = sample_count > 0
            && exact_zero_samples_total == sample_count
            && nonfinite_samples_total == 0;
        let consecutive_exact_zero_duration_ns = if frame_is_exact_zero && same_continuity {
            self.state
                .consecutive_exact_zero_duration_ns
                .load(Ordering::Relaxed)
                .saturating_add(lineage.duration_ns)
        } else if frame_is_exact_zero {
            lineage.duration_ns
        } else {
            0
        };

        let _ = self.state.write_sequence.fetch_add(1, Ordering::AcqRel);
        self.state.samples_observed_total.store(
            previous_samples_total.saturating_add(sample_count),
            Ordering::Relaxed,
        );
        saturating_add_atomic(
            &self.state.exact_zero_samples_observed_total,
            exact_zero_samples_total,
        );
        saturating_add_atomic(
            &self.state.nonzero_samples_observed_total,
            nonzero_samples_total,
        );
        saturating_add_atomic(
            &self.state.nonfinite_samples_observed_total,
            nonfinite_samples_total,
        );
        self.state
            .window_timestamp_start_ns
            .store(lineage.timestamp_start_ns, Ordering::Relaxed);
        self.state
            .window_duration_ns
            .store(lineage.duration_ns, Ordering::Relaxed);
        self.state
            .window_observed_at_ns
            .store(observed_at_ns, Ordering::Relaxed);
        self.state
            .window_sequence_number
            .store(lineage.sequence_num, Ordering::Relaxed);
        self.state
            .window_source_generation
            .store(lineage.source_generation, Ordering::Relaxed);
        self.state
            .window_discontinuity_epoch
            .store(lineage.discontinuity_epoch, Ordering::Relaxed);
        self.state
            .window_samples_total
            .store(sample_count, Ordering::Relaxed);
        self.state
            .window_exact_zero_samples_total
            .store(exact_zero_samples_total, Ordering::Relaxed);
        self.state
            .window_nonzero_samples_total
            .store(nonzero_samples_total, Ordering::Relaxed);
        self.state
            .window_nonfinite_samples_total
            .store(nonfinite_samples_total, Ordering::Relaxed);
        self.state
            .window_peak_linear_bits
            .store(peak_linear.to_bits(), Ordering::Relaxed);
        self.state
            .window_mean_square_linear_bits
            .store(mean_square_linear.to_bits(), Ordering::Relaxed);
        self.state
            .consecutive_exact_zero_duration_ns
            .store(consecutive_exact_zero_duration_ns, Ordering::Relaxed);
        let _ = self.state.write_sequence.fetch_add(1, Ordering::Release);
    }

    pub(super) fn observations(&self, observed_at_ns: u64) -> SessionSourceSignalObservations {
        loop {
            let sequence_before = self.state.write_sequence.load(Ordering::Acquire);
            if !sequence_before.is_multiple_of(2) {
                std::hint::spin_loop();
                continue;
            }
            let samples_observed_total = self.state.samples_observed_total.load(Ordering::Relaxed);
            let observations = SessionSourceSignalObservations::from_atomic_snapshot(
                SessionSourceSignalAtomicSnapshot {
                    observed_at_ns,
                    samples_observed_total,
                    exact_zero_samples_observed_total: self
                        .state
                        .exact_zero_samples_observed_total
                        .load(Ordering::Relaxed),
                    nonzero_samples_observed_total: self
                        .state
                        .nonzero_samples_observed_total
                        .load(Ordering::Relaxed),
                    nonfinite_samples_observed_total: self
                        .state
                        .nonfinite_samples_observed_total
                        .load(Ordering::Relaxed),
                    window_timestamp_start_ns: self
                        .state
                        .window_timestamp_start_ns
                        .load(Ordering::Relaxed),
                    window_duration_ns: self.state.window_duration_ns.load(Ordering::Relaxed),
                    window_observed_at_ns: self.state.window_observed_at_ns.load(Ordering::Relaxed),
                    window_sequence_number: self
                        .state
                        .window_sequence_number
                        .load(Ordering::Relaxed),
                    window_source_generation: self
                        .state
                        .window_source_generation
                        .load(Ordering::Relaxed),
                    window_discontinuity_epoch: self
                        .state
                        .window_discontinuity_epoch
                        .load(Ordering::Relaxed),
                    window_samples_total: self.state.window_samples_total.load(Ordering::Relaxed),
                    window_exact_zero_samples_total: self
                        .state
                        .window_exact_zero_samples_total
                        .load(Ordering::Relaxed),
                    window_nonzero_samples_total: self
                        .state
                        .window_nonzero_samples_total
                        .load(Ordering::Relaxed),
                    window_nonfinite_samples_total: self
                        .state
                        .window_nonfinite_samples_total
                        .load(Ordering::Relaxed),
                    window_peak_linear_bits: self
                        .state
                        .window_peak_linear_bits
                        .load(Ordering::Relaxed),
                    window_mean_square_linear_bits: self
                        .state
                        .window_mean_square_linear_bits
                        .load(Ordering::Relaxed),
                    consecutive_exact_zero_duration_ns: self
                        .state
                        .consecutive_exact_zero_duration_ns
                        .load(Ordering::Relaxed),
                },
            );
            let sequence_after = self.state.write_sequence.load(Ordering::Acquire);
            if sequence_before == sequence_after {
                return observations;
            }
        }
    }
}

fn saturating_add_atomic(value: &AtomicU64, increment: u64) {
    let current = value.load(Ordering::Relaxed);
    value.store(current.saturating_add(increment), Ordering::Relaxed);
}

pub(super) struct SourceObservationBinding {
    pub(super) stem_id: StemId,
    pub(super) capture: SourceCaptureObservationHandle,
    pub(super) ingress: PlanSourceObservationHandle,
    pub(super) activity: SourceActivityObservationHandle,
    pub(super) signal: SourceSignalObservationHandle,
}

struct SourceCaptureObservationState {
    current: Option<CaptureObservationReceipt>,
    attempts_total: u64,
    completed_total: u64,
    failed_before_attach_total: u64,
    response_timeouts_total: u64,
    attached_source_id: Option<crate::frame::SourceId>,
    source_generation: u32,
    discontinuity_epoch: u64,
    latest_completed_at_ns: Option<u64>,
}

/// Control-path indirection for the currently attached physical capture.
/// Replacement writes occur only on the Session runtime worker; metric reads
/// occur on caller control threads. Native callbacks never touch this lock.
#[derive(Clone)]
pub(super) struct SourceCaptureObservationHandle {
    state: Arc<Mutex<SourceCaptureObservationState>>,
}

impl SourceCaptureObservationHandle {
    pub(super) fn new(
        current: CaptureObservationReceipt,
        metadata: crate::capture::CaptureOpenMetadata,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(SourceCaptureObservationState {
                current: Some(current),
                attempts_total: 0,
                completed_total: 0,
                failed_before_attach_total: 0,
                response_timeouts_total: 0,
                attached_source_id: Some(metadata.source_id),
                source_generation: metadata.source_generation.0,
                discontinuity_epoch: metadata.discontinuity_epoch,
                latest_completed_at_ns: None,
            })),
        }
    }

    pub(super) fn observe_attempt(&self) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.attempts_total = state.attempts_total.saturating_add(1);
    }

    pub(super) fn observe_failed_before_attach(&self) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.failed_before_attach_total = state.failed_before_attach_total.saturating_add(1);
    }

    pub(super) fn observe_response_timeout(&self) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.response_timeouts_total = state.response_timeouts_total.saturating_add(1);
    }

    pub(super) fn replace(
        &self,
        current: CaptureObservationReceipt,
        metadata: crate::capture::CaptureOpenMetadata,
        completed_at_ns: u64,
    ) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.current = Some(current);
        state.completed_total = state.completed_total.saturating_add(1);
        state.attached_source_id = Some(metadata.source_id);
        state.source_generation = metadata.source_generation.0;
        state.discontinuity_epoch = metadata.discontinuity_epoch;
        state.latest_completed_at_ns = Some(completed_at_ns);
    }

    pub(super) fn detach(
        &self,
        source_generation: crate::capture::SourceGeneration,
        discontinuity_epoch: u64,
    ) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.current = None;
        state.attached_source_id = None;
        state.source_generation = source_generation.0;
        state.discontinuity_epoch = discontinuity_epoch;
    }

    pub(super) fn observations(&self) -> crate::capture::CaptureOwnerObservations {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .current
            .as_ref()
            .map_or_else(
                crate::capture::CaptureOwnerObservations::default,
                |current| current.observations(),
            )
    }

    pub(super) fn opened_native_format(&self) -> Option<crate::capture::CaptureNativeFormat> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .current
            .as_ref()
            .and_then(CaptureObservationReceipt::opened_native_format)
    }

    pub(super) fn replacement_observations(
        &self,
        stem_id: StemId,
    ) -> super::observations::SessionSourceReplacementObservations {
        let state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        super::observations::SessionSourceReplacementObservations {
            stem_id,
            attempts_total: state.attempts_total,
            completed_total: state.completed_total,
            failed_before_attach_total: state.failed_before_attach_total,
            response_timeouts_total: state.response_timeouts_total,
            attached_source_id: state.attached_source_id,
            source_generation: state.source_generation,
            discontinuity_epoch: state.discontinuity_epoch,
            latest_completed_at_ns: state.latest_completed_at_ns,
        }
    }
}

pub(super) struct RouteObservationBinding {
    pub(super) route_id: RouteId,
    pub(super) endpoint_id: EndpointId,
    pub(super) edge: PlanEdgeObservationHandle,
}

pub(super) struct DerivedRouteObservationBinding {
    pub(super) route_id: RouteId,
    pub(super) endpoint_id: EndpointId,
    pub(super) output: AsyncOperatorOutputObservationHandle,
}

pub(super) type IndexedSessionMetrics = (
    Box<[SessionSourceMetrics]>,
    Box<[SessionExternalSourceMetrics]>,
    Box<[SessionRouteMetrics]>,
    Box<[SessionOperatorMetrics]>,
    Box<[SessionDerivedRouteMetrics]>,
);

#[derive(Clone, Copy)]
pub(super) struct FinalEndpointObservation {
    pub(super) route_id: RouteId,
    pub(super) endpoint_id: EndpointId,
    pub(super) observations: EndpointDriverObservations,
    pub(super) finalization_failures_total: u64,
}

#[derive(Clone)]
pub(super) struct FinalOperatorObservation {
    pub(super) operator_instance_id: OperatorInstanceId,
    pub(super) input_delivery: EdgeObservations,
    pub(super) input_ports: Box<[SessionOperatorInputMetrics]>,
    pub(super) observations: AsyncOperatorObservations,
    pub(super) finalization_failures_total: u64,
}

#[cfg(test)]
mod tests {
    use super::SourceSignalObservationHandle;
    use crate::frame::{
        AudioBufferPool, AudioFrame, ClockDomainId, FrameLineage, LineagedAudioFrame, SessionId,
        SourceId, StemId, StreamId,
    };

    fn frame(
        samples: &[f32],
        sequence_number: u64,
        source_generation: u32,
        discontinuity_epoch: u64,
    ) -> LineagedAudioFrame {
        let pool = AudioBufferPool::new(1, samples.len());
        let mut buffer = pool.acquire().expect("test pool has one buffer");
        buffer
            .try_copy_from_slice(samples)
            .expect("test samples fit the buffer");
        let timestamp_start_ns = sequence_number.saturating_mul(20_000_000);
        LineagedAudioFrame::new(
            AudioFrame::new(
                StreamId(1),
                SourceId(2),
                sequence_number,
                timestamp_start_ns,
                1,
                buffer,
            ),
            FrameLineage::try_new(
                SessionId(3),
                SourceId(2),
                StemId(4),
                ClockDomainId(5),
                sequence_number,
                timestamp_start_ns,
                20_000_000,
                source_generation,
                discontinuity_epoch,
                1,
            )
            .expect("test lineage is valid"),
        )
        .expect("frame identity and lineage match")
    }

    #[test]
    fn given_delivered_pcm_when_observed_then_counts_energy_and_continuity_are_exact() {
        let signal = SourceSignalObservationHandle::default();
        assert_eq!(signal.observations(10).samples_observed_total, 0);

        signal.observe_frame(20, &frame(&[0.0, -0.0, 0.0, 0.0], 1, 1, 0));
        signal.observe_frame(30, &frame(&[0.0, 0.0, 0.0, 0.0], 2, 1, 0));
        let two_zero_frames = signal.observations(31);
        assert_eq!(two_zero_frames.samples_observed_total, 8);
        assert_eq!(two_zero_frames.exact_zero_samples_observed_total, 8);
        assert_eq!(two_zero_frames.nonzero_samples_observed_total, 0);
        assert_eq!(two_zero_frames.nonfinite_samples_observed_total, 0);
        assert_eq!(
            two_zero_frames.consecutive_exact_zero_duration_ns,
            40_000_000
        );
        assert_eq!(two_zero_frames.window_exact_zero_ratio(), Some(1.0));
        assert_eq!(two_zero_frames.window_peak_dbfs(), None);
        assert_eq!(two_zero_frames.window_rms_dbfs(), None);

        signal.observe_frame(40, &frame(&[0.0, 0.5, -0.25, 0.0], 3, 1, 0));
        let nonzero = signal.observations(41);
        assert_eq!(nonzero.samples_observed_total, 12);
        assert_eq!(nonzero.exact_zero_samples_observed_total, 10);
        assert_eq!(nonzero.nonzero_samples_observed_total, 2);
        assert_eq!(nonzero.consecutive_exact_zero_duration_ns, 0);
        assert_eq!(nonzero.window_exact_zero_ratio(), Some(0.5));
        assert_eq!(nonzero.window_peak_linear(), Some(0.5));
        assert!((nonzero.window_rms_linear().expect("finite RMS") - 0.279_508_497).abs() < 1e-9);

        signal.observe_frame(50, &frame(&[0.0, 0.0, 0.0, 0.0], 4, 2, 1));
        let changed_source = signal.observations(51);
        assert_eq!(changed_source.window_source_generation, 2);
        assert_eq!(changed_source.window_discontinuity_epoch, 1);
        assert_eq!(
            changed_source.consecutive_exact_zero_duration_ns,
            20_000_000
        );
    }

    #[test]
    fn given_nonfinite_samples_when_observed_then_they_are_not_hidden_in_energy() {
        let signal = SourceSignalObservationHandle::default();
        signal.observe_frame(20, &frame(&[f32::NAN, f32::INFINITY, 0.25, 0.0], 1, 1, 0));
        let observations = signal.observations(21);

        assert_eq!(observations.window_samples_total, 4);
        assert_eq!(observations.window_exact_zero_samples_total, 1);
        assert_eq!(observations.window_nonzero_samples_total, 1);
        assert_eq!(observations.window_nonfinite_samples_total, 2);
        assert_eq!(observations.window_peak_linear(), Some(0.25));
        assert!(
            (observations.window_rms_linear().expect("finite RMS") - 0.176_776_695).abs() < 1e-9
        );
        assert_eq!(observations.consecutive_exact_zero_duration_ns, 0);
    }
}

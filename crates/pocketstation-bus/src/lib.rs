use pocketstation_frame::AudioFrame;
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct FrameProducer {
    inner: Producer<AudioFrame>,
    dropped_newest: AtomicU64,
}

pub struct FrameConsumer {
    inner: Consumer<AudioFrame>,
}

pub fn frame_bus(capacity: usize) -> (FrameProducer, FrameConsumer) {
    let (p, c) = RingBuffer::<AudioFrame>::new(capacity);
    (
        FrameProducer {
            inner: p,
            dropped_newest: AtomicU64::new(0),
        },
        FrameConsumer { inner: c },
    )
}

impl FrameProducer {
    pub fn push_drop_newest(&mut self, frame: AudioFrame) -> Result<(), AudioFrame> {
        match self.inner.push(frame) {
            Ok(()) => Ok(()),
            Err(rtrb::PushError::Full(frame)) => {
                self.dropped_newest.fetch_add(1, Ordering::Relaxed);
                Err(frame)
            }
        }
    }
    pub fn dropped_newest(&self) -> u64 {
        self.dropped_newest.load(Ordering::Relaxed)
    }
}

impl FrameConsumer {
    pub fn pop(&mut self) -> Option<AudioFrame> {
        self.inner.pop().ok()
    }
}

/// Phase 0 placeholder for ADR-006 PI-controlled clock synchronisation.
///
/// The full proportional-integral controller (dual-stage, anti-windup) is
/// deferred to Phase 1. This stub exposes the stable public API surface so
/// callers can be written against it now.
#[derive(Debug, Clone, Copy)]
pub struct ClockSync {
    target_sample_rate: u32,
    drift_ppm_estimate: f32,
    correction_ratio: f32,
}

impl ClockSync {
    pub fn new(target_sample_rate: u32) -> Self {
        Self {
            target_sample_rate,
            drift_ppm_estimate: 0.0,
            correction_ratio: 1.0,
        }
    }

    /// Feed a new drift measurement (in PPM) and update the correction ratio.
    /// ADR-006 owns the full PI tuning; this is an exponential smoother placeholder.
    pub fn update_pi(&mut self, measured_drift_ppm: f32) {
        self.drift_ppm_estimate = 0.95 * self.drift_ppm_estimate + 0.05 * measured_drift_ppm;
        self.correction_ratio = 1.0 - (self.drift_ppm_estimate / 1_000_000.0);
    }

    /// Multiplicative ratio to apply to the SRC step. 1.0 = no correction.
    pub fn correction_ratio(&self) -> f32 {
        self.correction_ratio
    }

    pub fn drift_ppm_estimate(&self) -> f32 {
        self.drift_ppm_estimate
    }

    pub fn target_sample_rate(&self) -> u32 {
        self.target_sample_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

    fn make_frame(pool: &std::sync::Arc<AudioBufferPool>, seq: u64) -> AudioFrame {
        let h = pool.acquire().unwrap();
        AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, h)
    }

    #[test]
    fn push_pop_single_frame_succeeds() {
        // Given
        let pool = AudioBufferPool::new(2, 8);
        let handle = pool.acquire().unwrap();
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, handle);
        let (mut p, mut c) = frame_bus(1);

        // When
        p.push_drop_newest(frame).unwrap();

        // Then
        assert!(c.pop().is_some());
    }

    #[test]
    fn push_into_full_ring_drops_newest_and_increments_counter() {
        // Given
        let pool = AudioBufferPool::new(8, 4);
        let (mut p, _c) = frame_bus(3);
        for seq in 0..3 {
            assert!(p.push_drop_newest(make_frame(&pool, seq)).is_ok());
        }

        // When
        let fourth = make_frame(&pool, 3);
        let result = p.push_drop_newest(fourth);

        // Then
        assert!(result.is_err());
        assert_eq!(p.dropped_newest(), 1);
    }

    #[test]
    fn frames_pushed_in_order_are_popped_in_fifo_order() {
        // Given
        let pool = AudioBufferPool::new(8, 4);
        let (mut p, mut c) = frame_bus(4);
        for seq in 0..4 {
            p.push_drop_newest(make_frame(&pool, seq)).unwrap();
        }

        // When / Then (interleaved assertion loop)
        for expected_seq in 0..4u64 {
            let frame = c.pop().unwrap();
            assert_eq!(frame.sequence_number, expected_seq);
        }
        assert!(c.pop().is_none());
    }

    #[test]
    fn pop_on_empty_ring_returns_none() {
        // Given
        let (_p, mut c) = frame_bus(4);

        // When / Then
        assert!(c.pop().is_none());
    }

    #[test]
    fn ring_capacity_is_bounded_and_excess_frames_are_dropped() {
        // Given
        let pool = AudioBufferPool::new(16, 4);
        let (mut p, _c) = frame_bus(8);

        // When
        let mut pushed = 0u64;
        let mut dropped = 0u64;
        for seq in 0..16 {
            match p.push_drop_newest(make_frame(&pool, seq)) {
                Ok(()) => pushed += 1,
                Err(_) => dropped += 1,
            }
        }

        // Then
        assert_eq!(pushed, 8);
        assert_eq!(dropped, 8);
        assert_eq!(p.dropped_newest(), 8);
    }

    #[test]
    fn clock_sync_zero_drift_correction_ratio_is_one() {
        // Given / When
        let cs = ClockSync::new(48_000);

        // Then
        assert_eq!(cs.correction_ratio(), 1.0);
        assert_eq!(cs.drift_ppm_estimate(), 0.0);
        assert_eq!(cs.target_sample_rate(), 48_000);
    }

    #[test]
    fn clock_sync_positive_drift_converges_and_reduces_correction_ratio() {
        // Given
        let mut cs = ClockSync::new(48_000);

        // When: drive the smoother to convergence
        for _ in 0..200 {
            cs.update_pi(100.0);
        }

        // Then
        let est = cs.drift_ppm_estimate();
        assert!(
            (est - 100.0).abs() < 1.0,
            "drift estimate {est} not near 100 ppm"
        );
        let ratio = cs.correction_ratio();
        assert!(
            ratio < 1.0,
            "positive drift should yield ratio < 1.0, got {ratio}"
        );
        assert!(
            (ratio - 0.9999).abs() < 0.0001,
            "ratio {ratio} not near 0.9999"
        );
    }

    #[test]
    fn clock_sync_negative_drift_increases_correction_ratio_above_one() {
        // Given
        let mut cs = ClockSync::new(48_000);

        // When
        for _ in 0..200 {
            cs.update_pi(-100.0);
        }

        // Then
        let ratio = cs.correction_ratio();
        assert!(
            ratio > 1.0,
            "negative drift should yield ratio > 1.0, got {ratio}"
        );
    }
}

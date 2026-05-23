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

/// PI-controlled clock synchronisation per ADR-006.
///
/// # Design
///
/// Implements a proportional-integral (PI) controller that converts a measured
/// clock offset (in nanoseconds) into a correction value (also nanoseconds)
/// that the SRC layer applies to adjust the effective sample rate.
///
/// Output is clamped to ±10 ms (±10 000 000 ns) to prevent windup on startup
/// or after a large discontinuity.
///
/// # Gain defaults
///
/// `kp = 0.1`, `ki = 0.001` — conservative values suitable for typical
/// network jitter in a LAN/Wi-Fi voice call.  These will be tuned in Phase 5
/// once real-world measurements are available (ADR-006).
///
/// # Phase 3, ADR-006: PI controller implemented; gains tuned in Phase 5.
#[derive(Debug, Clone, Copy)]
pub struct ClockSync {
    kp: f64,
    ki: f64,
    integral: f64,
    last_offset_ns: i64,
}

/// Maximum correction magnitude: ±10 ms expressed in nanoseconds.
const CLOCK_SYNC_CLAMP_NS: i64 = 10_000_000;

impl ClockSync {
    /// Create a new PI controller with the given proportional and integral gains.
    ///
    /// Use [`ClockSync::default()`] for the ADR-006 recommended starting gains.
    pub fn new(kp: f64, ki: f64) -> Self {
        Self {
            kp,
            ki,
            integral: 0.0,
            last_offset_ns: 0,
        }
    }

    /// Advance the controller by one tick.
    ///
    /// `measured_offset_ns` is the signed difference between the local clock
    /// and the remote reference (positive = local is ahead).
    ///
    /// Returns the correction to apply, clamped to ±10 ms.
    pub fn tick(&mut self, measured_offset_ns: i64) -> i64 {
        let error = measured_offset_ns as f64;

        // Accumulate integral term.
        self.integral += error;

        let correction = self.kp * error + self.ki * self.integral;
        self.last_offset_ns = measured_offset_ns;

        // Clamp to ±10 ms.
        correction
            .round()
            .clamp(-CLOCK_SYNC_CLAMP_NS as f64, CLOCK_SYNC_CLAMP_NS as f64) as i64
    }

    /// Most recent measured offset supplied to [`tick`].
    pub fn last_offset_ns(&self) -> i64 {
        self.last_offset_ns
    }

    /// Current value of the integral accumulator (useful for diagnostics).
    pub fn integral(&self) -> f64 {
        self.integral
    }
}

impl Default for ClockSync {
    /// Returns a controller with the ADR-006 recommended starting gains:
    /// `kp = 0.1`, `ki = 0.001`.
    fn default() -> Self {
        Self::new(0.1, 0.001)
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
    fn clock_sync_zero_offset_produces_zero_correction() {
        // Given
        let mut cs = ClockSync::default();

        // When
        let correction = cs.tick(0);

        // Then
        assert_eq!(correction, 0);
        assert_eq!(cs.last_offset_ns(), 0);
    }

    #[test]
    fn clock_sync_positive_offset_produces_positive_correction() {
        // Given: 1 ms offset
        let mut cs = ClockSync::default();

        // When
        let correction = cs.tick(1_000_000);

        // Then: correction is positive (steer local clock back)
        assert!(
            correction > 0,
            "positive offset must produce positive correction, got {correction}"
        );
    }

    #[test]
    fn clock_sync_negative_offset_produces_negative_correction() {
        // Given: −1 ms offset
        let mut cs = ClockSync::default();

        // When
        let correction = cs.tick(-1_000_000);

        // Then
        assert!(
            correction < 0,
            "negative offset must produce negative correction, got {correction}"
        );
    }

    #[test]
    fn clock_sync_correction_is_clamped_to_ten_milliseconds() {
        // Given: offset far larger than the clamp window (100 ms)
        let mut cs = ClockSync::default();

        // When
        let correction = cs.tick(100_000_000);

        // Then: correction is clamped to ±10 ms
        assert!(
            correction.abs() <= 10_000_000,
            "correction {correction} exceeds ±10 ms clamp"
        );
    }

    #[test]
    fn clock_sync_integral_accumulates_across_ticks() {
        // Given
        let mut cs = ClockSync::new(0.0, 1.0); // pure I controller for isolation

        // When: two ticks of equal offset
        cs.tick(1_000);
        cs.tick(1_000);

        // Then: integral has accumulated both errors (clamped at output but
        // the raw integral field is unclamped)
        assert!(
            cs.integral() > 1_000.0,
            "integral {} should exceed single-tick error",
            cs.integral()
        );
    }
}

use pocketstation_frame::AudioFrame;
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackpressurePolicy {
    DropNewest,
    DropOldest,
}

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

#[derive(Debug, Clone, Copy)]
pub struct ClockSync {
    pub target_sample_rate: u32,
    pub drift_ppm_estimate: f32,
    pub correction_ratio: f32,
}

impl ClockSync {
    pub fn new(target_sample_rate: u32) -> Self {
        Self {
            target_sample_rate,
            drift_ppm_estimate: 0.0,
            correction_ratio: 1.0,
        }
    }
    pub fn update_pi(&mut self, measured_drift_ppm: f32) {
        // Phase 0 placeholder: smooth drift estimate. ADR-006 owns full PI tuning.
        self.drift_ppm_estimate = 0.95 * self.drift_ppm_estimate + 0.05 * measured_drift_ppm;
        self.correction_ratio = 1.0 - (self.drift_ppm_estimate / 1_000_000.0);
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
    fn push_pop_frame() {
        let pool = AudioBufferPool::new(2, 8);
        let handle = pool.acquire().unwrap();
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, handle);
        let (mut p, mut c) = frame_bus(1);
        p.push_drop_newest(frame).unwrap();
        assert!(c.pop().is_some());
    }

    #[test]
    fn given_ring_capacity_3_when_push_4_then_4th_dropped_newest() {
        let pool = AudioBufferPool::new(8, 4);
        let (mut p, _c) = frame_bus(3);
        for seq in 0..3 {
            assert!(p.push_drop_newest(make_frame(&pool, seq)).is_ok());
        }
        let fourth = make_frame(&pool, 3);
        assert!(p.push_drop_newest(fourth).is_err());
        assert_eq!(p.dropped_newest(), 1);
    }

    #[test]
    fn given_frames_pushed_in_order_when_popped_then_fifo_order() {
        let pool = AudioBufferPool::new(8, 4);
        let (mut p, mut c) = frame_bus(4);
        for seq in 0..4 {
            p.push_drop_newest(make_frame(&pool, seq)).unwrap();
        }
        for expected_seq in 0..4u64 {
            let frame = c.pop().unwrap();
            assert_eq!(frame.sequence_number, expected_seq);
        }
        assert!(c.pop().is_none());
    }

    #[test]
    fn given_empty_ring_when_pop_then_returns_none() {
        let (_p, mut c) = frame_bus(4);
        assert!(c.pop().is_none());
    }

    #[test]
    fn given_full_ring_when_no_drop_policy_applied_then_capacity_bounded() {
        let pool = AudioBufferPool::new(16, 4);
        let (mut p, _c) = frame_bus(8);
        let mut pushed = 0u64;
        let mut dropped = 0u64;
        for seq in 0..16 {
            match p.push_drop_newest(make_frame(&pool, seq)) {
                Ok(()) => pushed += 1,
                Err(_) => dropped += 1,
            }
        }
        assert_eq!(pushed, 8);
        assert_eq!(dropped, 8);
        assert_eq!(p.dropped_newest(), 8);
    }
}

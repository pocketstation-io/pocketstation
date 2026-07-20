//! Bounded SPSC channel between execution partitions.
//!
//! The realtime endpoint never blocks. Telemetry uses shared atomics and adds
//! no allocation or locking to per-frame send/receive operations.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use pks_frame::AudioFrame;
use rtrb::{Consumer, Producer, RingBuffer};

pub struct EdgeChannel;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EdgeTelemetrySnapshot {
    pub enqueued_count: u64,
    pub dequeued_count: u64,
    pub dropped_count: u64,
    pub current_depth_frames: u64,
    pub peak_depth_frames: u64,
    pub max_dequeue_age_ns: u64,
}

#[derive(Default)]
struct EdgeTelemetry {
    enqueued_count: AtomicU64,
    dequeued_count: AtomicU64,
    dropped_count: AtomicU64,
    peak_depth_frames: AtomicU64,
    max_dequeue_age_ns: AtomicU64,
}

impl EdgeTelemetry {
    fn observe_enqueue(&self) {
        let enqueued_count = self.enqueued_count.fetch_add(1, Ordering::Relaxed) + 1;
        let dequeued_count = self.dequeued_count.load(Ordering::Relaxed);
        let depth_frames = enqueued_count.saturating_sub(dequeued_count);
        self.peak_depth_frames
            .fetch_max(depth_frames, Ordering::Relaxed);
    }

    fn observe_dequeue(&self, frame_timestamp_ns: u64, now_ns: u64) {
        self.dequeued_count.fetch_add(1, Ordering::Relaxed);
        if frame_timestamp_ns != 0 {
            self.max_dequeue_age_ns
                .fetch_max(now_ns.saturating_sub(frame_timestamp_ns), Ordering::Relaxed);
        }
    }

    fn snapshot(&self) -> EdgeTelemetrySnapshot {
        let enqueued_count = self.enqueued_count.load(Ordering::Relaxed);
        let dequeued_count = self.dequeued_count.load(Ordering::Relaxed);
        EdgeTelemetrySnapshot {
            enqueued_count,
            dequeued_count,
            dropped_count: self.dropped_count.load(Ordering::Relaxed),
            current_depth_frames: enqueued_count.saturating_sub(dequeued_count),
            peak_depth_frames: self.peak_depth_frames.load(Ordering::Relaxed),
            max_dequeue_age_ns: self.max_dequeue_age_ns.load(Ordering::Relaxed),
        }
    }
}

impl EdgeChannel {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(capacity_frames: usize) -> (EdgeSender, EdgeReceiver) {
        let (producer, consumer) = RingBuffer::<AudioFrame>::new(capacity_frames);
        let telemetry = Arc::new(EdgeTelemetry::default());
        (
            EdgeSender {
                producer,
                telemetry: Arc::clone(&telemetry),
            },
            EdgeReceiver {
                consumer,
                telemetry,
            },
        )
    }
}

pub struct EdgeSender {
    producer: Producer<AudioFrame>,
    telemetry: Arc<EdgeTelemetry>,
}

impl EdgeSender {
    pub fn send(&mut self, frame: AudioFrame) -> Result<(), AudioFrame> {
        match self.producer.push(frame) {
            Ok(()) => {
                self.telemetry.observe_enqueue();
                Ok(())
            }
            Err(rtrb::PushError::Full(frame)) => {
                self.telemetry.dropped_count.fetch_add(1, Ordering::Relaxed);
                Err(frame)
            }
        }
    }

    pub fn dropped_count(&self) -> u64 {
        self.telemetry.dropped_count.load(Ordering::Relaxed)
    }

    pub fn telemetry(&self) -> EdgeTelemetrySnapshot {
        self.telemetry.snapshot()
    }
}

pub struct EdgeReceiver {
    consumer: Consumer<AudioFrame>,
    telemetry: Arc<EdgeTelemetry>,
}

impl EdgeReceiver {
    pub fn recv(&mut self) -> Option<AudioFrame> {
        self.recv_at(0)
    }

    pub fn recv_at(&mut self, now_ns: u64) -> Option<AudioFrame> {
        let frame = self.consumer.pop().ok()?;
        self.telemetry.observe_dequeue(frame.timestamp_ns, now_ns);
        Some(frame)
    }

    pub fn telemetry(&self) -> EdgeTelemetrySnapshot {
        self.telemetry.snapshot()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_frame::{AudioBufferPool, SourceId, StreamId};

    fn frame_with_samples(samples: &[f32]) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples.len());
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(samples);
        AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle)
    }

    #[test]
    fn given_channel_with_room_when_send_then_recv_returns_same_frame() {
        let (mut sender, mut receiver) = EdgeChannel::new(2);
        sender.send(frame_with_samples(&[0.25, -0.5])).unwrap();

        let received = receiver.recv().unwrap();
        assert_eq!(received.buffer.as_slice(), &[0.25, -0.5]);
        assert_eq!(sender.dropped_count(), 0);
    }

    #[test]
    fn given_full_ring_when_send_then_returns_err_and_increments_dropped_count() {
        let (mut sender, _receiver) = EdgeChannel::new(1);
        sender.send(frame_with_samples(&[1.0])).unwrap();

        let rejected = sender.send(frame_with_samples(&[2.0]));
        assert!(rejected.is_err());
        assert_eq!(rejected.unwrap_err().buffer.as_slice(), &[2.0]);
        assert_eq!(sender.dropped_count(), 1);
    }

    #[test]
    fn given_empty_channel_when_recv_then_returns_none() {
        let (_sender, mut receiver) = EdgeChannel::new(2);
        assert!(receiver.recv().is_none());
    }

    #[test]
    fn given_queued_frames_when_observed_then_depth_and_peak_are_shared() {
        let (mut sender, mut receiver) = EdgeChannel::new(2);
        sender.send(frame_with_samples(&[1.0])).unwrap();
        sender.send(frame_with_samples(&[2.0])).unwrap();

        assert_eq!(sender.telemetry().current_depth_frames, 2);
        assert_eq!(receiver.telemetry().peak_depth_frames, 2);

        receiver.recv().unwrap();
        assert_eq!(sender.telemetry().current_depth_frames, 1);
        assert_eq!(receiver.telemetry().dequeued_count, 1);
    }

    #[test]
    fn given_timestamped_frame_when_received_then_age_is_recorded() {
        let (mut sender, mut receiver) = EdgeChannel::new(1);
        let mut frame = frame_with_samples(&[1.0]);
        frame.timestamp_ns = 10;
        sender.send(frame).unwrap();

        receiver.recv_at(35).unwrap();

        assert_eq!(receiver.telemetry().max_dequeue_age_ns, 25);
    }

    #[test]
    fn given_dequeue_observed_before_enqueue_counter_when_snapshotted_then_depth_does_not_underflow(
    ) {
        let telemetry = EdgeTelemetry::default();
        telemetry.dequeued_count.store(1, Ordering::Relaxed);

        assert_eq!(telemetry.snapshot().current_depth_frames, 0);

        telemetry.observe_enqueue();
        assert_eq!(telemetry.snapshot().current_depth_frames, 0);
    }
}

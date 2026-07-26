//! Bounded SPSC channel for immutable frames after a fan-out freeze boundary.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use pks_frame::SharedAudioFrame;
use rtrb::{Consumer, Producer, RingBuffer};

pub struct SharedEdgeChannel;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SharedEdgeTelemetrySnapshot {
    pub enqueued_count: u64,
    pub dequeued_count: u64,
    pub dropped_count: u64,
    pub shutdown_discarded_count: u64,
    pub current_depth_frames: u64,
    pub peak_depth_frames: u64,
    pub max_dequeue_age_ns: u64,
}

#[derive(Default)]
struct SharedEdgeTelemetry {
    enqueued_count: AtomicU64,
    dequeued_count: AtomicU64,
    dropped_count: AtomicU64,
    shutdown_discarded_count: AtomicU64,
    peak_depth_frames: AtomicU64,
    max_dequeue_age_ns: AtomicU64,
}

impl SharedEdgeTelemetry {
    fn observe_enqueue(&self) {
        let enqueued_count = self.enqueued_count.fetch_add(1, Ordering::Relaxed) + 1;
        let completed_count = self
            .dequeued_count
            .load(Ordering::Relaxed)
            .saturating_add(self.shutdown_discarded_count.load(Ordering::Relaxed));
        self.peak_depth_frames.fetch_max(
            enqueued_count.saturating_sub(completed_count),
            Ordering::Relaxed,
        );
    }

    fn observe_dequeue(&self, frame_timestamp_ns: u64, now_ns: u64) {
        self.dequeued_count.fetch_add(1, Ordering::Relaxed);
        if frame_timestamp_ns != 0 {
            self.max_dequeue_age_ns
                .fetch_max(now_ns.saturating_sub(frame_timestamp_ns), Ordering::Relaxed);
        }
    }

    fn snapshot(&self) -> SharedEdgeTelemetrySnapshot {
        let enqueued_count = self.enqueued_count.load(Ordering::Relaxed);
        let dequeued_count = self.dequeued_count.load(Ordering::Relaxed);
        let shutdown_discarded_count = self.shutdown_discarded_count.load(Ordering::Relaxed);
        SharedEdgeTelemetrySnapshot {
            enqueued_count,
            dequeued_count,
            dropped_count: self.dropped_count.load(Ordering::Relaxed),
            shutdown_discarded_count,
            current_depth_frames: enqueued_count
                .saturating_sub(dequeued_count.saturating_add(shutdown_discarded_count)),
            peak_depth_frames: self.peak_depth_frames.load(Ordering::Relaxed),
            max_dequeue_age_ns: self.max_dequeue_age_ns.load(Ordering::Relaxed),
        }
    }
}

impl SharedEdgeChannel {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(capacity_frames: usize) -> (SharedEdgeSender, SharedEdgeReceiver) {
        let (producer, consumer) = RingBuffer::<SharedAudioFrame>::new(capacity_frames);
        let telemetry = Arc::new(SharedEdgeTelemetry::default());
        (
            SharedEdgeSender {
                producer,
                telemetry: Arc::clone(&telemetry),
            },
            SharedEdgeReceiver {
                consumer,
                telemetry,
            },
        )
    }
}

pub struct SharedEdgeSender {
    producer: Producer<SharedAudioFrame>,
    telemetry: Arc<SharedEdgeTelemetry>,
}

impl SharedEdgeSender {
    pub fn send(&mut self, frame: SharedAudioFrame) -> Result<(), SharedAudioFrame> {
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

    pub fn telemetry(&self) -> SharedEdgeTelemetrySnapshot {
        self.telemetry.snapshot()
    }
}

pub struct SharedEdgeReceiver {
    consumer: Consumer<SharedAudioFrame>,
    telemetry: Arc<SharedEdgeTelemetry>,
}

impl SharedEdgeReceiver {
    pub fn recv(&mut self) -> Option<SharedAudioFrame> {
        self.recv_at(0)
    }

    pub fn recv_at(&mut self, now_ns: u64) -> Option<SharedAudioFrame> {
        let frame = self.consumer.pop().ok()?;
        self.telemetry.observe_dequeue(frame.timestamp_ns, now_ns);
        Some(frame)
    }

    pub fn telemetry(&self) -> SharedEdgeTelemetrySnapshot {
        self.telemetry.snapshot()
    }
}

impl Drop for SharedEdgeReceiver {
    fn drop(&mut self) {
        while self.consumer.pop().is_ok() {
            self.telemetry
                .shutdown_discarded_count
                .fetch_add(1, Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

    fn shared_frame(pool: &Arc<AudioBufferPool>, value: f32) -> SharedAudioFrame {
        let mut buffer = pool.acquire().unwrap();
        buffer.copy_from_slice(&[value]);
        AudioFrame::new(StreamId(1), SourceId(2), 3, 4, 1, buffer)
            .freeze()
            .unwrap()
    }

    #[test]
    fn given_shared_channel_with_room_when_frame_sent_then_receiver_gets_same_slot() {
        // Given
        let pool = AudioBufferPool::new(1, 1);
        let frame = shared_frame(&pool, 0.25);
        let slot_index = frame.buffer.index();
        let (mut sender, mut receiver) = SharedEdgeChannel::new(2);

        // When
        sender.send(frame).unwrap();
        let received = receiver.recv().unwrap();

        // Then
        assert_eq!(received.buffer.index(), slot_index);
        assert_eq!(received.buffer.as_slice(), &[0.25]);
        assert_eq!(receiver.telemetry().current_depth_frames, 0);
    }

    #[test]
    fn given_full_shared_channel_when_new_branch_rejected_then_drop_releases_reference() {
        // Given
        let pool = AudioBufferPool::new(1, 1);
        let frame = shared_frame(&pool, 0.5);
        let queued = frame.try_clone().unwrap();
        let rejected = frame.try_clone().unwrap();
        let (mut sender, _receiver) = SharedEdgeChannel::new(1);
        sender.send(queued).unwrap();

        // When
        let rejected = sender.send(rejected).unwrap_err();
        drop(rejected);

        // Then
        assert_eq!(frame.buffer.shared_ref_count(), 2);
        assert_eq!(sender.telemetry().dropped_count, 1);
    }

    #[test]
    fn given_worker_receiver_with_queued_frames_when_dropped_then_queue_references_are_released() {
        // Given
        let pool = AudioBufferPool::new(1, 1);
        let frame = shared_frame(&pool, 0.75);
        let (mut sender, receiver) = SharedEdgeChannel::new(2);
        sender.send(frame.try_clone().unwrap()).unwrap();
        sender.send(frame.try_clone().unwrap()).unwrap();
        assert_eq!(frame.buffer.shared_ref_count(), 3);

        // When
        drop(receiver);

        // Then
        assert_eq!(frame.buffer.shared_ref_count(), 1);
        assert_eq!(sender.telemetry().shutdown_discarded_count, 2);
        assert_eq!(sender.telemetry().current_depth_frames, 0);
    }
}

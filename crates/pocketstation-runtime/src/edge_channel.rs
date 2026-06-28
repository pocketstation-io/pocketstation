//! Bounded SPSC channel that carries `AudioFrame`s between two execution
//! partitions. The realtime side never blocks: a full ring makes `send` return
//! the frame back so the caller applies its own drop policy (drop-newest here),
//! satisfying the hot-path purity rule (LAW 15: no alloc/lock/block on send).

use std::sync::atomic::{AtomicU64, Ordering};

use pocketstation_frame::AudioFrame;
use rtrb::{Consumer, Producer, RingBuffer};

pub struct EdgeChannel;

impl EdgeChannel {
    // A channel is a producer/consumer pair, not a single owned value, so `new`
    // returns the two endpoints rather than `Self`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(capacity_frames: usize) -> (EdgeSender, EdgeReceiver) {
        let (producer, consumer) = RingBuffer::<AudioFrame>::new(capacity_frames);
        (
            EdgeSender {
                producer,
                dropped_count: AtomicU64::new(0),
            },
            EdgeReceiver { consumer },
        )
    }
}

pub struct EdgeSender {
    producer: Producer<AudioFrame>,
    dropped_count: AtomicU64, // drop-newest events when the ring was full
}

impl EdgeSender {
    pub fn send(&mut self, frame: AudioFrame) -> Result<(), AudioFrame> {
        match self.producer.push(frame) {
            Ok(()) => Ok(()),
            Err(rtrb::PushError::Full(frame)) => {
                self.dropped_count.fetch_add(1, Ordering::Relaxed);
                Err(frame)
            }
        }
    }

    pub fn dropped_count(&self) -> u64 {
        self.dropped_count.load(Ordering::Relaxed)
    }
}

pub struct EdgeReceiver {
    consumer: Consumer<AudioFrame>,
}

impl EdgeReceiver {
    pub fn recv(&mut self) -> Option<AudioFrame> {
        self.consumer.pop().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, SourceId, StreamId};

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
}

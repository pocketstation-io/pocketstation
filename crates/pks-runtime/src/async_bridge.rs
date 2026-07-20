//! Bounded bridge from realtime partitions to async/blocking/external work.
//!
//! The realtime side uses `try_send` semantics only: full ring means the caller
//! gets the envelope back and applies its edge drop policy. No blocking, locks,
//! logging, or heap allocation occurs in `send_audio`.

use std::sync::atomic::{AtomicU64, Ordering};

use pks_frame::AudioFrame;
use pks_graph::async_node::{AsyncEnvelope, AsyncSignal};
use rtrb::{Consumer, Producer, RingBuffer};

pub struct AsyncBridge;

impl AsyncBridge {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(capacity_frames: usize) -> (AsyncBridgeSender, AsyncBridgeReceiver) {
        let (producer, consumer) = RingBuffer::<AsyncEnvelope>::new(capacity_frames);
        (
            AsyncBridgeSender {
                producer,
                dropped_count: AtomicU64::new(0),
            },
            AsyncBridgeReceiver { consumer },
        )
    }
}

pub struct AsyncBridgeSender {
    producer: Producer<AsyncEnvelope>,
    dropped_count: AtomicU64,
}

impl AsyncBridgeSender {
    pub fn send(&mut self, envelope: AsyncEnvelope) -> Result<(), AsyncEnvelope> {
        match self.producer.push(envelope) {
            Ok(()) => Ok(()),
            Err(rtrb::PushError::Full(envelope)) => {
                self.dropped_count.fetch_add(1, Ordering::Relaxed);
                Err(envelope)
            }
        }
    }

    pub fn send_audio(
        &mut self,
        frame: AudioFrame,
        sequence_number: u64,
        timestamp_ns: u64,
    ) -> Result<(), AudioFrame> {
        let envelope = AsyncEnvelope::new(AsyncSignal::Audio(frame), sequence_number, timestamp_ns);
        match self.producer.push(envelope) {
            Ok(()) => Ok(()),
            Err(rtrb::PushError::Full(envelope)) => {
                self.dropped_count.fetch_add(1, Ordering::Relaxed);
                let AsyncSignal::Audio(frame) = envelope.signal else {
                    return Ok(());
                };
                Err(frame)
            }
        }
    }

    pub fn dropped_count(&self) -> u64 {
        self.dropped_count.load(Ordering::Relaxed)
    }
}

pub struct AsyncBridgeReceiver {
    consumer: Consumer<AsyncEnvelope>,
}

impl AsyncBridgeReceiver {
    pub fn recv(&mut self) -> Option<AsyncEnvelope> {
        self.consumer.pop().ok()
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
        AudioFrame::new(StreamId(0), SourceId(0), 3, 5, 1, handle)
    }

    #[test]
    fn given_bridge_with_room_when_send_audio_then_receiver_gets_audio_envelope() {
        let (mut sender, mut receiver) = AsyncBridge::new(2);
        sender
            .send_audio(frame_with_samples(&[0.25, -0.5]), 7, 11)
            .unwrap();

        let envelope = receiver.recv().unwrap();
        assert_eq!(envelope.sequence_number, 7);
        assert_eq!(envelope.timestamp_ns, 11);
        match envelope.signal {
            AsyncSignal::Audio(frame) => assert_eq!(frame.buffer.as_slice(), &[0.25, -0.5]),
            _ => panic!("expected audio envelope"),
        }
        assert_eq!(sender.dropped_count(), 0);
    }

    #[test]
    fn given_full_bridge_when_send_audio_then_frame_returned_and_drop_count_incremented() {
        let (mut sender, _receiver) = AsyncBridge::new(1);
        sender.send_audio(frame_with_samples(&[1.0]), 0, 0).unwrap();

        let rejected = sender.send_audio(frame_with_samples(&[2.0]), 1, 1);
        assert!(rejected.is_err());
        assert_eq!(rejected.unwrap_err().buffer.as_slice(), &[2.0]);
        assert_eq!(sender.dropped_count(), 1);
    }

    #[test]
    fn given_empty_bridge_when_recv_then_none() {
        let (_sender, mut receiver) = AsyncBridge::new(2);
        assert!(receiver.recv().is_none());
    }
}

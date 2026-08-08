//! Bounded bridge from realtime partitions to async/blocking/external work.
//!
//! The realtime side uses `try_send` semantics only: full ring means the caller
//! gets the envelope back and applies its edge drop policy. No blocking, locks,
//! logging, or heap allocation occurs in `send_audio`.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::frame::AudioFrame;
use crate::graph::async_node::{SignalEnvelope, SignalPayload};
use rtrb::{Consumer, Producer, RingBuffer};

pub struct AsyncBridge;

impl AsyncBridge {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(capacity_frames: usize) -> (AsyncBridgeSender, AsyncBridgeReceiver) {
        let (producer, consumer) = RingBuffer::<SignalEnvelope>::new(capacity_frames);
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
    producer: Producer<SignalEnvelope>,
    dropped_count: AtomicU64,
}

#[derive(Debug)]
pub struct AsyncBridgeSendError {
    rejected: Box<SignalEnvelope>,
}

impl AsyncBridgeSendError {
    pub fn into_rejected(self) -> SignalEnvelope {
        *self.rejected
    }
}

impl AsyncBridgeSender {
    pub fn send(&mut self, envelope: SignalEnvelope) -> Result<(), AsyncBridgeSendError> {
        match self.producer.push(envelope) {
            Ok(()) => Ok(()),
            Err(rtrb::PushError::Full(envelope)) => {
                self.dropped_count.fetch_add(1, Ordering::Relaxed);
                Err(AsyncBridgeSendError {
                    rejected: Box::new(envelope),
                })
            }
        }
    }

    pub fn send_audio(
        &mut self,
        mut frame: AudioFrame,
        sequence_number: u64,
        timestamp_ns: u64,
    ) -> Result<(), AudioFrame> {
        frame.sequence_number = sequence_number;
        frame.timestamp_ns = timestamp_ns;
        let envelope = SignalEnvelope::from_audio(frame, None);
        match self.producer.push(envelope) {
            Ok(()) => Ok(()),
            Err(rtrb::PushError::Full(envelope)) => {
                self.dropped_count.fetch_add(1, Ordering::Relaxed);
                let SignalPayload::Audio(frame) = envelope.payload else {
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
    consumer: Consumer<SignalEnvelope>,
}

impl AsyncBridgeReceiver {
    pub fn recv(&mut self) -> Option<SignalEnvelope> {
        self.consumer.pop().ok()
    }

    pub fn is_abandoned(&self) -> bool {
        self.consumer.is_abandoned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{AudioBufferPool, SourceId, StreamId};

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
        assert_eq!(envelope.sequence_number(), Some(7));
        assert_eq!(envelope.timestamp_ns(), 11);
        match envelope.payload {
            SignalPayload::Audio(frame) => assert_eq!(frame.buffer.as_slice(), &[0.25, -0.5]),
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

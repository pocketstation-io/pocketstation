use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use crate::frame::AudioFrame;
use rtrb::{Consumer, Producer, PushError, RingBuffer};

use crate::capture::CaptureError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Reports whether a captured frame was accepted, dropped, or rejected by delivery."]
pub enum CapturedFrameDelivery {
    #[doc = "Indicates the delivered state for `CapturedFrameDelivery`."]
    Delivered,
    #[doc = "Indicates the dropped newest state for `CapturedFrameDelivery`."]
    DroppedNewest,
    #[doc = "Indicates the discarded before start state for `CapturedFrameDelivery`."]
    DiscardedBeforeStart,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[doc = "Reports the captured frame stream stats collected at an observation boundary."]
pub struct CapturedFrameStreamStats {
    #[doc = "Contains the delivered frames owned or reported by `CapturedFrameStreamStats`."]
    pub delivered_frames: u64,
    #[doc = "Contains the dropped newest frames owned or reported by `CapturedFrameStreamStats`."]
    pub dropped_newest_frames: u64,
    #[doc = "Counts the total number of frames discarded before start observed by `CapturedFrameStreamStats`."]
    pub frames_discarded_before_start_total: u64,
}

#[derive(Debug, Default)]
struct CapturedFrameStreamCounters {
    delivered_frames: AtomicU64,
    dropped_newest_frames: AtomicU64,
    frames_discarded_before_start_total: AtomicU64,
}

#[derive(Clone, Debug)]
#[doc = "Holds the ownership or bounded access represented by captured frame observation handle."]
pub struct CapturedFrameObservationHandle {
    counters: Arc<CapturedFrameStreamCounters>,
}

impl CapturedFrameObservationHandle {
    #[doc = "Returns the observations exposed by `CapturedFrameObservationHandle`."]
    pub fn observations(&self) -> CapturedFrameStreamStats {
        self.counters.snapshot()
    }
}

impl CapturedFrameStreamCounters {
    fn snapshot(&self) -> CapturedFrameStreamStats {
        CapturedFrameStreamStats {
            delivered_frames: self.delivered_frames.load(Ordering::Relaxed),
            dropped_newest_frames: self.dropped_newest_frames.load(Ordering::Relaxed),
            frames_discarded_before_start_total: self
                .frames_discarded_before_start_total
                .load(Ordering::Relaxed),
        }
    }
}

/// Read-only one-way start barrier checked by capture delivery callbacks.
pub struct CaptureDeliveryStartGate {
    open: AtomicBool,
}

impl CaptureDeliveryStartGate {
    fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub(crate) fn opened() -> Arc<Self> {
        Arc::new(Self {
            open: AtomicBool::new(true),
        })
    }
}

/// Session-owned authority that opens one capture delivery start gate.
pub struct CaptureDeliveryStartGateController {
    gate: Arc<CaptureDeliveryStartGate>,
}

impl CaptureDeliveryStartGateController {
    #[doc = "Opens the resource represented by `CaptureDeliveryStartGateController`."]
    pub fn open(&self) -> bool {
        !self.gate.open.swap(true, Ordering::AcqRel)
    }
}

/// Creates a closed Session-owned controller and callback-visible start gate.
pub fn capture_delivery_start_gate() -> (
    CaptureDeliveryStartGateController,
    Arc<CaptureDeliveryStartGate>,
) {
    let gate = Arc::new(CaptureDeliveryStartGate {
        open: AtomicBool::new(false),
    });
    (
        CaptureDeliveryStartGateController {
            gate: Arc::clone(&gate),
        },
        gate,
    )
}

/// Single-producer endpoint passed into a platform capture callback.
///
/// `try_send` is wait-free, lock-free, and allocation-free. A full stream drops
/// the newest frame immediately, preserving the age of frames already queued.
pub struct CapturedFrameSender {
    producer: Producer<AudioFrame>,
    counters: Arc<CapturedFrameStreamCounters>,
    start_gate: Arc<CaptureDeliveryStartGate>,
}

impl CapturedFrameSender {
    #[doc = "Attempts to send a value through `CapturedFrameSender` without waiting for capacity."]
    pub fn try_send(&mut self, frame: AudioFrame) -> CapturedFrameDelivery {
        if !self.start_gate.is_open() {
            self.counters
                .frames_discarded_before_start_total
                .fetch_add(1, Ordering::Relaxed);
            return CapturedFrameDelivery::DiscardedBeforeStart;
        }
        match self.producer.push(frame) {
            Ok(()) => {
                self.counters
                    .delivered_frames
                    .fetch_add(1, Ordering::Relaxed);
                CapturedFrameDelivery::Delivered
            }
            Err(PushError::Full(_frame)) => {
                self.counters
                    .dropped_newest_frames
                    .fetch_add(1, Ordering::Relaxed);
                CapturedFrameDelivery::DroppedNewest
            }
        }
    }

    #[doc = "Converts `CapturedFrameSender` into callback."]
    pub fn into_callback(mut self) -> impl FnMut(AudioFrame) + Send + 'static {
        move |frame| {
            let _ = self.try_send(frame);
        }
    }

    #[doc = "Returns the current statistics for `CapturedFrameSender`."]
    pub fn stats(&self) -> CapturedFrameStreamStats {
        self.counters.snapshot()
    }

    #[doc = "Returns a handle for reading observations from `CapturedFrameSender`."]
    pub fn observation_handle(&self) -> CapturedFrameObservationHandle {
        CapturedFrameObservationHandle {
            counters: Arc::clone(&self.counters),
        }
    }
}

/// Non-blocking consumer for captured `AudioFrame`s.
///
/// The consumer owns each pool-backed frame until it is dropped or forwarded.
/// Callers choose their own polling or async wake strategy outside the capture
/// callback; this type never creates a runtime or blocks an audio thread.
pub struct CapturedFrameStream {
    consumer: Consumer<AudioFrame>,
    counters: Arc<CapturedFrameStreamCounters>,
}

impl CapturedFrameStream {
    #[doc = "Attempts to next through `CapturedFrameStream`."]
    pub fn try_next(&mut self) -> Option<AudioFrame> {
        self.consumer.pop().ok()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the capacity frames held by `CapturedFrameStream`."]
    pub fn capacity_frames(&self) -> usize {
        self.consumer.buffer().capacity()
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Reports whether closed is true for `CapturedFrameStream`."]
    pub fn is_closed(&self) -> bool {
        let abandoned = self.consumer.is_abandoned();
        if abandoned {
            std::sync::atomic::fence(Ordering::Acquire);
        }
        abandoned
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the current statistics for `CapturedFrameStream`."]
    pub fn stats(&self) -> CapturedFrameStreamStats {
        self.counters.snapshot()
    }

    #[doc = "Returns a handle for reading observations from `CapturedFrameStream`."]
    pub fn observation_handle(&self) -> CapturedFrameObservationHandle {
        CapturedFrameObservationHandle {
            counters: Arc::clone(&self.counters),
        }
    }
}

#[cfg(any(test, feature = "internal-testing"))]
#[doc = "Wraps the supplied capture receiver as a stream of captured frames."]
pub fn captured_frame_stream(
    capacity_frames: usize,
) -> Result<(CapturedFrameSender, CapturedFrameStream), CaptureError> {
    captured_frame_stream_with_start_gate(capacity_frames, CaptureDeliveryStartGate::opened())
}

#[doc(hidden)]
pub fn captured_frame_stream_with_start_gate(
    capacity_frames: usize,
    start_gate: Arc<CaptureDeliveryStartGate>,
) -> Result<(CapturedFrameSender, CapturedFrameStream), CaptureError> {
    if capacity_frames == 0 {
        return Err(CaptureError::InvalidStreamCapacity);
    }
    let (producer, consumer) = RingBuffer::new(capacity_frames);
    let counters = Arc::new(CapturedFrameStreamCounters::default());
    Ok((
        CapturedFrameSender {
            producer,
            counters: Arc::clone(&counters),
            start_gate,
        },
        CapturedFrameStream { consumer, counters },
    ))
}

#[cfg(test)]
mod tests {
    use crate::frame::{AudioBufferPool, SourceId, StreamId};

    use super::*;

    fn frame(pool: &Arc<AudioBufferPool>, sequence_number: u64) -> AudioFrame {
        let handle = pool.acquire().expect("test pool should have a free slot");
        AudioFrame::new(StreamId(1), SourceId(2), sequence_number, 3, 1, handle)
    }

    #[test]
    fn given_zero_capacity_when_stream_is_created_then_error_is_returned() {
        assert!(matches!(
            captured_frame_stream(0),
            Err(CaptureError::InvalidStreamCapacity)
        ));
    }

    #[test]
    fn given_available_capacity_when_frame_is_sent_then_stream_preserves_frame() {
        let pool = AudioBufferPool::new(2, 960);
        let (mut sender, mut stream) = captured_frame_stream(2).unwrap();
        let observations = stream.observation_handle();

        assert_eq!(
            sender.try_send(frame(&pool, 17)),
            CapturedFrameDelivery::Delivered
        );
        assert_eq!(observations.observations().delivered_frames, 1);
        assert_eq!(stream.try_next().unwrap().sequence_number, 17);
        assert_eq!(
            stream.stats(),
            CapturedFrameStreamStats {
                delivered_frames: 1,
                dropped_newest_frames: 0,
                frames_discarded_before_start_total: 0,
            }
        );
    }

    #[test]
    fn given_closed_start_gate_when_frame_is_sent_then_frame_is_discarded_and_counted() {
        let pool = AudioBufferPool::new(1, 960);
        let (controller, gate) = capture_delivery_start_gate();
        let (mut sender, mut stream) = captured_frame_stream_with_start_gate(1, gate).unwrap();

        assert_eq!(
            sender.try_send(frame(&pool, 1)),
            CapturedFrameDelivery::DiscardedBeforeStart
        );
        assert_eq!(stream.stats().frames_discarded_before_start_total, 1);
        assert!(stream.try_next().is_none());
        assert!(controller.open());
        assert_eq!(
            sender.try_send(frame(&pool, 2)),
            CapturedFrameDelivery::Delivered
        );
        assert_eq!(stream.try_next().unwrap().sequence_number, 2);
        assert!(!controller.open());
    }

    #[test]
    fn given_full_stream_when_frame_is_sent_then_newest_is_dropped_and_counted() {
        let pool = AudioBufferPool::new(3, 960);
        let (mut sender, mut stream) = captured_frame_stream(1).unwrap();

        assert_eq!(
            sender.try_send(frame(&pool, 1)),
            CapturedFrameDelivery::Delivered
        );
        assert_eq!(
            sender.try_send(frame(&pool, 2)),
            CapturedFrameDelivery::DroppedNewest
        );
        assert_eq!(stream.try_next().unwrap().sequence_number, 1);
        assert_eq!(stream.stats().dropped_newest_frames, 1);
        assert!(
            pool.acquire().is_some(),
            "dropped frame must release its pool slot"
        );
    }

    #[test]
    fn given_sender_callback_when_frame_arrives_then_stream_receives_it() {
        let pool = AudioBufferPool::new(1, 960);
        let (sender, mut stream) = captured_frame_stream(1).unwrap();
        let mut callback = sender.into_callback();

        callback(frame(&pool, 23));

        assert_eq!(stream.try_next().unwrap().sequence_number, 23);
    }

    #[test]
    fn given_sender_dropped_when_stream_checked_then_closed_is_true() {
        let (sender, stream) = captured_frame_stream(1).unwrap();
        drop(sender);
        assert!(stream.is_closed());
    }
}

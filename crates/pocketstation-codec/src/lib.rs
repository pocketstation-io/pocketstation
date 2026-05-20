use pocketstation_frame::AudioFrame;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub enum OpusFrameDuration {
    Ms10,
    Ms20,
    Ms40,
    Ms60,
}

impl OpusFrameDuration {
    pub fn samples_at_48k(self) -> usize {
        match self {
            Self::Ms10 => 480,
            Self::Ms20 => 960,
            Self::Ms40 => 1920,
            Self::Ms60 => 2880,
        }
    }
}

pub struct EncodedFrame {
    pub sequence_number: u64,
    pub timestamp_ns: u64,
    pub payload: Vec<u8>,
}

/// Mock encoder for tests and examples only. NOT suitable for production.
/// TODO(ADR-008): replace with real Opus encoder in Phase 1; hot path must
/// use the allocation-free encode_into() API below.
pub struct MockOpusEncoder;

/// Mock decoder for tests and examples only. NOT suitable for production.
/// TODO(ADR-008): replace with real Opus decoder in Phase 1.
pub struct MockOpusDecoder;

impl MockOpusEncoder {
    /// Allocation-free encode: writes raw PCM bytes into a caller-supplied
    /// buffer. Returns the number of bytes written.
    /// This is the correct hot-path API shape; `encode()` is convenience-only.
    pub fn encode_into(&mut self, frame: &AudioFrame, out: &mut Vec<u8>) -> usize {
        out.clear();
        for s in frame.buffer.as_slice() {
            out.extend_from_slice(&s.to_le_bytes());
        }
        out.len()
    }

    /// Allocates a new Vec per call — for tests and examples only.
    pub fn encode(&mut self, frame: &AudioFrame) -> EncodedFrame {
        // TODO: hot-path callers should use encode_into() with a pooled buffer.
        let mut payload = Vec::with_capacity(frame.buffer.len() * 4);
        self.encode_into(frame, &mut payload);
        EncodedFrame {
            sequence_number: frame.sequence_number,
            timestamp_ns: frame.timestamp_ns,
            payload,
        }
    }
}

impl MockOpusDecoder {
    /// Allocation-free decode: appends decoded f32 samples into a caller-
    /// supplied buffer. Returns the number of samples written.
    pub fn decode_into(&mut self, encoded: &EncodedFrame, out: &mut Vec<f32>) -> usize {
        let before = out.len();
        out.extend(
            encoded
                .payload
                .chunks_exact(4)
                .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])),
        );
        out.len() - before
    }

    /// Allocates a new Vec per call — for tests and examples only.
    pub fn decode_to_vec(&mut self, encoded: &EncodedFrame) -> Vec<f32> {
        // TODO: hot-path callers should use decode_into() with a pooled buffer.
        let mut out = Vec::with_capacity(encoded.payload.len() / 4);
        self.decode_into(encoded, &mut out);
        out
    }

}

#[cfg(feature = "real-opus")]
pub mod real_opus {
    use opus::{Application, Channels, Decoder, Encoder};

    pub struct RealOpusEncoder {
        inner: Encoder,
        channels: usize,
    }
    pub struct RealOpusDecoder {
        inner: Decoder,
        channels: usize,
    }

    impl RealOpusEncoder {
        pub fn new(sample_rate: u32, channels: usize) -> Result<Self, opus::Error> {
            let ch = if channels == 1 {
                Channels::Mono
            } else {
                Channels::Stereo
            };
            Ok(Self {
                inner: Encoder::new(sample_rate, ch, Application::Audio)?,
                channels,
            })
        }
        pub fn encode_float(&mut self, pcm: &[f32], out: &mut [u8]) -> Result<usize, opus::Error> {
            self.inner.encode_float(pcm, out)
        }
    }

    impl RealOpusDecoder {
        pub fn new(sample_rate: u32, channels: usize) -> Result<Self, opus::Error> {
            let ch = if channels == 1 {
                Channels::Mono
            } else {
                Channels::Stereo
            };
            Ok(Self {
                inner: Decoder::new(sample_rate, ch)?,
                channels,
            })
        }
        pub fn decode_float(
            &mut self,
            payload: &[u8],
            out: &mut [f32],
        ) -> Result<usize, opus::Error> {
            self.inner.decode_float(payload, out, false)
        }
    }
}

/// Phase 0 fixed-depth jitter buffer.
///
/// This is **not** production NetEQ. It is a FIFO queue that withholds frames
/// until `target_depth` have accumulated, then releases one per `pop_ready`
/// call. It does NOT:
/// - reorder late-arriving frames
/// - perform packet-loss concealment (PLC)
/// - implement adaptive depth control
///
/// ADR-009 owns the full adaptive JitterBuffer design. `sequence_gap_ahead()`
/// is provided as a hook point for the future PLC layer.
pub struct JitterBuffer {
    target_depth: usize,
    queue: VecDeque<EncodedFrame>,
    next_expected_seq: Option<u64>,
}

impl JitterBuffer {
    pub fn new(target_depth: usize) -> Self {
        Self {
            target_depth,
            queue: VecDeque::new(),
            next_expected_seq: None,
        }
    }

    pub fn push(&mut self, frame: EncodedFrame) {
        if self.next_expected_seq.is_none() {
            self.next_expected_seq = Some(frame.sequence_number + 1);
        } else {
            self.next_expected_seq = Some(
                self.next_expected_seq
                    .unwrap()
                    .max(frame.sequence_number + 1),
            );
        }
        self.queue.push_back(frame);
    }

    /// Returns `Some(frame)` once the buffer has accumulated at least
    /// `target_depth` frames; returns `None` while buffering.
    pub fn pop_ready(&mut self) -> Option<EncodedFrame> {
        if self.queue.len() >= self.target_depth {
            self.queue.pop_front()
        } else {
            None
        }
    }

    /// Returns `true` when the head of the queue has a sequence number that
    /// is not contiguous with the one before it (gap detected). Phase 0:
    /// always false when the queue is empty or there is no gap.
    /// Intended as a hook for the PLC layer (ADR-009).
    pub fn sequence_gap_ahead(&self) -> bool {
        if self.queue.len() < 2 {
            return false;
        }
        let a = self.queue[0].sequence_number;
        let b = self.queue[1].sequence_number;
        b != a + 1
    }

    pub fn depth(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_encoded(seq: u64) -> EncodedFrame {
        EncodedFrame { sequence_number: seq, timestamp_ns: seq * 20_000_000, payload: vec![] }
    }

    #[test]
    fn duration_samples() {
        assert_eq!(OpusFrameDuration::Ms20.samples_at_48k(), 960);
    }

    #[test]
    fn jitter_buffer_withholds_until_target_depth_reached() {
        let mut jb = JitterBuffer::new(3);
        jb.push(make_encoded(0));
        assert!(jb.pop_ready().is_none());
        jb.push(make_encoded(1));
        assert!(jb.pop_ready().is_none());
        jb.push(make_encoded(2));
        assert!(jb.pop_ready().is_some());
    }

    #[test]
    fn jitter_buffer_ordered_frames_pop_in_fifo_order() {
        // target_depth=1 so every push is immediately eligible; tests pure FIFO order.
        let mut jb = JitterBuffer::new(1);
        for seq in 0..4 {
            jb.push(make_encoded(seq));
        }
        for expected in 0..4u64 {
            let frame = jb.pop_ready().unwrap();
            assert_eq!(frame.sequence_number, expected);
        }
        assert!(jb.pop_ready().is_none());
    }

    #[test]
    fn jitter_buffer_late_frame_is_not_reordered_phase0() {
        // Phase 0: late frames are NOT reordered; they join the back of the queue.
        // This test documents that known limitation.
        let mut jb = JitterBuffer::new(1);
        jb.push(make_encoded(1)); // seq 1 arrives before seq 0
        jb.push(make_encoded(0)); // seq 0 arrives late
        let first = jb.pop_ready().unwrap();
        assert_eq!(
            first.sequence_number, 1,
            "Phase 0 JitterBuffer does not reorder late frames"
        );
    }

    #[test]
    fn jitter_buffer_missing_frame_detected_via_sequence_gap() {
        let mut jb = JitterBuffer::new(1);
        jb.push(make_encoded(0));
        jb.push(make_encoded(2)); // seq 1 is missing
        assert!(jb.sequence_gap_ahead(), "gap between seq 0 and seq 2 should be detected");
        // Phase 0: no PLC is generated; caller is responsible for concealment.
        let first = jb.pop_ready().unwrap();
        assert_eq!(first.sequence_number, 0);
    }

    #[test]
    fn jitter_buffer_contiguous_frames_have_no_gap() {
        let mut jb = JitterBuffer::new(1);
        jb.push(make_encoded(5));
        jb.push(make_encoded(6));
        assert!(!jb.sequence_gap_ahead());
    }
}

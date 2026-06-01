// Phase 3, ADR-012: real libopus bindings replace the Phase 0 mock.
// Dependency approved: opus = "0.3" wraps libopus via libopus-sys. Chosen over
// audiopus because it is the de-facto Rust binding (most downloads, active
// maintenance, matches the workspace dep already declared).

use opus::{Application, Channels, Decoder, Encoder};
use pocketstation_frame::AudioFrame;
use std::collections::VecDeque;

/// Opus frame duration.  20 ms is the ADR-012 default; 10 ms is available for
/// voice-agent mode once CPU/overhead benchmarks justify it.
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

/// 48 000 Hz, mono, VOIP application profile (ADR-012 default).
pub const OPUS_SAMPLE_RATE: u32 = 48_000;

/// 20 ms frame = 960 samples at 48 kHz (ADR-012).
pub const OPUS_FRAME_SAMPLES: usize = 960;

/// Maximum number of bytes the Opus encoder can emit per 20 ms frame.
/// libopus guarantees this upper bound.
pub const OPUS_MAX_PACKET_BYTES: usize = 4_000;

/// Scale factor for f32 ↔ i16 conversion.
const I16_SCALE: f32 = i16::MAX as f32;

pub struct EncodedFrame {
    pub sequence_number: u64,
    pub timestamp_ns: u64,
    pub payload: Vec<u8>,
}

/// Real Opus encoder wrapping libopus via the `opus` crate.
///
/// Configured for 48 000 Hz, mono, [`Application::Voip`], 20 ms frames
/// (960 samples) per ADR-012.
///
/// # Heap allocation notes
///
/// - `new()` allocates the libopus encoder state once; no per-frame allocation
///   inside libopus itself after that.
/// - `encode_into()` writes into a caller-supplied `Vec<u8>` (pre-allocated,
///   cleared per call).  The only allocation that may occur is if the caller
///   passes a `Vec` whose capacity is smaller than `OPUS_MAX_PACKET_BYTES`; the
///   `Vec` will then grow once and remain stable for subsequent calls.
/// - `encode()` allocates one `Vec<u8>` per call and is intended for tests and
///   examples only.  Hot-path callers must use `encode_into()` with a pooled
///   output buffer.
pub struct OpusEncoder {
    inner: Encoder,
}

impl OpusEncoder {
    /// Create a new encoder.  Returns `Err` only if libopus rejects the
    /// parameters (which cannot happen for the fixed 48 kHz / mono / Voip
    /// combination used here).
    pub fn new() -> Result<Self, opus::Error> {
        Ok(Self {
            inner: Encoder::new(OPUS_SAMPLE_RATE, Channels::Mono, Application::Voip)?,
        })
    }

    /// Encode a 960-sample f32 PCM slice into `out`.
    ///
    /// Converts f32 → i16 (multiply by 32 767.0, clamp) then calls
    /// `encoder.encode()`.  Returns the number of compressed bytes written.
    /// `out` is cleared and reused; no heap allocation occurs after the first
    /// call provided `out` already has `OPUS_MAX_PACKET_BYTES` capacity.
    pub fn encode_into(&mut self, pcm: &[f32], out: &mut Vec<u8>) -> Result<usize, opus::Error> {
        debug_assert_eq!(
            pcm.len(),
            OPUS_FRAME_SAMPLES,
            "encode_into: expected {OPUS_FRAME_SAMPLES} samples, got {}",
            pcm.len()
        );

        // f32 → i16.  Written as a plain iterator loop so LLVM auto-vectorises
        // to NEON/AVX2 when compiled with target-cpu=native (.cargo/config.toml).
        let mut i16_buf = [0i16; OPUS_FRAME_SAMPLES];
        for (dst, &src) in i16_buf.iter_mut().zip(pcm.iter()) {
            *dst = (src.clamp(-1.0, 1.0) * I16_SCALE) as i16;
        }

        // Avoid the 4 000-byte zero-fill that `resize(cap, 0)` performs.
        // Safety: libopus writes sequentially into `out` before reading any
        // byte of its output; `truncate(n)` then hides the unwritten tail.
        out.clear();
        if out.capacity() < OPUS_MAX_PACKET_BYTES {
            out.reserve(OPUS_MAX_PACKET_BYTES);
        }
        unsafe { out.set_len(OPUS_MAX_PACKET_BYTES) };

        let n = self.inner.encode(&i16_buf, out)?;
        out.truncate(n);
        Ok(n)
    }

    /// Set encoder complexity (0 = fastest, 10 = highest quality).
    ///
    /// Production default is 9. Set to 0 only in throughput benchmarks where
    /// quality is irrelevant (ADR-012 §10.4 codec control).
    pub fn set_complexity(&mut self, complexity: i32) -> Result<(), opus::Error> {
        self.inner.set_complexity(complexity)
    }

    /// Convenience wrapper that allocates a `Vec<u8>` per call.
    /// For tests and examples only; hot-path callers must use `encode_into()`.
    pub fn encode(&mut self, frame: &AudioFrame) -> Result<EncodedFrame, opus::Error> {
        let mut payload = Vec::with_capacity(OPUS_MAX_PACKET_BYTES);
        self.encode_into(frame.buffer.as_slice(), &mut payload)?;
        Ok(EncodedFrame {
            sequence_number: frame.sequence_number,
            timestamp_ns: frame.timestamp_ns,
            payload,
        })
    }
}

impl Default for OpusEncoder {
    fn default() -> Self {
        Self::new().expect("OpusEncoder::new failed with fixed parameters — libopus not linked?")
    }
}

/// Real Opus decoder wrapping libopus via the `opus` crate.
///
/// Configured for 48 000 Hz, mono per ADR-012.
///
/// # Heap allocation notes
///
/// - `new()` allocates the libopus decoder state once.
/// - `decode_into()` writes into a caller-supplied `Vec<f32>` (no internal
///   allocation after the first call, provided the `Vec` has enough capacity).
/// - `decode_to_vec()` allocates one `Vec<f32>` per call — tests/examples only.
pub struct OpusDecoder {
    inner: Decoder,
}

impl OpusDecoder {
    pub fn new() -> Result<Self, opus::Error> {
        Ok(Self {
            inner: Decoder::new(OPUS_SAMPLE_RATE, Channels::Mono)?,
        })
    }

    /// Decode a compressed Opus packet into i16 samples, then convert to f32.
    ///
    /// Appends decoded f32 samples to `out`.  Returns the number of samples
    /// appended.  No heap allocation after the first call if `out` has
    /// sufficient capacity.
    pub fn decode_into(
        &mut self,
        payload: &[u8],
        out: &mut Vec<f32>,
    ) -> Result<usize, opus::Error> {
        let before = out.len();
        let mut i16_buf = [0i16; OPUS_FRAME_SAMPLES];
        let n = self.inner.decode(payload, &mut i16_buf, false)?;

        // Pre-size output then write with a plain loop.  Avoids the per-element
        // bounds check inside `push` and is auto-vectorised by LLVM (NEON/AVX2)
        // when target-cpu=native is set.
        out.resize(before + n, 0.0f32);
        for (dst, &src) in out[before..].iter_mut().zip(&i16_buf[..n]) {
            *dst = src as f32 / I16_SCALE;
        }
        Ok(n)
    }

    /// Convenience wrapper allocating a `Vec<f32>` — tests/examples only.
    pub fn decode_to_vec(&mut self, encoded: &EncodedFrame) -> Result<Vec<f32>, opus::Error> {
        let mut out = Vec::with_capacity(OPUS_FRAME_SAMPLES);
        self.decode_into(&encoded.payload, &mut out)?;
        Ok(out)
    }
}

impl Default for OpusDecoder {
    fn default() -> Self {
        Self::new().expect("OpusDecoder::new failed with fixed parameters — libopus not linked?")
    }
}

// ---------------------------------------------------------------------------
// Legacy mock aliases — kept so that existing tests and the sine_to_wav example
// continue to compile without modification.  They delegate to the real encoder
// and decoder.  Remove in Phase 5 once all call sites have been migrated.
// ---------------------------------------------------------------------------

/// Deprecated alias for [`OpusEncoder`].  Use `OpusEncoder` directly.
pub struct MockOpusEncoder {
    pub inner: OpusEncoder,
}

impl MockOpusEncoder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: OpusEncoder::default(),
        }
    }

    /// Allocation-free encode into a caller-supplied buffer.
    pub fn encode_into(&mut self, frame: &AudioFrame, out: &mut Vec<u8>) -> usize {
        self.inner
            .encode_into(frame.buffer.as_slice(), out)
            .expect("MockOpusEncoder.encode_into failed")
    }

    /// Allocates per call — for tests and examples only.
    pub fn encode(&mut self, frame: &AudioFrame) -> EncodedFrame {
        self.inner
            .encode(frame)
            .expect("MockOpusEncoder.encode failed")
    }
}

impl Default for MockOpusEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Deprecated alias for [`OpusDecoder`].  Use `OpusDecoder` directly.
pub struct MockOpusDecoder {
    inner: OpusDecoder,
}

impl MockOpusDecoder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: OpusDecoder::default(),
        }
    }

    /// Allocation-free decode from a raw byte slice.
    pub fn decode_slice_into(&mut self, payload: &[u8], out: &mut Vec<f32>) -> usize {
        self.inner
            .decode_into(payload, out)
            .expect("MockOpusDecoder.decode_slice_into failed")
    }

    /// Allocation-free decode from an [`EncodedFrame`].
    pub fn decode_into(&mut self, encoded: &EncodedFrame, out: &mut Vec<f32>) -> usize {
        self.decode_slice_into(&encoded.payload, out)
    }

    /// Allocates per call — for tests and examples only.
    pub fn decode_to_vec(&mut self, encoded: &EncodedFrame) -> Vec<f32> {
        self.inner
            .decode_to_vec(encoded)
            .expect("MockOpusDecoder.decode_to_vec failed")
    }
}

impl Default for MockOpusDecoder {
    fn default() -> Self {
        Self::new()
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
}

impl JitterBuffer {
    pub fn new(target_depth: usize) -> Self {
        Self {
            target_depth,
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, frame: EncodedFrame) {
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
    use pocketstation_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

    fn make_encoded(seq: u64) -> EncodedFrame {
        EncodedFrame {
            sequence_number: seq,
            timestamp_ns: seq * 20_000_000,
            payload: vec![],
        }
    }

    #[test]
    fn opus_frame_duration_ms20_is_960_samples_at_48k() {
        // Given / When / Then
        assert_eq!(OpusFrameDuration::Ms20.samples_at_48k(), 960);
    }

    #[test]
    fn opus_encoder_encodes_960_sample_frame_to_non_empty_packet() {
        // Given: 960 silent samples (valid 20 ms frame per ADR-012)
        let mut enc = OpusEncoder::new().unwrap();
        let pcm = vec![0.0f32; OPUS_FRAME_SAMPLES];
        let mut out = Vec::new();

        // When
        let n = enc.encode_into(&pcm, &mut out).unwrap();

        // Then: packet is non-empty and length matches the returned count
        assert!(n > 0, "encoded packet must be non-empty");
        assert_eq!(out.len(), n);
    }

    #[test]
    fn opus_decoder_decodes_encoded_packet_to_960_samples() {
        // Given: encode a 20 ms frame of silence
        let mut enc = OpusEncoder::new().unwrap();
        let mut dec = OpusDecoder::new().unwrap();
        let pcm_in = vec![0.0f32; OPUS_FRAME_SAMPLES];
        let mut packet = Vec::new();
        enc.encode_into(&pcm_in, &mut packet).unwrap();

        let mut pcm_out = Vec::new();

        // When
        let n = dec.decode_into(&packet, &mut pcm_out).unwrap();

        // Then: decoder produces exactly one frame of samples
        assert_eq!(n, OPUS_FRAME_SAMPLES);
        assert_eq!(pcm_out.len(), OPUS_FRAME_SAMPLES);
    }

    #[test]
    fn opus_round_trip_sine_preserves_approximate_magnitude() {
        // Given: 960-sample 440 Hz sine at 48 kHz, amplitude 0.25
        let mut enc = OpusEncoder::new().unwrap();
        let mut dec = OpusDecoder::new().unwrap();

        use std::f32::consts::PI;
        let pcm_in: Vec<f32> = (0..OPUS_FRAME_SAMPLES)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 48_000.0).sin() * 0.25)
            .collect();

        let mut packet = Vec::new();
        enc.encode_into(&pcm_in, &mut packet).unwrap();

        let mut pcm_out = Vec::new();
        dec.decode_into(&packet, &mut pcm_out).unwrap();

        // Then: RMS of decoded signal is within 10 dB of the original
        let rms_in = rms(&pcm_in);
        let rms_out = rms(&pcm_out);
        let ratio = rms_out / rms_in;
        assert!(
            ratio > 0.3 && ratio < 3.0,
            "RMS ratio {ratio:.3} outside acceptable range (Opus VOIP mode may attenuate sine)"
        );
    }

    fn rms(s: &[f32]) -> f32 {
        let sum_sq: f32 = s.iter().map(|x| x * x).sum();
        (sum_sq / s.len() as f32).sqrt()
    }

    #[test]
    fn jitter_buffer_withholds_frames_until_target_depth_is_reached() {
        // Given
        let mut jb = JitterBuffer::new(3);

        // When / Then (interleaved: each push either gates or releases)
        jb.push(make_encoded(0));
        assert!(jb.pop_ready().is_none());
        jb.push(make_encoded(1));
        assert!(jb.pop_ready().is_none());
        jb.push(make_encoded(2));
        assert!(jb.pop_ready().is_some());
    }

    #[test]
    fn jitter_buffer_ordered_frames_pop_in_fifo_order() {
        // Given: target_depth=1 so every push is immediately eligible
        let mut jb = JitterBuffer::new(1);
        for seq in 0..4 {
            jb.push(make_encoded(seq));
        }

        // When / Then
        for expected in 0..4u64 {
            let frame = jb.pop_ready().unwrap();
            assert_eq!(frame.sequence_number, expected);
        }
        assert!(jb.pop_ready().is_none());
    }

    #[test]
    fn jitter_buffer_late_frame_is_not_reordered_in_phase0() {
        // Given: seq 1 arrives before seq 0 (out-of-order delivery)
        let mut jb = JitterBuffer::new(1);
        jb.push(make_encoded(1));
        jb.push(make_encoded(0));

        // When
        let first = jb.pop_ready().unwrap();

        // Then: Phase 0 does not reorder late frames; documents known limitation
        assert_eq!(
            first.sequence_number, 1,
            "Phase 0 JitterBuffer does not reorder late frames"
        );
    }

    #[test]
    fn jitter_buffer_detects_sequence_gap_when_frame_is_missing() {
        // Given
        let mut jb = JitterBuffer::new(1);
        jb.push(make_encoded(0));
        jb.push(make_encoded(2)); // seq 1 is missing

        // When / Then
        assert!(
            jb.sequence_gap_ahead(),
            "gap between seq 0 and seq 2 should be detected"
        );
        // Phase 0: no PLC is generated; caller is responsible for concealment.
        let first = jb.pop_ready().unwrap();
        assert_eq!(first.sequence_number, 0);
    }

    #[test]
    fn jitter_buffer_contiguous_frames_report_no_gap() {
        // Given
        let mut jb = JitterBuffer::new(1);

        // When
        jb.push(make_encoded(5));
        jb.push(make_encoded(6));

        // Then
        assert!(!jb.sequence_gap_ahead());
    }

    #[test]
    fn mock_encoder_and_decoder_round_trip_via_legacy_api() {
        // Given: legacy API used by sine_to_wav example
        let pool = AudioBufferPool::new(2, OPUS_FRAME_SAMPLES);
        let handle = pool.acquire().unwrap();
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, handle);
        let mut enc = MockOpusEncoder::default();
        let mut dec = MockOpusDecoder::default();

        // When
        let encoded = enc.encode(&frame);
        let decoded = dec.decode_to_vec(&encoded);

        // Then: decoded samples equal one frame
        assert_eq!(decoded.len(), OPUS_FRAME_SAMPLES);
    }
}

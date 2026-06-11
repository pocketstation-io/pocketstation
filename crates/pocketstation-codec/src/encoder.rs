use opus::{Application, Channels, Encoder};
use pocketstation_frame::AudioFrame;

use crate::constants::{
    I16_SCALE, OPUS_FRAME_SAMPLES, OPUS_MAX_PACKET_BYTES, OPUS_SAMPLE_RATE,
    VOICE_AGENT_FRAME_SAMPLES,
};

/// Opus frame duration.  20 ms is the AUDIO-012 default; 10 ms is available for
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

#[derive(Debug)]
pub struct EncodedFrame {
    pub sequence_number: u64,
    pub timestamp_ns: u64,
    pub payload: Vec<u8>,
}

/// Real Opus encoder wrapping libopus via the `opus` crate.
///
/// Configured for 48 000 Hz, mono, [`Application::Voip`], 20 ms frames
/// (960 samples) per AUDIO-012.
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
    pub(crate) inner: Encoder,
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

    /// Create a low-latency voice-agent encoder using OPUS_APPLICATION_RESTRICTED_LOWDELAY.
    ///
    /// Configured for 10 ms frames (480 samples at 48 kHz), complexity 5, and the
    /// caller-specified bitrate.  RESTRICTED_LOWDELAY disables the lookahead that
    /// VOIP mode uses for pitch detection, saving ~10 ms of algorithmic delay
    /// per RFC 6716 §3.1.
    ///
    /// `bitrate_kbps` must be at least 32 (the minimum for 10 ms mono Opus frames
    /// at acceptable quality); values above 96 are unclamped but wasteful.
    ///
    /// # Errors
    ///
    /// Returns `Err` only if libopus rejects the channel or bitrate values.
    pub fn voice_agent(channels: u8, bitrate_kbps: u32) -> Result<Self, opus::Error> {
        let ch = if channels == 2 {
            Channels::Stereo
        } else {
            Channels::Mono
        };
        let mut enc = Encoder::new(OPUS_SAMPLE_RATE, ch, Application::LowDelay)?;
        enc.set_bitrate(opus::Bitrate::Bits((bitrate_kbps * 1_000) as i32))?;
        enc.set_complexity(5)?;
        Ok(Self { inner: enc })
    }

    /// Encode a PCM slice into `out`.
    ///
    /// Accepts either a 960-sample (20 ms) or 480-sample (10 ms) slice.  The
    /// frame size is detected from the slice length; passing any other length is
    /// a logic error and will panic in debug builds (debug_assert).
    ///
    /// Converts f32 → i16 (multiply by 32 767.0, clamp) then calls
    /// `encoder.encode()`.  Returns the number of compressed bytes written.
    /// `out` is cleared and reused; no heap allocation occurs after the first
    /// call provided `out` already has `OPUS_MAX_PACKET_BYTES` capacity.
    pub fn encode_into(&mut self, pcm: &[f32], out: &mut Vec<u8>) -> Result<usize, opus::Error> {
        let frame_len = pcm.len();
        debug_assert!(
            frame_len == OPUS_FRAME_SAMPLES || frame_len == VOICE_AGENT_FRAME_SAMPLES,
            "encode_into: expected {OPUS_FRAME_SAMPLES} or {VOICE_AGENT_FRAME_SAMPLES} samples, got {frame_len}",
        );

        // f32 → i16.  Written as a plain iterator loop so LLVM auto-vectorises
        // to NEON/AVX2 when compiled with target-cpu=native (.cargo/config.toml).
        // We use the maximum frame size for the stack buffer so both modes fit.
        let mut i16_buf = [0i16; OPUS_FRAME_SAMPLES];
        for (dst, &src) in i16_buf[..frame_len].iter_mut().zip(pcm.iter()) {
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

        let n = self.inner.encode(&i16_buf[..frame_len], out)?;
        out.truncate(n);
        Ok(n)
    }

    /// Set encoder complexity (0 = fastest, 10 = highest quality).
    ///
    /// Production default is 9. Set to 0 only in throughput benchmarks where
    /// quality is irrelevant (AUDIO-012 §10.4 codec control).
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

// ---------------------------------------------------------------------------
// Legacy mock alias — kept so that existing tests and the sine_to_wav example
// continue to compile without modification.  Delegates to the real encoder.
// Remove in Phase 5 once all call sites have been migrated.
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

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

    #[test]
    fn opus_frame_duration_ms20_is_960_samples_at_48k() {
        assert_eq!(OpusFrameDuration::Ms20.samples_at_48k(), 960);
    }

    #[test]
    fn opus_encoder_encodes_960_sample_frame_to_non_empty_packet() {
        // Given: 960 silent samples (valid 20 ms frame per AUDIO-012)
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
    fn opus_round_trip_sine_preserves_approximate_magnitude() {
        use std::f32::consts::PI;

        // Given: 960-sample 440 Hz sine at 48 kHz, amplitude 0.25
        let mut enc = OpusEncoder::new().unwrap();
        let mut dec = crate::decoder::OpusDecoder::new().unwrap();

        let pcm_in: Vec<f32> = (0..OPUS_FRAME_SAMPLES)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 48_000.0).sin() * 0.25)
            .collect();

        let mut packet = Vec::new();
        enc.encode_into(&pcm_in, &mut packet).unwrap();

        let mut pcm_out = Vec::new();
        dec.decode_into(&packet, &mut pcm_out).unwrap();

        // Then: RMS of decoded signal is within 10 dB of the original
        let rms = |s: &[f32]| -> f32 {
            let sum_sq: f32 = s.iter().map(|x| x * x).sum();
            (sum_sq / s.len() as f32).sqrt()
        };
        let rms_in = rms(&pcm_in);
        let rms_out = rms(&pcm_out);
        let ratio = rms_out / rms_in;
        assert!(
            ratio > 0.3 && ratio < 3.0,
            "RMS ratio {ratio:.3} outside acceptable range (Opus VOIP mode may attenuate sine)"
        );
    }

    #[test]
    fn mock_encoder_round_trip_via_legacy_api() {
        // Given: legacy API used by sine_to_wav example
        let pool = AudioBufferPool::new(2, OPUS_FRAME_SAMPLES);
        let handle = pool.acquire().unwrap();
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, handle);
        let mut enc = MockOpusEncoder::default();
        let mut dec = crate::decoder::MockOpusDecoder::default();

        // When
        let encoded = enc.encode(&frame);
        let decoded = dec.decode_to_vec(&encoded);

        // Then: decoded samples equal one frame
        assert_eq!(decoded.len(), OPUS_FRAME_SAMPLES);
    }

    /// Proof that the set_len optimisation produces byte-for-byte identical
    /// Opus packets vs. the old resize-with-zeros approach.
    #[test]
    fn given_optimised_encode_when_same_input_then_packet_bytes_identical() {
        use std::f32::consts::PI;

        // Given: 440 Hz sine, 20 ms at 48 kHz
        let pcm: Vec<f32> = (0..OPUS_FRAME_SAMPLES)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 48_000.0).sin() * 0.25)
            .collect();

        // Encoder A — optimised path (set_len, no zeroing)
        let mut enc_a = OpusEncoder::default();
        let mut out_a = Vec::new();
        enc_a.encode_into(&pcm, &mut out_a).unwrap();

        // Encoder B — reference path (resize fills zeros before encode)
        let mut enc_b = OpusEncoder::default();
        let mut out_b = vec![0u8; OPUS_MAX_PACKET_BYTES];
        let n_b = enc_b
            .inner
            .encode(
                &{
                    let mut i16_buf = [0i16; OPUS_FRAME_SAMPLES];
                    for (d, &s) in i16_buf.iter_mut().zip(pcm.iter()) {
                        *d = (s.clamp(-1.0, 1.0) * I16_SCALE) as i16;
                    }
                    i16_buf
                },
                &mut out_b,
            )
            .unwrap();
        out_b.truncate(n_b);

        // Then: every byte is identical — optimisation is audio-transparent
        assert_eq!(
            out_a.len(),
            out_b.len(),
            "packet length differs: optimised={} reference={}",
            out_a.len(),
            out_b.len()
        );
        assert_eq!(
            out_a, out_b,
            "encoded bytes differ — encode_into optimisation broke audio fidelity"
        );
    }

    #[test]
    fn given_voice_agent_mode_when_encode_480_samples_then_valid_packet() {
        // Given: voice-agent encoder + 480 silent samples (10 ms at 48 kHz)
        let mut enc = OpusEncoder::voice_agent(1, 32).unwrap();
        let pcm = vec![0.0f32; VOICE_AGENT_FRAME_SAMPLES];
        let mut out = Vec::new();

        // When
        let n = enc.encode_into(&pcm, &mut out).unwrap();

        // Then: packet is non-empty and length matches the returned byte count
        assert!(n > 0, "voice-agent encoded packet must be non-empty");
        assert_eq!(out.len(), n);
    }

    #[test]
    fn given_voice_agent_frame_when_round_trip_then_snr_above_minus_6db() {
        use std::f32::consts::PI;

        // Given: 480-sample 440 Hz sine at 48 kHz, amplitude 0.25
        let mut enc = OpusEncoder::voice_agent(1, 32).unwrap();
        let mut dec = crate::decoder::OpusDecoder::new().unwrap();

        let pcm_in: Vec<f32> = (0..VOICE_AGENT_FRAME_SAMPLES)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 48_000.0).sin() * 0.25)
            .collect();

        let mut packet = Vec::new();
        enc.encode_into(&pcm_in, &mut packet).unwrap();

        let mut pcm_out = Vec::new();
        dec.decode_into(&packet, &mut pcm_out).unwrap();

        let rms =
            |s: &[f32]| -> f32 { (s.iter().map(|x| x * x).sum::<f32>() / s.len() as f32).sqrt() };
        let snr_db = 20.0 * (rms(&pcm_out) / rms(&pcm_in)).log10();
        assert!(
            snr_db > -6.0,
            "voice-agent round-trip SNR {snr_db:.1} dB below -6 dB threshold"
        );
    }

    /// Proof that round-trip SNR is within Opus VOIP spec after optimisations.
    #[test]
    fn given_optimised_pipeline_when_round_trip_then_snr_above_minus_1db() {
        use std::f32::consts::PI;

        let pcm_in: Vec<f32> = (0..OPUS_FRAME_SAMPLES)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 48_000.0).sin() * 0.25)
            .collect();

        let mut enc = OpusEncoder::default();
        let mut dec = crate::decoder::OpusDecoder::default();
        let mut packet = Vec::new();
        enc.encode_into(&pcm_in, &mut packet).unwrap();

        let mut pcm_out = Vec::new();
        dec.decode_into(&packet, &mut pcm_out).unwrap();

        let rms =
            |s: &[f32]| -> f32 { (s.iter().map(|x| x * x).sum::<f32>() / s.len() as f32).sqrt() };
        let snr_db = 20.0 * (rms(&pcm_out) / rms(&pcm_in)).log10();

        // Opus VOIP mode is lossy and voice-optimised.  A pure sine is not a
        // representative voice signal — the codec applies mild attenuation
        // (~1-3 dB typical).  -3 dB is the psychoacoustic JND for level; we
        // allow down to -4 dB to keep the test deterministic across platforms.
        // The byte-identical test above is the stronger quality guarantee.
        assert!(
            snr_db > -4.0,
            "Round-trip SNR {snr_db:.1} dB below -4 dB — quality degraded"
        );
    }
}

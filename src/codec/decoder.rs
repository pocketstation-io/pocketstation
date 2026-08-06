use opus::{Channels, Decoder};

use crate::codec::constants::{I16_SCALE, OPUS_FRAME_SAMPLES, OPUS_SAMPLE_RATE_HZ};
use crate::codec::encoder::{EncodedFrame, OpusChannels, OpusFrameDuration};

const OPUS_MAX_FRAME_SAMPLES_PER_CHANNEL: usize = 2_880; // 60 ms at 48 kHz

/// Real Opus decoder wrapping libopus via the `opus` crate.
///
/// Configured for 48 000 Hz, mono per AUDIO-012.
///
/// # Heap allocation notes
///
/// - `new()` allocates the libopus decoder state once.
/// - `decode_into()` writes into a caller-supplied `Vec<f32>` (no internal
///   allocation after the first call, provided the `Vec` has enough capacity).
/// - `decode_to_vec()` allocates one `Vec<f32>` per call — tests/examples only.
pub struct OpusDecoder {
    inner: Decoder,
    /// Channel count (1 = mono, 2 = stereo). libopus decodes `frame_size`
    /// samples per channel and writes `frame_size × channels` interleaved values,
    /// so the output sizing depends on this.
    channels: usize,
}

impl OpusDecoder {
    /// Mono decoder (48 kHz). Back-compatible default for the existing pipeline.
    pub fn new() -> Result<Self, opus::Error> {
        Self::with_channels(OpusChannels::Mono)
    }

    /// Decoder for an explicit channel layout. Use `Stereo` to decode the music
    /// pipeline's stereo Opus stream into interleaved L/R f32.
    pub fn with_channels(channels: OpusChannels) -> Result<Self, opus::Error> {
        let (ch, n) = match channels {
            OpusChannels::Mono => (Channels::Mono, 1),
            OpusChannels::Stereo => (Channels::Stereo, 2),
        };
        Ok(Self {
            inner: Decoder::new(OPUS_SAMPLE_RATE_HZ, ch)?,
            channels: n,
        })
    }

    /// Decode a compressed Opus packet into i16 samples, then convert to f32.
    ///
    /// Appends decoded f32 samples to `out`.  Returns the number of samples
    /// appended.  No heap allocation after the first call if `out` has
    /// sufficient capacity.
    ///
    /// `fec` — pass `false` for normal (no-loss) decoding.  Pass `true` when
    /// the caller detects a gap in the sequence number stream: Opus will extract
    /// forward error correction data embedded in `payload` by the sender and use
    /// it to reconstruct the *preceding* lost packet.  Callers must hold the
    /// current packet and call this first with `fec=true` (and the *previous*
    /// lost-packet length hint) to recover the lost frame, then again with
    /// `fec=false` to decode the current packet normally.
    pub fn decode_into(
        &mut self,
        payload: &[u8],
        out: &mut Vec<f32>,
        fec: bool,
    ) -> Result<usize, opus::Error> {
        let before = out.len();
        // Sized for the widest case (20 ms stereo = 1920 interleaved samples).
        let mut i16_buf = [0i16; OPUS_FRAME_SAMPLES * 2];
        // libopus returns samples-PER-CHANNEL; the buffer holds that many × channels
        // interleaved values.
        let per_channel = self.inner.decode(payload, &mut i16_buf, fec)?;
        let total = per_channel * self.channels;

        // Pre-size output then write with a plain loop.  Avoids the per-element
        // bounds check inside `push` and is auto-vectorised by LLVM (NEON/AVX2)
        // when target-cpu=native is set.
        out.resize(before + total, 0.0f32);
        for (dst, &src) in out[before..].iter_mut().zip(&i16_buf[..total]) {
            *dst = src as f32 / I16_SCALE;
        }
        Ok(total)
    }

    /// Conceal one missing packet while preserving libopus decoder state.
    ///
    /// The caller declares the missing packet duration because an empty Opus
    /// payload does not carry that information. Output appends to caller-owned
    /// storage and remains allocation-stable after the vector is pre-sized.
    pub fn decode_plc_into(
        &mut self,
        frame_duration: OpusFrameDuration,
        out: &mut Vec<f32>,
    ) -> Result<usize, opus::Error> {
        let frame_samples_per_channel = frame_duration.samples_at_48k();
        let total_samples = frame_samples_per_channel * self.channels;
        let before_samples = out.len();
        let mut i16_buf = [0i16; OPUS_MAX_FRAME_SAMPLES_PER_CHANNEL * 2];
        let decoded_samples_per_channel =
            self.inner
                .decode(&[], &mut i16_buf[..total_samples], false)?;
        let decoded_samples = decoded_samples_per_channel * self.channels;

        out.resize(before_samples + decoded_samples, 0.0);
        for (dst, &src) in out[before_samples..]
            .iter_mut()
            .zip(&i16_buf[..decoded_samples])
        {
            *dst = src as f32 / I16_SCALE;
        }
        Ok(decoded_samples)
    }

    /// Convenience wrapper allocating a `Vec<f32>` — tests/examples only.
    pub fn decode_to_vec(&mut self, encoded: &EncodedFrame) -> Result<Vec<f32>, opus::Error> {
        let mut out = Vec::with_capacity(OPUS_FRAME_SAMPLES);
        self.decode_into(&encoded.payload, &mut out, false)?;
        Ok(out)
    }
}

impl Default for OpusDecoder {
    fn default() -> Self {
        Self::new().expect("OpusDecoder::new failed with fixed parameters — libopus not linked?")
    }
}

// Legacy mock alias — kept so that existing tests continue to compile without
// modification.  Delegates to the real decoder.
// Remove in Phase 5 once all call sites have been migrated.

/// Deprecated alias for [`OpusDecoder`].  Use `OpusDecoder` directly.
#[cfg(any(test, feature = "test-helpers"))]
pub struct MockOpusDecoder {
    inner: OpusDecoder,
}

#[cfg(any(test, feature = "test-helpers"))]
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
            .decode_into(payload, out, false)
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

#[cfg(any(test, feature = "test-helpers"))]
impl Default for MockOpusDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::constants::OPUS_FRAME_SAMPLES;
    use crate::codec::encoder::OpusEncoder;

    #[test]
    fn given_encoded_opus_packet_when_decoded_then_contains_960_samples() {
        // Given: encode a 20 ms frame of silence
        let mut enc = OpusEncoder::new().unwrap();
        let mut dec = OpusDecoder::new().unwrap();
        let pcm_in = vec![0.0f32; OPUS_FRAME_SAMPLES];
        let mut packet = Vec::new();
        enc.encode_into(&pcm_in, &mut packet).unwrap();

        let mut pcm_out = Vec::new();

        // When
        let n = dec.decode_into(&packet, &mut pcm_out, false).unwrap();

        // Then: decoder produces exactly one frame of samples
        assert_eq!(n, OPUS_FRAME_SAMPLES);
        assert_eq!(pcm_out.len(), OPUS_FRAME_SAMPLES);
    }

    #[test]
    fn given_mono_decoder_when_concealing_10ms_then_480_samples_are_appended() {
        let mut decoder = OpusDecoder::new().unwrap();
        let mut pcm_out = Vec::with_capacity(480);

        let decoded_samples = decoder
            .decode_plc_into(OpusFrameDuration::Ms10, &mut pcm_out)
            .unwrap();

        assert_eq!(decoded_samples, 480);
        assert_eq!(pcm_out.len(), 480);
    }

    #[test]
    fn given_stereo_decoder_when_concealing_20ms_then_1920_samples_are_appended() {
        let mut decoder = OpusDecoder::with_channels(OpusChannels::Stereo).unwrap();
        let mut pcm_out = Vec::with_capacity(1_920);

        let decoded_samples = decoder
            .decode_plc_into(OpusFrameDuration::Ms20, &mut pcm_out)
            .unwrap();

        assert_eq!(decoded_samples, 1_920);
        assert_eq!(pcm_out.len(), 1_920);
    }

    #[test]
    fn given_presized_output_when_concealing_repeatedly_then_capacity_stays_fixed() {
        let mut decoder = OpusDecoder::new().unwrap();
        let mut pcm_out = Vec::with_capacity(OPUS_FRAME_SAMPLES);
        let initial_capacity_samples = pcm_out.capacity();

        decoder
            .decode_plc_into(OpusFrameDuration::Ms20, &mut pcm_out)
            .unwrap();
        pcm_out.clear();
        decoder
            .decode_plc_into(OpusFrameDuration::Ms20, &mut pcm_out)
            .unwrap();

        assert_eq!(pcm_out.capacity(), initial_capacity_samples);
    }
}

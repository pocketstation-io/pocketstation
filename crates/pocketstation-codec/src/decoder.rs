use opus::{Channels, Decoder};

use crate::constants::{I16_SCALE, OPUS_FRAME_SAMPLES, OPUS_SAMPLE_RATE};
use crate::encoder::EncodedFrame;

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
// Legacy mock alias — kept so that existing tests continue to compile without
// modification.  Delegates to the real decoder.
// Remove in Phase 5 once all call sites have been migrated.
// ---------------------------------------------------------------------------

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

#[cfg(any(test, feature = "test-helpers"))]
impl Default for MockOpusDecoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::OPUS_FRAME_SAMPLES;
    use crate::encoder::OpusEncoder;

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
}

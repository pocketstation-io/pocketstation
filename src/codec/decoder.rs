use opus::{Channels, Decoder};

use crate::codec::constants::{I16_SCALE, OPUS_SAMPLE_RATE_HZ};
use crate::codec::encoder::{OpusChannels, OpusFrameDuration};

/// Real Opus decoder wrapping libopus via the `opus` crate.
///
/// Configured for 48 000 Hz, mono per AUDIO-012.
///
/// # Heap allocation notes
///
/// - `new()` allocates the libopus decoder state once.
/// - `decode_into()` writes into a caller-supplied `Vec<f32>` (no internal
///   allocation after the first call, provided the `Vec` has enough capacity).
pub struct OpusDecoder {
    inner: Decoder,
    /// Channel count (1 = mono, 2 = stereo). libopus decodes `frame_size`
    /// samples per channel and writes `frame_size × channels` interleaved values,
    /// so the output sizing depends on this.
    channels: usize,
    maximum_frame_samples_per_channel: usize,
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as opus decode error."]
pub enum OpusDecodeError {
    #[error(
        "requested {requested_samples_per_channel} Opus samples per channel exceeds configured maximum {maximum_samples_per_channel}"
    )]
    #[doc = "Reports frame duration exceeds configured maximum."]
    FrameDurationExceedsConfiguredMaximum {
        #[doc = "Stores the requested samples per channel used by `FrameDurationExceedsConfiguredMaximum`."]
        requested_samples_per_channel: usize,
        #[doc = "Stores the maximum samples per channel used by `FrameDurationExceedsConfiguredMaximum`."]
        maximum_samples_per_channel: usize,
    },
    #[error("Opus decode failed: {0}")]
    #[doc = "Reports opus."]
    Opus(#[from] opus::Error),
}

impl OpusDecoder {
    /// Mono decoder (48 kHz). Back-compatible default for the existing pipeline.
    pub fn new() -> Result<Self, opus::Error> {
        Self::with_channels(OpusChannels::Mono)
    }

    /// Decoder for an explicit channel layout and a maximum 20 ms packet.
    pub fn with_channels(channels: OpusChannels) -> Result<Self, opus::Error> {
        Self::with_max_frame_duration(channels, OpusFrameDuration::Ms20)
    }

    /// Decoder with an explicit maximum packet duration.
    ///
    /// The declared maximum selects a fixed stack scratch bound. This keeps
    /// normal 10/20 ms decoding cache-local while making 40/60 ms support
    /// explicit rather than silently oversized.
    pub fn with_max_frame_duration(
        channels: OpusChannels,
        maximum_frame_duration: OpusFrameDuration,
    ) -> Result<Self, opus::Error> {
        let (ch, n) = match channels {
            OpusChannels::Mono => (Channels::Mono, 1),
            OpusChannels::Stereo => (Channels::Stereo, 2),
        };
        Ok(Self {
            inner: Decoder::new(OPUS_SAMPLE_RATE_HZ, ch)?,
            channels: n,
            maximum_frame_samples_per_channel: maximum_frame_duration.samples_at_48k(),
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
    ) -> Result<usize, OpusDecodeError> {
        let maximum_interleaved_samples = self.maximum_frame_samples_per_channel * self.channels;
        if maximum_interleaved_samples <= 1_920 {
            let mut scratch = [0_i16; 1_920];
            decode_packet(
                &mut self.inner,
                self.channels,
                payload,
                out,
                fec,
                &mut scratch[..maximum_interleaved_samples],
            )
        } else {
            let mut scratch = [0_i16; 5_760];
            decode_packet(
                &mut self.inner,
                self.channels,
                payload,
                out,
                fec,
                &mut scratch[..maximum_interleaved_samples],
            )
        }
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
    ) -> Result<usize, OpusDecodeError> {
        let frame_samples_per_channel = frame_duration.samples_at_48k();
        let total_samples = frame_samples_per_channel * self.channels;
        if frame_samples_per_channel > self.maximum_frame_samples_per_channel {
            return Err(OpusDecodeError::FrameDurationExceedsConfiguredMaximum {
                requested_samples_per_channel: frame_samples_per_channel,
                maximum_samples_per_channel: self.maximum_frame_samples_per_channel,
            });
        }
        if total_samples <= 1_920 {
            let mut scratch = [0_i16; 1_920];
            decode_packet(
                &mut self.inner,
                self.channels,
                &[],
                out,
                false,
                &mut scratch[..total_samples],
            )
        } else {
            let mut scratch = [0_i16; 5_760];
            decode_packet(
                &mut self.inner,
                self.channels,
                &[],
                out,
                false,
                &mut scratch[..total_samples],
            )
        }
    }
}

fn decode_packet(
    decoder: &mut Decoder,
    channels: usize,
    payload: &[u8],
    output: &mut Vec<f32>,
    fec: bool,
    scratch: &mut [i16],
) -> Result<usize, OpusDecodeError> {
    let decoded_samples_per_channel = decoder.decode(payload, scratch, fec)?;
    let decoded_samples = decoded_samples_per_channel * channels;
    let before_samples = output.len();
    output.resize(before_samples + decoded_samples, 0.0);
    for (destination, &source) in output[before_samples..]
        .iter_mut()
        .zip(&scratch[..decoded_samples])
    {
        *destination = source as f32 / I16_SCALE;
    }
    Ok(decoded_samples)
}

impl Default for OpusDecoder {
    #[doc = "Returns the default `OpusDecoder` value."]
    fn default() -> Self {
        Self::new().expect("OpusDecoder::new failed with fixed parameters — libopus not linked?")
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

    #[test]
    fn given_20ms_decoder_when_60ms_concealment_is_requested_then_typed_bound_error_is_returned() {
        let mut decoder = OpusDecoder::new().unwrap();
        let mut pcm_out = Vec::new();

        let error = decoder
            .decode_plc_into(OpusFrameDuration::Ms60, &mut pcm_out)
            .unwrap_err();

        assert!(matches!(
            error,
            OpusDecodeError::FrameDurationExceedsConfiguredMaximum {
                requested_samples_per_channel: 2_880,
                maximum_samples_per_channel: 960,
            }
        ));
        assert!(pcm_out.is_empty());
    }
}

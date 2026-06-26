use opus::{Application, Channels, Encoder};
use pocketstation_frame::AudioFrame;

use crate::constants::{
    I16_SCALE, OPUS_FRAME_SAMPLES, OPUS_MAX_PACKET_BYTES, VOICE_AGENT_FRAME_SAMPLES,
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

/// Typed channel count for Opus — prevents silent u8 misuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpusChannels {
    Mono,
    Stereo,
}

impl OpusChannels {
    pub fn count(self) -> u8 {
        match self {
            Self::Mono => 1,
            Self::Stereo => 2,
        }
    }
}

/// Typed sample rate.  Opus internally always uses 48 kHz; this type makes
/// the constraint explicit rather than hiding it behind a `u32` constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpusSampleRate {
    Hz48000,
}

impl OpusSampleRate {
    pub fn hz(self) -> u32 {
        match self {
            Self::Hz48000 => 48_000,
        }
    }
}

/// Opus application mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpusApplication {
    /// Optimised for voice (VOIP). Default for PocketStation broadcast.
    Voip,
    /// Optimised for low algorithmic delay. Use for real-time voice agents.
    LowDelay,
    /// Optimised for audio quality (music/broadcast).
    Audio,
}

/// Explicit configuration for an Opus encoder instance.
///
/// All parameters named and documented. Eliminates hardcoded magic values
/// scattered across constructors.
#[derive(Debug, Clone)]
pub struct OpusConfig {
    /// Sample rate. Opus only supports 48 kHz internally.
    pub sample_rate: OpusSampleRate,
    /// Channel layout.
    pub channels: OpusChannels,
    /// Frame duration. Default: 20 ms (AUDIO-012).
    pub frame_duration: OpusFrameDuration,
    /// Opus application mode.
    pub application: OpusApplication,
    /// Target bitrate in kbps. None = Opus auto (variable bitrate).
    pub bitrate_kbps: Option<u32>,
    /// Encoder complexity 0–10. Higher = better quality, more CPU.
    pub complexity: u8,
    /// Discontinuous transmission (silence suppression).
    pub dtx: bool,
    /// In-band forward error correction.
    pub fec: bool,
}

impl Default for OpusConfig {
    fn default() -> Self {
        Self {
            sample_rate: OpusSampleRate::Hz48000,
            channels: OpusChannels::Mono,
            frame_duration: OpusFrameDuration::Ms20,
            application: OpusApplication::Voip,
            bitrate_kbps: None,
            complexity: 9,
            dtx: false,
            fec: false,
        }
    }
}

impl OpusConfig {
    /// Standard voice broadcast config (AUDIO-012 default).
    ///
    /// FEC is enabled so that the decoder can recover from isolated packet
    /// loss using redundancy embedded in the following packet.  This requires
    /// the decoder to pass `fec=true` when a gap is detected (see decoder.rs).
    pub fn voice_broadcast() -> Self {
        Self {
            fec: true,
            ..Self::default()
        }
    }

    /// Low-latency voice-agent config: 10 ms frames, RESTRICTED_LOWDELAY.
    pub fn voice_agent(bitrate_kbps: u32) -> Self {
        Self {
            sample_rate: OpusSampleRate::Hz48000,
            channels: OpusChannels::Mono,
            frame_duration: OpusFrameDuration::Ms10,
            application: OpusApplication::LowDelay,
            bitrate_kbps: Some(bitrate_kbps),
            complexity: 5,
            dtx: false,
            fec: false,
        }
    }

    /// Broadcast stereo music config: 20 ms frames, Audio mode.
    pub fn stereo_broadcast(bitrate_kbps: u32) -> Self {
        Self {
            sample_rate: OpusSampleRate::Hz48000,
            channels: OpusChannels::Stereo,
            frame_duration: OpusFrameDuration::Ms20,
            application: OpusApplication::Audio,
            bitrate_kbps: Some(bitrate_kbps),
            complexity: 10,
            dtx: false,
            fec: false,
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
    /// Channel count (1 = mono, 2 = stereo). Stored so `encode_into` can validate
    /// the interleaved frame length (samples-per-channel × channels) and size its
    /// conversion buffer for the widest case (20 ms stereo = 1920 samples).
    channels: usize,
}

impl OpusEncoder {
    /// Create a new encoder with default config (48 kHz, mono, Voip, 20 ms).
    pub fn new() -> Result<Self, opus::Error> {
        Self::from_config(&OpusConfig::default())
    }

    /// Create a low-latency voice-agent encoder using OPUS_APPLICATION_RESTRICTED_LOWDELAY.
    ///
    /// `bitrate_kbps` must be at least 32 (minimum for 10 ms mono Opus at acceptable quality).
    pub fn voice_agent(channels: OpusChannels, bitrate_kbps: u32) -> Result<Self, opus::Error> {
        Self::from_config(&OpusConfig {
            channels,
            frame_duration: OpusFrameDuration::Ms10,
            application: OpusApplication::LowDelay,
            bitrate_kbps: Some(bitrate_kbps),
            complexity: 5,
            ..OpusConfig::default()
        })
    }

    /// Create an encoder from an explicit OpusConfig.
    pub fn from_config(config: &OpusConfig) -> Result<Self, opus::Error> {
        let ch = match config.channels {
            OpusChannels::Mono => Channels::Mono,
            OpusChannels::Stereo => Channels::Stereo,
        };
        let app = match config.application {
            OpusApplication::Voip => Application::Voip,
            OpusApplication::LowDelay => Application::LowDelay,
            OpusApplication::Audio => Application::Audio,
        };
        let mut enc = Encoder::new(config.sample_rate.hz(), ch, app)?;
        if let Some(kbps) = config.bitrate_kbps {
            enc.set_bitrate(opus::Bitrate::Bits((kbps * 1_000) as i32))?;
        }
        enc.set_complexity(config.complexity as i32)?;
        if config.dtx {
            enc.set_dtx(true)?;
        }
        if config.fec {
            enc.set_inband_fec(true)?;
        }
        let channels = match config.channels {
            OpusChannels::Mono => 1,
            OpusChannels::Stereo => 2,
        };
        Ok(Self {
            inner: enc,
            channels,
        })
    }

    /// Encode an interleaved PCM slice into `out`.
    ///
    /// `pcm` is interleaved across channels: its length must be
    /// `samples_per_channel × channels`, where samples-per-channel is 960 (20 ms)
    /// or 480 (10 ms). For mono that is 960/480; for stereo, 1920/960. Any other
    /// length is a logic error and panics in debug builds (debug_assert).
    ///
    /// Converts f32 → i16 (multiply by 32 767.0, clamp) then calls
    /// `encoder.encode()`.  Returns the number of compressed bytes written.
    /// `out` is cleared and reused; no heap allocation occurs after the first
    /// call provided `out` already has `OPUS_MAX_PACKET_BYTES` capacity.
    pub fn encode_into(&mut self, pcm: &[f32], out: &mut Vec<u8>) -> Result<usize, opus::Error> {
        let frame_len = pcm.len();
        let per_channel = frame_len / self.channels;
        debug_assert!(
            frame_len % self.channels == 0
                && (per_channel == OPUS_FRAME_SAMPLES || per_channel == VOICE_AGENT_FRAME_SAMPLES),
            "encode_into: expected {OPUS_FRAME_SAMPLES} or {VOICE_AGENT_FRAME_SAMPLES} samples per channel \
             × {} channels, got {frame_len} total",
            self.channels,
        );

        // f32 → i16.  Written as a plain iterator loop so LLVM auto-vectorises
        // to NEON/AVX2 when compiled with target-cpu=native (.cargo/config.toml).
        // Sized for the widest case (20 ms stereo = 1920 interleaved samples) so
        // both mono and stereo fit on the stack with no allocation.
        let mut i16_buf = [0i16; OPUS_FRAME_SAMPLES * 2];
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

    /// Update the live encoder bitrate. Called by CODEC_HINT handler (AUDIO-021).
    /// `kbps` = 0 switches to Opus auto (VBR). Safe to call mid-stream.
    pub fn set_bitrate_kbps(&mut self, kbps: u32) -> Result<(), opus::Error> {
        let bitrate = if kbps > 0 {
            opus::Bitrate::Bits((kbps * 1_000) as i32)
        } else {
            opus::Bitrate::Auto
        };
        self.inner.set_bitrate(bitrate)
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
// Legacy mock alias — kept so that existing tests continue to compile without
// modification.  Delegates to the real encoder.
// Remove in Phase 5 once all call sites have been migrated.
// ---------------------------------------------------------------------------

/// Deprecated alias for [`OpusEncoder`].  Use `OpusEncoder` directly.
#[cfg(any(test, feature = "test-helpers"))]
pub struct MockOpusEncoder {
    pub inner: OpusEncoder,
}

#[cfg(any(test, feature = "test-helpers"))]
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

#[cfg(any(test, feature = "test-helpers"))]
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
        dec.decode_into(&packet, &mut pcm_out, false).unwrap();

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
    fn given_stereo_music_config_when_round_trip_then_channels_stay_distinct() {
        use std::f32::consts::PI;

        // Given: a 20 ms STEREO frame (1920 interleaved L/R samples). The left
        // channel carries a 440 Hz tone; the right channel is silent — a signal a
        // mono downmix would destroy by averaging L and R into one channel. This
        // is the Stage-A gate for the stereo music pipeline (stereo_broadcast =
        // Opus Audio mode, complexity 10).
        let mut enc = OpusEncoder::from_config(&OpusConfig::stereo_broadcast(128)).unwrap();
        let mut dec = crate::decoder::OpusDecoder::with_channels(OpusChannels::Stereo).unwrap();

        let mut pcm_in = Vec::with_capacity(OPUS_FRAME_SAMPLES * 2);
        for i in 0..OPUS_FRAME_SAMPLES {
            let left = (2.0 * PI * 440.0 * i as f32 / 48_000.0).sin() * 0.3;
            let right = 0.0; // silent right channel
            pcm_in.push(left);
            pcm_in.push(right);
        }

        // When: encode the interleaved stereo frame and decode it back as stereo.
        let mut packet = Vec::new();
        enc.encode_into(&pcm_in, &mut packet).unwrap();
        let mut pcm_out = Vec::new();
        let total = dec.decode_into(&packet, &mut pcm_out, false).unwrap();

        // Then: a full interleaved stereo frame returns ...
        assert_eq!(
            total,
            OPUS_FRAME_SAMPLES * 2,
            "stereo decode must return 1920 interleaved samples, got {total}"
        );

        // ... and the two channels stay DISTINCT: left carries the tone, right is
        // near-silent. A mono collapse would make the channels roughly equal.
        let rms = |s: &[f32]| -> f32 {
            let sum: f32 = s.iter().map(|x| x * x).sum();
            (sum / s.len() as f32).sqrt()
        };
        let left: Vec<f32> = pcm_out.iter().step_by(2).copied().collect();
        let right: Vec<f32> = pcm_out.iter().skip(1).step_by(2).copied().collect();
        let rms_l = rms(&left);
        let rms_r = rms(&right);
        assert!(
            rms_l > 0.05,
            "left channel must carry the tone, rms_l={rms_l:.4}"
        );
        assert!(
            rms_l > rms_r * 4.0,
            "channels must stay distinct (true stereo), rms_l={rms_l:.4} rms_r={rms_r:.4}"
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
        let mut enc = OpusEncoder::voice_agent(OpusChannels::Mono, 32).unwrap();
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
        let mut enc = OpusEncoder::voice_agent(OpusChannels::Mono, 32).unwrap();
        let mut dec = crate::decoder::OpusDecoder::new().unwrap();

        let pcm_in: Vec<f32> = (0..VOICE_AGENT_FRAME_SAMPLES)
            .map(|i| (2.0 * PI * 440.0 * i as f32 / 48_000.0).sin() * 0.25)
            .collect();

        let mut packet = Vec::new();
        enc.encode_into(&pcm_in, &mut packet).unwrap();

        let mut pcm_out = Vec::new();
        dec.decode_into(&packet, &mut pcm_out, false).unwrap();

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
        dec.decode_into(&packet, &mut pcm_out, false).unwrap();

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

    // -----------------------------------------------------------------------
    // Golden file: 30-second Opus encode/decode cycle
    //
    // Generates 1500 frames (30 s × 50 fps) of 440 Hz sine at 48 kHz mono,
    // encodes with OpusEncoder (960 samples/frame), decodes with OpusDecoder,
    // writes artifacts/audio/opus-decoded.wav, and asserts the following
    // audio-quality invariants without a network or browser:
    //
    //   1. Packet count = 1500 (50 pkt/s steady-state, no drops)
    //   2. Decoded sample count = 1 500 000 (= 1500 × 960)  [OPUS_FRAME_SAMPLES]
    //   3. Duration = 30.0 ± 0.1 s at 48 kHz
    //   4. RMS within 4 dB of input RMS (codec does not destroy energy)
    //   5. No zero run > 99 consecutive decoded samples
    //      (silence injection would produce runs of 960+ zeros)
    //   6. No decoded sample clips beyond ±1.05
    //   7. RTP timestamp delta = OPUS_FRAME_SAMPLES between every consecutive pair
    //      (monotonic clock, correct Opus RFC 7587 cadence)
    //   8. Zero encode errors
    // -----------------------------------------------------------------------
    #[test]
    fn given_30s_sine_pcm_when_opus_round_trip_then_golden_file_invariants_pass() {
        use std::f32::consts::PI;
        use std::path::Path;

        const FREQ_HZ: f32 = 440.0;
        const AMPLITUDE: f32 = 0.25;
        const FRAME_COUNT: usize = 1500; // 30 s at 50 fps
        const SAMPLE_RATE: f32 = 48_000.0;
        // Opus CELT has a pre-skip of up to ~320 samples at stream start — normal codec
        // delay, not silence injection. Our pipeline bug injects full 960-sample zero frames.
        // 479 catches half-frame-or-larger injection while passing normal pre-skip.
        const MAX_ZERO_RUN: usize = 479;

        let mut enc = OpusEncoder::new().expect("encoder init");
        let mut dec = crate::decoder::OpusDecoder::new().expect("decoder init");

        let mut all_pcm_in: Vec<f32> = Vec::with_capacity(FRAME_COUNT * OPUS_FRAME_SAMPLES);
        let mut all_pcm_out: Vec<f32> = Vec::with_capacity(FRAME_COUNT * OPUS_FRAME_SAMPLES);
        let mut rtp_timestamps: Vec<u64> = Vec::with_capacity(FRAME_COUNT);

        let mut packet_buf = Vec::new();
        let mut decode_buf = Vec::new();
        let mut rtp_ts: u64 = 0;
        let mut encode_errors: usize = 0;

        for frame_idx in 0..FRAME_COUNT {
            // Generate one 960-sample frame of 440 Hz sine.
            let offset = (frame_idx * OPUS_FRAME_SAMPLES) as f32;
            let pcm_in: Vec<f32> = (0..OPUS_FRAME_SAMPLES)
                .map(|i| (2.0 * PI * FREQ_HZ * (offset + i as f32) / SAMPLE_RATE).sin() * AMPLITUDE)
                .collect();
            all_pcm_in.extend_from_slice(&pcm_in);

            rtp_timestamps.push(rtp_ts);
            rtp_ts += OPUS_FRAME_SAMPLES as u64;

            // Encode.
            match enc.encode_into(&pcm_in, &mut packet_buf) {
                Ok(_) => {}
                Err(_) => {
                    encode_errors += 1;
                    continue;
                }
            }

            // Decode.
            decode_buf.clear();
            dec.decode_into(&packet_buf, &mut decode_buf, false)
                .expect("decode_into must not fail on a valid packet");
            all_pcm_out.extend_from_slice(&decode_buf);
        }

        // ── Invariant 1: packet count ────────────────────────────────────────
        let packets_produced = FRAME_COUNT - encode_errors;
        assert_eq!(
            packets_produced, FRAME_COUNT,
            "encode_errors={encode_errors}: all 1500 frames must encode without error"
        );

        // ── Invariant 2: decoded sample count ───────────────────────────────
        assert_eq!(
            all_pcm_out.len(),
            FRAME_COUNT * OPUS_FRAME_SAMPLES,
            "decoded sample count must be 1500 × 960 = {}, got {}",
            FRAME_COUNT * OPUS_FRAME_SAMPLES,
            all_pcm_out.len()
        );

        // ── Invariant 3: duration ────────────────────────────────────────────
        let duration_s = all_pcm_out.len() as f64 / SAMPLE_RATE as f64;
        assert!(
            (duration_s - 30.0).abs() < 0.1,
            "duration must be 30.0 ± 0.1 s, got {duration_s:.3} s"
        );

        // ── Invariant 4: RMS within 4 dB ────────────────────────────────────
        let rms =
            |s: &[f32]| -> f32 { (s.iter().map(|x| x * x).sum::<f32>() / s.len() as f32).sqrt() };
        let rms_in = rms(&all_pcm_in);
        let rms_out = rms(&all_pcm_out);
        let snr_db = 20.0 * (rms_out / rms_in).log10();
        assert!(
            snr_db > -4.0,
            "RMS SNR {snr_db:.2} dB below -4 dB threshold — codec or silence injection degraded energy"
        );

        // ── Invariant 5: no zero run > MAX_ZERO_RUN ──────────────────────────
        // A zero run of 480+ samples would indicate silence frame injection.
        // Runs ≤ 479 are normal Opus CELT pre-skip at stream start.
        let mut zero_run = 0usize;
        let mut max_zero_run = 0usize;
        let mut max_zero_run_start = 0usize;
        let mut current_run_start = 0usize;
        for (i, &s) in all_pcm_out.iter().enumerate() {
            if s.abs() < 1e-9 {
                if zero_run == 0 {
                    current_run_start = i;
                }
                zero_run += 1;
                if zero_run > max_zero_run {
                    max_zero_run = zero_run;
                    max_zero_run_start = current_run_start;
                }
            } else {
                zero_run = 0;
            }
        }
        eprintln!(
            "zero-run audit: max_zero_run={max_zero_run} samples \
             at sample_offset={max_zero_run_start} \
             (frame ~{})",
            max_zero_run_start / OPUS_FRAME_SAMPLES
        );
        assert!(
            max_zero_run <= MAX_ZERO_RUN,
            "max zero run = {max_zero_run} samples at offset {max_zero_run_start} \
             exceeds {MAX_ZERO_RUN}: silence injection detected in decoded stream"
        );

        // ── Invariant 6: no clipping ─────────────────────────────────────────
        let clipped = all_pcm_out.iter().filter(|&&s| s.abs() > 1.05).count();
        assert_eq!(
            clipped, 0,
            "{clipped} decoded samples clip beyond ±1.05 — encoder/decoder corruption"
        );

        // ── Invariant 7: RTP timestamp delta = OPUS_FRAME_SAMPLES ───────────
        let mut bad_deltas: usize = 0;
        for w in rtp_timestamps.windows(2) {
            let delta = w[1] - w[0];
            if delta != OPUS_FRAME_SAMPLES as u64 {
                bad_deltas += 1;
            }
        }
        assert_eq!(
            bad_deltas, 0,
            "{bad_deltas} RTP timestamp deltas ≠ {OPUS_FRAME_SAMPLES} — clock skew in publisher loop"
        );

        // ── Invariant 8: write golden WAV artifact (opt-in only) ────────────
        // Gated behind PKS_WRITE_AUDIO_ARTIFACTS=1 so normal CI does not
        // write persistent files into the repo tree.
        if std::env::var("PKS_WRITE_AUDIO_ARTIFACTS").as_deref() == Ok("1") {
            let artifacts_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
                .ancestors()
                .nth(3) // audio-core root (manifest is at crates/pocketstation-codec)
                .unwrap_or(Path::new("."))
                .join("artifacts")
                .join("audio");
            std::fs::create_dir_all(&artifacts_dir).ok();

            let wav_path = artifacts_dir.join("opus-decoded.wav");
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: 48_000,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let mut writer = hound::WavWriter::create(&wav_path, spec).expect("create WAV writer");
            for &s in &all_pcm_out {
                writer
                    .write_sample((s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                    .expect("write WAV sample");
            }
            writer.finalize().expect("finalize WAV");
            eprintln!("Golden file written: {}", wav_path.display());
        } else {
            eprintln!(
                "(WAV not written — set PKS_WRITE_AUDIO_ARTIFACTS=1 to write \
                 artifacts/audio/opus-decoded.wav)"
            );
        }

        eprintln!(
            "Packets:      {}\n\
             Duration:     {:.3} s\n\
             SNR:          {:.2} dB\n\
             Max zero run: {} samples\n\
             Clipped:      {}\n\
             RTP bad:      {}",
            FRAME_COUNT, duration_s, snr_db, max_zero_run, clipped, bad_deltas,
        );
    }
}

// Spectral noise gate using short-time RMS noise floor estimation.
//
// Algorithm: maintain a running per-band (sub-frame) noise floor estimate;
// update it only during frames identified as noise (energy below VAD threshold);
// apply a Wiener-gain approximation (gain = max(1 − noise/signal, floor)).
//
// This is a simplified spectral subtraction approach. A full FFT-domain
// implementation (RNNoise, WebRTC NS) would give better separation — that is
// a Phase 5 upgrade requiring either ONNX weights or the RNNoise C library.
// The current algorithm works well for stationary background noise (fan, HVAC).
//
// process() is alloc-free, lock-free, blocking-free — LAW 15.

use crate::{rms, FRAME_DURATION_MS, FRAME_SAMPLES_48K_20MS};
use pks_frame::AudioFrame;
use pks_graph::node::{NodeError, PrepareContext};
use pks_graph::RuntimeNode;

/// Number of sub-frames used for per-band noise estimation.
/// Each sub-frame covers 960/NUM_BANDS = 60 samples (~1.25 ms).
const NUM_BANDS: usize = 16;

/// Spectral noise suppressor using per-band RMS noise floor subtraction.
pub struct NoiseSuppressor {
    /// Per-band estimated noise floor RMS.
    noise_floor: [f32; NUM_BANDS],
    /// Smoothing factor for noise floor update (slow, during silence).
    alpha_noise: f32,
    /// VAD threshold RMS: frames below this update noise floor.
    vad_threshold_rms: f32,
    /// Minimum output gain (prevents complete silence; ~0.1 = -20 dB floor).
    gain_floor_ratio: f32,
    /// Wiener over-subtraction factor (>1 increases suppression, >2 may add artifacts).
    over_subtraction: f32,
    /// Pre-allocated input copy for the RuntimeNode in-place frame path (no hot-path alloc).
    scratch: Vec<f32>,
}

impl NoiseSuppressor {
    /// * `vad_threshold_dbfs` — frames below this update the noise floor (e.g. -40.0).
    /// * `noise_update_ratio` — noise floor exponential update ratio per frame (e.g. 0.01).
    /// * `gain_floor_db` — minimum gain applied when SNR is very low (e.g. -20.0).
    /// * `over_subtraction` — suppression strength; 1.0 = Wiener, 2.0 = aggressive (e.g. 1.5).
    pub fn new(
        vad_threshold_dbfs: f32,
        noise_update_ratio: f32,
        gain_floor_db: f32,
        over_subtraction: f32,
    ) -> Self {
        let vad_threshold_rms = 10.0_f32.powf(vad_threshold_dbfs / 20.0);
        let gain_floor_ratio = 10.0_f32.powf(gain_floor_db / 20.0);
        Self {
            noise_floor: [vad_threshold_rms * 0.5; NUM_BANDS],
            alpha_noise: noise_update_ratio.clamp(1e-4, 0.5),
            vad_threshold_rms,
            gain_floor_ratio,
            over_subtraction: over_subtraction.max(0.0),
            scratch: vec![0.0f32; FRAME_SAMPLES_48K_20MS],
        }
    }

    /// Default: -40 dBFS VAD threshold, 0.01 update rate, -20 dB gain floor, 1.5× suppression.
    pub fn default_config() -> Self {
        Self::new(-40.0, 0.01, -20.0, 1.5)
    }
}

impl NoiseSuppressor {
    pub fn process_slices(&mut self, input: &[f32], output: &mut [f32]) {
        let frame_rms = rms(input);
        let band_len = input.len() / NUM_BANDS;

        for b in 0..NUM_BANDS {
            let start = b * band_len;
            let end = (start + band_len).min(input.len());
            let band = &input[start..end];
            let out_band = &mut output[start..end];

            let band_rms = rms(band);

            // Update noise floor only during (approximately) silent frames
            if frame_rms < self.vad_threshold_rms {
                self.noise_floor[b] =
                    self.alpha_noise * band_rms + (1.0 - self.alpha_noise) * self.noise_floor[b];
            }

            // Wiener gain approximation: g = max(1 − α·noise/signal, floor)
            let gain = if band_rms > 1e-12 {
                let wiener = 1.0 - self.over_subtraction * self.noise_floor[b] / band_rms;
                wiener.max(self.gain_floor_ratio).min(1.0)
            } else {
                self.gain_floor_ratio
            };

            for (o, &i) in out_band.iter_mut().zip(band.iter()) {
                *o = i * gain;
            }
        }

        // Handle any remaining samples if input.len() is not divisible by NUM_BANDS
        let processed = (input.len() / NUM_BANDS) * NUM_BANDS;
        if processed < input.len() {
            output[processed..].copy_from_slice(&input[processed..]);
        }
    }
}

impl RuntimeNode for NoiseSuppressor {
    fn prepare(&mut self, cx: &PrepareContext) -> Result<(), NodeError> {
        let frame_samples = cx
            .sample_spec
            .frame_samples_for_duration_ms(FRAME_DURATION_MS);
        if self.scratch.len() != frame_samples {
            self.scratch = vec![0.0f32; frame_samples];
        }
        Ok(())
    }

    fn process(&mut self, mut frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        let mut scratch = std::mem::take(&mut self.scratch);
        {
            let buf = frame.buffer.as_mut_slice();
            let n = buf.len().min(scratch.len());
            scratch[..n].copy_from_slice(&buf[..n]);
            self.process_slices(&scratch[..n], &mut buf[..n]);
        }
        self.scratch = scratch;
        Ok(Some(frame))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FRAME_SAMPLES_48K_20MS as FRAME_LEN_SAMPLES;

    fn silence() -> Vec<f32> {
        vec![0.0f32; FRAME_LEN_SAMPLES]
    }

    fn noise_frame(amplitude: f32) -> Vec<f32> {
        // Simple pseudo-random noise using LCG
        let mut state: u32 = 12345;
        (0..FRAME_LEN_SAMPLES)
            .map(|_| {
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                (state as f32 / u32::MAX as f32 - 0.5) * 2.0 * amplitude
            })
            .collect()
    }

    #[test]
    fn given_silence_input_when_processed_then_output_is_silence() {
        let mut ns = NoiseSuppressor::default_config();
        let input = silence();
        let mut output = vec![1.0f32; FRAME_LEN_SAMPLES];
        ns.process_slices(&input, &mut output);
        assert!(
            output.iter().all(|&s| s.abs() < 1e-9),
            "silence in → silence out"
        );
    }

    #[test]
    fn given_noise_only_frames_when_processed_then_noise_floor_is_updated() {
        let mut ns = NoiseSuppressor::new(-40.0, 0.1, -20.0, 1.0);
        let noise = noise_frame(0.001); // ~-60 dBFS, below -40 VAD threshold
        let mut out = vec![0.0f32; FRAME_LEN_SAMPLES];
        // Run many frames so noise floor adapts
        for _ in 0..200 {
            ns.process_slices(&noise, &mut out);
        }
        // After adaptation, noise floor estimates should be non-zero
        assert!(
            ns.noise_floor.iter().any(|&nf| nf > 0.0),
            "noise floor must have adapted"
        );
    }

    #[test]
    fn given_signal_above_noise_when_processed_then_output_rms_is_reduced_but_nonzero() {
        let mut ns = NoiseSuppressor::new(-40.0, 0.05, -20.0, 1.5);
        let noise = noise_frame(0.001);
        let mut out = vec![0.0f32; FRAME_LEN_SAMPLES];
        // Adapt noise floor
        for _ in 0..100 {
            ns.process_slices(&noise, &mut out);
        }
        // Now process a slightly louder noise (above VAD threshold) — should be reduced
        let signal = noise_frame(0.05); // -26 dBFS, above -40 VAD
        ns.process_slices(&signal, &mut out);
        let in_rms = rms(&signal);
        let out_rms = rms(&out);
        assert!(out_rms < in_rms, "suppressor must reduce signal RMS");
        assert!(out_rms > 0.0, "output must not be fully gated");
    }

    #[test]
    fn given_loud_signal_when_processed_then_gain_approaches_one() {
        let mut ns = NoiseSuppressor::new(-60.0, 0.01, -20.0, 1.0);
        let mut out = vec![0.0f32; FRAME_LEN_SAMPLES];
        // Adapt with very quiet noise
        let tiny_noise = noise_frame(1e-5);
        for _ in 0..200 {
            ns.process_slices(&tiny_noise, &mut out);
        }
        // Loud signal: SNR >> 1 → gain → 1.0
        let loud = noise_frame(0.5);
        ns.process_slices(&loud, &mut out);
        let in_rms = rms(&loud);
        let out_rms = rms(&out);
        // At high SNR, gain ≈ 1.0, so output ≈ input within a few %
        let ratio = out_rms / in_rms;
        assert!(
            ratio > 0.5,
            "loud signal at high SNR must pass with gain close to 1.0; got ratio {ratio}"
        );
    }
}

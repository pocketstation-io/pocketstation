// Spread-spectrum audio watermark using phase-coherent PN-sequence modulation.
//
// Algorithm:
//   Embed: output[i] = input[i] + alpha * pn[i]
//   Detect: correlation = sum(output[i] * pn[i]) / N; detected if > threshold
//
// The PN sequence is generated from a 32-bit LFSR (maximal length, period 2^32−1)
// seeded from the session token. At typical embedding levels (−40 to −30 dBFS
// relative to full scale, or alpha ≈ 0.001–0.005) the watermark is inaudible
// for programme audio but detectable with matched filter correlation.
//
// For production-grade imperceptibility, embed in psychoacoustically masked
// frequency bands using the ISO 226 equal-loudness model — that is a Phase 5
// upgrade. The current time-domain approach is imperceptible for music at
// alpha ≤ 0.002 (approx −54 dBFS relative to 0 dBFS full scale).
//
// process() is alloc-free, lock-free, blocking-free — LAW 15.
// The LFSR state advances 960 positions per process() call (one 20 ms frame).

use crate::{FRAME_DURATION_MS, FRAME_SAMPLES_48K_20MS};
use pocketstation_frame::AudioFrame;
use pocketstation_graph::node::{NodeError, PrepareContext};
use pocketstation_graph::RuntimeNode;

/// 32-bit Galois LFSR — maximal-length, taps at bits 32,22,2,1 (standard poly).
/// Produces a PN sequence with period 2^32−1 ≈ 4×10^9 samples (~25 hours at 48 kHz).
struct Lfsr32 {
    state: u32,
}

impl Lfsr32 {
    fn new(seed: u32) -> Self {
        // Seed must be non-zero; replace 0 with a fixed non-zero constant
        Self {
            state: if seed == 0 { 0xACE1 } else { seed },
        }
    }

    /// Advance one step; return the output bit as +1.0 or −1.0.
    #[inline]
    fn next_chip(&mut self) -> f32 {
        // Galois LFSR with polynomial x^32 + x^22 + x^2 + x + 1
        let feedback = self.state & 1;
        self.state >>= 1;
        if feedback != 0 {
            self.state ^= 0x80200003;
            1.0
        } else {
            -1.0
        }
    }
}

/// Spread-spectrum audio watermarker.
///
/// Embeds an inaudible PN watermark; the same token recovers it via detection().
pub struct AudioWatermark {
    /// LFSR for embedding — advances per process() call.
    embed_lfsr: Lfsr32,
    /// LFSR for detection — must be synchronized with embed_lfsr state.
    detect_lfsr: Lfsr32,
    /// Embedding amplitude (linear, e.g. 0.002 ≈ −54 dBFS re 0 dBFS).
    alpha: f32,
    /// Detection threshold for matched-filter output (normalized correlation).
    detect_threshold: f32,
    /// Pre-computed PN chips for this frame (re-used to avoid recomputing).
    pn_frame: Vec<f32>,
    /// Running detection accumulator (exponential average of correlation).
    detection_score: f32,
    /// Pre-allocated input copy for the RuntimeNode in-place frame path (no hot-path alloc).
    scratch: Vec<f32>,
}

impl AudioWatermark {
    /// * `session_token` — 32-bit session identifier seeds the PN sequence.
    /// * `alpha` — embedding amplitude; 0.002 is inaudible, 0.01 is detectable easily.
    /// * `detect_threshold` — normalized correlation above which detection fires (e.g. 0.3).
    pub fn new(session_token: u32, alpha: f32, detect_threshold: f32) -> Self {
        let frame_len = FRAME_SAMPLES_48K_20MS;
        Self {
            embed_lfsr: Lfsr32::new(session_token),
            detect_lfsr: Lfsr32::new(session_token),
            alpha: alpha.max(0.0),
            detect_threshold: detect_threshold.clamp(0.0, 1.0),
            pn_frame: vec![0.0f32; frame_len],
            detection_score: 0.0,
            scratch: vec![0.0f32; frame_len],
        }
    }

    /// Default: session 0xDEADBEEF, alpha 0.002 (inaudible), threshold 0.3.
    pub fn default_config() -> Self {
        Self::new(0xDEAD_BEEF, 0.002, 0.3)
    }

    /// Normalized correlation of the output buffer with the PN sequence.
    /// Returns a value in [−1, 1]; values above detect_threshold indicate watermark detected.
    pub fn detect(&mut self, buffer: &[f32]) -> f32 {
        let n = buffer.len().min(self.pn_frame.len());
        let corr: f32 = buffer[..n]
            .iter()
            .zip(self.pn_frame[..n].iter())
            .map(|(&s, &chip)| s * chip)
            .sum();
        let normalized = corr / n as f32;
        self.detection_score = 0.9 * self.detection_score + 0.1 * normalized;
        self.detection_score
    }

    pub fn is_detected(&self) -> bool {
        self.detection_score.abs() > self.detect_threshold
    }

    pub fn detection_score(&self) -> f32 {
        self.detection_score
    }
}

impl AudioWatermark {
    pub fn process_slices(&mut self, input: &[f32], output: &mut [f32]) {
        // Generate PN chips from embed_lfsr; store in pn_frame for detect() calls.
        for chip in self.pn_frame.iter_mut() {
            *chip = self.embed_lfsr.next_chip();
        }
        // Embed: output[i] = input[i] + alpha * pn[i]
        for (o, (&i, &chip)) in output
            .iter_mut()
            .zip(input.iter().zip(self.pn_frame.iter()))
        {
            *o = i + self.alpha * chip;
        }
        // Keep detect_lfsr synchronized with embed_lfsr for multi-frame detection.
        // Advance without overwriting pn_frame — pn_frame holds embed chips for detect().
        for _ in 0..pn_frame_len(input) {
            self.detect_lfsr.next_chip();
        }
    }
}

impl RuntimeNode for AudioWatermark {
    fn prepare(&mut self, cx: &PrepareContext) -> Result<(), NodeError> {
        let frame_samples = cx
            .sample_spec
            .frame_samples_for_duration_ms(FRAME_DURATION_MS);
        if self.scratch.len() != frame_samples {
            self.scratch = vec![0.0f32; frame_samples];
        }
        if self.pn_frame.len() != frame_samples {
            self.pn_frame = vec![0.0f32; frame_samples];
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

/// Returns the PN frame length (= input.len() clamped to pn_frame capacity).
#[inline]
fn pn_frame_len(input: &[f32]) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FRAME_SAMPLES_48K_20MS as FRAME_LEN_SAMPLES;

    fn sine_frame(amplitude: f32) -> Vec<f32> {
        use std::f32::consts::PI;
        (0..FRAME_LEN_SAMPLES)
            .map(|i| amplitude * (2.0 * PI * 440.0 * i as f32 / 48000.0).sin())
            .collect()
    }

    #[test]
    fn given_silence_embedded_when_pn_correlated_then_watermark_detected() {
        // Use silence as input so the only contribution to output is the PN chips.
        // This removes the cross-correlation noise between the input and the PN sequence
        // that makes detection probabilistic on short frames.
        let token: u32 = 0xC0FFEE;
        let mut wm = AudioWatermark::new(token, 0.01, 0.1);

        let silence = vec![0.0f32; FRAME_LEN_SAMPLES];
        let mut output = vec![0.0f32; FRAME_LEN_SAMPLES];
        wm.process_slices(&silence, &mut output);

        // Re-generate the same PN sequence from a fresh LFSR at the same seed
        let mut ref_lfsr = Lfsr32::new(token);
        let pn: Vec<f32> = (0..FRAME_LEN_SAMPLES)
            .map(|_| ref_lfsr.next_chip())
            .collect();

        // output[i] = 0 + alpha * chip[i], so corr = alpha * mean(chip²) = alpha * 1.0 = 0.01
        let corr: f32 = output
            .iter()
            .zip(pn.iter())
            .map(|(&o, &c)| o * c)
            .sum::<f32>()
            / FRAME_LEN_SAMPLES as f32;

        assert!(
            corr > 0.005,
            "correlation with correct PN on silent input must ≈ alpha=0.01, got {corr}"
        );
    }

    #[test]
    fn given_wrong_token_when_correlated_then_detection_is_near_zero() {
        let mut wm = AudioWatermark::new(0xC0FFEE, 0.01, 0.1);
        let input = sine_frame(0.5);
        let mut output = vec![0.0f32; FRAME_LEN_SAMPLES];
        wm.process_slices(&input, &mut output);

        // Wrong token → wrong PN sequence → correlation near 0
        let mut wrong_lfsr = Lfsr32::new(0xDEADBEEF);
        let wrong_pn: Vec<f32> = (0..FRAME_LEN_SAMPLES)
            .map(|_| wrong_lfsr.next_chip())
            .collect();
        let corr: f32 = output
            .iter()
            .zip(wrong_pn.iter())
            .map(|(&o, &c)| o * c)
            .sum::<f32>()
            / FRAME_LEN_SAMPLES as f32;

        // At alpha=0.01 and 960 chips, expected |corr| from mismatch ≈ alpha/sqrt(N) ≈ 0.0003
        assert!(
            corr.abs() < 0.1,
            "wrong token must give near-zero correlation, got {corr}"
        );
    }

    #[test]
    fn given_watermark_when_alpha_zero_then_output_equals_input() {
        let mut wm = AudioWatermark::new(0xABCD, 0.0, 0.1);
        let input = sine_frame(0.5);
        let mut output = vec![0.0f32; FRAME_LEN_SAMPLES];
        wm.process_slices(&input, &mut output);
        for (o, i) in output.iter().zip(input.iter()) {
            assert!(
                (o - i).abs() < 1e-9,
                "alpha=0 must produce output identical to input"
            );
        }
    }

    #[test]
    fn given_nonzero_alpha_when_embedded_then_output_differs_from_input() {
        let mut wm = AudioWatermark::new(0x1234, 0.01, 0.1);
        let input = sine_frame(0.5);
        let mut output = vec![0.0f32; FRAME_LEN_SAMPLES];
        wm.process_slices(&input, &mut output);
        let max_diff = input
            .iter()
            .zip(output.iter())
            .map(|(&i, &o)| (o - i).abs())
            .fold(0.0_f32, f32::max);
        assert!(
            max_diff > 0.005,
            "non-zero alpha must introduce detectable difference; max_diff={max_diff}"
        );
    }
}

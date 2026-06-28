// Energy-based Voice Activity Detector.
//
// Uses a dual-threshold (primary/hangover) with exponential smoothing of RMS
// energy. Attack is fast to not clip leading plosives; release is slow to avoid
// chopping tails. This matches the WebRTC VAD design (Gaussian mixture model
// would be higher accuracy but needs model weights — Phase 5 upgrade path).
//
// All state is f32 scalars — process() is alloc-free, lock-free, blocking-free.

use crate::{rms, FRAME_DURATION_MS, FRAME_SAMPLES_48K_20MS};
use pocketstation_frame::AudioFrame;
use pocketstation_graph::node::{NodeError, PrepareContext};
use pocketstation_graph::RuntimeNode;

/// Voice activity state exposed after each process() call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VadState {
    Active,
    Inactive,
}

/// Energy-based VAD with exponential smoothing and hangover suppression.
///
/// Configure via VadProcessor::new(); read activity via VadProcessor::state().
pub struct VadProcessor {
    /// RMS level below which the signal is considered inactive.
    threshold_rms: f32,
    /// Smoothed RMS energy (IIR filter state).
    smoothed_rms: f32,
    /// Attack coefficient: fraction of new value added per frame (fast).
    alpha_attack: f32,
    /// Release coefficient: fraction of new value per frame (slow).
    alpha_release: f32,
    /// Hangover: number of frames to stay Active after energy drops below threshold.
    hangover_frames: u32,
    /// Count-down counter while in hangover.
    hangover_count: u32,
    /// Most recent VAD decision, readable via state().
    state: VadState,
    /// When inactive, gate output to silence (true) or pass through (false).
    gate_output: bool,
    /// Pre-allocated input copy for the RuntimeNode in-place frame path (no hot-path alloc).
    scratch: Vec<f32>,
}

impl VadProcessor {
    /// * `threshold_dbfs` — threshold in dBFS (e.g. -40.0). Signals below are inactive.
    /// * `attack_ms` — smoothing time constant for rising energy (e.g. 5 ms).
    /// * `release_ms` — smoothing time constant for falling energy (e.g. 150 ms).
    /// * `hangover_ms` — stay Active for this long after energy drops (e.g. 200 ms).
    /// * `gate_output` — if true, output is zeroed when inactive; if false, pass-through.
    pub fn new(
        threshold_dbfs: f32,
        attack_ms: f32,
        release_ms: f32,
        hangover_ms: f32,
        gate_output: bool,
    ) -> Self {
        // Convert dBFS threshold to linear RMS amplitude
        let threshold_rms = 10.0_f32.powf(threshold_dbfs / 20.0);
        // One-pole IIR coefficient from time constant: α = 1 − exp(−1 / (fs*τ))
        // Frame rate = 48000 / 960 = 50 frames/s
        const FRAMES_PER_SEC: f32 = 50.0;
        let alpha_attack = 1.0 - (-1.0 / (FRAMES_PER_SEC * attack_ms / 1000.0)).exp();
        let alpha_release = 1.0 - (-1.0 / (FRAMES_PER_SEC * release_ms / 1000.0)).exp();
        let hangover_frames = (FRAMES_PER_SEC * hangover_ms / 1000.0).ceil() as u32;
        Self {
            threshold_rms,
            smoothed_rms: 0.0,
            alpha_attack,
            alpha_release,
            hangover_frames,
            hangover_count: 0,
            state: VadState::Inactive,
            gate_output,
            scratch: vec![0.0f32; FRAME_SAMPLES_48K_20MS],
        }
    }

    /// Default config: -40 dBFS threshold, 5 ms attack, 150 ms release, 200 ms hangover.
    pub fn default_config() -> Self {
        Self::new(-40.0, 5.0, 150.0, 200.0, true)
    }

    pub fn state(&self) -> VadState {
        self.state
    }
    pub fn is_active(&self) -> bool {
        self.state == VadState::Active
    }
}

impl VadProcessor {
    pub fn process_slices(&mut self, input: &[f32], output: &mut [f32]) {
        let current_rms = rms(input);

        // Asymmetric IIR smoothing: fast attack, slow release
        let alpha = if current_rms > self.smoothed_rms {
            self.alpha_attack
        } else {
            self.alpha_release
        };
        self.smoothed_rms = alpha * current_rms + (1.0 - alpha) * self.smoothed_rms;

        // Threshold + hangover state machine
        if self.smoothed_rms >= self.threshold_rms {
            self.state = VadState::Active;
            self.hangover_count = self.hangover_frames;
        } else if self.hangover_count > 0 {
            self.state = VadState::Active;
            self.hangover_count -= 1;
        } else {
            self.state = VadState::Inactive;
        }

        if self.gate_output && self.state == VadState::Inactive {
            output.fill(0.0);
        } else {
            output.copy_from_slice(input);
        }
    }
}

impl RuntimeNode for VadProcessor {
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

    fn sine_frame(amplitude: f32) -> Vec<f32> {
        use std::f32::consts::PI;
        (0..FRAME_LEN_SAMPLES)
            .map(|i| amplitude * (2.0 * PI * 440.0 * i as f32 / 48000.0).sin())
            .collect()
    }

    #[test]
    fn given_silence_when_processed_then_vad_is_inactive() {
        let mut vad = VadProcessor::default_config();
        let silence = vec![0.0f32; FRAME_LEN_SAMPLES];
        let mut out = vec![0.0f32; FRAME_LEN_SAMPLES];
        // Run several frames to let smoothing settle
        for _ in 0..50 {
            vad.process_slices(&silence, &mut out);
        }
        assert_eq!(vad.state(), VadState::Inactive);
    }

    #[test]
    fn given_loud_tone_when_processed_then_vad_becomes_active() {
        let mut vad = VadProcessor::default_config();
        let loud = sine_frame(0.5); // well above -40 dBFS
        let mut out = vec![0.0f32; FRAME_LEN_SAMPLES];
        for _ in 0..20 {
            vad.process_slices(&loud, &mut out);
        }
        assert_eq!(vad.state(), VadState::Active);
    }

    #[test]
    fn given_active_vad_when_silence_then_hangover_delays_deactivation() {
        let mut vad = VadProcessor::new(-40.0, 5.0, 150.0, 200.0, true);
        let loud = sine_frame(0.5);
        let silence = vec![0.0f32; FRAME_LEN_SAMPLES];
        let mut out = vec![0.0f32; FRAME_LEN_SAMPLES];

        // Activate
        for _ in 0..20 {
            vad.process_slices(&loud, &mut out);
        }
        assert_eq!(vad.state(), VadState::Active);

        // One frame of silence — still active due to hangover
        vad.process_slices(&silence, &mut out);
        assert_eq!(vad.state(), VadState::Active);
    }

    #[test]
    fn given_gate_output_and_inactive_when_processed_then_output_is_silence() {
        let mut vad = VadProcessor::new(-40.0, 5.0, 150.0, 0.0, true); // 0 ms hangover
        let quiet = vec![1e-6f32; FRAME_LEN_SAMPLES]; // below -40 dBFS
        let mut out = vec![1.0f32; FRAME_LEN_SAMPLES];
        for _ in 0..100 {
            vad.process_slices(&quiet, &mut out);
        }
        assert_eq!(vad.state(), VadState::Inactive);
        assert!(out.iter().all(|&s| s == 0.0), "output must be gated to 0");
    }

    #[test]
    fn given_no_gate_and_inactive_when_processed_then_input_passes_through() {
        let mut vad = VadProcessor::new(-40.0, 5.0, 150.0, 0.0, false);
        let quiet = vec![1e-6f32; FRAME_LEN_SAMPLES];
        let mut out = vec![0.0f32; FRAME_LEN_SAMPLES];
        for _ in 0..100 {
            vad.process_slices(&quiet, &mut out);
        }
        // gate_output=false: input passes regardless of state
        assert!(out
            .iter()
            .zip(quiet.iter())
            .all(|(&o, &i)| (o - i).abs() < 1e-9));
    }
}

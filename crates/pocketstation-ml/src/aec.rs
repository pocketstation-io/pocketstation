// Acoustic Echo Canceller using NLMS (Normalized LMS) adaptive filtering.
//
// Design: a single adaptive FIR filter models the echo path from loudspeaker
// to microphone. The reference (far-end / loudspeaker) signal is supplied via
// set_reference() before each process() call. process() interleaves reference
// and mic samples so the ring buffer update and NLMS weight update happen at
// the correct sample time (not frame time), enabling convergence at delay D=0.
//
// NLMS convergence condition: 0 < mu < 2. At mu=0.5 the filter is conservative
// (stable on most signals); at mu=1.0 it is faster but may diverge on white noise.
//
// Production AEC (WebRTC AEC3, SpeexDSP) adds delay estimation, non-linear
// processing for double-talk, and frequency-domain partitioned convolution.
// This NLMS is the canonical foundational algorithm they all build on.
//
// process() is alloc-free, lock-free, blocking-free — LAW 15.
// Computing reference power per sample is O(FILTER_TAPS) per sample.

use pocketstation_graph::{GraphProcessor, FRAME_LEN_SAMPLES};

/// Number of adaptive filter taps. Covers the first 10.7 ms of echo path at 48 kHz.
const FILTER_TAPS: usize = 512;

/// NLMS acoustic echo canceller.
///
/// Call set_reference() with the far-end signal before each process() call.
/// The GraphProcessor::process() input is the microphone signal; output is
/// the echo-cancelled residual.
pub struct EchoCanceller {
    /// Adaptive filter weights (echo path model), pre-allocated.
    weights: [f32; FILTER_TAPS],
    /// Ring buffer for reference samples; indexed by ref_head (next write slot).
    ref_ring: Vec<f32>,
    ref_head: usize,
    /// Buffered reference frame for sample-by-sample interleaving in process().
    ref_frame: Vec<f32>,
    /// NLMS step size in (0, 2). Larger = faster convergence, less stable.
    mu: f32,
    /// Regularization added to power before normalization (prevents division by zero).
    regularization: f32,
    /// True after set_reference() is called at least once.
    has_reference: bool,
}

impl EchoCanceller {
    /// * `mu` — step size in (0, 2); 0.5 is conservative.
    /// * `regularization` — minimum power floor (e.g. 1e-4).
    pub fn new(mu: f32, regularization: f32) -> Self {
        Self {
            weights: [0.0f32; FILTER_TAPS],
            ref_ring: vec![0.0f32; FILTER_TAPS],
            ref_head: 0,
            ref_frame: vec![0.0f32; FRAME_LEN_SAMPLES],
            mu: mu.clamp(0.01, 1.99),
            regularization: regularization.max(1e-10),
            has_reference: false,
        }
    }

    /// Default: step size 0.5, regularization 1e-4.
    pub fn default_config() -> Self {
        Self::new(0.5, 1e-4)
    }

    /// Supply the loudspeaker (far-end) reference signal for the upcoming frame.
    /// Length must match the process() input length (FRAME_LEN_SAMPLES).
    pub fn set_reference(&mut self, reference: &[f32]) {
        let len = reference.len().min(self.ref_frame.len());
        self.ref_frame[..len].copy_from_slice(&reference[..len]);
        self.has_reference = true;
    }
}

impl GraphProcessor for EchoCanceller {
    /// Cancel echo from the microphone signal using the reference supplied via set_reference().
    ///
    /// Samples are interleaved: for each sample i, push ref_frame[i] into the ring buffer,
    /// compute the echo estimate, subtract from mic[i], and update the adaptive weights.
    /// This ensures zero-delay echo is fully cancellable (tap 0 = current reference sample).
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        if !self.has_reference {
            output.copy_from_slice(input);
            return;
        }

        let len = input.len().min(output.len()).min(self.ref_frame.len());

        for i in 0..len {
            // Push current reference sample into ring (sample-aligned with mic)
            self.ref_ring[self.ref_head] = self.ref_frame[i];
            self.ref_head = (self.ref_head + 1) % FILTER_TAPS;

            // Reference power for NLMS normalization
            let power: f32 =
                self.ref_ring.iter().map(|&r| r * r).sum::<f32>() + self.regularization;

            // Echo estimate: dot product of weights and ring buffer
            let mut echo_est = 0.0f32;
            for (tap, &w) in self.weights.iter().enumerate() {
                let ring_idx = (self.ref_head + FILTER_TAPS - 1 - tap) % FILTER_TAPS;
                echo_est += w * self.ref_ring[ring_idx];
            }

            let error = input[i] - echo_est;
            output[i] = error;

            // NLMS weight update: w += (mu / power) * error * reference
            let step = self.mu / power * error;
            for (tap, w) in self.weights.iter_mut().enumerate() {
                let ring_idx = (self.ref_head + FILTER_TAPS - 1 - tap) % FILTER_TAPS;
                *w += step * self.ref_ring[ring_idx];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine_frame(freq_hz: f32, amplitude: f32, offset_samples: usize) -> Vec<f32> {
        use std::f32::consts::PI;
        (0..FRAME_LEN_SAMPLES)
            .map(|i| amplitude * (2.0 * PI * freq_hz * (i + offset_samples) as f32 / 48000.0).sin())
            .collect()
    }

    #[test]
    fn given_no_reference_when_processed_then_input_passes_through() {
        let mut aec = EchoCanceller::default_config();
        let input = sine_frame(440.0, 0.5, 0);
        let mut output = vec![0.0f32; FRAME_LEN_SAMPLES];
        aec.process(&input, &mut output);
        for (o, i) in output.iter().zip(input.iter()) {
            assert!(
                (o - i).abs() < 1e-9,
                "without reference, output must equal input"
            );
        }
    }

    #[test]
    fn given_identical_mic_and_reference_when_processed_then_residual_decreases_over_time() {
        let mut aec = EchoCanceller::new(1.0, 1e-6);
        let mut total_residual_early = 0.0f32;
        let mut total_residual_late = 0.0f32;

        // Sample-interleaved design: set_reference and process with the SAME signal.
        // Tap 0 = current reference sample = mic sample → weights[0] → 1.0 at convergence.
        for frame in 0..400 {
            let signal = sine_frame(440.0, 0.5, frame * FRAME_LEN_SAMPLES);
            aec.set_reference(&signal);
            let mut output = vec![0.0f32; FRAME_LEN_SAMPLES];
            aec.process(&signal, &mut output);
            let rms: f32 =
                (output.iter().map(|&s| s * s).sum::<f32>() / FRAME_LEN_SAMPLES as f32).sqrt();
            if frame < 10 {
                total_residual_early += rms;
            } else if frame > 350 {
                total_residual_late += rms;
            }
        }
        assert!(
            total_residual_late < total_residual_early * 0.5,
            "residual must decrease as filter converges: early={total_residual_early:.4} late={total_residual_late:.4}"
        );
    }

    #[test]
    fn given_desired_signal_plus_echo_when_processed_then_residual_rms_is_lower_than_mic_rms() {
        let mut aec = EchoCanceller::new(0.5, 1e-4);
        let mut out = vec![0.0f32; FRAME_LEN_SAMPLES];

        // Converge over many frames: desired=440Hz, echo=880Hz reference
        for frame in 0..300 {
            let desired = sine_frame(440.0, 0.3, frame * FRAME_LEN_SAMPLES);
            let reference = sine_frame(880.0, 0.5, frame * FRAME_LEN_SAMPLES);
            let mic: Vec<f32> = desired
                .iter()
                .zip(reference.iter())
                .map(|(&d, &r)| d + r)
                .collect();
            aec.set_reference(&reference);
            aec.process(&mic, &mut out);
        }

        let desired = sine_frame(440.0, 0.3, 300 * FRAME_LEN_SAMPLES);
        let reference = sine_frame(880.0, 0.5, 300 * FRAME_LEN_SAMPLES);
        let mic: Vec<f32> = desired
            .iter()
            .zip(reference.iter())
            .map(|(&d, &r)| d + r)
            .collect();
        aec.set_reference(&reference);
        aec.process(&mic, &mut out);

        let mic_rms: f32 =
            (mic.iter().map(|&s| s * s).sum::<f32>() / FRAME_LEN_SAMPLES as f32).sqrt();
        let out_rms: f32 =
            (out.iter().map(|&s| s * s).sum::<f32>() / FRAME_LEN_SAMPLES as f32).sqrt();
        assert!(
            out_rms < mic_rms,
            "AEC must reduce RMS after convergence: mic={mic_rms:.4} out={out_rms:.4}"
        );
    }

    #[test]
    fn given_silence_reference_when_processed_then_weights_do_not_diverge() {
        let mut aec = EchoCanceller::new(1.0, 1e-4);
        let silence = vec![0.0f32; FRAME_LEN_SAMPLES];
        let mic = sine_frame(440.0, 0.5, 0);
        let mut out = vec![0.0f32; FRAME_LEN_SAMPLES];
        aec.set_reference(&silence);
        aec.process(&mic, &mut out);
        let max_weight = aec.weights.iter().map(|w| w.abs()).fold(0.0_f32, f32::max);
        assert!(
            max_weight < 1e6,
            "weights must not diverge with silence reference; max={max_weight}"
        );
    }
}

// pocketstation-ml — real-time audio signal processing nodes.
//
// Each processor exposes a slice-based DSP core (process_slices) and implements
// pocketstation_graph::RuntimeNode (frame-based, lifecycle, typed errors).
// LAW 15: the DSP core is alloc-free, lock-free, blocking-free, log-free.
// All state is pre-allocated in the constructor / prepare().

pub mod aec;
pub mod denoise;
pub mod vad;
pub mod watermark;

pub use aec::EchoCanceller;
pub use denoise::NoiseSuppressor;
pub use vad::VadProcessor;
pub use watermark::AudioWatermark;

/// Samples in one internal frame: 20 ms × 48 kHz mono (matches ADR-012/013).
pub const FRAME_SAMPLES_48K_20MS: usize = 960;

/// Frame duration the RuntimeNode lifecycle prepares these nodes for.
pub(crate) const FRAME_DURATION_MS: u32 = 20;

/// Compute the root-mean-square energy of a sample slice.
/// Used by all processors for level estimation.
#[inline]
pub(crate) fn rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: f32 = samples.iter().map(|&s| s * s).sum();
    (sum_sq / samples.len() as f32).sqrt()
}

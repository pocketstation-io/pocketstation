// pocketstation-ml — real-time audio signal processing nodes.
//
// All processors implement pocketstation_graph::GraphProcessor and satisfy
// LAW 15: process() is alloc-free, lock-free, blocking-free, log-free.
// All state is pre-allocated in the struct constructor.

pub mod aec;
pub mod denoise;
pub mod vad;
pub mod watermark;

pub use aec::EchoCanceller;
pub use denoise::NoiseSuppressor;
pub use vad::VadProcessor;
pub use watermark::AudioWatermark;

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

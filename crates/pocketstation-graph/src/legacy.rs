//! LEGACY slice-based DSP trait, retained only for `pocketstation-ml` until those
//! nodes migrate to the `RuntimeNode` lifecycle (Wave 6/7). Do not use in new code —
//! `RuntimeNode` (frame-based, typed errors, lifecycle) is the canonical interface.

/// Number of f32 samples in one 20 ms mono frame at 48 kHz.
pub const FRAME_LEN_SAMPLES: usize = 960; // 20ms × 48kHz (ADR-012)

/// Per-node slice processor. LAW 15: `process` must be alloc-free, lock-free,
/// blocking-free, and log-free; all working state is pre-allocated in the impl.
pub trait GraphProcessor: Send {
    /// Process one 20 ms frame. `input` and `output` share length FRAME_LEN_SAMPLES.
    fn process(&mut self, input: &[f32], output: &mut [f32]);
}

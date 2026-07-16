//! Timing primitives owned by the PocketStation runtime.
//!
//! Clock estimation and correction support source-aware stem alignment.
//! Voice-agent segment interruption remains compiled under [`experimental`]
//! until a generated-audio product path consumes it.

mod clock_correction;
mod clock_drift;

pub mod experimental;

pub use clock_correction::ClockCorrectionController;
pub use clock_drift::{ClockDriftEstimator, ClockDriftSnapshot};

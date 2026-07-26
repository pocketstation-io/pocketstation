//! Timing primitives owned by the PocketStation runtime.
//!
//! Clock estimation and correction support source-aware stem alignment.
//! Voice-agent segment interruption remains compiled under [`experimental`]
//! until a generated-audio product path consumes it.

mod clock_correction;
mod clock_drift;
mod timeline_mapping;

use std::sync::OnceLock;
use std::time::Instant;

pub mod experimental;

pub use clock_correction::ClockCorrectionController;
pub use clock_drift::{ClockDriftEstimator, ClockDriftSnapshot};
pub use timeline_mapping::TimelineMapping;

/// Process-wide monotonic timestamp domain shared by capture, routing, and
/// destination workers.
///
/// Keeping the origin in this owning timing crate makes timestamps comparable
/// across PocketStation crates. The value is never derived from a wall clock,
/// never moves backwards, and is always non-zero.
pub fn monotonic_timestamp_ns() -> u64 {
    static ORIGIN: OnceLock<Instant> = OnceLock::new();
    let elapsed_ns = ORIGIN.get_or_init(Instant::now).elapsed().as_nanos();
    u64::try_from(elapsed_ns)
        .unwrap_or(u64::MAX)
        .saturating_add(1)
}

#[cfg(test)]
mod monotonic_clock_tests {
    use super::monotonic_timestamp_ns;

    #[test]
    fn given_shared_monotonic_clock_when_sampled_then_value_never_moves_backwards() {
        let first = monotonic_timestamp_ns();
        let second = monotonic_timestamp_ns();

        assert!(first > 0);
        assert!(second >= first);
    }
}

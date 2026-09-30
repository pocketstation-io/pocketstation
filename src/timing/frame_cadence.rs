//! Source-clock residuals for fixed-duration PCM frames.
//!
//! Scheduling arrival times are deliberately absent. Small hardware timestamp
//! residuals are observations, not evidence that PCM samples were lost.

pub(crate) fn cadence_error_ns(previous_ns: u64, current_ns: u64, duration_ns: u64) -> u64 {
    match current_ns.checked_sub(previous_ns) {
        Some(elapsed_ns) if elapsed_ns > 0 => elapsed_ns.abs_diff(duration_ns),
        _ => u64::MAX,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_hardware_jitter_when_observed_then_residual_is_reported_without_inventing_gap() {
        assert_eq!(
            cadence_error_ns(1_000_000_000, 1_020_100_000, 20_000_000),
            100_000
        );
        assert_eq!(
            cadence_error_ns(1_000_000_000, 1_019_900_000, 20_000_000),
            100_000
        );
        assert_eq!(
            cadence_error_ns(1_000_000_000, 1_020_000_000, 20_000_000),
            0
        );
    }

    #[test]
    fn given_large_epoch_or_regression_when_observed_then_residual_is_exact() {
        assert_eq!(
            cadence_error_ns(u64::MAX - 40_000_000, u64::MAX - 19_998_000, 20_000_000),
            2_000
        );
        assert_eq!(cadence_error_ns(20, 19, 20_000_000), u64::MAX);
        assert_eq!(cadence_error_ns(20, 20, 20_000_000), u64::MAX);
    }
}

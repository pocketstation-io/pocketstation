const DRIFT_WINDOW_SAMPLES: usize = 100;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[doc = "Reports the clock drift snapshot collected at an observation boundary."]
pub struct ClockDriftSnapshot {
    #[doc = "Reports the estimated clock drift for `ClockDriftSnapshot`, in parts per million."]
    pub drift_ppm: f64,
    #[doc = "Stores the accumulated error value for `ClockDriftSnapshot`, in nanoseconds."]
    pub accumulated_error_ns: i64,
    #[doc = "Stores the number of observed samples represented by `ClockDriftSnapshot`."]
    pub observed_samples_count: u64,
}

#[doc = "Estimates source-clock drift from accumulated source and Session timing observations."]
pub struct ClockDriftEstimator {
    observations: [(f64, f64); DRIFT_WINDOW_SAMPLES],
    write_index: usize,
    observed_samples_count: u64,
    base_source_timestamp_ns: Option<u64>,
    base_runtime_timestamp_ns: Option<u64>,
    drift_ppm: f64,
    accumulated_error_ns: i64,
    window_full: bool,
}

impl ClockDriftEstimator {
    #[doc = "Creates a new `ClockDriftEstimator`."]
    pub fn new() -> Self {
        Self {
            observations: [(0.0, 0.0); DRIFT_WINDOW_SAMPLES],
            write_index: 0,
            observed_samples_count: 0,
            base_source_timestamp_ns: None,
            base_runtime_timestamp_ns: None,
            drift_ppm: 0.0,
            accumulated_error_ns: 0,
            window_full: false,
        }
    }

    #[doc = "Returns the current observation exposed by `ClockDriftEstimator`."]
    pub fn observe(&mut self, source_timestamp_ns: u64, runtime_timestamp_ns: u64) {
        let base_source_timestamp_ns = *self
            .base_source_timestamp_ns
            .get_or_insert(source_timestamp_ns);
        let base_runtime_timestamp_ns = *self
            .base_runtime_timestamp_ns
            .get_or_insert(runtime_timestamp_ns);

        self.observations[self.write_index] = (
            source_timestamp_ns.saturating_sub(base_source_timestamp_ns) as f64,
            runtime_timestamp_ns.saturating_sub(base_runtime_timestamp_ns) as f64,
        );
        self.write_index = (self.write_index + 1) % DRIFT_WINDOW_SAMPLES;
        self.observed_samples_count = self.observed_samples_count.saturating_add(1);
        if self.write_index == 0 {
            self.window_full = true;
        }

        let error_ns = i128::from(runtime_timestamp_ns) - i128::from(source_timestamp_ns);
        let bounded_error_ns = error_ns.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64;
        self.accumulated_error_ns = self.accumulated_error_ns.saturating_add(bounded_error_ns);
        self.estimate();
    }

    #[doc = "Returns the drift ppm held by `ClockDriftEstimator`."]
    pub fn drift_ppm(&self) -> f64 {
        self.drift_ppm
    }
    #[doc = "Returns the accumulated error nanoseconds held by `ClockDriftEstimator`."]
    pub fn accumulated_error_ns(&self) -> i64 {
        self.accumulated_error_ns
    }

    #[doc = "Returns a point-in-time snapshot of `ClockDriftEstimator`."]
    pub fn snapshot(&self) -> ClockDriftSnapshot {
        ClockDriftSnapshot {
            drift_ppm: self.drift_ppm,
            accumulated_error_ns: self.accumulated_error_ns,
            observed_samples_count: self.observed_samples_count,
        }
    }

    fn estimate(&mut self) {
        let sample_count = if self.window_full {
            DRIFT_WINDOW_SAMPLES
        } else {
            self.write_index
        };
        if sample_count < 2 {
            return;
        }

        let observations = &self.observations[..sample_count];
        let source_mean_ns = observations
            .iter()
            .map(|(source_ns, _)| source_ns)
            .sum::<f64>()
            / sample_count as f64;
        let runtime_mean_ns = observations
            .iter()
            .map(|(_, runtime_ns)| runtime_ns)
            .sum::<f64>()
            / sample_count as f64;

        let mut covariance_ns2 = 0.0;
        let mut source_variance_ns2 = 0.0;
        for &(source_ns, runtime_ns) in observations {
            let source_delta_ns = source_ns - source_mean_ns;
            let runtime_delta_ns = runtime_ns - runtime_mean_ns;
            covariance_ns2 += source_delta_ns * runtime_delta_ns;
            source_variance_ns2 += source_delta_ns * source_delta_ns;
        }
        if source_variance_ns2.abs() < f64::EPSILON {
            self.drift_ppm = 0.0;
            return;
        }

        let slope = covariance_ns2 / source_variance_ns2;
        self.drift_ppm = (slope - 1.0) * 1_000_000.0;
    }
}

impl Default for ClockDriftEstimator {
    #[doc = "Returns the default `ClockDriftEstimator` value."]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_aligned_clocks_when_observed_then_drift_is_near_zero() {
        let mut estimator = ClockDriftEstimator::new();
        for sample_index in 0..100u64 {
            let timestamp_ns = sample_index * 20_000_000;
            estimator.observe(timestamp_ns, timestamp_ns);
        }
        assert!(estimator.drift_ppm().abs() < 1.0);
    }

    #[test]
    fn given_faster_runtime_clock_when_observed_then_drift_is_positive() {
        let mut estimator = ClockDriftEstimator::new();
        for sample_index in 0..100u64 {
            estimator.observe(sample_index * 20_000_000, sample_index * 20_001_000);
        }
        assert!(estimator.drift_ppm() > 10.0);
    }

    #[test]
    fn given_slower_runtime_clock_when_observed_then_drift_is_negative() {
        let mut estimator = ClockDriftEstimator::new();
        for sample_index in 0..100u64 {
            estimator.observe(sample_index * 20_000_000, sample_index * 19_999_000);
        }
        assert!(estimator.drift_ppm() < -10.0);
    }

    #[test]
    fn given_large_absolute_timestamps_when_observed_then_relative_drift_stays_precise() {
        let mut estimator = ClockDriftEstimator::new();
        let base_timestamp_ns = 8_000_000_000_000_000_000;
        for sample_index in 0..100u64 {
            estimator.observe(
                base_timestamp_ns + sample_index * 20_000_000,
                base_timestamp_ns + sample_index * 20_001_000,
            );
        }
        assert!((estimator.drift_ppm() - 50.0).abs() < 1.0);
    }

    #[test]
    fn given_observations_when_snapshotted_then_lineage_metrics_are_reported() {
        let mut estimator = ClockDriftEstimator::new();
        estimator.observe(100, 125);
        estimator.observe(200, 250);
        let snapshot = estimator.snapshot();
        assert_eq!(snapshot.observed_samples_count, 2);
        assert_eq!(snapshot.accumulated_error_ns, 75);
    }
}

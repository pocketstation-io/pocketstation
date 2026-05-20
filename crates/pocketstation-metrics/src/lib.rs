use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct Counter(AtomicU64);
impl Counter {
    pub fn inc(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
    pub fn add(&self, n: u64) {
        self.0.fetch_add(n, Ordering::Relaxed);
    }
    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

#[derive(Default)]
pub struct Gauge(AtomicU64);
impl Gauge {
    pub fn set_scaled(&self, v: f64) {
        self.0.store((v * 1_000_000.0) as u64, Ordering::Relaxed);
    }
    pub fn get_scaled(&self) -> f64 {
        self.0.load(Ordering::Relaxed) as f64 / 1_000_000.0
    }
}

#[derive(Default)]
pub struct SimpleHistogram {
    count: AtomicU64,
    sum_ns: AtomicU64,
    max_ns: AtomicU64,
}
impl SimpleHistogram {
    pub fn record_ns(&self, v: u64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum_ns.fetch_add(v, Ordering::Relaxed);
        let mut old = self.max_ns.load(Ordering::Relaxed);
        while v > old
            && self
                .max_ns
                .compare_exchange_weak(old, v, Ordering::Relaxed, Ordering::Relaxed)
                .is_err()
        {
            old = self.max_ns.load(Ordering::Relaxed);
        }
    }
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }
    pub fn sum_ns(&self) -> u64 {
        self.sum_ns.load(Ordering::Relaxed)
    }
    pub fn max_ns(&self) -> u64 {
        self.max_ns.load(Ordering::Relaxed)
    }
}

#[derive(Default)]
pub struct BusMetrics {
    pub capture_to_bus_ns: SimpleHistogram,
    pub ring_utilization: Gauge,
    pub overruns_total: Counter,
    pub pool_exhaustion: Counter,
    pub frames_total: Counter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_inc_and_add_accumulate_correctly() {
        // Given
        let c = Counter::default();

        // When
        c.inc();
        c.inc();
        c.add(3);

        // Then
        assert_eq!(c.get(), 5);
    }

    #[test]
    fn gauge_set_scaled_round_trips_without_precision_loss() {
        // Given
        let g = Gauge::default();

        // When
        g.set_scaled(0.75);

        // Then
        let v = g.get_scaled();
        assert!((v - 0.75).abs() < 1e-5, "gauge round-trip: {v}");
    }

    #[test]
    fn histogram_records_count_sum_and_max_for_three_observations() {
        // Given
        let h = SimpleHistogram::default();

        // When
        h.record_ns(100);
        h.record_ns(300);
        h.record_ns(200);

        // Then
        assert_eq!(h.count(), 3);
        assert_eq!(h.sum_ns(), 600);
        assert_eq!(h.max_ns(), 300);
    }

    #[test]
    fn histogram_max_does_not_decrease_on_smaller_observation() {
        // Given
        let h = SimpleHistogram::default();
        h.record_ns(50);

        // When
        h.record_ns(10);

        // Then
        assert_eq!(h.max_ns(), 50);
    }

    #[test]
    fn bus_metrics_all_fields_are_zero_on_default() {
        // Given / When
        let m = BusMetrics::default();

        // Then
        assert_eq!(m.frames_total.get(), 0);
        assert_eq!(m.overruns_total.get(), 0);
        assert_eq!(m.pool_exhaustion.get(), 0);
        assert_eq!(m.ring_utilization.get_scaled(), 0.0);
        assert_eq!(m.capture_to_bus_ns.count(), 0);
    }

    #[test]
    fn bus_metrics_fields_are_independent_of_each_other() {
        // Given
        let m = BusMetrics::default();

        // When
        m.frames_total.add(10);
        m.overruns_total.inc();

        // Then
        assert_eq!(m.frames_total.get(), 10);
        assert_eq!(m.overruns_total.get(), 1);
        assert_eq!(m.pool_exhaustion.get(), 0);
    }
}

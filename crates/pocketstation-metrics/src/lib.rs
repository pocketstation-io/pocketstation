use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct Counter(AtomicU64);
impl Counter {
    pub fn inc(&self)          { self.0.fetch_add(1, Ordering::Relaxed); }
    pub fn add(&self, n: u64)  { self.0.fetch_add(n, Ordering::Relaxed); }
    pub fn get(&self) -> u64   { self.0.load(Ordering::Relaxed) }
}

#[derive(Default)]
pub struct Gauge(AtomicU64);
impl Gauge {
    pub fn set_scaled(&self, v: f64) { self.0.store((v * 1_000_000.0) as u64, Ordering::Relaxed); }
    pub fn get_scaled(&self) -> f64  { self.0.load(Ordering::Relaxed) as f64 / 1_000_000.0 }
}

const BUCKET_WIDTH_NS: u64   = 250_000;            // 250 µs per bucket
const NUM_BUCKETS:     usize = 64;
const OVERFLOW_BUCKET: usize = NUM_BUCKETS - 1;    // captures values >= 16 ms

#[inline]
fn bucket_for(v: u64) -> usize {
    let idx = (v / BUCKET_WIDTH_NS) as usize;
    if idx >= OVERFLOW_BUCKET { OVERFLOW_BUCKET } else { idx }
}

#[inline]
fn bucket_midpoint_ns(idx: usize) -> u64 {
    if idx >= OVERFLOW_BUCKET {
        (OVERFLOW_BUCKET as u64) * BUCKET_WIDTH_NS + BUCKET_WIDTH_NS / 2
    } else {
        idx as u64 * BUCKET_WIDTH_NS + BUCKET_WIDTH_NS / 2
    }
}

/// Lock-free histogram for nanosecond latency values.
/// 64 linear 250 µs buckets (0..16 ms); bucket 63 is the overflow bucket.
/// All counters are `AtomicU64` — Send + Sync without any mutex.
pub struct SimpleHistogram {
    buckets: [AtomicU64; NUM_BUCKETS],
    count:   AtomicU64,
    sum_ns:  AtomicU64,
    max_ns:  AtomicU64,
}

impl SimpleHistogram {
    fn new_buckets() -> [AtomicU64; NUM_BUCKETS] {
        // SAFETY: AtomicU64 has the same bit-pattern as u64 and is valid when
        // zeroed. MaybeUninit::zeroed() produces all-zero bits — the correct
        // initial value. Standard idiom for large atomic arrays on stable Rust.
        use std::mem::MaybeUninit;
        let mut arr: MaybeUninit<[AtomicU64; NUM_BUCKETS]> = MaybeUninit::uninit();
        let ptr = arr.as_mut_ptr() as *mut AtomicU64;
        for i in 0..NUM_BUCKETS {
            // SAFETY: ptr is valid for NUM_BUCKETS elements.
            unsafe { ptr.add(i).write(AtomicU64::new(0)); }
        }
        // SAFETY: every element has been written.
        unsafe { arr.assume_init() }
    }

    /// No heap allocation · no locks · safe to call from a real-time callback.
    pub fn record_ns(&self, v: u64) {
        let idx = bucket_for(v);
        self.buckets[idx].fetch_add(1, Ordering::Relaxed);
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum_ns.fetch_add(v, Ordering::Relaxed);
        let mut old = self.max_ns.load(Ordering::Relaxed);
        while v > old {
            match self.max_ns.compare_exchange_weak(old, v, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => break,
                Err(current) => old = current,
            }
        }
    }

    pub fn count(&self) -> u64   { self.count.load(Ordering::Relaxed) }
    pub fn sum_ns(&self) -> u64  { self.sum_ns.load(Ordering::Relaxed) }
    pub fn max_ns(&self) -> u64  { self.max_ns.load(Ordering::Relaxed) }

    /// Non-atomic snapshot — slightly inconsistent under concurrent writes.
    /// Acceptable for metrics reporting; `p` must be in `[0.0, 1.0]`.
    pub fn percentile_ns(&self, p: f64) -> u64 {
        debug_assert!((0.0..=1.0).contains(&p), "percentile must be in [0.0, 1.0]");
        let mut counts = [0u64; NUM_BUCKETS];
        let mut total: u64 = 0;
        for (i, bucket) in self.buckets.iter().enumerate() {
            let c = bucket.load(Ordering::Relaxed);
            counts[i] = c;
            total = total.saturating_add(c);
        }
        if total == 0 { return 0; }
        let threshold = ((p * total as f64).ceil() as u64).max(1);
        let mut accumulated: u64 = 0;
        for (i, &c) in counts.iter().enumerate() {
            accumulated = accumulated.saturating_add(c);
            if accumulated >= threshold { return bucket_midpoint_ns(i); }
        }
        bucket_midpoint_ns(OVERFLOW_BUCKET)
    }

    pub fn p50_ns(&self) -> u64  { self.percentile_ns(0.50) }
    pub fn p95_ns(&self) -> u64  { self.percentile_ns(0.95) }
    pub fn p99_ns(&self) -> u64  { self.percentile_ns(0.99) }
    pub fn p999_ns(&self) -> u64 { self.percentile_ns(0.999) }
}

impl Default for SimpleHistogram {
    fn default() -> Self {
        Self {
            buckets: Self::new_buckets(),
            count:   AtomicU64::new(0),
            sum_ns:  AtomicU64::new(0),
            max_ns:  AtomicU64::new(0),
        }
    }
}

// SAFETY: all interior mutability is via AtomicU64, which is Send + Sync.
unsafe impl Send for SimpleHistogram {}
unsafe impl Sync for SimpleHistogram {}

#[derive(Default)]
pub struct BusMetrics {
    pub capture_to_bus_ns: SimpleHistogram,
    pub ring_utilization:  Gauge,
    pub overruns_total:    Counter,
    pub pool_exhaustion:   Counter,
    pub frames_total:      Counter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_inc_and_add_accumulate_correctly() {
        let c = Counter::default();
        c.inc();
        c.inc();
        c.add(3);
        assert_eq!(c.get(), 5);
    }

    #[test]
    fn gauge_set_scaled_round_trips_without_precision_loss() {
        let g = Gauge::default();
        g.set_scaled(0.75);
        let v = g.get_scaled();
        assert!((v - 0.75).abs() < 1e-5, "gauge round-trip: {v}");
    }

    #[test]
    fn histogram_records_count_sum_and_max_for_three_observations() {
        let h = SimpleHistogram::default();
        h.record_ns(100);
        h.record_ns(300);
        h.record_ns(200);
        assert_eq!(h.count(), 3);
        assert_eq!(h.sum_ns(), 600);
        assert_eq!(h.max_ns(), 300);
    }

    #[test]
    fn histogram_max_does_not_decrease_on_smaller_observation() {
        let h = SimpleHistogram::default();
        h.record_ns(50);
        h.record_ns(10);
        assert_eq!(h.max_ns(), 50);
    }

    #[test]
    fn bus_metrics_all_fields_are_zero_on_default() {
        let m = BusMetrics::default();
        assert_eq!(m.frames_total.get(), 0);
        assert_eq!(m.overruns_total.get(), 0);
        assert_eq!(m.pool_exhaustion.get(), 0);
        assert_eq!(m.ring_utilization.get_scaled(), 0.0);
        assert_eq!(m.capture_to_bus_ns.count(), 0);
    }

    #[test]
    fn bus_metrics_fields_are_independent_of_each_other() {
        let m = BusMetrics::default();
        m.frames_total.add(10);
        m.overruns_total.inc();
        assert_eq!(m.frames_total.get(), 10);
        assert_eq!(m.overruns_total.get(), 1);
        assert_eq!(m.pool_exhaustion.get(), 0);
    }

    #[test]
    fn histogram_p50_of_uniform_0_to_10ms_is_near_5ms() {
        let h = SimpleHistogram::default();
        for i in 0..100u64 { h.record_ns(i * 100_000); }
        let p50 = h.p50_ns();
        let lo: u64 = 4_750_000;
        let hi: u64 = 5_250_000;
        assert!(p50 >= lo && p50 <= hi, "P50 = {p50} ns, expected {lo}..{hi} ns");
    }

    #[test]
    fn histogram_single_sample_all_percentiles_return_same_bucket() {
        let h = SimpleHistogram::default();
        h.record_ns(3_000_000);
        let p50 = h.p50_ns();
        assert_eq!(p50, h.p95_ns(), "P50 != P95 for single sample");
        assert_eq!(h.p95_ns(), h.p99_ns(), "P95 != P99 for single sample");
        assert_eq!(h.p99_ns(), h.p999_ns(), "P99 != P999 for single sample");
        assert_eq!(p50, bucket_midpoint_ns(bucket_for(3_000_000)));
    }

    #[test]
    fn histogram_p99_of_uniform_0_to_10ms_is_above_9ms() {
        let h = SimpleHistogram::default();
        for i in 0..100u64 { h.record_ns(i * 100_000); }
        assert!(h.p99_ns() >= 9_000_000, "P99 = {} ns", h.p99_ns());
    }

    #[test]
    fn histogram_overflow_sample_lands_in_overflow_bucket() {
        let h = SimpleHistogram::default();
        h.record_ns(20_000_000);
        assert_eq!(h.p50_ns(), bucket_midpoint_ns(OVERFLOW_BUCKET));
    }

    #[test]
    fn histogram_record_ns_accumulates_10000_observations_correctly() {
        let h = SimpleHistogram::default();
        for i in 0..10_000u64 { h.record_ns(i * 1_000); }
        assert_eq!(h.count(), 10_000);
    }
}

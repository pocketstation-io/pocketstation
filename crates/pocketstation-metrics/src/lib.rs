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

// ---------------------------------------------------------------------------
// Histogram — 64 linear buckets of 250 µs each (0..16 ms), bucket 63 is the
// overflow bucket (>= 16 ms).  Fixed-size array on the stack / in the struct;
// no heap allocation after construction.  All mutation is via AtomicU64 so the
// type is Send + Sync without any locks.
// ---------------------------------------------------------------------------

/// Width of each histogram bucket in nanoseconds (250 µs).
const BUCKET_WIDTH_NS: u64 = 250_000;

/// Number of buckets (including the overflow bucket at index 63).
const NUM_BUCKETS: usize = 64;

/// Index of the overflow bucket (captures all values >= 16 ms).
const OVERFLOW_BUCKET: usize = NUM_BUCKETS - 1;

/// Returns the bucket index for a nanosecond value.
#[inline]
fn bucket_for(v: u64) -> usize {
    let idx = (v / BUCKET_WIDTH_NS) as usize;
    if idx >= OVERFLOW_BUCKET {
        OVERFLOW_BUCKET
    } else {
        idx
    }
}

/// Returns the midpoint (in ns) of the given bucket.
#[inline]
fn bucket_midpoint_ns(idx: usize) -> u64 {
    if idx >= OVERFLOW_BUCKET {
        // Midpoint of the overflow bucket is defined as its lower bound + half
        // a bucket width; this is an approximation but consistent.
        (OVERFLOW_BUCKET as u64) * BUCKET_WIDTH_NS + BUCKET_WIDTH_NS / 2
    } else {
        idx as u64 * BUCKET_WIDTH_NS + BUCKET_WIDTH_NS / 2
    }
}

/// Lock-free, heap-allocation-free histogram for nanosecond latency values.
///
/// Uses 64 linear 250 µs buckets covering 0..16 ms.  The 64th bucket is the
/// overflow bucket for values >= 16 ms.  All counters are `AtomicU64`, making
/// the type `Send + Sync` without requiring any mutex.
///
/// Backward-compatible with the former `SimpleHistogram`: `count()`, `sum_ns()`
/// and `max_ns()` are all preserved.
pub struct SimpleHistogram {
    buckets: [AtomicU64; NUM_BUCKETS],
    count: AtomicU64,
    sum_ns: AtomicU64,
    max_ns: AtomicU64,
}

impl SimpleHistogram {
    // `AtomicU64` is not `Copy`, so we cannot use a const initialiser for the
    // array in a `Default` impl in stable Rust without `unsafe` or a macro.
    // We use a `const fn`-compatible approach: initialise via a helper that
    // fills the array using `MaybeUninit`.
    fn new_buckets() -> [AtomicU64; NUM_BUCKETS] {
        // SAFETY: AtomicU64 has the same representation as u64 and is valid
        // when initialised to zero.  `MaybeUninit::zeroed()` produces a
        // bit-pattern of all zeros which is the correct initial value.
        // This is the standard idiom for initialising large atomic arrays in
        // stable Rust without const generics + Copy.
        use std::mem::MaybeUninit;
        let mut arr: MaybeUninit<[AtomicU64; NUM_BUCKETS]> = MaybeUninit::uninit();
        let ptr = arr.as_mut_ptr() as *mut AtomicU64;
        for i in 0..NUM_BUCKETS {
            // SAFETY: ptr is valid for NUM_BUCKETS elements.
            unsafe {
                ptr.add(i).write(AtomicU64::new(0));
            }
        }
        // SAFETY: every element has been written.
        unsafe { arr.assume_init() }
    }

    /// Record a single latency observation (nanoseconds).
    ///
    /// No heap allocation.  No locks.  Safe to call from a real-time callback.
    pub fn record_ns(&self, v: u64) {
        let idx = bucket_for(v);
        self.buckets[idx].fetch_add(1, Ordering::Relaxed);

        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum_ns.fetch_add(v, Ordering::Relaxed);

        let mut old = self.max_ns.load(Ordering::Relaxed);
        while v > old {
            match self
                .max_ns
                .compare_exchange_weak(old, v, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(current) => old = current,
            }
        }
    }

    /// Total number of recorded observations.
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    /// Sum of all recorded values in nanoseconds.
    pub fn sum_ns(&self) -> u64 {
        self.sum_ns.load(Ordering::Relaxed)
    }

    /// Maximum recorded value in nanoseconds.
    pub fn max_ns(&self) -> u64 {
        self.max_ns.load(Ordering::Relaxed)
    }

    /// Compute a percentile from the bucket snapshot.
    ///
    /// `p` must be in `[0.0, 1.0]`.  Returns the midpoint of the bucket that
    /// contains the p-th percentile.  Returns 0 if no observations have been
    /// recorded.
    ///
    /// This is non-atomic: it reads each bucket independently with
    /// `Ordering::Relaxed`, so the snapshot may be slightly inconsistent under
    /// concurrent writes.  This is acceptable for metrics reporting.
    pub fn percentile_ns(&self, p: f64) -> u64 {
        debug_assert!((0.0..=1.0).contains(&p), "percentile must be in [0.0, 1.0]");

        // Snapshot all buckets.
        let mut counts = [0u64; NUM_BUCKETS];
        let mut total: u64 = 0;
        for (i, bucket) in self.buckets.iter().enumerate() {
            let c = bucket.load(Ordering::Relaxed);
            counts[i] = c;
            total = total.saturating_add(c);
        }

        if total == 0 {
            return 0;
        }

        // Number of observations that must be accumulated before we reach the
        // p-th percentile.  Use ceiling so that e.g. P50 of 1 observation
        // returns that observation's bucket.
        let threshold = (p * total as f64).ceil() as u64;
        let threshold = threshold.max(1);

        let mut accumulated: u64 = 0;
        for (i, &c) in counts.iter().enumerate() {
            accumulated = accumulated.saturating_add(c);
            if accumulated >= threshold {
                return bucket_midpoint_ns(i);
            }
        }

        // Should not be reached; return the overflow midpoint as a fallback.
        bucket_midpoint_ns(OVERFLOW_BUCKET)
    }

    /// 50th percentile latency in nanoseconds.
    pub fn p50_ns(&self) -> u64 {
        self.percentile_ns(0.50)
    }

    /// 95th percentile latency in nanoseconds.
    pub fn p95_ns(&self) -> u64 {
        self.percentile_ns(0.95)
    }

    /// 99th percentile latency in nanoseconds.
    pub fn p99_ns(&self) -> u64 {
        self.percentile_ns(0.99)
    }

    /// 99.9th percentile latency in nanoseconds.
    pub fn p999_ns(&self) -> u64 {
        self.percentile_ns(0.999)
    }
}

impl Default for SimpleHistogram {
    fn default() -> Self {
        Self {
            buckets: Self::new_buckets(),
            count: AtomicU64::new(0),
            sum_ns: AtomicU64::new(0),
            max_ns: AtomicU64::new(0),
        }
    }
}

// SAFETY: all interior mutability is via AtomicU64, which is Send + Sync.
unsafe impl Send for SimpleHistogram {}
unsafe impl Sync for SimpleHistogram {}

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

    // -----------------------------------------------------------------------
    // New percentile tests
    // -----------------------------------------------------------------------

    /// 100 samples uniformly distributed over 0..10 ms.
    /// P50 should fall in the 5 ms region (± one 250 µs bucket).
    #[test]
    fn histogram_p50_of_uniform_0_to_10ms_is_near_5ms() {
        // Given: 100 samples at 100 µs spacing = 0 ns, 100_000 ns, … , 9_900_000 ns
        let h = SimpleHistogram::default();
        for i in 0..100u64 {
            h.record_ns(i * 100_000);
        }

        // When
        let p50 = h.p50_ns();

        // Then: P50 ≈ 5 ms.  Acceptable range: 4.75 ms .. 5.25 ms (one bucket width).
        let lo: u64 = 4_750_000;
        let hi: u64 = 5_250_000;
        assert!(
            p50 >= lo && p50 <= hi,
            "P50 = {p50} ns, expected {lo}..{hi} ns"
        );
    }

    /// Single sample recorded; P50, P95, P99, P999 all return the same bucket.
    #[test]
    fn histogram_single_sample_all_percentiles_return_same_bucket() {
        // Given
        let h = SimpleHistogram::default();
        let sample_ns = 3_000_000u64; // 3 ms

        // When
        h.record_ns(sample_ns);

        // Then: all percentiles land in the same bucket
        let p50 = h.p50_ns();
        let p95 = h.p95_ns();
        let p99 = h.p99_ns();
        let p999 = h.p999_ns();
        assert_eq!(p50, p95, "P50 != P95 for single sample");
        assert_eq!(p95, p99, "P95 != P99 for single sample");
        assert_eq!(p99, p999, "P99 != P999 for single sample");

        // And the bucket midpoint should be within one bucket width of the sample.
        let expected_bucket = bucket_for(sample_ns);
        let expected_mid = bucket_midpoint_ns(expected_bucket);
        assert_eq!(p50, expected_mid, "wrong bucket midpoint for single sample");
    }

    /// P99 of 100 uniform samples across 0..10 ms should be above 9 ms.
    #[test]
    fn histogram_p99_of_uniform_0_to_10ms_is_above_9ms() {
        // Given
        let h = SimpleHistogram::default();
        for i in 0..100u64 {
            h.record_ns(i * 100_000);
        }

        // When
        let p99 = h.p99_ns();

        // Then: the 99th-percentile value should be at least 9 ms
        assert!(
            p99 >= 9_000_000,
            "P99 = {p99} ns, expected >= 9_000_000 ns"
        );
    }

    /// Overflow bucket: a value of 20 ms (above 16 ms) lands in bucket 63.
    #[test]
    fn histogram_overflow_sample_lands_in_overflow_bucket() {
        // Given
        let h = SimpleHistogram::default();
        h.record_ns(20_000_000); // 20 ms

        // When / Then: P50 returns the overflow bucket midpoint
        let p50 = h.p50_ns();
        assert_eq!(p50, bucket_midpoint_ns(OVERFLOW_BUCKET));
    }

    /// `record_ns` is callable without any dynamic allocation in the hot path.
    /// We can't prove zero-allocation statically here, but we verify that calling
    /// it 10 000 times is functionally correct (count matches).
    #[test]
    fn histogram_record_ns_accumulates_10000_observations_correctly() {
        // Given
        let h = SimpleHistogram::default();

        // When
        for i in 0..10_000u64 {
            h.record_ns(i * 1_000); // 0 ns .. 9.999 ms in 1 µs steps
        }

        // Then
        assert_eq!(h.count(), 10_000);
    }
}

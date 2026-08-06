//! Lock-free runtime counters and audio-edge observations.

use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

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
pub struct Gauge(AtomicI64);
impl Gauge {
    pub fn set_scaled(&self, v: f64) {
        self.0.store((v * 1_000_000.0) as i64, Ordering::Relaxed);
    }
    pub fn get_scaled(&self) -> f64 {
        self.0.load(Ordering::Relaxed) as f64 / 1_000_000.0
    }
}

const BUCKET_WIDTH_NS: u64 = 250_000; // 250 µs per bucket
const NUM_BUCKETS: usize = 64;
const OVERFLOW_BUCKET: usize = NUM_BUCKETS - 1; // captures values >= 16 ms

#[inline]
fn bucket_for(v: u64) -> usize {
    let idx = (v / BUCKET_WIDTH_NS) as usize;
    if idx >= OVERFLOW_BUCKET {
        OVERFLOW_BUCKET
    } else {
        idx
    }
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
    count: AtomicU64,
    sum_ns: AtomicU64,
    max_ns: AtomicU64,
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
            unsafe {
                ptr.add(i).write(AtomicU64::new(0));
            }
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
            match self
                .max_ns
                .compare_exchange_weak(old, v, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(current) => old = current,
            }
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
        if total == 0 {
            return 0;
        }
        let threshold = ((p * total as f64).ceil() as u64).max(1);
        let mut accumulated: u64 = 0;
        for (i, &c) in counts.iter().enumerate() {
            accumulated = accumulated.saturating_add(c);
            if accumulated >= threshold {
                return bucket_midpoint_ns(i);
            }
        }
        bucket_midpoint_ns(OVERFLOW_BUCKET)
    }

    pub fn p50_ns(&self) -> u64 {
        self.percentile_ns(0.50)
    }
    pub fn p95_ns(&self) -> u64 {
        self.percentile_ns(0.95)
    }
    pub fn p99_ns(&self) -> u64 {
        self.percentile_ns(0.99)
    }
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

/// Loudness reference: digital full scale (peak |sample| = 1.0) maps to 0 dBFS.
const FULL_SCALE_AMPLITUDE: f32 = 1.0;
/// Floor reported when a frame is digital silence (avoids log10(0) = -inf).
pub const SILENCE_FLOOR_DBFS: f32 = -120.0;

/// Per-frame audio observation, computed from real samples crossing one edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeObservation {
    pub peak_dbfs: f32,
    pub loudness_dbfs: f32,  // RMS level in dBFS
    pub clipping_count: u32, // samples at or beyond full scale
}

impl EdgeObservation {
    pub fn observe(samples: &[f32]) -> Self {
        if samples.is_empty() {
            return Self {
                peak_dbfs: SILENCE_FLOOR_DBFS,
                loudness_dbfs: SILENCE_FLOOR_DBFS,
                clipping_count: 0,
            };
        }
        let mut peak = 0.0f32;
        let mut sum_sq = 0.0f32;
        let mut clipping_count = 0u32;
        for &s in samples {
            let mag = s.abs();
            if mag > peak {
                peak = mag;
            }
            if mag >= FULL_SCALE_AMPLITUDE {
                clipping_count += 1;
            }
            sum_sq += s * s;
        }
        let rms = (sum_sq / samples.len() as f32).sqrt();
        Self {
            peak_dbfs: Self::to_dbfs(peak),
            loudness_dbfs: Self::to_dbfs(rms),
            clipping_count,
        }
    }

    fn to_dbfs(amplitude: f32) -> f32 {
        if amplitude <= 0.0 {
            return SILENCE_FLOOR_DBFS;
        }
        (20.0 * amplitude.log10()).max(SILENCE_FLOOR_DBFS)
    }

    pub fn is_clipping(&self) -> bool {
        self.clipping_count > 0
    }
}

/// Per-edge running metrics: frame throughput plus worst-case level observations.
#[derive(Default)]
pub struct EdgeMetrics {
    pub frames_in: Counter,
    pub frames_out: Counter,
    pub frames_dropped: Counter,
    pub clipping_frames: Counter,
    pub worst_peak_dbfs: Gauge,
}

impl EdgeMetrics {
    pub fn record_in(&self, observation: EdgeObservation) {
        self.frames_in.inc();
        if observation.is_clipping() {
            self.clipping_frames.inc();
        }
        if observation.peak_dbfs as f64 > self.worst_peak_dbfs.get_scaled()
            || self.frames_in.get() == 1
        {
            self.worst_peak_dbfs
                .set_scaled(observation.peak_dbfs as f64);
        }
    }

    pub fn record_out(&self) {
        self.frames_out.inc();
    }

    pub fn record_dropped(&self) {
        self.frames_dropped.inc();
    }

    pub fn drop_rate_pct(&self) -> f64 {
        let total = self.frames_in.get();
        if total == 0 {
            return 0.0;
        }
        self.frames_dropped.get() as f64 / total as f64 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_empty_counter_when_incremented_and_added_then_total_accumulates() {
        let c = Counter::default();
        c.inc();
        c.inc();
        c.add(3);
        assert_eq!(c.get(), 5);
    }

    #[test]
    fn given_scaled_gauge_when_value_round_trips_then_precision_is_preserved() {
        let g = Gauge::default();
        g.set_scaled(0.75);
        let v = g.get_scaled();
        assert!((v - 0.75).abs() < 1e-5, "gauge round-trip: {v}");
    }

    #[test]
    fn given_negative_scaled_gauge_when_value_round_trips_then_sign_is_preserved() {
        let gauge = Gauge::default();
        gauge.set_scaled(-6.0206);

        assert!(
            (gauge.get_scaled() - (-6.0206)).abs() < 1e-5,
            "negative gauge round-trip: {}",
            gauge.get_scaled()
        );
    }

    #[test]
    fn given_three_observations_when_recorded_then_count_sum_and_max_are_preserved() {
        let h = SimpleHistogram::default();
        h.record_ns(100);
        h.record_ns(300);
        h.record_ns(200);
        assert_eq!(h.count(), 3);
        assert_eq!(h.sum_ns(), 600);
        assert_eq!(h.max_ns(), 300);
    }

    #[test]
    fn given_histogram_max_when_smaller_value_is_recorded_then_max_does_not_decrease() {
        let h = SimpleHistogram::default();
        h.record_ns(50);
        h.record_ns(10);
        assert_eq!(h.max_ns(), 50);
    }

    #[test]
    fn given_default_bus_metrics_when_read_then_all_fields_are_zero() {
        let m = BusMetrics::default();
        assert_eq!(m.frames_total.get(), 0);
        assert_eq!(m.overruns_total.get(), 0);
        assert_eq!(m.pool_exhaustion.get(), 0);
        assert_eq!(m.ring_utilization.get_scaled(), 0.0);
        assert_eq!(m.capture_to_bus_ns.count(), 0);
    }

    #[test]
    fn given_bus_metrics_when_one_field_changes_then_other_fields_remain_independent() {
        let m = BusMetrics::default();
        m.frames_total.add(10);
        m.overruns_total.inc();
        assert_eq!(m.frames_total.get(), 10);
        assert_eq!(m.overruns_total.get(), 1);
        assert_eq!(m.pool_exhaustion.get(), 0);
    }

    #[test]
    fn given_uniform_0_to_10ms_values_when_p50_is_read_then_value_is_near_5ms() {
        let h = SimpleHistogram::default();
        for i in 0..100u64 {
            h.record_ns(i * 100_000);
        }
        let p50 = h.p50_ns();
        let lo: u64 = 4_750_000;
        let hi: u64 = 5_250_000;
        assert!(
            p50 >= lo && p50 <= hi,
            "P50 = {p50} ns, expected {lo}..{hi} ns"
        );
    }

    #[test]
    fn given_single_sample_when_percentiles_are_read_then_each_returns_same_bucket() {
        let h = SimpleHistogram::default();
        h.record_ns(3_000_000);
        let p50 = h.p50_ns();
        assert_eq!(p50, h.p95_ns(), "P50 != P95 for single sample");
        assert_eq!(h.p95_ns(), h.p99_ns(), "P95 != P99 for single sample");
        assert_eq!(h.p99_ns(), h.p999_ns(), "P99 != P999 for single sample");
        assert_eq!(p50, bucket_midpoint_ns(bucket_for(3_000_000)));
    }

    #[test]
    fn given_uniform_0_to_10ms_values_when_p99_is_read_then_value_is_above_9ms() {
        let h = SimpleHistogram::default();
        for i in 0..100u64 {
            h.record_ns(i * 100_000);
        }
        assert!(h.p99_ns() >= 9_000_000, "P99 = {} ns", h.p99_ns());
    }

    #[test]
    fn given_overflow_sample_when_recorded_then_value_lands_in_overflow_bucket() {
        let h = SimpleHistogram::default();
        h.record_ns(20_000_000);
        assert_eq!(h.p50_ns(), bucket_midpoint_ns(OVERFLOW_BUCKET));
    }

    #[test]
    fn given_10000_observations_when_recorded_then_histogram_accumulates_each_value() {
        let h = SimpleHistogram::default();
        for i in 0..10_000u64 {
            h.record_ns(i * 1_000);
        }
        assert_eq!(h.count(), 10_000);
    }

    #[test]
    fn given_silence_when_observed_then_levels_are_at_silence_floor() {
        let obs = EdgeObservation::observe(&[0.0; 960]);
        assert_eq!(obs.peak_dbfs, SILENCE_FLOOR_DBFS);
        assert_eq!(obs.loudness_dbfs, SILENCE_FLOOR_DBFS);
        assert_eq!(obs.clipping_count, 0);
    }

    #[test]
    fn given_full_scale_tone_when_observed_then_peak_is_zero_dbfs() {
        let obs = EdgeObservation::observe(&[1.0, -1.0, 1.0, -1.0]);
        assert!(
            (obs.peak_dbfs - 0.0).abs() < 1e-4,
            "peak {} dbfs",
            obs.peak_dbfs
        );
        assert_eq!(obs.clipping_count, 4);
        assert!(obs.is_clipping());
    }

    #[test]
    fn given_half_scale_signal_when_observed_then_peak_near_minus_six_dbfs() {
        let obs = EdgeObservation::observe(&[0.5, -0.5, 0.5, -0.5]);
        assert!(
            (obs.peak_dbfs - (-6.0206)).abs() < 0.01,
            "peak {} dbfs",
            obs.peak_dbfs
        );
        assert_eq!(obs.clipping_count, 0);
    }

    #[test]
    fn given_empty_frame_when_observed_then_silence_floor_and_no_clipping() {
        let obs = EdgeObservation::observe(&[]);
        assert_eq!(obs.peak_dbfs, SILENCE_FLOOR_DBFS);
        assert_eq!(obs.clipping_count, 0);
    }

    #[test]
    fn given_edge_metrics_when_three_in_one_dropped_then_drop_rate_is_thirty_three_pct() {
        let m = EdgeMetrics::default();
        m.record_in(EdgeObservation::observe(&[0.1; 8]));
        m.record_in(EdgeObservation::observe(&[0.1; 8]));
        m.record_in(EdgeObservation::observe(&[0.1; 8]));
        m.record_dropped();
        assert_eq!(m.frames_in.get(), 3);
        assert!((m.drop_rate_pct() - 33.333).abs() < 0.01);
    }

    #[test]
    fn given_edge_metrics_when_clipping_frame_recorded_then_clipping_counter_increments() {
        let m = EdgeMetrics::default();
        m.record_in(EdgeObservation::observe(&[0.1; 8]));
        m.record_in(EdgeObservation::observe(&[1.0; 8]));
        assert_eq!(m.clipping_frames.get(), 1);
        assert!((m.worst_peak_dbfs.get_scaled() - 0.0).abs() < 1e-3);
    }

    #[test]
    fn given_only_subscale_frames_when_recorded_then_worst_peak_remains_negative_dbfs() {
        let metrics = EdgeMetrics::default();
        metrics.record_in(EdgeObservation::observe(&[0.25; 8]));
        metrics.record_in(EdgeObservation::observe(&[0.5; 8]));

        assert!(
            (metrics.worst_peak_dbfs.get_scaled() - (-6.0206)).abs() < 0.01,
            "worst peak: {} dBFS",
            metrics.worst_peak_dbfs.get_scaled()
        );
    }
}

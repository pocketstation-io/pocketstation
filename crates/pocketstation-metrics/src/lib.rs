use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct Counter(AtomicU64);
impl Counter { pub fn inc(&self) { self.0.fetch_add(1, Ordering::Relaxed); } pub fn add(&self, n: u64) { self.0.fetch_add(n, Ordering::Relaxed); } pub fn get(&self) -> u64 { self.0.load(Ordering::Relaxed) } }

#[derive(Default)]
pub struct Gauge(AtomicU64);
impl Gauge { pub fn set_scaled(&self, v: f64) { self.0.store((v * 1_000_000.0) as u64, Ordering::Relaxed); } pub fn get_scaled(&self) -> f64 { self.0.load(Ordering::Relaxed) as f64 / 1_000_000.0 } }

#[derive(Default)]
pub struct SimpleHistogram { count: AtomicU64, sum_ns: AtomicU64, max_ns: AtomicU64 }
impl SimpleHistogram {
    pub fn record_ns(&self, v: u64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.sum_ns.fetch_add(v, Ordering::Relaxed);
        let mut old = self.max_ns.load(Ordering::Relaxed);
        while v > old && self.max_ns.compare_exchange_weak(old, v, Ordering::Relaxed, Ordering::Relaxed).is_err() { old = self.max_ns.load(Ordering::Relaxed); }
    }
    pub fn count(&self) -> u64 { self.count.load(Ordering::Relaxed) }
    pub fn max_ns(&self) -> u64 { self.max_ns.load(Ordering::Relaxed) }
}

#[derive(Default)]
pub struct BusMetrics {
    pub capture_to_bus_ns: SimpleHistogram,
    pub ring_utilization: Gauge,
    pub overruns_total: Counter,
    pub pool_exhaustion: Counter,
    pub frames_total: Counter,
}

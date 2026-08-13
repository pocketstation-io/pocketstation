use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pocketstation::internal::timing::{ClockCorrectionController, ClockDriftEstimator};

fn bench_timing(c: &mut Criterion) {
    let mut group = c.benchmark_group("timing");
    group.throughput(Throughput::Elements(1));

    group.bench_function("drift_observe_100_sample_window", |b| {
        let mut estimator = ClockDriftEstimator::new();
        let mut sample_index = 0_u64;
        b.iter(|| {
            let source_timestamp_ns = sample_index.saturating_mul(20_000_000);
            let runtime_timestamp_ns = sample_index.saturating_mul(20_001_000);
            estimator.observe(
                black_box(source_timestamp_ns),
                black_box(runtime_timestamp_ns),
            );
            sample_index = sample_index.wrapping_add(1);
            black_box(estimator.snapshot())
        });
    });

    group.bench_function("clock_correction_tick", |b| {
        let mut controller = ClockCorrectionController::default();
        b.iter(|| black_box(controller.tick(black_box(1_000_000))));
    });

    group.finish();
}

criterion_group!(benches, bench_timing);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pocketstation_audio::{AudioBufferPool, POOL_MAX_SLOTS, POOL_SLOT_SAMPLES};

fn bench_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("audio_buffer_pool");

    // Pool sized to the architecture maximum: 64 slots of 960 samples each.
    // Constructed once outside the loop — measures the atomic CAS acquire+release
    // cycle only, not allocation.
    let pool = AudioBufferPool::new(POOL_MAX_SLOTS, POOL_SLOT_SAMPLES);

    group.bench_function("acquire_and_drop", |b| {
        b.iter(|| {
            let handle = pool.acquire();
            black_box(&handle);
            // Drop calls AudioBufferPool::release via the handle's Drop impl,
            // which does a single fetch_or on the free_mask.  Both acquire and
            // release are measured in this iteration.
            drop(handle);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_pool);
criterion_main!(benches);

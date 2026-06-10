use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pocketstation_audio::{
    frame_bus, AudioBufferPool, AudioFrame, SourceId, StreamId,
    DEFAULT_SLOT_SAMPLES_MONO_20MS,
};

fn bench_bus(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_bus");

    // Pool of 64 slots — large enough that acquire never fails during the bench.
    let pool = AudioBufferPool::new(64, DEFAULT_SLOT_SAMPLES_MONO_20MS);

    group.bench_function("push_drop_newest_and_pop", |b| {
        // Create a fresh bus each iteration set so the ring is never saturated.
        // Capacity 2 is sufficient: one slot for the frame in flight, one spare
        // to avoid spurious drops during the iteration.
        let (mut producer, mut consumer) = frame_bus(2);

        b.iter(|| {
            let handle = pool.acquire().expect("pool exhausted in bus bench");
            let frame = AudioFrame::new(
                StreamId(1),
                SourceId(1),
                black_box(0u64),
                black_box(0u64),
                1,
                handle,
            );

            producer.push_drop_newest(frame).ok();
            let popped = consumer.pop();
            black_box(popped);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_bus);
criterion_main!(benches);

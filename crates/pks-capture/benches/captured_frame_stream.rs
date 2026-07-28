use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pks_capture::captured_frame_stream;
use pks_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId, POOL_SLOT_SAMPLES};

fn bench_bus(c: &mut Criterion) {
    let mut group = c.benchmark_group("captured_frame_stream");

    // Pool of 64 slots — large enough that acquire never fails during the bench.
    let pool = AudioBufferPool::new(64, POOL_SLOT_SAMPLES);

    group.bench_function("push_drop_newest_and_pop", |b| {
        // Create a fresh bus each iteration set so the ring is never saturated.
        // Capacity 2 is sufficient: one slot for the frame in flight, one spare
        // to avoid spurious drops during the iteration.
        let (mut sender, mut stream) =
            captured_frame_stream(2).expect("bounded captured-frame stream");

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

            let _ = sender.try_send(frame);
            let popped = stream.try_next();
            black_box(popped);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_bus);
criterion_main!(benches);

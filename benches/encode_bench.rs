use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pks_audio::{OpusEncoder, OPUS_FRAME_SAMPLES, OPUS_MAX_PACKET_BYTES};

fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("opus_encode");

    // Pre-warm: construct encoder once outside the measurement loop.
    let mut enc = OpusEncoder::default();

    // Pre-allocate output buffer once; encode_into reuses it with no extra allocation.
    let mut out = Vec::with_capacity(OPUS_MAX_PACKET_BYTES);

    // Case 1: silence (all zeros) — exercises the silence-detection path.
    let silence = vec![0.0f32; OPUS_FRAME_SAMPLES];
    group.bench_function("960_samples_silence", |b| {
        b.iter(|| {
            out.clear();
            enc.encode_into(black_box(&silence), &mut out).unwrap();
            black_box(out.len())
        })
    });

    // Case 2: 440 Hz sine at 25 % amplitude — exercises the full MDCT path.
    let sine: Vec<f32> = (0..OPUS_FRAME_SAMPLES)
        .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / 48_000.0).sin() * 0.25)
        .collect();
    group.bench_function("960_samples_sine", |b| {
        b.iter(|| {
            out.clear();
            enc.encode_into(black_box(&sine), &mut out).unwrap();
            black_box(out.len())
        })
    });

    group.finish();
}

criterion_group!(benches, bench_encode);
criterion_main!(benches);

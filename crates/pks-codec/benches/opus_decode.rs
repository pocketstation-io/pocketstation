use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pks_codec::{OpusDecoder, OpusEncoder, OPUS_FRAME_SAMPLES, OPUS_MAX_PACKET_BYTES};

fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("opus_decode");

    // Pre-encode a packet outside the benchmark loop so the loop measures
    // only the decode path, not encode overhead.
    let mut enc = OpusEncoder::default();
    let sine: Vec<f32> = (0..OPUS_FRAME_SAMPLES)
        .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / 48_000.0).sin() * 0.25)
        .collect();
    let mut packet = Vec::with_capacity(OPUS_MAX_PACKET_BYTES);
    enc.encode_into(&sine, &mut packet).unwrap();

    // Pre-warm the decoder; reuse across iterations.
    let mut dec = OpusDecoder::default();
    let mut out = Vec::with_capacity(OPUS_FRAME_SAMPLES);

    group.bench_function("960_samples_from_packet", |b| {
        b.iter(|| {
            out.clear();
            dec.decode_into(black_box(&packet), &mut out, false)
                .unwrap();
            black_box(out.len())
        })
    });

    group.finish();
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);

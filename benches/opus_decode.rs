use criterion::{black_box, criterion_group, criterion_main, Criterion};
use opus::{Channels, Decoder};
use pocketstation::internal::codec::{
    OpusDecoder, OpusEncoder, OpusFrameDuration, OPUS_FRAME_SAMPLES, OPUS_MAX_PACKET_BYTES,
};

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

    let mut direct_decoder = Decoder::new(48_000, Channels::Mono).expect("direct decoder");
    let mut direct_pcm = [0_i16; OPUS_FRAME_SAMPLES * 2];
    let mut direct_output = Vec::with_capacity(OPUS_FRAME_SAMPLES);
    group.bench_function("calibration_direct_libopus_960_samples", |b| {
        b.iter(|| {
            direct_output.clear();
            let decoded = direct_decoder
                .decode(black_box(&packet), &mut direct_pcm, false)
                .expect("direct decode");
            direct_output.resize(decoded, 0.0_f32);
            for (destination, &source) in direct_output.iter_mut().zip(&direct_pcm[..decoded]) {
                *destination = source as f32 / 32_767.0;
            }
            black_box(direct_output.len());
        })
    });

    let mut plc_decoder = OpusDecoder::default();
    let mut concealed = Vec::with_capacity(OPUS_FRAME_SAMPLES);
    plc_decoder
        .decode_plc_into(OpusFrameDuration::Ms20, &mut concealed)
        .expect("PLC warmup");
    concealed.clear();
    group.bench_function("plc_20ms_mono", |b| {
        b.iter(|| {
            concealed.clear();
            plc_decoder
                .decode_plc_into(OpusFrameDuration::Ms20, &mut concealed)
                .unwrap();
            black_box(concealed.len())
        })
    });

    group.finish();
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);

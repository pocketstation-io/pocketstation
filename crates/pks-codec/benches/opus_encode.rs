//! A2 — OpusEncode budget validation.
//!
//! Validates §11.3: FrameBus → Opus encoder ≤ 2ms per 20ms frame.
//!
//! Run:
//!   cargo bench -p pks-codec --bench opus_encode
//!
//! Pass threshold: p99 ≤ 2ms (2_000_000 ns).

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pks_codec::{
    OpusApplication, OpusChannels, OpusConfig, OpusEncoder, OpusFrameDuration, OpusSampleRate,
};

/// 20ms mono at 48kHz = 960 samples.
const SAMPLES_20MS_MONO: usize = 960;
/// 20ms stereo at 48kHz = 960 samples × 2 channels.
const SAMPLES_20MS_STEREO: usize = 1920;
/// 10ms mono at 48kHz = 480 samples (voice-agent mode).
const SAMPLES_10MS_MONO: usize = 480;

fn pcm_sine(samples: usize) -> Vec<f32> {
    (0..samples)
        .map(|i| (i as f32 * 2.0 * std::f32::consts::PI / 48.0).sin() * 0.5)
        .collect()
}

fn bench_opus_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("A2_OpusEncode");

    // voice20 — 20ms mono 32kbps (default broadcast profile).
    {
        let pcm = pcm_sine(SAMPLES_20MS_MONO);
        let config = OpusConfig {
            sample_rate: OpusSampleRate::Hz48000,
            channels: OpusChannels::Mono,
            frame_duration: OpusFrameDuration::Ms20,
            application: OpusApplication::Voip,
            bitrate_kbps: Some(32),
            complexity: 10,
            dtx: false,
            fec: false,
        };
        let mut enc = OpusEncoder::from_config(&config).expect("encoder");
        let mut out = Vec::with_capacity(4096);
        group.bench_function(BenchmarkId::new("voice20_mono_32k", ""), |b| {
            b.iter(|| {
                enc.encode_into(&pcm, &mut out).expect("encode");
            })
        });
    }

    // agent10 — 10ms mono 32kbps (voice-agent low-latency profile).
    {
        let pcm = pcm_sine(SAMPLES_10MS_MONO);
        let config = OpusConfig {
            sample_rate: OpusSampleRate::Hz48000,
            channels: OpusChannels::Mono,
            frame_duration: OpusFrameDuration::Ms10,
            application: OpusApplication::LowDelay,
            bitrate_kbps: Some(32),
            complexity: 10,
            dtx: false,
            fec: false,
        };
        let mut enc = OpusEncoder::from_config(&config).expect("encoder");
        let mut out = Vec::with_capacity(4096);
        group.bench_function(BenchmarkId::new("agent10_mono_32k", ""), |b| {
            b.iter(|| {
                enc.encode_into(&pcm, &mut out).expect("encode");
            })
        });
    }

    // music20 — 20ms stereo 96kbps (stereo music profile).
    {
        let pcm = pcm_sine(SAMPLES_20MS_STEREO);
        let config = OpusConfig {
            sample_rate: OpusSampleRate::Hz48000,
            channels: OpusChannels::Stereo,
            frame_duration: OpusFrameDuration::Ms20,
            application: OpusApplication::Audio,
            bitrate_kbps: Some(96),
            complexity: 10,
            dtx: false,
            fec: false,
        };
        let mut enc = OpusEncoder::from_config(&config).expect("encoder");
        let mut out = Vec::with_capacity(4096);
        group.bench_function(BenchmarkId::new("music20_stereo_96k", ""), |b| {
            b.iter(|| {
                enc.encode_into(&pcm, &mut out).expect("encode");
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_opus_encode);
criterion_main!(benches);

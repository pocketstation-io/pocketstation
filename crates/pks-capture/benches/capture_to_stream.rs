//! A1 §11.3 — Source capture → bounded captured-frame stream benchmark.
//!
//! **LABEL: SYNTHETIC** — no real CoreAudio/WASAPI/ALSA callback is used.
//! This bench measures the pure Rust hot-path cost:
//!   pool.acquire() → copy PCM into slot → AudioFrame::new() → CapturedFrameSender::try_send()
//!
//! It does NOT include:
//!   - CoreAudio/CPAL callback scheduling jitter
//!   - OS audio driver round-trip latency
//!   - Real hardware DMA transfer time
//!   - SCKit latency on macOS (≥10ms floor; sub-5ms needs CoreAudio tap, macOS 14.2+)
//!
//! §11.3 target: p99 ≤ 5ms.
//! A synthetic mean well under 5ms is necessary but not sufficient;
//! real CoreAudio measurement requires a tap harness that does not yet exist.
//! See `pocketstation-bench/BENCHMARK_CLAIMS.md` for the claim boundary.
//!
//! Command:
//!   cargo bench -p pks-capture --bench capture_to_stream

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pks_capture::captured_frame_stream;
use pks_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId, POOL_SLOT_SAMPLES};

/// Simulate one audio callback: fill a pool slot with synthetic PCM,
/// wrap it in an AudioFrame, and push it to the captured-frame stream.
/// This is the hot path that runs every 20ms on the callback thread.
fn bench_capture_framebus(c: &mut Criterion) {
    let mut group = c.benchmark_group("capture_framebus");
    group.measurement_time(std::time::Duration::from_secs(10));

    // Pool sized for 20ms voice frames. 64 slots = well beyond what the
    // callback needs; acquire never fails during the bench.
    let pool = AudioBufferPool::new(64, POOL_SLOT_SAMPLES);

    // Stream capacity 4: simulate a consumer that pops 1–2 frames behind the
    // sender. Capacity is large enough that this bench does not drop.
    let (mut sender, mut stream) = captured_frame_stream(4).expect("bounded captured-frame stream");

    // Synthetic 20ms mono voice PCM (48kHz × 20ms = 960 samples).
    // Simulates the DMA-delivered PCM that the platform callback hands us.
    let synthetic_pcm: Vec<f32> = (0..POOL_SLOT_SAMPLES)
        .map(|i| ((i as f32) * 0.001).sin())
        .collect();

    group.bench_function("voice20_mono_SYNTHETIC", |b| {
        b.iter(|| {
            // Step 1: acquire a pool slot (lock-free CAS; no allocation).
            let mut handle = pool.acquire().expect("pool exhausted in bench");

            // Step 2: copy PCM into the slot (simulates DMA → pool copy).
            handle.copy_from_slice(black_box(&synthetic_pcm));

            // Step 3: wrap in AudioFrame (zero-copy header construction).
            let frame = AudioFrame::new(
                StreamId(1),
                SourceId(1),
                black_box(0u64), // capture_ts_ns
                black_box(0u64), // sequence_id
                1,               // channel_count
                handle,
            );

            // Step 4: publish through the bounded capture SPSC stream.
            // This is the exact end of the §11.3 A1 segment.
            let _ = sender.try_send(black_box(frame));

            // Drain one frame to prevent ring saturation across iterations.
            let _ = stream.try_next();
        })
    });

    group.finish();
}

criterion_group!(benches, bench_capture_framebus);
criterion_main!(benches);

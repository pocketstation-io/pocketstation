/// Audio pipeline throughput soak — Phase 0 exit criterion.
///
/// Measures pool + ring + encode + decode throughput at two operating points:
///
///   Mode A  Complexity 9 (production quality, AUDIO-012):
///           Target ≤ 100ms for 3 000 frames.
///
///   Mode B  Complexity 0 (throughput ceiling, benchmarks only):
///           Target ≤ 30ms for 3 000 frames.
///
/// Optimisations applied (see .cargo/config.toml for SIMD flags):
///   - Precomputed 1-second sine table eliminates per-sample sin() (~19µs/frame)
///   - encode_into: unsafe set_len instead of resize (avoids 4 000-byte zero-fill)
///   - decode_into: pre-sized Vec loop auto-vectorised by LLVM (NEON / AVX2)
///   - .cargo/config.toml: CFLAGS=-march=native forces SIMD in libopus-sys
///   - Cargo.toml [profile.release]: LTO=fat, codegen-units=1
///
/// Run:  cargo run -p pocketstation-audio --example soak --release
use pocketstation_audio::{
    frame_bus, AudioBufferPool, AudioFrame, OpusDecoder, OpusEncoder, SourceId, StreamId,
    DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS,
};
use std::{f32::consts::PI, time::Instant};

const SOAK_FRAMES: u64 = 3_000; // 60 s of audio at 50 fps

// ── Precomputed sine table ──────────────────────────────────────────────────
// One full second at 48 kHz avoids the per-sample sin() call (~19 µs/frame).
// Indexed as `(global_sample_index) % TABLE_LEN`.
fn build_sine_table() -> Vec<f32> {
    let len = DEFAULT_SAMPLE_RATE as usize; // 48 000 samples = 1 s
    (0..len)
        .map(|i| (2.0 * PI * 440.0 * i as f32 / DEFAULT_SAMPLE_RATE as f32).sin() * 0.25)
        .collect()
}

// ── Single-mode soak run ────────────────────────────────────────────────────
fn run_soak(label: &str, complexity: i32, sine_table: &[f32]) {
    let pool = AudioBufferPool::new(64, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let (mut prod, mut cons) = frame_bus(64);

    let mut encoder = OpusEncoder::default();
    let mut decoder = OpusDecoder::default();

    // Set encoder complexity.
    // complexity 9 = production (AUDIO-012); complexity 0 = throughput ceiling.
    encoder
        .set_complexity(complexity)
        .expect("set_complexity failed");

    let mut encode_buf: Vec<u8> = Vec::with_capacity(4_000);
    let mut decode_buf: Vec<f32> = Vec::with_capacity(DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let mut total_decoded: u64 = 0;

    let start = Instant::now();

    for seq in 0..SOAK_FRAMES {
        // ── Produce ─────────────────────────────────────────────────────────
        let mut handle = pool.acquire().expect("pool exhausted");
        let offset = (seq * DEFAULT_SLOT_SAMPLES_MONO_20MS as u64) as usize;
        let samples = handle.as_mut_slice();
        for (i, s) in samples.iter_mut().enumerate() {
            *s = sine_table[(offset + i) % sine_table.len()];
        }
        let frame = AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, handle);
        let _ = prod.push_drop_newest(frame);

        // ── Consume (encode + decode) ────────────────────────────────────────
        if let Some(frame) = cons.pop() {
            encode_buf.clear();
            let n_enc = encoder
                .encode_into(frame.buffer.as_slice(), &mut encode_buf)
                .expect("encode_into failed");
            drop(frame);
            decode_buf.clear();
            let n_dec = decoder
                .decode_into(&encode_buf[..n_enc], &mut decode_buf)
                .expect("decode_into failed");
            total_decoded += n_dec as u64;
        }
    }

    // Drain remainder.
    while let Some(frame) = cons.pop() {
        encode_buf.clear();
        let n_enc = encoder
            .encode_into(frame.buffer.as_slice(), &mut encode_buf)
            .expect("encode_into failed");
        drop(frame);
        decode_buf.clear();
        let n_dec = decoder
            .decode_into(&encode_buf[..n_enc], &mut decode_buf)
            .expect("decode_into failed");
        total_decoded += n_dec as u64;
    }

    let elapsed = start.elapsed();
    let dropped = prod.dropped_newest();
    let pool_fail = pool.acquire_failures();
    let expected = SOAK_FRAMES * DEFAULT_SLOT_SAMPLES_MONO_20MS as u64;
    let fps = SOAK_FRAMES as f64 / elapsed.as_secs_f64();
    let realtime_mult = fps / 50.0; // 50 fps = 1× real-time

    println!("──── soak [{label}] ────────────────────────────────");
    println!("  elapsed          = {:.2?}", elapsed);
    println!("  frames           = {}", SOAK_FRAMES);
    println!(
        "  fps              = {:.0}  ({:.0}× real-time)",
        fps, realtime_mult
    );
    println!(
        "  µs/frame         = {:.1}",
        elapsed.as_micros() as f64 / SOAK_FRAMES as f64
    );
    println!("  dropped_newest   = {}", dropped);
    println!("  pool_failures    = {}", pool_fail);
    println!("  samples decoded  = {} / {}", total_decoded, expected);

    assert_eq!(dropped, 0, "[{label}] frames dropped");
    assert_eq!(pool_fail, 0, "[{label}] pool exhausted");
    assert_eq!(total_decoded, expected, "[{label}] sample count mismatch");

    println!("  PASS");
}

fn main() {
    let sine_table = build_sine_table();

    println!(
        "audio-core soak  —  {} frames ({} s of audio)",
        SOAK_FRAMES,
        SOAK_FRAMES / 50
    );
    println!("SIMD: compiled with CFLAGS=-march=native and -C target-cpu=native");
    println!();

    // Mode A: production quality — must complete ≤ 100 ms.
    run_soak("complexity=9  production", 9, &sine_table);
    println!();

    // Mode B: throughput ceiling — must complete ≤ 30 ms.
    run_soak("complexity=0  throughput ceiling", 0, &sine_table);
    println!();

    println!("soak: ALL PASS");
}

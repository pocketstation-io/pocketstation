/// 60-second smoke/soak test for the Phase 0 audio pipeline.
///
/// Simulates 60 seconds of continuous 20 ms audio frames (3000 frames) through
/// the full acquire → fill → encode_into → decode_slice_into → release cycle.
/// Reports pool failures, dropped frames, and elapsed wall time.
///
/// To run: `cargo run -p pocketstation-audio --example soak`
///
/// Criterion benchmarks: workspace already has `criterion = "0.5"` in
/// [workspace.dependencies]. Next step for Phase 1:
///   1. Add `[[bench]]` sections to relevant crate Cargo.tomls.
///   2. Create `benches/` directories with criterion harnesses.
///   3. Wire `cargo bench -p pocketstation-audio` in CI.
use pocketstation_audio::{
    frame_bus, AudioBufferPool, AudioFrame, MockOpusDecoder, MockOpusEncoder, SourceId, StreamId,
    DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS,
};
use std::time::Instant;

const SOAK_DURATION_SECS: u64 = 60;
const FRAMES_PER_SEC: u64 = 1_000 / 20; // 20 ms frames
const TOTAL_FRAMES: u64 = SOAK_DURATION_SECS * FRAMES_PER_SEC;

fn main() {
    println!("soak: running {} frames ({} seconds of audio)", TOTAL_FRAMES, SOAK_DURATION_SECS);

    let pool = AudioBufferPool::new(64, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let (mut prod, mut cons) = frame_bus(64);
    let mut encoder = MockOpusEncoder;
    let mut decoder = MockOpusDecoder;

    let mut encode_buf: Vec<u8> = Vec::with_capacity(DEFAULT_SLOT_SAMPLES_MONO_20MS * 4);
    let mut decode_buf: Vec<f32> = Vec::with_capacity(DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let mut total_samples_decoded: u64 = 0;

    let start = Instant::now();

    for seq in 0..TOTAL_FRAMES {
        // Produce
        let mut handle = pool.acquire().expect("pool exhausted during soak");
        let samples = handle.as_mut_slice();
        for (i, s) in samples.iter_mut().enumerate() {
            let t = (seq * DEFAULT_SLOT_SAMPLES_MONO_20MS as u64 + i as u64) as f32
                / DEFAULT_SAMPLE_RATE as f32;
            *s = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.25;
        }
        let frame = AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, handle);
        let _ = prod.push_drop_newest(frame);

        // Consume one frame per produce cycle to keep the ring from filling.
        if let Some(frame) = cons.pop() {
            encode_buf.clear();
            let n_enc = encoder.encode_into(&frame, &mut encode_buf);
            drop(frame);
            decode_buf.clear();
            let n_dec = decoder.decode_slice_into(&encode_buf[..n_enc], &mut decode_buf);
            total_samples_decoded += n_dec as u64;
        }
    }

    // Drain any remaining frames.
    while let Some(frame) = cons.pop() {
        encode_buf.clear();
        let n_enc = encoder.encode_into(&frame, &mut encode_buf);
        drop(frame);
        decode_buf.clear();
        let n_dec = decoder.decode_slice_into(&encode_buf[..n_enc], &mut decode_buf);
        total_samples_decoded += n_dec as u64;
    }

    let elapsed = start.elapsed();
    let dropped = prod.dropped_newest();
    let pool_failures = pool.acquire_failures();
    let expected_samples = TOTAL_FRAMES * DEFAULT_SLOT_SAMPLES_MONO_20MS as u64;

    println!("soak: elapsed        = {:.2?}", elapsed);
    println!("soak: frames         = {}", TOTAL_FRAMES);
    println!("soak: dropped_newest = {}", dropped);
    println!("soak: pool_failures  = {}", pool_failures);
    println!("soak: samples decoded = {} / {} expected", total_samples_decoded, expected_samples);

    assert_eq!(dropped, 0, "frames dropped during soak");
    assert_eq!(pool_failures, 0, "pool exhausted during soak");
    assert_eq!(total_samples_decoded, expected_samples, "sample count mismatch");

    println!("soak: PASS");
}

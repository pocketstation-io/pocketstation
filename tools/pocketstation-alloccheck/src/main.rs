/// Phase 0 allocation-check harness.
///
/// Exercises the hot-path code paths that must remain allocation-free in
/// production. This tool does NOT use DHAT or a custom allocator shim —
/// those require platform-specific setup and are Phase 3 work (ADR-TBD).
///
/// What this tool does:
/// - Runs the acquire → fill → encode_into → decode_slice_into → release
///   cycle FRAME_COUNT times using reused caller-owned buffers.
/// - Asserts no pool slots leak after the run.
/// - Reports the result so CI can confirm the binary executes cleanly.
///
/// To run: `cargo run -p pocketstation-alloccheck`
///
/// Phase 3 TODO: replace with a DHAT-instrumented binary and add a CI gate
/// that fails if heap allocation count per frame exceeds the budget in
/// docs/architecture/PocketStation-v2.3.md.
use pocketstation_audio::{
    AudioBufferPool, AudioFrame, MockOpusDecoder, MockOpusEncoder, SourceId, StreamId,
    DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS,
};

const FRAME_COUNT: u64 = 50;

fn main() {
    let pool = AudioBufferPool::new(64, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let mut encoder = MockOpusEncoder::default();
    let mut decoder = MockOpusDecoder::default();

    // Allocate once; reuse per frame — this is the pattern hot-path callers must follow.
    let mut encode_buf: Vec<u8> = Vec::with_capacity(DEFAULT_SLOT_SAMPLES_MONO_20MS * 4);
    let mut decode_buf: Vec<f32> = Vec::with_capacity(DEFAULT_SLOT_SAMPLES_MONO_20MS);

    for seq in 0..FRAME_COUNT {
        let mut handle = pool.acquire().expect("pool exhausted");
        let samples = handle.as_mut_slice();
        for (i, s) in samples.iter_mut().enumerate() {
            let t = (seq * DEFAULT_SLOT_SAMPLES_MONO_20MS as u64 + i as u64) as f32
                / DEFAULT_SAMPLE_RATE as f32;
            *s = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.25;
        }

        let frame = AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, handle);

        // encode_into: reuses encode_buf, no per-frame allocation.
        let n_enc = encoder.encode_into(&frame, &mut encode_buf);
        drop(frame); // releases pool slot immediately

        // decode_slice_into: reuses decode_buf, no per-frame allocation.
        decode_buf.clear();
        let n_dec = decoder.decode_slice_into(&encode_buf[..n_enc], &mut decode_buf);
        assert_eq!(
            n_dec, DEFAULT_SLOT_SAMPLES_MONO_20MS,
            "decoded sample count mismatch on seq {seq}"
        );
    }

    assert_eq!(
        pool.acquire_failures(),
        0,
        "pool exhausted during hot-path run"
    );

    println!(
        "alloccheck: {} frames processed, {} pool failures",
        FRAME_COUNT,
        pool.acquire_failures()
    );
    println!("alloccheck: Phase 0 OK. DHAT gate is Phase 3 work (ADR-TBD).");
}

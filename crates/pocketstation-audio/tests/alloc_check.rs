// Verifies zero heap allocation on the audio hot path — Phase 0 exit criterion.
//
// Uses the `assert_no_alloc` crate (assert_no_alloc = "1.1") which panics if
// any heap allocation occurs inside the guarded closure.  The `AllocDisabler`
// global allocator replaces the NIH counting allocator previously in
// `tools/pocketstation-alloccheck`.

use assert_no_alloc::*;
use pocketstation_audio::{
    frame_bus, AudioBufferPool, AudioFrame, OpusDecoder, OpusEncoder, SourceId, StreamId,
    DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS, OPUS_MAX_PACKET_BYTES,
};

#[cfg(test)]
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

/// Given the audio hot path (pool acquire → ring push → encode → ring pop → decode),
/// when 100 consecutive frames are processed,
/// then zero heap allocations must occur.
#[test]
fn given_hot_path_when_100_frames_then_zero_heap_allocs() {
    let pool = AudioBufferPool::new(64, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let (mut prod, mut cons) = frame_bus(64);
    let mut encoder = OpusEncoder::default();
    let mut decoder = OpusDecoder::default();

    let pcm: Vec<f32> = build_sine(0, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let mut encode_buf: Vec<u8> = Vec::with_capacity(OPUS_MAX_PACKET_BYTES);
    let mut decode_buf: Vec<f32> = Vec::with_capacity(DEFAULT_SLOT_SAMPLES_MONO_20MS);

    // Warm up 10 frames to exhaust libopus lazy-init paths before the gate opens.
    for wu in 0u64..10 {
        let mut h = pool.acquire().expect("pool exhausted on warmup");
        h.copy_from_slice(&pcm);
        let frame = AudioFrame::new(StreamId(1), SourceId(1), wu, wu * 20_000_000, 1, h);
        let _ = prod.push_drop_newest(frame);
        if let Some(f) = cons.pop() {
            encode_buf.clear();
            let n = encoder
                .encode_into(f.buffer.as_slice(), &mut encode_buf)
                .unwrap();
            drop(f);
            decode_buf.clear();
            decoder
                .decode_into(&encode_buf[..n], &mut decode_buf)
                .unwrap();
        }
    }

    assert_no_alloc(|| {
        for seq in 1u64..=100 {
            let mut h = pool.acquire().expect("pool exhausted on hot path");
            h.copy_from_slice(&pcm);
            let frame = AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, h);
            let _ = prod.push_drop_newest(frame);
            if let Some(f) = cons.pop() {
                encode_buf.clear();
                let n = encoder
                    .encode_into(f.buffer.as_slice(), &mut encode_buf)
                    .unwrap();
                drop(f);
                decode_buf.clear();
                decoder
                    .decode_into(&encode_buf[..n], &mut decode_buf)
                    .unwrap();
            }
        }
    });

    assert_eq!(pool.acquire_failures(), 0);
    assert_eq!(prod.dropped_newest(), 0);
}

fn build_sine(start: u64, len: usize) -> Vec<f32> {
    (0..len)
        .map(|i| {
            let t = (start + i as u64) as f32 / DEFAULT_SAMPLE_RATE as f32;
            (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.25
        })
        .collect()
}

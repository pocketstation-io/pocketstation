// Verifies zero heap allocation on the audio hot path.
//
// Uses the `assert_no_alloc` crate (assert_no_alloc = "1.1") which panics if
// any heap allocation occurs inside the guarded closure.  The `AllocDisabler`
// global allocator replaces the NIH counting allocator previously in
// `tools/pocketstation-alloccheck`.

use assert_no_alloc::*;
use pocketstation::internal::codec::{
    OpusDecoder, OpusEncoder, OpusFrameDuration, OPUS_FRAME_SAMPLES, OPUS_MAX_PACKET_BYTES,
};
use pocketstation::internal::frame::{AudioBufferPool, POOL_SLOT_SAMPLES, SAMPLE_RATE_HZ};

#[cfg(test)]
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

/// Given the codec hot path (pool acquire → encode → decode),
/// when 100 consecutive frames are processed,
/// then zero heap allocations must occur.
#[test]
fn given_hot_path_when_100_frames_then_zero_heap_allocs() {
    let pool = AudioBufferPool::new(64, POOL_SLOT_SAMPLES);
    let mut encoder = OpusEncoder::default();
    let mut decoder = OpusDecoder::default();

    let pcm: Vec<f32> = build_sine(0, POOL_SLOT_SAMPLES);
    let mut encode_buf: Vec<u8> = Vec::with_capacity(OPUS_MAX_PACKET_BYTES);
    let mut decode_buf: Vec<f32> = Vec::with_capacity(POOL_SLOT_SAMPLES);

    // Warm up 10 frames to exhaust libopus lazy-init paths before the gate opens.
    for _ in 0..10 {
        let mut h = pool.acquire().expect("pool exhausted on warmup");
        h.try_copy_from_slice(&pcm).unwrap();
        encode_buf.clear();
        let n = encoder.encode_into(h.as_slice(), &mut encode_buf).unwrap();
        drop(h);
        decode_buf.clear();
        decoder
            .decode_into(&encode_buf[..n], &mut decode_buf, false)
            .unwrap();
    }

    assert_no_alloc(|| {
        for _ in 0..100 {
            let mut h = pool.acquire().expect("pool exhausted on hot path");
            h.try_copy_from_slice(&pcm).unwrap();
            encode_buf.clear();
            let n = encoder.encode_into(h.as_slice(), &mut encode_buf).unwrap();
            drop(h);
            decode_buf.clear();
            decoder
                .decode_into(&encode_buf[..n], &mut decode_buf, false)
                .unwrap();
        }
    });

    assert_eq!(pool.acquire_failures(), 0);
}

/// Packet-loss concealment is stateful decoder work and must remain usable on
/// a preallocated codec worker without a recovery-path heap allocation.
#[test]
fn given_preallocated_decoder_when_100_packets_are_concealed_then_zero_heap_allocs() {
    let mut decoder = OpusDecoder::default();
    let mut output = Vec::with_capacity(OPUS_FRAME_SAMPLES);

    decoder
        .decode_plc_into(OpusFrameDuration::Ms20, &mut output)
        .expect("PLC warmup");
    output.clear();

    assert_no_alloc(|| {
        for _ in 0..100 {
            decoder
                .decode_plc_into(OpusFrameDuration::Ms20, &mut output)
                .expect("preallocated PLC");
            assert_eq!(output.len(), OPUS_FRAME_SAMPLES);
            output.clear();
        }
    });
}

fn build_sine(start: u64, len: usize) -> Vec<f32> {
    (0..len)
        .map(|i| {
            let t = (start + i as u64) as f32 / SAMPLE_RATE_HZ as f32;
            (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.25
        })
        .collect()
}

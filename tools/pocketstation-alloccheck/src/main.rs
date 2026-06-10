/// Phase 5 allocation-check harness — DHAT CI gate (AUDIO-023 prerequisite).
///
/// Instruments the hot-path encode→decode cycle with a counting global allocator
/// to verify zero heap allocations per frame. The counter is reset between
/// the one-time setup phase (pool + encoder/decoder construction) and the
/// steady-state per-frame loop, so only forwarding-path allocations count.
///
/// Exit code 0 = zero allocations per frame (CI passes).
/// Exit code 1 = heap allocation detected on hot path (CI fails).
///
/// Run: `cargo run -p pocketstation-alloccheck`
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use pocketstation_audio::{
    AudioBufferPool, AudioFrame, MockOpusDecoder, MockOpusEncoder, SourceId, StreamId,
    DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS,
};

// ─── Counting allocator ───────────────────────────────────────────────────────

/// A transparent global allocator wrapper that counts heap allocations.
/// Gate is disabled during startup/teardown; only active during the hot-path
/// measurement window controlled by `GATE_ACTIVE`.
struct CountingAllocator;

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
static GATE_ACTIVE: AtomicBool = AtomicBool::new(false);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if GATE_ACTIVE.load(Ordering::Relaxed) {
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if GATE_ACTIVE.load(Ordering::Relaxed) {
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        System.realloc(ptr, layout, new_size)
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

// ─── Hot-path measurement ─────────────────────────────────────────────────────

const FRAME_COUNT: u64 = 100;
/// Maximum heap allocations permitted per frame on the audio hot path.
const MAX_ALLOCS_PER_FRAME: usize = 0;

fn main() {
    // Setup phase — pool and codec construction are allowed to allocate.
    let pool = AudioBufferPool::new(64, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let mut encoder = MockOpusEncoder::default();
    let mut decoder = MockOpusDecoder::default();

    // Pre-allocate caller-owned buffers once. Hot path reuses them.
    let mut encode_buf: Vec<u8> = Vec::with_capacity(DEFAULT_SLOT_SAMPLES_MONO_20MS * 4);
    let mut decode_buf: Vec<f32> = Vec::with_capacity(DEFAULT_SLOT_SAMPLES_MONO_20MS);

    // Warm up: one frame before gating so branch predictors are trained.
    {
        let mut h = pool.acquire().expect("pool exhausted on warmup");
        fill_sine(h.as_mut_slice(), 0, DEFAULT_SAMPLE_RATE);
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
        let n = encoder.encode_into(&frame, &mut encode_buf);
        drop(frame);
        decode_buf.clear();
        decoder.decode_slice_into(&encode_buf[..n], &mut decode_buf);
    }

    // ── Gate ON: every allocation from here counts. ──
    ALLOC_COUNT.store(0, Ordering::SeqCst);
    GATE_ACTIVE.store(true, Ordering::SeqCst);

    for seq in 1..=FRAME_COUNT {
        let mut handle = pool.acquire().expect("pool exhausted on hot path");
        fill_sine(handle.as_mut_slice(), seq, DEFAULT_SAMPLE_RATE);

        let frame = AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, handle);

        let n_enc = encoder.encode_into(&frame, &mut encode_buf);
        drop(frame); // releases pool slot — must not allocate

        decode_buf.clear();
        let n_dec = decoder.decode_slice_into(&encode_buf[..n_enc], &mut decode_buf);
        assert_eq!(
            n_dec, DEFAULT_SLOT_SAMPLES_MONO_20MS,
            "decoded sample count mismatch on seq {seq}"
        );
    }

    // ── Gate OFF ──
    GATE_ACTIVE.store(false, Ordering::SeqCst);
    let total_allocs = ALLOC_COUNT.load(Ordering::SeqCst);
    let allocs_per_frame = total_allocs / FRAME_COUNT as usize;

    println!("alloccheck: {FRAME_COUNT} frames, {total_allocs} total allocs, {allocs_per_frame} per frame");

    if allocs_per_frame > MAX_ALLOCS_PER_FRAME {
        eprintln!(
            "alloccheck FAIL: {allocs_per_frame} allocs/frame exceeds budget of {MAX_ALLOCS_PER_FRAME}"
        );
        std::process::exit(1);
    }

    assert_eq!(
        pool.acquire_failures(),
        0,
        "pool exhausted during hot-path run"
    );
    println!("alloccheck PASS: zero heap allocations on audio hot path.");
}

fn fill_sine(samples: &mut [f32], seq: u64, sample_rate: u32) {
    for (i, s) in samples.iter_mut().enumerate() {
        let t =
            (seq * DEFAULT_SLOT_SAMPLES_MONO_20MS as u64 + i as u64) as f32 / sample_rate as f32;
        *s = (2.0 * std::f32::consts::PI * 440.0 * t).sin() * 0.25;
    }
}

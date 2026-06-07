// Verifies zero heap allocation on the audio hot path — Phase 0 exit criterion.
//
// A counting global allocator is gated by an atomic flag. The flag is off during
// setup (pool + encoder/decoder construction), then switched on for exactly 100
// frames of the steady-state loop, then switched off again. Any allocation that
// fires while the gate is open is counted; the test asserts the count is zero.
//
// This is an integration test (separate binary) so it can declare its own
// #[global_allocator] without conflicting with any other test binary or the
// pocketstation-alloccheck tool binary.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use pocketstation_audio::{
    frame_bus, AudioBufferPool, AudioFrame, MockOpusDecoder, MockOpusEncoder, SourceId, StreamId,
    DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS, OPUS_MAX_PACKET_BYTES,
};

// ── Counting allocator ────────────────────────────────────────────────────────

struct CountingAllocator;

static GATE_ACTIVE: AtomicBool = AtomicBool::new(false);
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

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

// ── Constants ─────────────────────────────────────────────────────────────────

/// Number of steady-state frames used for the allocation measurement window.
const MEASURED_FRAMES: u64 = 100;

/// Maximum heap allocations permitted across all measured frames.
const MAX_TOTAL_ALLOCS: usize = 0;

// ── Test ──────────────────────────────────────────────────────────────────────

/// Given the audio hot path (pool acquire → ring push → encode → ring pop → decode),
/// when 100 consecutive frames are processed,
/// then zero heap allocations must occur.
#[test]
fn given_hot_path_when_100_frames_then_zero_heap_allocs() {
    // Setup phase — allocations are allowed; gate is off.
    let pool = AudioBufferPool::new(64, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let (mut prod, mut cons) = frame_bus(64);
    let mut encoder = MockOpusEncoder::default();
    let mut decoder = MockOpusDecoder::default();

    // Pre-allocate caller-side buffers once; the hot path only reuses them.
    // encode_buf must have at least OPUS_MAX_PACKET_BYTES capacity so that
    // OpusEncoder::encode_into never grows it during the measurement window.
    // decode_buf must have at least OPUS_FRAME_SAMPLES capacity so that
    // OpusDecoder::decode_into never grows it during the measurement window.
    let pcm: Vec<f32> = build_sine(0, DEFAULT_SLOT_SAMPLES_MONO_20MS);
    let mut encode_buf: Vec<u8> = Vec::with_capacity(OPUS_MAX_PACKET_BYTES);
    let mut decode_buf: Vec<f32> = Vec::with_capacity(DEFAULT_SLOT_SAMPLES_MONO_20MS);

    // Warm-up: 10 frames to prime branch predictors, ring internals, and
    // libopus internal SILK decoder state (which allocates lazily on first
    // few frames on Linux glibc; 10 frames is sufficient to exhaust all
    // lazy-init paths before the allocation gate opens).
    for wu in 0u64..10 {
        let mut handle = pool.acquire().expect("pool exhausted on warmup");
        handle.copy_from_slice(&pcm);
        let frame = AudioFrame::new(StreamId(1), SourceId(1), wu, wu * 20_000_000, 1, handle);
        let _ = prod.push_drop_newest(frame);
        if let Some(f) = cons.pop() {
            encode_buf.clear();
            let n = encoder.encode_into(&f, &mut encode_buf);
            drop(f);
            decode_buf.clear();
            decoder.decode_slice_into(&encode_buf[..n], &mut decode_buf);
        }
    }

    // ── Gate ON: every allocation from here counts. ──────────────────────────
    ALLOC_COUNT.store(0, Ordering::SeqCst);
    GATE_ACTIVE.store(true, Ordering::SeqCst);

    for seq in 1..=MEASURED_FRAMES {
        // Acquire a pool slot — must not allocate; pool was pre-populated.
        let mut handle = pool.acquire().expect("pool exhausted on hot path");
        handle.copy_from_slice(&pcm);

        let frame = AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, handle);

        // Ring push — rtrb uses a fixed-capacity ring; no allocation on push.
        let _ = prod.push_drop_newest(frame);

        // Encode into pre-allocated buffer.
        if let Some(f) = cons.pop() {
            encode_buf.clear();
            let n_enc = encoder.encode_into(&f, &mut encode_buf);
            // Drop releases the pool slot without allocating.
            drop(f);

            // Decode into pre-sized buffer.
            decode_buf.clear();
            decoder.decode_slice_into(&encode_buf[..n_enc], &mut decode_buf);
        }
    }

    // ── Gate OFF ─────────────────────────────────────────────────────────────
    GATE_ACTIVE.store(false, Ordering::SeqCst);
    let total_allocs = ALLOC_COUNT.load(Ordering::SeqCst);

    assert_eq!(
        total_allocs, MAX_TOTAL_ALLOCS,
        "hot path must be allocation-free: {total_allocs} heap allocation(s) detected over \
         {MEASURED_FRAMES} frames"
    );
    assert_eq!(
        pool.acquire_failures(),
        0,
        "pool must not be exhausted during the hot-path measurement window"
    );
    assert_eq!(
        prod.dropped_newest(),
        0,
        "ring must not drop frames during the hot-path measurement window"
    );
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn build_sine(start_sample: u64, len: usize) -> Vec<f32> {
    use std::f32::consts::PI;
    (0..len)
        .map(|i| {
            let t = (start_sample + i as u64) as f32 / DEFAULT_SAMPLE_RATE as f32;
            (2.0 * PI * 440.0 * t).sin() * 0.25
        })
        .collect()
}

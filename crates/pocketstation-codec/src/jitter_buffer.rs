use std::collections::VecDeque;

use crate::encoder::EncodedFrame;

// ---------------------------------------------------------------------------
// Adaptive constants
// ---------------------------------------------------------------------------

/// EWMA smoothing factor for inter-arrival jitter (alpha = 0.125 = 1/8).
/// Represented as a shift: new_ewma = (7 * old + sample) >> 3.
const EWMA_SHIFT: u32 = 3;

/// Adaptation runs every this many frames (~1 second at 20 ms/frame).
const ADAPT_INTERVAL: u64 = 50;

/// 20 ms frame duration in nanoseconds — used to convert jitter → frame count.
const FRAME_DURATION_NS: u64 = 20_000_000;

/// Safety margin added on top of the measured jitter target (frames).
const ADAPT_SAFETY_MARGIN: usize = 1;

/// Result of a [`JitterBuffer::pop`] call.
#[derive(Debug)]
pub enum PopResult {
    /// A frame is ready and is returned.
    Frame(EncodedFrame),
    /// A gap in the sequence was detected; the caller should synthesize silence
    /// or perform PLC.  `gap_count` is the number of missing sequence numbers.
    GapDetected { gap_count: u64 },
    /// The buffer has not yet accumulated `target_depth` frames.
    NotReady,
}

/// Adaptive jitter buffer (AUDIO-010 / architecture §26.7 and §13.3).
///
/// Owns 60 ms of the 170 ms transport-P95 budget:
/// - P50 target ≈ 20 ms (1 frame)
/// - P95 target ≤ 60 ms (3 frames at 20 ms)
///
/// # Design constraints
/// - No heap allocation in `push` / `pop` after construction.
/// - No locks (single-threaded usage in `ProcessorGraph`).
/// - No logging.
/// - No blocking.
///
/// # Adaptive algorithm
/// Inter-arrival jitter is tracked as an EWMA (α = 0.125).  Every
/// [`ADAPT_INTERVAL`] frames the target depth is recomputed:
/// ```text
/// new_target = ceil(ewma_ns / FRAME_DURATION_NS) + ADAPT_SAFETY_MARGIN
/// new_target = clamp(new_target, min_depth, max_depth)
/// ```
///
/// The depth grows when `consecutive_empty` pops accumulate (starvation
/// signal) and shrinks naturally via the jitter EWMA falling.
pub struct JitterBuffer {
    // Configuration
    min_depth: usize,
    max_depth: usize,
    target_depth: usize,

    // Storage — bounded at max_depth; no allocation after construction.
    queue: VecDeque<EncodedFrame>,

    // Sequence tracking
    next_expected_seq: Option<u64>,

    // Adaptive state — tracks inter-arrival time between consecutive pushes.
    last_arrival_ns: u64,
    inter_arrival_ewma_ns: u64,
    consecutive_empty: u32,

    // Frame counter driving the adaptation interval
    frames_since_adapt: u64,

    // Metrics
    late_frames: u64,
    concealed_frames: u64,
    total_frames: u64,
}

impl JitterBuffer {
    /// Create a new adaptive jitter buffer.
    ///
    /// - `min_depth`: minimum frames buffered before any frame is released
    ///   (default recommendation: 1).
    /// - `max_depth`: hard upper bound on buffered frames — frames arriving
    ///   when the queue is full are silently dropped (default: 8 = 160 ms).
    ///
    /// `target_depth` starts at `min_depth + 1` (2 when min=1), clamped to
    /// `[min_depth, max_depth]`.
    ///
    /// # Panics
    /// Panics in debug builds if `min_depth == 0` or `min_depth > max_depth`.
    pub fn new(min_depth: usize, max_depth: usize) -> Self {
        debug_assert!(min_depth > 0, "min_depth must be at least 1");
        debug_assert!(min_depth <= max_depth, "min_depth must be ≤ max_depth");
        let target_depth = (min_depth + 1).min(max_depth);
        Self {
            min_depth,
            max_depth,
            target_depth,
            queue: VecDeque::with_capacity(max_depth),
            next_expected_seq: None,
            last_arrival_ns: 0,
            inter_arrival_ewma_ns: FRAME_DURATION_NS,
            consecutive_empty: 0,
            frames_since_adapt: 0,
            late_frames: 0,
            concealed_frames: 0,
            total_frames: 0,
        }
    }

    /// Push an encoded frame into the buffer.
    ///
    /// Frames are inserted in sequence order (ordered insertion).  Frames
    /// arriving after they have already been delivered (sequence number <
    /// `next_expected_seq`) are discarded and counted as late.  Duplicate
    /// sequence numbers already present in the queue are silently dropped.
    ///
    /// Frames arriving when the queue is already at `max_depth` are dropped
    /// (the oldest frame is not evicted — caller controls pacing).
    ///
    /// No heap allocation occurs after the first call if the queue was
    /// pre-allocated at construction time (it is, via `with_capacity`).
    pub fn push(&mut self, frame: EncodedFrame) {
        // Update inter-arrival EWMA using the frame's own timestamp.
        // On the very first frame there is no prior timestamp, so we skip.
        if self.last_arrival_ns > 0 {
            let inter_arrival = frame.timestamp_ns.saturating_sub(self.last_arrival_ns);
            // EWMA: new = (7*old + sample) / 8
            self.inter_arrival_ewma_ns =
                ((self.inter_arrival_ewma_ns * 7) + inter_arrival) >> EWMA_SHIFT;
        }
        self.last_arrival_ns = frame.timestamp_ns;

        // Discard frames already delivered (too late) or duplicates in flight.
        if let Some(expected) = self.next_expected_seq {
            if frame.sequence_number < expected {
                self.late_frames += 1;
                return;
            }
        }
        if self
            .queue
            .iter()
            .any(|f| f.sequence_number == frame.sequence_number)
        {
            // Duplicate already in queue — silent drop (not a late frame).
            return;
        }

        // Enforce max_depth — drop if full.
        if self.queue.len() >= self.max_depth {
            return;
        }

        // Insert in sequence order (small VecDeque, insertion sort is fine).
        let pos = self
            .queue
            .partition_point(|f| f.sequence_number < frame.sequence_number);
        self.queue.insert(pos, frame);
    }

    /// Pop the next frame, applying adaptive depth and sequence-gap detection.
    ///
    /// Returns:
    /// - [`PopResult::Frame`] — a frame is ready.
    /// - [`PopResult::GapDetected`] — the next expected sequence number is
    ///   missing; the caller should synthesize silence or do PLC.  The gap is
    ///   consumed (next call will attempt the frame after the gap).
    /// - [`PopResult::NotReady`] — fewer than `target_depth` frames are
    ///   buffered; caller should wait for more data.
    pub fn pop(&mut self) -> PopResult {
        // Not enough frames buffered yet.
        if self.queue.len() < self.target_depth {
            self.consecutive_empty += 1;
            self.maybe_adapt();
            return PopResult::NotReady;
        }

        // Peek at the head frame to check for a sequence gap.
        let head_seq = self.queue.front().map(|f| f.sequence_number);

        if let (Some(head_seq), Some(expected)) = (head_seq, self.next_expected_seq) {
            if head_seq > expected {
                // Gap: frame(s) are missing before the head.
                let gap_count = head_seq - expected;
                // Advance the expected pointer past the gap so the next pop
                // attempts to deliver head_seq.
                self.next_expected_seq = Some(head_seq);
                self.concealed_frames += gap_count;
                self.consecutive_empty = 0;
                self.maybe_adapt();
                return PopResult::GapDetected { gap_count };
            }
        }

        // Deliver the frame.
        if let Some(frame) = self.queue.pop_front() {
            self.next_expected_seq = Some(frame.sequence_number + 1);
            self.total_frames += 1;
            self.consecutive_empty = 0;
            self.frames_since_adapt += 1;
            self.maybe_adapt();
            return PopResult::Frame(frame);
        }

        // Queue became empty between the depth check and pop (should not
        // happen in single-threaded use, but be defensive).
        self.consecutive_empty += 1;
        self.maybe_adapt();
        PopResult::NotReady
    }

    /// Backward-compatible alias for [`pop`].
    ///
    /// Returns `Some(frame)` when a frame is ready, `None` otherwise.
    /// Gap signals are collapsed to `None` — use [`pop`] directly to receive
    /// [`PopResult::GapDetected`] for proper PLC handling.
    #[deprecated(note = "Use pop() to receive GapDetected for proper PLC handling")]
    pub fn pop_ready(&mut self) -> Option<EncodedFrame> {
        match self.pop() {
            PopResult::Frame(f) => Some(f),
            PopResult::GapDetected { .. } | PopResult::NotReady => None,
        }
    }

    /// Returns `true` when the head of the queue has a sequence number that
    /// is not contiguous with `next_expected_seq` (gap detected ahead).
    pub fn sequence_gap_ahead(&self) -> bool {
        let head_seq = match self.queue.front() {
            Some(f) => f.sequence_number,
            None => return false,
        };
        match self.next_expected_seq {
            Some(expected) => head_seq > expected,
            // No frame has been popped yet: compare adjacent queue entries
            // (Phase 0 fallback so existing tests pass before any pop).
            None => {
                if self.queue.len() < 2 {
                    return false;
                }
                let a = self.queue[0].sequence_number;
                let b = self.queue[1].sequence_number;
                b != a + 1
            }
        }
    }

    /// Number of frames currently in the buffer.
    pub fn depth(&self) -> usize {
        self.queue.len()
    }

    /// Current adaptive target depth (in frames).
    pub fn target_depth(&self) -> usize {
        self.target_depth
    }

    /// Total late frames received (arrived with seq < expected, discarded).
    pub fn late_frames(&self) -> u64 {
        self.late_frames
    }

    /// Total frames for which PLC/silence was synthesized (gap-detected pops).
    pub fn concealed_frames(&self) -> u64 {
        self.concealed_frames
    }

    /// Total frames successfully delivered via [`pop`].
    pub fn total_frames(&self) -> u64 {
        self.total_frames
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    /// Run the adaptive depth algorithm if the adaptation interval has elapsed
    /// or if starvation is detected.
    ///
    /// Called at the end of every `pop()` — no allocation, no branching on the
    /// hot path when the interval has not elapsed.
    fn maybe_adapt(&mut self) {
        // Grow immediately on starvation (two consecutive empty pops).
        if self.consecutive_empty >= 2 {
            if self.target_depth < self.max_depth {
                self.target_depth += 1;
            }
            self.consecutive_empty = 0;
            self.frames_since_adapt = 0;
            return;
        }

        if self.frames_since_adapt < ADAPT_INTERVAL {
            return;
        }
        self.frames_since_adapt = 0;

        // Recompute target from measured jitter EWMA.
        let jitter_frames = self.inter_arrival_ewma_ns.div_ceil(FRAME_DURATION_NS);
        let new_target = (jitter_frames as usize + ADAPT_SAFETY_MARGIN)
            .max(self.min_depth)
            .min(self.max_depth);
        self.target_depth = new_target;
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    fn make_encoded(seq: u64) -> EncodedFrame {
        EncodedFrame {
            sequence_number: seq,
            timestamp_ns: seq * 20_000_000,
            payload: vec![],
        }
    }

    fn make_encoded_with_ts(seq: u64, ts_ns: u64) -> EncodedFrame {
        EncodedFrame {
            sequence_number: seq,
            timestamp_ns: ts_ns,
            payload: vec![],
        }
    }

    #[test]
    fn jitter_buffer_withholds_frames_until_target_depth_is_reached() {
        // Given: min=3, max=3 so buffer releases as soon as 3 frames arrive
        let mut jb = JitterBuffer::new(3, 3);

        // When / Then (interleaved)
        jb.push(make_encoded(0));
        assert!(jb.pop_ready().is_none());
        jb.push(make_encoded(1));
        assert!(jb.pop_ready().is_none());
        jb.push(make_encoded(2));
        assert!(jb.pop_ready().is_some());
    }

    #[test]
    fn jitter_buffer_ordered_frames_pop_in_fifo_order() {
        // Given: min=1, max=8 → target_depth=2.
        let mut jb = JitterBuffer::new(1, 8);
        const POP_COUNT: u64 = 4;

        for seq in 0..POP_COUNT + 2 {
            jb.push(make_encoded(seq));
        }

        // When / Then: all POP_COUNT pops return frames in order
        for expected in 0..POP_COUNT {
            let frame = jb.pop_ready().unwrap();
            assert_eq!(frame.sequence_number, expected);
        }
        assert!(jb.pop_ready().is_some());
        assert!(jb.pop_ready().is_none());
    }

    #[test]
    fn given_out_of_order_arrival_when_depth_met_then_delivered_in_sequence_order() {
        // Given: seq 1 arrives before seq 0 — ordered insertion must fix this.
        let mut jb = JitterBuffer::new(1, 8);
        jb.push(make_encoded(1));
        jb.push(make_encoded(0)); // late arrival, but before any pop
        jb.push(make_encoded(2)); // fill to target_depth=2

        // When / Then: seq 0 must come out first
        let first = match jb.pop() {
            PopResult::Frame(f) => f,
            other => panic!("expected Frame, got {:?}", other),
        };
        assert_eq!(
            first.sequence_number, 0,
            "ordered insertion must deliver seq 0 before seq 1"
        );
    }

    #[test]
    fn jitter_buffer_detects_sequence_gap_when_frame_is_missing() {
        // Given: min=1 max=8 → target_depth=2; push 2 frames with a gap
        let mut jb = JitterBuffer::new(1, 8);
        jb.push(make_encoded(0));
        jb.push(make_encoded(2)); // seq 1 is missing

        assert!(
            jb.sequence_gap_ahead(),
            "gap between seq 0 and seq 2 should be detected"
        );
        let first = jb.pop_ready().unwrap();
        assert_eq!(first.sequence_number, 0);
    }

    #[test]
    fn jitter_buffer_contiguous_frames_report_no_gap() {
        let mut jb = JitterBuffer::new(1, 8);
        jb.push(make_encoded(5));
        jb.push(make_encoded(6));
        assert!(!jb.sequence_gap_ahead());
    }

    #[test]
    fn given_min1_max8_target2_when_2_frames_pushed_then_pop_returns_frame() {
        // Given: min=1, max=8 → target_depth starts at 2
        let mut jb = JitterBuffer::new(1, 8);
        assert_eq!(jb.target_depth(), 2);

        jb.push(make_encoded(1));
        jb.push(make_encoded(2));

        assert!(
            matches!(jb.pop(), PopResult::Frame(_)),
            "expected PopResult::Frame when depth == target_depth"
        );
    }

    #[test]
    fn given_target_depth_2_when_only_1_frame_pushed_then_pop_returns_not_ready() {
        let mut jb = JitterBuffer::new(1, 8);
        jb.push(make_encoded(1));
        assert!(
            matches!(jb.pop(), PopResult::NotReady),
            "expected PopResult::NotReady when depth < target_depth"
        );
    }

    #[test]
    fn given_seq_gap_when_pop_reaches_missing_seq_then_gap_detected_returned() {
        let mut jb = JitterBuffer::new(1, 8);
        jb.push(make_encoded(1));
        jb.push(make_encoded(2));
        jb.push(make_encoded(4)); // seq 3 missing

        let r1 = jb.pop();
        assert!(matches!(r1, PopResult::Frame(ref f) if f.sequence_number == 1));

        let r2 = jb.pop();
        assert!(matches!(r2, PopResult::Frame(ref f) if f.sequence_number == 2));

        jb.push(make_encoded(5));
        let r3 = jb.pop();
        assert!(
            matches!(r3, PopResult::GapDetected { gap_count: 1 }),
            "expected GapDetected{{gap_count:1}} at seq 3, got {:?}",
            r3
        );
    }

    #[test]
    fn given_50_uniform_frames_when_adapt_runs_then_target_stays_at_min() {
        let mut jb = JitterBuffer::new(1, 8);

        jb.push(EncodedFrame {
            sequence_number: 0,
            timestamp_ns: 0,
            payload: vec![],
        });
        jb.push(EncodedFrame {
            sequence_number: 1,
            timestamp_ns: FRAME_DURATION_NS,
            payload: vec![],
        });

        for i in 2u64..52 {
            jb.push(EncodedFrame {
                sequence_number: i,
                timestamp_ns: i * FRAME_DURATION_NS,
                payload: vec![],
            });
            jb.pop();
        }

        assert!(
            jb.target_depth() <= 2,
            "uniform arrivals should keep target_depth ≤ 2, got {}",
            jb.target_depth()
        );
    }

    #[test]
    fn given_high_inter_arrival_jitter_when_adapt_runs_then_target_increases() {
        let inter_arrival_ns: u64 = 40_000_000;
        let mut jb = JitterBuffer::new(1, 8);

        jb.push(EncodedFrame {
            sequence_number: 0,
            timestamp_ns: 0,
            payload: vec![],
        });
        jb.push(EncodedFrame {
            sequence_number: 1,
            timestamp_ns: inter_arrival_ns,
            payload: vec![],
        });

        for i in 2u64..52 {
            jb.push(EncodedFrame {
                sequence_number: i,
                timestamp_ns: i * inter_arrival_ns,
                payload: vec![],
            });
            jb.pop();
        }

        assert!(
            jb.target_depth() >= 3,
            "high jitter (40ms inter-arrival) should push target_depth to ≥ 3, got {}",
            jb.target_depth()
        );
    }

    #[test]
    fn given_duplicate_seq_when_pushed_twice_then_only_one_delivered() {
        let mut jb = JitterBuffer::new(1, 8);
        jb.push(make_encoded(5));
        jb.push(make_encoded(5)); // duplicate — must be silently dropped
        jb.push(make_encoded(6)); // fill to target_depth=2

        // First pop delivers seq 5.
        let first = match jb.pop() {
            PopResult::Frame(f) => f,
            other => panic!("expected Frame, got {:?}", other),
        };
        assert_eq!(first.sequence_number, 5);

        // After seq 5 is delivered, queue has only [6] (depth=1 < target_depth=2).
        // Push seq 7 to satisfy target_depth before the second pop.
        jb.push(make_encoded(7));

        let second = match jb.pop() {
            PopResult::Frame(f) => f,
            other => panic!("expected Frame, got {:?}", other),
        };
        assert_eq!(second.sequence_number, 6);
        assert_eq!(
            jb.depth(),
            1,
            "only seq 7 remains; duplicate must not inflate queue depth"
        );
    }

    #[test]
    fn given_burst_loss_of_5_when_pop_then_gap_count_is_5() {
        // seq 0, burst loss of seq 1-5, then seq 6
        let mut jb = JitterBuffer::new(1, 8);
        jb.push(make_encoded(0));
        jb.push(make_encoded(6)); // gap_count = 5 (seq 1-5 missing)
        jb.push(make_encoded(7));

        // Deliver seq 0
        assert!(matches!(jb.pop(), PopResult::Frame(ref f) if f.sequence_number == 0));

        // Next pop should signal the gap
        jb.push(make_encoded(8));
        let gap = jb.pop();
        assert!(
            matches!(gap, PopResult::GapDetected { gap_count: 5 }),
            "expected GapDetected{{gap_count:5}}, got {:?}",
            gap
        );
    }

    #[test]
    fn given_very_late_frame_when_pushed_after_delivery_then_discarded_and_late_count_incremented()
    {
        let mut jb = JitterBuffer::new(1, 8);
        // Push and deliver seq 0, 1
        for i in 0u64..4 {
            jb.push(make_encoded(i));
        }
        assert!(matches!(jb.pop(), PopResult::Frame(ref f) if f.sequence_number == 0));
        assert!(matches!(jb.pop(), PopResult::Frame(ref f) if f.sequence_number == 1));

        // Now push seq 0 again (very late)
        let late_before = jb.late_frames();
        jb.push(make_encoded(0));

        assert_eq!(jb.depth(), 2, "very late frame must not enter the queue");
        assert_eq!(
            jb.late_frames(),
            late_before + 1,
            "late_frames must increment"
        );
    }

    #[test]
    fn given_300_uniform_frames_soak_then_all_delivered_and_depth_bounded() {
        let mut jb = JitterBuffer::new(1, 8);
        const N: u64 = 300;
        const FRAME_NS: u64 = 20_000_000;

        let mut delivered = 0u64;
        let mut max_depth = 0usize;

        for i in 0..N {
            jb.push(make_encoded_with_ts(i, i * FRAME_NS));
            if let PopResult::Frame(_) = jb.pop() {
                delivered += 1;
            }
            max_depth = max_depth.max(jb.depth());
        }
        // Drain remaining
        for _ in 0..8 {
            if let PopResult::Frame(_) = jb.pop() {
                delivered += 1;
            }
        }

        assert!(
            delivered >= N - 2, // allow 2-frame startup pipeline
            "soak: only {delivered}/{N} frames delivered"
        );
        assert!(
            max_depth <= 3,
            "soak: max buffer depth {max_depth} > 3 frames (60 ms) under uniform arrival"
        );
    }
}

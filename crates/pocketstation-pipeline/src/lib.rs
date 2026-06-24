// pocketstation-pipeline: frame bus, clock sync, and audio processing graph.
// Merges pocketstation-bus and pocketstation-graph.

use pocketstation_frame::AudioFrame;
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::atomic::{AtomicU64, Ordering};

// ---------------------------------------------------------------------------
// Frame bus (from pocketstation-bus)
// ---------------------------------------------------------------------------

pub struct FrameProducer {
    inner: Producer<AudioFrame>,
    dropped_newest: AtomicU64,
}

pub struct FrameConsumer {
    inner: Consumer<AudioFrame>,
}

pub fn frame_bus(capacity: usize) -> (FrameProducer, FrameConsumer) {
    let (p, c) = RingBuffer::<AudioFrame>::new(capacity);
    (FrameProducer { inner: p, dropped_newest: AtomicU64::new(0) }, FrameConsumer { inner: c })
}

impl FrameProducer {
    pub fn push_drop_newest(&mut self, frame: AudioFrame) -> Result<(), AudioFrame> {
        match self.inner.push(frame) {
            Ok(()) => Ok(()),
            Err(rtrb::PushError::Full(frame)) => {
                self.dropped_newest.fetch_add(1, Ordering::Relaxed);
                Err(frame)
            }
        }
    }
    pub fn dropped_newest(&self) -> u64 {
        self.dropped_newest.load(Ordering::Relaxed)
    }
}

impl FrameConsumer {
    pub fn pop(&mut self) -> Option<AudioFrame> {
        self.inner.pop().ok()
    }
}

// ---------------------------------------------------------------------------
// Clock sync — PI controller (AUDIO-006, from pocketstation-bus)
// ---------------------------------------------------------------------------

const CLOCK_SYNC_CLAMP_NS: i64 = 10_000_000;

#[derive(Debug, Clone, Copy)]
pub struct ClockSync {
    kp: f64,
    ki: f64,
    integral: f64,
    last_offset_ns: i64,
}

impl ClockSync {
    pub fn new(kp: f64, ki: f64) -> Self {
        Self { kp, ki, integral: 0.0, last_offset_ns: 0 }
    }

    pub fn tick(&mut self, measured_offset_ns: i64) -> i64 {
        let error = measured_offset_ns as f64;
        self.integral += error;
        let correction = self.kp * error + self.ki * self.integral;
        self.last_offset_ns = measured_offset_ns;
        correction.round().clamp(-CLOCK_SYNC_CLAMP_NS as f64, CLOCK_SYNC_CLAMP_NS as f64) as i64
    }

    pub fn last_offset_ns(&self) -> i64 { self.last_offset_ns }
    pub fn integral(&self) -> f64 { self.integral }
}

impl Default for ClockSync {
    fn default() -> Self { Self::new(0.1, 0.001) }
}

// ---------------------------------------------------------------------------
// Processing graph (from pocketstation-graph)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelLayout {
    MonoOnly,
    StereoOnly,
    Either,
}

pub trait AudioProcessorNode: Send {
    fn name(&self) -> &'static str;
    fn process(&mut self, frame: AudioFrame) -> Option<AudioFrame>;
    fn accepted_channels(&self) -> ChannelLayout { ChannelLayout::Either }
}

pub struct ProcessorGraph {
    nodes: Vec<Box<dyn AudioProcessorNode>>,
}

impl ProcessorGraph {
    pub fn new() -> Self { Self { nodes: Vec::new() } }

    pub fn add_node<N: AudioProcessorNode + 'static>(&mut self, node: N) {
        self.nodes.push(Box::new(node));
    }

    pub fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        for node in self.nodes.iter_mut() {
            frame = node.process(frame)?;
        }
        Some(frame)
    }
}

impl Default for ProcessorGraph {
    fn default() -> Self { Self::new() }
}

pub struct PassthroughNode;
impl AudioProcessorNode for PassthroughNode {
    fn name(&self) -> &'static str { "passthrough" }
    fn process(&mut self, frame: AudioFrame) -> Option<AudioFrame> { Some(frame) }
}

pub struct GainNode { gain: f32 }
impl GainNode {
    pub fn new(gain: f32) -> Self { Self { gain } }
}
impl AudioProcessorNode for GainNode {
    fn name(&self) -> &'static str { "gain" }
    fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        for s in frame.buffer.as_mut_slice().iter_mut() { *s *= self.gain; }
        Some(frame)
    }
}

pub struct MonoMixNode;
impl Default for MonoMixNode { fn default() -> Self { Self } }
impl AudioProcessorNode for MonoMixNode {
    fn name(&self) -> &'static str { "mono_mix" }
    fn accepted_channels(&self) -> ChannelLayout { ChannelLayout::StereoOnly }
    fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        if frame.channels == 2 {
            let len = frame.buffer.len();
            let samples = frame.buffer.as_mut_slice();
            let mut out = 0usize;
            let mut i = 0usize;
            while i + 1 < len {
                samples[out] = (samples[i] + samples[i + 1]) * 0.5;
                out += 1;
                i += 2;
            }
            frame.buffer.set_len(out);
            frame.channels = 1;
        }
        Some(frame)
    }
}

// ---------------------------------------------------------------------------
// Resample node
// ---------------------------------------------------------------------------

const NS_PER_SEC: f64 = 1_000_000_000.0;
const RESAMPLE_MAX_OUT_SAMPLES: usize = 1920;

pub struct ResampleNode {
    source_rate: u32,
    target_rate: u32,
    channels: u8,
    clock_sync: ClockSync,
    phase: f64,
    last_sample: f32,
    out_buf: Vec<f32>,
}

impl ResampleNode {
    /// Create a node that will downsamples or pass-through frames.
    ///
    /// Panics if `target_rate > source_rate` — upsampling is not implemented.
    /// Use `new_unchecked()` only in tests that explicitly verify the None-return
    /// contract for unsupported configurations.
    pub fn new(source_rate: u32, target_rate: u32, channels: u8) -> Self {
        assert!(
            target_rate <= source_rate,
            "ResampleNode: upsampling {source_rate}→{target_rate} is not implemented; \
             ensure target_rate <= source_rate"
        );
        Self::new_unchecked(source_rate, target_rate, channels)
    }

    /// Like `new()` but skips the upsampling guard. Only for tests verifying the
    /// None-return contract on unsupported configurations.
    pub fn new_unchecked(source_rate: u32, target_rate: u32, channels: u8) -> Self {
        Self {
            source_rate, target_rate, channels,
            clock_sync: ClockSync::default(),
            phase: 0.0, last_sample: 0.0,
            out_buf: Vec::with_capacity(RESAMPLE_MAX_OUT_SAMPLES),
        }
    }

    pub fn identity_48k() -> Self { Self::new(48_000, 48_000, 1) }
}

impl AudioProcessorNode for ResampleNode {
    fn name(&self) -> &'static str { "resample" }
    fn accepted_channels(&self) -> ChannelLayout { ChannelLayout::Either }

    fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        if frame.channels != self.channels { return None; }

        let correction_ns = self.clock_sync.tick(frame.timestamp_ns as i64);
        let drift_ratio = correction_ns as f64 / NS_PER_SEC;

        if self.source_rate == self.target_rate {
            let _ = drift_ratio;
            return Some(frame);
        }

        let base_ratio = self.source_rate as f64 / self.target_rate as f64;
        let ratio = base_ratio - drift_ratio;

        if self.target_rate > self.source_rate {
            // Upsampling is not implemented. Frame is dropped (not passed through
            // unchanged) — a dropped frame is a louder failure than a silent rate
            // mismatch. Production callers must not configure target_rate > source_rate;
            // this is caught at construction time via ResampleNode::new_checked().
            return None;
        }

        {
            let input = frame.buffer.as_slice();
            let input_len = input.len();
            let expected_out = ((input_len as f64 / ratio).ceil() as usize).min(RESAMPLE_MAX_OUT_SAMPLES);
            self.out_buf.clear();

            let mut in_idx: usize = 0;
            let mut phase = self.phase;
            let mut out_count = 0usize;

            while out_count < expected_out {
                phase += ratio;
                while phase >= 1.0 {
                    self.last_sample = if in_idx < input_len { input[in_idx] } else { 0.0 };
                    in_idx += 1;
                    phase -= 1.0;
                }
                let current = if in_idx < input_len { input[in_idx] } else { 0.0_f32 };
                let t = phase as f32;
                self.out_buf.push(self.last_sample * (1.0_f32 - t) + current * t);
                out_count += 1;
            }
            self.phase = phase;
        }

        let out_len = self.out_buf.len();
        {
            let dst = frame.buffer.as_mut_slice();
            dst[..out_len].copy_from_slice(&self.out_buf[..out_len]);
        }
        frame.buffer.set_len(out_len);
        frame.sample_rate = self.target_rate;
        Some(frame)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

    fn make_frame(pool: &std::sync::Arc<AudioBufferPool>, seq: u64) -> AudioFrame {
        let h = pool.acquire().unwrap();
        AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, h)
    }

    // --- Bus tests ---

    #[test]
    fn push_pop_single_frame_succeeds() {
        let pool = AudioBufferPool::new(2, 8);
        let handle = pool.acquire().unwrap();
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, handle);
        let (mut p, mut c) = frame_bus(1);
        p.push_drop_newest(frame).unwrap();
        assert!(c.pop().is_some());
    }

    #[test]
    fn push_into_full_ring_drops_newest_and_increments_counter() {
        let pool = AudioBufferPool::new(8, 4);
        let (mut p, _c) = frame_bus(3);
        for seq in 0..3 { assert!(p.push_drop_newest(make_frame(&pool, seq)).is_ok()); }
        let result = p.push_drop_newest(make_frame(&pool, 3));
        assert!(result.is_err());
        assert_eq!(p.dropped_newest(), 1);
    }

    #[test]
    fn frames_pushed_in_order_are_popped_in_fifo_order() {
        let pool = AudioBufferPool::new(8, 4);
        let (mut p, mut c) = frame_bus(4);
        for seq in 0..4 { p.push_drop_newest(make_frame(&pool, seq)).unwrap(); }
        for expected_seq in 0..4u64 {
            let frame = c.pop().unwrap();
            assert_eq!(frame.sequence_number, expected_seq);
        }
        assert!(c.pop().is_none());
    }

    #[test]
    fn pop_on_empty_ring_returns_none() {
        let (_p, mut c) = frame_bus(4);
        assert!(c.pop().is_none());
    }

    #[test]
    fn ring_capacity_is_bounded_and_excess_frames_are_dropped() {
        let pool = AudioBufferPool::new(16, 4);
        let (mut p, _c) = frame_bus(8);
        let mut pushed = 0u64;
        let mut dropped = 0u64;
        for seq in 0..16 {
            match p.push_drop_newest(make_frame(&pool, seq)) {
                Ok(()) => pushed += 1, Err(_) => dropped += 1,
            }
        }
        assert_eq!(pushed, 8);
        assert_eq!(dropped, 8);
        assert_eq!(p.dropped_newest(), 8);
    }

    // --- ClockSync tests ---

    #[test]
    fn clock_sync_zero_offset_produces_zero_correction() {
        let mut cs = ClockSync::default();
        assert_eq!(cs.tick(0), 0);
        assert_eq!(cs.last_offset_ns(), 0);
    }

    #[test]
    fn clock_sync_positive_offset_produces_positive_correction() {
        let mut cs = ClockSync::default();
        let correction = cs.tick(1_000_000);
        assert!(correction > 0, "positive offset must produce positive correction, got {correction}");
    }

    #[test]
    fn clock_sync_negative_offset_produces_negative_correction() {
        let mut cs = ClockSync::default();
        let correction = cs.tick(-1_000_000);
        assert!(correction < 0, "negative offset must produce negative correction, got {correction}");
    }

    #[test]
    fn clock_sync_correction_is_clamped_to_ten_milliseconds() {
        let mut cs = ClockSync::default();
        let correction = cs.tick(100_000_000);
        assert!(correction.abs() <= 10_000_000, "correction {correction} exceeds ±10 ms clamp");
    }

    #[test]
    fn clock_sync_integral_accumulates_across_ticks() {
        let mut cs = ClockSync::new(0.0, 1.0);
        cs.tick(1_000);
        cs.tick(1_000);
        assert!(cs.integral() > 1_000.0, "integral {} should exceed single-tick error", cs.integral());
    }

    // --- Graph tests ---

    #[test]
    fn gain_node_mutates_samples() {
        let pool = AudioBufferPool::new(1, 4);
        let mut h = pool.acquire().unwrap();
        h.copy_from_slice(&[1.0, -1.0]);
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
        let mut graph = ProcessorGraph::new();
        graph.add_node(GainNode::new(0.5));
        let out = graph.process(frame).unwrap();
        assert_eq!(out.buffer.as_slice(), &[0.5, -0.5]);
    }

    // --- ResampleNode tests ---

    fn make_sample_frame(pool: &std::sync::Arc<AudioBufferPool>, samples: &[f32], rate: u32) -> AudioFrame {
        let mut h = pool.acquire().unwrap();
        h.copy_from_slice(samples);
        let mut f = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
        f.sample_rate = rate;
        f
    }

    #[test]
    fn given_equal_rates_when_process_then_frame_returned_unchanged() {
        let pool = AudioBufferPool::new(4, 960);
        let input: Vec<f32> = (0..480).map(|i| i as f32 * 0.001).collect();
        let frame = make_sample_frame(&pool, &input, 48_000);
        let mut node = ResampleNode::identity_48k();
        let out = node.process(frame).unwrap();
        assert_eq!(out.buffer.len(), input.len());
        assert_eq!(out.sample_rate, 48_000);
    }

    #[test]
    fn given_downsample_48k_to_44100_when_process_then_output_len_is_smaller() {
        let pool = AudioBufferPool::new(4, 960);
        let input: Vec<f32> = (0..960).map(|i| (i as f32) / 960.0).collect();
        let frame = make_sample_frame(&pool, &input, 48_000);
        let mut node = ResampleNode::new(48_000, 44_100, 1);
        let out = node.process(frame).unwrap();
        assert!(out.buffer.len() < input.len());
        assert_eq!(out.sample_rate, 44_100);
    }

    #[test]
    fn given_upsample_44100_to_48000_when_process_then_none_returned() {
        let pool = AudioBufferPool::new(4, 960);
        let input: Vec<f32> = (0..441).map(|i| i as f32 * 0.001).collect();
        let frame = make_sample_frame(&pool, &input, 44_100);
        let mut node = ResampleNode::new_unchecked(44_100, 48_000, 1);
        assert!(node.process(frame).is_none());
    }

    #[test]
    fn given_stereo_frame_when_mono_resample_node_then_none_returned() {
        let pool = AudioBufferPool::new(4, 1920);
        let input: Vec<f32> = (0..960).map(|i| i as f32 * 0.001).collect();
        let mut h = pool.acquire().unwrap();
        h.copy_from_slice(&input);
        let mut frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 2, h);
        frame.sample_rate = 48_000;
        let mut node = ResampleNode::new(48_000, 48_000, 1);
        assert!(node.process(frame).is_none());
    }

    #[test]
    fn given_identity_node_when_1000_frames_processed_then_vec_capacity_is_stable() {
        let pool = AudioBufferPool::new(4, 960);
        let input = vec![0.0f32; 480];
        let mut node = ResampleNode::identity_48k();
        let initial_cap = node.out_buf.capacity();
        for _ in 0..1000 {
            let frame = make_sample_frame(&pool, &input, 48_000);
            let _ = node.process(frame).unwrap();
        }
        assert_eq!(node.out_buf.capacity(), initial_cap);
    }
}

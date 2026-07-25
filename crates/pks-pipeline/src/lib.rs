use pks_frame::AudioFrame;
pub use pks_timing::{ClockCorrectionController, ClockCorrectionController as ClockSync};
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct FrameProducer {
    ring_producer: Producer<AudioFrame>,
    dropped_newest: AtomicU64,
}

pub struct FrameConsumer {
    ring_consumer: Consumer<AudioFrame>,
}

pub fn frame_bus(capacity: usize) -> (FrameProducer, FrameConsumer) {
    let (p, c) = RingBuffer::<AudioFrame>::new(capacity);
    (
        FrameProducer {
            ring_producer: p,
            dropped_newest: AtomicU64::new(0),
        },
        FrameConsumer { ring_consumer: c },
    )
}

impl FrameProducer {
    pub fn push_drop_newest(&mut self, frame: AudioFrame) -> Result<(), AudioFrame> {
        match self.ring_producer.push(frame) {
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
        self.ring_consumer.pop().ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelLayout {
    MonoOnly,
    StereoOnly,
    Either,
}

pub trait AudioProcessorNode: Send {
    fn name(&self) -> &'static str;
    fn process(&mut self, frame: AudioFrame) -> Option<AudioFrame>;
    fn accepted_channels(&self) -> ChannelLayout {
        ChannelLayout::Either
    }
}

pub struct ProcessorGraph {
    nodes: Vec<Box<dyn AudioProcessorNode>>,
}

impl ProcessorGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

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
    fn default() -> Self {
        Self::new()
    }
}

pub struct PassthroughNode;

impl AudioProcessorNode for PassthroughNode {
    fn name(&self) -> &'static str {
        "passthrough"
    }
    fn process(&mut self, frame: AudioFrame) -> Option<AudioFrame> {
        Some(frame)
    }
}

pub struct GainNode {
    gain_ratio: f32,
}

impl GainNode {
    pub fn new(gain_ratio: f32) -> Self {
        Self { gain_ratio }
    }
}

impl AudioProcessorNode for GainNode {
    fn name(&self) -> &'static str {
        "gain"
    }
    fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        for s in frame.buffer.as_mut_slice().iter_mut() {
            *s *= self.gain_ratio;
        }
        Some(frame)
    }
}

pub struct MonoMixNode;

impl Default for MonoMixNode {
    fn default() -> Self {
        Self
    }
}

impl AudioProcessorNode for MonoMixNode {
    fn name(&self) -> &'static str {
        "mono_mix"
    }
    fn accepted_channels(&self) -> ChannelLayout {
        ChannelLayout::StereoOnly
    }

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

const NS_PER_SEC: f64 = 1_000_000_000.0;
const RESAMPLE_MAX_OUT_SAMPLES: usize = 1920;

pub struct ResampleNode {
    source_rate_hz: u32,
    target_rate_hz: u32,
    channel_count: u8,
    clock_correction: ClockCorrectionController,
    phase: f64,
    last_sample_linear: f32,
    out_buf: Vec<f32>,
}

impl ResampleNode {
    // Panics if target_rate_hz > source_rate_hz — upsampling not implemented.
    pub fn new(source_rate_hz: u32, target_rate_hz: u32, channel_count: u8) -> Self {
        assert!(
            target_rate_hz <= source_rate_hz,
            "ResampleNode: upsampling {source_rate_hz}→{target_rate_hz} is not implemented; \
             ensure target_rate_hz <= source_rate_hz"
        );
        Self::new_unchecked(source_rate_hz, target_rate_hz, channel_count)
    }

    // Skips the upsampling guard. Only for tests verifying the None-return contract.
    pub fn new_unchecked(source_rate_hz: u32, target_rate_hz: u32, channel_count: u8) -> Self {
        Self {
            source_rate_hz,
            target_rate_hz,
            channel_count,
            clock_correction: ClockCorrectionController::default(),
            phase: 0.0,
            last_sample_linear: 0.0,
            out_buf: vec![0.0; RESAMPLE_MAX_OUT_SAMPLES],
        }
    }

    pub fn identity_48k() -> Self {
        Self::new(48_000, 48_000, 1)
    }

    pub fn observe_clock_offset(&mut self, measured_offset_ns: i64) -> i64 {
        self.clock_correction.tick(measured_offset_ns)
    }
}

impl AudioProcessorNode for ResampleNode {
    fn name(&self) -> &'static str {
        "resample"
    }
    fn accepted_channels(&self) -> ChannelLayout {
        ChannelLayout::Either
    }

    fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        if frame.channels != self.channel_count {
            return None;
        }

        // A frame timestamp is an observation in one clock domain, not an
        // offset between two clocks. Runtime clock comparison updates the
        // controller through observe_clock_offset().
        let correction_ns = self.clock_correction.last_correction_ns();
        let drift_ratio = correction_ns as f64 / NS_PER_SEC;

        if self.source_rate_hz == self.target_rate_hz {
            let _ = drift_ratio;
            return Some(frame);
        }

        let base_ratio = self.source_rate_hz as f64 / self.target_rate_hz as f64;
        let ratio = base_ratio - drift_ratio;

        if self.target_rate_hz > self.source_rate_hz {
            // Upsampling not implemented; drop frame rather than pass silent mismatch.
            return None;
        }

        let out_len;
        {
            let input = frame.buffer.as_slice();
            let input_len = input.len();
            out_len = ((input_len as f64 / ratio).ceil() as usize).min(RESAMPLE_MAX_OUT_SAMPLES);
            let mut in_idx: usize = 0;
            let mut phase = self.phase;
            let mut out_count = 0usize;

            while out_count < out_len {
                phase += ratio;
                while phase >= 1.0 {
                    self.last_sample_linear = if in_idx < input_len {
                        input[in_idx]
                    } else {
                        0.0
                    };
                    in_idx += 1;
                    phase -= 1.0;
                }
                let current = if in_idx < input_len {
                    input[in_idx]
                } else {
                    0.0_f32
                };
                let t = phase as f32;
                self.out_buf[out_count] = self.last_sample_linear * (1.0_f32 - t) + current * t;
                out_count += 1;
            }
            self.phase = phase;
        }

        frame.buffer.as_mut_slice()[..out_len].copy_from_slice(&self.out_buf[..out_len]);
        frame.buffer.set_len(out_len);
        frame.sample_rate_hz = self.target_rate_hz;
        Some(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

    fn make_frame(pool: &std::sync::Arc<AudioBufferPool>, seq: u64) -> AudioFrame {
        let h = pool.acquire().unwrap();
        AudioFrame::new(StreamId(1), SourceId(1), seq, seq * 20_000_000, 1, h)
    }

    #[test]
    fn given_empty_ring_when_frame_is_pushed_then_same_frame_is_popped() {
        let pool = AudioBufferPool::new(2, 8);
        let handle = pool.acquire().unwrap();
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, handle);
        let (mut p, mut c) = frame_bus(1);
        p.push_drop_newest(frame).unwrap();
        assert!(c.pop().is_some());
    }

    #[test]
    fn given_full_ring_when_frame_is_pushed_then_newest_is_dropped_and_counted() {
        let pool = AudioBufferPool::new(8, 4);
        let (mut p, _c) = frame_bus(3);
        for seq in 0..3 {
            assert!(p.push_drop_newest(make_frame(&pool, seq)).is_ok());
        }
        let result = p.push_drop_newest(make_frame(&pool, 3));
        assert!(result.is_err());
        assert_eq!(p.dropped_newest(), 1);
    }

    #[test]
    fn given_ordered_frames_when_popped_then_fifo_order_is_preserved() {
        let pool = AudioBufferPool::new(8, 4);
        let (mut p, mut c) = frame_bus(4);
        for seq in 0..4 {
            p.push_drop_newest(make_frame(&pool, seq)).unwrap();
        }
        for expected_seq in 0..4u64 {
            let frame = c.pop().unwrap();
            assert_eq!(frame.sequence_number, expected_seq);
        }
        assert!(c.pop().is_none());
    }

    #[test]
    fn given_empty_ring_when_popped_then_none_is_returned() {
        let (_p, mut c) = frame_bus(4);
        assert!(c.pop().is_none());
    }

    #[test]
    fn given_bounded_ring_when_capacity_is_exceeded_then_excess_frames_are_dropped() {
        let pool = AudioBufferPool::new(16, 4);
        let (mut p, _c) = frame_bus(8);
        let mut pushed = 0u64;
        let mut dropped = 0u64;
        for seq in 0..16 {
            match p.push_drop_newest(make_frame(&pool, seq)) {
                Ok(()) => pushed += 1,
                Err(_) => dropped += 1,
            }
        }
        assert_eq!(pushed, 8);
        assert_eq!(dropped, 8);
        assert_eq!(p.dropped_newest(), 8);
    }

    #[test]
    fn given_zero_clock_offset_when_corrected_then_correction_is_zero() {
        let mut controller = ClockCorrectionController::default();
        assert_eq!(controller.tick(0), 0);
        assert_eq!(controller.last_offset_ns(), 0);
    }

    #[test]
    fn given_positive_clock_offset_when_corrected_then_correction_is_positive() {
        let mut controller = ClockCorrectionController::default();
        let correction = controller.tick(1_000_000);
        assert!(
            correction > 0,
            "positive offset must produce positive correction, got {correction}"
        );
    }

    #[test]
    fn given_negative_clock_offset_when_corrected_then_correction_is_negative() {
        let mut controller = ClockCorrectionController::default();
        let correction = controller.tick(-1_000_000);
        assert!(
            correction < 0,
            "negative offset must produce negative correction, got {correction}"
        );
    }

    #[test]
    fn given_large_clock_offset_when_corrected_then_correction_is_clamped() {
        let mut controller = ClockCorrectionController::default();
        let correction = controller.tick(100_000_000);
        assert!(
            correction.abs() <= 10_000_000,
            "correction {correction} exceeds ±10 ms clamp"
        );
    }

    #[test]
    fn given_repeated_clock_offset_when_corrected_then_integral_accumulates() {
        let mut controller = ClockCorrectionController::new(0.0, 1.0);
        controller.tick(1_000);
        controller.tick(1_000);
        assert!(
            controller.integral_ns() > 1_000.0,
            "integral {} should exceed single-tick error",
            controller.integral_ns()
        );
    }

    #[test]
    fn given_audio_samples_when_gain_runs_then_sample_amplitudes_change() {
        let pool = AudioBufferPool::new(1, 4);
        let mut h = pool.acquire().unwrap();
        h.copy_from_slice(&[1.0, -1.0]);
        let frame = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
        let mut graph = ProcessorGraph::new();
        graph.add_node(GainNode::new(0.5));
        let out = graph.process(frame).unwrap();
        assert_eq!(out.buffer.as_slice(), &[0.5, -0.5]);
    }

    fn make_sample_frame(
        pool: &std::sync::Arc<AudioBufferPool>,
        samples: &[f32],
        sample_rate_hz: u32,
    ) -> AudioFrame {
        let mut h = pool.acquire().unwrap();
        h.copy_from_slice(samples);
        let mut f = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
        f.sample_rate_hz = sample_rate_hz;
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
        assert_eq!(out.sample_rate_hz, 48_000);
    }

    #[test]
    fn given_downsample_48k_to_44100_when_process_then_output_len_is_smaller() {
        let pool = AudioBufferPool::new(4, 960);
        let input: Vec<f32> = (0..960).map(|i| (i as f32) / 960.0).collect();
        let frame = make_sample_frame(&pool, &input, 48_000);
        let mut node = ResampleNode::new(48_000, 44_100, 1);
        let out = node.process(frame).unwrap();
        assert!(out.buffer.len() < input.len());
        assert_eq!(out.sample_rate_hz, 44_100);
    }

    #[test]
    fn given_timestamp_without_clock_comparison_when_processed_then_no_offset_is_inferred() {
        let pool = AudioBufferPool::new(4, 960);
        let input: Vec<f32> = (0..960).map(|i| (i as f32) / 960.0).collect();
        let mut frame = make_sample_frame(&pool, &input, 48_000);
        frame.timestamp_ns = 9_000_000_000;
        let mut node = ResampleNode::new(48_000, 44_100, 1);
        let out = node.process(frame).unwrap();
        assert_eq!(out.buffer.len(), 882);
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
        frame.sample_rate_hz = 48_000;
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

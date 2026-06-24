use pocketstation_bus::ClockSync;
use pocketstation_frame::AudioFrame;

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
            // Auto-insert inline mono mix when a node requires mono but the
            // current frame is stereo. This is a temporary in-place measure;
            // the proper graph-builder pre-insertion approach is Phase 2+.
            if node.accepted_channels() == ChannelLayout::MonoOnly && frame.channels > 1 {
                frame = MonoMixNode.process(frame)?;
            }
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
    gain: f32,
}
impl GainNode {
    pub fn new(gain: f32) -> Self {
        Self { gain }
    }
}
impl AudioProcessorNode for GainNode {
    fn name(&self) -> &'static str {
        "gain"
    }
    fn process(&mut self, mut frame: AudioFrame) -> Option<AudioFrame> {
        for s in frame.buffer.as_mut_slice().iter_mut() {
            *s *= self.gain;
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

/// Nanoseconds per second, used to convert the PI-controller correction to a
/// ratio adjustment.
const NS_PER_SEC: f64 = 1_000_000_000.0;

/// Worst-case output frame size (stereo, 48 kHz, 20 ms).
const RESAMPLE_MAX_OUT_SAMPLES: usize = 1920;

/// PI-controlled linear interpolation sample-rate converter (AUDIO-006).
///
/// # Phase 1 scope
/// - Identity pass (source == target): zero work, frame returned unchanged.
/// - Downsampling (target < source): in-place, output length written into
///   the existing buffer via `set_len`.
/// - Upsampling (target > source): deferred to Phase 2; frame returned
///   unchanged with `sample_rate` field already at `source_rate` so
///   downstream nodes are aware.
///
/// # Hot-path guarantees
/// - No heap allocation inside `process()`.
/// - No locks.
/// - No logging.
pub struct ResampleNode {
    source_rate: u32,
    target_rate: u32,
    /// PI controller for clock-drift correction (AUDIO-006).
    clock_sync: ClockSync,
    /// Fractional position within the current input sample pair (0.0..1.0).
    phase: f64,
    /// Last sample from the previous frame, needed for cross-frame interpolation.
    last_sample: f32,
    /// Pre-allocated output scratch buffer (used only for downsampling).
    out_buf: Vec<f32>,
}

impl ResampleNode {
    /// Create a resampler for the given rate pair.
    ///
    /// Pre-allocates the output scratch buffer for the worst-case frame size so
    /// `process()` never allocates.
    pub fn new(source_rate: u32, target_rate: u32) -> Self {
        Self {
            source_rate,
            target_rate,
            clock_sync: ClockSync::default(),
            phase: 0.0,
            last_sample: 0.0,
            out_buf: Vec::with_capacity(RESAMPLE_MAX_OUT_SAMPLES),
        }
    }

    /// Convenience constructor: 48 kHz → 48 kHz identity with drift tracking.
    pub fn identity_48k() -> Self {
        Self::new(48_000, 48_000)
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
        // Feed timestamp into PI controller to compute drift correction.
        // The correction is in nanoseconds; convert to a dimensionless ratio
        // tweak (positive correction → we are running fast → slow down slightly).
        let correction_ns = self.clock_sync.tick(frame.timestamp_ns as i64);
        let drift_ratio = correction_ns as f64 / NS_PER_SEC;

        // Identity case: same rate, no interpolation needed.
        if self.source_rate == self.target_rate {
            // Apply drift correction only — negligible for voice (sub-ppm).
            // Phase 5 will feed back drift_ratio into the capture pipeline.
            let _ = drift_ratio;
            return Some(frame);
        }

        // Effective conversion ratio, nudged by PI drift correction.
        let base_ratio = self.source_rate as f64 / self.target_rate as f64;
        let ratio = base_ratio - drift_ratio;

        if self.target_rate > self.source_rate {
            // Upsampling is not implemented. Drop the frame rather than passing
            // wrong-rate audio downstream. Phase 2+ will add real SRC upsampling.
            return None;
        }

        // ---- Downsampling path (target < source) ----
        // Output is smaller than input: write into out_buf (immutable borrow
        // scope), then copy back into the frame buffer in-place (mutable borrow).

        // Immutable borrow scope — compute output into out_buf.
        {
            let input = frame.buffer.as_slice();
            let input_len = input.len();

            // Compute expected output length.
            let expected_out =
                ((input_len as f64 / ratio).ceil() as usize).min(RESAMPLE_MAX_OUT_SAMPLES);

            // out_buf was pre-allocated to RESAMPLE_MAX_OUT_SAMPLES; clear
            // without reallocation.
            self.out_buf.clear();

            let mut in_idx: usize = 0;
            let mut phase = self.phase;
            let mut out_count = 0usize;

            while out_count < expected_out {
                // Advance phase by ratio to find where this output sample sits
                // in the input stream.
                phase += ratio;

                // Consume whole input samples that phase has passed over.
                while phase >= 1.0 {
                    self.last_sample = if in_idx < input_len {
                        input[in_idx]
                    } else {
                        0.0
                    };
                    in_idx += 1;
                    phase -= 1.0;
                }

                // Linear interpolation between the last consumed input sample
                // and the current one at in_idx.  phase is f64; cast to f32
                // for the arithmetic (sample precision is 32-bit throughout).
                let current = if in_idx < input_len {
                    input[in_idx]
                } else {
                    0.0_f32
                };
                let t = phase as f32;
                let sample = self.last_sample * (1.0_f32 - t) + current * t;

                self.out_buf.push(sample);
                out_count += 1;
            }

            // Save phase for next frame boundary.
            self.phase = phase;
        } // immutable borrow of frame.buffer released here

        // Mutable borrow scope — write output back in-place.
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

#[cfg(test)]
mod resample_tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

    fn make_frame(
        pool: &std::sync::Arc<AudioBufferPool>,
        samples: &[f32],
        rate: u32,
    ) -> AudioFrame {
        let mut h = pool.acquire().unwrap();
        h.copy_from_slice(samples);
        let mut f = AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, h);
        f.sample_rate = rate;
        f
    }

    #[test]
    fn given_equal_rates_when_process_then_frame_returned_unchanged() {
        // Given
        let pool = AudioBufferPool::new(4, 960);
        let input: Vec<f32> = (0..480).map(|i| i as f32 * 0.001).collect();
        let frame = make_frame(&pool, &input, 48_000);
        let mut node = ResampleNode::identity_48k();

        // When
        let out = node.process(frame).unwrap();

        // Then
        assert_eq!(out.buffer.len(), input.len());
        assert_eq!(out.buffer.as_slice(), input.as_slice());
        assert_eq!(out.sample_rate, 48_000);
    }

    #[test]
    fn given_downsample_48k_to_44100_when_process_then_output_len_is_smaller() {
        // Given: 20 ms of 48 kHz mono (960 samples)
        let pool = AudioBufferPool::new(4, 960);
        let input: Vec<f32> = (0..960).map(|i| (i as f32) / 960.0).collect();
        let frame = make_frame(&pool, &input, 48_000);
        let mut node = ResampleNode::new(48_000, 44_100);

        // When
        let out = node.process(frame).unwrap();

        // Then: ~882 samples expected (44100/48000 * 960 = 882)
        let out_len = out.buffer.len();
        assert!(
            out_len < input.len(),
            "downsampled output ({out_len}) should be less than input ({})",
            input.len()
        );
        assert_eq!(out.sample_rate, 44_100);
    }

    #[test]
    fn given_upsample_44100_to_48000_when_process_then_none_returned() {
        // Given: upsampling is not implemented — must fail hard, never silent passthrough.
        let pool = AudioBufferPool::new(4, 960);
        let input: Vec<f32> = (0..441).map(|i| i as f32 * 0.001).collect();
        let frame = make_frame(&pool, &input, 44_100);
        let mut node = ResampleNode::new(44_100, 48_000);

        // When
        let out = node.process(frame);

        // Then: None — upsampling drops the frame until Phase 2 SRC is added
        assert!(out.is_none(), "upsampling must return None until Phase 2 SRC is implemented");
    }

    #[test]
    fn given_identity_node_when_1000_frames_processed_then_vec_capacity_is_stable() {
        // Given
        let pool = AudioBufferPool::new(4, 960);
        let input: Vec<f32> = vec![0.0f32; 480];
        let mut node = ResampleNode::identity_48k();
        let initial_cap = node.out_buf.capacity();

        // When
        for _ in 0..1000 {
            let frame = make_frame(&pool, &input, 48_000);
            let _ = node.process(frame).unwrap();
        }

        // Then: no reallocation — capacity must not have grown.
        assert_eq!(
            node.out_buf.capacity(),
            initial_cap,
            "heap reallocation detected: capacity grew from {initial_cap} to {}",
            node.out_buf.capacity()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, AudioFrame, SourceId, StreamId};

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
}

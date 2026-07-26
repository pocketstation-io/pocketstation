use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
use pks_frame::{AudioBufferPool, AudioFrame, SampleFormat, SourceId, StreamId, POOL_SLOT_SAMPLES};
use rtrb::{Consumer, Producer, RingBuffer};

use pks_graph::node::{
    ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PrepareContext,
};
use pks_graph::partition::ExecutionPartition;
use pks_graph::{NodeFactory, RuntimeNode};

const MONO_MIX_TYPE_ID: &str = "transform.mono_mix";
const MIXER_FRAME_DURATION_NS: u64 = 20_000_000;
const STEREO_CHANNEL_COUNT: u8 = 2;
const MONO_CHANNEL_COUNT: u8 = 1;
const MONO_MIX_SCALE: f32 = 0.5; // 0.5·(L+R) keeps a centred downmix at unity (−6 dB per side)

fn audio_input_port() -> PortSpec {
    PortSpec {
        name: "in".to_owned(),
        direction: PortDirection::Input,
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: ChannelLayout::Any, // accepts stereo or mono
            format: SampleFormat::F32Interleaved,
        }),
        multiplicity: Multiplicity::One,
        required: true,
    }
}

fn mono_output_port() -> PortSpec {
    PortSpec {
        name: "out".to_owned(),
        direction: PortDirection::Output,
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: ChannelLayout::Mono,
            format: SampleFormat::F32Interleaved,
        }),
        multiplicity: Multiplicity::One,
        required: true,
    }
}

pub struct MonoMixFactory;

impl NodeFactory for MonoMixFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(MONO_MIX_TYPE_ID),
            display_name: "Mono Mix",
            inputs: vec![audio_input_port()],
            outputs: vec![mono_output_port()],
            execution: ExecutionPartition::RealtimeCpu,
            realtime_safe: true,
            stateful: false,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _cx: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        Ok(Box::new(MonoMixNode))
    }
}

pub struct MonoMixNode;

impl RuntimeNode for MonoMixNode {
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, mut frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        if frame.channels == STEREO_CHANNEL_COUNT {
            let buf = frame.buffer.as_mut_slice();
            let mono_len = buf.len() / 2;
            for i in 0..mono_len {
                buf[i] = MONO_MIX_SCALE * (buf[2 * i] + buf[2 * i + 1]);
            }
            frame.buffer.set_len(mono_len);
            frame.channels = MONO_CHANNEL_COUNT;
        }
        Ok(Some(frame))
    }
}

// N-input fan-in mixer. Collects AudioFrames from N capture lanes via SPSC
// ring buffers and emits one mono or stereo AudioFrame on each `mix_tick()`.

const MIXER_SOURCE_TYPE_ID: &str = "transform.mixer";

const MIXER_FRAME_SAMPLES: usize = POOL_SLOT_SAMPLES; // 20ms × 48kHz
const MIXER_MAX_CHANNELS: usize = 2;
const MIXER_MAX_FRAME_SAMPLES: usize = MIXER_FRAME_SAMPLES * MIXER_MAX_CHANNELS;
const MIXER_ACCUM_PREALLOC_SAMPLES: usize = MIXER_MAX_FRAME_SAMPLES * 4; // 4 × 20ms stereo headroom
const MIXER_OUTPUT_POOL_SLOTS: usize = 4; // 4 frames in-flight max
const MIXER_CLIP_LIMIT: f32 = 1.0; // unity, unitless ratio

/// Observability counters for `MixerSourceNode`. Monotonically increasing totals;
/// all reads use `Ordering::Relaxed` (no happens-before dependency on the values).
#[derive(Default)]
pub struct MixerTelemetry {
    pub frames_mixed: AtomicU64,            // frames emitted by mix_tick()
    pub lane_underruns: AtomicU64,          // lanes without one output frame at mix time
    pub clipped_samples: AtomicU64,         // samples clamped to ±MIXER_CLIP_LIMIT
    pub output_pool_exhaustions: AtomicU64, // output frames dropped when pool is exhausted
}

impl MixerTelemetry {
    pub fn frames_mixed(&self) -> u64 {
        self.frames_mixed.load(Ordering::Relaxed)
    }
    pub fn lane_underruns(&self) -> u64 {
        self.lane_underruns.load(Ordering::Relaxed)
    }
    pub fn clipped_samples(&self) -> u64 {
        self.clipped_samples.load(Ordering::Relaxed)
    }

    pub fn output_pool_exhaustions(&self) -> u64 {
        self.output_pool_exhaustions.load(Ordering::Relaxed)
    }
}

/// Factory for `MixerSourceNode`. Returns the factory, one `Producer<AudioFrame>`
/// per lane (for capture callbacks), and a shared telemetry handle.
///
/// The consumers are wrapped in `Mutex<Option<…>>` so the factory satisfies
/// `Sync`. The mutex is locked only during `instantiate()` (graph setup, off
/// the hot path); the consumers are moved into the `MixerSourceNode`.
pub struct MixerSourceFactory {
    consumers: Mutex<Option<Vec<Consumer<AudioFrame>>>>,
    telemetry: Arc<MixerTelemetry>,
}

impl MixerSourceFactory {
    pub fn new(
        lane_count: usize,
        capacity_frames: usize,
    ) -> (Self, Vec<Producer<AudioFrame>>, Arc<MixerTelemetry>) {
        let mut producers = Vec::with_capacity(lane_count);
        let mut consumers = Vec::with_capacity(lane_count);
        for _ in 0..lane_count {
            let (prod, cons) = RingBuffer::new(capacity_frames);
            producers.push(prod);
            consumers.push(cons);
        }
        let telemetry = Arc::new(MixerTelemetry::default());
        (
            Self {
                consumers: Mutex::new(Some(consumers)),
                telemetry: Arc::clone(&telemetry),
            },
            producers,
            telemetry,
        )
    }
}

impl NodeFactory for MixerSourceFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(MIXER_SOURCE_TYPE_ID),
            display_name: "Mixer Source",
            inputs: vec![],
            outputs: vec![PortSpec {
                name: "out".to_owned(),
                direction: PortDirection::Output,
                media: MediaCaps::Audio(AudioCaps {
                    sample_rate_hz: Some(48_000),
                    frame_samples: Some(MIXER_FRAME_SAMPLES),
                    channel_layout: ChannelLayout::Mono,
                    format: SampleFormat::F32Interleaved,
                }),
                multiplicity: Multiplicity::One,
                required: true,
            }],
            execution: ExecutionPartition::RealtimeCpu,
            realtime_safe: true,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _cx: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        let consumers = self
            .consumers
            .lock()
            .map_err(|e| NodeError::Prepare(format!("lock poisoned: {e}")))?
            .take()
            .ok_or_else(|| {
                NodeError::Prepare(
                    "MixerSourceFactory consumers already taken; only one node per factory"
                        .to_owned(),
                )
            })?;
        Ok(Box::new(MixerSourceNode::from_consumers(
            consumers,
            Arc::clone(&self.telemetry),
            MONO_CHANNEL_COUNT,
        )))
    }
}

/// N-input fan-in mixer node with an explicit mono or stereo output contract.
///
/// Holds N SPSC consumers (one per capture lane) and per-lane sample
/// accumulators. `mix_tick()` drains each consumer into its accumulator, then
/// when a lane has one output frame of samples, contributes it to the mix.
/// Stereo output preserves stereo input and upmixes mono input to dual mono.
/// Mono output downmixes stereo input with `0.5 * (L + R)`.
///
/// Hot-path contract for `mix_tick()`: no heap allocation during steady state
/// (accumulators pre-allocated in `new()`; pool acquire is a single atomic CAS).
pub struct MixerSourceNode {
    consumers: Vec<Consumer<AudioFrame>>,
    accumulators: Vec<Vec<f32>>, // per-lane samples in the configured output layout
    accumulator_timestamp_ns: Vec<Option<u64>>,
    output_channels: u8,
    output_frame_samples: usize,
    pool: Arc<AudioBufferPool>,
    telemetry: Arc<MixerTelemetry>,
    seq: u64,
}

impl MixerSourceNode {
    /// Direct constructor — use this when calling `mix_tick()` from a time-driven
    /// thread without going through a `NodeFactory`.
    pub fn new(
        lane_count: usize,
        capacity_frames: usize,
    ) -> (Self, Vec<Producer<AudioFrame>>, Arc<MixerTelemetry>) {
        Self::new_with_output_channels(lane_count, capacity_frames, MONO_CHANNEL_COUNT)
    }

    /// Direct constructor with an explicit output channel count.
    /// `output_channels` must be 1 (mono) or 2 (stereo).
    pub fn new_with_output_channels(
        lane_count: usize,
        capacity_frames: usize,
        output_channels: u8,
    ) -> (Self, Vec<Producer<AudioFrame>>, Arc<MixerTelemetry>) {
        assert!(
            matches!(output_channels, MONO_CHANNEL_COUNT | STEREO_CHANNEL_COUNT),
            "mixer output_channels must be 1 or 2"
        );
        let mut producers = Vec::with_capacity(lane_count);
        let mut consumers = Vec::with_capacity(lane_count);
        for _ in 0..lane_count {
            let (prod, cons) = RingBuffer::new(capacity_frames);
            producers.push(prod);
            consumers.push(cons);
        }
        let telemetry = Arc::new(MixerTelemetry::default());
        (
            Self::from_consumers(consumers, Arc::clone(&telemetry), output_channels),
            producers,
            telemetry,
        )
    }

    fn from_consumers(
        consumers: Vec<Consumer<AudioFrame>>,
        telemetry: Arc<MixerTelemetry>,
        output_channels: u8,
    ) -> Self {
        let lane_count = consumers.len();
        let accumulators = (0..lane_count)
            .map(|_| Vec::with_capacity(MIXER_ACCUM_PREALLOC_SAMPLES))
            .collect();
        let accumulator_timestamp_ns = vec![None; lane_count];
        let output_frame_samples = MIXER_FRAME_SAMPLES * output_channels as usize;
        let pool = AudioBufferPool::new(MIXER_OUTPUT_POOL_SLOTS, output_frame_samples);
        Self {
            consumers,
            accumulators,
            accumulator_timestamp_ns,
            output_channels,
            output_frame_samples,
            pool,
            telemetry,
            seq: 0,
        }
    }

    /// Drain all consumers into their per-lane accumulators, then mix one
    /// output frame from lanes that have enough data.
    ///
    /// Returns `None` only when the output pool is exhausted (should not happen
    /// in normal operation with `MIXER_OUTPUT_POOL_SLOTS = 4`). Returns silence
    /// (`Some(zeroed frame)`) when no lane has data yet.
    pub fn mix_tick(
        &mut self,
        stream_id: StreamId,
        source_id: SourceId,
        timestamp_ns: u64,
    ) -> Option<AudioFrame> {
        // Drain each consumer into its per-lane output-layout accumulator.
        // Cap at MIXER_ACCUM_PREALLOC_SAMPLES: accum.len() never exceeds capacity,
        // so Vec::push / extend_from_slice never reallocates on the hot path.
        for ((consumer, accum), accumulator_timestamp_ns) in self
            .consumers
            .iter_mut()
            .zip(self.accumulators.iter_mut())
            .zip(self.accumulator_timestamp_ns.iter_mut())
        {
            while let Ok(captured) = consumer.pop() {
                if accum.is_empty() {
                    *accumulator_timestamp_ns =
                        (captured.timestamp_ns != 0).then_some(captured.timestamp_ns);
                }
                let src = captured.buffer.as_slice();
                match (self.output_channels, captured.channels) {
                    (STEREO_CHANNEL_COUNT, STEREO_CHANNEL_COUNT) => {
                        let headroom = MIXER_ACCUM_PREALLOC_SAMPLES.saturating_sub(accum.len());
                        let take = headroom.min(src.len()) & !1;
                        accum.extend_from_slice(&src[..take]);
                    }
                    (STEREO_CHANNEL_COUNT, MONO_CHANNEL_COUNT) => {
                        for &sample in src {
                            if accum.len() + 2 > MIXER_ACCUM_PREALLOC_SAMPLES {
                                break;
                            }
                            accum.push(sample);
                            accum.push(sample);
                        }
                    }
                    (MONO_CHANNEL_COUNT, STEREO_CHANNEL_COUNT) => {
                        for pair in src.chunks_exact(2) {
                            if accum.len() < MIXER_ACCUM_PREALLOC_SAMPLES {
                                accum.push(MONO_MIX_SCALE * (pair[0] + pair[1]));
                            }
                        }
                    }
                    (MONO_CHANNEL_COUNT, MONO_CHANNEL_COUNT) => {
                        let headroom = MIXER_ACCUM_PREALLOC_SAMPLES.saturating_sub(accum.len());
                        accum.extend_from_slice(&src[..headroom.min(src.len())]);
                    }
                    _ => {}
                }
                // captured dropped → pool slot returned.
            }
        }

        // Mix one frame from lanes with enough accumulated data.
        let mut mix = [0.0f32; MIXER_MAX_FRAME_SAMPLES];
        let mix = &mut mix[..self.output_frame_samples];
        let mut contributing: u32 = 0;
        let mut output_timestamp_ns: Option<u64> = None;

        for (accum, accumulator_timestamp_ns) in self
            .accumulators
            .iter_mut()
            .zip(self.accumulator_timestamp_ns.iter_mut())
        {
            if accum.len() >= self.output_frame_samples {
                for (out, &s) in mix.iter_mut().zip(accum.iter()) {
                    *out += s;
                }
                accum.drain(..self.output_frame_samples);
                contributing += 1;
                if let Some(lane_timestamp_ns) = *accumulator_timestamp_ns {
                    output_timestamp_ns = Some(
                        output_timestamp_ns
                            .map_or(lane_timestamp_ns, |current| current.min(lane_timestamp_ns)),
                    );
                    if accum.is_empty() {
                        *accumulator_timestamp_ns = None;
                    } else {
                        *accumulator_timestamp_ns =
                            Some(lane_timestamp_ns.saturating_add(MIXER_FRAME_DURATION_NS));
                    }
                }
            } else {
                self.telemetry
                    .lane_underruns
                    .fetch_add(1, Ordering::Relaxed);
            }
        }

        let scale = if contributing > 0 {
            1.0 / contributing as f32
        } else {
            0.0
        };
        let mut clips: u64 = 0;
        for s in mix.iter_mut() {
            *s *= scale;
            if *s > MIXER_CLIP_LIMIT {
                *s = MIXER_CLIP_LIMIT;
                clips += 1;
            } else if *s < -MIXER_CLIP_LIMIT {
                *s = -MIXER_CLIP_LIMIT;
                clips += 1;
            }
        }
        if clips > 0 {
            self.telemetry
                .clipped_samples
                .fetch_add(clips, Ordering::Relaxed);
        }
        let Some(mut out) = self.pool.acquire() else {
            self.telemetry
                .output_pool_exhaustions
                .fetch_add(1, Ordering::Relaxed);
            return None;
        };
        out.copy_from_slice(mix);
        let seq = self.seq;
        self.seq += 1;
        self.telemetry.frames_mixed.fetch_add(1, Ordering::Relaxed);
        Some(AudioFrame::new(
            stream_id,
            source_id,
            seq,
            output_timestamp_ns.unwrap_or(timestamp_ns),
            self.output_channels,
            out,
        ))
    }
}

impl RuntimeNode for MixerSourceNode {
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        self.seq = 0;
        for accum in &mut self.accumulators {
            accum.clear();
        }
        self.accumulator_timestamp_ns.fill(None);
        Ok(())
    }

    /// Clock-tick entry point when used inside the graph executor.
    ///
    /// The `clock_frame` provides the stream/source identity and timestamp;
    /// its sample buffer is discarded. Returns the mixed frame, or the
    /// `None` if the output pool is exhausted.
    fn process(&mut self, clock_frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        let ts = clock_frame.timestamp_ns;
        let sid = clock_frame.stream_id;
        let src = clock_frame.source_id;
        // clock_frame dropped here — its buffer returns to the executor's pool.
        Ok(self.mix_tick(sid, src, ts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_frame::{AudioBufferPool, SampleSpec, SourceId, StreamId};
    use proptest::prelude::*;

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved))
    }

    fn frame_with(samples: &[f32], channels: u8) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples.len().max(1));
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(samples);
        AudioFrame::new(StreamId(0), SourceId(0), 0, 0, channels, handle)
    }

    // MixerSourceNode tests

    #[test]
    fn given_mixer_with_one_mono_lane_when_enough_samples_pushed_then_mix_tick_returns_frame() {
        let (mut node, mut producers, telemetry) = MixerSourceNode::new(1, 8);
        let pool = AudioBufferPool::new(4, MIXER_FRAME_SAMPLES);

        // Push exactly one 960-sample mono frame.
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&vec![0.5f32; MIXER_FRAME_SAMPLES]);
        producers[0]
            .push(AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle))
            .unwrap();

        let out = node.mix_tick(StreamId(0), SourceId(0), 0).unwrap();
        assert_eq!(out.channels, MONO_CHANNEL_COUNT);
        assert_eq!(out.buffer.len(), MIXER_FRAME_SAMPLES);
        let peak = out.buffer.as_slice().iter().copied().fold(0.0f32, f32::max);
        assert!((peak - 0.5).abs() < 1e-6, "expected 0.5 peak, got {peak}");
        assert_eq!(telemetry.frames_mixed(), 1);
        assert_eq!(telemetry.lane_underruns(), 0);
    }

    #[test]
    fn given_mixer_with_empty_lane_when_mix_tick_called_then_underrun_incremented_and_silence_returned(
    ) {
        let (mut node, _producers, telemetry) = MixerSourceNode::new(1, 4);
        let out = node.mix_tick(StreamId(0), SourceId(0), 0).unwrap();
        // No contributors → scaled to 0.0 × 0 = silence.
        let peak = out.buffer.as_slice().iter().copied().fold(0.0f32, f32::max);
        assert!(
            peak.abs() < 1e-9,
            "expected silence on underrun, got {peak}"
        );
        assert_eq!(telemetry.lane_underruns(), 1);
    }

    #[test]
    fn given_mixer_with_stereo_lane_when_data_pushed_then_output_is_downmixed_to_mono() {
        let (mut node, mut producers, _telemetry) = MixerSourceNode::new(1, 8);
        let pool = AudioBufferPool::new(4, MIXER_FRAME_SAMPLES * 2);

        // Push one stereo frame: L=1.0, R=0.0 → expected mono = 0.5.
        let stereo: Vec<f32> = (0..MIXER_FRAME_SAMPLES)
            .flat_map(|_| [1.0f32, 0.0f32])
            .collect();
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&stereo);
        producers[0]
            .push(AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 2, handle))
            .unwrap();

        let out = node.mix_tick(StreamId(0), SourceId(0), 0).unwrap();
        assert_eq!(out.channels, MONO_CHANNEL_COUNT);
        let peak = out.buffer.as_slice().iter().copied().fold(0.0f32, f32::max);
        assert!(
            (peak - 0.5).abs() < 1e-6,
            "expected 0.5 from stereo downmix (L=1.0, R=0.0), got {peak}"
        );
    }

    #[test]
    fn given_stereo_mixer_with_stereo_lane_when_mixed_then_channels_remain_distinct() {
        let (mut node, mut producers, _telemetry) =
            MixerSourceNode::new_with_output_channels(1, 4, STEREO_CHANNEL_COUNT);
        let pool = AudioBufferPool::new(1, MIXER_MAX_FRAME_SAMPLES);
        let stereo: Vec<f32> = (0..MIXER_FRAME_SAMPLES)
            .flat_map(|_| [1.0f32, 0.0f32])
            .collect();
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&stereo);
        producers[0]
            .push(AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 2, handle))
            .unwrap();

        let out = node.mix_tick(StreamId(0), SourceId(0), 0).unwrap();

        assert_eq!(out.channels, STEREO_CHANNEL_COUNT);
        assert_eq!(out.buffer.len(), MIXER_MAX_FRAME_SAMPLES);
        for pair in out.buffer.as_slice().chunks_exact(2) {
            assert!((pair[0] - 1.0).abs() < 1e-6);
            assert!(pair[1].abs() < 1e-6);
        }
    }

    #[test]
    fn given_stereo_mixer_with_mono_lane_when_mixed_then_input_is_dual_mono() {
        let (mut node, mut producers, _telemetry) =
            MixerSourceNode::new_with_output_channels(1, 4, STEREO_CHANNEL_COUNT);
        let pool = AudioBufferPool::new(1, MIXER_FRAME_SAMPLES);
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&[0.25; MIXER_FRAME_SAMPLES]);
        producers[0]
            .push(AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle))
            .unwrap();

        let out = node.mix_tick(StreamId(0), SourceId(0), 0).unwrap();

        assert_eq!(out.channels, STEREO_CHANNEL_COUNT);
        for pair in out.buffer.as_slice().chunks_exact(2) {
            assert!((pair[0] - 0.25).abs() < 1e-6);
            assert!((pair[1] - 0.25).abs() < 1e-6);
        }
    }

    #[test]
    fn given_mixer_with_two_lanes_when_both_have_data_then_output_is_averaged() {
        let (mut node, mut producers, telemetry) = MixerSourceNode::new(2, 8);
        let pool_a = AudioBufferPool::new(4, MIXER_FRAME_SAMPLES);
        let pool_b = AudioBufferPool::new(4, MIXER_FRAME_SAMPLES);

        let mut ha = pool_a.acquire().unwrap();
        ha.copy_from_slice(&vec![0.8f32; MIXER_FRAME_SAMPLES]);
        producers[0]
            .push(AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, ha))
            .unwrap();

        let mut hb = pool_b.acquire().unwrap();
        hb.copy_from_slice(&vec![0.2f32; MIXER_FRAME_SAMPLES]);
        producers[1]
            .push(AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, hb))
            .unwrap();

        let out = node.mix_tick(StreamId(0), SourceId(0), 0).unwrap();
        let peak = out.buffer.as_slice().iter().copied().fold(0.0f32, f32::max);
        // Average of 0.8 and 0.2 = 0.5.
        assert!(
            (peak - 0.5).abs() < 1e-6,
            "expected averaged 0.5 from two lanes, got {peak}"
        );
        assert_eq!(telemetry.lane_underruns(), 0);
    }

    #[test]
    fn given_lanes_with_capture_time_when_mixed_then_oldest_origin_is_preserved() {
        let (mut node, mut producers, _telemetry) = MixerSourceNode::new(2, 4);
        let pool_a = AudioBufferPool::new(1, MIXER_FRAME_SAMPLES);
        let pool_b = AudioBufferPool::new(1, MIXER_FRAME_SAMPLES);
        let mut handle_a = pool_a.acquire().unwrap();
        let mut handle_b = pool_b.acquire().unwrap();
        handle_a.copy_from_slice(&[0.5; MIXER_FRAME_SAMPLES]);
        handle_b.copy_from_slice(&[0.5; MIXER_FRAME_SAMPLES]);
        producers[0]
            .push(AudioFrame::new(
                StreamId(0),
                SourceId(0),
                0,
                120,
                1,
                handle_a,
            ))
            .unwrap();
        producers[1]
            .push(AudioFrame::new(
                StreamId(1),
                SourceId(1),
                0,
                100,
                1,
                handle_b,
            ))
            .unwrap();

        let output = node.mix_tick(StreamId(0), SourceId(0), 999).unwrap();

        assert_eq!(output.timestamp_ns, 100);
    }

    #[test]
    fn given_mixer_output_pool_exhausted_when_processed_then_frame_dropped_without_panic() {
        let (mut node, _producers, telemetry) = MixerSourceNode::new(1, 4);
        let clock_pool = AudioBufferPool::new(1, MIXER_FRAME_SAMPLES);
        let mut held_outputs = Vec::with_capacity(MIXER_OUTPUT_POOL_SLOTS);

        for seq in 0..MIXER_OUTPUT_POOL_SLOTS {
            held_outputs.push(
                node.mix_tick(StreamId(0), SourceId(0), seq as u64)
                    .expect("output pool should have initial capacity"),
            );
        }

        let clock = AudioFrame::new(
            StreamId(0),
            SourceId(0),
            99,
            99,
            MONO_CHANNEL_COUNT,
            clock_pool.acquire().unwrap(),
        );
        let out = node.process(clock).unwrap();

        assert!(out.is_none(), "pool exhaustion should drop the frame");
        assert_eq!(telemetry.output_pool_exhaustions(), 1);
        assert_eq!(telemetry.frames_mixed(), MIXER_OUTPUT_POOL_SLOTS as u64);
        drop(held_outputs);
    }

    #[test]
    fn given_mixer_factory_when_instantiated_twice_then_second_call_returns_error() {
        let (factory, _producers, _telemetry) = MixerSourceFactory::new(2, 4);
        let cx = PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved));
        let _first = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        let second = factory.instantiate(&cx, &NodeConfig::new());
        assert!(second.is_err(), "second instantiation must fail");
    }

    #[test]
    fn given_mono_mix_when_stereo_frame_processed_then_output_is_mono() {
        let mut node = MonoMixFactory
            .instantiate(&prepare_cx(), &NodeConfig::new())
            .unwrap();
        let interleaved = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
        let out = node
            .process(frame_with(&interleaved, STEREO_CHANNEL_COUNT))
            .unwrap()
            .unwrap();
        assert_eq!(out.channels, MONO_CHANNEL_COUNT);
        assert_eq!(out.buffer.len(), interleaved.len() / 2);
        for &sample in out.buffer.as_slice() {
            assert!((sample - 0.5).abs() < 1e-6);
        }
    }

    #[test]
    fn given_mono_mix_when_mono_frame_processed_then_frame_passes_through_unchanged() {
        let mut node = MonoMixFactory
            .instantiate(&prepare_cx(), &NodeConfig::new())
            .unwrap();
        let mono = [0.1, 0.2, 0.3, 0.4];
        let out = node
            .process(frame_with(&mono, MONO_CHANNEL_COUNT))
            .unwrap()
            .unwrap();
        assert_eq!(out.channels, MONO_CHANNEL_COUNT);
        assert_eq!(out.buffer.as_slice(), &mono);
    }

    proptest! {
        #[test]
        fn given_random_stereo_frame_when_mono_mixed_then_output_len_is_half_input(
            pairs in prop::collection::vec((-1.0f32..1.0, -1.0f32..1.0), 1..32usize),
        ) {
            let mut node = MonoMixFactory
                .instantiate(&prepare_cx(), &NodeConfig::new())
                .unwrap();
            let mut interleaved = Vec::with_capacity(pairs.len() * 2);
            for (left, right) in &pairs {
                interleaved.push(*left);
                interleaved.push(*right);
            }
            let input_len = interleaved.len();
            let out = node
                .process(frame_with(&interleaved, STEREO_CHANNEL_COUNT))
                .unwrap()
                .unwrap();
            prop_assert_eq!(out.buffer.len(), input_len / 2);
            prop_assert_eq!(out.channels, MONO_CHANNEL_COUNT);
        }
    }
}

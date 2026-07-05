use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
use pks_frame::{AudioBufferPool, AudioFrame, SampleFormat, SourceId, StreamId, POOL_SLOT_SAMPLES};
use rtrb::{Consumer, Producer, RingBuffer};

use pks_graph::node::{
    ConfigError, ExecutionClass, NodeConfig, NodeDescriptor, NodeError, NodeKind, NodeTypeId,
    PrepareContext,
};
use pks_graph::{NodeFactory, RuntimeNode};

const MONO_MIX_TYPE_ID: &str = "transform.mono_mix";
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
            kind: NodeKind::Transform,
            inputs: vec![audio_input_port()],
            outputs: vec![mono_output_port()],
            execution: ExecutionClass::RealtimeCpu,
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

// N-input fan-in mono mixer. Collects AudioFrames from N capture lanes via SPSC
// ring buffers, accumulates per-lane samples, and emits one mono AudioFrame on
// each `mix_tick()` call from the time-driven mixer thread.

const MIXER_SOURCE_TYPE_ID: &str = "transform.mixer";

const MIXER_FRAME_SAMPLES: usize = POOL_SLOT_SAMPLES; // 20ms × 48kHz
const MIXER_ACCUM_PREALLOC_SAMPLES: usize = MIXER_FRAME_SAMPLES * 4; // 4 × 20ms headroom
const MIXER_OUTPUT_POOL_SLOTS: usize = 4; // 4 frames in-flight max
const MIXER_CLIP_LIMIT: f32 = 1.0; // unity, unitless ratio

/// Observability counters for `MixerSourceNode`. Monotonically increasing totals;
/// all reads use `Ordering::Relaxed` (no happens-before dependency on the values).
#[derive(Default)]
pub struct MixerTelemetry {
    pub frames_mixed: AtomicU64,    // mono frames emitted by mix_tick()
    pub lane_underruns: AtomicU64,  // lanes with < MIXER_FRAME_SAMPLES at mix time
    pub clipped_samples: AtomicU64, // samples clamped to ±MIXER_CLIP_LIMIT
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
            kind: NodeKind::Source,
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
            execution: ExecutionClass::RealtimeCpu,
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
        )))
    }
}

/// N-input fan-in mono mixer node.
///
/// Holds N SPSC consumers (one per capture lane) and per-lane sample
/// accumulators. `mix_tick()` drains each consumer into its accumulator, then
/// when every lane has ≥ `MIXER_FRAME_SAMPLES` (960) samples, contributes them
/// to the mix. Lanes with fewer samples count as underruns.
///
/// Hot-path contract for `mix_tick()`: no heap allocation during steady state
/// (accumulators pre-allocated in `new()`; pool acquire is a single atomic CAS).
pub struct MixerSourceNode {
    consumers: Vec<Consumer<AudioFrame>>,
    accumulators: Vec<Vec<f32>>, // per-lane mono; stereo downmixed on drain
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
        let mut producers = Vec::with_capacity(lane_count);
        let mut consumers = Vec::with_capacity(lane_count);
        for _ in 0..lane_count {
            let (prod, cons) = RingBuffer::new(capacity_frames);
            producers.push(prod);
            consumers.push(cons);
        }
        let telemetry = Arc::new(MixerTelemetry::default());
        (
            Self::from_consumers(consumers, Arc::clone(&telemetry)),
            producers,
            telemetry,
        )
    }

    fn from_consumers(
        consumers: Vec<Consumer<AudioFrame>>,
        telemetry: Arc<MixerTelemetry>,
    ) -> Self {
        let lane_count = consumers.len();
        let accumulators = (0..lane_count)
            .map(|_| Vec::with_capacity(MIXER_ACCUM_PREALLOC_SAMPLES))
            .collect();
        let pool = AudioBufferPool::new(MIXER_OUTPUT_POOL_SLOTS, MIXER_FRAME_SAMPLES);
        Self {
            consumers,
            accumulators,
            pool,
            telemetry,
            seq: 0,
        }
    }

    /// Drain all consumers into their per-lane accumulators, then mix one
    /// `MIXER_FRAME_SAMPLES`-wide frame from lanes that have enough data.
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
        // Drain each consumer into its per-lane mono accumulator.
        for (consumer, accum) in self.consumers.iter_mut().zip(self.accumulators.iter_mut()) {
            while let Ok(captured) = consumer.pop() {
                let src = captured.buffer.as_slice();
                if captured.channels == 2 {
                    for pair in src.chunks_exact(2) {
                        accum.push((pair[0] + pair[1]) * 0.5); // 0.5*(L+R) downmix
                    }
                } else {
                    accum.extend_from_slice(src);
                }
                // captured dropped → pool slot returned.
            }
        }

        // Mix one frame from lanes with enough accumulated data.
        let mut mix = [0.0f32; MIXER_FRAME_SAMPLES];
        let mut contributing: u32 = 0;

        for accum in &mut self.accumulators {
            if accum.len() >= MIXER_FRAME_SAMPLES {
                for (out, &s) in mix.iter_mut().zip(accum.iter()) {
                    *out += s;
                }
                accum.drain(..MIXER_FRAME_SAMPLES);
                contributing += 1;
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
        self.telemetry.frames_mixed.fetch_add(1, Ordering::Relaxed);

        let mut out = self.pool.acquire()?;
        out.copy_from_slice(&mix);
        let seq = self.seq;
        self.seq += 1;
        Some(AudioFrame::new(
            stream_id,
            source_id,
            seq,
            timestamp_ns,
            MONO_CHANNEL_COUNT,
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
        Ok(())
    }

    /// Clock-tick entry point when used inside the graph executor.
    ///
    /// The `clock_frame` provides the stream/source identity and timestamp;
    /// its sample buffer is discarded. Returns the mixed frame, or the
    /// zeroed clock frame as silence if the output pool is exhausted.
    fn process(&mut self, clock_frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        let ts = clock_frame.timestamp_ns;
        let sid = clock_frame.stream_id;
        let src = clock_frame.source_id;
        // clock_frame dropped here — its buffer returns to the executor's pool.
        Ok(Some(self.mix_tick(sid, src, ts).unwrap_or_else(|| {
            // Pool exhausted: allocate a fallback silence frame from the clock pool.
            // This path is never taken in steady state (MIXER_OUTPUT_POOL_SLOTS = 4).
            let fallback_pool = AudioBufferPool::new(1, MIXER_FRAME_SAMPLES);
            let mut h = fallback_pool
                .acquire()
                .expect("single-slot pool never exhausted");
            h.copy_from_slice(&[0.0f32; MIXER_FRAME_SAMPLES]);
            AudioFrame::new(sid, src, self.seq, ts, MONO_CHANNEL_COUNT, h)
        })))
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

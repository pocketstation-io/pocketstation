//! First-party CLI realtime nodes retained behind `internal-testing`.
//!
//! These concrete bounded nodes execute on the realtime plan;
//! they are not a second runtime and are not part of the stable SDK surface.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use rtrb::{Consumer, Producer, RingBuffer};

use crate::frame::{AudioBufferPool, AudioFrame, SampleFormat, SourceId, StreamId};
use crate::graph::{
    register_builtins, AudioCaps, ChannelLayout, ConfigError, ExecutionPartition, MediaCaps,
    Multiplicity, NodeConfig, NodeDescriptor, NodeError, NodeFactory, NodeRegistrationError,
    NodeRegistry, NodeTypeId, PortDirection, PortSpec, PrepareContext, RuntimeNode, SafetyContract,
    SignalSpec,
};

pub(crate) const SYSTEM_OUTPUT_NODE_TYPE_ID: &str = "source.system_output";
pub(crate) const BRIDGE_ENDPOINT_NODE_TYPE_ID: &str = "endpoint.bridge";
const MIXER_FRAME_SAMPLES_PER_CHANNEL: usize = 960;
const MIXER_MAX_FRAME_SAMPLES: usize = MIXER_FRAME_SAMPLES_PER_CHANNEL * 2;
const MIXER_ACCUMULATOR_CAPACITY: usize = MIXER_MAX_FRAME_SAMPLES * 4;
const MIXER_POOL_SLOTS: usize = 4;
const MIXER_FRAME_DURATION_NS: u64 = 20_000_000;

fn audio_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        signal: SignalSpec::audio(),
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(48_000),
            frame_samples: None,
            channel_layout: ChannelLayout::Any,
            format: SampleFormat::F32Interleaved,
        }),
        multiplicity: Multiplicity::One,
        required: true,
    }
}

pub fn register_runtime_nodes(registry: &mut NodeRegistry) -> Result<(), NodeRegistrationError> {
    register_builtins(registry)
}

#[derive(Default)]
pub struct SystemOutputTelemetry {
    frames_captured: AtomicU64,
    frames_emitted: AtomicU64,
    underrun_count: AtomicU64,
}

impl SystemOutputTelemetry {
    pub fn frames_captured(&self) -> u64 {
        self.frames_captured.load(Ordering::Relaxed)
    }
    pub fn frames_emitted(&self) -> u64 {
        self.frames_emitted.load(Ordering::Relaxed)
    }
    pub fn underrun_count(&self) -> u64 {
        self.underrun_count.load(Ordering::Relaxed)
    }
}

pub struct SystemOutputSourceFactory {
    consumer: Mutex<Option<Consumer<AudioFrame>>>,
    telemetry: Arc<SystemOutputTelemetry>,
}

impl SystemOutputSourceFactory {
    pub fn new(capacity_frames: usize) -> (Self, Producer<AudioFrame>, Arc<SystemOutputTelemetry>) {
        let (producer, consumer) = RingBuffer::new(capacity_frames);
        let telemetry = Arc::new(SystemOutputTelemetry::default());
        (
            Self {
                consumer: Mutex::new(Some(consumer)),
                telemetry: Arc::clone(&telemetry),
            },
            producer,
            telemetry,
        )
    }
}

impl NodeFactory for SystemOutputSourceFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(SYSTEM_OUTPUT_NODE_TYPE_ID),
            display_name: "Bounded capture ingress",
            inputs: Vec::new(),
            outputs: vec![audio_port("audio", PortDirection::Output)],
            execution: ExecutionPartition::RealtimeCpu,
            safety: SafetyContract::RealtimeSafe,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _context: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        let consumer = self
            .consumer
            .lock()
            .map_err(|error| NodeError::Prepare(format!("capture ingress lock poisoned: {error}")))?
            .take()
            .ok_or_else(|| NodeError::Prepare("capture ingress already instantiated".to_owned()))?;
        Ok(Box::new(SystemOutputSourceNode {
            consumer,
            telemetry: Arc::clone(&self.telemetry),
        }))
    }
}

struct SystemOutputSourceNode {
    consumer: Consumer<AudioFrame>,
    telemetry: Arc<SystemOutputTelemetry>,
}

impl RuntimeNode for SystemOutputSourceNode {
    fn prepare(&mut self, _context: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        match self.consumer.pop() {
            Ok(captured) => {
                self.telemetry
                    .frames_captured
                    .fetch_add(1, Ordering::Relaxed);
                self.telemetry
                    .frames_emitted
                    .fetch_add(1, Ordering::Relaxed);
                Ok(Some(captured))
            }
            Err(_) => {
                self.telemetry
                    .underrun_count
                    .fetch_add(1, Ordering::Relaxed);
                self.telemetry
                    .frames_emitted
                    .fetch_add(1, Ordering::Relaxed);
                Ok(Some(frame))
            }
        }
    }
}

#[derive(Default)]
pub struct BridgeSinkTelemetry {
    frames_pushed: AtomicU64,
    overrun_count: AtomicU64,
}

impl BridgeSinkTelemetry {
    pub fn frames_pushed(&self) -> u64 {
        self.frames_pushed.load(Ordering::Relaxed)
    }
    pub fn overrun_count(&self) -> u64 {
        self.overrun_count.load(Ordering::Relaxed)
    }
}

pub struct BridgeSinkFactory {
    producer: Mutex<Option<Producer<AudioFrame>>>,
    telemetry: Arc<BridgeSinkTelemetry>,
}

impl BridgeSinkFactory {
    pub fn new(capacity_frames: usize) -> (Self, Consumer<AudioFrame>, Arc<BridgeSinkTelemetry>) {
        let (producer, consumer) = RingBuffer::new(capacity_frames);
        let telemetry = Arc::new(BridgeSinkTelemetry::default());
        (
            Self {
                producer: Mutex::new(Some(producer)),
                telemetry: Arc::clone(&telemetry),
            },
            consumer,
            telemetry,
        )
    }
}

impl NodeFactory for BridgeSinkFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(BRIDGE_ENDPOINT_NODE_TYPE_ID),
            display_name: "Bounded audio bridge",
            inputs: vec![audio_port("in", PortDirection::Input)],
            outputs: Vec::new(),
            execution: ExecutionPartition::RealtimeCpu,
            safety: SafetyContract::RealtimeSafe,
            stateful: true,
        }
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _context: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        let producer = self
            .producer
            .lock()
            .map_err(|error| NodeError::Prepare(format!("bridge lock poisoned: {error}")))?
            .take()
            .ok_or_else(|| NodeError::Prepare("bridge already instantiated".to_owned()))?;
        Ok(Box::new(BridgeSinkNode {
            producer,
            telemetry: Arc::clone(&self.telemetry),
        }))
    }
}

struct BridgeSinkNode {
    producer: Producer<AudioFrame>,
    telemetry: Arc<BridgeSinkTelemetry>,
}

impl RuntimeNode for BridgeSinkNode {
    fn prepare(&mut self, _context: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        if self.producer.push(frame).is_ok() {
            self.telemetry.frames_pushed.fetch_add(1, Ordering::Relaxed);
        } else {
            self.telemetry.overrun_count.fetch_add(1, Ordering::Relaxed);
        }
        Ok(None)
    }
}

#[derive(Default)]
pub struct MixerTelemetry {
    frames_mixed: AtomicU64,
    lane_underruns: AtomicU64,
    clipped_samples: AtomicU64,
    output_pool_exhaustions: AtomicU64,
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

pub struct MixerSourceNode {
    consumers: Vec<Consumer<AudioFrame>>,
    accumulators: Vec<Vec<f32>>,
    timestamps: Vec<Option<u64>>,
    output_channels: u8,
    output_samples: usize,
    pool: Arc<AudioBufferPool>,
    telemetry: Arc<MixerTelemetry>,
    sequence_number: u64,
}

impl MixerSourceNode {
    pub fn new_with_output_channels(
        lane_count: usize,
        capacity_frames: usize,
        output_channels: u8,
    ) -> (Self, Vec<Producer<AudioFrame>>, Arc<MixerTelemetry>) {
        assert!(matches!(output_channels, 1 | 2));
        let mut producers = Vec::with_capacity(lane_count);
        let mut consumers = Vec::with_capacity(lane_count);
        for _ in 0..lane_count {
            let (producer, consumer) = RingBuffer::new(capacity_frames);
            producers.push(producer);
            consumers.push(consumer);
        }
        let output_samples = MIXER_FRAME_SAMPLES_PER_CHANNEL * usize::from(output_channels);
        let telemetry = Arc::new(MixerTelemetry::default());
        (
            Self {
                consumers,
                accumulators: (0..lane_count)
                    .map(|_| Vec::with_capacity(MIXER_ACCUMULATOR_CAPACITY))
                    .collect(),
                timestamps: vec![None; lane_count],
                output_channels,
                output_samples,
                pool: AudioBufferPool::new(MIXER_POOL_SLOTS, output_samples),
                telemetry: Arc::clone(&telemetry),
                sequence_number: 0,
            },
            producers,
            telemetry,
        )
    }

    fn mix_tick(
        &mut self,
        stream_id: StreamId,
        source_id: SourceId,
        fallback_timestamp_ns: u64,
    ) -> Option<AudioFrame> {
        for ((consumer, accumulator), timestamp) in self
            .consumers
            .iter_mut()
            .zip(&mut self.accumulators)
            .zip(&mut self.timestamps)
        {
            while let Ok(frame) = consumer.pop() {
                if accumulator.is_empty() {
                    *timestamp = (frame.timestamp_ns() != 0).then_some(frame.timestamp_ns());
                }
                let samples = frame.samples();
                match (self.output_channels, frame.channels()) {
                    (2, 2) => {
                        let take = MIXER_ACCUMULATOR_CAPACITY
                            .saturating_sub(accumulator.len())
                            .min(samples.len())
                            & !1;
                        accumulator.extend_from_slice(&samples[..take]);
                    }
                    (2, 1) => {
                        for &sample in samples {
                            if accumulator.len() + 2 > MIXER_ACCUMULATOR_CAPACITY {
                                break;
                            }
                            accumulator.extend_from_slice(&[sample, sample]);
                        }
                    }
                    (1, 2) => {
                        for pair in samples.as_chunks::<2>().0 {
                            if accumulator.len() == MIXER_ACCUMULATOR_CAPACITY {
                                break;
                            }
                            accumulator.push(0.5 * (pair[0] + pair[1]));
                        }
                    }
                    (1, 1) => {
                        let take = MIXER_ACCUMULATOR_CAPACITY
                            .saturating_sub(accumulator.len())
                            .min(samples.len());
                        accumulator.extend_from_slice(&samples[..take]);
                    }
                    _ => {}
                }
            }
        }

        let mut mixed = [0.0_f32; MIXER_MAX_FRAME_SAMPLES];
        let mixed = &mut mixed[..self.output_samples];
        let mut contributors = 0_u32;
        let mut output_timestamp = None;
        for (accumulator, timestamp) in self.accumulators.iter_mut().zip(&mut self.timestamps) {
            if accumulator.len() < self.output_samples {
                self.telemetry
                    .lane_underruns
                    .fetch_add(1, Ordering::Relaxed);
                continue;
            }
            for (output, input) in mixed.iter_mut().zip(accumulator.iter()) {
                *output += *input;
            }
            accumulator.drain(..self.output_samples);
            contributors += 1;
            if let Some(lane_timestamp) = *timestamp {
                output_timestamp = Some(
                    output_timestamp
                        .map_or(lane_timestamp, |current: u64| current.min(lane_timestamp)),
                );
                *timestamp = (!accumulator.is_empty())
                    .then_some(lane_timestamp.saturating_add(MIXER_FRAME_DURATION_NS));
            }
        }
        let scale = if contributors == 0 {
            0.0
        } else {
            1.0 / contributors as f32
        };
        let mut clipped = 0_u64;
        for sample in mixed.iter_mut() {
            let scaled = *sample * scale;
            let limited = scaled.clamp(-1.0, 1.0);
            clipped += u64::from(limited != scaled);
            *sample = limited;
        }
        self.telemetry
            .clipped_samples
            .fetch_add(clipped, Ordering::Relaxed);
        let Some(mut buffer) = self.pool.acquire() else {
            self.telemetry
                .output_pool_exhaustions
                .fetch_add(1, Ordering::Relaxed);
            return None;
        };
        buffer.try_copy_from_slice(mixed).ok()?;
        let sequence_number = self.sequence_number;
        self.sequence_number = self.sequence_number.saturating_add(1);
        self.telemetry.frames_mixed.fetch_add(1, Ordering::Relaxed);
        AudioFrame::try_new(
            stream_id,
            source_id,
            sequence_number,
            output_timestamp.unwrap_or(fallback_timestamp_ns),
            crate::frame::SampleSpec::new(
                48_000,
                self.output_channels,
                SampleFormat::F32Interleaved,
            ),
            buffer,
        )
        .ok()
    }
}

impl RuntimeNode for MixerSourceNode {
    fn prepare(&mut self, _context: &PrepareContext) -> Result<(), NodeError> {
        self.sequence_number = 0;
        for accumulator in &mut self.accumulators {
            accumulator.clear();
        }
        self.timestamps.fill(None);
        Ok(())
    }

    fn process(&mut self, clock: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        Ok(self.mix_tick(clock.stream_id(), clock.source_id(), clock.timestamp_ns()))
    }
}

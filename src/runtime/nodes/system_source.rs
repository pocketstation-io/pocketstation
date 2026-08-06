use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::frame::{AudioFrame, SampleFormat};
use crate::graph::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
use rtrb::{Consumer, Producer, RingBuffer};

use crate::graph::node::{
    ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PrepareContext,
};
use crate::graph::partition::ExecutionPartition;
use crate::graph::{NodeFactory, RuntimeNode, SafetyContract, SignalSpec};

const SYSTEM_OUTPUT_SOURCE_TYPE_ID: &str = "source.system_output";
const DEFAULT_RING_CAPACITY_FRAMES: usize = 8;

fn audio_output_port() -> PortSpec {
    PortSpec {
        name: "audio".to_owned(),
        direction: PortDirection::Output,
        signal: SignalSpec::audio(),
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(48_000),
            frame_samples: None,
            channel_layout: ChannelLayout::Mono,
            format: SampleFormat::F32Interleaved,
        }),
        multiplicity: Multiplicity::One,
        required: true,
    }
}

/// Shared counters exposed to callers for observability without logging on the
/// hot path. All updates use `Ordering::Relaxed` (monotonic counters; no
/// happens-before dependency on the values).
#[derive(Default)]
pub struct SystemOutputTelemetry {
    pub frames_captured: AtomicU64,
    pub frames_emitted: AtomicU64,
    pub underrun_count: AtomicU64,
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

/// Creates `SystemOutputSourceNode` instances that read captured system audio
/// from an SPSC ring buffer. Platform loopback capture callbacks (SCKit on
/// macOS, WASAPI loopback on Windows, PipeWire monitor on Linux) push
/// `AudioFrame`s into the `Producer` side; the graph executor pulls from the
/// `Consumer` side via `RuntimeNode::process()`.
///
/// Construction returns the factory, the `Producer` endpoint the capture thread
/// must write to, and a shared telemetry handle for observability.
///
/// The consumer is wrapped in `Mutex<Option<…>>` so the factory satisfies the
/// `Sync` bound required by `NodeFactory`. The mutex is locked only during
/// `instantiate()` (graph setup, off the hot path) and the consumer is moved
/// into the `SystemOutputSourceNode`; subsequent calls return an error.
pub struct SystemOutputSourceFactory {
    consumer: Mutex<Option<Consumer<AudioFrame>>>,
    telemetry: Arc<SystemOutputTelemetry>,
}

impl SystemOutputSourceFactory {
    pub fn new(capacity_frames: usize) -> (Self, Producer<AudioFrame>, Arc<SystemOutputTelemetry>) {
        let (producer, consumer) = RingBuffer::<AudioFrame>::new(capacity_frames);
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

    pub fn with_default_capacity() -> (Self, Producer<AudioFrame>, Arc<SystemOutputTelemetry>) {
        Self::new(DEFAULT_RING_CAPACITY_FRAMES)
    }
}

impl NodeFactory for SystemOutputSourceFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(SYSTEM_OUTPUT_SOURCE_TYPE_ID),
            display_name: "System Output Source",
            inputs: vec![],
            outputs: vec![audio_output_port()],
            execution: ExecutionPartition::AudioCallback,
            safety: SafetyContract::RealtimeSafe,
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
        let consumer = self
            .consumer
            .lock()
            .map_err(|e| NodeError::Prepare(format!("lock poisoned: {e}")))?
            .take()
            .ok_or_else(|| {
                NodeError::Prepare(
                    "SystemOutputSourceFactory consumer already taken; only one node per factory"
                        .to_owned(),
                )
            })?;
        Ok(Box::new(SystemOutputSourceNode {
            consumer,
            telemetry: Arc::clone(&self.telemetry),
            seq: 0,
        }))
    }
}

/// Reads captured system-output audio from the SPSC ring. When no frame is
/// available (underrun), the executor-provided frame is zeroed to produce
/// silence.
///
/// Hot-path contract: `process()` is alloc-free, lock-free, log-free,
/// blocking-free. The only operations are an `rtrb::Consumer::pop` (single
/// atomic CAS) and atomic counter increments.
pub struct SystemOutputSourceNode {
    consumer: Consumer<AudioFrame>,
    telemetry: Arc<SystemOutputTelemetry>,
    seq: u64,
}

impl RuntimeNode for SystemOutputSourceNode {
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        self.seq = 0;
        Ok(())
    }

    fn process(&mut self, mut frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        match self.consumer.pop() {
            Ok(captured) => {
                self.telemetry
                    .frames_captured
                    .fetch_add(1, Ordering::Relaxed);
                self.telemetry
                    .frames_emitted
                    .fetch_add(1, Ordering::Relaxed);
                self.seq += 1;
                Ok(Some(captured))
            }
            Err(_) => {
                // Underrun: zero the executor-provided frame to emit silence.
                self.telemetry
                    .underrun_count
                    .fetch_add(1, Ordering::Relaxed);
                self.telemetry
                    .frames_emitted
                    .fetch_add(1, Ordering::Relaxed);
                let buf = frame.buffer.as_mut_slice();
                for s in buf.iter_mut() {
                    *s = 0.0;
                }
                self.seq += 1;
                Ok(Some(frame))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{AudioBufferPool, SampleSpec, SourceId, StreamId};

    const FRAME_SAMPLES: usize = 960;

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn silent_frame(samples: usize) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples);
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&vec![0.0f32; samples]);
        AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle)
    }

    fn tone_frame(samples: usize, amplitude: f32) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples);
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&vec![amplitude; samples]);
        AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, handle)
    }

    #[test]
    fn given_system_source_with_captured_frame_when_processed_then_returns_captured_audio() {
        let (factory, mut producer, telemetry) = SystemOutputSourceFactory::new(4);
        let cx = prepare_cx();
        let mut node = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        node.prepare(&cx).unwrap();

        let captured = tone_frame(FRAME_SAMPLES, 0.5);
        producer.push(captured).unwrap();

        let out = node.process(silent_frame(FRAME_SAMPLES)).unwrap().unwrap();
        let peak = out
            .buffer
            .as_slice()
            .iter()
            .fold(0.0f32, |m, &s| m.max(s.abs()));
        assert!(
            (peak - 0.5).abs() < 1e-6,
            "expected captured audio (0.5), got peak={peak}"
        );
        assert_eq!(telemetry.frames_captured(), 1);
        assert_eq!(telemetry.frames_emitted(), 1);
        assert_eq!(telemetry.underrun_count(), 0);
    }

    #[test]
    fn given_system_source_with_empty_ring_when_processed_then_returns_silence_and_increments_underrun(
    ) {
        let (factory, _producer, telemetry) = SystemOutputSourceFactory::new(4);
        let cx = prepare_cx();
        let mut node = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        node.prepare(&cx).unwrap();

        let out = node
            .process(tone_frame(FRAME_SAMPLES, 0.9))
            .unwrap()
            .unwrap();
        let peak = out
            .buffer
            .as_slice()
            .iter()
            .fold(0.0f32, |m, &s| m.max(s.abs()));
        assert!(peak < 1e-6, "expected silence on underrun, got peak={peak}");
        assert_eq!(telemetry.frames_captured(), 0);
        assert_eq!(telemetry.frames_emitted(), 1);
        assert_eq!(telemetry.underrun_count(), 1);
    }

    #[test]
    fn given_system_factory_when_instantiated_twice_then_second_call_returns_error() {
        let (factory, _producer, _telemetry) = SystemOutputSourceFactory::new(4);
        let cx = prepare_cx();
        let _first = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        let second = factory.instantiate(&cx, &NodeConfig::new());
        assert!(second.is_err());
    }

    #[test]
    fn given_system_source_when_multiple_frames_pushed_then_delivered_in_order() {
        let (factory, mut producer, telemetry) = SystemOutputSourceFactory::new(8);
        let cx = prepare_cx();
        let mut node = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        node.prepare(&cx).unwrap();

        let frame_a = tone_frame(FRAME_SAMPLES, 0.1);
        let frame_b = tone_frame(FRAME_SAMPLES, 0.9);
        producer.push(frame_a).unwrap();
        producer.push(frame_b).unwrap();

        let out_a = node.process(silent_frame(FRAME_SAMPLES)).unwrap().unwrap();
        let out_b = node.process(silent_frame(FRAME_SAMPLES)).unwrap().unwrap();

        let peak_a = out_a
            .buffer
            .as_slice()
            .iter()
            .fold(0.0f32, |m, &s| m.max(s.abs()));
        let peak_b = out_b
            .buffer
            .as_slice()
            .iter()
            .fold(0.0f32, |m, &s| m.max(s.abs()));

        assert!(
            (peak_a - 0.1).abs() < 1e-6,
            "first frame wrong: peak={peak_a}"
        );
        assert!(
            (peak_b - 0.9).abs() < 1e-6,
            "second frame wrong: peak={peak_b}"
        );
        assert_eq!(telemetry.frames_captured(), 2);
        assert_eq!(telemetry.frames_emitted(), 2);
    }
}

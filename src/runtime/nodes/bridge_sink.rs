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

const BRIDGE_SINK_TYPE_ID: &str = "sink.bridge";
const DEFAULT_RING_CAPACITY_FRAMES: usize = 8;

fn audio_input_port() -> PortSpec {
    PortSpec {
        name: "in".to_owned(),
        direction: PortDirection::Input,
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
pub struct BridgeSinkTelemetry {
    pub frames_pushed: AtomicU64,
    pub overrun_count: AtomicU64,
}

impl BridgeSinkTelemetry {
    pub fn frames_pushed(&self) -> u64 {
        self.frames_pushed.load(Ordering::Relaxed)
    }
    pub fn overrun_count(&self) -> u64 {
        self.overrun_count.load(Ordering::Relaxed)
    }
}

/// Creates `BridgeSinkNode` instances that push processed audio into an SPSC
/// ring buffer. The graph executor pushes `AudioFrame`s into the `Producer`
/// side via `RuntimeNode::process()`; an external consumer (CLI, relay
/// transport, etc.) reads from the `Consumer` side.
///
/// Construction returns the factory, the `Consumer` endpoint the downstream
/// reader must pop from, and a shared telemetry handle for observability.
///
/// The producer is wrapped in `Mutex<Option<…>>` so the factory satisfies the
/// `Sync` bound required by `NodeFactory`. The mutex is locked only during
/// `instantiate()` (graph setup, off the hot path) and the producer is moved
/// into the `BridgeSinkNode`; subsequent calls return an error.
pub struct BridgeSinkFactory {
    producer: Mutex<Option<Producer<AudioFrame>>>,
    telemetry: Arc<BridgeSinkTelemetry>,
}

impl BridgeSinkFactory {
    pub fn new(capacity_frames: usize) -> (Self, Consumer<AudioFrame>, Arc<BridgeSinkTelemetry>) {
        let (producer, consumer) = RingBuffer::<AudioFrame>::new(capacity_frames);
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

    pub fn with_default_capacity() -> (Self, Consumer<AudioFrame>, Arc<BridgeSinkTelemetry>) {
        Self::new(DEFAULT_RING_CAPACITY_FRAMES)
    }
}

impl NodeFactory for BridgeSinkFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(BRIDGE_SINK_TYPE_ID),
            display_name: "Bridge Sink",
            inputs: vec![audio_input_port()],
            outputs: vec![],
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
        _cx: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        let producer = self
            .producer
            .lock()
            .map_err(|e| NodeError::Prepare(format!("lock poisoned: {e}")))?
            .take()
            .ok_or_else(|| {
                NodeError::Prepare(
                    "BridgeSinkFactory producer already taken; only one node per factory"
                        .to_owned(),
                )
            })?;
        Ok(Box::new(BridgeSinkNode {
            producer,
            telemetry: Arc::clone(&self.telemetry),
            seq: 0,
        }))
    }
}

/// Pushes processed audio into the SPSC ring for external consumption. When
/// the ring is full (overrun), the frame is dropped and the overrun counter is
/// incremented.
///
/// Hot-path contract: `process()` is alloc-free, lock-free, log-free,
/// blocking-free. The only operations are an `rtrb::Producer::push` (single
/// atomic CAS) and atomic counter increments.
pub struct BridgeSinkNode {
    producer: Producer<AudioFrame>,
    telemetry: Arc<BridgeSinkTelemetry>,
    seq: u64,
}

impl RuntimeNode for BridgeSinkNode {
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        self.seq = 0;
        Ok(())
    }

    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        match self.producer.push(frame) {
            Ok(()) => {
                self.telemetry.frames_pushed.fetch_add(1, Ordering::Relaxed);
                self.seq += 1;
                Ok(None)
            }
            Err(_) => {
                // Overrun: ring full, frame is dropped. No panic, no log.
                self.telemetry.overrun_count.fetch_add(1, Ordering::Relaxed);
                self.seq += 1;
                Ok(None)
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

    fn tone_frame(samples: usize, amplitude: f32) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples);
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&vec![amplitude; samples]);
        AudioFrame::new(StreamId(1), SourceId(1), 0, 0, 1, handle)
    }

    #[test]
    fn given_bridge_sink_when_frame_processed_then_consumer_receives_frame() {
        let (factory, mut consumer, telemetry) = BridgeSinkFactory::new(4);
        let cx = prepare_cx();
        let mut node = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        node.prepare(&cx).unwrap();

        let frame = tone_frame(FRAME_SAMPLES, 0.7);
        let result = node.process(frame).unwrap();
        assert!(result.is_none(), "sink must return None from process()");

        let received = consumer.pop().unwrap();
        let peak = received
            .buffer
            .as_slice()
            .iter()
            .fold(0.0f32, |m, &s| m.max(s.abs()));
        assert!(
            (peak - 0.7).abs() < 1e-6,
            "expected amplitude 0.7, got peak={peak}"
        );
        assert_eq!(telemetry.frames_pushed(), 1);
        assert_eq!(telemetry.overrun_count(), 0);
    }

    #[test]
    fn given_bridge_sink_when_ring_full_then_overrun_counted_and_no_panic() {
        let ring_capacity = 2;
        let (factory, _consumer, telemetry) = BridgeSinkFactory::new(ring_capacity);
        let cx = prepare_cx();
        let mut node = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        node.prepare(&cx).unwrap();

        // Fill the ring to capacity.
        for _ in 0..ring_capacity {
            let frame = tone_frame(FRAME_SAMPLES, 0.5);
            node.process(frame).unwrap();
        }
        assert_eq!(telemetry.frames_pushed(), ring_capacity as u64);
        assert_eq!(telemetry.overrun_count(), 0);

        // Next push should overrun.
        let overflow = tone_frame(FRAME_SAMPLES, 0.5);
        let result = node.process(overflow).unwrap();
        assert!(result.is_none(), "sink must return None even on overrun");
        assert_eq!(telemetry.frames_pushed(), ring_capacity as u64);
        assert_eq!(telemetry.overrun_count(), 1);
    }

    #[test]
    fn given_bridge_factory_when_instantiated_twice_then_second_returns_error() {
        let (factory, _consumer, _telemetry) = BridgeSinkFactory::new(4);
        let cx = prepare_cx();
        let _first = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        let second = factory.instantiate(&cx, &NodeConfig::new());
        assert!(second.is_err());
    }

    #[test]
    fn given_bridge_sink_when_multiple_frames_pushed_then_consumer_receives_in_order() {
        let (factory, mut consumer, telemetry) = BridgeSinkFactory::new(8);
        let cx = prepare_cx();
        let mut node = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        node.prepare(&cx).unwrap();

        let frame_a = tone_frame(FRAME_SAMPLES, 0.1);
        let frame_b = tone_frame(FRAME_SAMPLES, 0.9);
        node.process(frame_a).unwrap();
        node.process(frame_b).unwrap();

        let out_a = consumer.pop().unwrap();
        let out_b = consumer.pop().unwrap();

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
        assert_eq!(telemetry.frames_pushed(), 2);
    }
}

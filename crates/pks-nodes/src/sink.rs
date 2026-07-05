use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
use pks_frame::{AudioFrame, SampleFormat};

use pks_graph::node::{
    ConfigError, ExecutionClass, NodeConfig, NodeDescriptor, NodeError, NodeKind, NodeTypeId,
    PrepareContext,
};
use pks_graph::{NodeFactory, RuntimeNode};

const RECORDING_SINK_TYPE_ID: &str = "sink.recording";

fn audio_input_port() -> PortSpec {
    PortSpec {
        name: "in".to_owned(),
        direction: PortDirection::Input,
        media: MediaCaps::Audio(AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: ChannelLayout::Any,
            format: SampleFormat::F32Interleaved,
        }),
        multiplicity: Multiplicity::One,
        required: true,
    }
}

/// Records frame throughput. A real terminal sink (offline verification / soak
/// counting); it counts frames and samples consumed via a shared atomic tally so a
/// caller can read totals after a run. It passes the frame through so a graph can
/// fan a recorder alongside another sink.
pub struct RecordingSinkFactory {
    tally: Arc<RecordingTally>,
}

#[derive(Default)]
pub struct RecordingTally {
    pub frames_recorded: AtomicU64,
    pub samples_recorded: AtomicU64,
}

impl RecordingTally {
    pub fn frames_recorded(&self) -> u64 {
        self.frames_recorded.load(Ordering::Relaxed)
    }
    pub fn samples_recorded(&self) -> u64 {
        self.samples_recorded.load(Ordering::Relaxed)
    }
}

impl RecordingSinkFactory {
    pub fn new() -> (Self, Arc<RecordingTally>) {
        let tally = Arc::new(RecordingTally::default());
        (
            Self {
                tally: Arc::clone(&tally),
            },
            tally,
        )
    }
}

impl NodeFactory for RecordingSinkFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(RECORDING_SINK_TYPE_ID),
            display_name: "Recording Sink",
            kind: NodeKind::Sink,
            inputs: vec![audio_input_port()],
            outputs: vec![],
            execution: ExecutionClass::Recorder,
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
        Ok(Box::new(RecordingSinkNode {
            tally: Arc::clone(&self.tally),
        }))
    }
}

pub struct RecordingSinkNode {
    tally: Arc<RecordingTally>,
}

impl RuntimeNode for RecordingSinkNode {
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        self.tally.frames_recorded.fetch_add(1, Ordering::Relaxed);
        self.tally
            .samples_recorded
            .fetch_add(frame.buffer.len() as u64, Ordering::Relaxed);
        Ok(Some(frame))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_frame::{AudioBufferPool, SampleSpec, SourceId, StreamId};

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn frame(samples: usize) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples);
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&vec![0.2f32; samples]);
        AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle)
    }

    #[test]
    fn given_recording_sink_when_two_frames_processed_then_tally_counts_both() {
        let (factory, tally) = RecordingSinkFactory::new();
        let cx = prepare_cx();
        let mut node = factory.instantiate(&cx, &NodeConfig::new()).unwrap();
        node.process(frame(960)).unwrap();
        node.process(frame(960)).unwrap();
        assert_eq!(tally.frames_recorded(), 2);
        assert_eq!(tally.samples_recorded(), 1920);
    }
}

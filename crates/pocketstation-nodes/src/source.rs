use std::f32::consts::PI;

use pocketstation_caps::{
    AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec,
};
use pocketstation_frame::{AudioFrame, SampleFormat};

use pocketstation_graph::node::{
    ConfigError, ExecutionClass, NodeConfig, NodeDescriptor, NodeError, NodeKind, NodeTypeId,
    PrepareContext,
};
use pocketstation_graph::{NodeFactory, RuntimeNode};

const SYNTHETIC_SOURCE_TYPE_ID: &str = "source.synthetic";
const DEFAULT_TONE_HZ: f32 = 440.0;
const DEFAULT_AMPLITUDE: f32 = 0.25; // −12 dBFS; well clear of clipping

fn audio_output_port() -> PortSpec {
    PortSpec {
        name: "out".to_owned(),
        direction: PortDirection::Output,
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

/// Generates a steady sine tone. A real source (used for smoke tests and latency
/// measurement) — it overwrites the frame buffer with generated audio, ignoring input.
pub struct SyntheticSourceFactory;

impl NodeFactory for SyntheticSourceFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(SYNTHETIC_SOURCE_TYPE_ID),
            display_name: "Synthetic Source",
            kind: NodeKind::Source,
            inputs: vec![],
            outputs: vec![audio_output_port()],
            execution: ExecutionClass::RealtimeCpu,
            realtime_safe: true,
            stateful: true,
        }
    }

    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        if config.get("tone_hz").is_some() && config.get_f32("tone_hz").is_none() {
            return Err(ConfigError::Invalid {
                key: "tone_hz".to_owned(),
                reason: "must parse as f32".to_owned(),
            });
        }
        Ok(())
    }

    fn instantiate(
        &self,
        _cx: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        let tone_hz = config.get_f32("tone_hz").unwrap_or(DEFAULT_TONE_HZ);
        let amplitude = config.get_f32("amplitude").unwrap_or(DEFAULT_AMPLITUDE);
        Ok(Box::new(SyntheticSourceNode {
            tone_hz,
            amplitude,
            sample_rate_hz: 48_000,
            phase_samples: 0,
        }))
    }
}

pub struct SyntheticSourceNode {
    tone_hz: f32,
    amplitude: f32,
    sample_rate_hz: u32,
    phase_samples: u64,
}

impl RuntimeNode for SyntheticSourceNode {
    fn prepare(&mut self, cx: &PrepareContext) -> Result<(), NodeError> {
        self.sample_rate_hz = cx.sample_spec.sample_rate_hz;
        self.phase_samples = 0;
        Ok(())
    }

    fn process(&mut self, mut frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        let rate_hz = self.sample_rate_hz as f32;
        let buf = frame.buffer.as_mut_slice();
        for (i, s) in buf.iter_mut().enumerate() {
            let t = (self.phase_samples + i as u64) as f32 / rate_hz;
            *s = self.amplitude * (2.0 * PI * self.tone_hz * t).sin();
        }
        self.phase_samples += buf.len() as u64;
        Ok(Some(frame))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, SampleSpec, SourceId, StreamId};

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn silent_frame(samples: usize) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples);
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(&vec![0.0f32; samples]);
        AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle)
    }

    #[test]
    fn given_synthetic_source_when_processed_then_buffer_holds_nonzero_tone() {
        let cx = prepare_cx();
        let mut node = SyntheticSourceFactory
            .instantiate(&cx, &NodeConfig::new())
            .unwrap();
        node.prepare(&cx).unwrap();
        let out = node.process(silent_frame(960)).unwrap().unwrap();
        let peak = out
            .buffer
            .as_slice()
            .iter()
            .fold(0.0f32, |m, &s| m.max(s.abs()));
        assert!(peak > 0.1, "expected an audible tone, peak={peak}");
    }

    #[test]
    fn given_invalid_tone_config_when_validate_then_config_error() {
        let bad = NodeConfig::new().with("tone_hz", "notanumber");
        assert!(SyntheticSourceFactory.validate_config(&bad).is_err());
    }
}

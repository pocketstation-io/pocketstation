use pks_caps::{AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec};
use pks_frame::{AudioFrame, SampleFormat};

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

#[cfg(test)]
mod tests {
    use super::*;
    use pks_frame::{AudioBufferPool, SampleSpec, SourceId, StreamId};
    use proptest::prelude::*;

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved))
    }

    fn frame_with(samples: &[f32], channels: u8) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples.len());
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(samples);
        AudioFrame::new(StreamId(0), SourceId(0), 0, 0, channels, handle)
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

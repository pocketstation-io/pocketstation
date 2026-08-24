//! Provides the built-in passthrough, gain, and mono-mix graph nodes.
//!
//! The module exposes node factories, their runtime nodes, and registration
//! support for conformance and internal-testing builds. Application graph
//! composition uses the public graph contracts rather than depending on these
//! fixture implementations as a provider-extension guarantee.

use std::sync::Arc;

use crate::frame::{AudioFrame, SampleFormat};

use crate::graph::node::{
    ConfigError, NodeConfig, NodeDescriptor, NodeError, NodeTypeId, PrepareContext,
};
use crate::graph::partition::{ExecutionPartition, SafetyContract};
use crate::graph::ports::{
    AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec,
};
use crate::graph::registry::{NodeFactory, NodeRegistry};
use crate::graph::runtime_node::RuntimeNode;
use crate::graph::signal::SignalSpec;

pub(crate) const GAIN_CONFIGURATION_KEY: &str = "gain_db";
pub(crate) const PASSTHROUGH_NODE_TYPE_ID: &str = "passthrough";
pub(crate) const GAIN_NODE_TYPE_ID: &str = "gain";
pub(crate) const MONO_MIX_NODE_TYPE_ID: &str = "transform.mono_mix";
const MONO_CHANNEL_COUNT: u8 = 1;
const STEREO_CHANNEL_COUNT: u8 = 2;
const MONO_MIX_SCALE: f32 = 0.5;

fn any_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        signal: SignalSpec::any(),
        media: MediaCaps::Any,
        multiplicity: Multiplicity::One,
        required: true,
    }
}

fn audio_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        signal: SignalSpec::audio(),
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

fn mono_audio_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        signal: SignalSpec::audio(),
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

#[doc = "Constructs passthrough implementations from validated declarations."]
pub struct PassthroughFactory;

impl NodeFactory for PassthroughFactory {
    #[doc = "Returns the descriptor held by `PassthroughFactory`."]
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(PASSTHROUGH_NODE_TYPE_ID),
            display_name: "Passthrough",
            inputs: vec![any_port("in", PortDirection::Input)],
            outputs: vec![any_port("out", PortDirection::Output)],
            execution: ExecutionPartition::RealtimeCpu,
            safety: SafetyContract::RealtimeSafe,
            stateful: false,
        }
    }

    #[doc = "Validates supplied node configuration against the schema declared by `PassthroughFactory`."]
    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    #[doc = "Instantiates the runtime node described by `PassthroughFactory`."]
    fn instantiate(
        &self,
        _cx: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        Ok(Box::new(PassthroughNode))
    }
}

#[doc = "Represents the executable graph node for passthrough."]
pub struct PassthroughNode;

impl RuntimeNode for PassthroughNode {
    #[doc = "Prepares resources required by `PassthroughNode`."]
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    #[doc = "Processes an input value through `PassthroughNode`."]
    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        Ok(Some(frame))
    }
}

#[doc = "Constructs gain implementations from validated declarations."]
pub struct GainFactory;

impl NodeFactory for GainFactory {
    #[doc = "Returns the descriptor held by `GainFactory`."]
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(GAIN_NODE_TYPE_ID),
            display_name: "Gain",
            inputs: vec![audio_port("in", PortDirection::Input)],
            outputs: vec![audio_port("out", PortDirection::Output)],
            execution: ExecutionPartition::RealtimeCpu,
            safety: SafetyContract::RealtimeSafe,
            stateful: true,
        }
    }

    #[doc = "Validates supplied node configuration against the schema declared by `GainFactory`."]
    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        match config.get(GAIN_CONFIGURATION_KEY) {
            None => Err(ConfigError::Missing(GAIN_CONFIGURATION_KEY.to_owned())),
            Some(raw) => raw
                .parse::<f32>()
                .map(|_| ())
                .map_err(|err| ConfigError::Invalid {
                    key: GAIN_CONFIGURATION_KEY.to_owned(),
                    reason: err.to_string(),
                }),
        }
    }

    #[doc = "Instantiates the runtime node described by `GainFactory`."]
    fn instantiate(
        &self,
        _cx: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        self.validate_config(config)?;
        let gain_db = config
            .get_f32(GAIN_CONFIGURATION_KEY)
            .ok_or_else(|| ConfigError::Missing(GAIN_CONFIGURATION_KEY.to_owned()))?;
        let gain_ratio = 10f32.powf(gain_db / 20.0);
        Ok(Box::new(GainNode { gain_ratio }))
    }
}

#[doc = "Represents the executable graph node for gain."]
pub struct GainNode {
    gain_ratio: f32, // dimensionless amplitude ratio derived from gain_db
}

impl RuntimeNode for GainNode {
    #[doc = "Prepares resources required by `GainNode`."]
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    #[doc = "Processes an input value through `GainNode`."]
    fn process(&mut self, mut frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        for sample in frame.buffer.as_mut_slice() {
            *sample *= self.gain_ratio;
        }
        Ok(Some(frame))
    }
}

#[doc = "Constructs mono mix implementations from validated declarations."]
pub struct MonoMixFactory;

impl NodeFactory for MonoMixFactory {
    #[doc = "Returns the descriptor held by `MonoMixFactory`."]
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from(MONO_MIX_NODE_TYPE_ID),
            display_name: "Mono Mix",
            inputs: vec![audio_port("in", PortDirection::Input)],
            outputs: vec![mono_audio_port("out", PortDirection::Output)],
            execution: ExecutionPartition::RealtimeCpu,
            safety: SafetyContract::RealtimeSafe,
            stateful: false,
        }
    }

    #[doc = "Validates supplied node configuration against the schema declared by `MonoMixFactory`."]
    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    #[doc = "Instantiates the runtime node described by `MonoMixFactory`."]
    fn instantiate(
        &self,
        _cx: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        Ok(Box::new(MonoMixNode))
    }
}

#[doc = "Represents the executable graph node for mono mix."]
pub struct MonoMixNode;

impl RuntimeNode for MonoMixNode {
    #[doc = "Prepares resources required by `MonoMixNode`."]
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    #[doc = "Processes an input value through `MonoMixNode`."]
    fn process(&mut self, mut frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        if frame.channels == STEREO_CHANNEL_COUNT {
            let samples = frame.buffer.as_mut_slice();
            let mono_sample_count = samples.len() / STEREO_CHANNEL_COUNT as usize;
            for sample_index in 0..mono_sample_count {
                let stereo_index = sample_index * STEREO_CHANNEL_COUNT as usize;
                samples[sample_index] =
                    MONO_MIX_SCALE * (samples[stereo_index] + samples[stereo_index + 1]);
            }
            frame
                .buffer
                .try_set_len(mono_sample_count)
                .map_err(|error| NodeError::Process(error.to_string()))?;
            frame.channels = MONO_CHANNEL_COUNT;
        }
        Ok(Some(frame))
    }
}

#[doc = "Registers the passthrough, gain, and mono-mix node factories in the supplied registry."]
pub fn register_builtins(
    registry: &mut NodeRegistry,
) -> Result<(), crate::graph::NodeRegistrationError> {
    registry.register(Arc::new(PassthroughFactory))?;
    registry.register(Arc::new(GainFactory))?;
    registry.register(Arc::new(MonoMixFactory))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{AudioBufferPool, SampleSpec, SourceId, StreamId};

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn frame_with_samples(samples: &[f32]) -> AudioFrame {
        frame_with_channels(samples, MONO_CHANNEL_COUNT)
    }

    fn frame_with_channels(samples: &[f32], channels: u8) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples.len());
        let mut handle = pool.acquire().unwrap();
        handle
            .try_copy_from_slice(samples)
            .expect("test samples fit the fixed-capacity buffer");
        AudioFrame::new(StreamId(0), SourceId(0), 0, 0, channels, handle)
    }

    #[test]
    fn given_gain_config_without_gain_db_when_validate_then_missing_error() {
        let factory = GainFactory;
        let result = factory.validate_config(&NodeConfig::new());
        assert!(matches!(result, Err(ConfigError::Missing(_))));
    }

    #[test]
    fn given_gain_config_with_non_numeric_gain_db_when_validate_then_invalid_error() {
        let factory = GainFactory;
        let config = NodeConfig::new().with(GAIN_CONFIGURATION_KEY, "loud");
        let result = factory.validate_config(&config);
        assert!(matches!(result, Err(ConfigError::Invalid { .. })));
    }

    #[test]
    fn given_gain_config_with_valid_gain_db_when_validate_then_ok() {
        let factory = GainFactory;
        let config = NodeConfig::new().with(GAIN_CONFIGURATION_KEY, "6.0");
        assert!(factory.validate_config(&config).is_ok());
    }

    #[test]
    fn given_unity_gain_node_when_process_then_samples_unchanged() {
        let factory = GainFactory;
        let config = NodeConfig::new().with(GAIN_CONFIGURATION_KEY, "0.0");
        let mut node = factory.instantiate(&prepare_cx(), &config).unwrap();
        let processed = node
            .process(frame_with_samples(&[0.5, -0.25, 1.0]))
            .unwrap()
            .unwrap();
        let out = processed.buffer.as_slice();
        assert!((out[0] - 0.5).abs() < 1e-6);
        assert!((out[1] + 0.25).abs() < 1e-6);
        assert!((out[2] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn given_six_db_gain_node_when_process_then_samples_scaled_by_linear_gain() {
        let factory = GainFactory;
        let config = NodeConfig::new().with(GAIN_CONFIGURATION_KEY, "6.0");
        let mut node = factory.instantiate(&prepare_cx(), &config).unwrap();
        let input = [0.5, -0.25, 1.0];
        let expected_linear = 10f32.powf(6.0 / 20.0);
        let processed = node.process(frame_with_samples(&input)).unwrap().unwrap();
        let out = processed.buffer.as_slice();
        for (got, raw) in out.iter().zip(input.iter()) {
            assert!((got - raw * expected_linear).abs() < 1e-5);
        }
    }

    #[test]
    fn given_passthrough_node_when_process_then_returns_frame_unchanged() {
        let factory = PassthroughFactory;
        let mut node = factory
            .instantiate(&prepare_cx(), &NodeConfig::new())
            .unwrap();
        let processed = node
            .process(frame_with_samples(&[0.1, 0.2, 0.3]))
            .unwrap()
            .unwrap();
        assert_eq!(processed.buffer.as_slice(), &[0.1, 0.2, 0.3]);
    }

    #[test]
    fn given_stereo_frame_when_mono_mixed_then_channels_and_samples_are_downmixed() {
        let mut node = MonoMixFactory
            .instantiate(&prepare_cx(), &NodeConfig::new())
            .unwrap();
        let processed = node
            .process(frame_with_channels(
                &[1.0, 0.0, 0.0, 1.0],
                STEREO_CHANNEL_COUNT,
            ))
            .unwrap()
            .unwrap();

        assert_eq!(processed.channels, MONO_CHANNEL_COUNT);
        assert_eq!(processed.buffer.as_slice(), &[0.5, 0.5]);
    }

    #[test]
    fn given_mono_frame_when_mono_mixed_then_frame_is_unchanged() {
        let mut node = MonoMixFactory
            .instantiate(&prepare_cx(), &NodeConfig::new())
            .unwrap();
        let input = [0.1, 0.2, 0.3];
        let processed = node.process(frame_with_samples(&input)).unwrap().unwrap();

        assert_eq!(processed.channels, MONO_CHANNEL_COUNT);
        assert_eq!(processed.buffer.as_slice(), &input);
    }
}

use std::sync::Arc;

use pocketstation_caps::{
    AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec,
};
use pocketstation_frame::{AudioFrame, SampleFormat};

use crate::node::{
    ConfigError, ExecutionClass, NodeConfig, NodeDescriptor, NodeError, NodeKind, NodeTypeId,
    PrepareContext,
};
use crate::registry::{NodeFactory, NodeRegistry};
use crate::runtime_node::RuntimeNode;

const GAIN_DB_KEY: &str = "gain_db";

fn any_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
        media: MediaCaps::Any,
        multiplicity: Multiplicity::One,
        required: true,
    }
}

fn audio_port(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_owned(),
        direction,
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

pub struct PassthroughFactory;

impl NodeFactory for PassthroughFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from("passthrough"),
            display_name: "Passthrough",
            kind: NodeKind::Transform,
            inputs: vec![any_port("in", PortDirection::Input)],
            outputs: vec![any_port("out", PortDirection::Output)],
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
        Ok(Box::new(PassthroughNode))
    }
}

pub struct PassthroughNode;

impl RuntimeNode for PassthroughNode {
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        Ok(Some(frame))
    }
}

pub struct GainFactory;

impl NodeFactory for GainFactory {
    fn descriptor(&self) -> NodeDescriptor {
        NodeDescriptor {
            type_id: NodeTypeId::from("gain"),
            display_name: "Gain",
            kind: NodeKind::Transform,
            inputs: vec![audio_port("in", PortDirection::Input)],
            outputs: vec![audio_port("out", PortDirection::Output)],
            execution: ExecutionClass::RealtimeCpu,
            realtime_safe: true,
            stateful: true,
        }
    }

    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        match config.get(GAIN_DB_KEY) {
            None => Err(ConfigError::Missing(GAIN_DB_KEY.to_owned())),
            Some(raw) => raw
                .parse::<f32>()
                .map(|_| ())
                .map_err(|err| ConfigError::Invalid {
                    key: GAIN_DB_KEY.to_owned(),
                    reason: err.to_string(),
                }),
        }
    }

    fn instantiate(
        &self,
        _cx: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        self.validate_config(config)?;
        let gain_db = config
            .get_f32(GAIN_DB_KEY)
            .ok_or_else(|| ConfigError::Missing(GAIN_DB_KEY.to_owned()))?;
        let gain_linear = 10f32.powf(gain_db / 20.0);
        Ok(Box::new(GainNode { gain_linear }))
    }
}

pub struct GainNode {
    gain_linear: f32, // dimensionless amplitude ratio derived from gain_db
}

impl RuntimeNode for GainNode {
    fn prepare(&mut self, _cx: &PrepareContext) -> Result<(), NodeError> {
        Ok(())
    }

    fn process(&mut self, mut frame: AudioFrame) -> Result<Option<AudioFrame>, NodeError> {
        for sample in frame.buffer.as_mut_slice() {
            *sample *= self.gain_linear;
        }
        Ok(Some(frame))
    }
}

pub fn register_builtins(registry: &mut NodeRegistry) {
    registry.register(Arc::new(PassthroughFactory));
    registry.register(Arc::new(GainFactory));
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::{AudioBufferPool, SampleSpec, SourceId, StreamId};

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn frame_with_samples(samples: &[f32]) -> AudioFrame {
        let pool = AudioBufferPool::new(1, samples.len());
        let mut handle = pool.acquire().unwrap();
        handle.copy_from_slice(samples);
        AudioFrame::new(StreamId(0), SourceId(0), 0, 0, 1, handle)
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
        let config = NodeConfig::new().with(GAIN_DB_KEY, "loud");
        let result = factory.validate_config(&config);
        assert!(matches!(result, Err(ConfigError::Invalid { .. })));
    }

    #[test]
    fn given_gain_config_with_valid_gain_db_when_validate_then_ok() {
        let factory = GainFactory;
        let config = NodeConfig::new().with(GAIN_DB_KEY, "6.0");
        assert!(factory.validate_config(&config).is_ok());
    }

    #[test]
    fn given_unity_gain_node_when_process_then_samples_unchanged() {
        let factory = GainFactory;
        let config = NodeConfig::new().with(GAIN_DB_KEY, "0.0");
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
        let config = NodeConfig::new().with(GAIN_DB_KEY, "6.0");
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
}

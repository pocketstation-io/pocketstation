use pocketstation_caps::{
    AudioCaps, ChannelLayout, MediaCaps, Multiplicity, PortDirection, PortSpec,
};
use pocketstation_frame::SampleFormat;

use pocketstation_graph::node::{
    ConfigError, ExecutionClass, NodeConfig, NodeDescriptor, NodeError, NodeKind, NodeTypeId,
    PrepareContext,
};
use pocketstation_graph::{NodeFactory, RuntimeNode};
use pocketstation_ml::{AudioWatermark, EchoCanceller, NoiseSuppressor, VadProcessor};

const VAD_TYPE_ID: &str = "transform.vad";
const NOISE_SUPPRESS_TYPE_ID: &str = "transform.noise_suppress";
const ECHO_CANCEL_TYPE_ID: &str = "transform.echo_cancel";
const WATERMARK_TYPE_ID: &str = "transform.watermark";

const THRESHOLD_DBFS_KEY: &str = "threshold_dbfs";
const VAD_ATTACK_MS: f32 = 5.0;
const VAD_RELEASE_MS: f32 = 150.0;
const VAD_HANGOVER_MS: f32 = 200.0;
const VAD_GATE_OUTPUT: bool = true;

const SESSION_TOKEN_KEY: &str = "session_token";
const ALPHA_KEY: &str = "alpha";
const WATERMARK_SESSION_TOKEN: u32 = 0xDEAD_BEEF;
const WATERMARK_ALPHA: f32 = 0.002; // inaudible embedding amplitude; matches AudioWatermark default
const WATERMARK_DETECT_THRESHOLD: f32 = 0.3; // normalized-correlation detection gate

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

fn parse_f32(config: &NodeConfig, key: &str) -> Result<(), ConfigError> {
    match config.get(key) {
        None => Ok(()),
        Some(raw) => raw
            .parse::<f32>()
            .map(|_| ())
            .map_err(|err| ConfigError::Invalid {
                key: key.to_owned(),
                reason: err.to_string(),
            }),
    }
}

fn parse_u32(config: &NodeConfig, key: &str) -> Result<(), ConfigError> {
    match config.get(key) {
        None => Ok(()),
        Some(raw) => raw
            .parse::<u32>()
            .map(|_| ())
            .map_err(|err| ConfigError::Invalid {
                key: key.to_owned(),
                reason: err.to_string(),
            }),
    }
}

fn transform_descriptor(type_id: &str, display_name: &'static str) -> NodeDescriptor {
    NodeDescriptor {
        type_id: NodeTypeId::from(type_id),
        display_name,
        kind: NodeKind::Transform,
        inputs: vec![audio_port("in", PortDirection::Input)],
        outputs: vec![audio_port("out", PortDirection::Output)],
        execution: ExecutionClass::RealtimeCpu,
        realtime_safe: true,
        stateful: true,
    }
}

pub struct VadFactory;

impl NodeFactory for VadFactory {
    fn descriptor(&self) -> NodeDescriptor {
        transform_descriptor(VAD_TYPE_ID, "Voice Activity Detector")
    }

    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        parse_f32(config, THRESHOLD_DBFS_KEY)
    }

    fn instantiate(
        &self,
        _cx: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        self.validate_config(config)?;
        let node = match config.get_f32(THRESHOLD_DBFS_KEY) {
            Some(threshold_dbfs) => VadProcessor::new(
                threshold_dbfs,
                VAD_ATTACK_MS,
                VAD_RELEASE_MS,
                VAD_HANGOVER_MS,
                VAD_GATE_OUTPUT,
            ),
            None => VadProcessor::default_config(),
        };
        Ok(Box::new(node))
    }
}

pub struct NoiseSuppressFactory;

impl NodeFactory for NoiseSuppressFactory {
    fn descriptor(&self) -> NodeDescriptor {
        transform_descriptor(NOISE_SUPPRESS_TYPE_ID, "Noise Suppressor")
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _cx: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        Ok(Box::new(NoiseSuppressor::default_config()))
    }
}

pub struct EchoCancelFactory;

impl NodeFactory for EchoCancelFactory {
    fn descriptor(&self) -> NodeDescriptor {
        transform_descriptor(ECHO_CANCEL_TYPE_ID, "Echo Canceller")
    }

    fn validate_config(&self, _config: &NodeConfig) -> Result<(), ConfigError> {
        Ok(())
    }

    fn instantiate(
        &self,
        _cx: &PrepareContext,
        _config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        Ok(Box::new(EchoCanceller::default_config()))
    }
}

pub struct WatermarkFactory;

impl NodeFactory for WatermarkFactory {
    fn descriptor(&self) -> NodeDescriptor {
        transform_descriptor(WATERMARK_TYPE_ID, "Audio Watermark")
    }

    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        parse_u32(config, SESSION_TOKEN_KEY)?;
        parse_f32(config, ALPHA_KEY)
    }

    fn instantiate(
        &self,
        _cx: &PrepareContext,
        config: &NodeConfig,
    ) -> Result<Box<dyn RuntimeNode>, NodeError> {
        self.validate_config(config)?;
        let session_token = config
            .get_u32(SESSION_TOKEN_KEY)
            .unwrap_or(WATERMARK_SESSION_TOKEN);
        let alpha = config.get_f32(ALPHA_KEY).unwrap_or(WATERMARK_ALPHA);
        Ok(Box::new(AudioWatermark::new(
            session_token,
            alpha,
            WATERMARK_DETECT_THRESHOLD,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_vad_factory_with_non_numeric_threshold_when_validate_then_config_error() {
        let config = NodeConfig::new().with(THRESHOLD_DBFS_KEY, "quiet");
        assert!(matches!(
            VadFactory.validate_config(&config),
            Err(ConfigError::Invalid { .. })
        ));
    }

    #[test]
    fn given_vad_factory_without_threshold_when_validate_then_ok() {
        assert!(VadFactory.validate_config(&NodeConfig::new()).is_ok());
    }

    #[test]
    fn given_watermark_factory_with_invalid_alpha_when_validate_then_config_error() {
        let config = NodeConfig::new().with(ALPHA_KEY, "notanumber");
        assert!(matches!(
            WatermarkFactory.validate_config(&config),
            Err(ConfigError::Invalid { .. })
        ));
    }

    #[test]
    fn given_watermark_factory_with_invalid_session_token_when_validate_then_config_error() {
        let config = NodeConfig::new().with(SESSION_TOKEN_KEY, "-1");
        assert!(matches!(
            WatermarkFactory.validate_config(&config),
            Err(ConfigError::Invalid { .. })
        ));
    }

    #[test]
    fn given_watermark_factory_with_valid_config_when_validate_then_ok() {
        let config = NodeConfig::new()
            .with(SESSION_TOKEN_KEY, "42")
            .with(ALPHA_KEY, "0.01");
        assert!(WatermarkFactory.validate_config(&config).is_ok());
    }
}

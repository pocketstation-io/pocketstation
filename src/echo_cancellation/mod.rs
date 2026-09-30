//! Built-in echo processing; only Session composition is public.
mod config;
mod observations;
mod processor;
mod worker;

pub(crate) use config::{AecConfiguration, ReferenceChannels};
pub(crate) use observations::ObservationState;
pub use observations::{EchoCancellationObservations, EchoCancellationState};

use crate::{
    AsyncNode, AsyncOperatorFactory, AsyncOperatorManifest, AudioCaps, ConfigError, CopyPolicy,
    ExecutionPartition, ExecutionSafety, MediaCaps, Multiplicity, NodeDescriptor, NodeError,
    NodeTypeId, OperatorCancellationPolicy, OperatorConfiguration, OperatorDeadlinePolicy,
    OperatorFailurePolicy, OperatorId, OperatorOutputRolePolicy, OperatorPermissionPolicy,
    PortDirection, PortSpec, RouteSettings, SampleFormat, SignalSpec,
};
use config::{PROCESS_DEADLINE_MS, SAMPLE_RATE_HZ};

pub(crate) struct AecOperatorFactory {
    observations: ObservationState,
    operator_id: OperatorId,
    configuration: AecConfiguration,
    manifest: AsyncOperatorManifest,
}

impl AecOperatorFactory {
    pub(crate) fn new(
        configuration: AecConfiguration,
        operator_id: OperatorId,
        observations: ObservationState,
    ) -> Result<Self, ConfigError> {
        configuration.validate()?;
        let audio = |layout| {
            MediaCaps::Audio(AudioCaps {
                sample_rate_hz: Some(SAMPLE_RATE_HZ),
                frame_samples: Some(configuration.frame_samples()),
                channel_layout: layout,
                format: SampleFormat::F32Interleaved,
            })
        };
        let microphone = audio(configuration.reference_channels.layout());
        let reference = audio(configuration.reference_channels.layout());
        let port = |name, direction, spec, media, required| {
            PortSpec::new(name, direction, spec, media, Multiplicity::One, required)
                .expect("constant validated AEC port")
        };
        let descriptor = NodeDescriptor::new(
            NodeTypeId::from(operator_id.as_str()),
            "Acoustic echo cancellation",
            vec![
                port(
                    "microphone",
                    PortDirection::Input,
                    SignalSpec::audio(),
                    microphone,
                    true,
                ),
                port(
                    "reference",
                    PortDirection::Input,
                    SignalSpec::audio(),
                    reference,
                    true,
                ),
            ],
            vec![port(
                "microphone",
                PortDirection::Output,
                SignalSpec::audio(),
                microphone,
                true,
            )],
            ExecutionPartition::BlockingWorker,
            ExecutionSafety::BlockingAllowed,
            true,
        )
        .expect("constant validated AEC descriptor");
        let manifest = AsyncOperatorManifest::new(
            operator_id.clone(),
            1,
            1,
            descriptor,
            RouteSettings::realtime_audio()
                .with_media(MediaCaps::Any)
                .with_copy_policy(CopyPolicy::CopyToBranchPool),
            RouteSettings::bounded_async().with_media(MediaCaps::Any),
            configuration.capacity_frames(),
            OperatorPermissionPolicy {
                network_allowed: false,
                filesystem_allowed: false,
            },
            OperatorDeadlinePolicy {
                process_timeout_ms: PROCESS_DEADLINE_MS,
            },
            OperatorCancellationPolicy::DiscardQueued,
            OperatorFailurePolicy::StopWorker,
            OperatorOutputRolePolicy::default(),
        )
        .expect("constant validated AEC manifest");
        Ok(Self {
            observations,
            operator_id,
            configuration,
            manifest,
        })
    }
}

impl AsyncOperatorFactory for AecOperatorFactory {
    fn manifest(&self) -> &AsyncOperatorManifest {
        &self.manifest
    }
    fn validate_config(&self, configuration: &OperatorConfiguration) -> Result<(), ConfigError> {
        if configuration.iter().next().is_some() {
            return Err(ConfigError::Invalid {
                key: "configuration".into(),
                reason: "Session echo cancellation does not accept free-form configuration".into(),
            });
        }
        self.configuration.validate()
    }
    fn create(
        &self,
        configuration: &OperatorConfiguration,
    ) -> Result<Box<dyn AsyncNode>, NodeError> {
        self.validate_config(configuration)
            .map_err(|e| NodeError::Prepare(e.to_string()))?;
        Ok(Box::new(worker::AecWorker::new(
            self.configuration,
            self.operator_id.clone(),
            self.observations.clone(),
        )))
    }
}

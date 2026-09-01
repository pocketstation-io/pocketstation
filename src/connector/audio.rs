use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::graph::{
    AudioCaps, ChannelLayout, ExecutionPartition, MediaCaps, Multiplicity, NodeDescriptor,
    NodeTypeId, OperatorId, PortDirection, PortSpec, SafetyContract, SignalSpec,
};
use crate::{
    EndpointAudioFrame, EndpointGroupId, EndpointPreparationGroup, EndpointShutdownMode, RouteId,
    SampleFormat,
};

use super::{
    Connector, ConnectorConfigurationSchema, ConnectorContext, ConnectorDeliveryOutcome,
    ConnectorDriver, ConnectorDriverFactory, ConnectorError, ConnectorErrorStage,
    ConnectorInputDescriptor, ConnectorItem, ConnectorManifest, ConnectorManifestError,
    ConnectorReadinessPolicy, ResolvedConnectorConfiguration,
};

const AUDIO_CONNECTOR_GROUP_PREFIX: &str = "local-audio-connector";
static NEXT_AUDIO_CONNECTOR_ID: AtomicU64 = AtomicU64::new(1);

/// Application-owned audio delivery executed by Core's bounded Connector worker.
///
/// Implementations own provider resources and protocol behavior. Core owns route
/// queues, source-aware frames, readiness, drain or abort, panic containment,
/// and joined shutdown.
pub trait AudioConnector: Send + 'static {
    fn start(&mut self) -> Result<(), ConnectorError> {
        Ok(())
    }

    fn send(&mut self, frame: &EndpointAudioFrame) -> Result<(), ConnectorError>;

    fn stop(&mut self) -> Result<(), ConnectorError> {
        Ok(())
    }
}

pub(super) fn from_audio<P>(provider: P) -> Result<Connector, AudioConnectorBuildError>
where
    P: AudioConnector,
{
    let instance_id = NEXT_AUDIO_CONNECTOR_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
            value.checked_add(1)
        })
        .map_err(|_| AudioConnectorBuildError::IdentityExhausted)?;
    let manifest = audio_manifest(instance_id)?;
    let factory = Arc::new(AudioProviderFactory {
        provider: Mutex::new(Some(provider)),
        group: EndpointGroupId::new(format!("{AUDIO_CONNECTOR_GROUP_PREFIX}:{instance_id}")),
    });
    Ok(Connector::with_driver(manifest, factory)?)
}

pub(super) fn from_audio_fn<F>(send: F) -> Result<Connector, AudioConnectorBuildError>
where
    F: FnMut(&EndpointAudioFrame) -> Result<(), ConnectorError> + Send + 'static,
{
    from_audio(FunctionAudioConnector { send })
}

struct FunctionAudioConnector<F> {
    send: F,
}

impl<F> AudioConnector for FunctionAudioConnector<F>
where
    F: FnMut(&EndpointAudioFrame) -> Result<(), ConnectorError> + Send + 'static,
{
    fn send(&mut self, frame: &EndpointAudioFrame) -> Result<(), ConnectorError> {
        (self.send)(frame)
    }
}

struct AudioProviderFactory<P> {
    provider: Mutex<Option<P>>,
    group: EndpointGroupId,
}

impl<P> ConnectorDriverFactory for AudioProviderFactory<P>
where
    P: AudioConnector,
{
    fn preparation_group(
        &self,
        _route_id: RouteId,
        _configuration: &ResolvedConnectorConfiguration,
    ) -> Result<EndpointPreparationGroup, ConnectorError> {
        Ok(EndpointPreparationGroup::Shared(self.group.clone()))
    }

    fn prepare(
        &self,
        inputs: &[ConnectorInputDescriptor],
    ) -> Result<Box<dyn ConnectorDriver>, ConnectorError> {
        if inputs.is_empty() {
            return Err(ConnectorError::internal(
                "core.audio_connector.no_inputs",
                ConnectorErrorStage::Prepare,
                "audio Connector requires at least one input",
            ));
        }
        let provider = self
            .provider
            .lock()
            .map_err(|_| {
                ConnectorError::internal(
                    "core.audio_connector.provider_lock",
                    ConnectorErrorStage::Prepare,
                    "audio Connector provider ownership is unavailable",
                )
            })?
            .take()
            .ok_or_else(|| {
                ConnectorError::internal(
                    "core.audio_connector.provider_taken",
                    ConnectorErrorStage::Prepare,
                    "audio Connector provider was already prepared",
                )
            })?;
        Ok(Box::new(AudioProviderDriver {
            provider: Some(provider),
        }))
    }
}

struct AudioProviderDriver<P> {
    provider: Option<P>,
}

impl<P> ConnectorDriver for AudioProviderDriver<P>
where
    P: AudioConnector,
{
    fn start(&mut self, context: &ConnectorContext) -> Result<(), ConnectorError> {
        let result = self
            .provider
            .as_mut()
            .ok_or_else(provider_unavailable)?
            .start();
        if let Err(error) = result {
            self.close_after_failure();
            return Err(error);
        }
        let _ = context.set_ready();
        Ok(())
    }

    fn deliver(
        &mut self,
        item: ConnectorItem<'_>,
        _context: &ConnectorContext,
    ) -> Result<ConnectorDeliveryOutcome, ConnectorError> {
        let ConnectorItem::Audio { frame, .. } = item else {
            self.close_after_failure();
            return Err(ConnectorError::internal(
                "core.audio_connector.signal_mismatch",
                ConnectorErrorStage::Delivery,
                "audio Connector received a non-audio item",
            ));
        };
        let result = self
            .provider
            .as_mut()
            .ok_or_else(provider_unavailable)?
            .send(&frame);
        if let Err(error) = result {
            self.close_after_failure();
            return Err(error);
        }
        Ok(ConnectorDeliveryOutcome::Delivered)
    }

    fn shutdown(
        &mut self,
        _mode: EndpointShutdownMode,
        _context: &ConnectorContext,
    ) -> Result<(), ConnectorError> {
        self.close()
    }

    fn cancel_preparation(mut self: Box<Self>) -> Result<(), ConnectorError> {
        self.close()
    }
}

impl<P> AudioProviderDriver<P>
where
    P: AudioConnector,
{
    fn close(&mut self) -> Result<(), ConnectorError> {
        self.provider
            .take()
            .map_or(Ok(()), |mut provider| provider.stop())
    }

    fn close_after_failure(&mut self) {
        let _ = self.close();
    }
}

fn provider_unavailable() -> ConnectorError {
    ConnectorError::internal(
        "core.audio_connector.provider_unavailable",
        ConnectorErrorStage::Delivery,
        "audio Connector provider ownership is unavailable",
    )
}

fn audio_manifest(instance_id: u64) -> Result<ConnectorManifest, AudioConnectorBuildError> {
    let local_identity = format!("dev.pocketstation.local.audio_connector.{instance_id}.v1");
    let media = MediaCaps::Audio(AudioCaps {
        sample_rate_hz: None,
        frame_samples: None,
        channel_layout: ChannelLayout::Any,
        format: SampleFormat::F32Interleaved,
    });
    let input = PortSpec::new(
        "audio",
        PortDirection::Input,
        SignalSpec::audio(),
        media,
        Multiplicity::Many,
        true,
    )
    .map_err(|_| AudioConnectorBuildError::InvalidPortContract)?;
    let node = NodeDescriptor::new(
        NodeTypeId::from(local_identity.as_str()),
        "Application audio Connector",
        vec![input],
        Vec::new(),
        ExecutionPartition::AsyncWorker,
        SafetyContract::NetworkAllowed,
        true,
    )
    .map_err(|_| AudioConnectorBuildError::InvalidNodeContract)?;
    let configuration = ConnectorConfigurationSchema::new(1, Vec::new())
        .map_err(|_| AudioConnectorBuildError::InvalidConfigurationContract)?;
    let readiness =
        ConnectorReadinessPolicy::new(Duration::from_secs(10), Duration::from_millis(100), 1, 1)
            .map_err(|_| AudioConnectorBuildError::InvalidReadinessContract)?;
    Ok(ConnectorManifest::new(
        1,
        OperatorId::new(local_identity),
        env!("CARGO_PKG_VERSION"),
        node,
        configuration,
        readiness,
    )?)
}

#[derive(Debug, thiserror::Error)]
pub enum AudioConnectorBuildError {
    #[error("application-local audio Connector identity space is exhausted")]
    IdentityExhausted,
    #[error("PocketStation's audio Connector port contract is invalid")]
    InvalidPortContract,
    #[error("PocketStation's audio Connector node contract is invalid")]
    InvalidNodeContract,
    #[error("PocketStation's audio Connector configuration contract is invalid")]
    InvalidConfigurationContract,
    #[error("PocketStation's audio Connector readiness contract is invalid")]
    InvalidReadinessContract,
    #[error(transparent)]
    InvalidManifest(#[from] ConnectorManifestError),
}

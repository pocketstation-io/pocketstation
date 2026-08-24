use std::sync::Arc;

use crate::runtime::SidecarHost;
use crate::{
    EndpointGroupId, EndpointPreparationGroup, EndpointShutdownMode, RouteId, SidecarHostError,
    SidecarMessage, SidecarMessageKind, SidecarProcessSpec, SidecarProtocolLimits, SignalPayload,
};

use super::{
    ConnectorAudioRecord, ConnectorConfigurationRecord, ConnectorDeliveryOutcome, ConnectorDriver,
    ConnectorDriverFactory, ConnectorError, ConnectorErrorStage, ConnectorInputDescriptor,
    ConnectorItem, ConnectorRetryability, ResolvedConnectorConfiguration,
};

#[doc = "Defines connector audio record signal identifier as `\"io.pocketstation.connector.audio-record.v1\"` for the owning public contract."]
pub const CONNECTOR_AUDIO_RECORD_SIGNAL_ID: &str = "io.pocketstation.connector.audio-record.v1";
#[doc = "Defines connector audio record schema as `\"urn:pocketstation:connector:audio-record:v1\"` for the owning public contract."]
pub const CONNECTOR_AUDIO_RECORD_SCHEMA: &str = "urn:pocketstation:connector:audio-record:v1";

/// Adapts a bounded PocketStation sidecar process to the Connector driver SPI.
///
/// Endpoint and the Core connector worker retain lifecycle and receiver
/// authority. The sidecar host owns only its child process, bounded protocol
/// queues, handshake, and exact reaping.
#[derive(Debug, Clone)]
pub struct SidecarConnectorDriverFactory {
    process: SidecarProcessSpec,
}

impl SidecarConnectorDriverFactory {
    #[doc = "Creates a new `SidecarConnectorDriverFactory`."]
    pub const fn new(process: SidecarProcessSpec) -> Self {
        Self { process }
    }

    #[doc = "Processes an input value through `SidecarConnectorDriverFactory`."]
    pub const fn process(&self) -> &SidecarProcessSpec {
        &self.process
    }
}

impl ConnectorDriverFactory for SidecarConnectorDriverFactory {
    #[doc = "Returns the preparation group held by `SidecarConnectorDriverFactory`."]
    fn preparation_group(
        &self,
        _route_id: RouteId,
        _configuration: &ResolvedConnectorConfiguration,
    ) -> Result<EndpointPreparationGroup, ConnectorError> {
        Ok(EndpointPreparationGroup::Shared(EndpointGroupId::new(
            format!("sidecar:{}", self.process.id),
        )))
    }

    #[doc = "Prepares resources required by `SidecarConnectorDriverFactory`."]
    fn prepare(
        &self,
        inputs: &[ConnectorInputDescriptor],
    ) -> Result<Box<dyn ConnectorDriver>, ConnectorError> {
        if inputs.is_empty() {
            return Err(connector_error(
                "core.sidecar.no_inputs",
                ConnectorErrorStage::Prepare,
                ConnectorRetryability::Never,
                "sidecar connector requires at least one input",
            ));
        }
        let first_configuration = inputs[0].configuration();
        if inputs
            .iter()
            .any(|input| input.configuration() != first_configuration)
        {
            return Err(connector_error(
                "core.sidecar.configuration_mismatch",
                ConnectorErrorStage::Prepare,
                ConnectorRetryability::Never,
                "grouped sidecar connector inputs must share one resolved configuration",
            ));
        }
        let mut process = self.process.clone();
        process.configuration = ConnectorConfigurationRecord::from_resolved(first_configuration)
            .encode()
            .map_err(|error| {
                connector_error(
                    "core.sidecar.configuration_record",
                    ConnectorErrorStage::Prepare,
                    ConnectorRetryability::Never,
                    error.to_string(),
                )
            })?;
        let protocol_limits = process.protocol_limits;
        let host = SidecarHost::spawn(process).map_err(|error| {
            sidecar_error("core.sidecar.prepare", ConnectorErrorStage::Prepare, error)
        })?;
        Ok(Box::new(SidecarConnectorDriver {
            host: Some(host),
            protocol_limits,
        }))
    }
}

struct SidecarConnectorDriver {
    host: Option<SidecarHost>,
    protocol_limits: SidecarProtocolLimits,
}

impl ConnectorDriver for SidecarConnectorDriver {
    fn deliver(
        &mut self,
        item: ConnectorItem<'_>,
        _context: &super::ConnectorContext,
    ) -> Result<ConnectorDeliveryOutcome, ConnectorError> {
        let message = message_from_item(&item)?;
        message.validate(self.protocol_limits).map_err(|error| {
            connector_error(
                "core.sidecar.message_invalid",
                ConnectorErrorStage::Delivery,
                ConnectorRetryability::Never,
                error.to_string(),
            )
        })?;
        let host = self.host.as_ref().ok_or_else(|| {
            connector_error(
                "core.sidecar.closed",
                ConnectorErrorStage::Delivery,
                ConnectorRetryability::Retryable,
                "sidecar connector is closed",
            )
        })?;
        match host.try_send_signal(message) {
            Ok(()) => Ok(ConnectorDeliveryOutcome::Delivered),
            Err(SidecarHostError::DataQueueFull) => Ok(ConnectorDeliveryOutcome::Dropped),
            Err(error) => Err(sidecar_error(
                "core.sidecar.delivery",
                ConnectorErrorStage::Delivery,
                error,
            )),
        }
    }

    fn shutdown(
        &mut self,
        mode: EndpointShutdownMode,
        _context: &super::ConnectorContext,
    ) -> Result<(), ConnectorError> {
        let Some(mut host) = self.host.take() else {
            return Ok(());
        };
        let result = match mode {
            EndpointShutdownMode::Drain => host.close_and_reap(),
            EndpointShutdownMode::Abort => host.cancel_and_reap(),
        };
        result.map(|_| ()).map_err(|error| {
            sidecar_error(
                "core.sidecar.shutdown",
                ConnectorErrorStage::Shutdown,
                error,
            )
        })
    }

    fn cancel_preparation(mut self: Box<Self>) -> Result<(), ConnectorError> {
        let Some(mut host) = self.host.take() else {
            return Ok(());
        };
        host.cancel_and_reap().map(|_| ()).map_err(|error| {
            sidecar_error(
                "core.sidecar.cancel_preparation",
                ConnectorErrorStage::Shutdown,
                error,
            )
        })
    }
}

fn message_from_item(item: &ConnectorItem<'_>) -> Result<SidecarMessage, ConnectorError> {
    match item {
        ConnectorItem::Audio { frame, .. } => {
            let record = ConnectorAudioRecord::from_item(item).map_err(|error| {
                connector_error(
                    "core.sidecar.audio_record",
                    ConnectorErrorStage::Delivery,
                    ConnectorRetryability::Never,
                    error.to_string(),
                )
            })?;
            let payload = record.encode().map_err(|error| {
                connector_error(
                    "core.sidecar.audio_record",
                    ConnectorErrorStage::Delivery,
                    ConnectorRetryability::Never,
                    error.to_string(),
                )
            })?;
            Ok(SidecarMessage {
                kind: SidecarMessageKind::Signal,
                terminal: false,
                stream_id: frame.stream_id().get(),
                sequence_number: frame.sequence_number(),
                timestamp_ns: frame.timestamp_ns(),
                signal_id: CONNECTOR_AUDIO_RECORD_SIGNAL_ID.to_owned(),
                role: Some(item.input().port_name().to_owned()),
                schema: Some(CONNECTOR_AUDIO_RECORD_SCHEMA.to_owned()),
                payload,
            })
        }
        ConnectorItem::Signal { signal, .. } => {
            let payload = match signal.payload() {
                SignalPayload::Text(text) => text.as_bytes().to_vec(),
                SignalPayload::Bytes(bytes) => bytes.clone(),
                SignalPayload::Audio(_) => {
                    return Err(connector_error(
                        "core.sidecar.audio_signal_lane",
                        ConnectorErrorStage::Delivery,
                        ConnectorRetryability::Never,
                        "PCM connector input must use the bounded audio lane",
                    ));
                }
            };
            Ok(SidecarMessage {
                kind: SidecarMessageKind::Signal,
                terminal: false,
                stream_id: signal
                    .lineage()
                    .map_or(0, |lineage| lineage.stream_id().get()),
                sequence_number: signal.sequence_number().unwrap_or(0),
                timestamp_ns: signal.timestamp_ns(),
                signal_id: signal.signal_spec().wire_id().to_owned(),
                role: signal
                    .signal_spec()
                    .role()
                    .map(|role| role.as_str().to_owned()),
                schema: signal
                    .signal_spec()
                    .schema()
                    .map(|schema| schema.as_str().to_owned()),
                payload,
            })
        }
    }
}

fn sidecar_error(
    code: &'static str,
    stage: ConnectorErrorStage,
    error: SidecarHostError,
) -> ConnectorError {
    let retryability = match error {
        SidecarHostError::InvalidConfiguration(_)
        | SidecarHostError::Protocol(_)
        | SidecarHostError::FrameTooLarge
        | SidecarHostError::InvalidDataKind(_)
        | SidecarHostError::UnknownSidecar(_) => ConnectorRetryability::Never,
        SidecarHostError::Spawn(_) | SidecarHostError::MissingPipe(_) => {
            ConnectorRetryability::RetryAfterReconfiguration
        }
        _ => ConnectorRetryability::Retryable,
    };
    connector_error(code, stage, retryability, error.to_string())
}

fn connector_error(
    code: &'static str,
    stage: ConnectorErrorStage,
    retryability: ConnectorRetryability,
    message: impl Into<String>,
) -> ConnectorError {
    ConnectorError::internal_with_retryability(code, stage, retryability, message)
}

#[doc = "Creates a connector driver factory backed by the supplied sidecar process."]
pub fn sidecar_connector_factory(process: SidecarProcessSpec) -> Arc<dyn ConnectorDriverFactory> {
    Arc::new(SidecarConnectorDriverFactory::new(process))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned() {
        let factory = SidecarConnectorDriverFactory::new(SidecarProcessSpec::new(
            7,
            "unused-sidecar-program",
        ));
        let error = match factory.prepare(&[]) {
            Ok(_) => panic!("empty input group must fail"),
            Err(error) => error,
        };

        assert_eq!(error.code().as_str(), "core.sidecar.no_inputs");
        assert_eq!(error.stage(), ConnectorErrorStage::Prepare);
        assert_eq!(error.retryability(), ConnectorRetryability::Never);
    }

    #[test]
    fn given_sidecar_host_errors_when_classified_then_retryability_is_preserved() {
        let invalid = sidecar_error(
            "core.sidecar.prepare",
            ConnectorErrorStage::Prepare,
            SidecarHostError::InvalidConfiguration("invalid"),
        );
        assert_eq!(invalid.retryability(), ConnectorRetryability::Never);

        let missing = sidecar_error(
            "core.sidecar.prepare",
            ConnectorErrorStage::Prepare,
            SidecarHostError::Spawn("missing".to_owned()),
        );
        assert_eq!(
            missing.retryability(),
            ConnectorRetryability::RetryAfterReconfiguration
        );

        let transient = sidecar_error(
            "core.sidecar.delivery",
            ConnectorErrorStage::Delivery,
            SidecarHostError::Closed,
        );
        assert_eq!(transient.retryability(), ConnectorRetryability::Retryable);
    }
}

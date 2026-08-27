use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use pocketstation::connector::{
    Connector, ConnectorConfiguration, ConnectorConfigurationConstraint,
    ConnectorConfigurationField, ConnectorConfigurationRequirement, ConnectorConfigurationSchema,
    ConnectorConfigurationValue, ConnectorConfigurationValueKind, ConnectorDeliveryOutcome,
    ConnectorDriver, ConnectorDriverFactory, ConnectorError, ConnectorErrorCode,
    ConnectorErrorStage, ConnectorInputDescriptor, ConnectorItem, ConnectorManifest,
    ConnectorReadinessPolicy, ConnectorRetryability, ConnectorSecret,
};
use pocketstation::{
    ApplicationSelector, AudioCaps, ChannelLayout, EdgeContract, ExecutionPartition, MediaCaps,
    Multiplicity, NodeDescriptor, NodeTypeId, OperatorId, PortDirection, PortSpec, SafetyContract,
    SampleFormat, Session, SignalSpec, Source,
};

struct ExampleConnectorFactory;

impl ConnectorDriverFactory for ExampleConnectorFactory {
    fn prepare(
        &self,
        inputs: &[ConnectorInputDescriptor],
    ) -> Result<Box<dyn ConnectorDriver>, ConnectorError> {
        let configuration = inputs
            .first()
            .map(ConnectorInputDescriptor::configuration)
            .ok_or_else(|| {
                connector_error("example.input_missing", "connector input is missing")
            })?;
        if configuration.get("destination").is_none() || configuration.get("api_token").is_none() {
            return Err(connector_error(
                "example.configuration_missing",
                "validated connector configuration is missing",
            ));
        }
        Ok(Box::new(ExampleConnector))
    }
}

struct ExampleConnector;

impl ConnectorDriver for ExampleConnector {
    fn deliver(
        &mut self,
        item: ConnectorItem<'_>,
        _context: &pocketstation::connector::ConnectorContext,
    ) -> Result<ConnectorDeliveryOutcome, ConnectorError> {
        match item {
            ConnectorItem::Audio { input, frame } => println!(
                "port={} stem={} source={} sequence={}",
                input.port_name(),
                frame.lineage().stem_id().get(),
                frame.lineage().source_id().get(),
                frame.lineage().sequence_number()
            ),
            ConnectorItem::Signal { input, signal } => println!(
                "port={} signal_timestamp_ns={}",
                input.port_name(),
                signal.timestamp_ns()
            ),
        }
        Ok(ConnectorDeliveryOutcome::Delivered)
    }
}

fn connector_error(code: &str, message: &str) -> ConnectorError {
    let code = ConnectorErrorCode::new(code).unwrap_or_else(|_| {
        ConnectorErrorCode::new("example.invalid_error_code")
            .expect("static example error code is valid")
    });
    ConnectorError::new(
        code,
        ConnectorErrorStage::Prepare,
        ConnectorRetryability::Never,
        message,
    )
    .expect("static example error is valid")
}

fn connector_manifest() -> Result<ConnectorManifest, Box<dyn Error>> {
    let media = MediaCaps::Audio(AudioCaps {
        sample_rate_hz: None,
        frame_samples: None,
        channel_layout: ChannelLayout::Any,
        format: SampleFormat::F32Interleaved,
    });
    let node = NodeDescriptor::new(
        NodeTypeId::from("dev.example.connector.node.v1"),
        "Example audio connector",
        vec![PortSpec::new(
            "audio",
            PortDirection::Input,
            SignalSpec::audio(),
            media,
            Multiplicity::Many,
            true,
        )?],
        Vec::new(),
        ExecutionPartition::AsyncWorker,
        SafetyContract::NetworkAllowed,
        true,
    )?;
    let configuration = ConnectorConfigurationSchema::new(
        1,
        vec![
            ConnectorConfigurationField::new(
                "destination",
                ConnectorConfigurationValueKind::Text,
                ConnectorConfigurationRequirement::Required,
                "Remote connector destination.",
            )
            .with_constraint(ConnectorConfigurationConstraint::NonEmpty),
            ConnectorConfigurationField::new(
                "api_token",
                ConnectorConfigurationValueKind::Secret,
                ConnectorConfigurationRequirement::Required,
                "Remote authentication token.",
            )
            .with_constraint(ConnectorConfigurationConstraint::NonEmpty),
        ],
    )?;
    Ok(ConnectorManifest::new(
        1,
        OperatorId::new("dev.example.connector.v1"),
        env!("CARGO_PKG_VERSION"),
        node,
        configuration,
        ConnectorReadinessPolicy::new(Duration::from_secs(10), Duration::from_millis(250), 1, 3)?,
    )?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let application_name = std::env::args()
        .nth(1)
        .ok_or("provide the running application name or identifier as the first argument")?;
    let session = Session::new();
    let connector =
        Connector::with_driver(connector_manifest()?, Arc::new(ExampleConnectorFactory))?;
    let registered = session.register_connector(connector)?;
    let endpoint = registered.declare(
        &session,
        ConnectorConfiguration::new()
            .with(
                "destination",
                ConnectorConfigurationValue::Text("https://connector.example".to_owned()),
            )
            .with(
                "api_token",
                ConnectorConfigurationValue::Secret(ConnectorSecret::new("replace-me")?),
            ),
        EdgeContract::realtime_audio(),
    )?;

    let application = session.capture(Source::application(ApplicationSelector::name(
        application_name,
    )))?;
    let microphone = session.capture(Source::microphone_default())?;
    application.send(endpoint)?;
    microphone.send(endpoint)?;

    println!(
        "registered {} as connector {:?}",
        registered.manifest().operator_id().as_str(),
        endpoint.connector_id()
    );
    Ok(())
}

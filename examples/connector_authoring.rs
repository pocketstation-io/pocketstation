use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use pocketstation::connector::{
    Connector, ConnectorConfiguration, ConnectorConfigurationConstraint,
    ConnectorConfigurationField, ConnectorConfigurationRequirement, ConnectorConfigurationSchema,
    ConnectorConfigurationValue, ConnectorConfigurationValueKind, ConnectorContext, ConnectorError,
    ConnectorErrorCode, ConnectorErrorStage, ConnectorFactory, ConnectorManifest,
    ConnectorReadinessPolicy, ConnectorRetryability, ConnectorRunOutcome, ConnectorSecret,
    ConnectorWorker,
};
use pocketstation::{
    ApplicationSelector, AudioCaps, ChannelLayout, EdgeContract, EndpointPortInput,
    EndpointReceiver, ExecutionPartition, MediaCaps, Multiplicity, NodeDescriptor, NodeTypeId,
    OperatorId, PortDirection, PortSpec, SafetyContract, SampleFormat, Session, SignalSpec, Source,
};

struct ExampleConnectorFactory;

impl ConnectorFactory for ExampleConnectorFactory {
    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn ConnectorWorker>, ConnectorError> {
        let configuration = inputs
            .first()
            .map(|input| input.context().node_configuration())
            .ok_or_else(|| {
                connector_error("example.input_missing", "connector input is missing")
            })?;
        if configuration.get("destination").is_none() || configuration.get("api_token").is_none() {
            return Err(connector_error(
                "example.configuration_missing",
                "validated connector configuration is missing",
            ));
        }
        Ok(Box::new(ExampleConnectorWorker { inputs }))
    }
}

struct ExampleConnectorWorker {
    inputs: Vec<EndpointPortInput>,
}

impl ConnectorWorker for ExampleConnectorWorker {
    fn run(self: Box<Self>, context: ConnectorContext) -> ConnectorRunOutcome {
        let mut receivers: Vec<_> = self
            .inputs
            .into_iter()
            .filter_map(|input| match input.into_parts().0 {
                EndpointReceiver::Audio { receiver, .. } => Some(receiver),
                EndpointReceiver::Signal(_) => None,
            })
            .collect();
        let _ = context.set_ready();
        while !context.is_stop_requested() {
            let mut progressed = false;
            for receiver in &mut receivers {
                if receiver.try_recv().is_some() {
                    context.record_frame_received(1);
                    context.record_frame_delivered(1);
                    progressed = true;
                }
            }
            if !progressed {
                let _ = context.wait_for_stop(Duration::from_millis(1));
            }
        }
        ConnectorRunOutcome::success()
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
        EdgeContract::realtime_audio(),
        ConnectorReadinessPolicy::new(Duration::from_secs(10), Duration::from_millis(250), 1, 3)?,
    )?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let session = Session::new();
    let connector = Connector::new(connector_manifest()?, Arc::new(ExampleConnectorFactory))?;
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
    )?;

    let application = session.capture(Source::application(ApplicationSelector::name(
        "PocketStation Demo",
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

use std::error::Error;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use pocketstation::connector::{
    Connector, ConnectorConfiguration, ConnectorConfigurationConstraint,
    ConnectorConfigurationField, ConnectorConfigurationRequirement, ConnectorConfigurationSchema,
    ConnectorConfigurationValue, ConnectorConfigurationValueKind, ConnectorDeliveryPolicy,
    ConnectorManifest, ConnectorReadinessPolicy, ConnectorRetryPolicy, ConnectorSecret,
};
use pocketstation::{
    ApplicationSelector, AudioCaps, ChannelLayout, EdgeContract, EndpointCancellationOutcome,
    EndpointDriverFactory, EndpointDriverFinalization, EndpointDriverObservations, EndpointFailure,
    EndpointFailureStage, EndpointPortInput, EndpointReceiver, EndpointStartGate,
    ExecutionPartition, MediaCaps, Multiplicity, NodeDescriptor, NodeTypeId, OperatorId,
    PortDirection, PortSpec, PreparedEndpointDriver, RunningEndpointDriver, SafetyContract,
    SampleFormat, Session, SignalSpec, Source,
};

struct ExampleConnectorFactory;

impl EndpointDriverFactory for ExampleConnectorFactory {
    fn prepare(
        &self,
        inputs: Vec<EndpointPortInput>,
    ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
        let configuration = inputs
            .first()
            .map(|input| input.context().node_configuration())
            .ok_or_else(|| {
                EndpointFailure::new(EndpointFailureStage::Prepare, "connector input is missing")
            })?;
        if configuration.get("destination").is_none() || configuration.get("api_token").is_none() {
            return Err(EndpointFailure::new(
                EndpointFailureStage::Prepare,
                "validated connector configuration is missing",
            ));
        }
        Ok(Box::new(PreparedExampleConnector { inputs }))
    }
}

struct PreparedExampleConnector {
    inputs: Vec<EndpointPortInput>,
}

impl PreparedEndpointDriver for PreparedExampleConnector {
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
        let stop_requested = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop_requested);
        let delivered = Arc::new(AtomicU64::new(0));
        let worker_delivered = Arc::clone(&delivered);
        let mut receivers: Vec<_> = self
            .inputs
            .into_iter()
            .filter_map(|input| match input.into_parts().0 {
                EndpointReceiver::Audio { receiver, .. } => Some(receiver),
                EndpointReceiver::Signal(_) => None,
            })
            .collect();
        let worker = std::thread::Builder::new()
            .name("pocketstation-connector-example".to_owned())
            .spawn(move || {
                while !start_gate.is_open() && !worker_stop.load(Ordering::Acquire) {
                    std::thread::yield_now();
                }
                while !worker_stop.load(Ordering::Acquire) {
                    let mut received = false;
                    for receiver in &mut receivers {
                        if receiver.try_recv().is_some() {
                            worker_delivered.fetch_add(1, Ordering::Relaxed);
                            received = true;
                        }
                    }
                    if !received {
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                }
            })
            .map_err(|error| {
                EndpointFailure::new(
                    EndpointFailureStage::Start,
                    format!("connector worker could not start: {error}"),
                )
            })?;
        Ok(Box::new(RunningExampleConnector {
            stop_requested,
            delivered,
            worker: Some(worker),
        }))
    }

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
        EndpointCancellationOutcome {
            observations: EndpointDriverObservations::default(),
            result: Ok(()),
        }
    }
}

struct RunningExampleConnector {
    stop_requested: Arc<AtomicBool>,
    delivered: Arc<AtomicU64>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl RunningExampleConnector {
    fn observations(&self) -> EndpointDriverObservations {
        let delivered = self.delivered.load(Ordering::Acquire);
        EndpointDriverObservations {
            frames_received_total: delivered,
            frames_delivered_total: delivered,
            ..EndpointDriverObservations::default()
        }
    }
}

impl RunningEndpointDriver for RunningExampleConnector {
    fn observations(&self) -> EndpointDriverObservations {
        self.observations()
    }

    fn request_stop(&mut self) -> Result<(), EndpointFailure> {
        self.stop_requested.store(true, Ordering::Release);
        Ok(())
    }

    fn join_and_finalize(mut self: Box<Self>) -> EndpointDriverFinalization {
        self.stop_requested.store(true, Ordering::Release);
        let result = self.worker.take().map_or(Ok(()), |worker| {
            worker.join().map_err(|_| {
                EndpointFailure::new(
                    EndpointFailureStage::JoinFinalize,
                    "connector worker panicked",
                )
            })
        });
        EndpointDriverFinalization {
            observations: self.observations(),
            result,
        }
    }
}

impl Drop for RunningExampleConnector {
    fn drop(&mut self) {
        self.stop_requested.store(true, Ordering::Release);
        let _ = self.worker.take();
    }
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
        ConnectorDeliveryPolicy::new(EdgeContract::realtime_audio(), 64)?,
        ConnectorRetryPolicy::new(5, 2_000, 100, 2_000, 10_000, 2_000, 20)?,
        ConnectorReadinessPolicy::new(10_000, 250, 1, 3)?,
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

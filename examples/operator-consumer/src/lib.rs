//! Compile proof for an external provider and typed Endpoint consumer.
//!
//! This package intentionally depends only on the public `pocketstation`
//! façade and an example provider package.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use pocketstation::{
        EndpointCancellationOutcome, EndpointDescriptor, EndpointDriverFactory,
        EndpointDriverFinalization, EndpointDriverObservations, EndpointFailure,
        EndpointFailureStage, EndpointPortInput, EndpointReceiver, EndpointStartGate,
        ExecutionPartition, MediaCaps, Multiplicity, NodeDefinition, NodeDescriptor, NodeTypeId,
        Operator, OperatorConfiguration, OperatorId, PortDirection, PortSpec,
        ExecutionSafety, PreparedEndpointDriver, RunningEndpointDriver, Session, SignalSpec,
        Source,
        TextFormat,
    };
    use whisper_transcribe_example::{WhisperOperatorFactory, WHISPER_OPERATOR_ID};

    const TEXT_ENDPOINT_OPERATOR_ID: &str = "example.endpoint.transcript.v1";
    const TEXT_ENDPOINT_NODE_TYPE_ID: &str = "endpoint.transcript.example";

    struct TextEndpointDefinition;

    impl NodeDefinition for TextEndpointDefinition {
        fn descriptor(&self) -> NodeDescriptor {
            NodeDescriptor::new(
                NodeTypeId::from(TEXT_ENDPOINT_NODE_TYPE_ID),
                "Example transcript endpoint",
                vec![PortSpec::new(
                    "transcript",
                    PortDirection::Input,
                    SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                    MediaCaps::Text,
                    Multiplicity::One,
                    true,
                )
                .expect("text endpoint input")],
                Vec::new(),
                ExecutionPartition::External,
                ExecutionSafety::ExternalService,
                true,
            )
            .expect("text endpoint descriptor")
        }

        fn validate_config(
            &self,
            _configuration: &OperatorConfiguration,
        ) -> Result<(), pocketstation::ConfigError> {
            Ok(())
        }
    }

    struct TextEndpointFactory;

    impl EndpointDriverFactory for TextEndpointFactory {
        fn prepare(
            &self,
            inputs: Vec<EndpointPortInput>,
        ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
            if inputs
                .iter()
                .any(|input| !matches!(input.receiver(), EndpointReceiver::Signal(_)))
            {
                return Err(EndpointFailure::new(
                    EndpointFailureStage::Prepare,
                    "text endpoint requires a signal receiver",
                ));
            }
            Ok(Box::new(PreparedTextEndpoint { inputs }))
        }
    }

    struct PreparedTextEndpoint {
        inputs: Vec<EndpointPortInput>,
    }

    impl PreparedEndpointDriver for PreparedTextEndpoint {
        fn start(
            self: Box<Self>,
            _start_gate: Arc<EndpointStartGate>,
        ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure> {
            Ok(Box::new(RunningTextEndpoint {
                inputs: self.inputs,
                observations: EndpointDriverObservations::default(),
            }))
        }

        fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome {
            EndpointCancellationOutcome {
                observations: EndpointDriverObservations::default(),
                result: Ok(()),
            }
        }
    }

    struct RunningTextEndpoint {
        inputs: Vec<EndpointPortInput>,
        observations: EndpointDriverObservations,
    }

    impl RunningEndpointDriver for RunningTextEndpoint {
        fn observations(&self) -> EndpointDriverObservations {
            self.observations
        }

        fn request_stop(&mut self) -> Result<(), EndpointFailure> {
            Ok(())
        }

        fn join_and_finalize(self: Box<Self>) -> EndpointDriverFinalization {
            let _input_count = self.inputs.len();
            EndpointDriverFinalization {
                observations: self.observations,
                result: Ok(()),
            }
        }
    }

    #[test]
    fn given_external_consumer_when_declared_then_provider_and_typed_endpoint_use_public_api() {
        let session = Session::new();
        session
            .register_operator(Arc::new(WhisperOperatorFactory::new(
                "/opt/whisper/whisper-cli",
                "/opt/whisper/model.bin",
                "en",
            )))
            .expect("operator registration");

        let endpoint = session
            .endpoint(EndpointDescriptor::new(
                NodeTypeId::from(TEXT_ENDPOINT_NODE_TYPE_ID),
                OperatorId::new(TEXT_ENDPOINT_OPERATOR_ID),
            ))
            .expect("endpoint declaration");
        session
            .register_endpoint(
                OperatorId::new(TEXT_ENDPOINT_OPERATOR_ID),
                Arc::new(TextEndpointDefinition),
                Arc::new(TextEndpointFactory),
            )
            .expect("typed endpoint extension registration");

        let microphone = session
            .capture(Source::microphone_default())
            .expect("microphone declaration");
        let transcript = microphone
            .through(Operator::new(
                OperatorId::new(WHISPER_OPERATOR_ID),
                OperatorConfiguration::new().with("language", "en"),
            ))
            .expect("operator declaration");
        transcript.send(endpoint).expect("typed endpoint route");
    }
}

//! Compile boundary for an external provider and typed endpoint consumer.
//!
//! This package intentionally depends only on the public `pocketstation`
//! façade and an example provider package.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use pocketstation::endpoint::{
        DerivedEndpointDriverInput, EndpointCancellationOutcome, EndpointDriverFactory,
        EndpointDriverFinalization, EndpointDriverInput, EndpointDriverObservations,
        EndpointFailure, EndpointFailureStage, EndpointStartGate, ExecutionPartition, MediaCaps,
        Multiplicity, NodeDefinition, NodeDescriptor, OperatorConfiguration, PortDirection,
        PortSpec, PreparedEndpointDriver, RunningEndpointDriver, SafetyContract, SignalSpec,
        TextFormat,
    };
    use pocketstation::{EndpointDescriptor, NodeTypeId, Operator, OperatorId, Session, Source};
    use whisper_transcribe_example::{WhisperOperatorFactory, WHISPER_OPERATOR_ID};

    const TEXT_ENDPOINT_OPERATOR_ID: &str = "example.endpoint.transcript.v1";
    const TEXT_ENDPOINT_NODE_TYPE_ID: &str = "endpoint.transcript.example";

    struct TextEndpointDefinition;

    impl NodeDefinition for TextEndpointDefinition {
        fn descriptor(&self) -> NodeDescriptor {
            NodeDescriptor {
                type_id: NodeTypeId::from(TEXT_ENDPOINT_NODE_TYPE_ID),
                display_name: "Example transcript endpoint",
                inputs: vec![PortSpec {
                    name: "transcript".to_owned(),
                    direction: PortDirection::Input,
                    signal: SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                    media: MediaCaps::Text,
                    multiplicity: Multiplicity::One,
                    required: true,
                }],
                outputs: Vec::new(),
                execution: ExecutionPartition::External,
                safety: SafetyContract::ExternalService,
                stateful: true,
            }
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
            _inputs: Vec<EndpointDriverInput>,
        ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
            Err(EndpointFailure::new(
                EndpointFailureStage::Prepare,
                "text endpoint accepts only typed derived signals",
            ))
        }

        fn prepare_derived(
            &self,
            inputs: Vec<DerivedEndpointDriverInput>,
        ) -> Result<Box<dyn PreparedEndpointDriver>, EndpointFailure> {
            Ok(Box::new(PreparedTextEndpoint { inputs }))
        }
    }

    struct PreparedTextEndpoint {
        inputs: Vec<DerivedEndpointDriverInput>,
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
        inputs: Vec<DerivedEndpointDriverInput>,
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
            .register_endpoint_definition(Arc::new(TextEndpointDefinition))
            .expect("typed endpoint definition registration");
        session
            .register_endpoint_driver(
                OperatorId::new(TEXT_ENDPOINT_OPERATOR_ID),
                NodeTypeId::from(TEXT_ENDPOINT_NODE_TYPE_ID),
                Arc::new(TextEndpointFactory),
            )
            .expect("typed endpoint registration");

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

use pocketstation::{
    Session, SessionSourceError, SourceConfiguration, SourceFactory, SourceInstanceHandle,
    SourceInstanceId, SourceOutputHandle, SourceTypeId,
};
use std::sync::Arc;

fn register_source_signature(
    session: &Session,
    factory: Arc<dyn SourceFactory>,
) -> Result<(), SessionSourceError> {
    session.register_source(factory)
}

mod session {
    mod external_source {
        use super::super::*;

        #[test]
        fn given_public_session_when_external_source_declared_then_handles_are_nameable() {
            let _registration_api: fn(
                &Session,
                Arc<dyn SourceFactory>,
            ) -> Result<(), SessionSourceError> = register_source_signature;
            let session = Session::new();
            let source: SourceInstanceHandle = session
                .source(
                    SourceTypeId::new("org.example.source.source-a.v1").unwrap(),
                    SourceConfiguration::default(),
                )
                .unwrap();
            let output: SourceOutputHandle = source.output("signal").unwrap();

            assert_eq!(source.instance_id(), SourceInstanceId::new(1));
            assert_eq!(source.source_id(), output.source_id());
            assert_eq!(source.instance_id(), output.source_instance_id());
            assert_eq!(output.output_port(), "signal");
            assert_ne!(output.stream_id().get(), 0);
        }
    }
}

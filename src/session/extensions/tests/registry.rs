use std::sync::Arc;

use crate::graph::{
    ConfigError, ExecutionPartition, ExecutionSafety, MediaCaps, Multiplicity, PortDirection,
    PortSpec, SignalSpec,
};
use crate::session::{
    SampleFormat, SampleSpec, SessionEngineBuilder, SessionStartOptions, SourceCancellation,
    SourceConfiguration, SourceDriver, SourceDriverError, SourceEmission, SourceFactory,
    SourceManifest, SourceManifestError, SourceRegistrationError, SourceTypeId,
};

use crate::graph::PrepareContext;

struct RegistrationOnlyDriver;

impl SourceDriver for RegistrationOnlyDriver {
    fn prepare(
        &mut self,
        _context: &crate::session::SourcePrepareContext,
    ) -> Result<(), SourceDriverError> {
        Ok(())
    }

    fn next(
        &mut self,
        _cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        Ok(None)
    }

    fn close(&mut self) -> Result<(), SourceDriverError> {
        Ok(())
    }
}

struct RegistrationOnlyFactory {
    manifest: SourceManifest,
}

impl SourceFactory for RegistrationOnlyFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }

    fn validate_config(&self, _configuration: &SourceConfiguration) -> Result<(), ConfigError> {
        Ok(())
    }

    fn create(
        &self,
        _configuration: &SourceConfiguration,
    ) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        Ok(Box::new(RegistrationOnlyDriver))
    }
}

fn source_type_id() -> SourceTypeId {
    SourceTypeId::new("dev.pocketstation.source.external-source.v1").unwrap()
}

fn factory(revision: u32) -> Arc<dyn SourceFactory> {
    Arc::new(RegistrationOnlyFactory {
        manifest: SourceManifest {
            source_type_id: source_type_id(),
            revision,
            implementation_generation: 1,
            outputs: vec![PortSpec {
                name: "samples".to_owned(),
                direction: PortDirection::Output,
                signal: SignalSpec::custom("dev.pocketstation.fixture.samples.v1")
                    .with_schema("urn:pocketstation:fixture:samples:v1"),
                media: MediaCaps::Binary(crate::graph::BinaryFormat::Raw),
                multiplicity: Multiplicity::Many,
                required: true,
            }],
            execution: ExecutionPartition::BlockingWorker,
            safety: ExecutionSafety::AllocationAllowed,
        },
    })
}

fn builder() -> SessionEngineBuilder {
    SessionEngineBuilder::new(
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved)),
        8,
        SessionStartOptions::default(),
    )
    .unwrap()
}

#[test]
fn given_source_factory_when_registered_then_built_engine_retains_manifest() {
    let mut builder = builder();
    builder.register_source_factory(factory(7)).unwrap();

    let registered = builder.source_manifest(&source_type_id()).unwrap();
    assert_eq!(registered.revision, 7);
    assert_eq!(registered.source_type_id, source_type_id());

    let engine = builder.build().unwrap();
    let retained = engine.source_manifest(&source_type_id()).unwrap();
    assert_eq!(retained.revision, 7);
    assert_eq!(retained.outputs[0].name, "samples");
}

#[test]
fn given_registered_identity_when_revision_conflicts_then_first_factory_is_preserved() {
    let mut builder = builder();
    builder.register_source_factory(factory(7)).unwrap();

    let error = builder.register_source_factory(factory(8)).err().unwrap();
    assert!(matches!(
        error,
        SourceRegistrationError::DuplicateSourceType(id) if id == source_type_id()
    ));
    assert_eq!(
        builder.source_manifest(&source_type_id()).unwrap().revision,
        7
    );
}

#[test]
fn given_zero_revision_when_registered_then_public_boundary_rejects_manifest() {
    let mut builder = builder();

    let error = builder.register_source_factory(factory(0)).err().unwrap();
    assert!(matches!(
        error,
        SourceRegistrationError::InvalidManifest(SourceManifestError::ZeroVersion)
    ));
    assert!(builder.source_manifest(&source_type_id()).is_none());
}

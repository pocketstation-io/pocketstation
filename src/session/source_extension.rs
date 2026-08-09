use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::graph::{
    ConfigError, ExecutionPartition, NodeConfig, NodeDefinition, NodeDescriptor, NodeTypeId,
    PortDirection, PortSpec, SafetyContract, SignalContinuityTracker, SignalEnvelope,
};
use crate::runtime::{
    TypedEdgeBranchSpec, TypedEdgeBuildError, TypedEdgeFanout, TypedEdgePublishError,
    TypedEdgeReceiver,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceTypeId(String);

impl SourceTypeId {
    pub fn new(value: impl Into<String>) -> Result<Self, SourceManifestError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SourceManifestError::EmptySourceTypeId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceConfiguration {
    values: BTreeMap<String, String>,
}

impl SourceConfiguration {
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

#[derive(Debug, Clone)]
pub struct SourceManifest {
    pub source_type_id: SourceTypeId,
    pub revision: u32,
    pub generation: u32,
    pub outputs: Vec<PortSpec>,
    pub execution: ExecutionPartition,
    pub safety: SafetyContract,
}

impl SourceManifest {
    pub fn validate(&self) -> Result<(), SourceManifestError> {
        if self.revision == 0 || self.generation == 0 {
            return Err(SourceManifestError::ZeroVersion);
        }
        if !self.safety.is_valid_for(self.execution) {
            return Err(SourceManifestError::InvalidSafetyContract);
        }
        if self.execution != ExecutionPartition::BlockingWorker {
            return Err(SourceManifestError::UnsupportedExecutionPartition);
        }
        if self.outputs.is_empty() {
            return Err(SourceManifestError::NoOutputs);
        }
        let mut names = BTreeSet::new();
        for output in &self.outputs {
            if output.direction != PortDirection::Output {
                return Err(SourceManifestError::NonOutputPort);
            }
            if output.name.trim().is_empty() {
                return Err(SourceManifestError::EmptyOutputName);
            }
            if !names.insert(output.name.as_str()) {
                return Err(SourceManifestError::DuplicateOutputName);
            }
            output
                .signal
                .validate()
                .map_err(|_| SourceManifestError::InvalidSignal)?;
            if !output.media.supports_signal(&output.signal) {
                return Err(SourceManifestError::SignalMediaMismatch);
            }
        }
        Ok(())
    }

    pub fn output_port(&self, name: &str) -> Option<&PortSpec> {
        self.outputs.iter().find(|output| output.name == name)
    }
}

#[derive(Debug, Clone)]
pub struct SourcePrepareContext {
    pub manifest: SourceManifest,
}

#[derive(Clone)]
pub struct SourceCancellation {
    cancelled: Arc<AtomicBool>,
}

impl SourceCancellation {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

#[derive(Debug)]
pub struct SourceEmission {
    pub output_port: String,
    pub envelope: SignalEnvelope,
    pub terminal: bool,
}

pub trait SourceDriver: Send {
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError>;
    fn next(
        &mut self,
        cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError>;
    fn close(&mut self) -> Result<(), SourceDriverError>;
}

pub trait SourceFactory: Send + Sync {
    fn manifest(&self) -> &SourceManifest;
    fn validate_config(&self, configuration: &SourceConfiguration) -> Result<(), ConfigError>;
    fn create(
        &self,
        configuration: &SourceConfiguration,
    ) -> Result<Box<dyn SourceDriver>, SourceDriverError>;
}

#[derive(Default)]
pub struct SourceRegistry {
    factories: BTreeMap<SourceTypeId, Arc<dyn SourceFactory>>,
}

impl SourceRegistry {
    pub fn manifest(&self, source_type_id: &SourceTypeId) -> Option<&SourceManifest> {
        self.factories
            .get(source_type_id)
            .map(|factory| factory.manifest())
    }

    pub fn register(
        &mut self,
        factory: Arc<dyn SourceFactory>,
    ) -> Result<(), SourceRegistrationError> {
        factory
            .manifest()
            .validate()
            .map_err(SourceRegistrationError::InvalidManifest)?;
        let source_type_id = factory.manifest().source_type_id.clone();
        if self.factories.contains_key(&source_type_id) {
            return Err(SourceRegistrationError::DuplicateSourceType(source_type_id));
        }
        self.factories.insert(source_type_id, factory);
        Ok(())
    }

    pub fn validate_config(
        &self,
        source_type_id: &SourceTypeId,
        configuration: &SourceConfiguration,
    ) -> Result<(), SourceRuntimeError> {
        let factory = self
            .factories
            .get(source_type_id)
            .ok_or_else(|| SourceRuntimeError::UnregisteredSource(source_type_id.clone()))?;
        factory
            .validate_config(configuration)
            .map_err(SourceRuntimeError::InvalidConfiguration)
    }

    pub fn spawn(
        &self,
        source_type_id: &SourceTypeId,
        configuration: &SourceConfiguration,
        branch_specs: &[SourceOutputBranchSpec],
    ) -> Result<(SourceRuntime, Vec<SourceOutputReceiver>), SourceRuntimeError> {
        let factory = self
            .factories
            .get(source_type_id)
            .cloned()
            .ok_or_else(|| SourceRuntimeError::UnregisteredSource(source_type_id.clone()))?;
        SourceRuntime::spawn(factory, configuration, branch_specs)
    }
}

#[derive(Debug, Clone)]
pub struct SourceOutputBranchSpec {
    pub output_port: String,
    pub branch: TypedEdgeBranchSpec,
}

pub struct SourceOutputReceiver {
    pub output_port: String,
    pub receiver: TypedEdgeReceiver,
}

#[derive(Default)]
struct SourceRuntimeObservationState {
    emitted_total: AtomicU64,
    dropped_total: AtomicU64,
    failure_total: AtomicU64,
    cancellation_total: AtomicU64,
    ready: AtomicBool,
    joined: AtomicBool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRuntimeObservations {
    pub emitted_total: u64,
    pub dropped_total: u64,
    pub failure_total: u64,
    pub cancellation_total: u64,
    pub ready: bool,
    pub joined: bool,
}

#[derive(Clone)]
pub struct SourceRuntimeObservationHandle {
    state: Arc<SourceRuntimeObservationState>,
}

impl SourceRuntimeObservationHandle {
    pub fn snapshot(&self) -> SourceRuntimeObservations {
        SourceRuntimeObservations {
            emitted_total: self.state.emitted_total.load(Ordering::Relaxed),
            dropped_total: self.state.dropped_total.load(Ordering::Relaxed),
            failure_total: self.state.failure_total.load(Ordering::Relaxed),
            cancellation_total: self.state.cancellation_total.load(Ordering::Relaxed),
            ready: self.state.ready.load(Ordering::Acquire),
            joined: self.state.joined.load(Ordering::Acquire),
        }
    }
}

pub struct SourceRuntime {
    cancellation: SourceCancellation,
    observations: SourceRuntimeObservationHandle,
    join: Option<JoinHandle<Result<(), SourceRuntimeError>>>,
}

impl SourceRuntime {
    pub fn spawn(
        factory: Arc<dyn SourceFactory>,
        configuration: &SourceConfiguration,
        branch_specs: &[SourceOutputBranchSpec],
    ) -> Result<(Self, Vec<SourceOutputReceiver>), SourceRuntimeError> {
        factory
            .manifest()
            .validate()
            .map_err(SourceRuntimeError::InvalidManifest)?;
        factory
            .validate_config(configuration)
            .map_err(SourceRuntimeError::InvalidConfiguration)?;
        let mut driver = factory
            .create(configuration)
            .map_err(SourceRuntimeError::Driver)?;
        let manifest = factory.manifest().clone();
        let mut fanouts = BTreeMap::new();
        let mut receivers = Vec::new();
        for output in &manifest.outputs {
            let specifications: Vec<_> = branch_specs
                .iter()
                .filter(|branch| branch.output_port == output.name)
                .map(|branch| branch.branch)
                .collect();
            if specifications.is_empty() {
                continue;
            }
            let (fanout, output_receivers) =
                TypedEdgeFanout::new(&specifications).map_err(SourceRuntimeError::EdgeBuild)?;
            fanouts.insert(output.name.clone(), fanout);
            receivers.extend(
                output_receivers
                    .into_iter()
                    .map(|receiver| SourceOutputReceiver {
                        output_port: output.name.clone(),
                        receiver,
                    }),
            );
        }
        if fanouts.is_empty() {
            return Err(SourceRuntimeError::NoRoutedOutputs);
        }
        driver
            .prepare(&SourcePrepareContext {
                manifest: manifest.clone(),
            })
            .map_err(SourceRuntimeError::Driver)?;
        let cancellation = SourceCancellation {
            cancelled: Arc::new(AtomicBool::new(false)),
        };
        let task_cancellation = cancellation.clone();
        let state = Arc::new(SourceRuntimeObservationState::default());
        state.ready.store(true, Ordering::Release);
        let task_state = Arc::clone(&state);
        let join = std::thread::Builder::new()
            .name("pks-typed-source".to_owned())
            .spawn(move || {
                let result = run_source_driver(
                    driver.as_mut(),
                    &manifest,
                    &mut fanouts,
                    &task_cancellation,
                    &task_state,
                );
                if result.is_err() {
                    task_state.failure_total.fetch_add(1, Ordering::Relaxed);
                }
                if task_cancellation.is_cancelled() {
                    task_state
                        .cancellation_total
                        .fetch_add(1, Ordering::Relaxed);
                }
                let close_result = driver.close().map_err(SourceRuntimeError::Driver);
                task_state.joined.store(true, Ordering::Release);
                result.and(close_result)
            })
            .map_err(SourceRuntimeError::Spawn)?;
        Ok((
            Self {
                cancellation,
                observations: SourceRuntimeObservationHandle { state },
                join: Some(join),
            },
            receivers,
        ))
    }

    pub fn cancel(&self) {
        self.cancellation.cancelled.store(true, Ordering::Release);
    }

    pub fn observations(&self) -> SourceRuntimeObservationHandle {
        self.observations.clone()
    }

    pub fn join(&mut self) -> Result<(), SourceRuntimeError> {
        self.join
            .take()
            .ok_or(SourceRuntimeError::AlreadyJoined)?
            .join()
            .map_err(|_| SourceRuntimeError::WorkerPanicked)??;
        Ok(())
    }
}

impl Drop for SourceRuntime {
    fn drop(&mut self) {
        self.cancel();
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

fn run_source_driver(
    driver: &mut dyn SourceDriver,
    manifest: &SourceManifest,
    fanouts: &mut BTreeMap<String, TypedEdgeFanout>,
    cancellation: &SourceCancellation,
    observations: &SourceRuntimeObservationState,
) -> Result<(), SourceRuntimeError> {
    let mut continuity = BTreeMap::<String, SignalContinuityTracker>::new();
    while !cancellation.is_cancelled() {
        let Some(emission) = driver
            .next(cancellation)
            .map_err(SourceRuntimeError::Driver)?
        else {
            break;
        };
        let output = manifest
            .output_port(&emission.output_port)
            .ok_or_else(|| SourceRuntimeError::UnknownOutput(emission.output_port.clone()))?;
        if emission.envelope.spec.class != output.signal.class
            || emission.envelope.spec.schema != output.signal.schema
            || !output.media.supports_signal(&emission.envelope.spec)
        {
            return Err(SourceRuntimeError::OutputContractMismatch);
        }
        continuity
            .entry(emission.output_port.clone())
            .or_default()
            .observe(&emission.envelope)
            .map_err(SourceRuntimeError::Continuity)?;
        let fanout = fanouts
            .get_mut(&emission.output_port)
            .ok_or_else(|| SourceRuntimeError::UnroutedOutput(emission.output_port.clone()))?;
        let report = fanout
            .publish(emission.envelope, emission.terminal)
            .map_err(SourceRuntimeError::Publish)?;
        observations
            .emitted_total
            .fetch_add(report.delivered_total, Ordering::Relaxed);
        observations
            .dropped_total
            .fetch_add(report.dropped_total, Ordering::Relaxed);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SourceManifestError {
    #[error("source type identifier cannot be empty")]
    EmptySourceTypeId,
    #[error("source revision and generation must be non-zero")]
    ZeroVersion,
    #[error("source manifest requires at least one output")]
    NoOutputs,
    #[error("source manifest contains a non-output port")]
    NonOutputPort,
    #[error("source output name cannot be empty")]
    EmptyOutputName,
    #[error("source output names must be unique")]
    DuplicateOutputName,
    #[error("source output SignalSpec is invalid")]
    InvalidSignal,
    #[error("source output SignalSpec and MediaCaps are incompatible")]
    SignalMediaMismatch,
    #[error("source safety contract is incompatible with its execution partition")]
    InvalidSafetyContract,
    #[error("in-process source drivers currently require the BlockingWorker partition")]
    UnsupportedExecutionPartition,
}

#[derive(Debug, thiserror::Error)]
pub enum SourceRegistrationError {
    #[error("invalid source manifest: {0}")]
    InvalidManifest(SourceManifestError),
    #[error("source type {0} is already registered")]
    DuplicateSourceType(SourceTypeId),
    #[error("source type {0} conflicts with an existing graph node type")]
    NodeTypeConflict(SourceTypeId),
}

pub(crate) const SOURCE_CONFIGURATION_PREFIX: &str = "source.config.";

pub(crate) fn source_node_definition(factory: Arc<dyn SourceFactory>) -> Arc<dyn NodeDefinition> {
    Arc::new(SourceNodeDefinition { factory })
}

struct SourceNodeDefinition {
    factory: Arc<dyn SourceFactory>,
}

impl NodeDefinition for SourceNodeDefinition {
    fn descriptor(&self) -> NodeDescriptor {
        let manifest = self.factory.manifest();
        NodeDescriptor {
            type_id: NodeTypeId::from(manifest.source_type_id.as_str()),
            display_name: "External source",
            inputs: Vec::new(),
            outputs: manifest.outputs.clone(),
            execution: manifest.execution,
            safety: manifest.safety,
            stateful: true,
        }
    }

    fn validate_config(&self, config: &NodeConfig) -> Result<(), ConfigError> {
        let mut source_configuration = SourceConfiguration::default();
        for (key, value) in config.iter() {
            if let Some(key) = key.strip_prefix(SOURCE_CONFIGURATION_PREFIX) {
                source_configuration.insert(key, value);
            }
        }
        self.factory.validate_config(&source_configuration)
    }
}

impl std::fmt::Display for SourceTypeId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SourceDriverError {
    #[error("source driver failed: {0}")]
    Failed(String),
}

#[derive(Debug, thiserror::Error)]
pub enum SourceRuntimeError {
    #[error("invalid source manifest: {0}")]
    InvalidManifest(SourceManifestError),
    #[error("invalid source configuration: {0}")]
    InvalidConfiguration(ConfigError),
    #[error("source driver failure: {0}")]
    Driver(SourceDriverError),
    #[error("typed edge build failed: {0}")]
    EdgeBuild(TypedEdgeBuildError),
    #[error("source runtime requires at least one routed output")]
    NoRoutedOutputs,
    #[error("source emitted unknown output {0}")]
    UnknownOutput(String),
    #[error("source emitted unrouted output {0}")]
    UnroutedOutput(String),
    #[error("source output does not match its manifest contract")]
    OutputContractMismatch,
    #[error("source continuity validation failed: {0}")]
    Continuity(crate::graph::SignalContinuityError),
    #[error("typed source publish failed: {0}")]
    Publish(TypedEdgePublishError),
    #[error("source worker could not spawn: {0}")]
    Spawn(std::io::Error),
    #[error("source worker panicked")]
    WorkerPanicked,
    #[error("source worker has already been joined")]
    AlreadyJoined,
    #[error("source type {0} is not registered")]
    UnregisteredSource(SourceTypeId),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{BinaryFormat, MediaCaps, Multiplicity, SignalSpec};

    fn output(name: &str) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction: PortDirection::Output,
            signal: SignalSpec::custom("dev.pocketstation.test.v1").with_schema("urn:test:v1"),
            media: MediaCaps::Binary(BinaryFormat::Raw),
            multiplicity: Multiplicity::Many,
            required: true,
        }
    }

    fn manifest(outputs: Vec<PortSpec>) -> SourceManifest {
        SourceManifest {
            source_type_id: SourceTypeId::new("dev.pocketstation.source.test.v1").unwrap(),
            revision: 1,
            generation: 1,
            outputs,
            execution: ExecutionPartition::BlockingWorker,
            safety: SafetyContract::AllocationAllowed,
        }
    }

    #[test]
    fn given_schema_backed_output_when_manifest_validated_then_contract_is_open() {
        assert_eq!(manifest(vec![output("out")]).validate(), Ok(()));
    }

    #[test]
    fn given_duplicate_output_names_when_manifest_validated_then_rejected() {
        assert_eq!(
            manifest(vec![output("out"), output("out")]).validate(),
            Err(SourceManifestError::DuplicateOutputName)
        );
    }
}

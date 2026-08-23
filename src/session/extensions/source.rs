use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::frame::{SessionId, SourceId, StreamId};
use crate::graph::{
    ConfigError, ExecutionPartition, NodeConfig, NodeDefinition, NodeDescriptor, NodeTypeId,
    PortDirection, PortSpec, SafetyContract, SignalContinuityTracker, SignalEnvelope,
};
use crate::runtime::{
    TypedEdgeBranchSpec, TypedEdgeBuildError, TypedEdgeFanout, TypedEdgePublishError,
    TypedEdgeReceiver,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "Uniquely identifies source type."]
pub struct SourceTypeId(String);

impl SourceTypeId {
    /// Creates a stable source implementation identity.
    ///
    /// Published source packages should use a reverse-domain identifier with
    /// an explicit contract revision, for example
    /// `io.example.source.device.v1`. Session instance identity belongs in
    /// `SourceInstanceId` and `SourceId`, never in this value.
    pub fn new(value: impl Into<String>) -> Result<Self, SourceTypeIdError> {
        let value = value.into();
        if value.is_empty() {
            return Err(SourceTypeIdError::Empty);
        }
        if value.trim() != value {
            return Err(SourceTypeIdError::SurroundingWhitespace);
        }
        if value.len() > crate::graph::identifier::MAX_IDENTIFIER_BYTES {
            return Err(SourceTypeIdError::TooLong {
                actual_bytes: value.len(),
                maximum_bytes: crate::graph::identifier::MAX_IDENTIFIER_BYTES,
            });
        }
        if !value.is_ascii() {
            return Err(SourceTypeIdError::NonAscii);
        }
        if !crate::graph::identifier::is_portable_contract_id(&value) {
            return Err(SourceTypeIdError::InvalidContractSyntax);
        }
        if !has_source_category(&value) {
            return Err(SourceTypeIdError::MissingSourceCategory);
        }
        Ok(Self(value))
    }

    #[doc = "Returns the stable string representation of `SourceTypeId`."]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn has_source_category(value: &str) -> bool {
    let Some((contract, _revision)) = value.rsplit_once('.') else {
        return false;
    };
    let Some((owner, source_name)) = contract.rsplit_once(".source.") else {
        return false;
    };
    owner.split('.').count() >= 2 && !source_name.is_empty()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as source type id error."]
pub enum SourceTypeIdError {
    #[error("source type identifier cannot be empty")]
    #[doc = "Represents an empty value or collection."]
    Empty,
    #[error("source type identifier cannot contain surrounding whitespace")]
    #[doc = "Reports surrounding whitespace."]
    SurroundingWhitespace,
    #[error("source type identifier is {actual_bytes} bytes; maximum is {maximum_bytes}")]
    #[doc = "Reports too long."]
    TooLong {
        #[doc = "Stores the actual size for `TooLong`, in bytes."]
        actual_bytes: usize,
        #[doc = "Stores the maximum size for `TooLong`, in bytes."]
        maximum_bytes: usize,
    },
    #[error("source type identifier must contain only ASCII contract characters")]
    #[doc = "Reports non ascii."]
    NonAscii,
    #[error("source type identifier must use bounded reverse-domain syntax ending in vN")]
    #[doc = "Reports invalid contract syntax."]
    InvalidContractSyntax,
    #[error("source type identifier must contain a source category and concrete source name")]
    #[doc = "Reports missing source category."]
    MissingSourceCategory,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[doc = "Configures source."]
pub struct SourceConfiguration {
    values: BTreeMap<String, String>,
}

impl SourceConfiguration {
    /// Adds declared source configuration.
    ///
    /// This map is for provider/user configuration only. Runtime handles,
    /// reservations, and Session instance identities must remain internal.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    #[doc = "Returns the value held by `SourceConfiguration`."]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    #[doc = "Iterates over the values held by `SourceConfiguration`."]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

#[derive(Debug, Clone)]
#[doc = "Describes the source manifest contract."]
pub struct SourceManifest {
    pub(crate) source_type_id: SourceTypeId,
    pub(crate) revision: u32,
    pub(crate) implementation_generation: u32,
    pub(crate) outputs: Vec<PortSpec>,
    pub(crate) execution: ExecutionPartition,
    pub(crate) safety: SafetyContract,
}

impl SourceManifest {
    #[doc = "Creates a new `SourceManifest`."]
    pub fn new(
        source_type_id: SourceTypeId,
        revision: u32,
        implementation_generation: u32,
        outputs: Vec<PortSpec>,
        execution: ExecutionPartition,
        safety: SafetyContract,
    ) -> Result<Self, SourceManifestError> {
        let manifest = Self {
            source_type_id,
            revision,
            implementation_generation,
            outputs,
            execution,
            safety,
        };
        manifest.validate()?;
        Ok(manifest)
    }

    #[doc = "Returns the source type identifier associated with `SourceManifest`."]
    pub const fn source_type_id(&self) -> &SourceTypeId {
        &self.source_type_id
    }

    /// Additive descriptor revision within the compatibility major encoded by
    /// the [`SourceTypeId`] suffix. A breaking source contract uses a new
    /// identifier ending in the next `vN`; it does not reuse this field.
    pub const fn revision(&self) -> u32 {
        self.revision
    }

    /// Monotonic implementation generation for this manifest revision.
    ///
    /// This is registration metadata. It is unrelated to the runtime
    /// `source_generation` carried by frame lineage when a concrete source
    /// disappears and reappears.
    pub const fn implementation_generation(&self) -> u32 {
        self.implementation_generation
    }

    /// Returns the implementation generation.
    ///
    /// Runtime source-attachment generations remain part of frame lineage and
    /// are not represented by this manifest field.
    pub const fn generation(&self) -> u32 {
        self.implementation_generation
    }

    #[doc = "Returns the outputs associated with `SourceManifest`."]
    pub fn outputs(&self) -> &[PortSpec] {
        &self.outputs
    }

    #[doc = "Returns the execution associated with `SourceManifest`."]
    pub const fn execution(&self) -> ExecutionPartition {
        self.execution
    }

    #[doc = "Returns the safety associated with `SourceManifest`."]
    pub const fn safety(&self) -> SafetyContract {
        self.safety
    }

    #[doc = "Validates `SourceManifest` against its declared contract."]
    pub fn validate(&self) -> Result<(), SourceManifestError> {
        if self.revision == 0 || self.implementation_generation == 0 {
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

    #[doc = "Returns the output port associated with `SourceManifest`."]
    pub fn output_port(&self, name: &str) -> Option<&PortSpec> {
        self.outputs.iter().find(|output| output.name == name)
    }
}

#[derive(Debug, Clone)]
#[doc = "Represents source prepare context in the PocketStation API."]
pub struct SourcePrepareContext {
    #[doc = "Stores the manifest associated with `SourcePrepareContext`."]
    pub manifest: SourceManifest,
    #[doc = "Stores the session associated with `SourcePrepareContext`."]
    pub session: Option<SourceSessionContext>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Represents source output identity in the PocketStation API."]
pub struct SourceOutputIdentity {
    #[doc = "Stores the output port associated with `SourceOutputIdentity`."]
    pub output_port: String,
    #[doc = "Identifies the stream associated with `SourceOutputIdentity`."]
    pub stream_id: StreamId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Represents source session context in the PocketStation API."]
pub struct SourceSessionContext {
    #[doc = "Identifies the session associated with `SourceSessionContext`."]
    pub session_id: SessionId,
    #[doc = "Identifies the source associated with `SourceSessionContext`."]
    pub source_id: SourceId,
    #[doc = "Stores the outputs associated with `SourceSessionContext`."]
    pub outputs: Vec<SourceOutputIdentity>,
}

impl SourceSessionContext {
    #[doc = "Returns the output associated with `SourceSessionContext`."]
    pub fn output(&self, output_port: &str) -> Option<&SourceOutputIdentity> {
        self.outputs
            .iter()
            .find(|output| output.output_port == output_port)
    }
}

#[derive(Clone)]
#[doc = "Represents source cancellation in the PocketStation API."]
pub struct SourceCancellation {
    cancelled: Arc<AtomicBool>,
}

impl SourceCancellation {
    #[doc = "Returns whether cancelled applies to `SourceCancellation`."]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

#[derive(Debug)]
#[doc = "Represents source emission in the PocketStation API."]
pub struct SourceEmission {
    #[doc = "Stores the output port associated with `SourceEmission`."]
    pub output_port: String,
    #[doc = "Stores the envelope associated with `SourceEmission`."]
    pub envelope: SignalEnvelope,
    #[doc = "Indicates whether terminal applies to `SourceEmission`."]
    pub terminal: bool,
}

#[doc = "Defines the implementation contract for source."]
pub trait SourceDriver: Send {
    #[doc = "Prepares resources required by `SourceDriver`."]
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError>;
    #[doc = "Produces the next source emission from `SourceDriver`."]
    fn next(
        &mut self,
        cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError>;
    #[doc = "Closes `SourceDriver` to further work."]
    fn close(&mut self) -> Result<(), SourceDriverError>;
}

#[doc = "Defines the implementation contract for source."]
pub trait SourceFactory: Send + Sync {
    #[doc = "Returns the manifest associated with `SourceFactory`."]
    fn manifest(&self) -> &SourceManifest;
    #[doc = "Validates config for `SourceFactory`."]
    fn validate_config(&self, configuration: &SourceConfiguration) -> Result<(), ConfigError>;
    #[doc = "Creates the runtime implementation described by `SourceFactory`."]
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

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn spawn(
        &self,
        source_type_id: &SourceTypeId,
        configuration: &SourceConfiguration,
        branch_specs: &[SourceOutputBranchSpec],
    ) -> Result<(SourceRuntime, Vec<SourceOutputReceiver>), SourceRuntimeError> {
        let (prepared, receivers) = self.prepare(source_type_id, configuration, branch_specs)?;
        Ok((prepared.start()?, receivers))
    }

    #[cfg(any(test, feature = "internal-testing"))]
    pub fn prepare(
        &self,
        source_type_id: &SourceTypeId,
        configuration: &SourceConfiguration,
        branch_specs: &[SourceOutputBranchSpec],
    ) -> Result<(PreparedSourceRuntime, Vec<SourceOutputReceiver>), SourceRuntimeError> {
        let factory = self
            .factories
            .get(source_type_id)
            .cloned()
            .ok_or_else(|| SourceRuntimeError::UnregisteredSource(source_type_id.clone()))?;
        PreparedSourceRuntime::prepare(factory, configuration, branch_specs, None)
    }

    pub fn prepare_session(
        &self,
        source_type_id: &SourceTypeId,
        configuration: &SourceConfiguration,
        branch_specs: &[SourceOutputBranchSpec],
        session: SourceSessionContext,
    ) -> Result<(PreparedSourceRuntime, Vec<SourceOutputReceiver>), SourceRuntimeError> {
        let factory = self
            .factories
            .get(source_type_id)
            .cloned()
            .ok_or_else(|| SourceRuntimeError::UnregisteredSource(source_type_id.clone()))?;
        PreparedSourceRuntime::prepare(factory, configuration, branch_specs, Some(session))
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
    discontinuity_total: AtomicU64,
    recovery_total: AtomicU64,
    policy_change_total: AtomicU64,
    ready: AtomicBool,
    joined: AtomicBool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Reports the source runtime observations collected at an observation boundary."]
pub struct SourceRuntimeObservations {
    #[doc = "Counts the total number of emitted observed by `SourceRuntimeObservations`."]
    pub emitted_total: u64,
    #[doc = "Counts the total number of dropped observed by `SourceRuntimeObservations`."]
    pub dropped_total: u64,
    #[doc = "Counts the total number of failure observed by `SourceRuntimeObservations`."]
    pub failure_total: u64,
    #[doc = "Counts the total number of cancellation observed by `SourceRuntimeObservations`."]
    pub cancellation_total: u64,
    #[doc = "Counts the total number of discontinuity observed by `SourceRuntimeObservations`."]
    pub discontinuity_total: u64,
    #[doc = "Counts the total number of recovery observed by `SourceRuntimeObservations`."]
    pub recovery_total: u64,
    #[doc = "Counts the total number of policy change observed by `SourceRuntimeObservations`."]
    pub policy_change_total: u64,
    #[doc = "Indicates whether ready applies to `SourceRuntimeObservations`."]
    pub ready: bool,
    #[doc = "Stores the joined associated with `SourceRuntimeObservations`."]
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
            discontinuity_total: self.state.discontinuity_total.load(Ordering::Relaxed),
            recovery_total: self.state.recovery_total.load(Ordering::Relaxed),
            policy_change_total: self.state.policy_change_total.load(Ordering::Relaxed),
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

/// Fully validated source resources which have not started producing signals.
///
/// Keeping this state distinct is what lets Session prepare every bounded
/// branch and endpoint transactionally before the first source callback runs.
pub struct PreparedSourceRuntime {
    driver: Option<Box<dyn SourceDriver>>,
    manifest: SourceManifest,
    fanouts: Option<BTreeMap<String, TypedEdgeFanout>>,
    session: Option<SourceSessionContext>,
}

impl PreparedSourceRuntime {
    fn prepare(
        factory: Arc<dyn SourceFactory>,
        configuration: &SourceConfiguration,
        branch_specs: &[SourceOutputBranchSpec],
        session: Option<SourceSessionContext>,
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
                session: session.clone(),
            })
            .map_err(SourceRuntimeError::Driver)?;
        Ok((
            Self {
                driver: Some(driver),
                manifest,
                fanouts: Some(fanouts),
                session,
            },
            receivers,
        ))
    }

    pub fn start(mut self) -> Result<SourceRuntime, SourceRuntimeError> {
        let mut driver = self
            .driver
            .take()
            .ok_or(SourceRuntimeError::PreparedStateConsumed)?;
        let manifest = self.manifest.clone();
        let mut fanouts = self
            .fanouts
            .take()
            .ok_or(SourceRuntimeError::PreparedStateConsumed)?;
        let session = self.session.clone();
        let cancellation = SourceCancellation {
            cancelled: Arc::new(AtomicBool::new(false)),
        };
        let task_cancellation = cancellation.clone();
        let state = Arc::new(SourceRuntimeObservationState::default());
        let task_state = Arc::clone(&state);
        let join = std::thread::Builder::new()
            .name("pks-typed-source".to_owned())
            .spawn(move || {
                task_state.ready.store(true, Ordering::Release);
                let result = run_source_driver(
                    driver.as_mut(),
                    &manifest,
                    &mut fanouts,
                    &task_cancellation,
                    &task_state,
                    session.as_ref(),
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
        Ok(SourceRuntime {
            cancellation,
            observations: SourceRuntimeObservationHandle { state },
            join: Some(join),
        })
    }
}

impl Drop for PreparedSourceRuntime {
    fn drop(&mut self) {
        if let Some(driver) = self.driver.as_mut() {
            let _ = driver.close();
        }
    }
}

impl SourceRuntime {
    #[cfg(any(test, feature = "internal-testing"))]
    pub fn spawn(
        factory: Arc<dyn SourceFactory>,
        configuration: &SourceConfiguration,
        branch_specs: &[SourceOutputBranchSpec],
    ) -> Result<(Self, Vec<SourceOutputReceiver>), SourceRuntimeError> {
        let (prepared, receivers) =
            PreparedSourceRuntime::prepare(factory, configuration, branch_specs, None)?;
        Ok((prepared.start()?, receivers))
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
    session: Option<&SourceSessionContext>,
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
        if let Some(session) = session {
            let identity = session
                .output(&emission.output_port)
                .ok_or_else(|| SourceRuntimeError::UnknownOutput(emission.output_port.clone()))?;
            let lineage = emission
                .envelope
                .lineage
                .ok_or(SourceRuntimeError::MissingSessionLineage)?;
            if lineage.session_id != session.session_id
                || lineage.source_id != session.source_id
                || lineage.stream_id != identity.stream_id
            {
                return Err(SourceRuntimeError::OutputIdentityMismatch);
            }
        }
        let continuity_observation = continuity
            .entry(emission.output_port.clone())
            .or_default()
            .observe(&emission.envelope)
            .map_err(SourceRuntimeError::Continuity)?;
        if continuity_observation.discontinuity_observed {
            observations
                .discontinuity_total
                .fetch_add(1, Ordering::Relaxed);
        }
        if continuity_observation.source_recovered {
            observations.recovery_total.fetch_add(1, Ordering::Relaxed);
        }
        if continuity_observation.policy_changed {
            observations
                .policy_change_total
                .fetch_add(1, Ordering::Relaxed);
        }
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
#[doc = "Classifies failures reported as source manifest error."]
pub enum SourceManifestError {
    #[error("source type identifier cannot be empty")]
    #[doc = "Reports empty source type identifier."]
    EmptySourceTypeId,
    #[error("source manifest revision and implementation generation must be non-zero")]
    #[doc = "Reports zero version."]
    ZeroVersion,
    #[error("source manifest requires at least one output")]
    #[doc = "Reports no outputs."]
    NoOutputs,
    #[error("source manifest contains a non-output port")]
    #[doc = "Reports non output port."]
    NonOutputPort,
    #[error("source output name cannot be empty")]
    #[doc = "Reports empty output name."]
    EmptyOutputName,
    #[error("source output names must be unique")]
    #[doc = "Reports duplicate output name."]
    DuplicateOutputName,
    #[error("source output SignalSpec is invalid")]
    #[doc = "Reports invalid signal."]
    InvalidSignal,
    #[error("source output SignalSpec and MediaCaps are incompatible")]
    #[doc = "Reports signal media mismatch."]
    SignalMediaMismatch,
    #[error("source safety contract is incompatible with its execution partition")]
    #[doc = "Reports invalid safety contract."]
    InvalidSafetyContract,
    #[error("in-process source drivers currently require the BlockingWorker partition")]
    #[doc = "Reports unsupported execution partition."]
    UnsupportedExecutionPartition,
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as source registration error."]
pub enum SourceRegistrationError {
    #[error("invalid source manifest: {0}")]
    #[doc = "Reports invalid manifest."]
    InvalidManifest(SourceManifestError),
    #[error("source type {0} is already registered")]
    #[doc = "Reports duplicate source type."]
    DuplicateSourceType(SourceTypeId),
    #[error("source type {0} conflicts with an existing graph node type")]
    #[doc = "Reports node type conflict."]
    NodeTypeConflict(SourceTypeId),
}

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
            source_configuration.insert(key, value);
        }
        self.factory.validate_config(&source_configuration)
    }
}

impl std::fmt::Display for SourceTypeId {
    #[doc = "Formats `SourceTypeId` with the requested formatter."]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as source driver error."]
pub enum SourceDriverError {
    #[error("source driver failed: {0}")]
    #[doc = "Reports failed."]
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
    #[error("Session-owned source output is missing signal lineage")]
    MissingSessionLineage,
    #[error("source output identity does not match the Session prepare context")]
    OutputIdentityMismatch,
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
    #[error("prepared source state has already been consumed")]
    PreparedStateConsumed,
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
            implementation_generation: 1,
            outputs,
            execution: ExecutionPartition::BlockingWorker,
            safety: SafetyContract::AllocationAllowed,
        }
    }

    #[test]
    fn given_portable_source_identity_when_constructed_then_contract_is_preserved() {
        let identity = SourceTypeId::new("io.pocketstation.source.pcm.v1").unwrap();

        assert_eq!(identity.as_str(), "io.pocketstation.source.pcm.v1");
    }

    #[test]
    fn given_nonportable_source_identities_when_constructed_then_each_fails_typed() {
        assert_eq!(SourceTypeId::new(""), Err(SourceTypeIdError::Empty));
        assert_eq!(
            SourceTypeId::new(" io.example.source.device.v1"),
            Err(SourceTypeIdError::SurroundingWhitespace)
        );
        assert_eq!(
            SourceTypeId::new("io.example.sourcé.device.v1"),
            Err(SourceTypeIdError::NonAscii)
        );
        assert_eq!(
            SourceTypeId::new("io.example.source.device.v0"),
            Err(SourceTypeIdError::InvalidContractSyntax)
        );
        assert_eq!(
            SourceTypeId::new("io.example.source..device.v1"),
            Err(SourceTypeIdError::InvalidContractSyntax)
        );
        assert_eq!(
            SourceTypeId::new("io.example.source.device\nv1"),
            Err(SourceTypeIdError::InvalidContractSyntax)
        );
        assert_eq!(
            SourceTypeId::new("io.example.operator.device.v1"),
            Err(SourceTypeIdError::MissingSourceCategory)
        );
        assert_eq!(
            SourceTypeId::new("io.example.source.v1"),
            Err(SourceTypeIdError::MissingSourceCategory)
        );
    }

    #[test]
    fn given_oversized_source_identity_when_constructed_then_bound_is_reported() {
        let value = format!("io.example.source.{}.v1", "a".repeat(240));

        assert_eq!(
            SourceTypeId::new(value.clone()),
            Err(SourceTypeIdError::TooLong {
                actual_bytes: value.len(),
                maximum_bytes: crate::graph::identifier::MAX_IDENTIFIER_BYTES,
            })
        );
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

    #[test]
    fn given_provider_owned_source_key_when_stored_then_core_keeps_it_open() {
        let mut configuration = SourceConfiguration::default();
        configuration.insert("provider.api-key", "opaque");

        assert_eq!(configuration.get("provider.api-key"), Some("opaque"));
    }
}

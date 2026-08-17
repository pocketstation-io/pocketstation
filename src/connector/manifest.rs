use std::collections::BTreeSet;

use crate::graph::{ExecutionPartition, NodeDescriptor, OperatorId};

use super::{
    ConnectorConfigurationSchema, ConnectorDeliveryPolicy, ConnectorReadinessPolicy,
    ConnectorRetryPolicy,
};

pub const CONNECTOR_API_REVISION: u32 = 1;
pub const MAX_CONNECTOR_MANIFEST_ENTRIES: usize = 128;
pub const MAX_CONNECTOR_MANIFEST_TEXT_BYTES: usize = 4_096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorCapability {
    id: String,
    documentation: String,
}

impl ConnectorCapability {
    pub fn new(
        id: impl Into<String>,
        documentation: impl Into<String>,
    ) -> Result<Self, ConnectorManifestError> {
        let entry = Self {
            id: id.into(),
            documentation: documentation.into(),
        };
        validate_manifest_entry(&entry.id, &entry.documentation)?;
        Ok(entry)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn documentation(&self) -> &str {
        &self.documentation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorRequirement {
    id: String,
    required: bool,
    documentation: String,
}

impl ConnectorRequirement {
    pub fn new(
        id: impl Into<String>,
        required: bool,
        documentation: impl Into<String>,
    ) -> Result<Self, ConnectorManifestError> {
        let entry = Self {
            id: id.into(),
            required,
            documentation: documentation.into(),
        };
        validate_manifest_entry(&entry.id, &entry.documentation)?;
        Ok(entry)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub const fn required(&self) -> bool {
        self.required
    }

    pub fn documentation(&self) -> &str {
        &self.documentation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorManifest {
    api_revision: u32,
    manifest_revision: u32,
    operator_id: OperatorId,
    package_version: String,
    node: NodeDescriptor,
    configuration: ConnectorConfigurationSchema,
    delivery: ConnectorDeliveryPolicy,
    retry: ConnectorRetryPolicy,
    readiness: ConnectorReadinessPolicy,
    capabilities: Vec<ConnectorCapability>,
    requirements: Vec<ConnectorRequirement>,
}

impl ConnectorManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        manifest_revision: u32,
        operator_id: OperatorId,
        package_version: impl Into<String>,
        node: NodeDescriptor,
        configuration: ConnectorConfigurationSchema,
        delivery: ConnectorDeliveryPolicy,
        retry: ConnectorRetryPolicy,
        readiness: ConnectorReadinessPolicy,
    ) -> Result<Self, ConnectorManifestError> {
        let manifest = Self {
            api_revision: CONNECTOR_API_REVISION,
            manifest_revision,
            operator_id,
            package_version: package_version.into(),
            node,
            configuration,
            delivery,
            retry,
            readiness,
            capabilities: Vec::new(),
            requirements: Vec::new(),
        };
        manifest.validate()?;
        Ok(manifest)
    }

    #[must_use]
    pub fn with_capability(mut self, capability: ConnectorCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    #[must_use]
    pub fn with_requirement(mut self, requirement: ConnectorRequirement) -> Self {
        self.requirements.push(requirement);
        self
    }

    pub const fn api_revision(&self) -> u32 {
        self.api_revision
    }

    pub const fn manifest_revision(&self) -> u32 {
        self.manifest_revision
    }

    pub const fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

    pub fn package_version(&self) -> &str {
        &self.package_version
    }

    pub const fn node(&self) -> &NodeDescriptor {
        &self.node
    }

    pub const fn configuration(&self) -> &ConnectorConfigurationSchema {
        &self.configuration
    }

    pub const fn delivery(&self) -> ConnectorDeliveryPolicy {
        self.delivery
    }

    pub const fn retry(&self) -> ConnectorRetryPolicy {
        self.retry
    }

    pub const fn readiness(&self) -> ConnectorReadinessPolicy {
        self.readiness
    }

    pub fn capabilities(&self) -> &[ConnectorCapability] {
        &self.capabilities
    }

    pub fn requirements(&self) -> &[ConnectorRequirement] {
        &self.requirements
    }

    pub fn validate(&self) -> Result<(), ConnectorManifestError> {
        if self.api_revision != CONNECTOR_API_REVISION {
            return Err(ConnectorManifestError::UnsupportedApiRevision {
                requested: self.api_revision,
                supported: CONNECTOR_API_REVISION,
            });
        }
        if self.manifest_revision == 0 {
            return Err(ConnectorManifestError::InvalidManifestRevision);
        }
        if self.operator_id.as_str().trim().is_empty() {
            return Err(ConnectorManifestError::EmptyOperatorId);
        }
        if self.package_version.trim().is_empty()
            || self.package_version.len() > MAX_CONNECTOR_MANIFEST_TEXT_BYTES
        {
            return Err(ConnectorManifestError::InvalidPackageVersion);
        }
        if self.node.inputs().is_empty() {
            return Err(ConnectorManifestError::MissingInputPort);
        }
        if !self.node.outputs().is_empty() {
            return Err(ConnectorManifestError::OutputPortNotSupported);
        }
        if matches!(
            self.node.execution(),
            ExecutionPartition::AudioCallback | ExecutionPartition::RealtimeCpu
        ) {
            return Err(ConnectorManifestError::RealtimeExecutionForbidden);
        }
        if self.node.inputs().iter().any(|port| {
            !self
                .delivery
                .input_edge()
                .media()
                .is_compatible_with(&port.media())
        }) {
            return Err(ConnectorManifestError::DeliveryMediaMismatch);
        }
        validate_unique_entries(
            self.capabilities.iter().map(ConnectorCapability::id),
            self.capabilities.len(),
        )?;
        validate_unique_entries(
            self.requirements.iter().map(ConnectorRequirement::id),
            self.requirements.len(),
        )?;
        Ok(())
    }
}

fn validate_manifest_entry(id: &str, documentation: &str) -> Result<(), ConnectorManifestError> {
    if id.trim().is_empty() || documentation.trim().is_empty() {
        return Err(ConnectorManifestError::InvalidManifestEntry);
    }
    if id.len() > MAX_CONNECTOR_MANIFEST_TEXT_BYTES
        || documentation.len() > MAX_CONNECTOR_MANIFEST_TEXT_BYTES
    {
        return Err(ConnectorManifestError::ManifestEntryTooLarge);
    }
    Ok(())
}

fn validate_unique_entries<'a>(
    ids: impl Iterator<Item = &'a str>,
    count: usize,
) -> Result<(), ConnectorManifestError> {
    if count > MAX_CONNECTOR_MANIFEST_ENTRIES {
        return Err(ConnectorManifestError::TooManyManifestEntries);
    }
    let mut unique = BTreeSet::new();
    for id in ids {
        if !unique.insert(id) {
            return Err(ConnectorManifestError::DuplicateManifestEntry { id: id.to_owned() });
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConnectorManifestError {
    #[error("connector API revision {requested} is unsupported; Core supports {supported}")]
    UnsupportedApiRevision { requested: u32, supported: u32 },
    #[error("connector manifest revision must be non-zero")]
    InvalidManifestRevision,
    #[error("connector operator id cannot be empty")]
    EmptyOperatorId,
    #[error("connector package version cannot be empty or exceed the byte limit")]
    InvalidPackageVersion,
    #[error("connector manifest requires at least one input port")]
    MissingInputPort,
    #[error("endpoint connectors cannot declare output ports in connector API revision 1")]
    OutputPortNotSupported,
    #[error("connector execution cannot run on a realtime partition")]
    RealtimeExecutionForbidden,
    #[error("connector delivery media is incompatible with a declared input port")]
    DeliveryMediaMismatch,
    #[error("connector manifest entry requires a non-empty id and documentation")]
    InvalidManifestEntry,
    #[error("connector manifest entry exceeds the byte limit")]
    ManifestEntryTooLarge,
    #[error("connector manifest exceeds the entry limit")]
    TooManyManifestEntries,
    #[error("connector manifest contains duplicate entry '{id}'")]
    DuplicateManifestEntry { id: String },
}

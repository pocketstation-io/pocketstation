use std::collections::BTreeSet;

use crate::graph::{ExecutionPartition, NodeDescriptor, OperatorId};

use super::{ConnectorConfigurationSchema, ConnectorReadinessPolicy};

#[doc = "Defines the public connector API revision value."]
pub const CONNECTOR_API_REVISION: u32 = 1;
#[doc = "Sets the maximum supported connector manifest entries."]
pub const MAX_CONNECTOR_MANIFEST_ENTRIES: usize = 128;
#[doc = "Sets the maximum supported connector manifest text bytes."]
pub const MAX_CONNECTOR_MANIFEST_TEXT_BYTES: usize = 4_096;

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Declares a capability advertised by a connector manifest."]
pub struct ConnectorCapability {
    id: String,
    documentation: String,
}

impl ConnectorCapability {
    #[doc = "Creates a new `ConnectorCapability`."]
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

    #[doc = "Returns the id held by `ConnectorCapability`."]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[doc = "Returns the documentation held by `ConnectorCapability`."]
    pub fn documentation(&self) -> &str {
        &self.documentation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Declares a host or configuration requirement that must be satisfied before connector use."]
pub struct ConnectorRequirement {
    id: String,
    required: bool,
    documentation: String,
}

impl ConnectorRequirement {
    #[doc = "Creates a new `ConnectorRequirement`."]
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

    #[doc = "Returns the id held by `ConnectorRequirement`."]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[doc = "Returns the required held by `ConnectorRequirement`."]
    pub const fn required(&self) -> bool {
        self.required
    }

    #[doc = "Returns the documentation held by `ConnectorRequirement`."]
    pub fn documentation(&self) -> &str {
        &self.documentation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Describes the connector manifest contract."]
pub struct ConnectorManifest {
    api_revision: u32,
    manifest_revision: u32,
    operator_id: OperatorId,
    package_version: String,
    node: NodeDescriptor,
    configuration: ConnectorConfigurationSchema,
    readiness: ConnectorReadinessPolicy,
    capabilities: Vec<ConnectorCapability>,
    requirements: Vec<ConnectorRequirement>,
}

impl ConnectorManifest {
    #[allow(clippy::too_many_arguments)]
    #[doc = "Creates a new `ConnectorManifest`."]
    pub fn new(
        manifest_revision: u32,
        operator_id: OperatorId,
        package_version: impl Into<String>,
        node: NodeDescriptor,
        configuration: ConnectorConfigurationSchema,
        readiness: ConnectorReadinessPolicy,
    ) -> Result<Self, ConnectorManifestError> {
        let manifest = Self {
            api_revision: CONNECTOR_API_REVISION,
            manifest_revision,
            operator_id,
            package_version: package_version.into(),
            node,
            configuration,
            readiness,
            capabilities: Vec::new(),
            requirements: Vec::new(),
        };
        manifest.validate()?;
        Ok(manifest)
    }

    #[must_use]
    #[doc = "Sets the capability on `ConnectorManifest` and returns the updated value."]
    pub fn with_capability(mut self, capability: ConnectorCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    #[must_use]
    #[doc = "Sets the requirement on `ConnectorManifest` and returns the updated value."]
    pub fn with_requirement(mut self, requirement: ConnectorRequirement) -> Self {
        self.requirements.push(requirement);
        self
    }

    #[doc = "Returns the API revision held by `ConnectorManifest`."]
    pub const fn api_revision(&self) -> u32 {
        self.api_revision
    }

    #[doc = "Returns the manifest revision held by `ConnectorManifest`."]
    pub const fn manifest_revision(&self) -> u32 {
        self.manifest_revision
    }

    #[doc = "Returns the operator identifier held by `ConnectorManifest`."]
    pub const fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

    #[doc = "Returns the package version held by `ConnectorManifest`."]
    pub fn package_version(&self) -> &str {
        &self.package_version
    }

    #[doc = "Returns the node held by `ConnectorManifest`."]
    pub const fn node(&self) -> &NodeDescriptor {
        &self.node
    }

    #[doc = "Returns the configuration held by `ConnectorManifest`."]
    pub const fn configuration(&self) -> &ConnectorConfigurationSchema {
        &self.configuration
    }

    #[doc = "Returns the readiness held by `ConnectorManifest`."]
    pub const fn readiness(&self) -> ConnectorReadinessPolicy {
        self.readiness
    }

    #[doc = "Returns the capabilities held by `ConnectorManifest`."]
    pub fn capabilities(&self) -> &[ConnectorCapability] {
        &self.capabilities
    }

    #[doc = "Returns the requirements held by `ConnectorManifest`."]
    pub fn requirements(&self) -> &[ConnectorRequirement] {
        &self.requirements
    }

    #[doc = "Validates `ConnectorManifest` against its declared contract."]
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
#[doc = "Classifies failures reported as connector manifest error."]
pub enum ConnectorManifestError {
    #[error("connector API revision {requested} is unsupported; Core supports {supported}")]
    #[doc = "Reports unsupported API revision."]
    UnsupportedApiRevision {
        #[doc = "Stores the requested used by `UnsupportedApiRevision`."]
        requested: u32,
        #[doc = "Stores the supported used by `UnsupportedApiRevision`."]
        supported: u32,
    },
    #[error("connector manifest revision must be non-zero")]
    #[doc = "Reports invalid manifest revision."]
    InvalidManifestRevision,
    #[error("connector operator id cannot be empty")]
    #[doc = "Reports empty operator identifier."]
    EmptyOperatorId,
    #[error("connector package version cannot be empty or exceed the byte limit")]
    #[doc = "Reports invalid package version."]
    InvalidPackageVersion,
    #[error("connector manifest requires at least one input port")]
    #[doc = "Reports missing input port."]
    MissingInputPort,
    #[error("endpoint connectors cannot declare output ports in connector API revision 1")]
    #[doc = "Reports output port not supported."]
    OutputPortNotSupported,
    #[error("connector execution cannot run on a realtime partition")]
    #[doc = "Reports realtime execution forbidden."]
    RealtimeExecutionForbidden,
    #[error("connector manifest entry requires a non-empty id and documentation")]
    #[doc = "Reports invalid manifest entry."]
    InvalidManifestEntry,
    #[error("connector manifest entry exceeds the byte limit")]
    #[doc = "Reports manifest entry too large."]
    ManifestEntryTooLarge,
    #[error("connector manifest exceeds the entry limit")]
    #[doc = "Reports too many manifest entries."]
    TooManyManifestEntries,
    #[error("connector manifest contains duplicate entry '{id}'")]
    #[doc = "Reports duplicate manifest entry."]
    DuplicateManifestEntry {
        #[doc = "Identifies the id recorded by `DuplicateManifestEntry`."]
        id: String,
    },
}

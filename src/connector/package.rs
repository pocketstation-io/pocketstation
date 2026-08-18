use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::{
    AsyncOperatorFactory, EndpointExtensionRegistration, PortSpec, Session, SourceFactory,
};

use super::{
    Connector, ConnectorCapability, ConnectorDefinition, ConnectorManifestError,
    ConnectorRequirement, RegisteredConnector, MAX_CONNECTOR_MANIFEST_ENTRIES,
    MAX_CONNECTOR_MANIFEST_TEXT_BYTES,
};

pub const CONNECTOR_PACKAGE_API_REVISION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorComponentKind {
    Source,
    Operator,
    Endpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectorComponentId(String);

impl ConnectorComponentId {
    pub fn new(value: impl Into<String>) -> Result<Self, ConnectorPackageError> {
        let value = value.into();
        validate_identity(&value, ConnectorPackageError::InvalidComponentId)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectorPackageId(String);

impl ConnectorPackageId {
    pub fn new(value: impl Into<String>) -> Result<Self, ConnectorPackageError> {
        let value = value.into();
        validate_identity(&value, ConnectorPackageError::InvalidPackageId)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorComponentManifest {
    component_id: ConnectorComponentId,
    kind: ConnectorComponentKind,
    ports: Vec<PortSpec>,
    capabilities: Vec<ConnectorCapability>,
    requirements: Vec<ConnectorRequirement>,
}

impl ConnectorComponentManifest {
    pub const fn component_id(&self) -> &ConnectorComponentId {
        &self.component_id
    }

    pub const fn kind(&self) -> ConnectorComponentKind {
        self.kind
    }

    pub fn ports(&self) -> &[PortSpec] {
        &self.ports
    }

    pub fn capabilities(&self) -> &[ConnectorCapability] {
        &self.capabilities
    }

    pub fn requirements(&self) -> &[ConnectorRequirement] {
        &self.requirements
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorPackageManifest {
    api_revision: u32,
    manifest_revision: u32,
    package_id: ConnectorPackageId,
    package_version: String,
    components: Vec<ConnectorComponentManifest>,
}

impl ConnectorPackageManifest {
    pub const fn api_revision(&self) -> u32 {
        self.api_revision
    }

    pub const fn manifest_revision(&self) -> u32 {
        self.manifest_revision
    }

    pub const fn package_id(&self) -> &ConnectorPackageId {
        &self.package_id
    }

    pub fn package_version(&self) -> &str {
        &self.package_version
    }

    pub fn components(&self) -> &[ConnectorComponentManifest] {
        &self.components
    }

    pub fn component(
        &self,
        component_id: &ConnectorComponentId,
    ) -> Option<&ConnectorComponentManifest> {
        self.components
            .iter()
            .find(|component| component.component_id == *component_id)
    }
}

pub struct ConnectorPackage {
    manifest_revision: u32,
    package_id: ConnectorPackageId,
    package_version: String,
    sources: Vec<SourceComponent>,
    operators: Vec<OperatorComponent>,
    endpoints: Vec<EndpointComponent>,
}

struct SourceComponent {
    component_id: ConnectorComponentId,
    factory: Arc<dyn SourceFactory>,
}

struct OperatorComponent {
    component_id: ConnectorComponentId,
    factory: Arc<dyn AsyncOperatorFactory>,
}

struct EndpointComponent {
    component_id: ConnectorComponentId,
    connector: Connector,
}

impl ConnectorPackage {
    pub fn new(
        manifest_revision: u32,
        package_id: ConnectorPackageId,
        package_version: impl Into<String>,
    ) -> Result<Self, ConnectorPackageError> {
        if manifest_revision == 0 {
            return Err(ConnectorPackageError::InvalidManifestRevision);
        }
        let package_version = package_version.into();
        validate_text(
            &package_version,
            ConnectorPackageError::InvalidPackageVersion,
        )?;
        Ok(Self {
            manifest_revision,
            package_id,
            package_version,
            sources: Vec::new(),
            operators: Vec::new(),
            endpoints: Vec::new(),
        })
    }

    pub fn add_source(
        &mut self,
        component_id: ConnectorComponentId,
        factory: Arc<dyn SourceFactory>,
    ) -> Result<(), ConnectorPackageError> {
        factory
            .manifest()
            .validate()
            .map_err(ConnectorPackageError::InvalidSourceManifest)?;
        self.ensure_component_available(&component_id)?;
        self.sources.push(SourceComponent {
            component_id,
            factory,
        });
        self.validate_size()
    }

    pub fn add_operator(
        &mut self,
        component_id: ConnectorComponentId,
        factory: Arc<dyn AsyncOperatorFactory>,
    ) -> Result<(), ConnectorPackageError> {
        factory
            .manifest()
            .validate()
            .map_err(ConnectorPackageError::InvalidOperatorManifest)?;
        self.ensure_component_available(&component_id)?;
        self.operators.push(OperatorComponent {
            component_id,
            factory,
        });
        self.validate_size()
    }

    pub fn add_endpoint(
        &mut self,
        component_id: ConnectorComponentId,
        connector: Connector,
    ) -> Result<(), ConnectorPackageError> {
        connector.manifest().validate()?;
        self.ensure_component_available(&component_id)?;
        self.endpoints.push(EndpointComponent {
            component_id,
            connector,
        });
        self.validate_size()
    }

    pub fn manifest(&self) -> ConnectorPackageManifest {
        let mut components = Vec::with_capacity(self.component_count());
        components.extend(
            self.sources
                .iter()
                .map(|source| ConnectorComponentManifest {
                    component_id: source.component_id.clone(),
                    kind: ConnectorComponentKind::Source,
                    ports: source.factory.manifest().outputs().to_vec(),
                    capabilities: Vec::new(),
                    requirements: Vec::new(),
                }),
        );
        components.extend(self.operators.iter().map(|operator| {
            let manifest = operator.factory.manifest();
            let mut ports = manifest.node().inputs().to_vec();
            ports.extend_from_slice(manifest.node().outputs());
            ConnectorComponentManifest {
                component_id: operator.component_id.clone(),
                kind: ConnectorComponentKind::Operator,
                ports,
                capabilities: Vec::new(),
                requirements: Vec::new(),
            }
        }));
        components.extend(self.endpoints.iter().map(|endpoint| {
            let manifest = endpoint.connector.manifest();
            let mut ports = manifest.node().inputs().to_vec();
            ports.extend_from_slice(manifest.node().outputs());
            ConnectorComponentManifest {
                component_id: endpoint.component_id.clone(),
                kind: ConnectorComponentKind::Endpoint,
                ports,
                capabilities: manifest.capabilities().to_vec(),
                requirements: manifest.requirements().to_vec(),
            }
        }));
        ConnectorPackageManifest {
            api_revision: CONNECTOR_PACKAGE_API_REVISION,
            manifest_revision: self.manifest_revision,
            package_id: self.package_id.clone(),
            package_version: self.package_version.clone(),
            components,
        }
    }

    pub fn install(
        self,
        session: &Session,
    ) -> Result<RegisteredConnectorPackage, ConnectorPackageInstallError> {
        if self.component_count() == 0 {
            return Err(ConnectorPackageError::MissingComponent.into());
        }
        self.validate_component_authorities()?;
        let manifest = self.manifest();
        let mut source_registrations = session.source_registrations.lock().map_err(|_| {
            ConnectorPackageInstallError::RegistrationStateUnavailable {
                authority: "source",
            }
        })?;
        let mut operator_registrations = session.operator_registrations.lock().map_err(|_| {
            ConnectorPackageInstallError::RegistrationStateUnavailable {
                authority: "operator",
            }
        })?;
        let mut endpoint_extensions = session.endpoint_extensions.lock().map_err(|_| {
            ConnectorPackageInstallError::RegistrationStateUnavailable {
                authority: "endpoint extension",
            }
        })?;
        let endpoint_registrations = session.endpoint_registrations.lock().map_err(|_| {
            ConnectorPackageInstallError::RegistrationStateUnavailable {
                authority: "endpoint driver",
            }
        })?;

        let mut existing_source_type_ids = source_registrations
            .iter()
            .map(|factory| factory.manifest().source_type_id().as_str().to_owned())
            .collect::<BTreeSet<_>>();
        let mut existing_operator_ids = operator_registrations
            .iter()
            .map(|factory| factory.manifest().operator_id().as_str().to_owned())
            .collect::<BTreeSet<_>>();
        existing_operator_ids.extend(
            endpoint_extensions
                .iter()
                .map(|registration| registration.operator_id.as_str().to_owned()),
        );
        existing_operator_ids.extend(
            endpoint_registrations
                .iter()
                .map(|registration| registration.operator_id.as_str().to_owned()),
        );
        let mut existing_node_type_ids = operator_registrations
            .iter()
            .map(|factory| factory.manifest().node().type_id().as_str().to_owned())
            .collect::<BTreeSet<_>>();
        existing_node_type_ids.extend(
            source_registrations
                .iter()
                .map(|factory| factory.manifest().source_type_id().as_str().to_owned()),
        );
        existing_node_type_ids.extend(endpoint_extensions.iter().map(|registration| {
            registration
                .definition
                .descriptor()
                .type_id()
                .as_str()
                .to_owned()
        }));
        existing_node_type_ids.extend(
            endpoint_registrations
                .iter()
                .map(|registration| registration.node_type_id.as_str().to_owned()),
        );

        for source in &self.sources {
            let source_type_id = source.factory.manifest().source_type_id().as_str();
            if !existing_source_type_ids.insert(source_type_id.to_owned()) {
                return Err(ConnectorPackageInstallError::DuplicateSourceTypeId {
                    source_type_id: source_type_id.to_owned(),
                });
            }
            if !existing_node_type_ids.insert(source_type_id.to_owned()) {
                return Err(ConnectorPackageInstallError::DuplicateNodeTypeId {
                    node_type_id: source_type_id.to_owned(),
                });
            }
        }
        for operator in &self.operators {
            let operator_id = operator.factory.manifest().operator_id().as_str();
            if !existing_operator_ids.insert(operator_id.to_owned()) {
                return Err(ConnectorPackageInstallError::DuplicateOperatorId {
                    operator_id: operator_id.to_owned(),
                });
            }
            let node_type_id = operator.factory.manifest().node().type_id().as_str();
            if !existing_node_type_ids.insert(node_type_id.to_owned()) {
                return Err(ConnectorPackageInstallError::DuplicateNodeTypeId {
                    node_type_id: node_type_id.to_owned(),
                });
            }
        }
        for endpoint in &self.endpoints {
            let operator_id = endpoint.connector.manifest.operator_id().as_str();
            if !existing_operator_ids.insert(operator_id.to_owned()) {
                return Err(ConnectorPackageInstallError::DuplicateOperatorId {
                    operator_id: operator_id.to_owned(),
                });
            }
            let node_type_id = endpoint.connector.manifest.node().type_id().as_str();
            if !existing_node_type_ids.insert(node_type_id.to_owned()) {
                return Err(ConnectorPackageInstallError::DuplicateNodeTypeId {
                    node_type_id: node_type_id.to_owned(),
                });
            }
        }

        let mut endpoints = BTreeMap::new();
        for endpoint in self.endpoints {
            let Connector {
                manifest,
                endpoint_factory,
                observations,
            } = endpoint.connector;
            let operator_id = manifest.operator_id().clone();
            endpoint_extensions.push(EndpointExtensionRegistration {
                operator_id,
                definition: Arc::new(ConnectorDefinition {
                    manifest: Arc::clone(&manifest),
                }),
                factory: endpoint_factory,
            });
            endpoints.insert(
                endpoint.component_id,
                RegisteredConnector {
                    session_id: session.id(),
                    manifest,
                    observations,
                },
            );
        }
        for source in self.sources {
            source_registrations.push(source.factory);
        }
        for operator in self.operators {
            operator_registrations.push(operator.factory);
        }
        Ok(RegisteredConnectorPackage {
            session_id: session.id(),
            manifest,
            endpoints,
        })
    }

    fn component_count(&self) -> usize {
        self.sources.len() + self.operators.len() + self.endpoints.len()
    }

    fn ensure_component_available(
        &self,
        component_id: &ConnectorComponentId,
    ) -> Result<(), ConnectorPackageError> {
        if self
            .sources
            .iter()
            .any(|component| component.component_id == *component_id)
            || self
                .operators
                .iter()
                .any(|component| component.component_id == *component_id)
            || self
                .endpoints
                .iter()
                .any(|component| component.component_id == *component_id)
        {
            return Err(ConnectorPackageError::DuplicateComponentId {
                component_id: component_id.as_str().to_owned(),
            });
        }
        Ok(())
    }

    fn validate_size(&self) -> Result<(), ConnectorPackageError> {
        if self.component_count() > MAX_CONNECTOR_MANIFEST_ENTRIES {
            return Err(ConnectorPackageError::TooManyComponents);
        }
        Ok(())
    }

    fn validate_component_authorities(&self) -> Result<(), ConnectorPackageError> {
        let mut node_type_ids = BTreeSet::new();
        let mut operator_ids = BTreeSet::new();
        let mut source_type_ids = BTreeSet::new();
        for source in &self.sources {
            let source_type_id = source.factory.manifest().source_type_id().as_str();
            if !source_type_ids.insert(source_type_id) {
                return Err(ConnectorPackageError::DuplicateSourceTypeId);
            }
            if !node_type_ids.insert(source_type_id) {
                return Err(ConnectorPackageError::DuplicateNodeTypeId);
            }
        }
        for operator in &self.operators {
            let manifest = operator.factory.manifest();
            if !node_type_ids.insert(manifest.node().type_id().as_str()) {
                return Err(ConnectorPackageError::DuplicateNodeTypeId);
            }
            if !operator_ids.insert(manifest.operator_id().as_str()) {
                return Err(ConnectorPackageError::DuplicateOperatorId);
            }
        }
        for endpoint in &self.endpoints {
            let manifest = endpoint.connector.manifest();
            if !node_type_ids.insert(manifest.node().type_id().as_str()) {
                return Err(ConnectorPackageError::DuplicateNodeTypeId);
            }
            if !operator_ids.insert(manifest.operator_id().as_str()) {
                return Err(ConnectorPackageError::DuplicateOperatorId);
            }
        }
        Ok(())
    }
}

pub struct RegisteredConnectorPackage {
    session_id: crate::SessionId,
    manifest: ConnectorPackageManifest,
    endpoints: BTreeMap<ConnectorComponentId, RegisteredConnector>,
}

impl RegisteredConnectorPackage {
    pub const fn session_id(&self) -> crate::SessionId {
        self.session_id
    }

    pub const fn manifest(&self) -> &ConnectorPackageManifest {
        &self.manifest
    }

    pub fn endpoint(&self, component_id: &ConnectorComponentId) -> Option<&RegisteredConnector> {
        self.endpoints.get(component_id)
    }
}

fn validate_identity(
    value: &str,
    error: ConnectorPackageError,
) -> Result<(), ConnectorPackageError> {
    validate_text(value, error.clone())?;
    if !value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
    }) {
        return Err(error);
    }
    Ok(())
}

fn validate_text(value: &str, error: ConnectorPackageError) -> Result<(), ConnectorPackageError> {
    if value.trim().is_empty() || value.len() > MAX_CONNECTOR_MANIFEST_TEXT_BYTES {
        return Err(error);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConnectorPackageError {
    #[error("connector package manifest revision must be non-zero")]
    InvalidManifestRevision,
    #[error("connector package id is empty, too large, or contains an invalid character")]
    InvalidPackageId,
    #[error("connector package version is empty or exceeds the byte limit")]
    InvalidPackageVersion,
    #[error("connector component id is empty, too large, or contains an invalid character")]
    InvalidComponentId,
    #[error("connector package contains duplicate component id '{component_id}'")]
    DuplicateComponentId { component_id: String },
    #[error("connector package requires at least one component")]
    MissingComponent,
    #[error("connector package exceeds the component limit")]
    TooManyComponents,
    #[error("connector package contains duplicate source type identity")]
    DuplicateSourceTypeId,
    #[error("connector package contains duplicate operator identity")]
    DuplicateOperatorId,
    #[error("connector package contains duplicate node type identity")]
    DuplicateNodeTypeId,
    #[error("connector package source manifest is invalid: {0}")]
    InvalidSourceManifest(crate::SourceManifestError),
    #[error("connector package operator manifest is invalid: {0}")]
    InvalidOperatorManifest(crate::AsyncOperatorManifestError),
    #[error(transparent)]
    InvalidEndpointManifest(#[from] ConnectorManifestError),
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectorPackageInstallError {
    #[error(transparent)]
    Package(#[from] ConnectorPackageError),
    #[error("connector package cannot access {authority} registration state")]
    RegistrationStateUnavailable { authority: &'static str },
    #[error("connector package source type id '{source_type_id}' is already registered")]
    DuplicateSourceTypeId { source_type_id: String },
    #[error("connector package operator id '{operator_id}' is already registered")]
    DuplicateOperatorId { operator_id: String },
    #[error("connector package node type id '{node_type_id}' is already registered")]
    DuplicateNodeTypeId { node_type_id: String },
}

use std::collections::HashSet;

use pks_frame::{ConnectorId, EndpointId, RouteId, SessionId, StemId};
use pks_graph::NodeTypeId;

use crate::{EndpointConfiguration, OperatorId, SessionError, Source};

pub const SESSION_SPEC_VERSION: SessionSpecVersion = SessionSpecVersion::new(1, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionSpecVersion {
    major: u16,
    minor: u16,
}

impl SessionSpecVersion {
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub const fn major(self) -> u16 {
        self.major
    }

    pub const fn minor(self) -> u16 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StemSpec {
    stem_id: StemId,
    source: Source,
}

impl StemSpec {
    pub const fn id(&self) -> StemId {
        self.stem_id
    }

    pub fn source(&self) -> &Source {
        &self.source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointSpec {
    endpoint_id: EndpointId,
    node_type_id: NodeTypeId,
    operator_id: OperatorId,
    configuration: EndpointConfiguration,
    connector_id: Option<ConnectorId>,
}

impl EndpointSpec {
    pub const fn id(&self) -> EndpointId {
        self.endpoint_id
    }

    pub fn node_type_id(&self) -> &NodeTypeId {
        &self.node_type_id
    }

    pub fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

    pub fn configuration(&self) -> &EndpointConfiguration {
        &self.configuration
    }

    pub const fn connector_id(&self) -> Option<ConnectorId> {
        self.connector_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteSpec {
    route_id: RouteId,
    stem_id: StemId,
    endpoint_id: EndpointId,
}

impl RouteSpec {
    pub const fn id(self) -> RouteId {
        self.route_id
    }

    pub const fn stem_id(self) -> StemId {
        self.stem_id
    }

    pub const fn endpoint_id(self) -> EndpointId {
        self.endpoint_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSpec {
    version: SessionSpecVersion,
    session_id: SessionId,
    stems: Vec<StemSpec>,
    endpoints: Vec<EndpointSpec>,
    routes: Vec<RouteSpec>,
}

impl SessionSpec {
    pub(crate) fn new(
        session_id: SessionId,
        stems: Vec<StemSpec>,
        endpoints: Vec<EndpointSpec>,
        routes: Vec<RouteSpec>,
    ) -> Self {
        Self {
            version: SESSION_SPEC_VERSION,
            session_id,
            stems,
            endpoints,
            routes,
        }
    }

    pub const fn version(&self) -> SessionSpecVersion {
        self.version
    }

    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn stems(&self) -> &[StemSpec] {
        &self.stems
    }

    pub fn endpoints(&self) -> &[EndpointSpec] {
        &self.endpoints
    }

    pub fn routes(&self) -> &[RouteSpec] {
        &self.routes
    }

    pub fn validate(&self) -> Result<(), SessionError> {
        if self.version.major != SESSION_SPEC_VERSION.major
            || self.version.minor > SESSION_SPEC_VERSION.minor
        {
            return Err(SessionError::UnsupportedVersion {
                major: self.version.major,
                minor: self.version.minor,
            });
        }
        if self.stems.is_empty() {
            return Err(SessionError::NoSources);
        }

        let mut stem_ids = HashSet::with_capacity(self.stems.len());
        for stem in &self.stems {
            if !stem_ids.insert(stem.stem_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate stem id {:?}", stem.stem_id),
                });
            }
            stem.source.validate()?;
            if !self
                .routes
                .iter()
                .any(|route| route.stem_id == stem.stem_id)
            {
                return Err(SessionError::NoRoutes {
                    stem_id: stem.stem_id,
                });
            }
        }

        let mut endpoint_ids = HashSet::with_capacity(self.endpoints.len());
        for endpoint in &self.endpoints {
            if !endpoint_ids.insert(endpoint.endpoint_id) {
                return Err(SessionError::InvalidEndpoint {
                    reason: format!("duplicate endpoint id {:?}", endpoint.endpoint_id),
                });
            }
            if endpoint.node_type_id.as_str().trim().is_empty()
                || endpoint.operator_id.as_str().trim().is_empty()
            {
                return Err(SessionError::InvalidEndpoint {
                    reason: format!(
                        "endpoint {:?} requires node type and operator ids",
                        endpoint.endpoint_id
                    ),
                });
            }
            endpoint.configuration.validate()?;
        }

        let mut route_ids = HashSet::with_capacity(self.routes.len());
        for route in &self.routes {
            if !route_ids.insert(route.route_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate route id {:?}", route.route_id),
                });
            }
            if !stem_ids.contains(&route.stem_id) {
                return Err(SessionError::UnknownStem {
                    stem_id: route.stem_id,
                });
            }
            if !endpoint_ids.contains(&route.endpoint_id) {
                return Err(SessionError::UnknownEndpoint {
                    endpoint_id: route.endpoint_id,
                });
            }
        }
        Ok(())
    }
}

pub(crate) fn stem_spec(stem_id: StemId, source: Source) -> StemSpec {
    StemSpec { stem_id, source }
}

pub(crate) fn endpoint_spec(
    endpoint_id: EndpointId,
    node_type_id: NodeTypeId,
    operator_id: OperatorId,
    configuration: EndpointConfiguration,
    connector_id: Option<ConnectorId>,
) -> EndpointSpec {
    EndpointSpec {
        endpoint_id,
        node_type_id,
        operator_id,
        configuration,
        connector_id,
    }
}

pub(crate) const fn route_spec(
    route_id: RouteId,
    stem_id: StemId,
    endpoint_id: EndpointId,
) -> RouteSpec {
    RouteSpec {
        route_id,
        stem_id,
        endpoint_id,
    }
}

use std::collections::HashSet;

use crate::frame::{ConnectorId, EndpointId, RouteId, SessionId, StemId};
use crate::graph::NodeTypeId;

use crate::session::{
    EndpointConfiguration, OperatorConfiguration, OperatorId, SessionError, Source,
};

pub const SESSION_SPEC_VERSION: SessionSpecVersion = SessionSpecVersion::new(1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperatorInstanceId(u64);

impl OperatorInstanceId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

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

#[derive(Debug, Clone)]
pub struct OperatorSpec {
    instance_id: OperatorInstanceId,
    input_route_id: RouteId,
    source_stem_id: StemId,
    input_origin: OperatorInputOrigin,
    input_port: Option<String>,
    operator_id: OperatorId,
    configuration: OperatorConfiguration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorInputOrigin {
    Stem(StemId),
    OperatorOutput {
        operator_instance_id: OperatorInstanceId,
        output_port: Option<String>,
    },
}

impl OperatorSpec {
    pub const fn instance_id(&self) -> OperatorInstanceId {
        self.instance_id
    }

    pub const fn input_route_id(&self) -> RouteId {
        self.input_route_id
    }

    pub const fn source_stem_id(&self) -> StemId {
        self.source_stem_id
    }

    pub const fn input_origin(&self) -> &OperatorInputOrigin {
        &self.input_origin
    }

    pub fn input_port(&self) -> Option<&str> {
        self.input_port.as_deref()
    }

    pub const fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

    pub const fn configuration(&self) -> &OperatorConfiguration {
        &self.configuration
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedRouteSpec {
    route_id: RouteId,
    operator_instance_id: OperatorInstanceId,
    endpoint_id: EndpointId,
    output_port: Option<String>,
}

impl DerivedRouteSpec {
    pub const fn id(&self) -> RouteId {
        self.route_id
    }

    pub const fn operator_instance_id(&self) -> OperatorInstanceId {
        self.operator_instance_id
    }

    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }

    pub fn output_port(&self) -> Option<&str> {
        self.output_port.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct SessionSpec {
    version: SessionSpecVersion,
    session_id: SessionId,
    stems: Vec<StemSpec>,
    endpoints: Vec<EndpointSpec>,
    routes: Vec<RouteSpec>,
    operators: Vec<OperatorSpec>,
    derived_routes: Vec<DerivedRouteSpec>,
}

impl SessionSpec {
    pub(crate) fn new(
        session_id: SessionId,
        stems: Vec<StemSpec>,
        endpoints: Vec<EndpointSpec>,
        routes: Vec<RouteSpec>,
        operators: Vec<OperatorSpec>,
        derived_routes: Vec<DerivedRouteSpec>,
    ) -> Self {
        Self {
            version: SESSION_SPEC_VERSION,
            session_id,
            stems,
            endpoints,
            routes,
            operators,
            derived_routes,
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

    pub fn operators(&self) -> &[OperatorSpec] {
        &self.operators
    }

    pub fn derived_routes(&self) -> &[DerivedRouteSpec] {
        &self.derived_routes
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
        if self.version.minor < 1 && (!self.operators.is_empty() || !self.derived_routes.is_empty())
        {
            return Err(SessionError::InvalidOperator {
                reason: "derived operators require Session schema version 1.1 or newer".to_owned(),
            });
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
                && !self
                    .operators
                    .iter()
                    .any(|operator| operator.source_stem_id == stem.stem_id)
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

        let mut operator_instance_ids = HashSet::with_capacity(self.operators.len());
        for operator in &self.operators {
            if !operator_instance_ids.insert(operator.instance_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate operator instance id {:?}", operator.instance_id),
                });
            }
            if !route_ids.insert(operator.input_route_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate route id {:?}", operator.input_route_id),
                });
            }
            if !stem_ids.contains(&operator.source_stem_id) {
                return Err(SessionError::UnknownStem {
                    stem_id: operator.source_stem_id,
                });
            }
            match &operator.input_origin {
                OperatorInputOrigin::Stem(stem_id) => {
                    if !stem_ids.contains(stem_id) || *stem_id != operator.source_stem_id {
                        return Err(SessionError::UnknownStem { stem_id: *stem_id });
                    }
                }
                OperatorInputOrigin::OperatorOutput {
                    operator_instance_id,
                    ..
                } => {
                    let Some(upstream) = self
                        .operators
                        .iter()
                        .find(|candidate| candidate.instance_id == *operator_instance_id)
                    else {
                        return Err(SessionError::UnknownOperatorInstance {
                            operator_instance_id: *operator_instance_id,
                        });
                    };
                    if upstream.source_stem_id != operator.source_stem_id {
                        return Err(SessionError::InvalidOperator {
                            reason: "chained operator source stem identity changed".to_owned(),
                        });
                    }
                }
            }
            if operator.operator_id.as_str().trim().is_empty() {
                return Err(SessionError::InvalidOperator {
                    reason: "operator id cannot be empty".to_owned(),
                });
            }
        }

        for route in &self.derived_routes {
            if !route_ids.insert(route.route_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate route id {:?}", route.route_id),
                });
            }
            if !operator_instance_ids.contains(&route.operator_instance_id) {
                return Err(SessionError::UnknownOperatorInstance {
                    operator_instance_id: route.operator_instance_id,
                });
            }
            if !endpoint_ids.contains(&route.endpoint_id) {
                return Err(SessionError::UnknownEndpoint {
                    endpoint_id: route.endpoint_id,
                });
            }
        }
        for operator in &self.operators {
            if !self
                .derived_routes
                .iter()
                .any(|route| route.operator_instance_id == operator.instance_id)
                && !self.operators.iter().any(|candidate| {
                    matches!(
                        candidate.input_origin,
                        OperatorInputOrigin::OperatorOutput {
                            operator_instance_id,
                            ..
                        } if operator_instance_id == operator.instance_id
                    )
                })
            {
                return Err(SessionError::OperatorHasNoDestination {
                    operator_instance_id: operator.instance_id,
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

pub(crate) fn operator_spec(
    instance_id: OperatorInstanceId,
    input_route_id: RouteId,
    source_stem_id: StemId,
    input_origin: OperatorInputOrigin,
    input_port: Option<String>,
    operator_id: OperatorId,
    configuration: OperatorConfiguration,
) -> OperatorSpec {
    OperatorSpec {
        instance_id,
        input_route_id,
        source_stem_id,
        input_origin,
        input_port,
        operator_id,
        configuration,
    }
}

pub(crate) fn derived_route_spec(
    route_id: RouteId,
    operator_instance_id: OperatorInstanceId,
    endpoint_id: EndpointId,
    output_port: Option<String>,
) -> DerivedRouteSpec {
    DerivedRouteSpec {
        route_id,
        operator_instance_id,
        endpoint_id,
        output_port,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{DeviceSelector, EndpointConfiguration};

    fn valid_legacy_compatible_spec() -> SessionSpec {
        SessionSpec {
            version: SessionSpecVersion::new(1, 0),
            session_id: SessionId(1),
            stems: vec![stem_spec(
                StemId(1),
                Source::microphone(DeviceSelector::default()),
            )],
            endpoints: vec![endpoint_spec(
                EndpointId(1),
                NodeTypeId::from("endpoint.test"),
                OperatorId::new("example.endpoint.test.v1"),
                EndpointConfiguration::new(),
                None,
            )],
            routes: vec![route_spec(RouteId(1), StemId(1), EndpointId(1))],
            operators: Vec::new(),
            derived_routes: Vec::new(),
        }
    }

    #[test]
    fn given_legacy_minor_version_when_validated_then_existing_direct_routes_remain_supported() {
        let spec = valid_legacy_compatible_spec();

        assert_eq!(spec.validate(), Ok(()));
    }

    #[test]
    fn given_newer_minor_version_when_validated_then_schema_fails_closed() {
        let mut spec = valid_legacy_compatible_spec();
        spec.version = SessionSpecVersion::new(1, SESSION_SPEC_VERSION.minor() + 1);

        assert!(matches!(
            spec.validate(),
            Err(SessionError::UnsupportedVersion { major: 1, minor: 3 })
        ));
    }

    #[test]
    fn given_current_schema_when_version_read_then_derived_route_extension_is_recorded() {
        assert_eq!(SESSION_SPEC_VERSION, SessionSpecVersion::new(1, 2));
    }

    #[test]
    fn given_legacy_version_with_derived_fields_when_validated_then_schema_lie_is_rejected() {
        let mut spec = valid_legacy_compatible_spec();
        spec.operators.push(operator_spec(
            OperatorInstanceId::new(1),
            RouteId(2),
            StemId(1),
            OperatorInputOrigin::Stem(StemId(1)),
            None,
            OperatorId::new("example.operator.test.v1"),
            OperatorConfiguration::new(),
        ));

        assert!(matches!(
            spec.validate(),
            Err(SessionError::InvalidOperator { .. })
        ));
    }
}

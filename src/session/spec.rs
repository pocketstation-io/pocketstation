use std::collections::HashSet;

use crate::frame::{ConnectorId, EndpointId, RouteId, SessionId, SourceId, StemId, StreamId};
use crate::graph::NodeTypeId;

use crate::session::{
    EndpointConfiguration, OperatorConfiguration, OperatorId, SessionError, Source,
    SourceConfiguration, SourceTypeId,
};

pub const SESSION_SPEC_VERSION: SessionSpecVersion = SessionSpecVersion::new(1, 4);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceInstanceId(u64);

impl SourceInstanceId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceInstanceSpec {
    instance_id: SourceInstanceId,
    source_id: SourceId,
    source_type_id: SourceTypeId,
    configuration: SourceConfiguration,
}

impl SourceInstanceSpec {
    pub const fn instance_id(&self) -> SourceInstanceId {
        self.instance_id
    }

    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    pub const fn source_type_id(&self) -> &SourceTypeId {
        &self.source_type_id
    }

    pub const fn configuration(&self) -> &SourceConfiguration {
        &self.configuration
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceOutputSpec {
    source_instance_id: SourceInstanceId,
    output_port: String,
    stream_id: StreamId,
}

impl SourceOutputSpec {
    pub const fn source_instance_id(&self) -> SourceInstanceId {
        self.source_instance_id
    }

    pub fn output_port(&self) -> &str {
        &self.output_port
    }

    pub const fn stream_id(&self) -> StreamId {
        self.stream_id
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRouteSpec {
    route_id: RouteId,
    source_instance_id: SourceInstanceId,
    output_port: String,
    stream_id: StreamId,
    source_id: SourceId,
    endpoint_id: EndpointId,
    endpoint_input_port: Option<String>,
}

impl SourceRouteSpec {
    pub const fn id(&self) -> RouteId {
        self.route_id
    }

    pub const fn source_instance_id(&self) -> SourceInstanceId {
        self.source_instance_id
    }

    pub fn output_port(&self) -> &str {
        &self.output_port
    }

    pub const fn stream_id(&self) -> StreamId {
        self.stream_id
    }

    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }

    pub fn endpoint_input_port(&self) -> Option<&str> {
        self.endpoint_input_port.as_deref()
    }
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
pub struct OperatorInstanceSpec {
    instance_id: OperatorInstanceId,
    operator_id: OperatorId,
    configuration: OperatorConfiguration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorConnectionSpec {
    route_id: RouteId,
    operator_instance_id: OperatorInstanceId,
    input_origin: OperatorInputOrigin,
    input_port: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorInputOrigin {
    Stem(StemId),
    SourceOutput {
        source_instance_id: SourceInstanceId,
        output_port: String,
        stream_id: StreamId,
        source_id: SourceId,
    },
    OperatorOutput {
        operator_instance_id: OperatorInstanceId,
        output_port: Option<String>,
    },
}

impl OperatorInstanceSpec {
    pub const fn instance_id(&self) -> OperatorInstanceId {
        self.instance_id
    }

    pub const fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

    pub const fn configuration(&self) -> &OperatorConfiguration {
        &self.configuration
    }
}

impl OperatorConnectionSpec {
    pub const fn route_id(&self) -> RouteId {
        self.route_id
    }

    pub const fn operator_instance_id(&self) -> OperatorInstanceId {
        self.operator_instance_id
    }

    pub const fn input_origin(&self) -> &OperatorInputOrigin {
        &self.input_origin
    }

    pub fn input_port(&self) -> Option<&str> {
        self.input_port.as_deref()
    }
}

#[deprecated(note = "use OperatorInstanceSpec")]
pub type OperatorSpec = OperatorInstanceSpec;

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
    source_instances: Vec<SourceInstanceSpec>,
    source_outputs: Vec<SourceOutputSpec>,
    endpoints: Vec<EndpointSpec>,
    routes: Vec<RouteSpec>,
    source_routes: Vec<SourceRouteSpec>,
    operators: Vec<OperatorInstanceSpec>,
    operator_connections: Vec<OperatorConnectionSpec>,
    derived_routes: Vec<DerivedRouteSpec>,
}

pub(crate) struct SessionSpecDeclarations {
    pub(crate) stems: Vec<StemSpec>,
    pub(crate) source_instances: Vec<SourceInstanceSpec>,
    pub(crate) source_outputs: Vec<SourceOutputSpec>,
    pub(crate) endpoints: Vec<EndpointSpec>,
    pub(crate) routes: Vec<RouteSpec>,
    pub(crate) source_routes: Vec<SourceRouteSpec>,
    pub(crate) operators: Vec<OperatorInstanceSpec>,
    pub(crate) operator_connections: Vec<OperatorConnectionSpec>,
    pub(crate) derived_routes: Vec<DerivedRouteSpec>,
}

impl SessionSpec {
    pub(crate) fn new(session_id: SessionId, declarations: SessionSpecDeclarations) -> Self {
        Self {
            version: SESSION_SPEC_VERSION,
            session_id,
            stems: declarations.stems,
            source_instances: declarations.source_instances,
            source_outputs: declarations.source_outputs,
            endpoints: declarations.endpoints,
            routes: declarations.routes,
            source_routes: declarations.source_routes,
            operators: declarations.operators,
            operator_connections: declarations.operator_connections,
            derived_routes: declarations.derived_routes,
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

    pub fn source_instances(&self) -> &[SourceInstanceSpec] {
        &self.source_instances
    }

    pub fn source_outputs(&self) -> &[SourceOutputSpec] {
        &self.source_outputs
    }

    pub fn endpoints(&self) -> &[EndpointSpec] {
        &self.endpoints
    }

    pub fn routes(&self) -> &[RouteSpec] {
        &self.routes
    }

    pub fn source_routes(&self) -> &[SourceRouteSpec] {
        &self.source_routes
    }

    pub fn operators(&self) -> &[OperatorInstanceSpec] {
        &self.operators
    }

    pub fn operator_connections(&self) -> &[OperatorConnectionSpec] {
        &self.operator_connections
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
        if self.stems.is_empty() && self.source_instances.is_empty() {
            return Err(SessionError::NoSources);
        }
        if self.version.minor < 1
            && (!self.operators.is_empty()
                || !self.operator_connections.is_empty()
                || !self.derived_routes.is_empty())
        {
            return Err(SessionError::InvalidOperator {
                reason: "derived operators require Session schema version 1.1 or newer".to_owned(),
            });
        }
        if self.version.minor < 3
            && (!self.source_instances.is_empty()
                || !self.source_outputs.is_empty()
                || !self.source_routes.is_empty()
                || self.operator_connections.iter().any(|connection| {
                    matches!(
                        connection.input_origin,
                        OperatorInputOrigin::SourceOutput { .. }
                    )
                }))
        {
            return Err(SessionError::InvalidRoute {
                reason: "external sources require Session schema version 1.3 or newer".to_owned(),
            });
        }
        if self.version.minor < 4 && !self.operator_connections.is_empty() {
            return Err(SessionError::InvalidOperator {
                reason: "named operator connections require Session schema version 1.4 or newer"
                    .to_owned(),
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
                    .operator_connections
                    .iter()
                    .any(|connection| {
                        matches!(connection.input_origin, OperatorInputOrigin::Stem(stem_id) if stem_id == stem.stem_id)
                    })
            {
                return Err(SessionError::NoRoutes {
                    stem_id: stem.stem_id,
                });
            }
        }

        let mut source_instance_ids = HashSet::with_capacity(self.source_instances.len());
        let mut source_ids = HashSet::with_capacity(self.source_instances.len());
        for source in &self.source_instances {
            if !source_instance_ids.insert(source.instance_id)
                || !source_ids.insert(source.source_id)
            {
                return Err(SessionError::InvalidRoute {
                    reason: "duplicate external source instance or source identity".to_owned(),
                });
            }
            if !self
                .source_outputs
                .iter()
                .any(|output| output.source_instance_id == source.instance_id)
            {
                return Err(SessionError::NoSourceOutputs {
                    source_instance_id: source.instance_id,
                });
            }
        }

        let mut source_output_keys = HashSet::with_capacity(self.source_outputs.len());
        let mut stream_ids = HashSet::with_capacity(self.source_outputs.len());
        for output in &self.source_outputs {
            if !source_instance_ids.contains(&output.source_instance_id) {
                return Err(SessionError::UnknownSourceInstance {
                    source_instance_id: output.source_instance_id,
                });
            }
            if output.output_port.trim().is_empty()
                || !source_output_keys
                    .insert((output.source_instance_id, output.output_port.clone()))
                || !stream_ids.insert(output.stream_id)
            {
                return Err(SessionError::InvalidRoute {
                    reason: "external source outputs require unique non-empty ports and stream identities"
                        .to_owned(),
                });
            }
            let directly_routed = self.source_routes.iter().any(|route| {
                route.source_instance_id == output.source_instance_id
                    && route.output_port == output.output_port
            });
            let operator_routed = self.operator_connections.iter().any(|connection| {
                matches!(
                    &connection.input_origin,
                    OperatorInputOrigin::SourceOutput {
                        source_instance_id,
                        output_port,
                        ..
                    } if *source_instance_id == output.source_instance_id
                        && output_port == &output.output_port
                )
            });
            if !directly_routed && !operator_routed {
                return Err(SessionError::NoSourceOutputRoutes {
                    source_instance_id: output.source_instance_id,
                    output_port: output.output_port.clone(),
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

        for route in &self.source_routes {
            if !route_ids.insert(route.route_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate route id {:?}", route.route_id),
                });
            }
            let Some(output) = self.source_outputs.iter().find(|output| {
                output.source_instance_id == route.source_instance_id
                    && output.output_port == route.output_port
            }) else {
                return Err(SessionError::UnknownSourceOutput {
                    source_instance_id: route.source_instance_id,
                    output_port: route.output_port.clone(),
                });
            };
            let source = self
                .source_instances
                .iter()
                .find(|source| source.instance_id == route.source_instance_id)
                .ok_or(SessionError::UnknownSourceInstance {
                    source_instance_id: route.source_instance_id,
                })?;
            if output.stream_id != route.stream_id || source.source_id != route.source_id {
                return Err(SessionError::InvalidRoute {
                    reason: "external source route changed assigned stream or source identity"
                        .to_owned(),
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
            if operator.operator_id.as_str().trim().is_empty() {
                return Err(SessionError::InvalidOperator {
                    reason: "operator id cannot be empty".to_owned(),
                });
            }
        }

        let mut connected_inputs = HashSet::with_capacity(self.operator_connections.len());
        for connection in &self.operator_connections {
            if !route_ids.insert(connection.route_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate route id {:?}", connection.route_id),
                });
            }
            if !operator_instance_ids.contains(&connection.operator_instance_id) {
                return Err(SessionError::UnknownOperatorInstance {
                    operator_instance_id: connection.operator_instance_id,
                });
            }
            if connection
                .input_port
                .as_deref()
                .is_some_and(|port| port.trim().is_empty())
            {
                return Err(SessionError::InvalidOperator {
                    reason: "operator input port cannot be empty".to_owned(),
                });
            }
            if let Some(input_port) = &connection.input_port {
                if !connected_inputs.insert((connection.operator_instance_id, input_port.clone())) {
                    return Err(SessionError::InvalidOperator {
                        reason: format!(
                            "operator instance {:?} input port '{}' is connected more than once",
                            connection.operator_instance_id, input_port
                        ),
                    });
                }
            }
            match &connection.input_origin {
                OperatorInputOrigin::Stem(stem_id) => {
                    if !stem_ids.contains(stem_id) {
                        return Err(SessionError::UnknownStem { stem_id: *stem_id });
                    }
                }
                OperatorInputOrigin::SourceOutput {
                    source_instance_id,
                    output_port,
                    stream_id,
                    source_id,
                } => {
                    let Some(output) = self.source_outputs.iter().find(|output| {
                        output.source_instance_id == *source_instance_id
                            && output.output_port == *output_port
                    }) else {
                        return Err(SessionError::UnknownSourceOutput {
                            source_instance_id: *source_instance_id,
                            output_port: output_port.clone(),
                        });
                    };
                    let source = self
                        .source_instances
                        .iter()
                        .find(|source| source.instance_id == *source_instance_id)
                        .ok_or(SessionError::UnknownSourceInstance {
                            source_instance_id: *source_instance_id,
                        })?;
                    if output.stream_id != *stream_id || source.source_id != *source_id {
                        return Err(SessionError::InvalidOperator {
                            reason:
                                "external operator input changed assigned stream or source identity"
                                    .to_owned(),
                        });
                    }
                }
                OperatorInputOrigin::OperatorOutput {
                    operator_instance_id,
                    ..
                } => {
                    let Some(_upstream) = self
                        .operators
                        .iter()
                        .find(|candidate| candidate.instance_id == *operator_instance_id)
                    else {
                        return Err(SessionError::UnknownOperatorInstance {
                            operator_instance_id: *operator_instance_id,
                        });
                    };
                }
            }
        }
        for operator in &self.operators {
            if !self
                .operator_connections
                .iter()
                .any(|connection| connection.operator_instance_id == operator.instance_id)
            {
                return Err(SessionError::InvalidOperator {
                    reason: format!(
                        "operator instance {:?} has no input connection",
                        operator.instance_id
                    ),
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
                && !self.operator_connections.iter().any(|connection| {
                    matches!(
                        connection.input_origin,
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

pub(crate) fn source_instance_spec(
    instance_id: SourceInstanceId,
    source_id: SourceId,
    source_type_id: SourceTypeId,
    configuration: SourceConfiguration,
) -> SourceInstanceSpec {
    SourceInstanceSpec {
        instance_id,
        source_id,
        source_type_id,
        configuration,
    }
}

pub(crate) fn source_output_spec(
    source_instance_id: SourceInstanceId,
    output_port: String,
    stream_id: StreamId,
) -> SourceOutputSpec {
    SourceOutputSpec {
        source_instance_id,
        output_port,
        stream_id,
    }
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

pub(crate) fn source_route_spec(
    route_id: RouteId,
    source_instance_id: SourceInstanceId,
    output_port: String,
    stream_id: StreamId,
    source_id: SourceId,
    endpoint_id: EndpointId,
    endpoint_input_port: Option<String>,
) -> SourceRouteSpec {
    SourceRouteSpec {
        route_id,
        source_instance_id,
        output_port,
        stream_id,
        source_id,
        endpoint_id,
        endpoint_input_port,
    }
}

pub(crate) fn operator_spec(
    instance_id: OperatorInstanceId,
    operator_id: OperatorId,
    configuration: OperatorConfiguration,
) -> OperatorInstanceSpec {
    OperatorInstanceSpec {
        instance_id,
        operator_id,
        configuration,
    }
}

pub(crate) fn operator_connection_spec(
    route_id: RouteId,
    operator_instance_id: OperatorInstanceId,
    input_origin: OperatorInputOrigin,
    input_port: Option<String>,
) -> OperatorConnectionSpec {
    OperatorConnectionSpec {
        route_id,
        operator_instance_id,
        input_origin,
        input_port,
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
            source_instances: Vec::new(),
            source_outputs: Vec::new(),
            endpoints: vec![endpoint_spec(
                EndpointId(1),
                NodeTypeId::from("endpoint.test"),
                OperatorId::new("example.endpoint.test.v1"),
                EndpointConfiguration::new(),
                None,
            )],
            routes: vec![route_spec(RouteId(1), StemId(1), EndpointId(1))],
            source_routes: Vec::new(),
            operators: Vec::new(),
            operator_connections: Vec::new(),
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
            Err(SessionError::UnsupportedVersion { major: 1, minor: 5 })
        ));
    }

    #[test]
    fn given_current_schema_when_version_read_then_derived_route_extension_is_recorded() {
        assert_eq!(SESSION_SPEC_VERSION, SessionSpecVersion::new(1, 4));
    }

    #[test]
    fn given_legacy_version_with_derived_fields_when_validated_then_schema_lie_is_rejected() {
        let mut spec = valid_legacy_compatible_spec();
        spec.operators.push(operator_spec(
            OperatorInstanceId::new(1),
            OperatorId::new("example.operator.test.v1"),
            OperatorConfiguration::new(),
        ));
        spec.operator_connections.push(operator_connection_spec(
            RouteId(2),
            OperatorInstanceId::new(1),
            OperatorInputOrigin::Stem(StemId(1)),
            None,
        ));

        #[allow(deprecated)]
        let compatibility_alias: &OperatorSpec = &spec.operators[0];
        assert_eq!(
            compatibility_alias.instance_id(),
            OperatorInstanceId::new(1)
        );

        assert!(matches!(
            spec.validate(),
            Err(SessionError::InvalidOperator { .. })
        ));
    }
}

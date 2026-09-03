use std::collections::HashSet;

use crate::frame::{EndpointId, RouteId, SessionId, SourceId, StemId, StreamId};
use crate::graph::NodeTypeId;

use crate::session::{
    EndpointConfiguration, OperatorConfiguration, OperatorId, SessionError, Source,
    SourceConfiguration, SourceTypeId,
};

pub const SESSION_SPEC_VERSION: SessionSpecVersion = SessionSpecVersion::new(1, 5);

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

    #[cfg(any(test, feature = "internal-testing"))]
    pub const fn major(self) -> u16 {
        self.major
    }

    #[cfg(any(test, feature = "internal-testing"))]
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

/// One operator PCM output that re-enters the specialized Session audio lane.
///
/// The declaration carries only engine identities. The compiler validates the
/// selected operator port against concrete PCM media before a runtime can be
/// prepared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedAudioIngressSpec {
    stem_id: StemId,
    operator_instance_id: OperatorInstanceId,
    output_port: Option<String>,
    source_id: SourceId,
    stream_id: StreamId,
}

impl GeneratedAudioIngressSpec {
    pub const fn stem_id(&self) -> StemId {
        self.stem_id
    }

    pub const fn operator_instance_id(&self) -> OperatorInstanceId {
        self.operator_instance_id
    }

    pub fn output_port(&self) -> Option<&str> {
        self.output_port.as_deref()
    }

    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    pub const fn stream_id(&self) -> StreamId {
        self.stream_id
    }
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
    connector_id: Option<crate::frame::ConnectorId>,
    node_type_id: NodeTypeId,
    operator_id: OperatorId,
    configuration: EndpointConfiguration,
    route_settings: Option<crate::graph::RouteSettings>,
}

impl EndpointSpec {
    pub const fn id(&self) -> EndpointId {
        self.endpoint_id
    }

    pub const fn connector_id(&self) -> Option<crate::frame::ConnectorId> {
        self.connector_id
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

    pub const fn route_settings(&self) -> Option<crate::graph::RouteSettings> {
        self.route_settings
    }
}

#[derive(Debug, Clone)]
pub struct OperatorInstanceSpec {
    instance_id: OperatorInstanceId,
    operator_id: OperatorId,
    configuration: OperatorConfiguration,
}

/// Stable origin of a declared Session stream.
///
/// The variants describe engine topology only. They do not encode provider,
/// endpoint, transport, or customer product categories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamOrigin {
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

/// Stable destination of a declared Session connection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionTarget {
    OperatorInput {
        operator_instance_id: OperatorInstanceId,
        input_port: Option<String>,
    },
    EndpointInput {
        endpoint_id: EndpointId,
        input_port: Option<String>,
    },
}

/// The single Session connection declaration used for every stream origin and
/// every operator/endpoint destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionSpec {
    route_id: RouteId,
    origin: StreamOrigin,
    target: ConnectionTarget,
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

impl ConnectionSpec {
    pub const fn id(&self) -> RouteId {
        self.route_id
    }

    pub const fn origin(&self) -> &StreamOrigin {
        &self.origin
    }

    pub const fn target(&self) -> &ConnectionTarget {
        &self.target
    }
}

#[derive(Debug, Clone)]
pub struct SessionSpec {
    version: SessionSpecVersion,
    session_id: SessionId,
    stems: Vec<StemSpec>,
    source_instances: Vec<SourceInstanceSpec>,
    source_outputs: Vec<SourceOutputSpec>,
    generated_audio_ingresses: Vec<GeneratedAudioIngressSpec>,
    endpoints: Vec<EndpointSpec>,
    operators: Vec<OperatorInstanceSpec>,
    connections: Vec<ConnectionSpec>,
}

pub(crate) struct SessionSpecDeclarations {
    pub(crate) stems: Vec<StemSpec>,
    pub(crate) source_instances: Vec<SourceInstanceSpec>,
    pub(crate) source_outputs: Vec<SourceOutputSpec>,
    pub(crate) generated_audio_ingresses: Vec<GeneratedAudioIngressSpec>,
    pub(crate) endpoints: Vec<EndpointSpec>,
    pub(crate) operators: Vec<OperatorInstanceSpec>,
    pub(crate) connections: Vec<ConnectionSpec>,
}

impl SessionSpec {
    pub(crate) fn new(session_id: SessionId, declarations: SessionSpecDeclarations) -> Self {
        Self {
            version: SESSION_SPEC_VERSION,
            session_id,
            stems: declarations.stems,
            source_instances: declarations.source_instances,
            source_outputs: declarations.source_outputs,
            generated_audio_ingresses: declarations.generated_audio_ingresses,
            endpoints: declarations.endpoints,
            operators: declarations.operators,
            connections: declarations.connections,
        }
    }

    #[cfg(any(test, feature = "internal-testing"))]
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

    pub fn generated_audio_ingresses(&self) -> &[GeneratedAudioIngressSpec] {
        &self.generated_audio_ingresses
    }

    pub fn endpoints(&self) -> &[EndpointSpec] {
        &self.endpoints
    }

    pub fn operators(&self) -> &[OperatorInstanceSpec] {
        &self.operators
    }

    pub fn connections(&self) -> &[ConnectionSpec] {
        &self.connections
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
        if self.stems.is_empty()
            && self.source_instances.is_empty()
            && self.generated_audio_ingresses.is_empty()
        {
            return Err(SessionError::NoSources);
        }
        if self.version.minor < 1
            && (!self.operators.is_empty()
                || self.connections.iter().any(|connection| {
                    matches!(connection.origin, StreamOrigin::OperatorOutput { .. })
                        || matches!(connection.target, ConnectionTarget::OperatorInput { .. })
                }))
        {
            return Err(SessionError::InvalidOperator {
                reason: "derived operators require Session schema version 1.1 or newer".to_owned(),
            });
        }
        if self.version.minor < 3
            && (!self.source_instances.is_empty()
                || !self.source_outputs.is_empty()
                || self.connections.iter().any(|connection| {
                    matches!(connection.origin, StreamOrigin::SourceOutput { .. })
                }))
        {
            return Err(SessionError::InvalidRoute {
                reason: "external sources require Session schema version 1.3 or newer".to_owned(),
            });
        }
        if self.version.minor < 4
            && self.connections.iter().any(|connection| {
                matches!(connection.target, ConnectionTarget::OperatorInput { .. })
            })
        {
            return Err(SessionError::InvalidOperator {
                reason: "named operator connections require Session schema version 1.4 or newer"
                    .to_owned(),
            });
        }
        if self.version.minor < 5 && !self.generated_audio_ingresses.is_empty() {
            return Err(SessionError::InvalidRoute {
                reason: "generated audio reentry requires Session schema version 1.5 or newer"
                    .to_owned(),
            });
        }

        let mut stem_ids = HashSet::with_capacity(
            self.stems
                .len()
                .saturating_add(self.generated_audio_ingresses.len()),
        );
        for stem in &self.stems {
            if !stem_ids.insert(stem.stem_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate stem id {:?}", stem.stem_id),
                });
            }
            stem.source.validate()?;
            if !self.connections.iter().any(
                |connection| matches!(connection.origin, StreamOrigin::Stem(stem_id) if stem_id == stem.stem_id),
            )
            {
                return Err(SessionError::NoRoutes {
                    stem_id: stem.stem_id,
                });
            }
        }

        for ingress in &self.generated_audio_ingresses {
            if !stem_ids.insert(ingress.stem_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate stem id {:?}", ingress.stem_id),
                });
            }
            if ingress
                .output_port
                .as_deref()
                .is_some_and(|port| port.trim().is_empty())
            {
                return Err(SessionError::InvalidOperator {
                    reason: "generated audio output port cannot be empty".to_owned(),
                });
            }
            if !self.connections.iter().any(
                |connection| matches!(connection.origin, StreamOrigin::Stem(stem_id) if stem_id == ingress.stem_id),
            )
            {
                return Err(SessionError::NoRoutes {
                    stem_id: ingress.stem_id,
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
            let routed = self.connections.iter().any(|connection| {
                matches!(
                    &connection.origin,
                    StreamOrigin::SourceOutput {
                        source_instance_id,
                        output_port,
                        ..
                    } if *source_instance_id == output.source_instance_id
                        && output_port == &output.output_port
                )
            });
            if !routed {
                return Err(SessionError::NoSourceOutputRoutes {
                    source_instance_id: output.source_instance_id,
                    output_port: output.output_port.clone(),
                });
            }
        }

        for ingress in &self.generated_audio_ingresses {
            if !source_ids.insert(ingress.source_id) || !stream_ids.insert(ingress.stream_id) {
                return Err(SessionError::InvalidRoute {
                    reason: "generated audio requires unique source and stream identities"
                        .to_owned(),
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
        for ingress in &self.generated_audio_ingresses {
            if !operator_instance_ids.contains(&ingress.operator_instance_id) {
                return Err(SessionError::UnknownOperatorInstance {
                    operator_instance_id: ingress.operator_instance_id,
                });
            }
        }

        let mut route_ids = HashSet::with_capacity(self.connections.len());
        for connection in &self.connections {
            if !route_ids.insert(connection.route_id) {
                return Err(SessionError::InvalidRoute {
                    reason: format!("duplicate route id {:?}", connection.route_id),
                });
            }
            match &connection.origin {
                StreamOrigin::Stem(stem_id) => {
                    if !stem_ids.contains(stem_id) {
                        return Err(SessionError::UnknownStem { stem_id: *stem_id });
                    }
                }
                StreamOrigin::SourceOutput {
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
                StreamOrigin::OperatorOutput {
                    operator_instance_id,
                    output_port,
                } => {
                    if !operator_instance_ids.contains(operator_instance_id) {
                        return Err(SessionError::UnknownOperatorInstance {
                            operator_instance_id: *operator_instance_id,
                        });
                    }
                    if output_port
                        .as_deref()
                        .is_some_and(|port| port.trim().is_empty())
                    {
                        return Err(SessionError::InvalidOperator {
                            reason: "operator output port cannot be empty".to_owned(),
                        });
                    }
                }
            }
            match &connection.target {
                ConnectionTarget::EndpointInput {
                    endpoint_id,
                    input_port,
                } => {
                    if !endpoint_ids.contains(endpoint_id) {
                        return Err(SessionError::UnknownEndpoint {
                            endpoint_id: *endpoint_id,
                        });
                    }
                    if input_port
                        .as_deref()
                        .is_some_and(|port| port.trim().is_empty())
                    {
                        return Err(SessionError::InvalidRoute {
                            reason: "endpoint input port cannot be empty".to_owned(),
                        });
                    }
                }
                ConnectionTarget::OperatorInput {
                    operator_instance_id,
                    input_port,
                } => {
                    if !operator_instance_ids.contains(operator_instance_id) {
                        return Err(SessionError::UnknownOperatorInstance {
                            operator_instance_id: *operator_instance_id,
                        });
                    }
                    if input_port
                        .as_deref()
                        .is_some_and(|port| port.trim().is_empty())
                    {
                        return Err(SessionError::InvalidOperator {
                            reason: "operator input port cannot be empty".to_owned(),
                        });
                    }
                }
            }
        }
        for operator in &self.operators {
            if !self.connections.iter().any(|connection| {
                matches!(
                    connection.target,
                    ConnectionTarget::OperatorInput {
                        operator_instance_id,
                        ..
                    } if operator_instance_id == operator.instance_id
                )
            }) {
                return Err(SessionError::InvalidOperator {
                    reason: format!(
                        "operator instance {:?} has no input connection",
                        operator.instance_id
                    ),
                });
            }
        }

        for operator in &self.operators {
            if !self.connections.iter().any(|connection| {
                matches!(
                    connection.origin,
                    StreamOrigin::OperatorOutput {
                        operator_instance_id,
                        ..
                    } if operator_instance_id == operator.instance_id
                )
            }) && !self
                .generated_audio_ingresses
                .iter()
                .any(|ingress| ingress.operator_instance_id == operator.instance_id)
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

pub(crate) fn generated_audio_ingress_spec(
    stem_id: StemId,
    operator_instance_id: OperatorInstanceId,
    output_port: Option<String>,
    source_id: SourceId,
    stream_id: StreamId,
) -> GeneratedAudioIngressSpec {
    GeneratedAudioIngressSpec {
        stem_id,
        operator_instance_id,
        output_port,
        source_id,
        stream_id,
    }
}

pub(crate) fn endpoint_spec(
    endpoint_id: EndpointId,
    connector_id: Option<crate::frame::ConnectorId>,
    node_type_id: NodeTypeId,
    operator_id: OperatorId,
    configuration: EndpointConfiguration,
    route_settings: Option<crate::graph::RouteSettings>,
) -> EndpointSpec {
    EndpointSpec {
        endpoint_id,
        connector_id,
        node_type_id,
        operator_id,
        configuration,
        route_settings,
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

pub(crate) const fn connection_spec(
    route_id: RouteId,
    origin: StreamOrigin,
    target: ConnectionTarget,
) -> ConnectionSpec {
    ConnectionSpec {
        route_id,
        origin,
        target,
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
            generated_audio_ingresses: Vec::new(),
            endpoints: vec![endpoint_spec(
                EndpointId(1),
                None,
                NodeTypeId::from("endpoint.test"),
                OperatorId::new("example.endpoint.test.v1"),
                EndpointConfiguration::new(),
                None,
            )],
            operators: Vec::new(),
            connections: vec![connection_spec(
                RouteId(1),
                StreamOrigin::Stem(StemId(1)),
                ConnectionTarget::EndpointInput {
                    endpoint_id: EndpointId(1),
                    input_port: None,
                },
            )],
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
            Err(SessionError::UnsupportedVersion { major: 1, minor: 6 })
        ));
    }

    #[test]
    fn given_current_schema_when_version_read_then_derived_route_extension_is_recorded() {
        assert_eq!(SESSION_SPEC_VERSION, SessionSpecVersion::new(1, 5));
    }

    #[test]
    fn given_legacy_version_with_derived_fields_when_validated_then_schema_lie_is_rejected() {
        let mut spec = valid_legacy_compatible_spec();
        spec.operators.push(operator_spec(
            OperatorInstanceId::new(1),
            OperatorId::new("example.operator.test.v1"),
            OperatorConfiguration::new(),
        ));
        spec.connections.push(connection_spec(
            RouteId(2),
            StreamOrigin::Stem(StemId(1)),
            ConnectionTarget::OperatorInput {
                operator_instance_id: OperatorInstanceId::new(1),
                input_port: None,
            },
        ));

        assert!(matches!(
            spec.validate(),
            Err(SessionError::InvalidOperator { .. })
        ));
    }
}

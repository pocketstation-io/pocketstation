use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::frame::{ConnectorId, EndpointId, RouteId, SessionId, SourceId, StemId, StreamId};
use crate::graph::NodeTypeId;

use super::endpoint::BROWSER_RECEIVER_URI_CONFIGURATION_KEY;
use crate::session::declaration::spec::{
    connection_spec, endpoint_spec, generated_audio_ingress_spec, operator_spec,
    source_instance_spec, source_output_spec, stem_spec, SessionSpecDeclarations,
};
use crate::session::{
    ConnectionTarget, EndpointConfiguration, EndpointDescriptor, OperatorConfiguration, OperatorId,
    OperatorInstanceId, SessionError, SessionSpec, Source, SourceConfiguration, SourceInstanceId,
    SourceTypeId, StreamOrigin, BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID,
};

static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
struct StemDraft {
    stem_id: StemId,
    source: Source,
}

#[derive(Debug)]
struct SourceInstanceDraft {
    instance_id: SourceInstanceId,
    source_id: SourceId,
    source_type_id: SourceTypeId,
    configuration: SourceConfiguration,
}

#[derive(Debug)]
struct SourceOutputDraft {
    source_instance_id: SourceInstanceId,
    output_port: String,
    stream_id: StreamId,
}

#[derive(Debug)]
struct EndpointDraft {
    endpoint_id: EndpointId,
    connector_id: Option<ConnectorId>,
    descriptor: EndpointDescriptor,
}

#[derive(Debug)]
struct OperatorDraft {
    instance_id: OperatorInstanceId,
    operator: Operator,
}

#[derive(Debug)]
struct ConnectionDraft {
    route_id: RouteId,
    origin: StreamOrigin,
    target: ConnectionTarget,
}

#[derive(Debug)]
struct GeneratedAudioIngressDraft {
    stem_id: StemId,
    operator_instance_id: OperatorInstanceId,
    output_port: Option<String>,
    source_id: SourceId,
    stream_id: StreamId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DraftStatus {
    Open,
    Frozen,
}

#[derive(Debug)]
struct SessionDraft {
    status: DraftStatus,
    next_stem_id: u64,
    next_endpoint_id: u64,
    next_connector_id: u64,
    next_route_id: u64,
    next_operator_instance_id: u64,
    next_source_instance_id: u64,
    next_external_source_id: u64,
    next_stream_id: u64,
    stems: Vec<StemDraft>,
    source_instances: Vec<SourceInstanceDraft>,
    source_outputs: Vec<SourceOutputDraft>,
    endpoints: Vec<EndpointDraft>,
    operators: Vec<OperatorDraft>,
    connections: Vec<ConnectionDraft>,
    generated_audio_ingresses: Vec<GeneratedAudioIngressDraft>,
}

impl Default for SessionDraft {
    fn default() -> Self {
        Self {
            status: DraftStatus::Open,
            next_stem_id: 0,
            next_endpoint_id: 0,
            next_connector_id: 0,
            next_route_id: 0,
            next_operator_instance_id: 0,
            next_source_instance_id: 0,
            next_external_source_id: 0,
            next_stream_id: 0,
            stems: Vec::new(),
            source_instances: Vec::new(),
            source_outputs: Vec::new(),
            endpoints: Vec::new(),
            operators: Vec::new(),
            connections: Vec::new(),
            generated_audio_ingresses: Vec::new(),
        }
    }
}

impl SessionDraft {
    fn ensure_open(&self, session_id: SessionId) -> Result<(), SessionError> {
        if self.status == DraftStatus::Open {
            Ok(())
        } else {
            Err(SessionError::DraftFrozen { session_id })
        }
    }

    fn allocate_stem_id(&mut self) -> Result<StemId, SessionError> {
        self.next_stem_id = self
            .next_stem_id
            .checked_add(1)
            .ok_or(SessionError::IdExhausted)?;
        Ok(StemId(self.next_stem_id))
    }

    fn allocate_endpoint_id(&mut self) -> Result<EndpointId, SessionError> {
        self.next_endpoint_id = self
            .next_endpoint_id
            .checked_add(1)
            .ok_or(SessionError::IdExhausted)?;
        Ok(EndpointId(self.next_endpoint_id))
    }

    fn allocate_connector_id(&mut self) -> Result<ConnectorId, SessionError> {
        self.next_connector_id = self
            .next_connector_id
            .checked_add(1)
            .ok_or(SessionError::IdExhausted)?;
        Ok(ConnectorId(self.next_connector_id))
    }

    fn allocate_route_id(&mut self) -> Result<RouteId, SessionError> {
        self.next_route_id = self
            .next_route_id
            .checked_add(1)
            .ok_or(SessionError::IdExhausted)?;
        Ok(RouteId(self.next_route_id))
    }

    fn allocate_operator_instance_id(&mut self) -> Result<OperatorInstanceId, SessionError> {
        self.next_operator_instance_id = self
            .next_operator_instance_id
            .checked_add(1)
            .ok_or(SessionError::IdExhausted)?;
        Ok(OperatorInstanceId::new(self.next_operator_instance_id))
    }

    fn allocate_source_instance_id(&mut self) -> Result<SourceInstanceId, SessionError> {
        self.next_source_instance_id = self
            .next_source_instance_id
            .checked_add(1)
            .ok_or(SessionError::IdExhausted)?;
        Ok(SourceInstanceId::new(self.next_source_instance_id))
    }

    fn allocate_external_source_id(&mut self) -> Result<SourceId, SessionError> {
        self.next_external_source_id = self
            .next_external_source_id
            .checked_add(1)
            .ok_or(SessionError::IdExhausted)?;
        Ok(SourceId(self.next_external_source_id))
    }

    fn allocate_stream_id(&mut self) -> Result<StreamId, SessionError> {
        self.next_stream_id = self
            .next_stream_id
            .checked_add(1)
            .ok_or(SessionError::IdExhausted)?;
        Ok(StreamId(self.next_stream_id))
    }

    fn declare_operator(&mut self, operator: Operator) -> Result<OperatorInstanceId, SessionError> {
        if operator.operator_id.as_str().trim().is_empty() {
            return Err(SessionError::InvalidOperator {
                reason: "operator id cannot be empty".to_owned(),
            });
        }
        let instance_id = self.allocate_operator_instance_id()?;
        self.operators.push(OperatorDraft {
            instance_id,
            operator,
        });
        Ok(instance_id)
    }

    fn connect_operator_input(
        &mut self,
        operator_instance_id: OperatorInstanceId,
        origin: StreamOrigin,
        input_port: Option<String>,
    ) -> Result<RouteId, SessionError> {
        if !self
            .operators
            .iter()
            .any(|operator| operator.instance_id == operator_instance_id)
        {
            return Err(SessionError::UnknownOperatorInstance {
                operator_instance_id,
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
        if let Some(input_port) = &input_port {
            if self.connections.iter().any(|connection| {
                matches!(
                    &connection.target,
                    ConnectionTarget::OperatorInput {
                        operator_instance_id: target_instance_id,
                        input_port: Some(target_port),
                    } if *target_instance_id == operator_instance_id && target_port == input_port
                )
            }) {
                return Err(SessionError::InvalidOperator {
                    reason: format!(
                        "operator instance {operator_instance_id:?} input port '{input_port}' is already connected"
                    ),
                });
            }
        }
        let route_id = self.allocate_route_id()?;
        self.connections.push(ConnectionDraft {
            route_id,
            origin,
            target: ConnectionTarget::OperatorInput {
                operator_instance_id,
                input_port,
            },
        });
        Ok(route_id)
    }

    fn connect_endpoint_input(
        &mut self,
        origin: StreamOrigin,
        endpoint_id: EndpointId,
        input_port: Option<String>,
    ) -> Result<RouteId, SessionError> {
        if !self
            .endpoints
            .iter()
            .any(|endpoint| endpoint.endpoint_id == endpoint_id)
        {
            return Err(SessionError::UnknownEndpoint { endpoint_id });
        }
        if input_port
            .as_deref()
            .is_some_and(|port| port.trim().is_empty())
        {
            return Err(SessionError::InvalidRoute {
                reason: "endpoint input port cannot be empty".to_owned(),
            });
        }
        let route_id = self.allocate_route_id()?;
        self.connections.push(ConnectionDraft {
            route_id,
            origin,
            target: ConnectionTarget::EndpointInput {
                endpoint_id,
                input_port,
            },
        });
        Ok(route_id)
    }
}

#[derive(Debug, Clone)]
#[doc = "Represents operator in the PocketStation API."]
pub struct Operator {
    operator_id: OperatorId,
    configuration: OperatorConfiguration,
}

impl Operator {
    #[doc = "Creates a new `Operator`."]
    pub fn new(operator_id: OperatorId, configuration: OperatorConfiguration) -> Self {
        Self {
            operator_id,
            configuration,
        }
    }

    #[doc = "Returns the operator identifier associated with `Operator`."]
    pub const fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

    #[doc = "Returns the configuration associated with `Operator`."]
    pub const fn configuration(&self) -> &OperatorConfiguration {
        &self.configuration
    }
}

#[derive(Debug)]
struct SessionShared {
    session_id: SessionId,
    draft: Mutex<SessionDraft>,
}

impl SessionShared {
    fn draft(&self) -> Result<MutexGuard<'_, SessionDraft>, SessionError> {
        self.draft.lock().map_err(|_| SessionError::DraftPoisoned)
    }
}

pub struct Session {
    shared: Arc<SessionShared>,
}

impl Session {
    pub fn new() -> Self {
        let session_id = SessionId(NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed));
        Self {
            shared: Arc::new(SessionShared {
                session_id,
                draft: Mutex::new(SessionDraft::default()),
            }),
        }
    }

    pub fn id(&self) -> SessionId {
        self.shared.session_id
    }

    pub fn capture(&self, source: Source) -> Result<StemHandle, SessionError> {
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.id())?;
        let stem_id = draft.allocate_stem_id()?;
        draft.stems.push(StemDraft { stem_id, source });
        Ok(StemHandle {
            stream: InternalStreamHandle::new(
                Arc::clone(&self.shared),
                self.id(),
                StreamOrigin::Stem(stem_id),
            ),
            stem_id,
        })
    }

    /// Declares one externally implemented source instance.
    ///
    /// The source type remains an open stable identifier. Output names are
    /// selected separately and resolved against the registered manifest when
    /// the Session is compiled by a `SessionEngine`.
    pub fn source(
        &self,
        source_type_id: SourceTypeId,
        configuration: SourceConfiguration,
    ) -> Result<SourceInstanceHandle, SessionError> {
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.id())?;
        let instance_id = draft.allocate_source_instance_id()?;
        let source_id = draft.allocate_external_source_id()?;
        draft.source_instances.push(SourceInstanceDraft {
            instance_id,
            source_id,
            source_type_id,
            configuration,
        });
        Ok(SourceInstanceHandle {
            shared: Arc::clone(&self.shared),
            session_id: self.id(),
            instance_id,
            source_id,
        })
    }

    /// Declares exactly one Session-owned operator instance.
    ///
    /// Inputs and outputs are selected separately by stable manifest port
    /// names. The compiler resolves and validates every connection before any
    /// runtime is started.
    pub fn operator(&self, operator: Operator) -> Result<OperatorInstanceHandle, SessionError> {
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.id())?;
        let instance_id = draft.declare_operator(operator)?;
        Ok(OperatorInstanceHandle {
            shared: Arc::clone(&self.shared),
            session_id: self.id(),
            instance_id,
        })
    }

    pub fn endpoint(&self, descriptor: EndpointDescriptor) -> Result<EndpointHandle, SessionError> {
        descriptor.validate()?;
        self.add_endpoint(descriptor, None)
    }

    pub fn connector(
        &self,
        operator_id: OperatorId,
        configuration: EndpointConfiguration,
    ) -> Result<EndpointHandle, SessionError> {
        let descriptor =
            EndpointDescriptor::new(NodeTypeId::from(CONNECTOR_NODE_TYPE_ID), operator_id)
                .with_configuration(configuration);
        self.connector_endpoint(descriptor)
    }

    pub(crate) fn connector_endpoint(
        &self,
        descriptor: EndpointDescriptor,
    ) -> Result<EndpointHandle, SessionError> {
        descriptor.validate()?;
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.id())?;
        let endpoint_id = draft.allocate_endpoint_id()?;
        let connector_id = draft.allocate_connector_id()?;
        draft.endpoints.push(EndpointDraft {
            endpoint_id,
            connector_id: Some(connector_id),
            descriptor,
        });
        Ok(EndpointHandle {
            session_id: self.id(),
            endpoint_id,
            connector_id: Some(connector_id),
        })
    }

    pub fn browser(&self, receiver_uri: impl Into<String>) -> Result<EndpointHandle, SessionError> {
        let descriptor = EndpointDescriptor::new(
            NodeTypeId::from(BROWSER_NODE_TYPE_ID),
            OperatorId::new(BROWSER_OPERATOR_ID),
        )
        .with_configuration(
            EndpointConfiguration::new().with(BROWSER_RECEIVER_URI_CONFIGURATION_KEY, receiver_uri),
        );
        descriptor.validate()?;
        self.add_endpoint(descriptor, None)
    }

    pub(crate) fn declares_endpoint_operator(
        &self,
        operator_id: &OperatorId,
    ) -> Result<bool, SessionError> {
        let draft = self.shared.draft()?;
        Ok(draft
            .endpoints
            .iter()
            .any(|endpoint| endpoint.descriptor.operator_id() == operator_id))
    }

    pub fn freeze(self) -> Result<SessionSpec, SessionError> {
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.id())?;
        draft.status = DraftStatus::Frozen;

        let declarations = SessionSpecDeclarations {
            stems: draft
                .stems
                .iter()
                .map(|stem| stem_spec(stem.stem_id, stem.source.clone()))
                .collect(),
            source_instances: draft
                .source_instances
                .iter()
                .map(|source| {
                    source_instance_spec(
                        source.instance_id,
                        source.source_id,
                        source.source_type_id.clone(),
                        source.configuration.clone(),
                    )
                })
                .collect(),
            source_outputs: draft
                .source_outputs
                .iter()
                .map(|output| {
                    source_output_spec(
                        output.source_instance_id,
                        output.output_port.clone(),
                        output.stream_id,
                    )
                })
                .collect(),
            generated_audio_ingresses: draft
                .generated_audio_ingresses
                .iter()
                .map(|ingress| {
                    generated_audio_ingress_spec(
                        ingress.stem_id,
                        ingress.operator_instance_id,
                        ingress.output_port.clone(),
                        ingress.source_id,
                        ingress.stream_id,
                    )
                })
                .collect(),
            endpoints: draft
                .endpoints
                .iter()
                .map(|endpoint| {
                    endpoint_spec(
                        endpoint.endpoint_id,
                        endpoint.connector_id,
                        endpoint.descriptor.node_type_id().clone(),
                        endpoint.descriptor.operator_id().clone(),
                        endpoint.descriptor.configuration().clone(),
                        endpoint.descriptor.input_edge(),
                    )
                })
                .collect(),
            operators: draft
                .operators
                .iter()
                .map(|operator| {
                    operator_spec(
                        operator.instance_id,
                        operator.operator.operator_id.clone(),
                        operator.operator.configuration.clone(),
                    )
                })
                .collect(),
            connections: draft
                .connections
                .iter()
                .map(|connection| {
                    connection_spec(
                        connection.route_id,
                        connection.origin.clone(),
                        connection.target.clone(),
                    )
                })
                .collect(),
        };
        let spec = SessionSpec::new(self.id(), declarations);
        spec.validate()?;
        Ok(spec)
    }

    fn add_endpoint(
        &self,
        descriptor: EndpointDescriptor,
        connector_id: Option<ConnectorId>,
    ) -> Result<EndpointHandle, SessionError> {
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.id())?;
        let endpoint_id = draft.allocate_endpoint_id()?;
        draft.endpoints.push(EndpointDraft {
            endpoint_id,
            connector_id,
            descriptor,
        });
        Ok(EndpointHandle {
            session_id: self.id(),
            endpoint_id,
            connector_id,
        })
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Session {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Session")
            .field("session_id", &self.id())
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Owns bounded access to endpoint."]
pub struct EndpointHandle {
    session_id: SessionId,
    endpoint_id: EndpointId,
    connector_id: Option<ConnectorId>,
}

impl EndpointHandle {
    #[doc = "Returns the session identifier associated with `EndpointHandle`."]
    pub const fn session_id(self) -> SessionId {
        self.session_id
    }

    #[doc = "Returns the id associated with `EndpointHandle`."]
    pub const fn id(self) -> EndpointId {
        self.endpoint_id
    }

    #[doc = "Returns the connector identifier associated with `EndpointHandle`."]
    pub const fn connector_id(self) -> Option<ConnectorId> {
        self.connector_id
    }
}

#[derive(Clone)]
pub(super) struct InternalStreamHandle {
    shared: Arc<SessionShared>,
    session_id: SessionId,
    origin: StreamOrigin,
}

impl InternalStreamHandle {
    fn new(shared: Arc<SessionShared>, session_id: SessionId, origin: StreamOrigin) -> Self {
        Self {
            shared,
            session_id,
            origin,
        }
    }

    pub(super) fn send_to(
        &self,
        endpoint: EndpointHandle,
        input_port: Option<String>,
    ) -> Result<RouteId, SessionError> {
        if endpoint.session_id != self.session_id {
            return Err(SessionError::ForeignEndpoint {
                expected: self.session_id,
                actual: endpoint.session_id,
            });
        }
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.session_id)?;
        draft.connect_endpoint_input(self.origin.clone(), endpoint.endpoint_id, input_port)
    }

    pub(super) fn declare_endpoint_and_send(
        &self,
        descriptor: EndpointDescriptor,
    ) -> Result<EndpointHandle, SessionError> {
        descriptor.validate()?;
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.session_id)?;
        let endpoint_id = draft.allocate_endpoint_id()?;
        draft.endpoints.push(EndpointDraft {
            endpoint_id,
            connector_id: None,
            descriptor,
        });
        draft.connect_endpoint_input(self.origin.clone(), endpoint_id, None)?;
        Ok(EndpointHandle {
            session_id: self.session_id,
            endpoint_id,
            connector_id: None,
        })
    }

    fn connect(&self, input: OperatorInputHandle) -> Result<RouteId, SessionError> {
        if input.session_id != self.session_id || !Arc::ptr_eq(&input.shared, &self.shared) {
            return Err(SessionError::InvalidOperator {
                reason: "operator input belongs to a different Session".to_owned(),
            });
        }
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.session_id)?;
        draft.connect_operator_input(
            input.operator_instance_id,
            self.origin.clone(),
            Some(input.port_name),
        )
    }

    pub(super) fn through_ports(
        &self,
        operator: Operator,
        input_port: Option<String>,
        output_port: Option<String>,
    ) -> Result<DerivedStreamHandle, SessionError> {
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.session_id)?;
        let instance_id = draft.declare_operator(operator)?;
        draft.connect_operator_input(instance_id, self.origin.clone(), input_port)?;
        Ok(DerivedStreamHandle::new(
            Arc::clone(&self.shared),
            self.session_id,
            instance_id,
            output_port,
        ))
    }
}

#[derive(Clone)]
#[doc = "Owns bounded access to stem."]
pub struct StemHandle {
    pub(super) stream: InternalStreamHandle,
    stem_id: StemId,
}

#[derive(Clone)]
#[doc = "Owns bounded access to operator instance."]
pub struct OperatorInstanceHandle {
    shared: Arc<SessionShared>,
    session_id: SessionId,
    instance_id: OperatorInstanceId,
}

#[derive(Clone)]
#[doc = "Owns bounded access to operator input."]
pub struct OperatorInputHandle {
    shared: Arc<SessionShared>,
    session_id: SessionId,
    operator_instance_id: OperatorInstanceId,
    port_name: String,
}

impl OperatorInstanceHandle {
    #[doc = "Returns the session identifier associated with `OperatorInstanceHandle`."]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    #[doc = "Returns the instance identifier associated with `OperatorInstanceHandle`."]
    pub const fn instance_id(&self) -> OperatorInstanceId {
        self.instance_id
    }

    #[doc = "Returns the input associated with `OperatorInstanceHandle`."]
    pub fn input(&self, port_name: impl Into<String>) -> Result<OperatorInputHandle, SessionError> {
        let port_name = port_name.into();
        if port_name.trim().is_empty() {
            return Err(SessionError::InvalidOperator {
                reason: "operator input port cannot be empty".to_owned(),
            });
        }
        Ok(OperatorInputHandle {
            shared: Arc::clone(&self.shared),
            session_id: self.session_id,
            operator_instance_id: self.instance_id,
            port_name,
        })
    }

    #[doc = "Returns the output associated with `OperatorInstanceHandle`."]
    pub fn output(
        &self,
        port_name: impl Into<String>,
    ) -> Result<DerivedStreamHandle, SessionError> {
        let port_name = port_name.into();
        if port_name.trim().is_empty() {
            return Err(SessionError::InvalidOperator {
                reason: "operator output port cannot be empty".to_owned(),
            });
        }
        Ok(DerivedStreamHandle {
            stream: InternalStreamHandle::new(
                Arc::clone(&self.shared),
                self.session_id,
                StreamOrigin::OperatorOutput {
                    operator_instance_id: self.instance_id,
                    output_port: Some(port_name.clone()),
                },
            ),
            operator_instance_id: self.instance_id,
            output_port: Some(port_name),
        })
    }
}

impl fmt::Debug for OperatorInstanceHandle {
    #[doc = "Formats `OperatorInstanceHandle` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperatorInstanceHandle")
            .field("session_id", &self.session_id)
            .field("instance_id", &self.instance_id)
            .finish()
    }
}

impl fmt::Debug for OperatorInputHandle {
    #[doc = "Formats `OperatorInputHandle` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperatorInputHandle")
            .field("session_id", &self.session_id)
            .field("operator_instance_id", &self.operator_instance_id)
            .field("port_name", &self.port_name)
            .finish()
    }
}

impl StemHandle {
    #[doc = "Returns the session identifier associated with `StemHandle`."]
    pub const fn session_id(&self) -> SessionId {
        self.stream.session_id
    }

    #[doc = "Returns the id associated with `StemHandle`."]
    pub const fn id(&self) -> StemId {
        self.stem_id
    }

    #[doc = "Sends a value through `StemHandle`."]
    pub fn send(&self, endpoint: EndpointHandle) -> Result<RouteId, SessionError> {
        self.stream.send_to(endpoint, None)
    }

    /// Connects this stream to one explicit endpoint input port.
    pub fn send_to(
        &self,
        endpoint: EndpointHandle,
        endpoint_input_port: impl Into<Option<String>>,
    ) -> Result<RouteId, SessionError> {
        self.stream.send_to(endpoint, endpoint_input_port.into())
    }

    pub(crate) fn declare_endpoint_and_send(
        &self,
        descriptor: EndpointDescriptor,
    ) -> Result<EndpointHandle, SessionError> {
        self.stream.declare_endpoint_and_send(descriptor)
    }

    #[doc = "Connects the requested ports through `StemHandle`."]
    pub fn connect(&self, input: OperatorInputHandle) -> Result<RouteId, SessionError> {
        self.stream.connect(input)
    }

    #[doc = "Routes the current stream through a declared operator using `StemHandle`."]
    pub fn through(&self, operator: Operator) -> Result<DerivedStreamHandle, SessionError> {
        self.through_ports(operator, None::<String>, None::<String>)
    }

    #[doc = "Returns the through ports associated with `StemHandle`."]
    pub fn through_ports(
        &self,
        operator: Operator,
        input_port: impl Into<Option<String>>,
        output_port: impl Into<Option<String>>,
    ) -> Result<DerivedStreamHandle, SessionError> {
        self.stream
            .through_ports(operator, input_port.into(), output_port.into())
    }
}

#[derive(Clone)]
#[doc = "Owns bounded access to derived stream."]
pub struct DerivedStreamHandle {
    pub(super) stream: InternalStreamHandle,
    operator_instance_id: OperatorInstanceId,
    output_port: Option<String>,
}

#[derive(Clone)]
#[doc = "Owns bounded access to source instance."]
pub struct SourceInstanceHandle {
    shared: Arc<SessionShared>,
    session_id: SessionId,
    instance_id: SourceInstanceId,
    source_id: SourceId,
}

impl SourceInstanceHandle {
    #[doc = "Returns the session identifier associated with `SourceInstanceHandle`."]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    #[doc = "Returns the instance identifier associated with `SourceInstanceHandle`."]
    pub const fn instance_id(&self) -> SourceInstanceId {
        self.instance_id
    }

    #[doc = "Returns the source identifier associated with `SourceInstanceHandle`."]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    #[doc = "Returns the output associated with `SourceInstanceHandle`."]
    pub fn output(
        &self,
        output_port: impl Into<String>,
    ) -> Result<SourceOutputHandle, SessionError> {
        let output_port = output_port.into();
        if output_port.trim().is_empty() {
            return Err(SessionError::InvalidRoute {
                reason: "external source output port cannot be empty".to_owned(),
            });
        }
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.session_id)?;
        let stream_id = if let Some(existing) = draft.source_outputs.iter().find(|output| {
            output.source_instance_id == self.instance_id && output.output_port == output_port
        }) {
            existing.stream_id
        } else {
            let stream_id = draft.allocate_stream_id()?;
            draft.source_outputs.push(SourceOutputDraft {
                source_instance_id: self.instance_id,
                output_port: output_port.clone(),
                stream_id,
            });
            stream_id
        };
        Ok(SourceOutputHandle {
            stream: InternalStreamHandle::new(
                Arc::clone(&self.shared),
                self.session_id,
                StreamOrigin::SourceOutput {
                    source_instance_id: self.instance_id,
                    output_port: output_port.clone(),
                    stream_id,
                    source_id: self.source_id,
                },
            ),
            source_instance_id: self.instance_id,
            source_id: self.source_id,
            stream_id,
            output_port,
        })
    }
}

impl fmt::Debug for SourceInstanceHandle {
    #[doc = "Formats `SourceInstanceHandle` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SourceInstanceHandle")
            .field("session_id", &self.session_id)
            .field("instance_id", &self.instance_id)
            .field("source_id", &self.source_id)
            .finish()
    }
}

#[derive(Clone)]
#[doc = "Owns bounded access to source output."]
pub struct SourceOutputHandle {
    pub(super) stream: InternalStreamHandle,
    source_instance_id: SourceInstanceId,
    source_id: SourceId,
    stream_id: StreamId,
    output_port: String,
}

impl SourceOutputHandle {
    #[doc = "Returns the session identifier associated with `SourceOutputHandle`."]
    pub const fn session_id(&self) -> SessionId {
        self.stream.session_id
    }

    #[doc = "Returns the source instance identifier associated with `SourceOutputHandle`."]
    pub const fn source_instance_id(&self) -> SourceInstanceId {
        self.source_instance_id
    }

    #[doc = "Returns the source identifier associated with `SourceOutputHandle`."]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    #[doc = "Returns the stream identifier associated with `SourceOutputHandle`."]
    pub const fn stream_id(&self) -> StreamId {
        self.stream_id
    }

    #[doc = "Returns the output port associated with `SourceOutputHandle`."]
    pub fn output_port(&self) -> &str {
        &self.output_port
    }

    #[doc = "Sends a value through `SourceOutputHandle`."]
    pub fn send(&self, endpoint: EndpointHandle) -> Result<RouteId, SessionError> {
        self.send_to(endpoint, None::<String>)
    }

    #[doc = "Connects the requested ports through `SourceOutputHandle`."]
    pub fn connect(&self, input: OperatorInputHandle) -> Result<RouteId, SessionError> {
        self.stream.connect(input)
    }

    #[doc = "Routes the current source output to the requested destination through `SourceOutputHandle`."]
    pub fn send_to(
        &self,
        endpoint: EndpointHandle,
        endpoint_input_port: impl Into<Option<String>>,
    ) -> Result<RouteId, SessionError> {
        let endpoint_input_port = endpoint_input_port.into();
        self.stream.send_to(endpoint, endpoint_input_port)
    }

    pub(crate) fn declare_endpoint_and_send(
        &self,
        descriptor: EndpointDescriptor,
    ) -> Result<EndpointHandle, SessionError> {
        self.stream.declare_endpoint_and_send(descriptor)
    }

    #[doc = "Routes the current stream through a declared operator using `SourceOutputHandle`."]
    pub fn through(&self, operator: Operator) -> Result<DerivedStreamHandle, SessionError> {
        self.through_ports(operator, None::<String>, None::<String>)
    }

    #[doc = "Returns the through ports associated with `SourceOutputHandle`."]
    pub fn through_ports(
        &self,
        operator: Operator,
        input_port: impl Into<Option<String>>,
        output_port: impl Into<Option<String>>,
    ) -> Result<DerivedStreamHandle, SessionError> {
        self.stream
            .through_ports(operator, input_port.into(), output_port.into())
    }
}

impl fmt::Debug for SourceOutputHandle {
    #[doc = "Formats `SourceOutputHandle` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SourceOutputHandle")
            .field("session_id", &self.stream.session_id)
            .field("source_instance_id", &self.source_instance_id)
            .field("source_id", &self.source_id)
            .field("stream_id", &self.stream_id)
            .field("output_port", &self.output_port)
            .finish()
    }
}

impl DerivedStreamHandle {
    fn new(
        shared: Arc<SessionShared>,
        session_id: SessionId,
        operator_instance_id: OperatorInstanceId,
        output_port: Option<String>,
    ) -> Self {
        Self {
            stream: InternalStreamHandle::new(
                shared,
                session_id,
                StreamOrigin::OperatorOutput {
                    operator_instance_id,
                    output_port: output_port.clone(),
                },
            ),
            operator_instance_id,
            output_port,
        }
    }

    #[doc = "Returns the session identifier associated with `DerivedStreamHandle`."]
    pub const fn session_id(&self) -> SessionId {
        self.stream.session_id
    }

    #[doc = "Returns the operator instance identifier associated with `DerivedStreamHandle`."]
    pub const fn operator_instance_id(&self) -> OperatorInstanceId {
        self.operator_instance_id
    }

    #[doc = "Returns the output port associated with `DerivedStreamHandle`."]
    pub fn output_port(&self) -> Option<&str> {
        self.output_port.as_deref()
    }

    #[doc = "Returns the output associated with `DerivedStreamHandle`."]
    pub fn output(&self, port_name: impl Into<String>) -> Result<Self, SessionError> {
        let port_name = port_name.into();
        if port_name.trim().is_empty() {
            return Err(SessionError::InvalidOperator {
                reason: "operator output port cannot be empty".to_owned(),
            });
        }
        Ok(Self::new(
            Arc::clone(&self.stream.shared),
            self.stream.session_id,
            self.operator_instance_id,
            Some(port_name),
        ))
    }

    #[doc = "Connects the requested ports through `DerivedStreamHandle`."]
    pub fn connect(&self, input: OperatorInputHandle) -> Result<RouteId, SessionError> {
        self.stream.connect(input)
    }

    #[doc = "Routes the current stream through a declared operator using `DerivedStreamHandle`."]
    pub fn through(&self, operator: Operator) -> Result<DerivedStreamHandle, SessionError> {
        self.through_ports(operator, None::<String>, None::<String>)
    }

    #[doc = "Returns the through ports associated with `DerivedStreamHandle`."]
    pub fn through_ports(
        &self,
        operator: Operator,
        input_port: impl Into<Option<String>>,
        output_port: impl Into<Option<String>>,
    ) -> Result<DerivedStreamHandle, SessionError> {
        self.stream
            .through_ports(operator, input_port.into(), output_port.into())
    }

    #[doc = "Sends a value through `DerivedStreamHandle`."]
    pub fn send(&self, endpoint: EndpointHandle) -> Result<RouteId, SessionError> {
        self.stream.send_to(endpoint, None)
    }

    /// Connects this derived output to one explicit endpoint input port.
    pub fn send_to(
        &self,
        endpoint: EndpointHandle,
        endpoint_input_port: impl Into<Option<String>>,
    ) -> Result<RouteId, SessionError> {
        self.stream.send_to(endpoint, endpoint_input_port.into())
    }

    /// Re-enters this operator output into the Session's specialized audio lane.
    ///
    /// Compilation rejects outputs that are not concrete PCM. The returned stem
    /// uses the ordinary bounded audio routing and operator APIs; no async or
    /// foreign callback executes on the realtime executor.
    pub fn reenter_audio(&self) -> Result<StemHandle, SessionError> {
        let mut draft = self.stream.shared.draft()?;
        draft.ensure_open(self.stream.session_id)?;
        let stem_id = draft.allocate_stem_id()?;
        let source_id = draft.allocate_external_source_id()?;
        let stream_id = draft.allocate_stream_id()?;
        draft
            .generated_audio_ingresses
            .push(GeneratedAudioIngressDraft {
                stem_id,
                operator_instance_id: self.operator_instance_id,
                output_port: self.output_port.clone(),
                source_id,
                stream_id,
            });
        Ok(StemHandle {
            stream: InternalStreamHandle::new(
                Arc::clone(&self.stream.shared),
                self.stream.session_id,
                StreamOrigin::Stem(stem_id),
            ),
            stem_id,
        })
    }
}

impl fmt::Debug for DerivedStreamHandle {
    #[doc = "Formats `DerivedStreamHandle` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DerivedStreamHandle")
            .field("session_id", &self.stream.session_id)
            .field("operator_instance_id", &self.operator_instance_id)
            .field("output_port", &self.output_port)
            .finish()
    }
}

impl fmt::Debug for StemHandle {
    #[doc = "Formats `StemHandle` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StemHandle")
            .field("session_id", &self.stream.session_id)
            .field("stem_id", &self.stem_id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{
        ApplicationSelector, DeviceSelector, DEFAULT_MULTISTEM_RECORDING_GROUP_ID,
        RECORDER_OPERATOR_ID, RECORDING_GROUP_CONFIGURATION_KEY,
    };

    fn proof_draft() -> (Session, StemHandle, StemHandle, EndpointHandle) {
        let session = Session::new();
        let application = session
            .capture(Source::application(ApplicationSelector::name(
                "Meeting App",
            )))
            .expect("application declaration");
        let microphone = session
            .capture(Source::microphone(DeviceSelector::default()))
            .expect("microphone declaration");
        let browser = session
            .browser("wss://receiver.example.test")
            .expect("browser declaration");
        (session, application, microphone, browser)
    }

    #[test]
    fn given_two_stems_when_sent_to_one_endpoint_then_routes_are_distinct() {
        let (session, application, microphone, browser) = proof_draft();
        let application_route_id = application.send(browser).expect("application route");
        let microphone_route_id = microphone.send(browser).expect("microphone route");

        assert_ne!(application_route_id, microphone_route_id);
        assert_eq!(session.id(), browser.session_id());
    }

    #[test]
    fn given_connector_endpoint_when_declared_then_allocated_identity_is_exposed() {
        let session = Session::new();
        let connector = session
            .connector(
                OperatorId::new("example.connector.test.v1"),
                EndpointConfiguration::new(),
            )
            .expect("connector declaration");

        assert_eq!(connector.connector_id(), Some(ConnectorId(1)));
    }

    #[test]
    fn given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec() {
        let session = Session::new();
        let microphone = session
            .capture(Source::microphone(DeviceSelector::default()))
            .expect("microphone");
        let first = microphone
            .through(Operator::new(
                OperatorId::new("example.operator.first.v1"),
                OperatorConfiguration::new(),
            ))
            .expect("first operator");
        let second = first
            .through(Operator::new(
                OperatorId::new("example.operator.second.v1"),
                OperatorConfiguration::new(),
            ))
            .expect("second operator");
        let endpoint = session
            .endpoint(EndpointDescriptor::new(
                NodeTypeId::from("endpoint.derived.test"),
                OperatorId::new("example.endpoint.derived.v1"),
            ))
            .expect("endpoint");
        second.send(endpoint).expect("derived route");

        let specification = session.freeze().expect("valid chained specification");
        assert_eq!(specification.operators().len(), 2);
        let operator_inputs = specification
            .connections()
            .iter()
            .filter(|connection| {
                matches!(connection.target(), ConnectionTarget::OperatorInput { .. })
            })
            .collect::<Vec<_>>();
        assert_eq!(operator_inputs.len(), 2);
        assert!(matches!(
            operator_inputs[1].origin(),
            StreamOrigin::OperatorOutput {
                operator_instance_id,
                output_port: None,
            } if *operator_instance_id == specification.operators()[0].instance_id()
        ));
    }

    #[test]
    fn given_two_record_declarations_when_frozen_then_default_group_identity_is_explicit_and_stable(
    ) {
        let (session, application, microphone, _browser) = proof_draft();
        application.record("application").unwrap();
        microphone.record("microphone").unwrap();

        let spec = session.freeze().unwrap();
        let recording_endpoints = spec
            .endpoints()
            .iter()
            .filter(|endpoint| endpoint.operator_id().as_str() == RECORDER_OPERATOR_ID)
            .collect::<Vec<_>>();

        assert_eq!(recording_endpoints.len(), 2);
        assert!(recording_endpoints.iter().all(|endpoint| {
            endpoint
                .configuration()
                .get(RECORDING_GROUP_CONFIGURATION_KEY)
                == Some(DEFAULT_MULTISTEM_RECORDING_GROUP_ID)
        }));
    }

    #[test]
    fn given_foreign_endpoint_when_route_declared_then_error_is_immediate() {
        let first = Session::new();
        let second = Session::new();
        let stem = first
            .capture(Source::microphone_default())
            .expect("microphone declaration");
        let foreign = second
            .browser("wss://receiver.example.test")
            .expect("browser declaration");

        let result = stem.send(foreign);

        assert!(matches!(result, Err(SessionError::ForeignEndpoint { .. })));
    }

    #[test]
    fn given_cloned_stem_when_session_frozen_then_mutation_is_rejected() {
        let (session, application, microphone, browser) = proof_draft();
        let retained_application = application.clone();
        application.send(browser).expect("application route");
        microphone.send(browser).expect("microphone route");
        let spec = session.freeze().expect("valid frozen spec");

        let result = retained_application.record("late-recording");

        assert_eq!(spec.stems().len(), 2);
        assert_eq!(
            result,
            Err(SessionError::DraftFrozen {
                session_id: retained_application.session_id(),
            })
        );
    }

    #[test]
    fn given_stale_handle_after_freeze_when_route_declared_then_mutation_is_rejected() {
        let (session, application, microphone, browser) = proof_draft();
        application.send(browser).expect("application route");
        microphone.send(browser).expect("microphone route");
        session.freeze().expect("valid frozen spec");

        let result = application.send(browser);

        assert!(matches!(result, Err(SessionError::DraftFrozen { .. })));
    }

    #[test]
    fn given_empty_operator_id_when_endpoint_declared_then_descriptor_is_rejected() {
        let session = Session::new();
        let descriptor =
            EndpointDescriptor::new(NodeTypeId::from("endpoint.external"), OperatorId::new(" "));

        let result = session.endpoint(descriptor);

        assert!(matches!(result, Err(SessionError::InvalidEndpoint { .. })));
    }

    #[test]
    fn given_unrouted_stem_when_session_frozen_then_validation_fails_closed() {
        let session = Session::new();
        session
            .capture(Source::microphone_default())
            .expect("microphone declaration");

        let result = session.freeze();

        assert!(matches!(result, Err(SessionError::NoRoutes { .. })));
    }

    #[test]
    fn given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved() {
        let session = Session::new();
        let microphone = session
            .capture(Source::microphone_default())
            .expect("microphone declaration");
        let terminal = session
            .endpoint(EndpointDescriptor::new(
                NodeTypeId::from("endpoint.text.test"),
                OperatorId::new("example.text.terminal.v1"),
            ))
            .expect("terminal declaration");
        let derived = microphone
            .through(Operator::new(
                OperatorId::new("example.stt.v1"),
                OperatorConfiguration::new().with("language", "auto"),
            ))
            .expect("operator declaration");
        let output_route_id = derived.send(terminal).expect("derived route");

        let spec = session.freeze().expect("derived Session spec");

        assert_eq!(spec.operators().len(), 1);
        assert_eq!(
            spec.connections()
                .iter()
                .filter(|connection| {
                    matches!(connection.origin(), StreamOrigin::OperatorOutput { .. })
                        && matches!(connection.target(), ConnectionTarget::EndpointInput { .. })
                })
                .count(),
            1
        );
        assert_eq!(
            spec.operators()[0].instance_id(),
            derived.operator_instance_id()
        );
        let input_route_id = spec
            .connections()
            .iter()
            .find(|connection| {
                matches!(connection.target(), ConnectionTarget::OperatorInput { .. })
            })
            .expect("operator input")
            .id();
        assert_ne!(input_route_id, output_route_id);
        assert_eq!(
            spec.operators()[0].configuration().get("language"),
            Some("auto")
        );
    }

    #[test]
    fn given_derived_stream_without_destination_when_frozen_then_validation_fails_closed() {
        let session = Session::new();
        let microphone = session
            .capture(Source::microphone_default())
            .expect("microphone declaration");
        let _derived = microphone
            .through(Operator::new(
                OperatorId::new("example.stt.v1"),
                OperatorConfiguration::new(),
            ))
            .expect("operator declaration");

        let result = session.freeze();

        assert!(matches!(
            result,
            Err(SessionError::OperatorHasNoDestination { .. })
        ));
    }

    #[test]
    fn given_foreign_endpoint_when_derived_route_declared_then_error_is_immediate() {
        let first = Session::new();
        let second = Session::new();
        let microphone = first
            .capture(Source::microphone_default())
            .expect("microphone declaration");
        let derived = microphone
            .through(Operator::new(
                OperatorId::new("example.stt.v1"),
                OperatorConfiguration::new(),
            ))
            .expect("operator declaration");
        let foreign = second
            .endpoint(EndpointDescriptor::new(
                NodeTypeId::from("endpoint.text.test"),
                OperatorId::new("example.text.terminal.v1"),
            ))
            .expect("foreign terminal");

        let result = derived.send(foreign);

        assert!(matches!(result, Err(SessionError::ForeignEndpoint { .. })));
    }
}

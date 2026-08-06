use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::frame::{ConnectorId, EndpointId, RouteId, SessionId, StemId};
use crate::graph::NodeTypeId;

use crate::session::compiler::{
    BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID,
    DEFAULT_MULTISTEM_RECORDING_GROUP_ID, RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
    RECORDING_GROUP_CONFIGURATION_KEY,
};
use crate::session::spec::{
    derived_route_spec, endpoint_spec, operator_spec, route_spec, stem_spec,
};
use crate::session::{
    EndpointConfiguration, EndpointDescriptor, OperatorConfiguration, OperatorId,
    OperatorInstanceId, SessionError, SessionSpec, Source,
};

static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
struct StemDraft {
    stem_id: StemId,
    source: Source,
}

#[derive(Debug)]
struct EndpointDraft {
    endpoint_id: EndpointId,
    descriptor: EndpointDescriptor,
    connector_id: Option<ConnectorId>,
}

#[derive(Debug)]
struct RouteDraft {
    route_id: RouteId,
    stem_id: StemId,
    endpoint_id: EndpointId,
}

#[derive(Debug)]
struct OperatorDraft {
    instance_id: OperatorInstanceId,
    input_route_id: RouteId,
    source_stem_id: StemId,
    operator: Operator,
}

#[derive(Debug)]
struct DerivedRouteDraft {
    route_id: RouteId,
    operator_instance_id: OperatorInstanceId,
    endpoint_id: EndpointId,
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
    stems: Vec<StemDraft>,
    endpoints: Vec<EndpointDraft>,
    routes: Vec<RouteDraft>,
    operators: Vec<OperatorDraft>,
    derived_routes: Vec<DerivedRouteDraft>,
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
            stems: Vec::new(),
            endpoints: Vec::new(),
            routes: Vec::new(),
            operators: Vec::new(),
            derived_routes: Vec::new(),
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
}

#[derive(Debug, Clone)]
pub struct Operator {
    operator_id: OperatorId,
    configuration: OperatorConfiguration,
}

impl Operator {
    pub fn new(operator_id: OperatorId, configuration: OperatorConfiguration) -> Self {
        Self {
            operator_id,
            configuration,
        }
    }

    pub const fn operator_id(&self) -> &OperatorId {
        &self.operator_id
    }

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
            shared: Arc::clone(&self.shared),
            session_id: self.id(),
            stem_id,
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
        descriptor.validate()?;

        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.id())?;
        let endpoint_id = draft.allocate_endpoint_id()?;
        let connector_id = draft.allocate_connector_id()?;
        draft.endpoints.push(EndpointDraft {
            endpoint_id,
            descriptor,
            connector_id: Some(connector_id),
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
        .with_configuration(EndpointConfiguration::new().with("receiver_uri", receiver_uri));
        descriptor.validate()?;
        self.add_endpoint(descriptor, None)
    }

    /// Returns whether this declaration contains a canonical multistem recording route.
    pub fn declares_multistem_recording(&self) -> Result<bool, SessionError> {
        let draft = self.shared.draft()?;
        Ok(draft
            .endpoints
            .iter()
            .any(|endpoint| endpoint.descriptor.operator_id().as_str() == RECORDER_OPERATOR_ID))
    }

    pub fn freeze(self) -> Result<SessionSpec, SessionError> {
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.id())?;
        draft.status = DraftStatus::Frozen;

        let spec = SessionSpec::new(
            self.id(),
            draft
                .stems
                .iter()
                .map(|stem| stem_spec(stem.stem_id, stem.source.clone()))
                .collect(),
            draft
                .endpoints
                .iter()
                .map(|endpoint| {
                    endpoint_spec(
                        endpoint.endpoint_id,
                        endpoint.descriptor.node_type_id().clone(),
                        endpoint.descriptor.operator_id().clone(),
                        endpoint.descriptor.configuration().clone(),
                        endpoint.connector_id,
                    )
                })
                .collect(),
            draft
                .routes
                .iter()
                .map(|route| route_spec(route.route_id, route.stem_id, route.endpoint_id))
                .collect(),
            draft
                .operators
                .iter()
                .map(|operator| {
                    operator_spec(
                        operator.instance_id,
                        operator.input_route_id,
                        operator.source_stem_id,
                        operator.operator.operator_id.clone(),
                        operator.operator.configuration.clone(),
                    )
                })
                .collect(),
            draft
                .derived_routes
                .iter()
                .map(|route| {
                    derived_route_spec(
                        route.route_id,
                        route.operator_instance_id,
                        route.endpoint_id,
                    )
                })
                .collect(),
        );
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
            descriptor,
            connector_id,
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
pub struct EndpointHandle {
    session_id: SessionId,
    endpoint_id: EndpointId,
    connector_id: Option<ConnectorId>,
}

impl EndpointHandle {
    pub const fn session_id(self) -> SessionId {
        self.session_id
    }

    pub const fn id(self) -> EndpointId {
        self.endpoint_id
    }

    pub const fn connector_id(self) -> Option<ConnectorId> {
        self.connector_id
    }
}

#[derive(Clone)]
pub struct StemHandle {
    shared: Arc<SessionShared>,
    session_id: SessionId,
    stem_id: StemId,
}

impl StemHandle {
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub const fn id(&self) -> StemId {
        self.stem_id
    }

    pub fn send(&self, endpoint: EndpointHandle) -> Result<RouteId, SessionError> {
        if endpoint.session_id != self.session_id {
            return Err(SessionError::ForeignEndpoint {
                expected: self.session_id,
                actual: endpoint.session_id,
            });
        }
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.session_id)?;
        let route_id = draft.allocate_route_id()?;
        draft.routes.push(RouteDraft {
            route_id,
            stem_id: self.stem_id,
            endpoint_id: endpoint.endpoint_id,
        });
        Ok(route_id)
    }

    pub fn through(&self, operator: Operator) -> Result<DerivedStreamHandle, SessionError> {
        if operator.operator_id.as_str().trim().is_empty() {
            return Err(SessionError::InvalidOperator {
                reason: "operator id cannot be empty".to_owned(),
            });
        }
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.session_id)?;
        let instance_id = draft.allocate_operator_instance_id()?;
        let input_route_id = draft.allocate_route_id()?;
        draft.operators.push(OperatorDraft {
            instance_id,
            input_route_id,
            source_stem_id: self.stem_id,
            operator,
        });
        Ok(DerivedStreamHandle {
            shared: Arc::clone(&self.shared),
            session_id: self.session_id,
            operator_instance_id: instance_id,
        })
    }

    pub fn record(&self, stem_name: impl Into<String>) -> Result<EndpointHandle, SessionError> {
        let descriptor = EndpointDescriptor::new(
            NodeTypeId::from(RECORDER_NODE_TYPE_ID),
            OperatorId::new(RECORDER_OPERATOR_ID),
        )
        .with_configuration(
            EndpointConfiguration::new()
                .with("stem_name", stem_name)
                .with(
                    RECORDING_GROUP_CONFIGURATION_KEY,
                    DEFAULT_MULTISTEM_RECORDING_GROUP_ID,
                ),
        );
        descriptor.validate()?;

        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.session_id)?;
        let endpoint_id = draft.allocate_endpoint_id()?;
        draft.endpoints.push(EndpointDraft {
            endpoint_id,
            descriptor,
            connector_id: None,
        });
        let route_id = draft.allocate_route_id()?;
        draft.routes.push(RouteDraft {
            route_id,
            stem_id: self.stem_id,
            endpoint_id,
        });
        Ok(EndpointHandle {
            session_id: self.session_id,
            endpoint_id,
            connector_id: None,
        })
    }
}

#[derive(Clone)]
pub struct DerivedStreamHandle {
    shared: Arc<SessionShared>,
    session_id: SessionId,
    operator_instance_id: OperatorInstanceId,
}

impl DerivedStreamHandle {
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub const fn operator_instance_id(&self) -> OperatorInstanceId {
        self.operator_instance_id
    }

    pub fn send(&self, endpoint: EndpointHandle) -> Result<RouteId, SessionError> {
        if endpoint.session_id != self.session_id {
            return Err(SessionError::ForeignEndpoint {
                expected: self.session_id,
                actual: endpoint.session_id,
            });
        }
        let mut draft = self.shared.draft()?;
        draft.ensure_open(self.session_id)?;
        let route_id = draft.allocate_route_id()?;
        draft.derived_routes.push(DerivedRouteDraft {
            route_id,
            operator_instance_id: self.operator_instance_id,
            endpoint_id: endpoint.endpoint_id,
        });
        Ok(route_id)
    }
}

impl fmt::Debug for DerivedStreamHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DerivedStreamHandle")
            .field("session_id", &self.session_id)
            .field("operator_instance_id", &self.operator_instance_id)
            .finish()
    }
}

impl fmt::Debug for StemHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StemHandle")
            .field("session_id", &self.session_id)
            .field("stem_id", &self.stem_id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{ApplicationSelector, DeviceSelector};

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
        assert_eq!(spec.derived_routes().len(), 1);
        assert_eq!(
            spec.operators()[0].instance_id(),
            derived.operator_instance_id()
        );
        assert_ne!(spec.operators()[0].input_route_id(), output_route_id);
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

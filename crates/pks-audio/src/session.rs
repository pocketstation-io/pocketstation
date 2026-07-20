use std::collections::HashSet;
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use pks_frame::{ConnectorId, EndpointId, RouteId, SessionId, StemId};

static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SessionState {
    Draft = 0,
    Compiled = 1,
    Starting = 2,
    Running = 3,
    Stopping = 4,
    Stopped = 5,
    Failed = 6,
}

impl SessionState {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Draft,
            1 => Self::Compiled,
            2 => Self::Starting,
            3 => Self::Running,
            4 => Self::Stopping,
            5 => Self::Stopped,
            _ => Self::Failed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(u32);

impl ProcessId {
    pub const fn new(process_id: u32) -> Self {
        Self(process_id)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId(String);

impl DeviceId {
    pub fn new(device_id: impl Into<String>) -> Self {
        Self(device_id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ApplicationSelection {
    BundleId(String),
    ProcessId(ProcessId),
    StableId(pks_capture::StableSourceId),
    Name(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationSelector(ApplicationSelection);

impl ApplicationSelector {
    pub fn bundle_id(bundle_id: impl Into<String>) -> Self {
        Self(ApplicationSelection::BundleId(bundle_id.into()))
    }

    pub const fn process_id(process_id: ProcessId) -> Self {
        Self(ApplicationSelection::ProcessId(process_id))
    }

    pub fn stable_id(source_id: pks_capture::StableSourceId) -> Self {
        Self(ApplicationSelection::StableId(source_id))
    }

    pub fn name(name: impl Into<String>) -> Self {
        Self(ApplicationSelection::Name(name.into()))
    }

    fn validate(&self) -> Result<(), SessionError> {
        match &self.0 {
            ApplicationSelection::BundleId(bundle_id) if bundle_id.trim().is_empty() => Err(
                SessionError::InvalidSelector("application bundle id cannot be empty".to_owned()),
            ),
            ApplicationSelection::ProcessId(process_id) if process_id.get() == 0 => Err(
                SessionError::InvalidSelector("application process id must be non-zero".to_owned()),
            ),
            ApplicationSelection::StableId(source_id)
                if source_id.kind != pks_capture::SourceKind::Application =>
            {
                Err(SessionError::InvalidSelector(
                    "application stable id must identify an application".to_owned(),
                ))
            }
            ApplicationSelection::StableId(source_id) if source_id.stable_key.trim().is_empty() => {
                Err(SessionError::InvalidSelector(
                    "application stable id cannot be empty".to_owned(),
                ))
            }
            ApplicationSelection::Name(name) if name.trim().is_empty() => Err(
                SessionError::InvalidSelector("application name cannot be empty".to_owned()),
            ),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DeviceSelection {
    Default,
    Id(DeviceId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSelector(DeviceSelection);

impl DeviceSelector {
    pub const fn default() -> Self {
        Self(DeviceSelection::Default)
    }

    pub fn id(device_id: DeviceId) -> Self {
        Self(DeviceSelection::Id(device_id))
    }

    fn validate(&self) -> Result<(), SessionError> {
        match &self.0 {
            DeviceSelection::Id(device_id) if device_id.as_str().trim().is_empty() => Err(
                SessionError::InvalidSelector("microphone device id cannot be empty".to_owned()),
            ),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SourceSelection {
    Application(ApplicationSelector),
    Microphone(DeviceSelector),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source(SourceSelection);

impl Source {
    pub fn application(selector: ApplicationSelector) -> Self {
        Self(SourceSelection::Application(selector))
    }

    pub fn microphone(selector: DeviceSelector) -> Self {
        Self(SourceSelection::Microphone(selector))
    }

    pub const fn microphone_default() -> Self {
        Self(SourceSelection::Microphone(DeviceSelector::default()))
    }

    fn validate(&self) -> Result<(), SessionError> {
        match &self.0 {
            SourceSelection::Application(selector) => selector.validate(),
            SourceSelection::Microphone(selector) => selector.validate(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConnectorKey(String);

impl ConnectorKey {
    pub fn new(connector_key: impl Into<String>) -> Self {
        Self(connector_key.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndpointHandle {
    session_id: SessionId,
    endpoint_id: EndpointId,
}

impl EndpointHandle {
    pub const fn id(self) -> EndpointId {
        self.endpoint_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectorHandle {
    endpoint: EndpointHandle,
    connector_id: ConnectorId,
}

impl ConnectorHandle {
    pub const fn id(self) -> ConnectorId {
        self.connector_id
    }

    pub const fn endpoint(self) -> EndpointHandle {
        self.endpoint
    }
}

impl From<ConnectorHandle> for EndpointHandle {
    fn from(connector: ConnectorHandle) -> Self {
        connector.endpoint
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SessionError {
    #[error("session must contain at least one capture source")]
    NoSources,
    #[error("stem {stem_id:?} has no destination route")]
    NoRoutes { stem_id: StemId },
    #[error("invalid source selector: {0}")]
    InvalidSelector(String),
    #[error("endpoint configuration is invalid: {0}")]
    InvalidEndpoint(String),
    #[error("endpoint belongs to session {actual:?}, expected {expected:?}")]
    ForeignEndpoint {
        expected: SessionId,
        actual: SessionId,
    },
    #[error("session lifecycle cannot transition from {actual:?} to {expected:?}")]
    InvalidState {
        actual: SessionState,
        expected: SessionState,
    },
    #[error("session was stopped before runtime startup")]
    StoppedBeforeStart,
    #[error("Session plan validated, but branched runtime integration is not implemented yet")]
    RuntimeNotIntegrated,
}

#[derive(Debug)]
struct StemDescriptor {
    stem_id: StemId,
    source: Source,
}

#[derive(Debug)]
struct EndpointDescriptor {
    endpoint_id: EndpointId,
    endpoint_type_id: String,
    configuration: String,
    connector_id: Option<ConnectorId>,
}

#[derive(Debug)]
struct RouteDescriptor {
    route_id: RouteId,
    stem_id: StemId,
    endpoint_id: EndpointId,
}

#[derive(Debug, Default)]
struct SessionDraft {
    next_stem_id: u64,
    next_endpoint_id: u64,
    next_connector_id: u64,
    next_route_id: u64,
    stems: Vec<StemDescriptor>,
    endpoints: Vec<EndpointDescriptor>,
    routes: Vec<RouteDescriptor>,
    error: Option<SessionError>,
}

impl SessionDraft {
    fn allocate_stem_id(&mut self) -> StemId {
        self.next_stem_id = self.next_stem_id.saturating_add(1);
        StemId(self.next_stem_id)
    }

    fn allocate_endpoint_id(&mut self) -> EndpointId {
        self.next_endpoint_id = self.next_endpoint_id.saturating_add(1);
        EndpointId(self.next_endpoint_id)
    }

    fn allocate_connector_id(&mut self) -> ConnectorId {
        self.next_connector_id = self.next_connector_id.saturating_add(1);
        ConnectorId(self.next_connector_id)
    }

    fn allocate_route_id(&mut self) -> RouteId {
        self.next_route_id = self.next_route_id.saturating_add(1);
        RouteId(self.next_route_id)
    }

    fn record_error(&mut self, error: SessionError) {
        if self.error.is_none() {
            self.error = Some(error);
        }
    }

    fn validate(&self) -> Result<(), SessionError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        if self.stems.is_empty() {
            return Err(SessionError::NoSources);
        }
        for stem in &self.stems {
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

        let endpoint_ids: HashSet<_> = self
            .endpoints
            .iter()
            .map(|endpoint| endpoint.endpoint_id)
            .collect();
        let mut route_ids = HashSet::with_capacity(self.routes.len());
        for route in &self.routes {
            if !endpoint_ids.contains(&route.endpoint_id) {
                return Err(SessionError::InvalidEndpoint(format!(
                    "route {:?} references unknown endpoint {:?}",
                    route.route_id, route.endpoint_id
                )));
            }
            if !route_ids.insert(route.route_id) {
                return Err(SessionError::InvalidEndpoint(format!(
                    "duplicate route id {:?}",
                    route.route_id
                )));
            }
        }

        for endpoint in &self.endpoints {
            if endpoint.endpoint_type_id.trim().is_empty()
                || endpoint.configuration.trim().is_empty()
            {
                return Err(SessionError::InvalidEndpoint(format!(
                    "endpoint {:?} requires a type id and configuration",
                    endpoint.endpoint_id
                )));
            }
            if endpoint.endpoint_type_id == "io.pocketstation.connector.v1"
                && endpoint.connector_id.is_none()
            {
                return Err(SessionError::InvalidEndpoint(format!(
                    "connector endpoint {:?} requires a connector id",
                    endpoint.endpoint_id
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct SessionShared {
    session_id: SessionId,
    state: AtomicU8,
    stop_requested: AtomicBool,
    draft: Mutex<SessionDraft>,
}

impl SessionShared {
    fn draft(&self) -> MutexGuard<'_, SessionDraft> {
        self.draft
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn state(&self) -> SessionState {
        SessionState::from_u8(self.state.load(Ordering::Acquire))
    }

    fn set_state(&self, state: SessionState) {
        self.state.store(state as u8, Ordering::Release);
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
                state: AtomicU8::new(SessionState::Draft as u8),
                stop_requested: AtomicBool::new(false),
                draft: Mutex::new(SessionDraft::default()),
            }),
        }
    }

    pub fn id(&self) -> SessionId {
        self.shared.session_id
    }

    pub fn state(&self) -> SessionState {
        self.shared.state()
    }

    pub fn capture(&self, source: Source) -> StemHandle {
        let stem_id = {
            let mut draft = self.shared.draft();
            let stem_id = draft.allocate_stem_id();
            draft.stems.push(StemDescriptor { stem_id, source });
            stem_id
        };
        StemHandle {
            shared: Arc::clone(&self.shared),
            stem_id,
        }
    }

    pub fn connector(&self, connector_key: ConnectorKey) -> ConnectorHandle {
        let mut draft = self.shared.draft();
        let endpoint_id = draft.allocate_endpoint_id();
        let connector_id = draft.allocate_connector_id();
        draft.endpoints.push(EndpointDescriptor {
            endpoint_id,
            endpoint_type_id: "io.pocketstation.connector.v1".to_owned(),
            configuration: connector_key.as_str().to_owned(),
            connector_id: Some(connector_id),
        });
        ConnectorHandle {
            endpoint: EndpointHandle {
                session_id: self.shared.session_id,
                endpoint_id,
            },
            connector_id,
        }
    }

    pub fn browser(&self, receiver_uri: impl Into<String>) -> EndpointHandle {
        self.add_endpoint(
            "io.pocketstation.browser-receiver.v1",
            receiver_uri.into(),
            None,
        )
    }

    pub fn stop_handle(&self) -> StopHandle {
        StopHandle {
            shared: Arc::clone(&self.shared),
        }
    }

    pub async fn run(self) -> Result<(), SessionError> {
        let transition = self.shared.state.compare_exchange(
            SessionState::Draft as u8,
            SessionState::Compiled as u8,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
        if let Err(actual) = transition {
            self.shared.set_state(SessionState::Failed);
            return Err(SessionError::InvalidState {
                actual: SessionState::from_u8(actual),
                expected: SessionState::Draft,
            });
        }

        if let Err(error) = self.shared.draft().validate() {
            self.shared.set_state(SessionState::Failed);
            return Err(error);
        }
        if self.shared.stop_requested.load(Ordering::Acquire) {
            self.shared.set_state(SessionState::Stopped);
            return Err(SessionError::StoppedBeforeStart);
        }

        self.shared.set_state(SessionState::Starting);
        self.shared.set_state(SessionState::Failed);
        Err(SessionError::RuntimeNotIntegrated)
    }

    fn add_endpoint(
        &self,
        endpoint_type_id: &str,
        configuration: String,
        connector_id: Option<ConnectorId>,
    ) -> EndpointHandle {
        let endpoint_id = {
            let mut draft = self.shared.draft();
            let endpoint_id = draft.allocate_endpoint_id();
            draft.endpoints.push(EndpointDescriptor {
                endpoint_id,
                endpoint_type_id: endpoint_type_id.to_owned(),
                configuration,
                connector_id,
            });
            endpoint_id
        };
        EndpointHandle {
            session_id: self.shared.session_id,
            endpoint_id,
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Session")
            .field("session_id", &self.shared.session_id)
            .field("state", &self.state())
            .finish_non_exhaustive()
    }
}

#[derive(Clone)]
pub struct StemHandle {
    shared: Arc<SessionShared>,
    stem_id: StemId,
}

impl StemHandle {
    pub fn id(&self) -> StemId {
        self.stem_id
    }

    pub fn send(&self, endpoint: impl Into<EndpointHandle>) -> RouteId {
        let endpoint = endpoint.into();
        let mut draft = self.shared.draft();
        let route_id = draft.allocate_route_id();
        if endpoint.session_id != self.shared.session_id {
            draft.record_error(SessionError::ForeignEndpoint {
                expected: self.shared.session_id,
                actual: endpoint.session_id,
            });
        }
        draft.routes.push(RouteDescriptor {
            route_id,
            stem_id: self.stem_id,
            endpoint_id: endpoint.endpoint_id,
        });
        route_id
    }

    pub fn record(&self, stem_name: impl Into<String>) -> EndpointHandle {
        let endpoint_id = {
            let mut draft = self.shared.draft();
            let endpoint_id = draft.allocate_endpoint_id();
            draft.endpoints.push(EndpointDescriptor {
                endpoint_id,
                endpoint_type_id: "io.pocketstation.multistem-recorder.v1".to_owned(),
                configuration: stem_name.into(),
                connector_id: None,
            });
            let route_id = draft.allocate_route_id();
            draft.routes.push(RouteDescriptor {
                route_id,
                stem_id: self.stem_id,
                endpoint_id,
            });
            endpoint_id
        };
        EndpointHandle {
            session_id: self.shared.session_id,
            endpoint_id,
        }
    }
}

impl fmt::Debug for StemHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StemHandle")
            .field("session_id", &self.shared.session_id)
            .field("stem_id", &self.stem_id)
            .finish()
    }
}

#[derive(Clone)]
pub struct StopHandle {
    shared: Arc<SessionShared>,
}

impl StopHandle {
    pub fn stop(&self) -> bool {
        let first_request = !self.shared.stop_requested.swap(true, Ordering::AcqRel);
        match self.shared.state() {
            SessionState::Compiled | SessionState::Starting | SessionState::Running => {
                self.shared.set_state(SessionState::Stopping);
            }
            SessionState::Draft
            | SessionState::Stopping
            | SessionState::Stopped
            | SessionState::Failed => {}
        }
        first_request
    }

    pub fn state(&self) -> SessionState {
        self.shared.state()
    }
}

impl fmt::Debug for StopHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StopHandle")
            .field("session_id", &self.shared.session_id)
            .field("state", &self.state())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    use super::*;

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    fn block_on_ready<T>(future: impl Future<Output = T>) -> T {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);
        let mut future = pin!(future);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("test future unexpectedly pending"),
        }
    }

    fn proof_draft() -> (Session, StopHandle) {
        let session = Session::new();
        let stop = session.stop_handle();
        let application = session.capture(Source::application(ApplicationSelector::name(
            "Meeting App",
        )));
        let microphone = session.capture(Source::microphone_default());
        let connector = session.connector(ConnectorKey::new("example.local-stt.v1"));
        let browser = session.browser("ws://127.0.0.1:4800");
        application.send(connector);
        application.send(browser);
        application.record("application");
        microphone.send(connector);
        microphone.send(browser);
        microphone.record("microphone");
        (session, stop)
    }

    #[test]
    fn given_copyable_destination_handles_when_two_stems_send_then_routes_are_distinct() {
        // Given
        let session = Session::new();
        let first = session.capture(Source::microphone_default());
        let second = session.capture(Source::application(ApplicationSelector::name("App")));
        let connector = session.connector(ConnectorKey::new("example.connector.v1"));

        // When
        let first_route_id = first.send(connector);
        let second_route_id = second.send(connector);

        // Then
        assert_ne!(first_route_id, second_route_id);
        assert_eq!(connector.endpoint(), connector.endpoint());
    }

    #[test]
    fn given_foreign_endpoint_when_session_runs_then_failure_is_visible() {
        // Given
        let first_session = Session::new();
        let second_session = Session::new();
        let stem = first_session.capture(Source::microphone_default());
        let foreign_endpoint = second_session.browser("ws://127.0.0.1:4800");
        stem.send(foreign_endpoint);

        // When
        let result = block_on_ready(first_session.run());

        // Then
        assert!(matches!(result, Err(SessionError::ForeignEndpoint { .. })));
    }

    #[test]
    fn given_empty_application_name_when_session_runs_then_selector_is_rejected() {
        // Given
        let session = Session::new();
        let stem = session.capture(Source::application(ApplicationSelector::name("  ")));
        stem.record("application");

        // When
        let result = block_on_ready(session.run());

        // Then
        assert!(matches!(result, Err(SessionError::InvalidSelector(_))));
    }

    #[test]
    fn given_valid_draft_when_session_runs_before_w3_then_runtime_gap_is_visible() {
        // Given
        let (session, stop) = proof_draft();

        // When
        let result = block_on_ready(session.run());

        // Then
        assert_eq!(result, Err(SessionError::RuntimeNotIntegrated));
        assert_eq!(stop.state(), SessionState::Failed);
    }

    #[test]
    fn given_stop_handle_when_stop_called_twice_then_request_is_idempotent() {
        // Given
        let (session, stop) = proof_draft();

        // When
        let first_request = stop.stop();
        let second_request = stop.stop();
        let result = block_on_ready(session.run());

        // Then
        assert!(first_request);
        assert!(!second_request);
        assert_eq!(result, Err(SessionError::StoppedBeforeStart));
        assert_eq!(stop.state(), SessionState::Stopped);
    }

    #[test]
    fn given_stem_without_route_when_session_runs_then_missing_route_is_rejected() {
        // Given
        let session = Session::new();
        let stem = session.capture(Source::microphone_default());

        // When
        let result = block_on_ready(session.run());

        // Then
        assert_eq!(result, Err(SessionError::NoRoutes { stem_id: stem.id() }));
    }
}

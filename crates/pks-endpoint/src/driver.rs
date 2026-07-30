use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use pks_frame::{EndpointId, RouteId, SessionId, StemId};
use pks_graph::{NodeConfig, PrepareContext};

/// One Session-owned anchor in PocketStation's monotonic nanosecond clock.
///
/// The Session samples this value once during startup and supplies the same
/// value to every endpoint route prepared in that transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionTimelineOrigin {
    monotonic_timestamp_ns: u64,
}

impl SessionTimelineOrigin {
    pub const fn from_monotonic_timestamp_ns(monotonic_timestamp_ns: u64) -> Self {
        Self {
            monotonic_timestamp_ns,
        }
    }

    pub const fn monotonic_timestamp_ns(self) -> u64 {
        self.monotonic_timestamp_ns
    }
}

/// Typed Session route identity supplied to one endpoint input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndpointRouteContext {
    stem_id: StemId,
    route_id: RouteId,
}

impl EndpointRouteContext {
    pub const fn new(stem_id: StemId, route_id: RouteId) -> Self {
        Self { stem_id, route_id }
    }

    pub const fn stem_id(self) -> StemId {
        self.stem_id
    }

    pub const fn route_id(self) -> RouteId {
        self.route_id
    }
}

#[derive(Debug, Clone)]
pub struct EndpointPrepareContext {
    session_id: SessionId,
    endpoint_id: EndpointId,
    route_context: Option<EndpointRouteContext>,
    session_timeline_origin: Option<SessionTimelineOrigin>,
    node_configuration: NodeConfig,
    node_prepare_context: PrepareContext,
}

impl EndpointPrepareContext {
    pub fn new(
        session_id: SessionId,
        endpoint_id: EndpointId,
        node_configuration: NodeConfig,
        node_prepare_context: PrepareContext,
    ) -> Self {
        Self {
            session_id,
            endpoint_id,
            route_context: None,
            session_timeline_origin: None,
            node_configuration,
            node_prepare_context,
        }
    }

    /// Adds the authoritative typed route and Session timeline context.
    ///
    /// The original constructor intentionally remains valid for endpoint
    /// integrations compiled against the 0.1 API. Canonical Session startup
    /// always supplies this additive context.
    pub fn with_session_route(
        mut self,
        route_context: EndpointRouteContext,
        session_timeline_origin: SessionTimelineOrigin,
    ) -> Self {
        self.route_context = Some(route_context);
        self.session_timeline_origin = Some(session_timeline_origin);
        self
    }

    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }

    pub const fn route_context(&self) -> Option<EndpointRouteContext> {
        self.route_context
    }

    pub const fn session_timeline_origin(&self) -> Option<SessionTimelineOrigin> {
        self.session_timeline_origin
    }

    pub const fn node_configuration(&self) -> &NodeConfig {
        &self.node_configuration
    }

    pub const fn node_prepare_context(&self) -> &PrepareContext {
        &self.node_prepare_context
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointFailureStage {
    Prepare,
    CancelPreparation,
    Start,
    RequestStop,
    JoinFinalize,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("endpoint {stage:?} failed: {message}")]
pub struct EndpointFailure {
    stage: EndpointFailureStage,
    message: String,
}

impl EndpointFailure {
    pub fn new(stage: EndpointFailureStage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
        }
    }

    pub const fn stage(&self) -> EndpointFailureStage {
        self.stage
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EndpointDriverObservations {
    pub frames_received_total: u64,
    pub frames_delivered_total: u64,
    pub frames_dropped_total: u64,
    pub discontinuities_total: u64,
    pub failures_total: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointCancellationOutcome {
    pub observations: EndpointDriverObservations,
    pub result: Result<(), EndpointFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointDriverFinalization {
    pub observations: EndpointDriverObservations,
    pub result: Result<(), EndpointFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointFinalizationOutcome {
    pub observations: EndpointDriverObservations,
    pub request_stop_result: Result<(), EndpointFailure>,
    pub join_finalize_result: Result<(), EndpointFailure>,
}

impl EndpointFinalizationOutcome {
    pub fn is_success(&self) -> bool {
        self.request_stop_result.is_ok() && self.join_finalize_result.is_ok()
    }
}

/// Prepared endpoint resources that have not started consuming their edge.
///
/// Implementations must reclaim preparation resources from
/// `cancel_preparation` and from `Drop`. `start` consumes the prepared state;
/// on failure it must reclaim the same resources before returning.
pub trait PreparedEndpointDriver: Send {
    /// Makes endpoint resources ready behind the supplied closed gate.
    ///
    /// The implementation must not pop its `PlanEdgeReceiver`, publish output,
    /// or otherwise begin delivery until `start_gate.is_open()` becomes true.
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure>;

    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome;
}

/// Active endpoint resources owned until finalization.
///
/// Implementations may own workers created by concrete endpoint packages, but
/// this contract does not create or schedule threads. `Drop` must reclaim the
/// same resources as the explicit stop/finalize path.
pub trait RunningEndpointDriver: Send {
    fn observations(&self) -> EndpointDriverObservations;

    fn request_stop(&mut self) -> Result<(), EndpointFailure>;

    fn join_and_finalize(self: Box<Self>) -> EndpointDriverFinalization;
}

/// Read-only one-way start barrier shared by endpoint drivers in one startup.
pub struct EndpointStartGate {
    open: AtomicBool,
}

impl EndpointStartGate {
    pub fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }
}

/// Session-owned authority that opens one endpoint start gate.
pub struct EndpointStartGateController {
    gate: Arc<EndpointStartGate>,
}

impl EndpointStartGateController {
    pub fn open(&self) -> bool {
        !self.gate.open.swap(true, Ordering::AcqRel)
    }
}

/// Creates a closed Session-owned controller and driver-visible start gate.
pub fn endpoint_start_gate() -> (EndpointStartGateController, Arc<EndpointStartGate>) {
    let gate = Arc::new(EndpointStartGate {
        open: AtomicBool::new(false),
    });
    (
        EndpointStartGateController {
            gate: Arc::clone(&gate),
        },
        gate,
    )
}

pub struct PreparedEndpoint {
    pub(crate) driver: Box<dyn PreparedEndpointDriver>,
}

impl PreparedEndpoint {
    pub fn cancel_preparation(self) -> EndpointCancellationOutcome {
        self.driver.cancel_preparation()
    }

    pub fn start(
        self,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<RunningEndpoint, EndpointStartFailure> {
        if start_gate.is_open() {
            return Err(EndpointStartFailure {
                cause: EndpointStartFailureCause::GateAlreadyOpen,
                prepared: Some(self),
            });
        }
        match self.driver.start(start_gate) {
            Ok(driver) => Ok(RunningEndpoint {
                driver,
                request_stop_result: None,
            }),
            Err(failure) => Err(EndpointStartFailure {
                cause: EndpointStartFailureCause::Driver(failure),
                prepared: None,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndpointStartFailureCause {
    GateAlreadyOpen,
    Driver(EndpointFailure),
}

pub struct EndpointStartFailure {
    cause: EndpointStartFailureCause,
    prepared: Option<PreparedEndpoint>,
}

impl EndpointStartFailure {
    pub const fn cause(&self) -> &EndpointStartFailureCause {
        &self.cause
    }

    pub fn into_prepared(self) -> Option<PreparedEndpoint> {
        self.prepared
    }
}

impl fmt::Debug for EndpointStartFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EndpointStartFailure")
            .field("cause", &self.cause)
            .field("prepared_recoverable", &self.prepared.is_some())
            .finish()
    }
}

impl fmt::Display for EndpointStartFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.cause {
            EndpointStartFailureCause::GateAlreadyOpen => {
                formatter.write_str("endpoint start gate was already open")
            }
            EndpointStartFailureCause::Driver(failure) => failure.fmt(formatter),
        }
    }
}

impl std::error::Error for EndpointStartFailure {}

pub struct RunningEndpoint {
    driver: Box<dyn RunningEndpointDriver>,
    request_stop_result: Option<Result<(), EndpointFailure>>,
}

impl RunningEndpoint {
    pub fn observations(&self) -> EndpointDriverObservations {
        self.driver.observations()
    }

    pub fn request_stop(&mut self) -> &Result<(), EndpointFailure> {
        self.request_stop_result
            .get_or_insert_with(|| self.driver.request_stop())
    }

    pub fn join_and_finalize(self) -> EndpointFinalizationOutcome {
        let Self {
            mut driver,
            request_stop_result,
        } = self;
        let request_stop_result = request_stop_result.unwrap_or_else(|| driver.request_stop());
        let finalization = driver.join_and_finalize();
        EndpointFinalizationOutcome {
            observations: finalization.observations,
            request_stop_result,
            join_finalize_result: finalization.result,
        }
    }
}

#[cfg(test)]
mod tests {
    use pks_frame::{EndpointId, RouteId, SampleFormat, SampleSpec, SessionId, StemId};
    use pks_graph::{NodeConfig, PrepareContext};

    use super::{EndpointPrepareContext, EndpointRouteContext, SessionTimelineOrigin};

    fn prepare_context() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    #[test]
    fn given_legacy_constructor_when_read_then_additive_session_route_is_absent() {
        let context = EndpointPrepareContext::new(
            SessionId(1),
            EndpointId(2),
            NodeConfig::new(),
            prepare_context(),
        );

        assert_eq!(context.route_context(), None);
        assert_eq!(context.session_timeline_origin(), None);
    }

    #[test]
    fn given_session_route_when_attached_then_typed_identity_and_origin_round_trip() {
        let context = EndpointPrepareContext::new(
            SessionId(1),
            EndpointId(2),
            NodeConfig::new(),
            prepare_context(),
        )
        .with_session_route(
            EndpointRouteContext::new(StemId(3), RouteId(4)),
            SessionTimelineOrigin::from_monotonic_timestamp_ns(5),
        );

        let route_context = context
            .route_context()
            .expect("typed route context must be present");
        assert_eq!(route_context.stem_id(), StemId(3));
        assert_eq!(route_context.route_id(), RouteId(4));
        assert_eq!(
            context
                .session_timeline_origin()
                .expect("Session timeline origin must be present")
                .monotonic_timestamp_ns(),
            5
        );
    }
}

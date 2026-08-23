use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use crate::frame::{ConnectorId, EndpointId, RouteId, SessionId, SourceId, StemId, StreamId};
use crate::graph::NodeConfig;

/// One Session-owned anchor in PocketStation's monotonic nanosecond clock.
///
/// The Session samples this value once during startup and supplies the same
/// value to every endpoint route prepared in that transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionTimelineOrigin {
    monotonic_timestamp_ns: u64,
}

impl SessionTimelineOrigin {
    #[doc = "Creates `SessionTimelineOrigin` from monotonic timestamp nanoseconds."]
    pub const fn from_monotonic_timestamp_ns(monotonic_timestamp_ns: u64) -> Self {
        Self {
            monotonic_timestamp_ns,
        }
    }

    #[doc = "Returns the monotonic timestamp nanoseconds held by `SessionTimelineOrigin`."]
    pub const fn monotonic_timestamp_ns(self) -> u64 {
        self.monotonic_timestamp_ns
    }
}

/// Provenance of one endpoint input, independent of its physical receiver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointInputOrigin {
    #[doc = "Represents the stem case of `EndpointInputOrigin`."]
    Stem(StemId),
    /// A typed signal whose detailed provenance is carried by `SignalLineage`.
    Signal,
    #[doc = "Represents the source case of `EndpointInputOrigin`."]
    Source {
        #[doc = "Identifies the source identifier recorded by `Source`."]
        source_id: SourceId,
        #[doc = "Identifies the stream identifier recorded by `Source`."]
        stream_id: StreamId,
        #[doc = "Identifies the audio stem identifier recorded by `Source`."]
        audio_stem_id: Option<StemId>,
    },
}

/// Typed Session route identity supplied to every endpoint input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndpointRouteContext {
    route_id: RouteId,
    origin: EndpointInputOrigin,
}

impl EndpointRouteContext {
    #[doc = "Creates `EndpointRouteContext` from stem."]
    pub const fn from_stem(route_id: RouteId, stem_id: StemId) -> Self {
        Self {
            route_id,
            origin: EndpointInputOrigin::Stem(stem_id),
        }
    }

    #[doc = "Creates `EndpointRouteContext` from source."]
    pub const fn from_source(
        route_id: RouteId,
        source_id: SourceId,
        stream_id: StreamId,
        audio_stem_id: Option<StemId>,
    ) -> Self {
        Self {
            route_id,
            origin: EndpointInputOrigin::Source {
                source_id,
                stream_id,
                audio_stem_id,
            },
        }
    }

    #[doc = "Returns the signal held by `EndpointRouteContext`."]
    pub const fn signal(route_id: RouteId) -> Self {
        Self {
            route_id,
            origin: EndpointInputOrigin::Signal,
        }
    }

    #[doc = "Returns the route identifier held by `EndpointRouteContext`."]
    pub const fn route_id(self) -> RouteId {
        self.route_id
    }

    #[doc = "Returns the origin held by `EndpointRouteContext`."]
    pub const fn origin(self) -> EndpointInputOrigin {
        self.origin
    }

    #[doc = "Returns the audio stem identifier held by `EndpointRouteContext`."]
    pub const fn audio_stem_id(self) -> Option<StemId> {
        match self.origin {
            EndpointInputOrigin::Stem(stem_id) => Some(stem_id),
            EndpointInputOrigin::Signal => None,
            EndpointInputOrigin::Source { audio_stem_id, .. } => audio_stem_id,
        }
    }
}

#[derive(Debug, Clone)]
#[doc = "Carries the inputs and runtime context required to endpoint prepare."]
pub struct EndpointPrepareContext {
    session_id: SessionId,
    endpoint_id: EndpointId,
    connector_id: Option<ConnectorId>,
    route_context: EndpointRouteContext,
    session_timeline_origin: SessionTimelineOrigin,
    node_configuration: NodeConfig,
}

impl EndpointPrepareContext {
    #[doc = "Creates a new `EndpointPrepareContext`."]
    pub fn new(
        session_id: SessionId,
        endpoint_id: EndpointId,
        route_context: EndpointRouteContext,
        session_timeline_origin: SessionTimelineOrigin,
        node_configuration: NodeConfig,
    ) -> Self {
        Self {
            session_id,
            endpoint_id,
            connector_id: None,
            route_context,
            session_timeline_origin,
            node_configuration,
        }
    }

    #[doc = "Returns the session identifier held by `EndpointPrepareContext`."]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    #[doc = "Returns the endpoint identifier held by `EndpointPrepareContext`."]
    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }

    pub(crate) const fn with_connector_id(mut self, connector_id: Option<ConnectorId>) -> Self {
        self.connector_id = connector_id;
        self
    }

    #[doc = "Returns the connector identifier held by `EndpointPrepareContext`."]
    pub const fn connector_id(&self) -> Option<ConnectorId> {
        self.connector_id
    }

    #[doc = "Returns the route context held by `EndpointPrepareContext`."]
    pub const fn route_context(&self) -> EndpointRouteContext {
        self.route_context
    }

    #[doc = "Returns the session timeline origin held by `EndpointPrepareContext`."]
    pub const fn session_timeline_origin(&self) -> SessionTimelineOrigin {
        self.session_timeline_origin
    }

    #[doc = "Returns the node configuration held by `EndpointPrepareContext`."]
    pub const fn node_configuration(&self) -> &NodeConfig {
        &self.node_configuration
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the endpoint failure stage used by PocketStation."]
pub enum EndpointFailureStage {
    #[doc = "Reports prepare."]
    Prepare,
    #[doc = "Reports cancel preparation."]
    CancelPreparation,
    #[doc = "Reports start."]
    Start,
    #[doc = "Reports request stop."]
    RequestStop,
    #[doc = "Reports join finalize."]
    JoinFinalize,
}

/// Machine-readable recovery classification retained in Session outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointFailureRetryability {
    #[doc = "Reports never."]
    Never,
    #[doc = "Reports retryable."]
    Retryable,
    #[doc = "Reports reconfiguration required."]
    ReconfigurationRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("endpoint {stage:?} failed: {message}")]
#[doc = "Reports a endpoint failure."]
pub struct EndpointFailure {
    stage: EndpointFailureStage,
    message: String,
    code: Option<String>,
    retryability: Option<EndpointFailureRetryability>,
}

impl EndpointFailure {
    #[doc = "Creates a new `EndpointFailure`."]
    pub fn new(stage: EndpointFailureStage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
            code: None,
            retryability: None,
        }
    }

    /// Attaches stable external failure details without changing Endpoint's
    /// provider-neutral lifecycle authority.
    #[must_use]
    pub fn with_external_details(
        mut self,
        code: impl Into<String>,
        retryability: EndpointFailureRetryability,
    ) -> Self {
        self.code = Some(code.into());
        self.retryability = Some(retryability);
        self
    }

    #[doc = "Returns the stage held by `EndpointFailure`."]
    pub const fn stage(&self) -> EndpointFailureStage {
        self.stage
    }

    #[doc = "Returns the diagnostic message reported by `EndpointFailure`."]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[doc = "Returns the stable error or status code represented by `EndpointFailure`."]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    #[doc = "Returns the retryability held by `EndpointFailure`."]
    pub const fn retryability(&self) -> Option<EndpointFailureRetryability> {
        self.retryability
    }

    pub(crate) fn owned_heap_bytes(&self) -> usize {
        self.message
            .capacity()
            .saturating_add(self.code.as_ref().map_or(0, String::capacity))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[doc = "Reports the endpoint driver observations collected at an observation boundary."]
pub struct EndpointDriverObservations {
    #[doc = "Counts the total number of frames received observed by `EndpointDriverObservations`."]
    pub frames_received_total: u64,
    #[doc = "Counts the total number of frames delivered observed by `EndpointDriverObservations`."]
    pub frames_delivered_total: u64,
    #[doc = "Counts the total number of frames dropped observed by `EndpointDriverObservations`."]
    pub frames_dropped_total: u64,
    #[doc = "Counts the total number of discontinuities observed by `EndpointDriverObservations`."]
    pub discontinuities_total: u64,
    #[doc = "Counts the total number of failures observed by `EndpointDriverObservations`."]
    pub failures_total: u64,
}

#[derive(Clone, Default)]
pub(crate) struct EndpointDriverObservationHandle {
    inner: Arc<EndpointDriverObservationState>,
}

#[derive(Default)]
struct EndpointDriverObservationState {
    frames_received_total: AtomicU64,
    frames_delivered_total: AtomicU64,
    frames_dropped_total: AtomicU64,
    discontinuities_total: AtomicU64,
    failures_total: AtomicU64,
}

impl EndpointDriverObservationHandle {
    pub(crate) fn record_received(&self, amount: u64) {
        increment(&self.inner.frames_received_total, amount);
    }

    pub(crate) fn record_delivered(&self, amount: u64) {
        increment(&self.inner.frames_delivered_total, amount);
    }

    pub(crate) fn record_dropped(&self, amount: u64) {
        increment(&self.inner.frames_dropped_total, amount);
    }

    pub(crate) fn record_discontinuity(&self, amount: u64) {
        increment(&self.inner.discontinuities_total, amount);
    }

    pub(crate) fn record_failure(&self, amount: u64) {
        increment(&self.inner.failures_total, amount);
    }

    pub(crate) fn snapshot(&self) -> EndpointDriverObservations {
        EndpointDriverObservations {
            frames_received_total: self.inner.frames_received_total.load(Ordering::Relaxed),
            frames_delivered_total: self.inner.frames_delivered_total.load(Ordering::Relaxed),
            frames_dropped_total: self.inner.frames_dropped_total.load(Ordering::Relaxed),
            discontinuities_total: self.inner.discontinuities_total.load(Ordering::Relaxed),
            failures_total: self.inner.failures_total.load(Ordering::Relaxed),
        }
    }
}

fn increment(counter: &AtomicU64, amount: u64) {
    let _ = counter.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
        Some(value.saturating_add(amount))
    });
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Reports the structured endpoint cancellation outcome."]
pub struct EndpointCancellationOutcome {
    #[doc = "Carries the observations collected for `EndpointCancellationOutcome`."]
    pub observations: EndpointDriverObservations,
    #[doc = "Stores the result used by `EndpointCancellationOutcome`."]
    pub result: Result<(), EndpointFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Reports an endpoint driver's terminal observations and any finalization failure."]
pub struct EndpointDriverFinalization {
    #[doc = "Carries the observations collected for `EndpointDriverFinalization`."]
    pub observations: EndpointDriverObservations,
    #[doc = "Stores the result used by `EndpointDriverFinalization`."]
    pub result: Result<(), EndpointFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Reports the structured endpoint finalization outcome."]
pub struct EndpointFinalizationOutcome {
    #[doc = "Carries the observations collected for `EndpointFinalizationOutcome`."]
    pub observations: EndpointDriverObservations,
    #[doc = "Stores the request stop result used by `EndpointFinalizationOutcome`."]
    pub request_stop_result: Result<(), EndpointFailure>,
    #[doc = "Stores the join finalize result used by `EndpointFinalizationOutcome`."]
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
    /// The implementation must not pop its `EndpointAudioReceiver`, publish output,
    /// or otherwise begin delivery until `start_gate.is_open()` becomes true.
    fn start(
        self: Box<Self>,
        start_gate: Arc<EndpointStartGate>,
    ) -> Result<Box<dyn RunningEndpointDriver>, EndpointFailure>;

    #[doc = "Cancels preparation for `PreparedEndpointDriver`."]
    fn cancel_preparation(self: Box<Self>) -> EndpointCancellationOutcome;
}

/// Active endpoint resources owned until finalization.
///
/// Implementations may own workers created by concrete endpoint packages, but
/// this contract does not create or schedule threads. `Drop` must reclaim the
/// same resources as the explicit stop/finalize path.
pub trait RunningEndpointDriver: Send {
    #[doc = "Returns the observations exposed by `RunningEndpointDriver`."]
    fn observations(&self) -> EndpointDriverObservations;

    #[doc = "Requests a graceful stop from `RunningEndpointDriver`."]
    fn request_stop(&mut self) -> Result<(), EndpointFailure>;

    #[doc = "Requests the selected shutdown mode from `RunningEndpointDriver`."]
    fn request_shutdown(&mut self, mode: EndpointShutdownMode) -> Result<(), EndpointFailure> {
        let _ = mode;
        self.request_stop()
    }

    #[doc = "Joins and finalize for `RunningEndpointDriver`."]
    fn join_and_finalize(self: Box<Self>) -> EndpointDriverFinalization;
}

/// Session shutdown intent delivered to an active endpoint.
///
/// `Drain` permits already accepted work to finish. `Abort` asks the endpoint
/// to stop without accepting or waiting for additional work. Endpoint drivers
/// that only implement the legacy `request_stop` method retain their existing
/// behavior through the default `request_shutdown` adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointShutdownMode {
    #[doc = "Selects drain behavior for `EndpointShutdownMode`."]
    Drain,
    #[doc = "Selects abort behavior for `EndpointShutdownMode`."]
    Abort,
}

impl EndpointShutdownMode {
    const fn priority(self) -> u8 {
        match self {
            Self::Drain => 1,
            Self::Abort => 2,
        }
    }
}

/// Read-only one-way start barrier shared by endpoint drivers in one startup.
pub struct EndpointStartGate {
    open: AtomicBool,
}

impl EndpointStartGate {
    #[doc = "Returns whether open applies to `EndpointStartGate`."]
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

#[doc = "Owns endpoint resources after preparation and before its runtime driver starts."]
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
                shutdown_request: None,
            }),
            Err(failure) => Err(EndpointStartFailure {
                cause: EndpointStartFailureCause::Driver(failure),
                prepared: None,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Enumerates the supported endpoint start failure cause cases."]
pub enum EndpointStartFailureCause {
    #[doc = "Reports gate already open."]
    GateAlreadyOpen,
    #[doc = "Reports driver."]
    Driver(EndpointFailure),
}

#[doc = "Reports a endpoint start failure."]
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

#[doc = "Owns a started endpoint driver until shutdown and finalization complete."]
pub struct RunningEndpoint {
    driver: Box<dyn RunningEndpointDriver>,
    shutdown_request: Option<(EndpointShutdownMode, Result<(), EndpointFailure>)>,
}

impl RunningEndpoint {
    pub fn observations(&self) -> EndpointDriverObservations {
        self.driver.observations()
    }

    pub fn request_stop(&mut self) -> &Result<(), EndpointFailure> {
        self.request_shutdown(EndpointShutdownMode::Drain)
    }

    pub fn request_shutdown(&mut self, mode: EndpointShutdownMode) -> &Result<(), EndpointFailure> {
        let should_request = self
            .shutdown_request
            .as_ref()
            .is_none_or(|(current, _)| mode.priority() > current.priority());
        if should_request {
            let result = self.driver.request_shutdown(mode);
            self.shutdown_request = Some((mode, result));
        }
        &self
            .shutdown_request
            .as_ref()
            .expect("shutdown request is installed")
            .1
    }

    pub fn join_and_finalize(self) -> EndpointFinalizationOutcome {
        let Self {
            mut driver,
            shutdown_request,
        } = self;
        let request_stop_result = shutdown_request.map_or_else(
            || driver.request_shutdown(EndpointShutdownMode::Drain),
            |(_, result)| result,
        );
        let finalization = driver.join_and_finalize();
        EndpointFinalizationOutcome {
            observations: finalization.observations,
            request_stop_result,
            join_finalize_result: finalization.result,
        }
    }
}

#[cfg(test)]
mod tests;

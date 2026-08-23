use std::sync::{
    mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError},
    Arc,
};

use crate::capture::SourceRuntimeEvent;
use crate::endpoint::{EndpointFailure, EndpointFailureStage};
use crate::frame::{EndpointId, RouteId, SessionId, StemId};

use crate::session::lifecycle::observations::{
    SessionEventQueueCounters, SessionEventQueueObservations, SessionEventReservation,
};
use crate::session::{OperatorInstanceId, SessionTraceRecorderHandle};

pub const MAX_SESSION_EVENT_OWNED_BYTES: usize = 1024 * 1024;

/// Public lifecycle states emitted by a running session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionLifecycleState {
    #[doc = "Indicates the starting state for `SessionLifecycleState`."]
    Starting,
    #[doc = "Indicates the running state for `SessionLifecycleState`."]
    Running,
    #[doc = "Indicates the stopping state for `SessionLifecycleState`."]
    Stopping,
    #[doc = "Indicates that the operation stopped normally."]
    Stopped,
    #[doc = "Indicates the failed state for `SessionLifecycleState`."]
    Failed,
}

/// The rollback operation that failed while unwinding a partial start.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionRollbackStage {
    #[doc = "Represents the cancel operator case of `SessionRollbackStage`."]
    CancelOperator,
    #[doc = "Represents the cancel endpoint preparation case of `SessionRollbackStage`."]
    CancelEndpointPreparation,
    #[doc = "Represents the finalize started endpoint case of `SessionRollbackStage`."]
    FinalizeStartedEndpoint,
    #[doc = "Represents the stop opened capture case of `SessionRollbackStage`."]
    StopOpenedCapture,
    #[doc = "Represents the discard runtime queues case of `SessionRollbackStage`."]
    DiscardRuntimeQueues,
}

/// The finalization operation that failed while stopping a session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionFinalizationStage {
    #[doc = "Represents the stop capture case of `SessionFinalizationStage`."]
    StopCapture,
    #[doc = "Represents the drain runtime case of `SessionFinalizationStage`."]
    DrainRuntime,
    #[doc = "Represents the drain operator case of `SessionFinalizationStage`."]
    DrainOperator,
    #[doc = "Represents the request endpoint stop case of `SessionFinalizationStage`."]
    RequestEndpointStop,
    #[doc = "Represents the join endpoint case of `SessionFinalizationStage`."]
    JoinEndpoint,
    #[doc = "Represents the finalize endpoint case of `SessionFinalizationStage`."]
    FinalizeEndpoint,
    #[doc = "Represents the drain sidecar case of `SessionFinalizationStage`."]
    DrainSidecar,
}

/// Stable identity of the component that produced a session control failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionComponentId {
    #[doc = "Represents the source case of `SessionComponentId`."]
    Source {
        #[doc = "Identifies the stem identifier recorded by `Source`."]
        stem_id: StemId,
    },
    #[doc = "Represents the endpoint case of `SessionComponentId`."]
    Endpoint {
        #[doc = "Identifies the route identifier recorded by `Endpoint`."]
        route_id: RouteId,
        #[doc = "Identifies the endpoint identifier recorded by `Endpoint`."]
        endpoint_id: EndpointId,
    },
    #[doc = "Represents the operator case of `SessionComponentId`."]
    Operator {
        #[doc = "Identifies the operator instance identifier recorded by `Operator`."]
        operator_instance_id: OperatorInstanceId,
    },
    #[doc = "Represents the sidecar case of `SessionComponentId`."]
    Sidecar {
        #[doc = "Identifies the sidecar identifier recorded by `Sidecar`."]
        sidecar_id: u64,
    },
    #[doc = "Represents the runtime case of `SessionComponentId`."]
    Runtime,
}

/// Typed control-plane failure without exposing an implementation error type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionControlFailure {
    component: SessionComponentId,
    operation: &'static str,
    error_class: String,
}

impl SessionControlFailure {
    pub(crate) fn new(
        component: SessionComponentId,
        operation: &'static str,
        error_class: impl Into<String>,
    ) -> Self {
        Self {
            component,
            operation,
            error_class: error_class.into(),
        }
    }

    #[doc = "Returns the component held by `SessionControlFailure`."]
    pub const fn component(&self) -> SessionComponentId {
        self.component
    }

    #[doc = "Returns the operation held by `SessionControlFailure`."]
    pub const fn operation(&self) -> &'static str {
        self.operation
    }

    #[doc = "Returns the error class held by `SessionControlFailure`."]
    pub fn error_class(&self) -> &str {
        &self.error_class
    }
}

/// Source failure associated with one stable session stem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionSourceFailure {
    stem_id: StemId,
    event: SourceRuntimeEvent,
}

impl SessionSourceFailure {
    pub(crate) const fn new(stem_id: StemId, event: SourceRuntimeEvent) -> Self {
        Self { stem_id, event }
    }

    #[doc = "Returns the stem identifier held by `SessionSourceFailure`."]
    pub const fn stem_id(&self) -> StemId {
        self.stem_id
    }

    #[doc = "Returns the event held by `SessionSourceFailure`."]
    pub const fn event(&self) -> &SourceRuntimeEvent {
        &self.event
    }
}

/// Endpoint failure associated with one stable route and endpoint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionEndpointFailure {
    route_id: RouteId,
    endpoint_id: EndpointId,
    stage: EndpointFailureStage,
    failure: EndpointFailure,
}

impl SessionEndpointFailure {
    pub(crate) fn new(
        route_id: RouteId,
        endpoint_id: EndpointId,
        failure: EndpointFailure,
    ) -> Self {
        Self {
            route_id,
            endpoint_id,
            stage: failure.stage(),
            failure,
        }
    }

    #[doc = "Returns the route identifier held by `SessionEndpointFailure`."]
    pub const fn route_id(&self) -> RouteId {
        self.route_id
    }

    #[doc = "Returns the endpoint identifier held by `SessionEndpointFailure`."]
    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }

    #[doc = "Returns the stage held by `SessionEndpointFailure`."]
    pub const fn stage(&self) -> EndpointFailureStage {
        self.stage
    }

    #[doc = "Returns the failure held by `SessionEndpointFailure`."]
    pub const fn failure(&self) -> &EndpointFailure {
        &self.failure
    }
}

/// Failure observed while rolling back a partial session start.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionRollbackFailure {
    stage: SessionRollbackStage,
    failure: SessionControlFailure,
}

impl SessionRollbackFailure {
    pub(crate) const fn new(stage: SessionRollbackStage, failure: SessionControlFailure) -> Self {
        Self { stage, failure }
    }

    #[doc = "Returns the stage held by `SessionRollbackFailure`."]
    pub const fn stage(&self) -> SessionRollbackStage {
        self.stage
    }

    #[doc = "Returns the failure held by `SessionRollbackFailure`."]
    pub const fn failure(&self) -> &SessionControlFailure {
        &self.failure
    }
}

/// Failure observed while finalizing a stopping session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionFinalizationFailure {
    stage: SessionFinalizationStage,
    failure: SessionControlFailure,
}

impl SessionFinalizationFailure {
    pub(crate) const fn new(
        stage: SessionFinalizationStage,
        failure: SessionControlFailure,
    ) -> Self {
        Self { stage, failure }
    }

    #[doc = "Returns the stage held by `SessionFinalizationFailure`."]
    pub const fn stage(&self) -> SessionFinalizationStage {
        self.stage
    }

    #[doc = "Returns the failure held by `SessionFinalizationFailure`."]
    pub const fn failure(&self) -> &SessionControlFailure {
        &self.failure
    }
}

/// Final state carried by the terminal session event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionTerminalState {
    #[doc = "Indicates that the operation stopped normally."]
    Stopped,
    #[doc = "Indicates the failed state for `SessionTerminalState`."]
    Failed,
}

/// Complete terminal result. Failure categories remain separate for diagnosis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionTerminalOutcome {
    session_id: SessionId,
    state: SessionTerminalState,
    source_failures: Box<[SessionSourceFailure]>,
    endpoint_failures: Box<[SessionEndpointFailure]>,
    rollback_failures: Box<[SessionRollbackFailure]>,
    finalization_failures: Box<[SessionFinalizationFailure]>,
}

impl SessionTerminalOutcome {
    pub(crate) fn new(
        session_id: SessionId,
        source_failures: Vec<SessionSourceFailure>,
        endpoint_failures: Vec<SessionEndpointFailure>,
        rollback_failures: Vec<SessionRollbackFailure>,
        finalization_failures: Vec<SessionFinalizationFailure>,
    ) -> Self {
        let state = if source_failures.is_empty()
            && endpoint_failures.is_empty()
            && rollback_failures.is_empty()
            && finalization_failures.is_empty()
        {
            SessionTerminalState::Stopped
        } else {
            SessionTerminalState::Failed
        };
        Self {
            session_id,
            state,
            source_failures: source_failures.into_boxed_slice(),
            endpoint_failures: endpoint_failures.into_boxed_slice(),
            rollback_failures: rollback_failures.into_boxed_slice(),
            finalization_failures: finalization_failures.into_boxed_slice(),
        }
    }

    pub(crate) fn failed_start(
        session_id: SessionId,
        rollback_failures: Vec<SessionRollbackFailure>,
    ) -> Self {
        Self {
            session_id,
            state: SessionTerminalState::Failed,
            source_failures: Box::new([]),
            endpoint_failures: Box::new([]),
            rollback_failures: rollback_failures.into_boxed_slice(),
            finalization_failures: Box::new([]),
        }
    }

    #[doc = "Returns the session identifier held by `SessionTerminalOutcome`."]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    #[doc = "Returns the state held by `SessionTerminalOutcome`."]
    pub const fn state(&self) -> SessionTerminalState {
        self.state
    }

    #[doc = "Returns the source failures held by `SessionTerminalOutcome`."]
    pub fn source_failures(&self) -> &[SessionSourceFailure] {
        &self.source_failures
    }

    #[doc = "Returns the endpoint failures held by `SessionTerminalOutcome`."]
    pub fn endpoint_failures(&self) -> &[SessionEndpointFailure] {
        &self.endpoint_failures
    }

    #[doc = "Returns the rollback failures held by `SessionTerminalOutcome`."]
    pub fn rollback_failures(&self) -> &[SessionRollbackFailure] {
        &self.rollback_failures
    }

    #[doc = "Returns the finalization failures held by `SessionTerminalOutcome`."]
    pub fn finalization_failures(&self) -> &[SessionFinalizationFailure] {
        &self.finalization_failures
    }
}

/// Payload of one authoritative session event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SessionEventKind {
    #[doc = "Indicates the lifecycle state for `SessionEventKind`."]
    Lifecycle(SessionLifecycleState),
    #[doc = "Indicates the source state for `SessionEventKind`."]
    Source(SessionSourceFailure),
    #[doc = "Indicates the endpoint state for `SessionEventKind`."]
    Endpoint(SessionEndpointFailure),
    #[doc = "Indicates the rollback state for `SessionEventKind`."]
    Rollback(SessionRollbackFailure),
    #[doc = "Indicates the finalization state for `SessionEventKind`."]
    Finalization(SessionFinalizationFailure),
    #[doc = "Indicates the terminal state for `SessionEventKind`."]
    Terminal(SessionTerminalOutcome),
}

/// Event emitted by the session lifecycle authority.
///
/// Construction and publication remain crate-private. Consumers can inspect
/// events, but cannot inject fabricated lifecycle transitions into the queue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionEvent {
    session_id: SessionId,
    kind: SessionEventKind,
}

impl SessionEvent {
    pub(crate) const fn new(session_id: SessionId, kind: SessionEventKind) -> Self {
        Self { session_id, kind }
    }

    #[doc = "Returns the session identifier held by `SessionEvent`."]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    #[doc = "Returns the kind represented by `SessionEvent`."]
    pub const fn kind(&self) -> &SessionEventKind {
        &self.kind
    }

    fn owned_bytes(&self) -> usize {
        std::mem::size_of::<Self>().saturating_add(match &self.kind {
            SessionEventKind::Lifecycle(_) => 0,
            SessionEventKind::Source(failure) => failure
                .event
                .owned_bytes()
                .saturating_sub(std::mem::size_of::<SourceRuntimeEvent>()),
            SessionEventKind::Endpoint(failure) => failure.failure.owned_heap_bytes(),
            SessionEventKind::Rollback(failure) => failure.failure.error_class.capacity(),
            SessionEventKind::Finalization(failure) => failure.failure.error_class.capacity(),
            SessionEventKind::Terminal(outcome) => outcome.owned_heap_bytes(),
        })
    }
}

impl SessionTerminalOutcome {
    fn owned_heap_bytes(&self) -> usize {
        let source_bytes = self.source_failures.iter().fold(
            self.source_failures.len() * std::mem::size_of::<SessionSourceFailure>(),
            |sum, failure| {
                sum.saturating_add(
                    failure
                        .event
                        .owned_bytes()
                        .saturating_sub(std::mem::size_of::<SourceRuntimeEvent>()),
                )
            },
        );
        let endpoint_bytes = self.endpoint_failures.iter().fold(
            self.endpoint_failures.len() * std::mem::size_of::<SessionEndpointFailure>(),
            |sum, failure| sum.saturating_add(failure.failure.owned_heap_bytes()),
        );
        let rollback_bytes = self.rollback_failures.iter().fold(
            self.rollback_failures.len() * std::mem::size_of::<SessionRollbackFailure>(),
            |sum, failure| sum.saturating_add(failure.failure.error_class.capacity()),
        );
        let finalization_bytes = self.finalization_failures.iter().fold(
            self.finalization_failures.len() * std::mem::size_of::<SessionFinalizationFailure>(),
            |sum, failure| sum.saturating_add(failure.failure.error_class.capacity()),
        );
        source_bytes
            .saturating_add(endpoint_bytes)
            .saturating_add(rollback_bytes)
            .saturating_add(finalization_bytes)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionEventDelivery {
    Enqueued,
    DroppedFull,
    DroppedOversized,
    ReceiverClosed,
}

#[derive(Debug)]
struct QueuedSessionEvent {
    event: SessionEvent,
    owned_bytes: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct SessionEventSender {
    sender: SyncSender<QueuedSessionEvent>,
    counters: Arc<SessionEventQueueCounters>,
    session_trace_recorder: Option<SessionTraceRecorderHandle>,
}

impl SessionEventSender {
    pub(crate) fn publish_lifecycle(
        &self,
        session_id: SessionId,
        state: SessionLifecycleState,
    ) -> SessionEventDelivery {
        self.try_send(SessionEvent::new(
            session_id,
            SessionEventKind::Lifecycle(state),
        ))
    }

    pub(crate) fn publish_source(
        &self,
        session_id: SessionId,
        failure: SessionSourceFailure,
    ) -> SessionEventDelivery {
        self.try_send(SessionEvent::new(
            session_id,
            SessionEventKind::Source(failure),
        ))
    }

    pub(crate) fn publish_endpoint(
        &self,
        session_id: SessionId,
        failure: SessionEndpointFailure,
    ) -> SessionEventDelivery {
        self.try_send(SessionEvent::new(
            session_id,
            SessionEventKind::Endpoint(failure),
        ))
    }

    pub(crate) fn publish_rollback(
        &self,
        session_id: SessionId,
        failure: SessionRollbackFailure,
    ) -> SessionEventDelivery {
        self.try_send(SessionEvent::new(
            session_id,
            SessionEventKind::Rollback(failure),
        ))
    }

    pub(crate) fn publish_finalization(
        &self,
        session_id: SessionId,
        failure: SessionFinalizationFailure,
    ) -> SessionEventDelivery {
        self.try_send(SessionEvent::new(
            session_id,
            SessionEventKind::Finalization(failure),
        ))
    }

    pub(crate) fn publish_terminal(&self, outcome: SessionTerminalOutcome) -> SessionEventDelivery {
        self.try_send(SessionEvent::new(
            outcome.session_id(),
            SessionEventKind::Terminal(outcome),
        ))
    }

    fn try_send(&self, event: SessionEvent) -> SessionEventDelivery {
        if let Some(session_trace_recorder) = &self.session_trace_recorder {
            let _ = session_trace_recorder.try_record_event(&event);
        }
        let owned_bytes = event.owned_bytes();
        match self.counters.reserve_event(owned_bytes) {
            SessionEventReservation::Reserved => {}
            SessionEventReservation::Full => return SessionEventDelivery::DroppedFull,
            SessionEventReservation::Oversized => {
                return SessionEventDelivery::DroppedOversized;
            }
        }

        match self
            .sender
            .try_send(QueuedSessionEvent { event, owned_bytes })
        {
            Ok(()) => {
                self.counters.observe_enqueued();
                SessionEventDelivery::Enqueued
            }
            Err(TrySendError::Full(queued)) => {
                self.counters.observe_send_full(queued.owned_bytes);
                SessionEventDelivery::DroppedFull
            }
            Err(TrySendError::Disconnected(queued)) => {
                self.counters.observe_receiver_closed(queued.owned_bytes);
                SessionEventDelivery::ReceiverClosed
            }
        }
    }
}

/// Result of non-blocking event polling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SessionEventReceive {
    #[doc = "Indicates the event state for `SessionEventReceive`."]
    Event(SessionEvent),
    #[doc = "Represents an empty value or collection."]
    Empty,
    #[doc = "Reports that the underlying channel or resource is closed."]
    Closed,
}

/// Sole consumer for a session's bounded control-event queue.
#[derive(Debug)]
pub struct SessionEventReceiver {
    receiver: Receiver<QueuedSessionEvent>,
    counters: Arc<SessionEventQueueCounters>,
}

impl SessionEventReceiver {
    #[doc = "Attempts to receive the next value from `SessionEventReceiver` without waiting."]
    pub fn try_recv(&self) -> SessionEventReceive {
        match self.receiver.try_recv() {
            Ok(queued) => {
                self.counters.observe_dequeued(queued.owned_bytes);
                SessionEventReceive::Event(queued.event)
            }
            Err(TryRecvError::Empty) => SessionEventReceive::Empty,
            Err(TryRecvError::Disconnected) => SessionEventReceive::Closed,
        }
    }

    #[doc = "Returns the observations exposed by `SessionEventReceiver`."]
    pub fn observations(&self) -> SessionEventQueueObservations {
        self.counters.snapshot()
    }
}

pub(crate) fn session_event_channel(
    capacity_events: usize,
    session_trace_recorder: Option<SessionTraceRecorderHandle>,
) -> (SessionEventSender, SessionEventReceiver) {
    assert!(
        capacity_events > 0,
        "session event capacity must be non-zero"
    );
    let (sender, receiver) = mpsc::sync_channel(capacity_events);
    let counters = Arc::new(SessionEventQueueCounters::new(
        capacity_events,
        MAX_SESSION_EVENT_OWNED_BYTES,
    ));
    (
        SessionEventSender {
            sender,
            counters: Arc::clone(&counters),
            session_trace_recorder,
        },
        SessionEventReceiver { receiver, counters },
    )
}

#[cfg(test)]
mod tests {
    use crate::capture::{
        CaptureRuntimeFailure, CaptureRuntimeFailureClass, SourceGeneration, SourceKind,
        SourceRecoveryRequirement, StableSourceId,
    };
    use crate::frame::Platform;

    use super::*;

    fn source_failure(stem_id: u64) -> SessionSourceFailure {
        SessionSourceFailure::new(
            StemId(stem_id),
            SourceRuntimeEvent::SourceUnavailable {
                stable_id: StableSourceId::new(
                    Platform::Unknown,
                    SourceKind::Application,
                    "test:source",
                ),
                generation: SourceGeneration::INITIAL,
                recovery_requirement: SourceRecoveryRequirement::ExplicitRediscoveryAndNewSession,
                failure: CaptureRuntimeFailure {
                    operation: "capture",
                    error_class: CaptureRuntimeFailureClass::SourceInstanceExited,
                },
            },
        )
    }

    #[test]
    fn given_full_queue_when_publishing_then_newest_event_is_dropped_and_counted() {
        let (sender, receiver) = session_event_channel(1, None);
        let lifecycle_owned_bytes = SessionEvent::new(
            SessionId(7),
            SessionEventKind::Lifecycle(SessionLifecycleState::Starting),
        )
        .owned_bytes() as u64;

        assert_eq!(
            sender.publish_lifecycle(SessionId(7), SessionLifecycleState::Starting),
            SessionEventDelivery::Enqueued
        );
        assert_eq!(
            sender.publish_lifecycle(SessionId(7), SessionLifecycleState::Running),
            SessionEventDelivery::DroppedFull
        );

        assert_eq!(
            receiver.observations(),
            SessionEventQueueObservations {
                capacity_event_count: 1,
                maximum_event_owned_bytes: MAX_SESSION_EVENT_OWNED_BYTES as u64,
                maximum_buffered_owned_bytes: MAX_SESSION_EVENT_OWNED_BYTES as u64,
                depth_events: 1,
                depth_owned_bytes: lifecycle_owned_bytes,
                peak_depth_event_count: 1,
                peak_depth_owned_bytes: lifecycle_owned_bytes,
                events_enqueued_total: 1,
                events_dropped_total: 1,
                events_dropped_oversized_total: 0,
                receiver_closed_total: 0,
            }
        );
    }

    #[test]
    fn given_oversized_session_event_when_published_then_queue_owned_memory_stays_bounded() {
        let (sender, receiver) = session_event_channel(2, None);
        let failure = SessionEndpointFailure::new(
            RouteId(3),
            EndpointId(4),
            EndpointFailure::new(
                EndpointFailureStage::JoinFinalize,
                "x".repeat(MAX_SESSION_EVENT_OWNED_BYTES),
            ),
        );

        assert_eq!(
            sender.publish_endpoint(SessionId(10), failure),
            SessionEventDelivery::DroppedOversized
        );
        assert_eq!(receiver.try_recv(), SessionEventReceive::Empty);
        let observations = receiver.observations();
        assert_eq!(observations.depth_events, 0);
        assert_eq!(observations.depth_owned_bytes, 0);
        assert_eq!(observations.events_dropped_total, 1);
        assert_eq!(observations.events_dropped_oversized_total, 1);
    }

    #[test]
    fn given_events_when_polled_then_fifo_order_and_depth_are_preserved() {
        let (sender, receiver) = session_event_channel(2, None);
        sender.publish_lifecycle(SessionId(8), SessionLifecycleState::Starting);
        sender.publish_lifecycle(SessionId(8), SessionLifecycleState::Running);

        let SessionEventReceive::Event(first) = receiver.try_recv() else {
            panic!("expected first event");
        };
        let SessionEventReceive::Event(second) = receiver.try_recv() else {
            panic!("expected second event");
        };

        assert_eq!(
            first.kind(),
            &SessionEventKind::Lifecycle(SessionLifecycleState::Starting)
        );
        assert_eq!(
            second.kind(),
            &SessionEventKind::Lifecycle(SessionLifecycleState::Running)
        );
        assert_eq!(receiver.observations().depth_events, 0);
    }

    #[test]
    fn given_all_senders_dropped_when_polled_then_receiver_reports_closed() {
        let (sender, receiver) = session_event_channel(1, None);
        drop(sender);

        assert_eq!(receiver.try_recv(), SessionEventReceive::Closed);
    }

    #[test]
    fn given_closed_receiver_when_publishing_then_drop_and_closure_are_counted() {
        let (sender, receiver) = session_event_channel(1, None);
        let counters = Arc::clone(&sender.counters);
        drop(receiver);

        assert_eq!(
            sender.publish_lifecycle(SessionId(9), SessionLifecycleState::Starting),
            SessionEventDelivery::ReceiverClosed
        );
        assert_eq!(counters.snapshot().depth_events, 0);
        assert_eq!(counters.snapshot().events_dropped_total, 1);
        assert_eq!(counters.snapshot().receiver_closed_total, 1);
    }

    #[test]
    fn given_all_failure_classes_when_terminal_then_each_class_is_preserved() {
        let endpoint_failure = SessionEndpointFailure::new(
            RouteId(3),
            EndpointId(4),
            EndpointFailure::new(EndpointFailureStage::JoinFinalize, "join failed"),
        );
        let rollback_failure = SessionRollbackFailure::new(
            SessionRollbackStage::StopOpenedCapture,
            SessionControlFailure::new(
                SessionComponentId::Source { stem_id: StemId(5) },
                "stop_capture",
                "capture-stop-failed",
            ),
        );
        let finalization_failure = SessionFinalizationFailure::new(
            SessionFinalizationStage::FinalizeEndpoint,
            SessionControlFailure::new(
                SessionComponentId::Endpoint {
                    route_id: RouteId(3),
                    endpoint_id: EndpointId(4),
                },
                "finalize_endpoint",
                "endpoint-finalize-failed",
            ),
        );

        let outcome = SessionTerminalOutcome::new(
            SessionId(10),
            vec![source_failure(2)],
            vec![endpoint_failure],
            vec![rollback_failure],
            vec![finalization_failure],
        );

        assert_eq!(outcome.state(), SessionTerminalState::Failed);
        assert_eq!(outcome.source_failures().len(), 1);
        assert_eq!(outcome.endpoint_failures().len(), 1);
        assert_eq!(outcome.rollback_failures().len(), 1);
        assert_eq!(outcome.finalization_failures().len(), 1);
    }

    #[test]
    fn given_no_failures_when_terminal_then_state_is_stopped() {
        let outcome = SessionTerminalOutcome::new(
            SessionId(11),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(outcome.state(), SessionTerminalState::Stopped);
    }
}

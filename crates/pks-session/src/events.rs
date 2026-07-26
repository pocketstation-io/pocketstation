use std::sync::{
    mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError},
    Arc,
};

use pks_capture::SourceRuntimeEvent;
use pks_endpoint::{EndpointFailure, EndpointFailureStage};
use pks_frame::{EndpointId, RouteId, SessionId, StemId};

use crate::observations::{SessionEventQueueCounters, SessionEventQueueObservations};

/// Public lifecycle states emitted by a running session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionLifecycleState {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

/// The rollback operation that failed while unwinding a partial start.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionRollbackStage {
    CancelEndpointPreparation,
    FinalizeStartedEndpoint,
    StopOpenedCapture,
    DiscardRuntimeQueues,
}

/// The finalization operation that failed while stopping a session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionFinalizationStage {
    StopCapture,
    DrainRuntime,
    RequestEndpointStop,
    JoinEndpoint,
    FinalizeEndpoint,
}

/// Stable identity of the component that produced a session control failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionComponentId {
    Source {
        stem_id: StemId,
    },
    Endpoint {
        route_id: RouteId,
        endpoint_id: EndpointId,
    },
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

    pub const fn component(&self) -> SessionComponentId {
        self.component
    }

    pub const fn operation(&self) -> &'static str {
        self.operation
    }

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

    pub const fn stem_id(&self) -> StemId {
        self.stem_id
    }

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

    pub const fn route_id(&self) -> RouteId {
        self.route_id
    }

    pub const fn endpoint_id(&self) -> EndpointId {
        self.endpoint_id
    }

    pub const fn stage(&self) -> EndpointFailureStage {
        self.stage
    }

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

    pub const fn stage(&self) -> SessionRollbackStage {
        self.stage
    }

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

    pub const fn stage(&self) -> SessionFinalizationStage {
        self.stage
    }

    pub const fn failure(&self) -> &SessionControlFailure {
        &self.failure
    }
}

/// Final state carried by the terminal session event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionTerminalState {
    Stopped,
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

    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub const fn state(&self) -> SessionTerminalState {
        self.state
    }

    pub fn source_failures(&self) -> &[SessionSourceFailure] {
        &self.source_failures
    }

    pub fn endpoint_failures(&self) -> &[SessionEndpointFailure] {
        &self.endpoint_failures
    }

    pub fn rollback_failures(&self) -> &[SessionRollbackFailure] {
        &self.rollback_failures
    }

    pub fn finalization_failures(&self) -> &[SessionFinalizationFailure] {
        &self.finalization_failures
    }
}

/// Payload of one authoritative session event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SessionEventKind {
    Lifecycle(SessionLifecycleState),
    Source(SessionSourceFailure),
    Endpoint(SessionEndpointFailure),
    Rollback(SessionRollbackFailure),
    Finalization(SessionFinalizationFailure),
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
    const fn new(session_id: SessionId, kind: SessionEventKind) -> Self {
        Self { session_id, kind }
    }

    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub const fn kind(&self) -> &SessionEventKind {
        &self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionEventDelivery {
    Enqueued,
    DroppedFull,
    ReceiverClosed,
}

#[derive(Clone, Debug)]
pub(crate) struct SessionEventSender {
    sender: SyncSender<SessionEvent>,
    counters: Arc<SessionEventQueueCounters>,
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
        if !self.counters.reserve_event() {
            return SessionEventDelivery::DroppedFull;
        }

        match self.sender.try_send(event) {
            Ok(()) => {
                self.counters.observe_enqueued();
                SessionEventDelivery::Enqueued
            }
            Err(TrySendError::Full(_)) => {
                self.counters.observe_send_full();
                SessionEventDelivery::DroppedFull
            }
            Err(TrySendError::Disconnected(_)) => {
                self.counters.observe_receiver_closed();
                SessionEventDelivery::ReceiverClosed
            }
        }
    }
}

/// Result of non-blocking event polling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SessionEventReceive {
    Event(SessionEvent),
    Empty,
    Closed,
}

/// Sole consumer for a session's bounded control-event queue.
#[derive(Debug)]
pub struct SessionEventReceiver {
    receiver: Receiver<SessionEvent>,
    counters: Arc<SessionEventQueueCounters>,
}

impl SessionEventReceiver {
    pub fn try_recv(&self) -> SessionEventReceive {
        match self.receiver.try_recv() {
            Ok(event) => {
                self.counters.observe_dequeued();
                SessionEventReceive::Event(event)
            }
            Err(TryRecvError::Empty) => SessionEventReceive::Empty,
            Err(TryRecvError::Disconnected) => SessionEventReceive::Closed,
        }
    }

    pub fn observations(&self) -> SessionEventQueueObservations {
        self.counters.snapshot()
    }
}

pub(crate) fn session_event_channel(
    capacity_events: usize,
) -> (SessionEventSender, SessionEventReceiver) {
    assert!(
        capacity_events > 0,
        "session event capacity must be non-zero"
    );
    let (sender, receiver) = mpsc::sync_channel(capacity_events);
    let counters = Arc::new(SessionEventQueueCounters::new(capacity_events));
    (
        SessionEventSender {
            sender,
            counters: Arc::clone(&counters),
        },
        SessionEventReceiver { receiver, counters },
    )
}

#[cfg(test)]
mod tests {
    use pks_capture::{
        CaptureRuntimeFailure, CaptureRuntimeFailureClass, SourceGeneration, SourceKind,
        SourceRecoveryRequirement, StableSourceId,
    };
    use pks_frame::Platform;

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
        let (sender, receiver) = session_event_channel(1);

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
                depth_events: 1,
                peak_depth_event_count: 1,
                events_enqueued_total: 1,
                events_dropped_total: 1,
                receiver_closed_total: 0,
            }
        );
    }

    #[test]
    fn given_events_when_polled_then_fifo_order_and_depth_are_preserved() {
        let (sender, receiver) = session_event_channel(2);
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
        let (sender, receiver) = session_event_channel(1);
        drop(sender);

        assert_eq!(receiver.try_recv(), SessionEventReceive::Closed);
    }

    #[test]
    fn given_closed_receiver_when_publishing_then_drop_and_closure_are_counted() {
        let (sender, receiver) = session_event_channel(1);
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

//! Bounded capture-runtime failure and lifecycle event channel.

use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::authorization::CaptureError;
use super::identity::StableSourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(transparent)]
#[doc = "Identifies one appearance generation of a capture source across loss and reappearance."]
pub struct SourceGeneration(pub u32);

impl SourceGeneration {
    #[doc = "Provides the initial value for `SourceGeneration`."]
    pub const INITIAL: Self = Self(1);

    /// Returns the generation assigned after explicit rediscovery.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Selects the source lifecycle event kind used by PocketStation."]
pub enum SourceLifecycleEventKind {
    #[doc = "Indicates the source unavailable state for `SourceLifecycleEventKind`."]
    SourceUnavailable,
    #[doc = "Indicates the replacement observed state for `SourceLifecycleEventKind`."]
    ReplacementObserved,
    #[doc = "Indicates the permission changed state for `SourceLifecycleEventKind`."]
    PermissionChanged,
    #[doc = "Indicates the permission revoked state for `SourceLifecycleEventKind`."]
    PermissionRevoked,
    #[doc = "Indicates the source reappeared state for `SourceLifecycleEventKind`."]
    SourceReappeared,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Selects the source recovery requirement used by PocketStation."]
pub enum SourceRecoveryRequirement {
    #[doc = "Requires explicit rediscovery and new session recovery after source loss."]
    ExplicitRediscoveryAndNewSession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Classifies the platform, permission, source, or worker cause of a capture failure."]
pub enum CaptureRuntimeFailureClass {
    #[doc = "Classifies a failure at the source instance exited stage or component of `CaptureRuntimeFailureClass`."]
    SourceInstanceExited,
    #[doc = "Classifies a failure at the platform status stage or component of `CaptureRuntimeFailureClass`."]
    PlatformStatus {
        #[doc = "Preserves the platform or protocol status code reported by `PlatformStatus`."]
        status_code: i32,
    },
    #[doc = "Classifies a failure at the backend class stage or component of `CaptureRuntimeFailureClass`."]
    BackendClass {
        #[doc = "Contains the class owned or reported by `BackendClass`."]
        class: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Reports a capture runtime failure."]
pub struct CaptureRuntimeFailure {
    #[doc = "Names the operation that produced `CaptureRuntimeFailure`."]
    pub operation: &'static str,
    #[doc = "Contains the error class owned or reported by `CaptureRuntimeFailure`."]
    pub error_class: CaptureRuntimeFailureClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Classifies the observable source runtime event."]
pub enum SourceRuntimeEvent {
    #[doc = "Indicates the source unavailable state for `SourceRuntimeEvent`."]
    SourceUnavailable {
        #[doc = "Identifies the stable identifier recorded by `SourceUnavailable`."]
        stable_id: StableSourceId,
        #[doc = "Identifies the generation of the resource represented by `SourceUnavailable`."]
        generation: SourceGeneration,
        #[doc = "Declares the recovery action required after the source event in `SourceUnavailable`."]
        recovery_requirement: SourceRecoveryRequirement,
        #[doc = "Carries the failure reported by `SourceUnavailable`."]
        failure: CaptureRuntimeFailure,
    },
    #[doc = "Indicates the backend failure state for `SourceRuntimeEvent`."]
    BackendFailure {
        #[doc = "Identifies the stable identifier recorded by `BackendFailure`."]
        stable_id: StableSourceId,
        #[doc = "Identifies the generation of the resource represented by `BackendFailure`."]
        generation: SourceGeneration,
        #[doc = "Carries the failure reported by `BackendFailure`."]
        failure: CaptureRuntimeFailure,
    },
}

/// Maximum heap storage retained by one queued capture-runtime event.
///
/// Native error text and stable platform identities are variable-length
/// control-plane data. The bounded event channel rejects a larger event before
/// enqueue so a count bound also has a concrete byte bound.
pub const MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES: usize = 64 * 1024;

impl SourceRuntimeEvent {
    pub(crate) fn owned_bytes(&self) -> usize {
        let (stable_id, failure) = match self {
            Self::SourceUnavailable {
                stable_id, failure, ..
            }
            | Self::BackendFailure {
                stable_id, failure, ..
            } => (stable_id, failure),
        };
        let error_class_bytes = match &failure.error_class {
            CaptureRuntimeFailureClass::BackendClass { class } => class.capacity(),
            CaptureRuntimeFailureClass::SourceInstanceExited
            | CaptureRuntimeFailureClass::PlatformStatus { .. } => 0,
        };
        std::mem::size_of::<Self>()
            .saturating_add(stable_id.stable_key.capacity())
            .saturating_add(error_class_bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Reports whether a source-runtime event was delivered, dropped, or rejected."]
pub enum SourceRuntimeEventDelivery {
    #[doc = "Indicates the enqueued state for `SourceRuntimeEventDelivery`."]
    Enqueued,
    #[doc = "Indicates the dropped full state for `SourceRuntimeEventDelivery`."]
    DroppedFull,
    #[doc = "Indicates the dropped oversized state for `SourceRuntimeEventDelivery`."]
    DroppedOversized,
    #[doc = "Indicates the receiver closed state for `SourceRuntimeEventDelivery`."]
    ReceiverClosed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Reports the outcome of receiving a source-runtime event."]
pub enum SourceRuntimeEventReceive {
    #[doc = "Identifies the event state or stage represented by `SourceRuntimeEventReceive`."]
    Event(SourceRuntimeEvent),
    #[doc = "Represents an empty value or collection."]
    Empty,
    #[doc = "Reports that the underlying channel or resource is closed."]
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[doc = "Reports the source runtime event observations collected at an observation boundary."]
pub struct SourceRuntimeEventObservations {
    #[doc = "Sets the capacity event count available to `SourceRuntimeEventObservations`."]
    pub capacity_event_count: u64,
    #[doc = "Stores the maximum event owned size for `SourceRuntimeEventObservations`, in bytes."]
    pub maximum_event_owned_bytes: u64,
    #[doc = "Stores the maximum buffered owned size for `SourceRuntimeEventObservations`, in bytes."]
    pub maximum_buffered_owned_bytes: u64,
    #[doc = "Reports the depth events observed by `SourceRuntimeEventObservations`."]
    pub depth_events: u64,
    #[doc = "Stores the depth owned size for `SourceRuntimeEventObservations`, in bytes."]
    pub depth_owned_bytes: u64,
    #[doc = "Stores the peak depth owned size for `SourceRuntimeEventObservations`, in bytes."]
    pub peak_depth_owned_bytes: u64,
    #[doc = "Counts the total number of events enqueued observed by `SourceRuntimeEventObservations`."]
    pub events_enqueued_total: u64,
    #[doc = "Counts the total number of events dropped observed by `SourceRuntimeEventObservations`."]
    pub events_dropped_total: u64,
    #[doc = "Counts the total number of events dropped oversized observed by `SourceRuntimeEventObservations`."]
    pub events_dropped_oversized_total: u64,
}

#[derive(Debug, Default)]
struct SourceRuntimeEventCounters {
    capacity_event_count: u64,
    maximum_event_owned_bytes: u64,
    maximum_buffered_owned_bytes: u64,
    depth_events: AtomicU64,
    depth_owned_bytes: AtomicU64,
    peak_depth_owned_bytes: AtomicU64,
    events_enqueued_total: AtomicU64,
    events_dropped_total: AtomicU64,
    events_dropped_oversized_total: AtomicU64,
}

impl SourceRuntimeEventCounters {
    fn new(capacity_events: usize) -> Self {
        Self {
            capacity_event_count: capacity_events as u64,
            maximum_event_owned_bytes: MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES as u64,
            maximum_buffered_owned_bytes: capacity_events
                .saturating_mul(MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES)
                as u64,
            depth_events: AtomicU64::new(0),
            depth_owned_bytes: AtomicU64::new(0),
            peak_depth_owned_bytes: AtomicU64::new(0),
            events_enqueued_total: AtomicU64::new(0),
            events_dropped_total: AtomicU64::new(0),
            events_dropped_oversized_total: AtomicU64::new(0),
        }
    }

    fn reserve_owned_bytes(&self, owned_bytes: usize) -> bool {
        if owned_bytes > MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES {
            self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
            self.events_dropped_oversized_total
                .fetch_add(1, Ordering::Relaxed);
            return false;
        }
        let owned_bytes = owned_bytes as u64;
        let mut depth = self.depth_owned_bytes.load(Ordering::Relaxed);
        loop {
            let Some(next) = depth.checked_add(owned_bytes) else {
                self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
                return false;
            };
            if next > self.maximum_buffered_owned_bytes {
                self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
                return false;
            }
            match self.depth_owned_bytes.compare_exchange_weak(
                depth,
                next,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    self.peak_depth_owned_bytes
                        .fetch_max(next, Ordering::Relaxed);
                    return true;
                }
                Err(observed) => depth = observed,
            }
        }
    }

    fn cancel_owned_bytes(&self, owned_bytes: usize) {
        self.depth_owned_bytes
            .fetch_sub(owned_bytes as u64, Ordering::AcqRel);
    }
}

#[derive(Debug)]
struct QueuedSourceRuntimeEvent {
    event: SourceRuntimeEvent,
    owned_bytes: usize,
}

#[derive(Debug, Clone)]
#[doc = "Holds the ownership or bounded access represented by source runtime event observation handle."]
pub struct SourceRuntimeEventObservationHandle {
    counters: Arc<SourceRuntimeEventCounters>,
}

impl SourceRuntimeEventObservationHandle {
    #[doc = "Returns the observations exposed by `SourceRuntimeEventObservationHandle`."]
    pub fn observations(&self) -> SourceRuntimeEventObservations {
        SourceRuntimeEventObservations {
            capacity_event_count: self.counters.capacity_event_count,
            maximum_event_owned_bytes: self.counters.maximum_event_owned_bytes,
            maximum_buffered_owned_bytes: self.counters.maximum_buffered_owned_bytes,
            depth_events: self.counters.depth_events.load(Ordering::Acquire),
            depth_owned_bytes: self.counters.depth_owned_bytes.load(Ordering::Acquire),
            peak_depth_owned_bytes: self.counters.peak_depth_owned_bytes.load(Ordering::Relaxed),
            events_enqueued_total: self.counters.events_enqueued_total.load(Ordering::Relaxed),
            events_dropped_total: self.counters.events_dropped_total.load(Ordering::Relaxed),
            events_dropped_oversized_total: self
                .counters
                .events_dropped_oversized_total
                .load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone)]
#[doc = "Sends source runtime event values across its declared ownership boundary."]
pub struct SourceRuntimeEventSender {
    sender: std::sync::mpsc::SyncSender<QueuedSourceRuntimeEvent>,
    counters: Arc<SourceRuntimeEventCounters>,
}

impl SourceRuntimeEventSender {
    /// Publishes from a capture worker without blocking. When the bounded
    /// control channel is full, the newest event is dropped and counted.
    pub fn try_send(&self, event: SourceRuntimeEvent) -> SourceRuntimeEventDelivery {
        let owned_bytes = event.owned_bytes();
        if !self.counters.reserve_owned_bytes(owned_bytes) {
            return if owned_bytes > MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES {
                SourceRuntimeEventDelivery::DroppedOversized
            } else {
                SourceRuntimeEventDelivery::DroppedFull
            };
        }
        match self
            .sender
            .try_send(QueuedSourceRuntimeEvent { event, owned_bytes })
        {
            Ok(()) => {
                self.counters.depth_events.fetch_add(1, Ordering::Relaxed);
                self.counters
                    .events_enqueued_total
                    .fetch_add(1, Ordering::Relaxed);
                SourceRuntimeEventDelivery::Enqueued
            }
            Err(std::sync::mpsc::TrySendError::Full(queued)) => {
                self.counters.cancel_owned_bytes(queued.owned_bytes);
                self.counters
                    .events_dropped_total
                    .fetch_add(1, Ordering::Relaxed);
                SourceRuntimeEventDelivery::DroppedFull
            }
            Err(std::sync::mpsc::TrySendError::Disconnected(queued)) => {
                self.counters.cancel_owned_bytes(queued.owned_bytes);
                SourceRuntimeEventDelivery::ReceiverClosed
            }
        }
    }

    #[doc = "Returns the observations exposed by `SourceRuntimeEventSender`."]
    pub fn observations(&self) -> SourceRuntimeEventObservations {
        self.observation_handle().observations()
    }

    #[doc = "Returns a handle for reading observations from `SourceRuntimeEventSender`."]
    pub fn observation_handle(&self) -> SourceRuntimeEventObservationHandle {
        SourceRuntimeEventObservationHandle {
            counters: Arc::clone(&self.counters),
        }
    }
}

/// Publishes one exact post-open backend failure without introducing another
/// event queue or worker.
#[cfg(any(test, feature = "internal-testing", feature = "native-capture"))]
pub fn publish_backend_failure(
    sender: &SourceRuntimeEventSender,
    stable_id: StableSourceId,
    generation: SourceGeneration,
    operation: &'static str,
    error_class: CaptureRuntimeFailureClass,
) -> SourceRuntimeEventDelivery {
    sender.try_send(SourceRuntimeEvent::BackendFailure {
        stable_id,
        generation,
        failure: CaptureRuntimeFailure {
            operation,
            error_class,
        },
    })
}

#[derive(Debug)]
#[doc = "Receives source runtime event values across its declared ownership boundary."]
pub struct SourceRuntimeEventReceiver {
    receiver: std::sync::mpsc::Receiver<QueuedSourceRuntimeEvent>,
    counters: Arc<SourceRuntimeEventCounters>,
}

impl SourceRuntimeEventReceiver {
    #[doc = "Attempts to receive the next value from `SourceRuntimeEventReceiver` without waiting."]
    pub fn try_recv(&self) -> SourceRuntimeEventReceive {
        match self.receiver.try_recv() {
            Ok(queued) => {
                self.counters.depth_events.fetch_sub(1, Ordering::AcqRel);
                self.counters.cancel_owned_bytes(queued.owned_bytes);
                SourceRuntimeEventReceive::Event(queued.event)
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => SourceRuntimeEventReceive::Empty,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => SourceRuntimeEventReceive::Closed,
        }
    }

    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the observations exposed by `SourceRuntimeEventReceiver`."]
    pub fn observations(&self) -> SourceRuntimeEventObservations {
        self.observation_handle().observations()
    }

    #[doc = "Returns a handle for reading observations from `SourceRuntimeEventReceiver`."]
    pub fn observation_handle(&self) -> SourceRuntimeEventObservationHandle {
        SourceRuntimeEventObservationHandle {
            counters: Arc::clone(&self.counters),
        }
    }
}

#[doc = "Creates the bounded sender and receiver used for source runtime events."]
pub fn source_runtime_event_channel(
    capacity_events: usize,
) -> Result<(SourceRuntimeEventSender, SourceRuntimeEventReceiver), CaptureError> {
    if capacity_events == 0 {
        return Err(CaptureError::InvalidRuntimeEventCapacity);
    }
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity_events);
    let counters = Arc::new(SourceRuntimeEventCounters::new(capacity_events));
    Ok((
        SourceRuntimeEventSender {
            sender,
            counters: counters.clone(),
        },
        SourceRuntimeEventReceiver { receiver, counters },
    ))
}

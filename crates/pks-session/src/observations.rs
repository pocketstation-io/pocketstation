use std::sync::atomic::{AtomicU64, Ordering};

/// Point-in-time observations for a session's bounded control-event queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionEventQueueObservations {
    pub capacity_event_count: u64,
    pub depth_events: u64,
    pub peak_depth_event_count: u64,
    pub events_enqueued_total: u64,
    pub events_dropped_total: u64,
    pub receiver_closed_total: u64,
}

#[derive(Debug)]
pub(crate) struct SessionEventQueueCounters {
    capacity_event_count: u64,
    depth_events: AtomicU64,
    peak_depth_event_count: AtomicU64,
    events_enqueued_total: AtomicU64,
    events_dropped_total: AtomicU64,
    receiver_closed_total: AtomicU64,
}

impl SessionEventQueueCounters {
    pub(crate) fn new(capacity_events: usize) -> Self {
        Self {
            capacity_event_count: capacity_events as u64,
            depth_events: AtomicU64::new(0),
            peak_depth_event_count: AtomicU64::new(0),
            events_enqueued_total: AtomicU64::new(0),
            events_dropped_total: AtomicU64::new(0),
            receiver_closed_total: AtomicU64::new(0),
        }
    }

    pub(crate) fn reserve_event(&self) -> bool {
        let mut depth_events = self.depth_events.load(Ordering::Relaxed);
        loop {
            if depth_events >= self.capacity_event_count {
                self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
                return false;
            }

            match self.depth_events.compare_exchange_weak(
                depth_events,
                depth_events + 1,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    self.peak_depth_event_count
                        .fetch_max(depth_events + 1, Ordering::Relaxed);
                    return true;
                }
                Err(observed_depth_events) => depth_events = observed_depth_events,
            }
        }
    }

    pub(crate) fn observe_enqueued(&self) {
        self.events_enqueued_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_send_full(&self) {
        self.cancel_reservation();
        self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_receiver_closed(&self) {
        self.cancel_reservation();
        self.events_dropped_total.fetch_add(1, Ordering::Relaxed);
        self.receiver_closed_total.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn observe_dequeued(&self) {
        let previous_depth_events = self.depth_events.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous_depth_events > 0);
    }

    pub(crate) fn snapshot(&self) -> SessionEventQueueObservations {
        SessionEventQueueObservations {
            capacity_event_count: self.capacity_event_count,
            depth_events: self.depth_events.load(Ordering::Acquire),
            peak_depth_event_count: self.peak_depth_event_count.load(Ordering::Relaxed),
            events_enqueued_total: self.events_enqueued_total.load(Ordering::Relaxed),
            events_dropped_total: self.events_dropped_total.load(Ordering::Relaxed),
            receiver_closed_total: self.receiver_closed_total.load(Ordering::Relaxed),
        }
    }

    fn cancel_reservation(&self) {
        let previous_depth_events = self.depth_events.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(previous_depth_events > 0);
    }
}

use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

use super::{ConnectorError, ConnectorReadiness, ConnectorReadinessTransitionError};

#[derive(Clone)]
pub struct ConnectorObservationHandle {
    inner: Arc<ConnectorObservationState>,
}

struct ConnectorObservationState {
    readiness: AtomicU8,
    readiness_transitions_total: AtomicU64,
    items_received_total: AtomicU64,
    items_delivered_total: AtomicU64,
    items_dropped_total: AtomicU64,
    retry_attempts_total: AtomicU64,
    reconnects_total: AtomicU64,
    failures_total: AtomicU64,
    last_error: Mutex<Option<ConnectorError>>,
}

impl ConnectorObservationHandle {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ConnectorObservationState {
                readiness: AtomicU8::new(ConnectorReadiness::Starting as u8),
                readiness_transitions_total: AtomicU64::new(0),
                items_received_total: AtomicU64::new(0),
                items_delivered_total: AtomicU64::new(0),
                items_dropped_total: AtomicU64::new(0),
                retry_attempts_total: AtomicU64::new(0),
                reconnects_total: AtomicU64::new(0),
                failures_total: AtomicU64::new(0),
                last_error: Mutex::new(None),
            }),
        }
    }

    pub fn transition(
        &self,
        requested: ConnectorReadiness,
    ) -> Result<bool, ConnectorReadinessTransitionError> {
        loop {
            let observed = self.inner.readiness.load(Ordering::Acquire);
            let current = ConnectorReadiness::from_u8(observed);
            if current == requested {
                return Ok(false);
            }
            if !current.can_transition_to(requested) {
                return Err(ConnectorReadinessTransitionError { current, requested });
            }
            if self
                .inner
                .readiness
                .compare_exchange(
                    observed,
                    requested as u8,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_ok()
            {
                increment(&self.inner.readiness_transitions_total, 1);
                if requested == ConnectorReadiness::Reconnecting {
                    increment(&self.inner.reconnects_total, 1);
                }
                return Ok(true);
            }
        }
    }

    pub fn record_received(&self, item_count: u64) {
        increment(&self.inner.items_received_total, item_count);
    }

    pub fn record_delivered(&self, item_count: u64) {
        increment(&self.inner.items_delivered_total, item_count);
    }

    pub fn record_dropped(&self, item_count: u64) {
        increment(&self.inner.items_dropped_total, item_count);
    }

    pub fn record_retry(&self) {
        increment(&self.inner.retry_attempts_total, 1);
    }

    pub fn record_failure(&self, error: ConnectorError) -> Result<(), ConnectorObservationError> {
        let mut last_error = self
            .inner
            .last_error
            .lock()
            .map_err(|_| ConnectorObservationError::StateUnavailable)?;
        *last_error = Some(error);
        increment(&self.inner.failures_total, 1);
        Ok(())
    }

    pub fn snapshot(&self) -> Result<ConnectorObservations, ConnectorObservationError> {
        let last_error = self
            .inner
            .last_error
            .lock()
            .map_err(|_| ConnectorObservationError::StateUnavailable)?
            .clone();
        Ok(ConnectorObservations {
            readiness: ConnectorReadiness::from_u8(self.inner.readiness.load(Ordering::Acquire)),
            readiness_transitions_total: self
                .inner
                .readiness_transitions_total
                .load(Ordering::Relaxed),
            items_received_total: self.inner.items_received_total.load(Ordering::Relaxed),
            items_delivered_total: self.inner.items_delivered_total.load(Ordering::Relaxed),
            items_dropped_total: self.inner.items_dropped_total.load(Ordering::Relaxed),
            retry_attempts_total: self.inner.retry_attempts_total.load(Ordering::Relaxed),
            reconnects_total: self.inner.reconnects_total.load(Ordering::Relaxed),
            failures_total: self.inner.failures_total.load(Ordering::Relaxed),
            last_error,
        })
    }
}

impl Default for ConnectorObservationHandle {
    fn default() -> Self {
        Self::new()
    }
}

fn increment(counter: &AtomicU64, amount: u64) {
    let _ = counter.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
        Some(value.saturating_add(amount))
    });
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorObservations {
    pub readiness: ConnectorReadiness,
    pub readiness_transitions_total: u64,
    pub items_received_total: u64,
    pub items_delivered_total: u64,
    pub items_dropped_total: u64,
    pub retry_attempts_total: u64,
    pub reconnects_total: u64,
    pub failures_total: u64,
    pub last_error: Option<ConnectorError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ConnectorObservationError {
    #[error("connector observation state is unavailable")]
    StateUnavailable,
}

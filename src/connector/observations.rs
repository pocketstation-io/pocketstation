use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::endpoint::EndpointDriverObservationHandle;
use crate::{EndpointDriverObservations, EndpointId};

use super::{
    ConnectorDeliveryReadiness, ConnectorError, ConnectorErrorCode, ConnectorHealth,
    ConnectorRecovery, ConnectorServiceStatus,
};

#[derive(Clone)]
#[doc = "Owns bounded access to connector observation."]
pub struct ConnectorObservationHandle {
    inner: Arc<ConnectorObservationState>,
}

struct ConnectorObservationState {
    created_at: Instant,
    status: Mutex<ConnectorServiceStatus>,
    status_transitions_total: AtomicU64,
    retry_attempts_total: AtomicU64,
    reconnects_total: AtomicU64,
    failures_total: AtomicU64,
    last_error: Mutex<Option<ConnectorError>>,
}

impl ConnectorObservationHandle {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(ConnectorObservationState {
                created_at: Instant::now(),
                status: Mutex::new(ConnectorServiceStatus {
                    delivery_readiness: ConnectorDeliveryReadiness::NotReady,
                    health: ConnectorHealth::Healthy,
                    recovery: ConnectorRecovery::Idle,
                    readiness_reason_code: None,
                    health_reason_code: None,
                    recovery_reason_code: None,
                    revision: 0,
                    last_transition_elapsed_ns: 0,
                }),
                status_transitions_total: AtomicU64::new(0),
                retry_attempts_total: AtomicU64::new(0),
                reconnects_total: AtomicU64::new(0),
                failures_total: AtomicU64::new(0),
                last_error: Mutex::new(None),
            }),
        }
    }

    #[doc = "Returns a point-in-time snapshot of `ConnectorObservationHandle`."]
    pub fn snapshot(&self) -> Result<ConnectorObservations, ConnectorObservationError> {
        let service_status = self
            .inner
            .status
            .lock()
            .map_err(|_| ConnectorObservationError::StateUnavailable)?
            .clone();
        let last_error = self
            .inner
            .last_error
            .lock()
            .map_err(|_| ConnectorObservationError::StateUnavailable)?
            .clone();
        Ok(ConnectorObservations {
            service_status,
            status_transitions_total: self.inner.status_transitions_total.load(Ordering::Relaxed),
            retry_attempts_total: self.inner.retry_attempts_total.load(Ordering::Relaxed),
            reconnects_total: self.inner.reconnects_total.load(Ordering::Relaxed),
            failures_total: self.inner.failures_total.load(Ordering::Relaxed),
            last_error,
        })
    }

    pub(crate) fn set_delivery_readiness(
        &self,
        readiness: ConnectorDeliveryReadiness,
        reason_code: Option<ConnectorErrorCode>,
    ) -> bool {
        let mut status = self.lock_status();
        if status.delivery_readiness == readiness && status.readiness_reason_code == reason_code {
            return false;
        }
        status.delivery_readiness = readiness;
        status.readiness_reason_code = reason_code;
        self.finish_transition(&mut status);
        true
    }

    pub(crate) fn set_health(
        &self,
        health: ConnectorHealth,
        reason_code: Option<ConnectorErrorCode>,
    ) -> bool {
        let mut status = self.lock_status();
        if status.health == health && status.health_reason_code == reason_code {
            return false;
        }
        status.health = health;
        status.health_reason_code = reason_code;
        self.finish_transition(&mut status);
        true
    }

    pub(crate) fn set_recovery(
        &self,
        recovery: ConnectorRecovery,
        reason_code: Option<ConnectorErrorCode>,
    ) -> bool {
        let mut status = self.lock_status();
        if status.recovery == recovery && status.recovery_reason_code == reason_code {
            return false;
        }
        if recovery == ConnectorRecovery::Reconnecting {
            increment(&self.inner.reconnects_total, 1);
            status.delivery_readiness = ConnectorDeliveryReadiness::NotReady;
        }
        status.recovery = recovery;
        status.recovery_reason_code = reason_code;
        self.finish_transition(&mut status);
        true
    }

    pub(crate) fn record_retry(&self) {
        increment(&self.inner.retry_attempts_total, 1);
    }

    pub(crate) fn record_failure(
        &self,
        error: ConnectorError,
    ) -> Result<(), ConnectorObservationError> {
        *self
            .inner
            .last_error
            .lock()
            .map_err(|_| ConnectorObservationError::StateUnavailable)? = Some(error);
        increment(&self.inner.failures_total, 1);
        Ok(())
    }

    fn lock_status(&self) -> std::sync::MutexGuard<'_, ConnectorServiceStatus> {
        self.inner
            .status
            .lock()
            .unwrap_or_else(|error| error.into_inner())
    }

    fn finish_transition(&self, status: &mut ConnectorServiceStatus) {
        status.revision = status.revision.saturating_add(1);
        status.last_transition_elapsed_ns =
            u64::try_from(self.inner.created_at.elapsed().as_nanos()).unwrap_or(u64::MAX);
        increment(&self.inner.status_transitions_total, 1);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Reports the connector observations collected at an observation boundary."]
pub struct ConnectorObservations {
    #[doc = "Stores the service status associated with `ConnectorObservations`."]
    pub service_status: ConnectorServiceStatus,
    #[doc = "Counts the total number of status transitions observed by `ConnectorObservations`."]
    pub status_transitions_total: u64,
    #[doc = "Counts the total number of retry attempts observed by `ConnectorObservations`."]
    pub retry_attempts_total: u64,
    #[doc = "Counts the total number of reconnects observed by `ConnectorObservations`."]
    pub reconnects_total: u64,
    #[doc = "Counts the total number of failures observed by `ConnectorObservations`."]
    pub failures_total: u64,
    #[doc = "Carries the last error reported by `ConnectorObservations`."]
    pub last_error: Option<ConnectorError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Reports the connector runtime observations collected at an observation boundary."]
pub struct ConnectorRuntimeObservations {
    #[doc = "Identifies the endpoint associated with `ConnectorRuntimeObservations`."]
    pub endpoint_ids: Arc<[EndpointId]>,
    #[doc = "Stores the connector associated with `ConnectorRuntimeObservations`."]
    pub connector: ConnectorObservations,
    #[doc = "Stores the endpoint associated with `ConnectorRuntimeObservations`."]
    pub endpoint: EndpointDriverObservations,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as connector observation error."]
pub enum ConnectorObservationError {
    #[error("connector observation state is unavailable")]
    #[doc = "Reports state unavailable."]
    StateUnavailable,
}

#[derive(Clone)]
pub(crate) struct ConnectorObservationStore {
    inner: Arc<Mutex<HashMap<EndpointId, ConnectorObservationEntry>>>,
}

#[derive(Clone)]
struct ConnectorObservationEntry {
    endpoint_ids: Arc<[EndpointId]>,
    connector: ConnectorObservationHandle,
    endpoint: EndpointDriverObservationHandle,
}

impl ConnectorObservationStore {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) fn install(
        &self,
        endpoint_ids: Arc<[EndpointId]>,
        connector: ConnectorObservationHandle,
        endpoint: EndpointDriverObservationHandle,
    ) {
        let mut entries = self.inner.lock().unwrap_or_else(|error| error.into_inner());
        for endpoint_id in endpoint_ids.iter().copied() {
            entries.insert(
                endpoint_id,
                ConnectorObservationEntry {
                    endpoint_ids: Arc::clone(&endpoint_ids),
                    connector: connector.clone(),
                    endpoint: endpoint.clone(),
                },
            );
        }
    }

    pub(crate) fn observation(
        &self,
        endpoint_id: EndpointId,
    ) -> Option<ConnectorObservationHandle> {
        self.inner
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .get(&endpoint_id)
            .map(|entry| entry.connector.clone())
    }

    pub(crate) fn snapshots(
        &self,
    ) -> Result<Vec<ConnectorRuntimeObservations>, ConnectorObservationError> {
        let entries = self.inner.lock().unwrap_or_else(|error| error.into_inner());
        let mut unique = Vec::<ConnectorObservationEntry>::new();
        for entry in entries.values() {
            if unique
                .iter()
                .any(|known| Arc::ptr_eq(&known.endpoint_ids, &entry.endpoint_ids))
            {
                continue;
            }
            unique.push(entry.clone());
        }
        unique
            .into_iter()
            .map(|entry| {
                Ok(ConnectorRuntimeObservations {
                    endpoint_ids: entry.endpoint_ids,
                    connector: entry.connector.snapshot()?,
                    endpoint: entry.endpoint.snapshot(),
                })
            })
            .collect()
    }
}

fn increment(counter: &AtomicU64, amount: u64) {
    let _ = counter.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
        Some(value.saturating_add(amount))
    });
}

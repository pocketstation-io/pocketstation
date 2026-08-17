use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use crate::EndpointDriverObservations;

use super::supervisor::ConnectorWorkerState;
use crate::connector::{
    ConnectorDeliveryReadiness, ConnectorError, ConnectorErrorCode, ConnectorHealth,
    ConnectorObservationError, ConnectorReadinessPolicy, ConnectorRecovery,
};

#[derive(Clone)]
pub struct ConnectorContext {
    pub(super) stop: ConnectorStopToken,
    pub(super) state: ConnectorWorkerState,
    pub(super) readiness_policy: ConnectorReadinessPolicy,
    pub(super) readiness_probes: Arc<Mutex<ReadinessProbeState>>,
}

#[derive(Default)]
pub(super) struct ReadinessProbeState {
    consecutive_successes: u32,
    consecutive_failures: u32,
}

impl ConnectorContext {
    pub fn is_stop_requested(&self) -> bool {
        self.stop.is_requested()
    }

    pub fn wait_for_stop(&self, timeout: Duration) -> bool {
        self.stop.wait_timeout(timeout)
    }

    pub fn set_ready(&self) -> bool {
        self.state
            .connector()
            .set_delivery_readiness(ConnectorDeliveryReadiness::Ready, None)
    }

    pub fn set_not_ready(&self, reason_code: Option<ConnectorErrorCode>) -> bool {
        self.state
            .connector()
            .set_delivery_readiness(ConnectorDeliveryReadiness::NotReady, reason_code)
    }

    pub fn set_degraded(&self, reason_code: ConnectorErrorCode) -> bool {
        self.state
            .connector()
            .set_health(ConnectorHealth::Degraded, Some(reason_code))
    }

    pub fn set_healthy(&self) -> bool {
        self.state
            .connector()
            .set_health(ConnectorHealth::Healthy, None)
    }

    pub fn set_reconnecting(&self, reason_code: ConnectorErrorCode) -> bool {
        self.state
            .connector()
            .set_recovery(ConnectorRecovery::Reconnecting, Some(reason_code))
    }

    pub fn set_connected(&self) -> bool {
        self.state
            .connector()
            .set_recovery(ConnectorRecovery::Idle, None)
    }

    pub fn report_readiness_success(&self) -> bool {
        let reached_threshold = {
            let mut probes = self
                .readiness_probes
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            probes.consecutive_failures = 0;
            probes.consecutive_successes = probes.consecutive_successes.saturating_add(1);
            probes.consecutive_successes >= self.readiness_policy.success_threshold()
        };
        if !reached_threshold {
            return false;
        }
        let _ = self.set_healthy();
        self.set_ready()
    }

    pub fn report_readiness_failure(&self, reason_code: ConnectorErrorCode) -> bool {
        let reached_threshold = {
            let mut probes = self
                .readiness_probes
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            probes.consecutive_successes = 0;
            probes.consecutive_failures = probes.consecutive_failures.saturating_add(1);
            probes.consecutive_failures >= self.readiness_policy.failure_threshold()
        };
        if !reached_threshold {
            return false;
        }
        let _ = self.set_degraded(reason_code.clone());
        self.set_not_ready(Some(reason_code))
    }

    pub fn record_frame_received(&self, amount: u64) {
        self.state.endpoint().record_received(amount);
    }

    pub fn record_frame_delivered(&self, amount: u64) {
        self.state.endpoint().record_delivered(amount);
    }

    pub fn record_frame_dropped(&self, amount: u64) {
        self.state.endpoint().record_dropped(amount);
    }

    pub fn record_discontinuity(&self, amount: u64) {
        self.state.endpoint().record_discontinuity(amount);
    }

    pub fn record_retry(&self) {
        self.state.connector().record_retry();
    }

    pub fn record_failure(&self, error: ConnectorError) -> Result<(), ConnectorObservationError> {
        self.state.endpoint().record_failure(1);
        self.state.connector().record_failure(error)
    }

    pub fn endpoint_observations(&self) -> EndpointDriverObservations {
        self.state.endpoint().snapshot()
    }
}

#[derive(Clone)]
pub(super) struct ConnectorStopToken {
    inner: Arc<ConnectorStopState>,
}

struct ConnectorStopState {
    requested: AtomicBool,
    wait_lock: Mutex<()>,
    wait: Condvar,
}

impl ConnectorStopToken {
    pub(super) fn new() -> Self {
        Self {
            inner: Arc::new(ConnectorStopState {
                requested: AtomicBool::new(false),
                wait_lock: Mutex::new(()),
                wait: Condvar::new(),
            }),
        }
    }

    pub(super) fn request(&self) -> bool {
        let changed = !self.inner.requested.swap(true, Ordering::AcqRel);
        self.inner.wait.notify_all();
        changed
    }

    pub(super) fn is_requested(&self) -> bool {
        self.inner.requested.load(Ordering::Acquire)
    }

    pub(super) fn wait_timeout(&self, timeout: Duration) -> bool {
        if self.is_requested() {
            return true;
        }
        let guard = self
            .inner
            .wait_lock
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let _guard = self
            .inner
            .wait
            .wait_timeout_while(guard, timeout, |_| !self.is_requested())
            .unwrap_or_else(|error| error.into_inner());
        self.is_requested()
    }
}

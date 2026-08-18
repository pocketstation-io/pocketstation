use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::endpoint::EndpointDriverObservationHandle;
use crate::{EndpointShutdownMode, EndpointStartGate};

use super::coordination::ConnectorStopToken;
use crate::connector::{
    ConnectorDeliveryReadiness, ConnectorError, ConnectorErrorStage, ConnectorObservationHandle,
    ConnectorReadinessPolicy, ConnectorRecovery,
};

pub(super) const START_GATE_POLL_INTERVAL: Duration = Duration::from_millis(1);

#[derive(Clone)]
pub(super) struct ConnectorWorkerState {
    connector: ConnectorObservationHandle,
    endpoint: EndpointDriverObservationHandle,
    terminal_error: Arc<Mutex<Option<ConnectorError>>>,
}

impl ConnectorWorkerState {
    pub(super) fn new(
        connector: ConnectorObservationHandle,
        endpoint: EndpointDriverObservationHandle,
    ) -> Self {
        Self {
            connector,
            endpoint,
            terminal_error: Arc::new(Mutex::new(None)),
        }
    }

    pub(super) fn connector(&self) -> &ConnectorObservationHandle {
        &self.connector
    }

    pub(super) fn endpoint(&self) -> &EndpointDriverObservationHandle {
        &self.endpoint
    }

    pub(super) fn record_terminal(&self, error: ConnectorError) {
        let first_terminal = {
            let mut terminal = self
                .terminal_error
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if terminal.is_some() {
                false
            } else {
                *terminal = Some(error.clone());
                true
            }
        };
        if first_terminal {
            self.endpoint.record_failure(1);
            let reason = error.code().clone();
            let _ = self
                .connector
                .set_delivery_readiness(ConnectorDeliveryReadiness::NotReady, Some(reason.clone()));
            let _ = self
                .connector
                .set_health(crate::connector::ConnectorHealth::Degraded, Some(reason));
            let _ = self.connector.record_failure(error);
        }
    }

    pub(super) fn terminal_error(&self) -> Option<ConnectorError> {
        self.terminal_error
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    pub(super) fn mark_stopping(&self) {
        if self.terminal_error().is_none() {
            let _ = self
                .connector
                .set_delivery_readiness(ConnectorDeliveryReadiness::NotReady, None);
        }
        let _ = self.connector.set_recovery(ConnectorRecovery::Idle, None);
    }
}

pub(super) fn wait_for_start_gate(
    stop: &ConnectorStopToken,
    start_gate: &EndpointStartGate,
) -> bool {
    while !start_gate.is_open() {
        if stop.wait_timeout(START_GATE_POLL_INTERVAL) {
            return false;
        }
    }
    true
}

pub(super) fn supervise_startup_readiness(
    stop: &ConnectorStopToken,
    state: &ConnectorWorkerState,
    start_gate: &EndpointStartGate,
    policy: ConnectorReadinessPolicy,
) {
    if !wait_for_start_gate(stop, start_gate) {
        return;
    }
    let deadline = Instant::now() + policy.startup_timeout();
    loop {
        if stop.is_requested() {
            return;
        }
        match state.connector().snapshot() {
            Ok(snapshot) if snapshot.service_status.accepts_delivery() => return,
            Ok(_) => {}
            Err(error) => {
                state.record_terminal(internal_connector_error(
                    "core.observation_unavailable",
                    ConnectorErrorStage::Readiness,
                    error.to_string(),
                ));
                stop.request(EndpointShutdownMode::Abort);
                return;
            }
        }
        let now = Instant::now();
        if now >= deadline {
            state.record_terminal(internal_connector_error(
                "core.readiness_timeout",
                ConnectorErrorStage::Readiness,
                "connector did not become ready before its declared startup deadline",
            ));
            stop.request(EndpointShutdownMode::Abort);
            return;
        }
        let wait = policy
            .probe_interval()
            .min(deadline.saturating_duration_since(now));
        if stop.wait_timeout(wait) {
            return;
        }
    }
}

pub(super) fn internal_connector_error(
    code: &'static str,
    stage: ConnectorErrorStage,
    message: impl Into<String>,
) -> ConnectorError {
    ConnectorError::internal(code, stage, message)
}

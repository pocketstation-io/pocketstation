use super::ConnectorErrorCode;
use std::time::Duration;

pub const MAX_CONNECTOR_READINESS_THRESHOLD: u32 = 1_000_000;
pub const MAX_CONNECTOR_READINESS_TIMEOUT: Duration = Duration::from_secs(86_400);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConnectorDeliveryReadiness {
    NotReady = 0,
    Ready = 1,
}

impl ConnectorDeliveryReadiness {
    pub const fn accepts_delivery(self) -> bool {
        matches!(self, Self::Ready)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConnectorHealth {
    Healthy = 0,
    Degraded = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConnectorRecovery {
    Idle = 0,
    Reconnecting = 1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorServiceStatus {
    pub(crate) delivery_readiness: ConnectorDeliveryReadiness,
    pub(crate) health: ConnectorHealth,
    pub(crate) recovery: ConnectorRecovery,
    pub(crate) readiness_reason_code: Option<ConnectorErrorCode>,
    pub(crate) health_reason_code: Option<ConnectorErrorCode>,
    pub(crate) recovery_reason_code: Option<ConnectorErrorCode>,
    pub(crate) revision: u64,
    pub(crate) last_transition_elapsed_ns: u64,
}

impl ConnectorServiceStatus {
    pub const fn delivery_readiness(&self) -> ConnectorDeliveryReadiness {
        self.delivery_readiness
    }

    pub const fn health(&self) -> ConnectorHealth {
        self.health
    }

    pub const fn recovery(&self) -> ConnectorRecovery {
        self.recovery
    }

    pub const fn readiness_reason_code(&self) -> Option<&ConnectorErrorCode> {
        self.readiness_reason_code.as_ref()
    }

    pub const fn health_reason_code(&self) -> Option<&ConnectorErrorCode> {
        self.health_reason_code.as_ref()
    }

    pub const fn recovery_reason_code(&self) -> Option<&ConnectorErrorCode> {
        self.recovery_reason_code.as_ref()
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub const fn last_transition_elapsed_ns(&self) -> u64 {
        self.last_transition_elapsed_ns
    }

    pub const fn accepts_delivery(&self) -> bool {
        self.delivery_readiness.accepts_delivery()
            && matches!(self.recovery, ConnectorRecovery::Idle)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectorReadinessPolicy {
    startup_timeout: Duration,
    probe_interval: Duration,
    success_threshold: u32,
    failure_threshold: u32,
}

impl ConnectorReadinessPolicy {
    pub fn new(
        startup_timeout: Duration,
        probe_interval: Duration,
        success_threshold: u32,
        failure_threshold: u32,
    ) -> Result<Self, ConnectorReadinessPolicyError> {
        if startup_timeout.is_zero()
            || startup_timeout > MAX_CONNECTOR_READINESS_TIMEOUT
            || probe_interval.is_zero()
            || probe_interval > startup_timeout
        {
            return Err(ConnectorReadinessPolicyError::InvalidDeadline);
        }
        if success_threshold == 0
            || failure_threshold == 0
            || success_threshold > MAX_CONNECTOR_READINESS_THRESHOLD
            || failure_threshold > MAX_CONNECTOR_READINESS_THRESHOLD
        {
            return Err(ConnectorReadinessPolicyError::InvalidThreshold);
        }
        Ok(Self {
            startup_timeout,
            probe_interval,
            success_threshold,
            failure_threshold,
        })
    }

    pub const fn startup_timeout(self) -> Duration {
        self.startup_timeout
    }

    pub const fn probe_interval(self) -> Duration {
        self.probe_interval
    }

    pub const fn success_threshold(self) -> u32 {
        self.success_threshold
    }

    pub const fn failure_threshold(self) -> u32 {
        self.failure_threshold
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ConnectorReadinessPolicyError {
    #[error("connector readiness timeout and probe interval must be finite and non-zero")]
    InvalidDeadline,
    #[error("connector readiness thresholds must be finite and non-zero")]
    InvalidThreshold,
}

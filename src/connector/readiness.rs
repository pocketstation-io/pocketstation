use std::time::Duration;

#[doc = "Sets the maximum supported connector readiness threshold."]
pub const MAX_CONNECTOR_READINESS_THRESHOLD: u32 = 1_000_000;
#[doc = "Sets the maximum supported connector readiness timeout."]
pub const MAX_CONNECTOR_READINESS_TIMEOUT: Duration = Duration::from_secs(86_400);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Configures connector readiness."]
pub struct ConnectorReadinessPolicy {
    startup_timeout: Duration,
    probe_interval: Duration,
    success_threshold: u32,
    failure_threshold: u32,
}

impl ConnectorReadinessPolicy {
    #[doc = "Creates a new `ConnectorReadinessPolicy`."]
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

    #[doc = "Returns the startup timeout associated with `ConnectorReadinessPolicy`."]
    pub const fn startup_timeout(self) -> Duration {
        self.startup_timeout
    }

    #[doc = "Returns the probe interval associated with `ConnectorReadinessPolicy`."]
    pub const fn probe_interval(self) -> Duration {
        self.probe_interval
    }

    #[doc = "Returns the success threshold associated with `ConnectorReadinessPolicy`."]
    pub const fn success_threshold(self) -> u32 {
        self.success_threshold
    }

    #[doc = "Returns the failure threshold associated with `ConnectorReadinessPolicy`."]
    pub const fn failure_threshold(self) -> u32 {
        self.failure_threshold
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as connector readiness policy error."]
pub enum ConnectorReadinessPolicyError {
    #[error("connector readiness timeout and probe interval must be finite and non-zero")]
    #[doc = "Reports invalid deadline."]
    InvalidDeadline,
    #[error("connector readiness thresholds must be finite and non-zero")]
    #[doc = "Reports invalid threshold."]
    InvalidThreshold,
}

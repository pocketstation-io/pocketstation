use crate::graph::EdgeContract;

pub const MAX_CONNECTOR_WORKER_QUEUE_ITEMS: usize = 1_048_576;
pub const MAX_CONNECTOR_ATTEMPTS: u32 = 1_000;
pub const MAX_CONNECTOR_TIMEOUT_MS: u64 = 86_400_000;
pub const MAX_CONNECTOR_READINESS_THRESHOLD: u32 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectorDeliveryPolicy {
    input_edge: EdgeContract,
    worker_queue_capacity_items: usize,
}

impl ConnectorDeliveryPolicy {
    pub fn new(
        input_edge: EdgeContract,
        worker_queue_capacity_items: usize,
    ) -> Result<Self, ConnectorPolicyError> {
        if worker_queue_capacity_items == 0
            || worker_queue_capacity_items > MAX_CONNECTOR_WORKER_QUEUE_ITEMS
        {
            return Err(ConnectorPolicyError::InvalidWorkerQueueCapacity);
        }
        Ok(Self {
            input_edge,
            worker_queue_capacity_items,
        })
    }

    pub const fn input_edge(self) -> EdgeContract {
        self.input_edge
    }

    pub const fn worker_queue_capacity_items(self) -> usize {
        self.worker_queue_capacity_items
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectorRetryPolicy {
    max_attempts: u32,
    attempt_timeout_ms: u64,
    initial_delay_ms: u64,
    maximum_delay_ms: u64,
    maximum_elapsed_ms: u64,
    multiplier_milli: u32,
    jitter_percent: u8,
}

impl ConnectorRetryPolicy {
    pub const fn disabled() -> Self {
        Self {
            max_attempts: 1,
            attempt_timeout_ms: 0,
            initial_delay_ms: 0,
            maximum_delay_ms: 0,
            maximum_elapsed_ms: 0,
            multiplier_milli: 1_000,
            jitter_percent: 0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        max_attempts: u32,
        attempt_timeout_ms: u64,
        initial_delay_ms: u64,
        maximum_delay_ms: u64,
        maximum_elapsed_ms: u64,
        multiplier_milli: u32,
        jitter_percent: u8,
    ) -> Result<Self, ConnectorPolicyError> {
        let policy = Self {
            max_attempts,
            attempt_timeout_ms,
            initial_delay_ms,
            maximum_delay_ms,
            maximum_elapsed_ms,
            multiplier_milli,
            jitter_percent,
        };
        policy.validate()?;
        Ok(policy)
    }

    pub const fn max_attempts(self) -> u32 {
        self.max_attempts
    }

    pub const fn attempt_timeout_ms(self) -> u64 {
        self.attempt_timeout_ms
    }

    pub const fn initial_delay_ms(self) -> u64 {
        self.initial_delay_ms
    }

    pub const fn maximum_delay_ms(self) -> u64 {
        self.maximum_delay_ms
    }

    pub const fn maximum_elapsed_ms(self) -> u64 {
        self.maximum_elapsed_ms
    }

    pub const fn multiplier_milli(self) -> u32 {
        self.multiplier_milli
    }

    pub const fn jitter_percent(self) -> u8 {
        self.jitter_percent
    }

    fn validate(self) -> Result<(), ConnectorPolicyError> {
        if self.max_attempts == 0 || self.max_attempts > MAX_CONNECTOR_ATTEMPTS {
            return Err(ConnectorPolicyError::InvalidMaxAttempts);
        }
        if [
            self.attempt_timeout_ms,
            self.initial_delay_ms,
            self.maximum_delay_ms,
            self.maximum_elapsed_ms,
        ]
        .into_iter()
        .any(|value| value > MAX_CONNECTOR_TIMEOUT_MS)
        {
            return Err(ConnectorPolicyError::InvalidTimeout);
        }
        if self.max_attempts > 1 {
            if self.attempt_timeout_ms == 0
                || self.initial_delay_ms == 0
                || self.maximum_delay_ms < self.initial_delay_ms
                || self.maximum_elapsed_ms < self.attempt_timeout_ms
            {
                return Err(ConnectorPolicyError::InvalidRetryWindow);
            }
            if !(1_000..=10_000).contains(&self.multiplier_milli) {
                return Err(ConnectorPolicyError::InvalidMultiplier);
            }
        }
        if self.jitter_percent > 100 {
            return Err(ConnectorPolicyError::InvalidJitter);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectorReadinessPolicy {
    startup_timeout_ms: u64,
    probe_interval_ms: u64,
    success_threshold: u32,
    failure_threshold: u32,
}

impl ConnectorReadinessPolicy {
    pub fn new(
        startup_timeout_ms: u64,
        probe_interval_ms: u64,
        success_threshold: u32,
        failure_threshold: u32,
    ) -> Result<Self, ConnectorPolicyError> {
        if startup_timeout_ms == 0
            || startup_timeout_ms > MAX_CONNECTOR_TIMEOUT_MS
            || probe_interval_ms == 0
            || probe_interval_ms > startup_timeout_ms
        {
            return Err(ConnectorPolicyError::InvalidReadinessDeadline);
        }
        if success_threshold == 0
            || failure_threshold == 0
            || success_threshold > MAX_CONNECTOR_READINESS_THRESHOLD
            || failure_threshold > MAX_CONNECTOR_READINESS_THRESHOLD
        {
            return Err(ConnectorPolicyError::InvalidReadinessThreshold);
        }
        Ok(Self {
            startup_timeout_ms,
            probe_interval_ms,
            success_threshold,
            failure_threshold,
        })
    }

    pub const fn startup_timeout_ms(self) -> u64 {
        self.startup_timeout_ms
    }

    pub const fn probe_interval_ms(self) -> u64 {
        self.probe_interval_ms
    }

    pub const fn success_threshold(self) -> u32 {
        self.success_threshold
    }

    pub const fn failure_threshold(self) -> u32 {
        self.failure_threshold
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ConnectorPolicyError {
    #[error("connector worker queue capacity must be finite and non-zero")]
    InvalidWorkerQueueCapacity,
    #[error("connector retry attempts must be finite and non-zero")]
    InvalidMaxAttempts,
    #[error("connector timeout exceeds the finite policy limit")]
    InvalidTimeout,
    #[error("connector retry timing window is inconsistent")]
    InvalidRetryWindow,
    #[error("connector retry multiplier must be between 1.0 and 10.0")]
    InvalidMultiplier,
    #[error("connector retry jitter cannot exceed 100 percent")]
    InvalidJitter,
    #[error("connector readiness deadline or probe interval is invalid")]
    InvalidReadinessDeadline,
    #[error("connector readiness thresholds must be non-zero")]
    InvalidReadinessThreshold,
}

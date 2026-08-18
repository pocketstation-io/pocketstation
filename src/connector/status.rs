use super::ConnectorErrorCode;
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

use super::ConnectorErrorCode;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[doc = "Enumerates the supported connector delivery readiness cases."]
pub enum ConnectorDeliveryReadiness {
    #[doc = "Indicates the not ready state for `ConnectorDeliveryReadiness`."]
    NotReady = 0,
    #[doc = "Indicates the ready state for `ConnectorDeliveryReadiness`."]
    Ready = 1,
}

impl ConnectorDeliveryReadiness {
    #[doc = "Returns whether accepts delivery applies to `ConnectorDeliveryReadiness`."]
    pub const fn accepts_delivery(self) -> bool {
        matches!(self, Self::Ready)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[doc = "Enumerates the supported connector health cases."]
pub enum ConnectorHealth {
    #[doc = "Represents the healthy case of `ConnectorHealth`."]
    Healthy = 0,
    #[doc = "Represents the degraded case of `ConnectorHealth`."]
    Degraded = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[doc = "Enumerates the supported connector recovery cases."]
pub enum ConnectorRecovery {
    #[doc = "Represents the idle case of `ConnectorRecovery`."]
    Idle = 0,
    #[doc = "Represents the reconnecting case of `ConnectorRecovery`."]
    Reconnecting = 1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "Reports the structured connector service status."]
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
    #[doc = "Returns the delivery readiness held by `ConnectorServiceStatus`."]
    pub const fn delivery_readiness(&self) -> ConnectorDeliveryReadiness {
        self.delivery_readiness
    }

    #[doc = "Returns the health held by `ConnectorServiceStatus`."]
    pub const fn health(&self) -> ConnectorHealth {
        self.health
    }

    #[doc = "Returns the recovery held by `ConnectorServiceStatus`."]
    pub const fn recovery(&self) -> ConnectorRecovery {
        self.recovery
    }

    #[doc = "Returns the readiness reason code held by `ConnectorServiceStatus`."]
    pub const fn readiness_reason_code(&self) -> Option<&ConnectorErrorCode> {
        self.readiness_reason_code.as_ref()
    }

    #[doc = "Returns the health reason code held by `ConnectorServiceStatus`."]
    pub const fn health_reason_code(&self) -> Option<&ConnectorErrorCode> {
        self.health_reason_code.as_ref()
    }

    #[doc = "Returns the recovery reason code held by `ConnectorServiceStatus`."]
    pub const fn recovery_reason_code(&self) -> Option<&ConnectorErrorCode> {
        self.recovery_reason_code.as_ref()
    }

    #[doc = "Returns the revision held by `ConnectorServiceStatus`."]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[doc = "Returns the last transition elapsed nanoseconds held by `ConnectorServiceStatus`."]
    pub const fn last_transition_elapsed_ns(&self) -> u64 {
        self.last_transition_elapsed_ns
    }

    #[doc = "Returns whether accepts delivery applies to `ConnectorServiceStatus`."]
    pub const fn accepts_delivery(&self) -> bool {
        self.delivery_readiness.accepts_delivery()
            && matches!(self.recovery, ConnectorRecovery::Idle)
    }
}

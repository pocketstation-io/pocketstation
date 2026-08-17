#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConnectorReadiness {
    Starting = 0,
    Ready = 1,
    Degraded = 2,
    Reconnecting = 3,
    Stopping = 4,
    Stopped = 5,
    Failed = 6,
}

impl ConnectorReadiness {
    pub(crate) const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Starting,
            1 => Self::Ready,
            2 => Self::Degraded,
            3 => Self::Reconnecting,
            4 => Self::Stopping,
            5 => Self::Stopped,
            _ => Self::Failed,
        }
    }

    pub const fn can_transition_to(self, next: Self) -> bool {
        if self as u8 == next as u8 {
            return true;
        }
        match self {
            Self::Starting => matches!(
                next,
                Self::Ready | Self::Degraded | Self::Stopping | Self::Failed
            ),
            Self::Ready => matches!(
                next,
                Self::Degraded | Self::Reconnecting | Self::Stopping | Self::Failed
            ),
            Self::Degraded => matches!(
                next,
                Self::Ready | Self::Reconnecting | Self::Stopping | Self::Failed
            ),
            Self::Reconnecting => matches!(
                next,
                Self::Ready | Self::Degraded | Self::Stopping | Self::Failed
            ),
            Self::Stopping => matches!(next, Self::Stopped | Self::Failed),
            Self::Stopped | Self::Failed => false,
        }
    }

    pub const fn accepts_delivery(self) -> bool {
        matches!(self, Self::Ready | Self::Degraded)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid connector readiness transition from {current:?} to {requested:?}")]
pub struct ConnectorReadinessTransitionError {
    pub current: ConnectorReadiness,
    pub requested: ConnectorReadiness,
}

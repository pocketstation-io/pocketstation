use crate::{
    PolledAudioPollError, SessionRuntimeError, SessionStartError, SessionStopDisposition,
    SessionStopResult,
};

/// Stable language-neutral code for a public Session start failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStartErrorCode {
    HostSetupFailed,
    StartCancelled,
    InvalidSelector,
    DeclarationInvalid,
    CompileFailed,
    RuntimePrepareFailed,
    TransactionalStartFailed,
    MissingAudioReceipt,
    MissingEventReceiver,
}

impl SessionStartErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostSetupFailed => "session.host_setup_failed",
            Self::StartCancelled => "session.start_cancelled",
            Self::InvalidSelector => "session.invalid_selector",
            Self::DeclarationInvalid => "session.declaration_invalid",
            Self::CompileFailed => "session.compile_failed",
            Self::RuntimePrepareFailed => "session.runtime_prepare_failed",
            Self::TransactionalStartFailed => "session.transactional_start_failed",
            Self::MissingAudioReceipt => "session.missing_audio_receipt",
            Self::MissingEventReceiver => "session.missing_event_receiver",
        }
    }
}

/// Stable language-neutral code for a public running-Session failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRuntimeErrorCode {
    MissingMetricsSnapshot,
}

impl SessionRuntimeErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingMetricsSnapshot => "session.missing_metrics_snapshot",
        }
    }
}

/// Stable language-neutral code for bounded polled-audio status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolledAudioPollErrorCode {
    Empty,
    LeaseCapacityExhausted,
    StatePoisoned,
}

impl PolledAudioPollErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "audio.poll_empty",
            Self::LeaseCapacityExhausted => "audio.lease_capacity_exhausted",
            Self::StatePoisoned => "audio.receipt_state_poisoned",
        }
    }
}

/// Stable language-neutral result code for Session finalization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStopCode {
    Stopped,
    AlreadyStopped,
    FinalizationFailed,
}

impl SessionStopCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stopped => "session.stopped",
            Self::AlreadyStopped => "session.already_stopped",
            Self::FinalizationFailed => "session.finalization_failed",
        }
    }
}

impl SessionStartError {
    pub fn code(&self) -> SessionStartErrorCode {
        match self {
            Self::Host(_) => SessionStartErrorCode::HostSetupFailed,
            Self::Engine(error)
                if matches!(
                    error.start_failure().map(|failure| failure.error()),
                    Some(pks_session::SessionStartError::Cancelled { .. })
                ) =>
            {
                SessionStartErrorCode::StartCancelled
            }
            Self::Engine(pks_session::SessionEngineStartError::Freeze(
                pks_session::SessionError::InvalidSelector { .. },
            )) => SessionStartErrorCode::InvalidSelector,
            Self::Engine(pks_session::SessionEngineStartError::Freeze(_)) => {
                SessionStartErrorCode::DeclarationInvalid
            }
            Self::Engine(pks_session::SessionEngineStartError::Compile(_)) => {
                SessionStartErrorCode::CompileFailed
            }
            Self::Engine(pks_session::SessionEngineStartError::Prepare(_)) => {
                SessionStartErrorCode::RuntimePrepareFailed
            }
            Self::Engine(pks_session::SessionEngineStartError::Start(_)) => {
                SessionStartErrorCode::TransactionalStartFailed
            }
            Self::MissingAudioReceipt => SessionStartErrorCode::MissingAudioReceipt,
            Self::MissingEventReceiver => SessionStartErrorCode::MissingEventReceiver,
        }
    }
}

impl SessionRuntimeError {
    pub const fn code(self) -> SessionRuntimeErrorCode {
        match self {
            Self::MissingMetricsSnapshot => SessionRuntimeErrorCode::MissingMetricsSnapshot,
        }
    }
}

pub const fn polled_audio_poll_error_code(error: PolledAudioPollError) -> PolledAudioPollErrorCode {
    match error {
        PolledAudioPollError::Empty => PolledAudioPollErrorCode::Empty,
        PolledAudioPollError::LeaseCapacityExhausted => {
            PolledAudioPollErrorCode::LeaseCapacityExhausted
        }
        PolledAudioPollError::StatePoisoned => PolledAudioPollErrorCode::StatePoisoned,
    }
}

impl SessionStopResult {
    pub fn code(self) -> SessionStopCode {
        if !self.is_success() {
            SessionStopCode::FinalizationFailed
        } else {
            match self.disposition() {
                SessionStopDisposition::Stopped => SessionStopCode::Stopped,
                SessionStopDisposition::AlreadyStopped => SessionStopCode::AlreadyStopped,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        polled_audio_poll_error_code, PolledAudioPollErrorCode, SessionRuntimeErrorCode,
        SessionStartErrorCode, SessionStopCode,
    };
    use crate::{PolledAudioPollError, SessionRuntimeError};

    #[test]
    fn given_public_error_codes_when_serialized_then_values_are_stable_and_namespaced() {
        assert_eq!(
            SessionStartErrorCode::StartCancelled.as_str(),
            "session.start_cancelled"
        );
        assert_eq!(
            SessionRuntimeError::MissingMetricsSnapshot.code(),
            SessionRuntimeErrorCode::MissingMetricsSnapshot
        );
        assert_eq!(
            polled_audio_poll_error_code(PolledAudioPollError::LeaseCapacityExhausted),
            PolledAudioPollErrorCode::LeaseCapacityExhausted
        );
        assert_eq!(
            SessionStopCode::FinalizationFailed.as_str(),
            "session.finalization_failed"
        );
    }
}

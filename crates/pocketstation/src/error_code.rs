use crate::{
    SessionError, SessionRuntimeError, SessionStartError, SessionStopDisposition, SessionStopResult,
};
use pks_session::{SessionStartErrorCode, SessionStopCode, SessionStopFailureCode};

impl SessionStartError {
    pub fn code(&self) -> SessionStartErrorCode {
        match self {
            Self::Host(pks_session::SessionEngineHostBuildError::UnsupportedPlatform) => {
                SessionStartErrorCode::UnsupportedPlatform
            }
            Self::Host(_) => SessionStartErrorCode::HostSetupFailed,
            Self::Engine(error)
                if error.start_failure().is_some_and(|failure| {
                    matches!(
                        failure.error(),
                        pks_session::SessionStartError::Cancelled { .. }
                    )
                }) =>
            {
                SessionStartErrorCode::StartCancelled
            }
            Self::Engine(pks_session::SessionEngineStartError::Freeze(
                SessionError::InvalidSelector { .. },
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
            Self::Engine(pks_session::SessionEngineStartError::Start(failure)) => {
                pks_session::session_start_failure_code(failure.error())
            }
            Self::MissingAudioReceipt => SessionStartErrorCode::MissingAudioReceipt,
            Self::MissingEventReceiver => SessionStartErrorCode::MissingEventReceiver,
        }
    }
}

impl SessionRuntimeError {
    pub const fn code(self) -> pks_session::SessionRuntimeErrorCode {
        match self {
            Self::MissingMetricsSnapshot => {
                pks_session::SessionRuntimeErrorCode::MissingMetricsSnapshot
            }
        }
    }
}

impl SessionStopResult {
    pub fn code(self) -> SessionStopCode {
        if !self.is_success() {
            SessionStopCode::StopFailed
        } else {
            match self.disposition() {
                SessionStopDisposition::Stopped => SessionStopCode::Stopped,
                SessionStopDisposition::AlreadyStopped => SessionStopCode::AlreadyStopped,
            }
        }
    }

    pub fn failure_codes(self) -> Box<[SessionStopFailureCode]> {
        pks_session::session_stop_failure_codes(&self.outcome())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SessionRuntimeError;
    use pks_session::{
        SessionDeclarationErrorCode, SessionRuntimeErrorCode, SessionStartErrorCode,
        SessionStopCode,
    };

    #[test]
    fn given_facade_errors_when_mapped_then_codes_use_canonical_session_vocabulary() {
        assert_eq!(
            SessionStartError::Host(pks_session::SessionEngineHostBuildError::UnsupportedPlatform,)
                .code(),
            SessionStartErrorCode::UnsupportedPlatform
        );
        assert_eq!(
            SessionStartError::Engine(pks_session::SessionEngineStartError::Freeze(
                SessionError::InvalidSelector {
                    reason: "invalid".to_owned(),
                },
            ))
            .code(),
            SessionStartErrorCode::InvalidSelector
        );
        assert_eq!(
            SessionStartError::MissingAudioReceipt.code(),
            SessionStartErrorCode::MissingAudioReceipt
        );
        assert_eq!(
            SessionStartError::MissingEventReceiver.code(),
            SessionStartErrorCode::MissingEventReceiver
        );
        assert_eq!(
            SessionRuntimeError::MissingMetricsSnapshot.code(),
            SessionRuntimeErrorCode::MissingMetricsSnapshot
        );
    }

    #[test]
    fn given_reexported_codes_when_serialized_then_canonical_values_are_unchanged() {
        assert_eq!(
            SessionDeclarationErrorCode::InvalidSelector.as_str(),
            "session.invalid_selector"
        );
        assert_eq!(
            SessionStartErrorCode::StartCancelled.as_str(),
            "session.start_cancelled"
        );
        assert_eq!(SessionStopCode::StopFailed.as_str(), "session.stop_failed");
    }
}

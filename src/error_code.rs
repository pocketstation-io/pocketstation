use crate::session::{SessionStopCode, SessionStopFailureCode};
use crate::{SessionRuntimeError, SessionStopDisposition, SessionStopResult};

impl SessionRuntimeError {
    pub const fn code(self) -> crate::session::SessionRuntimeErrorCode {
        match self {
            Self::MissingMetricsSnapshot => {
                crate::session::SessionRuntimeErrorCode::MissingMetricsSnapshot
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
        crate::session::session_stop_failure_codes(&self.outcome())
    }
}

#[cfg(test)]
mod tests {
    use crate::session::{
        SessionDeclarationErrorCode, SessionRuntimeErrorCode, SessionStartErrorCode,
        SessionStopCode,
    };
    use crate::{SessionError, SessionRuntimeError, SessionStartError};

    #[test]
    fn given_facade_errors_when_mapped_then_codes_use_canonical_session_vocabulary() {
        assert_eq!(
            SessionStartError::from(
                crate::session::SessionEngineHostBuildError::UnsupportedPlatform,
            )
            .code(),
            SessionStartErrorCode::UnsupportedPlatform
        );
        assert_eq!(
            SessionStartError::from(crate::session::SessionEngineStartError::Freeze(
                SessionError::InvalidSelector {
                    reason: "invalid".to_owned(),
                },
            ))
            .code(),
            SessionStartErrorCode::InvalidSelector
        );
        assert_eq!(
            SessionStartError::from(crate::session::SessionTraceRecorderStartError::ZeroCapacity)
                .code(),
            SessionStartErrorCode::TraceRecorderSetupFailed
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

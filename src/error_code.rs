use crate::session::{SessionStartErrorCode, SessionStopCode, SessionStopFailureCode};
use crate::{
    SessionError, SessionRuntimeError, SessionStartError, SessionStopDisposition, SessionStopResult,
};

impl SessionStartError {
    pub fn code(&self) -> SessionStartErrorCode {
        match self {
            Self::Host(crate::session::SessionEngineHostBuildError::UnsupportedPlatform) => {
                SessionStartErrorCode::UnsupportedPlatform
            }
            Self::Host(_) => SessionStartErrorCode::HostSetupFailed,
            Self::TraceRecorder(_) => SessionStartErrorCode::TraceRecorderSetupFailed,
            Self::Engine(error)
                if error.start_failure().is_some_and(|failure| {
                    matches!(
                        failure.error(),
                        crate::session::SessionStartError::Cancelled { .. }
                    )
                }) =>
            {
                SessionStartErrorCode::StartCancelled
            }
            Self::Engine(crate::session::SessionEngineStartError::Freeze(
                SessionError::InvalidSelector { .. },
            )) => SessionStartErrorCode::InvalidSelector,
            Self::Engine(crate::session::SessionEngineStartError::Freeze(_)) => {
                SessionStartErrorCode::DeclarationInvalid
            }
            Self::Engine(crate::session::SessionEngineStartError::Compile(_)) => {
                SessionStartErrorCode::CompileFailed
            }
            Self::Engine(crate::session::SessionEngineStartError::Prepare(_)) => {
                SessionStartErrorCode::RuntimePrepareFailed
            }
            Self::Engine(crate::session::SessionEngineStartError::Start(failure)) => {
                crate::session::session_start_failure_code(failure.error())
            }
            Self::MissingAudioReceipt => SessionStartErrorCode::MissingAudioReceipt,
            Self::MissingRecordingConfiguration => {
                SessionStartErrorCode::MissingRecordingConfiguration
            }
            Self::MissingEventReceiver => SessionStartErrorCode::MissingEventReceiver,
            Self::EndpointRegistrationStateUnavailable
            | Self::OperatorRegistrationStateUnavailable => SessionStartErrorCode::HostSetupFailed,
        }
    }
}

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
    use super::*;
    use crate::session::{
        SessionDeclarationErrorCode, SessionRuntimeErrorCode, SessionStartErrorCode,
        SessionStopCode,
    };
    use crate::SessionRuntimeError;

    #[test]
    fn given_facade_errors_when_mapped_then_codes_use_canonical_session_vocabulary() {
        assert_eq!(
            SessionStartError::Host(
                crate::session::SessionEngineHostBuildError::UnsupportedPlatform,
            )
            .code(),
            SessionStartErrorCode::UnsupportedPlatform
        );
        assert_eq!(
            SessionStartError::Engine(crate::session::SessionEngineStartError::Freeze(
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
            SessionStartError::MissingRecordingConfiguration.code(),
            SessionStartErrorCode::MissingRecordingConfiguration
        );
        assert_eq!(
            SessionStartError::MissingEventReceiver.code(),
            SessionStartErrorCode::MissingEventReceiver
        );
        assert_eq!(
            SessionStartError::TraceRecorder(
                crate::session::SessionTraceRecorderStartError::ZeroCapacity,
            )
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

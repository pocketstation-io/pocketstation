use crate::capture::CaptureError;
use crate::session::{PolledAudioPollError, SessionError, SessionStartError, SessionStopOutcome};

/// Stable language-neutral code for a Session declaration failure.
///
/// The value returned by [`Self::as_str`] is the compatibility contract.
/// Variant names and discriminants remain Rust implementation details.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionDeclarationErrorCode {
    NoSources,
    NoRoutes,
    NoSourceOutputs,
    InvalidSelector,
    InvalidEndpoint,
    InvalidOperator,
    InvalidRoute,
    ForeignEndpoint,
    DraftFrozen,
    InternalStateUnavailable,
    IdExhausted,
    UnsupportedVersion,
    UnknownEndpoint,
    UnknownStem,
    UnknownSource,
    UnknownOperatorInstance,
    OperatorHasNoDestination,
}

impl SessionDeclarationErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSources => "session.no_sources",
            Self::NoRoutes => "session.no_routes",
            Self::NoSourceOutputs => "session.no_source_outputs",
            Self::InvalidSelector => "session.invalid_selector",
            Self::InvalidEndpoint => "session.invalid_endpoint",
            Self::InvalidOperator => "session.invalid_operator",
            Self::InvalidRoute => "session.invalid_route",
            Self::ForeignEndpoint => "session.foreign_endpoint",
            Self::DraftFrozen => "session.draft_frozen",
            Self::InternalStateUnavailable => "session.internal_state_unavailable",
            Self::IdExhausted => "session.id_exhausted",
            Self::UnsupportedVersion => "session.unsupported_version",
            Self::UnknownEndpoint => "session.unknown_endpoint",
            Self::UnknownStem => "session.unknown_stem",
            Self::UnknownSource => "session.unknown_source",
            Self::UnknownOperatorInstance => "session.unknown_operator_instance",
            Self::OperatorHasNoDestination => "session.operator_has_no_destination",
        }
    }
}

/// Stable language-neutral code for Session startup.
///
/// This enum also reserves codes used by a language façade around the
/// canonical engine. The code vocabulary remains owned by the Session module; an
/// adapter owns only the mapping from its wrapper error.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStartErrorCode {
    HostSetupFailed,
    UnsupportedPlatform,
    StartCancelled,
    InvalidSelector,
    DeclarationInvalid,
    CompileFailed,
    RuntimePrepareFailed,
    InvalidStartOptions,
    UnsupportedSourceTopology,
    MissingEndpointDeclaration,
    EndpointPrepareFailed,
    CapturePermissionDenied,
    CaptureSourceUnavailable,
    CaptureUnsupported,
    CaptureBackendFailed,
    EndpointStartFailed,
    RuntimeStartFailed,
    MissingAudioReceipt,
    MissingRecordingConfiguration,
    MissingEventReceiver,
    TraceRecorderSetupFailed,
}

impl SessionStartErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostSetupFailed => "session.host_setup_failed",
            Self::UnsupportedPlatform => "session.unsupported_platform",
            Self::StartCancelled => "session.start_cancelled",
            Self::InvalidSelector => "session.start_invalid_selector",
            Self::DeclarationInvalid => "session.declaration_invalid",
            Self::CompileFailed => "session.compile_failed",
            Self::RuntimePrepareFailed => "session.runtime_prepare_failed",
            Self::InvalidStartOptions => "session.invalid_start_options",
            Self::UnsupportedSourceTopology => "session.unsupported_source_topology",
            Self::MissingEndpointDeclaration => "session.missing_endpoint_declaration",
            Self::EndpointPrepareFailed => "session.endpoint_prepare_failed",
            Self::CapturePermissionDenied => "capture.permission_denied",
            Self::CaptureSourceUnavailable => "capture.source_unavailable",
            Self::CaptureUnsupported => "capture.unsupported",
            Self::CaptureBackendFailed => "capture.backend_failed",
            Self::EndpointStartFailed => "session.endpoint_start_failed",
            Self::RuntimeStartFailed => "session.runtime_start_failed",
            Self::MissingAudioReceipt => "session.missing_audio_receipt",
            Self::MissingRecordingConfiguration => "session.missing_recording_configuration",
            Self::MissingEventReceiver => "session.missing_event_receiver",
            Self::TraceRecorderSetupFailed => "session.trace_recorder_setup_failed",
        }
    }
}

/// Stable language-neutral code for a running-Session projection failure.
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolledAudioPollErrorCode {
    Empty,
    LeaseCapacityExhausted,
    InternalStateUnavailable,
}

impl PolledAudioPollErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "audio.poll_empty",
            Self::LeaseCapacityExhausted => "audio.lease_capacity_exhausted",
            Self::InternalStateUnavailable => "audio.internal_state_unavailable",
        }
    }
}

/// Stable language-neutral status for an idempotent Session stop.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStopCode {
    Stopped,
    AlreadyStopped,
    StopFailed,
}

impl SessionStopCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stopped => "session.stopped",
            Self::AlreadyStopped => "session.already_stopped",
            Self::StopFailed => "session.stop_failed",
        }
    }
}

/// Stable language-neutral cause retained by a failed Session stop.
///
/// A terminal outcome can contain more than one cause.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStopFailureCode {
    RuntimeWorkerPanicked,
    CaptureFinalizationFailed,
    OperatorFinalizationFailed,
    EndpointFinalizationFailed,
    RuntimeFailed,
    LineageFailed,
    SourceSendRejected,
}

impl SessionStopFailureCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeWorkerPanicked => "session.runtime_worker_panicked",
            Self::CaptureFinalizationFailed => "capture.finalization_failed",
            Self::OperatorFinalizationFailed => "session.operator_finalization_failed",
            Self::EndpointFinalizationFailed => "session.endpoint_finalization_failed",
            Self::RuntimeFailed => "session.runtime_failed",
            Self::LineageFailed => "session.lineage_failed",
            Self::SourceSendRejected => "session.source_send_rejected",
        }
    }
}

pub const fn session_declaration_error_code(error: &SessionError) -> SessionDeclarationErrorCode {
    match error {
        SessionError::NoSources => SessionDeclarationErrorCode::NoSources,
        SessionError::NoRoutes { .. } => SessionDeclarationErrorCode::NoRoutes,
        SessionError::NoSourceOutputs { .. } | SessionError::NoSourceOutputRoutes { .. } => {
            SessionDeclarationErrorCode::NoSourceOutputs
        }
        SessionError::InvalidSelector { .. } => SessionDeclarationErrorCode::InvalidSelector,
        SessionError::InvalidEndpoint { .. } => SessionDeclarationErrorCode::InvalidEndpoint,
        SessionError::InvalidOperator { .. } => SessionDeclarationErrorCode::InvalidOperator,
        SessionError::InvalidRoute { .. } => SessionDeclarationErrorCode::InvalidRoute,
        SessionError::ForeignEndpoint { .. } => SessionDeclarationErrorCode::ForeignEndpoint,
        SessionError::DraftFrozen { .. } => SessionDeclarationErrorCode::DraftFrozen,
        SessionError::DraftPoisoned => SessionDeclarationErrorCode::InternalStateUnavailable,
        SessionError::IdExhausted => SessionDeclarationErrorCode::IdExhausted,
        SessionError::UnsupportedVersion { .. } => SessionDeclarationErrorCode::UnsupportedVersion,
        SessionError::UnknownEndpoint { .. } => SessionDeclarationErrorCode::UnknownEndpoint,
        SessionError::UnknownStem { .. } => SessionDeclarationErrorCode::UnknownStem,
        SessionError::UnknownSourceInstance { .. } | SessionError::UnknownSourceOutput { .. } => {
            SessionDeclarationErrorCode::UnknownSource
        }
        SessionError::UnknownOperatorInstance { .. } => {
            SessionDeclarationErrorCode::UnknownOperatorInstance
        }
        SessionError::OperatorHasNoDestination { .. } => {
            SessionDeclarationErrorCode::OperatorHasNoDestination
        }
    }
}

pub const fn session_start_failure_code(error: &SessionStartError) -> SessionStartErrorCode {
    match error {
        SessionStartError::InvalidOptions { .. } => SessionStartErrorCode::InvalidStartOptions,
        SessionStartError::UnsupportedSourceTopology => {
            SessionStartErrorCode::UnsupportedSourceTopology
        }
        SessionStartError::OperatorRuntimeHost { .. } => SessionStartErrorCode::RuntimeStartFailed,
        SessionStartError::OperatorPrepare { .. } => SessionStartErrorCode::RuntimePrepareFailed,
        SessionStartError::ExternalSourcePrepare { .. }
        | SessionStartError::ExternalAudioBridge { .. } => {
            SessionStartErrorCode::RuntimePrepareFailed
        }
        SessionStartError::MissingEndpointDeclaration { .. } => {
            SessionStartErrorCode::MissingEndpointDeclaration
        }
        SessionStartError::EndpointPrepare { .. } => SessionStartErrorCode::EndpointPrepareFailed,
        SessionStartError::CapturePrepare { source, .. }
        | SessionStartError::CaptureOpen { source, .. } => capture_error_code(source),
        SessionStartError::EndpointStart { .. } => SessionStartErrorCode::EndpointStartFailed,
        SessionStartError::RuntimeRunner { .. }
        | SessionStartError::RuntimeWorkerSpawn { .. }
        | SessionStartError::RuntimeWorkerReady { .. }
        | SessionStartError::ExternalSourceStart { .. } => {
            SessionStartErrorCode::RuntimeStartFailed
        }
        SessionStartError::Cancelled { .. } => SessionStartErrorCode::StartCancelled,
    }
}

pub const fn polled_audio_poll_error_code(error: PolledAudioPollError) -> PolledAudioPollErrorCode {
    match error {
        PolledAudioPollError::Empty => PolledAudioPollErrorCode::Empty,
        PolledAudioPollError::LeaseCapacityExhausted => {
            PolledAudioPollErrorCode::LeaseCapacityExhausted
        }
        PolledAudioPollError::StatePoisoned => PolledAudioPollErrorCode::InternalStateUnavailable,
    }
}

pub fn session_stop_failure_codes(outcome: &SessionStopOutcome) -> Box<[SessionStopFailureCode]> {
    let mut failures = Vec::with_capacity(7);
    if outcome.runtime_worker_panicked() {
        failures.push(SessionStopFailureCode::RuntimeWorkerPanicked);
    }
    if outcome.capture_finalization_failures_total() > 0 {
        failures.push(SessionStopFailureCode::CaptureFinalizationFailed);
    }
    if outcome.operator_finalization_failures_total() > 0 {
        failures.push(SessionStopFailureCode::OperatorFinalizationFailed);
    }
    if outcome.endpoint_finalization_failures_total() > 0 {
        failures.push(SessionStopFailureCode::EndpointFinalizationFailed);
    }
    if outcome.runtime_failures_total() > 0 {
        failures.push(SessionStopFailureCode::RuntimeFailed);
    }
    if outcome.lineage_failures_total() > 0 {
        failures.push(SessionStopFailureCode::LineageFailed);
    }
    if outcome.source_send_rejections_total() > 0 {
        failures.push(SessionStopFailureCode::SourceSendRejected);
    }
    failures.into_boxed_slice()
}

const fn capture_error_code(error: &CaptureError) -> SessionStartErrorCode {
    match error {
        CaptureError::PermissionDenied { .. } => SessionStartErrorCode::CapturePermissionDenied,
        CaptureError::SourceUnavailable { .. } => SessionStartErrorCode::CaptureSourceUnavailable,
        CaptureError::NotSupported | CaptureError::ModeUnsupported(_) => {
            SessionStartErrorCode::CaptureUnsupported
        }
        CaptureError::BackendInit(_)
        | CaptureError::BackendSetupRequired { .. }
        | CaptureError::BackendStatus { .. }
        | CaptureError::InvalidStreamCapacity
        | CaptureError::InvalidRuntimeEventCapacity
        | CaptureError::CaptureWorkerPanicked { .. } => SessionStartErrorCode::CaptureBackendFailed,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{
        capture_error_code, polled_audio_poll_error_code, session_declaration_error_code,
        session_start_failure_code, PolledAudioPollErrorCode, SessionDeclarationErrorCode,
        SessionRuntimeErrorCode, SessionStartErrorCode, SessionStopCode, SessionStopFailureCode,
    };
    use crate::capture::CaptureError;
    use crate::session::{
        EndpointId, PolledAudioPollError, SessionError, SessionId, SessionStartError, StemId,
    };

    #[test]
    fn given_stable_code_vocabulary_when_serialized_then_values_are_unique_and_namespaced() {
        let values = [
            SessionDeclarationErrorCode::NoSources.as_str(),
            SessionDeclarationErrorCode::NoRoutes.as_str(),
            SessionDeclarationErrorCode::InvalidSelector.as_str(),
            SessionDeclarationErrorCode::InvalidEndpoint.as_str(),
            SessionDeclarationErrorCode::InvalidOperator.as_str(),
            SessionDeclarationErrorCode::InvalidRoute.as_str(),
            SessionDeclarationErrorCode::ForeignEndpoint.as_str(),
            SessionDeclarationErrorCode::DraftFrozen.as_str(),
            SessionDeclarationErrorCode::InternalStateUnavailable.as_str(),
            SessionDeclarationErrorCode::IdExhausted.as_str(),
            SessionDeclarationErrorCode::UnsupportedVersion.as_str(),
            SessionDeclarationErrorCode::UnknownEndpoint.as_str(),
            SessionDeclarationErrorCode::UnknownStem.as_str(),
            SessionDeclarationErrorCode::UnknownOperatorInstance.as_str(),
            SessionDeclarationErrorCode::OperatorHasNoDestination.as_str(),
            SessionStartErrorCode::HostSetupFailed.as_str(),
            SessionStartErrorCode::UnsupportedPlatform.as_str(),
            SessionStartErrorCode::StartCancelled.as_str(),
            SessionStartErrorCode::InvalidSelector.as_str(),
            SessionStartErrorCode::DeclarationInvalid.as_str(),
            SessionStartErrorCode::CompileFailed.as_str(),
            SessionStartErrorCode::RuntimePrepareFailed.as_str(),
            SessionStartErrorCode::InvalidStartOptions.as_str(),
            SessionStartErrorCode::UnsupportedSourceTopology.as_str(),
            SessionStartErrorCode::MissingEndpointDeclaration.as_str(),
            SessionStartErrorCode::EndpointPrepareFailed.as_str(),
            SessionStartErrorCode::CapturePermissionDenied.as_str(),
            SessionStartErrorCode::CaptureSourceUnavailable.as_str(),
            SessionStartErrorCode::CaptureUnsupported.as_str(),
            SessionStartErrorCode::CaptureBackendFailed.as_str(),
            SessionStartErrorCode::EndpointStartFailed.as_str(),
            SessionStartErrorCode::RuntimeStartFailed.as_str(),
            SessionStartErrorCode::MissingAudioReceipt.as_str(),
            SessionStartErrorCode::MissingRecordingConfiguration.as_str(),
            SessionStartErrorCode::MissingEventReceiver.as_str(),
            SessionStartErrorCode::TraceRecorderSetupFailed.as_str(),
            SessionRuntimeErrorCode::MissingMetricsSnapshot.as_str(),
            PolledAudioPollErrorCode::Empty.as_str(),
            PolledAudioPollErrorCode::LeaseCapacityExhausted.as_str(),
            PolledAudioPollErrorCode::InternalStateUnavailable.as_str(),
            SessionStopCode::Stopped.as_str(),
            SessionStopCode::AlreadyStopped.as_str(),
            SessionStopCode::StopFailed.as_str(),
            SessionStopFailureCode::RuntimeWorkerPanicked.as_str(),
            SessionStopFailureCode::CaptureFinalizationFailed.as_str(),
            SessionStopFailureCode::OperatorFinalizationFailed.as_str(),
            SessionStopFailureCode::EndpointFinalizationFailed.as_str(),
            SessionStopFailureCode::RuntimeFailed.as_str(),
            SessionStopFailureCode::LineageFailed.as_str(),
            SessionStopFailureCode::SourceSendRejected.as_str(),
        ];
        let unique = values.iter().copied().collect::<HashSet<_>>();
        assert_eq!(unique.len(), values.len());
        assert!(values.iter().all(|value| {
            value.starts_with("session.")
                || value.starts_with("capture.")
                || value.starts_with("audio.")
        }));
    }

    #[test]
    fn given_declaration_errors_when_mapped_then_every_variant_has_a_stable_code() {
        let session_id = SessionId(7);
        let stem_id = StemId(8);
        let endpoint_id = EndpointId(9);
        let operator_instance_id = crate::session::OperatorInstanceId::new(10);
        let cases = [
            (
                SessionError::NoSources,
                SessionDeclarationErrorCode::NoSources,
            ),
            (
                SessionError::NoRoutes { stem_id },
                SessionDeclarationErrorCode::NoRoutes,
            ),
            (
                SessionError::InvalidSelector {
                    reason: "invalid".to_owned(),
                },
                SessionDeclarationErrorCode::InvalidSelector,
            ),
            (
                SessionError::InvalidEndpoint {
                    reason: "invalid".to_owned(),
                },
                SessionDeclarationErrorCode::InvalidEndpoint,
            ),
            (
                SessionError::InvalidOperator {
                    reason: "invalid".to_owned(),
                },
                SessionDeclarationErrorCode::InvalidOperator,
            ),
            (
                SessionError::InvalidRoute {
                    reason: "invalid".to_owned(),
                },
                SessionDeclarationErrorCode::InvalidRoute,
            ),
            (
                SessionError::ForeignEndpoint {
                    expected: session_id,
                    actual: SessionId(10),
                },
                SessionDeclarationErrorCode::ForeignEndpoint,
            ),
            (
                SessionError::DraftFrozen { session_id },
                SessionDeclarationErrorCode::DraftFrozen,
            ),
            (
                SessionError::DraftPoisoned,
                SessionDeclarationErrorCode::InternalStateUnavailable,
            ),
            (
                SessionError::IdExhausted,
                SessionDeclarationErrorCode::IdExhausted,
            ),
            (
                SessionError::UnsupportedVersion { major: 2, minor: 0 },
                SessionDeclarationErrorCode::UnsupportedVersion,
            ),
            (
                SessionError::UnknownEndpoint { endpoint_id },
                SessionDeclarationErrorCode::UnknownEndpoint,
            ),
            (
                SessionError::UnknownStem { stem_id },
                SessionDeclarationErrorCode::UnknownStem,
            ),
            (
                SessionError::UnknownOperatorInstance {
                    operator_instance_id,
                },
                SessionDeclarationErrorCode::UnknownOperatorInstance,
            ),
            (
                SessionError::OperatorHasNoDestination {
                    operator_instance_id,
                },
                SessionDeclarationErrorCode::OperatorHasNoDestination,
            ),
        ];
        for (error, expected) in cases {
            assert_eq!(session_declaration_error_code(&error), expected);
        }
    }

    #[test]
    fn given_start_and_capture_failures_when_mapped_then_specific_classes_are_preserved() {
        let endpoint_id = EndpointId(1);
        let start_cases = [
            (
                SessionStartError::InvalidOptions { reason: "invalid" },
                SessionStartErrorCode::InvalidStartOptions,
            ),
            (
                SessionStartError::UnsupportedSourceTopology,
                SessionStartErrorCode::UnsupportedSourceTopology,
            ),
            (
                SessionStartError::MissingEndpointDeclaration { endpoint_id },
                SessionStartErrorCode::MissingEndpointDeclaration,
            ),
            (
                SessionStartError::Cancelled {
                    rollback_failures_total: 0,
                },
                SessionStartErrorCode::StartCancelled,
            ),
        ];
        for (error, expected) in start_cases {
            assert_eq!(session_start_failure_code(&error), expected);
        }

        let capture_cases = [
            (
                CaptureError::PermissionDenied {
                    operation: "opening microphone",
                },
                SessionStartErrorCode::CapturePermissionDenied,
            ),
            (
                CaptureError::SourceUnavailable {
                    stable_key: "source".to_owned(),
                },
                SessionStartErrorCode::CaptureSourceUnavailable,
            ),
            (
                CaptureError::NotSupported,
                SessionStartErrorCode::CaptureUnsupported,
            ),
            (
                CaptureError::BackendInit("failure".to_owned()),
                SessionStartErrorCode::CaptureBackendFailed,
            ),
        ];
        for (error, expected) in capture_cases {
            assert_eq!(capture_error_code(&error), expected);
        }
    }

    #[test]
    fn given_polled_audio_failures_when_mapped_then_every_status_is_preserved() {
        let cases = [
            (PolledAudioPollError::Empty, PolledAudioPollErrorCode::Empty),
            (
                PolledAudioPollError::LeaseCapacityExhausted,
                PolledAudioPollErrorCode::LeaseCapacityExhausted,
            ),
            (
                PolledAudioPollError::StatePoisoned,
                PolledAudioPollErrorCode::InternalStateUnavailable,
            ),
        ];
        for (error, expected) in cases {
            assert_eq!(polled_audio_poll_error_code(error), expected);
        }
    }
}

//! Public controls and outcomes for starting and stopping a prepared Session.
//!
//! These types describe lifecycle input and failure. They deliberately contain
//! no runtime orchestration or capture callback behavior.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::capture::{CallbackCaptureBackend, CaptureError};
use crate::endpoint::{EndpointPrepareError, EndpointStartFailure};
use crate::frame::{EndpointId, StemId};
use crate::runtime::PlanRunnerError;
use crate::session::{
    OperatorInstanceId, PreparedSession, SessionEventReceiver, SessionRollbackFailure, Source,
};

#[doc = "Supplies the application and microphone capture backends used while preparing a Session."]
pub struct CaptureBackendSet<'backend> {
    #[doc = "Stores the application component of `CaptureBackendSet`."]
    pub application: &'backend dyn CallbackCaptureBackend,
    #[doc = "Stores the microphone component of `CaptureBackendSet`."]
    pub microphone: &'backend dyn CallbackCaptureBackend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Configures session start behavior at its owning API boundary."]
pub struct SessionStartOptions {
    #[doc = "Sets the capture frame capacity frames available to `SessionStartOptions`."]
    pub capture_frame_capacity_frames: usize,
    #[doc = "Sets the capture runtime event capacity events available to `SessionStartOptions`."]
    pub capture_runtime_event_capacity_events: usize,
    #[doc = "Contains the runtime work budget frames owned or reported by `SessionStartOptions`."]
    pub runtime_work_budget_frames: usize,
    #[doc = "Stores the runtime idle poll value for `SessionStartOptions`, in milliseconds."]
    pub runtime_idle_poll_ms: u64,
    #[doc = "Stores the runtime ready timeout value for `SessionStartOptions`, in milliseconds."]
    pub runtime_ready_timeout_ms: u64,
    #[doc = "Sets the session event capacity events available to `SessionStartOptions`."]
    pub session_event_capacity_events: usize,
}

impl Default for SessionStartOptions {
    #[doc = "Returns the default `SessionStartOptions` value."]
    fn default() -> Self {
        Self {
            capture_frame_capacity_frames: 32,
            capture_runtime_event_capacity_events: 8,
            runtime_work_budget_frames: 64,
            runtime_idle_poll_ms: 1,
            runtime_ready_timeout_ms: 1_000,
            session_event_capacity_events: 32,
        }
    }
}

pub(super) fn validate_start_options(
    options: SessionStartOptions,
) -> Result<(), SessionStartError> {
    let reason = if options.capture_frame_capacity_frames == 0 {
        Some("capture frame capacity must be greater than zero")
    } else if options.capture_runtime_event_capacity_events == 0 {
        Some("capture runtime-event capacity must be greater than zero")
    } else if options.runtime_work_budget_frames == 0 {
        Some("runtime work budget must be greater than zero")
    } else if options.runtime_idle_poll_ms == 0 {
        Some("runtime idle poll interval must be greater than zero")
    } else if options.runtime_ready_timeout_ms == 0 {
        Some("runtime ready timeout must be greater than zero")
    } else if options.session_event_capacity_events == 0 {
        Some("session event capacity must be greater than zero")
    } else {
        None
    };
    match reason {
        Some(reason) => Err(SessionStartError::InvalidOptions { reason }),
        None => Ok(()),
    }
}

pub(super) fn validate_source_topology(
    prepared: &PreparedSession,
) -> Result<(), SessionStartError> {
    let application_sources = prepared
        .spec
        .stems()
        .iter()
        .filter(|stem| matches!(stem.source(), Source::Application(_)))
        .count();
    let microphone_sources = prepared
        .spec
        .stems()
        .iter()
        .filter(|stem| matches!(stem.source(), Source::Microphone(_)))
        .count();
    if source_topology_has_input(
        application_sources,
        microphone_sources,
        prepared.spec.source_instances().len(),
    ) {
        Ok(())
    } else {
        Err(SessionStartError::UnsupportedSourceTopology)
    }
}

const fn source_topology_has_input(
    application_sources: usize,
    microphone_sources: usize,
    registered_sources: usize,
) -> bool {
    application_sources > 0 || microphone_sources > 0 || registered_sources > 0
}

/// Thread-safe cancellation request for a Session that has not reached
/// `Running` yet.
#[derive(Clone, Debug, Default)]
pub struct SessionStartCancellation {
    requested: Arc<AtomicBool>,
}

impl SessionStartCancellation {
    #[doc = "Requests the state transition represented by `SessionStartCancellation`."]
    pub fn request(&self) {
        self.requested.store(true, Ordering::Release);
    }

    #[doc = "Reports whether requested is true for `SessionStartCancellation`."]
    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures produced during session lifecycle start."]
pub enum SessionStartError {
    #[error("invalid Session start options: {reason}")]
    #[doc = "Reports that the supplied options is invalid."]
    InvalidOptions {
        #[doc = "Carries the reason reported by `InvalidOptions`."]
        reason: &'static str,
    },
    #[error("Session requires at least one built-in or registered source")]
    #[doc = "Reports that the requested source topology is unsupported."]
    UnsupportedSourceTopology,
    #[error("external source preparation failed: {message}")]
    #[doc = "Classifies a failure at the external source prepare stage or component of `SessionStartError`."]
    ExternalSourcePrepare {
        #[doc = "Carries the diagnostic message reported by `ExternalSourcePrepare`."]
        message: String,
        #[doc = "Counts the total number of rollback failures observed by `ExternalSourcePrepare`."]
        rollback_failures_total: u64,
    },
    #[error("external source audio ingress failed: {message}")]
    #[doc = "Classifies a failure at the external audio bridge stage or component of `SessionStartError`."]
    ExternalAudioBridge {
        #[doc = "Carries the diagnostic message reported by `ExternalAudioBridge`."]
        message: String,
        #[doc = "Counts the total number of rollback failures observed by `ExternalAudioBridge`."]
        rollback_failures_total: u64,
    },
    #[error("generated audio reentry failed: {message}")]
    #[doc = "Classifies a failure at the generated audio bridge stage or component of `SessionStartError`."]
    GeneratedAudioBridge {
        #[doc = "Carries the diagnostic message reported by `GeneratedAudioBridge`."]
        message: String,
        #[doc = "Counts the total number of rollback failures observed by `GeneratedAudioBridge`."]
        rollback_failures_total: u64,
    },
    #[error("external source start failed: {message}")]
    #[doc = "Classifies a failure at the external source start stage or component of `SessionStartError`."]
    ExternalSourceStart {
        #[doc = "Carries the diagnostic message reported by `ExternalSourceStart`."]
        message: String,
        #[doc = "Counts the total number of rollback failures observed by `ExternalSourceStart`."]
        rollback_failures_total: u64,
    },
    #[error("async operator runtime host could not start: {message}")]
    #[doc = "Classifies a failure at the operator runtime host stage or component of `SessionStartError`."]
    OperatorRuntimeHost {
        #[doc = "Carries the diagnostic message reported by `OperatorRuntimeHost`."]
        message: String,
        #[doc = "Counts the total number of rollback failures observed by `OperatorRuntimeHost`."]
        rollback_failures_total: u64,
    },
    #[error("operator {operator_instance_id:?} preparation failed: {message}")]
    #[doc = "Classifies a failure at the operator prepare stage or component of `SessionStartError`."]
    OperatorPrepare {
        #[doc = "Identifies the operator instance identifier recorded by `OperatorPrepare`."]
        operator_instance_id: OperatorInstanceId,
        #[doc = "Carries the diagnostic message reported by `OperatorPrepare`."]
        message: String,
        #[doc = "Counts the total number of rollback failures observed by `OperatorPrepare`."]
        rollback_failures_total: u64,
    },
    #[error("endpoint {endpoint_id:?} declaration is absent")]
    #[doc = "Reports that the required endpoint declaration is missing."]
    MissingEndpointDeclaration {
        #[doc = "Identifies the endpoint identifier recorded by `MissingEndpointDeclaration`."]
        endpoint_id: EndpointId,
    },
    #[error("endpoint preparation failed: {source}")]
    #[doc = "Classifies a failure at the endpoint prepare stage or component of `SessionStartError`."]
    EndpointPrepare {
        #[source]
        #[doc = "Carries the source selected for `EndpointPrepare`."]
        source: EndpointPrepareError,
        #[doc = "Counts the total number of rollback failures observed by `EndpointPrepare`."]
        rollback_failures_total: u64,
    },
    #[error("capture preparation failed for stem {stem_id:?}: {source}")]
    #[doc = "Classifies a failure at the capture prepare stage or component of `SessionStartError`."]
    CapturePrepare {
        #[doc = "Identifies the stem identifier recorded by `CapturePrepare`."]
        stem_id: StemId,
        #[source]
        #[doc = "Carries the source selected for `CapturePrepare`."]
        source: CaptureError,
        #[doc = "Counts the total number of rollback failures observed by `CapturePrepare`."]
        rollback_failures_total: u64,
    },
    #[error("capture open failed for stem {stem_id:?}: {source}")]
    #[doc = "Classifies a failure at the capture open stage or component of `SessionStartError`."]
    CaptureOpen {
        #[doc = "Identifies the stem identifier recorded by `CaptureOpen`."]
        stem_id: StemId,
        #[source]
        #[doc = "Carries the source selected for `CaptureOpen`."]
        source: CaptureError,
        #[doc = "Counts the total number of rollback failures observed by `CaptureOpen`."]
        rollback_failures_total: u64,
    },
    #[error("endpoint start failed: {source}")]
    #[doc = "Classifies a failure at the endpoint start stage or component of `SessionStartError`."]
    EndpointStart {
        #[source]
        #[doc = "Carries the source selected for `EndpointStart`."]
        source: EndpointStartFailure,
        #[doc = "Counts the total number of rollback failures observed by `EndpointStart`."]
        rollback_failures_total: u64,
    },
    #[error("runtime runner preparation failed: {source}")]
    #[doc = "Classifies a failure at the runtime runner stage or component of `SessionStartError`."]
    RuntimeRunner {
        #[source]
        #[doc = "Carries the source selected for `RuntimeRunner`."]
        source: PlanRunnerError,
        #[doc = "Counts the total number of rollback failures observed by `RuntimeRunner`."]
        rollback_failures_total: u64,
    },
    #[error("runtime worker thread could not start: {message}")]
    #[doc = "Classifies a failure at the runtime worker spawn stage or component of `SessionStartError`."]
    RuntimeWorkerSpawn {
        #[doc = "Carries the diagnostic message reported by `RuntimeWorkerSpawn`."]
        message: String,
        #[doc = "Counts the total number of rollback failures observed by `RuntimeWorkerSpawn`."]
        rollback_failures_total: u64,
    },
    #[error("runtime worker did not become ready: {message}")]
    #[doc = "Classifies a failure at the runtime worker ready stage or component of `SessionStartError`."]
    RuntimeWorkerReady {
        #[doc = "Carries the diagnostic message reported by `RuntimeWorkerReady`."]
        message: String,
        #[doc = "Counts the total number of rollback failures observed by `RuntimeWorkerReady`."]
        rollback_failures_total: u64,
    },
    #[error("Session start was cancelled")]
    #[doc = "Indicates that the operation was cancelled."]
    Cancelled {
        #[doc = "Counts the total number of rollback failures observed by `Cancelled`."]
        rollback_failures_total: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::source_topology_has_input;

    #[test]
    fn given_supported_source_compositions_when_validated_then_each_is_accepted() {
        assert!(source_topology_has_input(1, 0, 0));
        assert!(source_topology_has_input(0, 1, 0));
        assert!(source_topology_has_input(2, 0, 0));
        assert!(source_topology_has_input(0, 2, 0));
        assert!(source_topology_has_input(2, 3, 0));
        assert!(source_topology_has_input(0, 0, 1));
        assert!(source_topology_has_input(2, 1, 3));
    }

    #[test]
    fn given_session_without_source_when_validated_then_topology_is_rejected() {
        assert!(!source_topology_has_input(0, 0, 0));
    }
}

impl SessionStartError {
    #[doc = "Returns the rollback failures total held by `SessionStartError`."]
    pub const fn rollback_failures_total(&self) -> u64 {
        match self {
            Self::EndpointPrepare {
                rollback_failures_total,
                ..
            }
            | Self::CapturePrepare {
                rollback_failures_total,
                ..
            }
            | Self::CaptureOpen {
                rollback_failures_total,
                ..
            }
            | Self::EndpointStart {
                rollback_failures_total,
                ..
            }
            | Self::RuntimeWorkerSpawn {
                rollback_failures_total,
                ..
            }
            | Self::RuntimeRunner {
                rollback_failures_total,
                ..
            }
            | Self::RuntimeWorkerReady {
                rollback_failures_total,
                ..
            }
            | Self::OperatorRuntimeHost {
                rollback_failures_total,
                ..
            }
            | Self::OperatorPrepare {
                rollback_failures_total,
                ..
            }
            | Self::ExternalSourcePrepare {
                rollback_failures_total,
                ..
            }
            | Self::ExternalAudioBridge {
                rollback_failures_total,
                ..
            }
            | Self::GeneratedAudioBridge {
                rollback_failures_total,
                ..
            }
            | Self::ExternalSourceStart {
                rollback_failures_total,
                ..
            }
            | Self::Cancelled {
                rollback_failures_total,
            } => *rollback_failures_total,
            Self::InvalidOptions { .. }
            | Self::UnsupportedSourceTopology
            | Self::MissingEndpointDeclaration { .. } => 0,
        }
    }
}

#[derive(Debug)]
#[doc = "Reports a session start failure."]
pub struct SessionStartFailure {
    pub(super) error: SessionStartError,
    pub(super) event_receiver: Option<SessionEventReceiver>,
    pub(super) rollback_failures: Box<[SessionRollbackFailure]>,
}

impl SessionStartFailure {
    pub(super) fn input(error: SessionStartError) -> Self {
        Self {
            error,
            event_receiver: None,
            rollback_failures: Box::new([]),
        }
    }

    #[doc = "Returns the error held by `SessionStartFailure`."]
    pub const fn error(&self) -> &SessionStartError {
        &self.error
    }

    #[doc = "Returns the rollback failures held by `SessionStartFailure`."]
    pub fn rollback_failures(&self) -> &[SessionRollbackFailure] {
        &self.rollback_failures
    }

    #[doc = "Takes event receiver for `SessionStartFailure`."]
    pub fn take_event_receiver(&mut self) -> Option<SessionEventReceiver> {
        self.event_receiver.take()
    }

    #[doc = "Converts `SessionStartFailure` into error."]
    pub fn into_error(self) -> SessionStartError {
        self.error
    }
}

impl std::fmt::Display for SessionStartFailure {
    #[doc = "Formats `SessionStartFailure` with the requested formatter."]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for SessionStartFailure {
    #[doc = "Returns the source held by `SessionStartFailure`."]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Reports the structured session stop outcome."]
pub struct SessionStopOutcome {
    pub(super) runtime_worker_panicked: bool,
    pub(super) capture_finalization_failures_total: u64,
    pub(super) operator_finalization_failures_total: u64,
    pub(super) endpoint_finalization_failures_total: u64,
    pub(super) runtime_failures_total: u64,
    pub(super) lineage_failures_total: u64,
    pub(super) source_send_rejections_total: u64,
    pub(super) runtime_events_total: u64,
}

impl SessionStopOutcome {
    #[doc = "Reports whether success is true for `SessionStopOutcome`."]
    pub fn is_success(&self) -> bool {
        !self.runtime_worker_panicked
            && self.capture_finalization_failures_total == 0
            && self.operator_finalization_failures_total == 0
            && self.endpoint_finalization_failures_total == 0
            && self.runtime_failures_total == 0
            && self.lineage_failures_total == 0
            && self.source_send_rejections_total == 0
    }

    #[doc = "Returns the capture finalization failures total held by `SessionStopOutcome`."]
    pub const fn capture_finalization_failures_total(&self) -> u64 {
        self.capture_finalization_failures_total
    }

    #[doc = "Returns the endpoint finalization failures total held by `SessionStopOutcome`."]
    pub const fn endpoint_finalization_failures_total(&self) -> u64 {
        self.endpoint_finalization_failures_total
    }

    #[doc = "Returns the operator finalization failures total held by `SessionStopOutcome`."]
    pub const fn operator_finalization_failures_total(&self) -> u64 {
        self.operator_finalization_failures_total
    }

    #[doc = "Returns whether runtime worker panicked is true for `SessionStopOutcome`."]
    pub const fn runtime_worker_panicked(&self) -> bool {
        self.runtime_worker_panicked
    }

    #[doc = "Returns the runtime failures total held by `SessionStopOutcome`."]
    pub const fn runtime_failures_total(&self) -> u64 {
        self.runtime_failures_total
    }

    #[doc = "Returns the lineage failures total held by `SessionStopOutcome`."]
    pub const fn lineage_failures_total(&self) -> u64 {
        self.lineage_failures_total
    }

    #[doc = "Returns the source send rejections total held by `SessionStopOutcome`."]
    pub const fn source_send_rejections_total(&self) -> u64 {
        self.source_send_rejections_total
    }

    #[doc = "Returns the runtime events total held by `SessionStopOutcome`."]
    pub const fn runtime_events_total(&self) -> u64 {
        self.runtime_events_total
    }
}

//! Public contract for starting and stopping a prepared Session.
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

pub struct CaptureBackendSet<'backend> {
    pub application: &'backend dyn CallbackCaptureBackend,
    pub microphone: &'backend dyn CallbackCaptureBackend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionStartOptions {
    pub capture_frame_capacity_frames: usize,
    pub capture_runtime_event_capacity_events: usize,
    pub runtime_work_budget_frames: usize,
    pub runtime_idle_poll_ms: u64,
    pub runtime_ready_timeout_ms: u64,
    pub session_event_capacity_events: usize,
}

impl Default for SessionStartOptions {
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
    let built_in_topology_valid = (application_sources == 0 && microphone_sources == 0)
        || (application_sources == 1 && microphone_sources == 1);
    if built_in_topology_valid
        && (!prepared.spec.source_instances().is_empty() || application_sources == 1)
    {
        Ok(())
    } else {
        Err(SessionStartError::UnsupportedSourceTopology)
    }
}

/// Thread-safe cancellation request for a Session that has not reached
/// `Running` yet.
#[derive(Clone, Debug, Default)]
pub struct SessionStartCancellation {
    requested: Arc<AtomicBool>,
}

impl SessionStartCancellation {
    pub fn request(&self) {
        self.requested.store(true, Ordering::Release);
    }

    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SessionStartError {
    #[error("invalid Session start options: {reason}")]
    InvalidOptions { reason: &'static str },
    #[error("Session requires exactly one application and one microphone source")]
    UnsupportedSourceTopology,
    #[error("external source preparation failed: {message}")]
    ExternalSourcePrepare {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("external source audio ingress failed: {message}")]
    ExternalAudioBridge {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("generated audio reentry failed: {message}")]
    GeneratedAudioBridge {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("external source start failed: {message}")]
    ExternalSourceStart {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("async operator runtime host could not start: {message}")]
    OperatorRuntimeHost {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("operator {operator_instance_id:?} preparation failed: {message}")]
    OperatorPrepare {
        operator_instance_id: OperatorInstanceId,
        message: String,
        rollback_failures_total: u64,
    },
    #[error("endpoint {endpoint_id:?} declaration is absent")]
    MissingEndpointDeclaration { endpoint_id: EndpointId },
    #[error("endpoint preparation failed: {source}")]
    EndpointPrepare {
        #[source]
        source: EndpointPrepareError,
        rollback_failures_total: u64,
    },
    #[error("capture preparation failed for stem {stem_id:?}: {source}")]
    CapturePrepare {
        stem_id: StemId,
        #[source]
        source: CaptureError,
        rollback_failures_total: u64,
    },
    #[error("capture open failed for stem {stem_id:?}: {source}")]
    CaptureOpen {
        stem_id: StemId,
        #[source]
        source: CaptureError,
        rollback_failures_total: u64,
    },
    #[error("endpoint start failed: {source}")]
    EndpointStart {
        #[source]
        source: EndpointStartFailure,
        rollback_failures_total: u64,
    },
    #[error("runtime runner preparation failed: {source}")]
    RuntimeRunner {
        #[source]
        source: PlanRunnerError,
        rollback_failures_total: u64,
    },
    #[error("runtime worker thread could not start: {message}")]
    RuntimeWorkerSpawn {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("runtime worker did not become ready: {message}")]
    RuntimeWorkerReady {
        message: String,
        rollback_failures_total: u64,
    },
    #[error("Session start was cancelled")]
    Cancelled { rollback_failures_total: u64 },
}

impl SessionStartError {
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

    pub const fn error(&self) -> &SessionStartError {
        &self.error
    }

    pub fn rollback_failures(&self) -> &[SessionRollbackFailure] {
        &self.rollback_failures
    }

    pub fn take_event_receiver(&mut self) -> Option<SessionEventReceiver> {
        self.event_receiver.take()
    }

    pub fn into_error(self) -> SessionStartError {
        self.error
    }
}

impl std::fmt::Display for SessionStartFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for SessionStartFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn is_success(&self) -> bool {
        !self.runtime_worker_panicked
            && self.capture_finalization_failures_total == 0
            && self.operator_finalization_failures_total == 0
            && self.endpoint_finalization_failures_total == 0
            && self.runtime_failures_total == 0
            && self.lineage_failures_total == 0
            && self.source_send_rejections_total == 0
    }

    pub const fn capture_finalization_failures_total(&self) -> u64 {
        self.capture_finalization_failures_total
    }

    pub const fn endpoint_finalization_failures_total(&self) -> u64 {
        self.endpoint_finalization_failures_total
    }

    pub const fn operator_finalization_failures_total(&self) -> u64 {
        self.operator_finalization_failures_total
    }

    pub const fn runtime_worker_panicked(&self) -> bool {
        self.runtime_worker_panicked
    }

    pub const fn runtime_failures_total(&self) -> u64 {
        self.runtime_failures_total
    }

    pub const fn lineage_failures_total(&self) -> u64 {
        self.lineage_failures_total
    }

    pub const fn source_send_rejections_total(&self) -> u64 {
        self.source_send_rejections_total
    }

    pub const fn runtime_events_total(&self) -> u64 {
        self.runtime_events_total
    }
}

//! Language-owned Rust façade over the canonical `pks-session` engine.

use std::path::PathBuf;

#[cfg(feature = "conformance-fixtures")]
pub mod conformance;
mod error_code;

pub use pks_session::{
    polled_audio_poll_error_code, session_declaration_error_code,
    session_recording_outcome_error_code, session_start_failure_code, PolledAudioPollErrorCode,
    SessionDeclarationErrorCode, SessionRuntimeErrorCode, SessionStartErrorCode, SessionStopCode,
    SessionStopFailureCode,
};

pub use pks_session::{
    ApplicationSelector, DeviceId, DeviceSelector, EndpointHandle, ProcessId,
    SessionControlFailure, SessionError, SessionEvent, SessionEventKind,
    SessionEventQueueObservations, SessionEventReceive, SessionLifecycleState,
    SessionMetricsSnapshot, SessionRecordingErrorCode, SessionRecordingObservations,
    SessionRecordingOutcome, SessionRecordingState, SessionRecordingStemOutcome,
    SessionStartCancellation, SessionStopOutcome, Source, StemHandle,
};

use pks_session::{
    NativeSessionEngineHostOptions, SessionEngineHost, SessionEngineHostBuildError,
    SessionEngineStartError,
};

pub struct Session {
    declaration: pks_session::Session,
    host: Option<SessionEngineHost>,
    recording_root: Option<PathBuf>,
}

/// Setup-time configuration for the public Rust Session.
#[derive(Debug, Default)]
pub struct SessionBuilder {
    recording_root: Option<PathBuf>,
}

impl SessionBuilder {
    /// Configures the artifact root used by declared multistem recording routes.
    #[must_use]
    pub fn recording_root(mut self, output_root: impl Into<PathBuf>) -> Self {
        self.recording_root = Some(output_root.into());
        self
    }

    /// Builds the Session declaration owner.
    #[must_use]
    pub fn build(self) -> Session {
        Session {
            declaration: pks_session::Session::new(),
            host: None,
            recording_root: self.recording_root,
        }
    }
}

impl Session {
    pub fn new() -> Self {
        Self {
            declaration: pks_session::Session::new(),
            host: None,
            recording_root: None,
        }
    }

    pub fn builder() -> SessionBuilder {
        SessionBuilder::default()
    }

    pub fn id(&self) -> pks_session::SessionId {
        self.declaration.id()
    }

    pub fn capture(&self, source: Source) -> Result<StemHandle, SessionError> {
        self.declaration.capture(source)
    }

    pub fn polled_audio(&self) -> Result<EndpointHandle, SessionError> {
        self.declaration.polled_audio()
    }

    pub fn start(self) -> Result<RunningSession, SessionStartError> {
        self.start_cancellable(SessionStartCancellation::default())
    }

    pub fn start_cancellable(
        self,
        cancellation: SessionStartCancellation,
    ) -> Result<RunningSession, SessionStartError> {
        let Self {
            declaration,
            host,
            recording_root,
        } = self;
        let recording_declared = declaration
            .declares_multistem_recording()
            .map_err(|error| {
                SessionStartError::Engine(pks_session::SessionEngineStartError::Freeze(error))
            })?;
        let host = match host {
            Some(host) => host,
            None => match recording_root {
                Some(output_root) if !output_root.as_os_str().is_empty() => {
                    SessionEngineHost::native_with_multistem_recording(
                        NativeSessionEngineHostOptions::default(),
                        output_root,
                    )?
                }
                Some(_) | None => {
                    SessionEngineHost::native(NativeSessionEngineHostOptions::default())?
                }
            },
        };
        let recording_receipt = host.recording_receipt(0);
        if recording_declared && recording_receipt.is_none() {
            return Err(SessionStartError::MissingRecordingConfiguration);
        }
        let compiled = host.compile(declaration)?;
        let receipt = host
            .polled_audio_receipt(0)
            .ok_or(SessionStartError::MissingAudioReceipt)?;
        let mut running = host.start_compiled_cancellable(compiled, cancellation)?;
        let Some(events) = running.take_event_receiver() else {
            let _ = running.stop();
            return Err(SessionStartError::MissingEventReceiver);
        };
        Ok(RunningSession {
            host,
            running,
            events,
            receipt,
            recording_receipt,
            stopped: false,
        })
    }

    #[cfg(feature = "conformance-fixtures")]
    fn with_host(host: SessionEngineHost) -> Result<Self, SessionEngineHostBuildError> {
        Ok(Self {
            declaration: pks_session::Session::new(),
            host: Some(host),
            recording_root: None,
        })
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RunningSession {
    host: SessionEngineHost,
    running: pks_session::RunningSession,
    events: pks_session::SessionEventReceiver,
    receipt: pks_session::PolledAudioReceipt,
    recording_receipt: Option<pks_session::SessionRecordingReceipt>,
    stopped: bool,
}

impl RunningSession {
    pub fn session_id(&self) -> pks_session::SessionId {
        self.running.session_id()
    }

    pub fn try_poll_audio(&self) -> Result<PolledAudioBatchLease, PolledAudioPollError> {
        self.receipt.try_poll()
    }

    pub fn audio_observations(&self) -> PolledAudioObservations {
        self.receipt.observations()
    }

    pub fn recording_outcome(&self) -> Option<&pks_session::SessionRecordingOutcome> {
        self.recording_receipt
            .as_ref()
            .and_then(pks_session::SessionRecordingReceipt::outcome)
    }

    pub fn try_recv_event(&self) -> SessionEventReceive {
        self.events.try_recv()
    }

    pub fn event_observations(&self) -> SessionEventQueueObservations {
        self.events.observations()
    }

    pub fn metrics_snapshot(&self) -> Result<SessionMetricsSnapshot, SessionRuntimeError> {
        self.host
            .metrics_snapshot(&self.events, 0, Some(&self.running))
            .ok_or(SessionRuntimeError::MissingMetricsSnapshot)
    }

    pub fn stop(&mut self) -> SessionStopResult {
        let disposition = if self.stopped {
            SessionStopDisposition::AlreadyStopped
        } else {
            self.stopped = true;
            SessionStopDisposition::Stopped
        };
        SessionStopResult {
            disposition,
            outcome: self.running.stop(),
        }
    }
}

pub use pks_session::{
    PolledAudioBatchLease, PolledAudioFrame, PolledAudioObservations, PolledAudioPollError,
};

#[derive(Debug, thiserror::Error)]
pub enum SessionStartError {
    #[error("native Session host setup failed: {0}")]
    Host(#[from] SessionEngineHostBuildError),
    #[error("canonical Session start failed: {0}")]
    Engine(#[from] SessionEngineStartError),
    #[error("native Session host did not retain its bounded audio receipt")]
    MissingAudioReceipt,
    #[error("recording routes require an explicit Session recording root")]
    MissingRecordingConfiguration,
    #[error("canonical running Session did not retain its event receiver")]
    MissingEventReceiver,
}

impl SessionStartError {
    pub fn kind(&self) -> SessionStartErrorKind {
        match self {
            Self::Host(_) => SessionStartErrorKind::Host,
            Self::Engine(error)
                if matches!(
                    error.start_failure().map(|failure| failure.error()),
                    Some(pks_session::SessionStartError::Cancelled { .. })
                ) =>
            {
                SessionStartErrorKind::Cancelled
            }
            Self::Engine(SessionEngineStartError::Freeze(SessionError::InvalidSelector {
                ..
            })) => SessionStartErrorKind::InvalidSelector,
            Self::Engine(_) => SessionStartErrorKind::Engine,
            Self::MissingRecordingConfiguration => {
                SessionStartErrorKind::MissingRecordingConfiguration
            }
            Self::MissingAudioReceipt | Self::MissingEventReceiver => {
                SessionStartErrorKind::Invariant
            }
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.kind() == SessionStartErrorKind::Cancelled
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStartErrorKind {
    Host,
    Engine,
    Cancelled,
    InvalidSelector,
    MissingRecordingConfiguration,
    Invariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SessionRuntimeError {
    #[error("native running Session did not expose a metrics snapshot")]
    MissingMetricsSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStopDisposition {
    Stopped,
    AlreadyStopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionStopResult {
    disposition: SessionStopDisposition,
    outcome: SessionStopOutcome,
}

impl SessionStopResult {
    pub fn disposition(self) -> SessionStopDisposition {
        self.disposition
    }

    pub fn outcome(self) -> SessionStopOutcome {
        self.outcome
    }

    pub fn is_success(self) -> bool {
        self.outcome.is_success()
    }
}

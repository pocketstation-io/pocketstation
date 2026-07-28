//! Language-owned Rust façade over the canonical `pks-session` engine.

#[cfg(feature = "conformance-fixtures")]
pub mod conformance;

pub use pks_session::{
    ApplicationSelector, DeviceId, DeviceSelector, EndpointHandle, ProcessId,
    SessionControlFailure, SessionError, SessionEvent, SessionEventKind,
    SessionEventQueueObservations, SessionEventReceive, SessionLifecycleState,
    SessionMetricsSnapshot, SessionStartCancellation, SessionStopOutcome, Source, StemHandle,
};

use pks_session::{
    NativeSessionEngineHostOptions, SessionEngineHost, SessionEngineHostBuildError,
    SessionEngineStartError,
};

pub struct Session {
    declaration: pks_session::Session,
    host: Option<SessionEngineHost>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            declaration: pks_session::Session::new(),
            host: None,
        }
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
        let host = match self.host {
            Some(host) => host,
            None => SessionEngineHost::native(NativeSessionEngineHostOptions::default())?,
        };
        let compiled = host.compile(self.declaration)?;
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
            stopped: false,
        })
    }

    #[cfg(feature = "conformance-fixtures")]
    fn with_host(host: SessionEngineHost) -> Result<Self, SessionEngineHostBuildError> {
        Ok(Self {
            declaration: pks_session::Session::new(),
            host: Some(host),
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

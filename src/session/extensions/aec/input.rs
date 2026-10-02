use crate::{
    DerivedStreamHandle, Session, SessionError, SessionId, SourceOutputHandle, StemHandle,
    StreamOrigin,
};

/// What is known about echo processing upstream of an input.
///
/// `None` is unknown, not raw audio. `true` means a cancellation stage is
/// declared, not that it converged or that its acoustic quality is qualified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EchoInputProcessing {
    pub echo_processed: Option<bool>,
    /// `session` is Core-owned ancestry; `caller` is an explicit input declaration.
    /// `unknown` means no evidence is available. None is OS/device qualification.
    pub evidence: &'static str,
}

/// A Session-owned audio source selected for echo processing.
#[derive(Clone, Debug)]
pub struct EchoAudioInput {
    pub(super) session_id: SessionId,
    pub(super) origin: StreamOrigin,
    echo_processed: Option<bool>,
}

impl EchoAudioInput {
    /// Declares processing already applied by the embedding application's source.
    ///
    /// This is caller evidence, not automatic device discovery. It cannot erase
    /// Core's knowledge that a stream already passes through cancellation.
    pub fn with_echo_processing(mut self, echo_processed: bool) -> Self {
        self.echo_processed = Some(echo_processed);
        self
    }
}

impl From<&StemHandle> for EchoAudioInput {
    fn from(value: &StemHandle) -> Self {
        let (session_id, origin) = value.signal_origin();
        Self {
            session_id,
            origin,
            echo_processed: None,
        }
    }
}

impl From<&SourceOutputHandle> for EchoAudioInput {
    fn from(value: &SourceOutputHandle) -> Self {
        let (session_id, origin) = value.signal_origin();
        Self {
            session_id,
            origin,
            echo_processed: None,
        }
    }
}

impl From<&DerivedStreamHandle> for EchoAudioInput {
    fn from(value: &DerivedStreamHandle) -> Self {
        let (session_id, origin) = value.signal_origin();
        Self {
            session_id,
            origin,
            echo_processed: None,
        }
    }
}

impl Session {
    /// Inspects declaration evidence without opening a device or enabling AEC.
    ///
    /// Generic computation cannot erase a known upstream cancellation stage.
    /// An output with any processed ancestor is conservatively treated as
    /// processed; this does not infer which samples an external operator uses.
    pub fn echo_input_processing(
        &self,
        input: impl Into<EchoAudioInput>,
    ) -> Result<EchoInputProcessing, SessionError> {
        let input = input.into();
        if input.session_id != self.id() {
            return Err(SessionError::InvalidOperator {
                reason: "echo input belongs to a different Session".into(),
            });
        }
        if self.declaration.has_echo_processing(&input.origin)? {
            return Ok(EchoInputProcessing {
                echo_processed: Some(true),
                evidence: "session",
            });
        }
        Ok(EchoInputProcessing {
            echo_processed: input.echo_processed,
            evidence: if input.echo_processed.is_some() {
                "caller"
            } else {
                "unknown"
            },
        })
    }
}

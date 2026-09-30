use crate::aec::{AecConfiguration, AecOperatorFactory, ObservationState, ReferenceChannels};
use crate::{
    DerivedStreamHandle, EchoCancellationObservations, Operator, OperatorConfiguration, OperatorId,
    Session, SessionError, SessionId, SourceOutputHandle, StemHandle, StreamOrigin,
};
use std::sync::Arc;

/// A Session-owned audio source selected for echo processing.
#[derive(Clone, Debug)]
pub struct EchoAudioInput {
    session_id: SessionId,
    origin: StreamOrigin,
}

impl From<&StemHandle> for EchoAudioInput {
    fn from(value: &StemHandle) -> Self {
        let (session_id, origin) = value.signal_origin();
        Self { session_id, origin }
    }
}
impl From<&SourceOutputHandle> for EchoAudioInput {
    fn from(value: &SourceOutputHandle) -> Self {
        let (session_id, origin) = value.signal_origin();
        Self { session_id, origin }
    }
}
impl From<&DerivedStreamHandle> for EchoAudioInput {
    fn from(value: &DerivedStreamHandle) -> Self {
        let (session_id, origin) = value.signal_origin();
        Self { session_id, origin }
    }
}

/// Explicit permission to use an already-declared stream as an echo reference.
/// This never opens, records or sends another application's audio.
#[derive(Clone, Debug)]
pub struct PlaybackReference {
    input: EchoAudioInput,
    coverage: &'static str,
}

impl PlaybackReference {
    /// Limits reference coverage to the selected application's audio.
    pub fn selected_application(input: impl Into<EchoAudioInput>) -> Self {
        Self {
            input: input.into(),
            coverage: "selected-application",
        }
    }
    /// Uses an output mix that the caller has separately authorized and captured.
    pub fn output_mix(input: impl Into<EchoAudioInput>) -> Self {
        Self {
            input: input.into(),
            coverage: "authorized-output-mix",
        }
    }
    /// Uses caller-owned audio that is also submitted to a playback device.
    pub fn rendered_audio(input: impl Into<EchoAudioInput>) -> Self {
        Self {
            input: input.into(),
            coverage: "caller-rendered-audio",
        }
    }
}

/// Processed audio and retained observations from one echo-cancellation instance.
#[derive(Clone)]
pub struct EchoCancelledAudio {
    audio: StemHandle,
    microphone: EchoAudioInput,
    reference: PlaybackReference,
    observations: ObservationState,
}

impl EchoCancelledAudio {
    /// Routes through the same endpoint and recording methods as captured audio.
    pub fn audio(&self) -> &StemHandle {
        &self.audio
    }
    /// Processing observations remain readable after the Session stops.
    pub fn observations(&self) -> EchoCancellationObservations {
        self.observations.snapshot()
    }
    /// The original microphone's Session stream identity.
    pub fn microphone_origin(&self) -> &StreamOrigin {
        &self.microphone.origin
    }
    /// The reference is a dependency, separate from the microphone origin.
    pub fn reference_origin(&self) -> &StreamOrigin {
        &self.reference.input.origin
    }
    pub fn reference_coverage(&self) -> &str {
        self.reference.coverage
    }
}

impl Session {
    /// Declares built-in echo cancellation using two explicitly selected audio inputs.
    ///
    /// The processor preserves mono or stereo channels and runs on its own native
    /// thread. Both inputs currently require the Session's 48 kHz PCM format and
    /// aligned source clocks. Additional device-clock preparation is not inferred.
    /// The original inputs remain available for independent routing.
    pub fn echo_cancel(
        &self,
        microphone: impl Into<EchoAudioInput>,
        reference: PlaybackReference,
    ) -> Result<EchoCancelledAudio, SessionError> {
        let microphone = microphone.into();
        if microphone.session_id != self.id() || reference.input.session_id != self.id() {
            return Err(SessionError::InvalidOperator {
                reason: "echo inputs belong to a different Session".into(),
            });
        }
        if microphone.origin == reference.input.origin {
            return Err(SessionError::InvalidOperator {
                reason: "microphone and playback reference must be different streams".into(),
            });
        }
        if self.sample_spec.sample_rate_hz != 48_000 || !matches!(self.sample_spec.channels, 1 | 2)
        {
            return Err(SessionError::InvalidOperator {
                reason: "echo cancellation currently supports 48 kHz mono or stereo PCM".into(),
            });
        }
        let configuration = AecConfiguration::new(
            u32::from(self.audio_frame_duration.milliseconds()),
            if self.sample_spec.channels == 1 {
                ReferenceChannels::Mono
            } else {
                ReferenceChannels::Stereo
            },
        );
        let observations = ObservationState::new(configuration.capacity_frames());
        let mut factories = self
            .operator_registrations
            .lock()
            .map_err(|_| SessionError::DraftPoisoned)?;
        let operator_id = OperatorId::new(format!("io.pocketstation.aec3.{}", factories.len()));
        let factory =
            AecOperatorFactory::new(configuration, operator_id.clone(), observations.clone())
                .map_err(|e| SessionError::InvalidOperator {
                    reason: e.to_string(),
                })?;
        let audio = self.declaration.connected_audio_operator(
            Operator::new(operator_id, OperatorConfiguration::new()),
            &[
                (
                    microphone.session_id,
                    microphone.origin.clone(),
                    "microphone",
                ),
                (
                    reference.input.session_id,
                    reference.input.origin.clone(),
                    "reference",
                ),
            ],
            "microphone",
        )?;
        factories.push(Arc::new(factory));
        Ok(EchoCancelledAudio {
            audio,
            microphone,
            reference,
            observations,
        })
    }
}

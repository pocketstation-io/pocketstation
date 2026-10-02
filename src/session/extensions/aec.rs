use crate::aec::ObservationState;
#[cfg(feature = "aec")]
use crate::aec::{AecConfiguration, AecOperatorFactory, Channels};
#[cfg(feature = "aec")]
use crate::session::{compile::select_operator_port, declaration::OriginDefinition};
#[cfg(feature = "aec")]
use crate::{
    AudioCaps, ChannelLayout, MediaCaps, Operator, OperatorConfiguration, OperatorId,
    PortDirection, SampleFormat,
};
mod input;
mod selection;
use crate::{EchoCancellationObservations, Session, SessionError, StemHandle, StreamOrigin};
pub use input::{EchoAudioInput, EchoInputProcessing};
pub use selection::NativePlaybackReference;
#[cfg(feature = "aec")]
use std::sync::Arc;

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

#[cfg(feature = "aec")]
impl Session {
    /// Declares built-in echo cancellation using two explicitly selected audio inputs.
    ///
    /// The processor preserves mono or stereo channels and runs on its own native
    /// thread. Each input declares its own mono or stereo layout at 48 kHz and
    /// the Session's frame cadence. Inputs require a shared clock domain;
    /// additional device-clock preparation is not inferred.
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
        if self
            .declaration
            .has_native_aec_request(&microphone.origin)?
        {
            return Err(SessionError::InvalidOperator {
                reason: "native AEC is already requested upstream; use that microphone without a portable stage".into(),
            });
        }
        if self
            .echo_input_processing(microphone.clone())?
            .echo_processed
            == Some(true)
        {
            return Err(SessionError::InvalidOperator {
                reason: "echo input already passes through cancellation; route the existing processed audio instead".into(),
            });
        }
        // Read owned declaration snapshots and consult cloned factory Arcs outside
        // both draft and registration locks. Rejection leaves the draft intact.
        let microphone_channels = self.echo_input_channels(&microphone.origin)?;
        let reference_channels = self.echo_input_channels(&reference.input.origin)?;
        let configuration = AecConfiguration::with_channels(
            u32::from(self.audio_frame_duration.milliseconds()),
            microphone_channels,
            reference_channels,
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
            Operator::new(operator_id, OperatorConfiguration::new()).with_echo_processing(),
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

    fn echo_input_channels(&self, origin: &StreamOrigin) -> Result<Channels, SessionError> {
        let invalid = |reason: String| SessionError::InvalidOperator { reason };
        let media = match self.declaration.origin_definition(origin)? {
            OriginDefinition::Capture(source) => {
                MediaCaps::Audio(super::builtins::capture_audio_caps(
                    &source,
                    self.sample_spec,
                    self.audio_frame_duration,
                ))
            }
            OriginDefinition::SourceOutput {
                source_type_id,
                configuration,
                output_port,
            } => {
                let factories = self
                    .source_registrations
                    .lock()
                    .map_err(|_| SessionError::DraftPoisoned)?
                    .clone();
                let factory = factories
                    .iter()
                    .find(|factory| factory.manifest().source_type_id() == &source_type_id)
                    .ok_or_else(|| {
                        invalid(format!(
                            "echo input source {source_type_id} is not registered"
                        ))
                    })?;
                factory
                    .validate_config(&configuration)
                    .map_err(|error| invalid(error.to_string()))?;
                let port = factory
                    .manifest()
                    .output_port(&output_port)
                    .ok_or_else(|| {
                        invalid(format!("echo input source has no output {output_port}"))
                    })?;
                if !port.signal.class.is_audio() {
                    return Err(invalid("echo input must be an audio output".into()));
                }
                port.media
            }
            OriginDefinition::OperatorOutput {
                operator,
                output_port,
            } => {
                let factories = self
                    .operator_registrations
                    .lock()
                    .map_err(|_| SessionError::DraftPoisoned)?
                    .clone();
                let factory = factories
                    .iter()
                    .find(|factory| &factory.manifest().operator_id == operator.operator_id())
                    .ok_or_else(|| invalid("echo input operator is not registered".into()))?;
                let manifest = factory
                    .resolve_manifest(operator.configuration())
                    .map_err(|error| invalid(error.to_string()))?;
                let port =
                    select_operator_port(&manifest, PortDirection::Output, output_port.as_deref())
                        .map_err(|error| invalid(error.to_string()))?;
                let declared = select_operator_port(
                    factory.manifest(),
                    PortDirection::Output,
                    output_port.as_deref(),
                )
                .map_err(|error| invalid(error.to_string()))?;
                if !port.signal.class.is_audio() || port.media != declared.media {
                    return Err(invalid(
                        "echo input requires a stable declared audio format".into(),
                    ));
                }
                port.media
            }
        };
        let MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(48_000),
            frame_samples: Some(samples),
            channel_layout,
            format: SampleFormat::F32Interleaved,
        }) = media
        else {
            return Err(invalid(
                "echo input requires concrete 48 kHz f32 PCM".into(),
            ));
        };
        if self.sample_spec.sample_rate_hz != 48_000
            || samples != self.audio_frame_duration.samples_per_channel(48_000)
        {
            return Err(invalid(
                "echo input must use the Session's 48 kHz frame cadence".into(),
            ));
        }
        match channel_layout {
            ChannelLayout::Mono => Ok(Channels::Mono),
            ChannelLayout::Stereo => Ok(Channels::Stereo),
            ChannelLayout::Any => Err(invalid(
                "echo input requires a concrete channel layout".into(),
            )),
        }
    }
}

#[cfg(not(feature = "aec"))]
impl Session {
    /// AEC is opt-in. An unavailable engine leaves the Session declaration unchanged.
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
        if self
            .declaration
            .has_native_aec_request(&microphone.origin)?
        {
            return Err(SessionError::InvalidOperator {
                reason: "native AEC is already requested upstream; use that microphone without a portable stage".into(),
            });
        }
        Err(SessionError::InvalidOperator {
            reason: "AEC is unavailable in this build; enable the `aec` Cargo feature".into(),
        })
    }
}

#[cfg(all(test, feature = "aec"))]
mod tests {
    use super::*;
    use crate::graph::PrepareContext;
    use crate::session::{SessionEngineBuilder, SessionStartOptions};
    use crate::{ApplicationSelector, AudioFrameDuration, DeviceSelector, SampleSpec, Source};

    #[test]
    fn given_native_mono_microphone_and_stereo_capture_when_declared_then_compiles_without_downmix()
    {
        for duration in [AudioFrameDuration::Ms10, AudioFrameDuration::Ms20] {
            for default_channels in [1, 2] {
                for source in [
                    Source::Application(ApplicationSelector::name("Browser")),
                    Source::SystemAudio,
                ] {
                    let session = Session::builder()
                        .sample_spec(SampleSpec::new(
                            48_000,
                            default_channels,
                            SampleFormat::F32Interleaved,
                        ))
                        .audio_frame_duration(duration)
                        .build();
                    let microphone = session
                        .capture(Source::Microphone(DeviceSelector::Default))
                        .unwrap();
                    let reference = session.capture(source).unwrap();
                    let processed = session
                        .echo_cancel(&microphone, PlaybackReference::output_mix(&reference))
                        .unwrap();
                    processed
                        .audio()
                        .send(session.polled_audio().unwrap())
                        .unwrap();
                    let endpoint =
                        crate::endpoint::PolledAudioEndpoint::new(session.polled_audio_endpoint)
                            .unwrap();
                    let mut engine = SessionEngineBuilder::new_with_audio_frame_duration(
                        PrepareContext::new(session.sample_spec),
                        duration,
                        8,
                        SessionStartOptions::default(),
                    )
                    .unwrap();
                    engine.register_polled_audio_endpoint(&endpoint).unwrap();
                    for factory in session.operator_registrations.into_inner().unwrap() {
                        let manifest = factory.manifest();
                        assert_eq!(
                            manifest.node.inputs[0].media,
                            MediaCaps::Audio(AudioCaps {
                                sample_rate_hz: Some(48_000),
                                frame_samples: Some(duration.samples_per_channel(48_000)),
                                channel_layout: ChannelLayout::Mono,
                                format: SampleFormat::F32Interleaved,
                            })
                        );
                        assert!(matches!(
                            manifest.node.inputs[1].media,
                            MediaCaps::Audio(AudioCaps {
                                channel_layout: ChannelLayout::Stereo,
                                ..
                            })
                        ));
                        engine.register_async_operator(factory).unwrap();
                    }
                    let compiled = engine
                        .build()
                        .unwrap()
                        .compile(session.declaration)
                        .unwrap();
                    let generated = compiled
                        .graph_ir()
                        .nodes
                        .iter()
                        .find(|node| {
                            node.spec
                                .type_id
                                .as_str()
                                .starts_with("source.generated_audio_ingress")
                        })
                        .unwrap();
                    assert!(matches!(
                        generated.descriptor.outputs[0].media,
                        MediaCaps::Audio(AudioCaps {
                            channel_layout: ChannelLayout::Mono,
                            ..
                        })
                    ));
                }
            }
        }
    }

    struct DeclarationSource {
        manifest: crate::SourceManifest,
        session: std::sync::Weak<Session>,
        reject_configuration: bool,
    }

    impl crate::SourceFactory for DeclarationSource {
        fn manifest(&self) -> &crate::SourceManifest {
            if let Some(session) = self.session.upgrade() {
                assert!(
                    session.source_registrations.try_lock().is_ok(),
                    "factory called under registrations lock"
                );
            }
            &self.manifest
        }
        fn validate_config(
            &self,
            _: &crate::SourceConfiguration,
        ) -> Result<(), crate::ConfigError> {
            if self.reject_configuration {
                Err(crate::ConfigError::Invalid {
                    key: "fixture".into(),
                    reason: "rejected by source".into(),
                })
            } else {
                Ok(())
            }
        }
        fn create(
            &self,
            _: &crate::SourceConfiguration,
        ) -> Result<Box<dyn crate::SourceDriver>, crate::SourceDriverError> {
            Err(crate::SourceDriverError::Failed(
                "declaration-only fixture must never run".into(),
            ))
        }
    }

    #[test]
    fn given_unresolved_or_incompatible_input_when_echo_declared_then_no_operator_or_routes_are_added(
    ) {
        for (rate, frame_samples, layout, reject_configuration, registered) in [
            (Some(44_100), Some(960), ChannelLayout::Mono, false, true),
            (Some(48_000), Some(480), ChannelLayout::Mono, false, true),
            (Some(48_000), Some(960), ChannelLayout::Any, false, true),
            (None, Some(960), ChannelLayout::Mono, false, true),
            (Some(48_000), None, ChannelLayout::Mono, false, true),
            (Some(48_000), Some(960), ChannelLayout::Mono, true, true),
            (Some(48_000), Some(960), ChannelLayout::Mono, false, false),
        ] {
            let session = Arc::new(Session::new());
            let microphone = session
                .capture(Source::Microphone(DeviceSelector::Default))
                .unwrap();
            let reference = session.capture(Source::SystemAudio).unwrap();
            let source_type =
                crate::SourceTypeId::new("io.pocketstation.source.declaration.v1").unwrap();
            if registered {
                let manifest = crate::SourceManifest::new(
                    source_type.clone(),
                    1,
                    1,
                    vec![crate::PortSpec::new(
                        "audio",
                        PortDirection::Output,
                        crate::SignalSpec::audio(),
                        MediaCaps::Audio(AudioCaps {
                            sample_rate_hz: rate,
                            frame_samples,
                            channel_layout: layout,
                            format: SampleFormat::F32Interleaved,
                        }),
                        crate::Multiplicity::Many,
                        true,
                    )
                    .unwrap()],
                    crate::ExecutionPartition::BlockingWorker,
                    crate::ExecutionSafety::AllocationAllowed,
                )
                .unwrap();
                session
                    .register_source(Arc::new(DeclarationSource {
                        manifest,
                        session: Arc::downgrade(&session),
                        reject_configuration,
                    }))
                    .unwrap();
            }
            let invalid = session
                .source(source_type, crate::SourceConfiguration::default())
                .unwrap()
                .output("audio")
                .unwrap();
            invalid.send(session.polled_audio().unwrap()).unwrap();
            assert!(session
                .echo_cancel(&invalid, PlaybackReference::output_mix(&reference))
                .is_err());
            assert!(session
                .echo_cancel(&microphone, PlaybackReference::rendered_audio(&invalid))
                .is_err());
            assert!(session.operator_registrations.lock().unwrap().is_empty());
            let processed = session
                .echo_cancel(&microphone, PlaybackReference::output_mix(&reference))
                .unwrap();
            assert_eq!(
                processed.audio().id(),
                crate::StemId(3),
                "failed declaration consumed an ID"
            );
            assert_eq!(
                session
                    .echo_input_channels(&EchoAudioInput::from(processed.audio()).origin)
                    .unwrap()
                    .count(),
                1
            );
            processed
                .audio()
                .send(session.polled_audio().unwrap())
                .unwrap();
            let session = Arc::try_unwrap(session).ok().unwrap();
            let spec = session.declaration.freeze().unwrap();
            assert_eq!(spec.operators().len(), 1);
            assert_eq!(spec.generated_audio_ingresses().len(), 1);
            assert_eq!(spec.connections().len(), 4);
        }
    }
}

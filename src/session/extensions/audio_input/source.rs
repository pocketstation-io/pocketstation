use std::collections::BTreeMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::frame::{AudioBufferPool, AudioFrame};
use crate::graph::{
    AudioCaps, ChannelLayout, ConfigError, ExecutionPartition, MediaCaps, Multiplicity,
    PortDirection, PortSpec, SafetyContract, SignalEnvelope, SignalLineage, SignalSpec,
    SignalTiming,
};
use crate::runtime::{SignalEdge, SignalEdgeReceiver};
use crate::session::declaration::{SourceInstanceHandle, SourceOutputHandle};
use crate::timing::{monotonic_timestamp_ns, PROCESS_MONOTONIC_CLOCK_DOMAIN_ID};
use crate::{SampleSpec, SessionError};

use super::super::source::{
    SourceCancellation, SourceConfiguration, SourceDriver, SourceDriverError, SourceEmission,
    SourceFactory, SourceManifest, SourceManifestError, SourcePrepareContext, SourceSessionContext,
    SourceTypeId, SourceTypeIdError,
};
use super::buffer::{AudioInputState, AudioInputWriter, QueuedAudioInputFrame};
use super::{AudioInputConfig, AudioInputConfigError, PCM_SOURCE_TYPE_ID};

const OUTPUT_PORT: &str = "audio";
const DRIVER_IDLE_WAIT: Duration = Duration::from_millis(2);

static NEXT_AUDIO_INPUT_WRITER_ID: AtomicU64 = AtomicU64::new(1);

/// Low-level PCM source ownership for integrations that separately retain the
/// Session handles and producer writer.
pub struct PcmSource {
    source: SourceInstanceHandle,
    output: SourceOutputHandle,
    writer: AudioInputWriter,
}

impl PcmSource {
    pub(crate) fn new(
        source: SourceInstanceHandle,
        output: SourceOutputHandle,
        writer: AudioInputWriter,
    ) -> Self {
        Self {
            source,
            output,
            writer,
        }
    }

    #[doc = "Returns the source held by `PcmSource`."]
    pub const fn source(&self) -> &SourceInstanceHandle {
        &self.source
    }

    #[doc = "Returns the output held by `PcmSource`."]
    pub const fn output(&self) -> &SourceOutputHandle {
        &self.output
    }

    #[doc = "Returns the writer held by `PcmSource`."]
    pub const fn writer(&self) -> &AudioInputWriter {
        &self.writer
    }

    #[doc = "Returns the writer mut held by `PcmSource`."]
    pub fn writer_mut(&mut self) -> &mut AudioInputWriter {
        &mut self.writer
    }

    #[doc = "Consumes `PcmSource` and returns its component values."]
    pub fn into_parts(self) -> (SourceInstanceHandle, SourceOutputHandle, AudioInputWriter) {
        (self.source, self.output, self.writer)
    }
}

impl fmt::Debug for PcmSource {
    #[doc = "Formats `PcmSource` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PcmSource")
            .field("source", &self.source)
            .field("output", &self.output)
            .field("writer", &self.writer)
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures surfaced by audio input operations."]
pub enum AudioInputError {
    #[error("invalid audio input configuration: {0}")]
    #[doc = "Classifies a failure at the configuration stage or component of `AudioInputError`."]
    Configuration(#[from] AudioInputConfigError),
    #[error("invalid PCM source identity: {0}")]
    #[doc = "Classifies a failure at the source type identifier stage or component of `AudioInputError`."]
    SourceTypeId(#[from] SourceTypeIdError),
    #[error("audio input manifest failed: {0}")]
    #[doc = "Classifies a failure at the manifest stage or component of `AudioInputError`."]
    Manifest(#[from] SourceManifestError),
    #[error("audio input Session declaration failed: {0}")]
    #[doc = "Classifies a failure at the session stage or component of `AudioInputError`."]
    Session(#[from] SessionError),
    #[error("audio input registration state is unavailable")]
    #[doc = "Reports that registration state is unavailable."]
    RegistrationStateUnavailable,
    #[error(
        "all audio inputs in one Session must use the same concrete sample and frame contract"
    )]
    #[doc = "Reports that contract is incompatible with the required contract."]
    IncompatibleContract,
    #[error("audio input instance identity space is exhausted")]
    #[doc = "Reports that the available instance identity range or capacity is exhausted."]
    InstanceIdentityExhausted,
}

struct PendingAudioInput {
    receiver: SignalEdgeReceiver<QueuedAudioInputFrame>,
    state: Arc<AudioInputState>,
}

pub(crate) struct AudioInputReservation {
    pending: PendingAudioInput,
    writer: AudioInputWriter,
}

pub(crate) struct AudioInputFactory {
    manifest: SourceManifest,
    sample_spec: SampleSpec,
    frame_samples_per_channel: usize,
    pending: Arc<Mutex<BTreeMap<u64, PendingAudioInput>>>,
}

struct AudioInputDriver {
    pending: Arc<Mutex<BTreeMap<u64, PendingAudioInput>>>,
    receiver: Option<SignalEdgeReceiver<QueuedAudioInputFrame>>,
    state: Option<Arc<AudioInputState>>,
    sample_spec: SampleSpec,
    session: Option<SourceSessionContext>,
    receiver_thread_registered: bool,
}

impl SourceFactory for AudioInputFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }

    fn validate_config(&self, configuration: &SourceConfiguration) -> Result<(), ConfigError> {
        if configuration.iter().next().is_some() {
            Err(ConfigError::Invalid {
                key: "<configuration>".to_owned(),
                reason: "audio input does not accept public configuration fields".to_owned(),
            })
        } else {
            Ok(())
        }
    }

    fn create(
        &self,
        configuration: &SourceConfiguration,
    ) -> Result<Box<dyn SourceDriver>, SourceDriverError> {
        self.validate_config(configuration)
            .map_err(|error| SourceDriverError::Failed(error.to_string()))?;
        Ok(Box::new(AudioInputDriver {
            pending: Arc::clone(&self.pending),
            receiver: None,
            state: None,
            sample_spec: self.sample_spec,
            session: None,
            receiver_thread_registered: false,
        }))
    }
}

impl AudioInputFactory {
    pub(crate) fn new(config: AudioInputConfig) -> Result<Arc<Self>, AudioInputError> {
        let channel_layout = match config.sample_spec.channels {
            1 => ChannelLayout::Mono,
            2 => ChannelLayout::Stereo,
            _ => return Err(AudioInputConfigError::UnsupportedChannelCount.into()),
        };
        let manifest = SourceManifest::new(
            SourceTypeId::new(PCM_SOURCE_TYPE_ID)?,
            1,
            1,
            vec![PortSpec {
                name: OUTPUT_PORT.to_owned(),
                direction: PortDirection::Output,
                signal: SignalSpec::audio(),
                media: MediaCaps::Audio(AudioCaps {
                    sample_rate_hz: Some(config.sample_spec.sample_rate_hz),
                    frame_samples: Some(config.frame_samples_per_channel),
                    channel_layout,
                    format: config.sample_spec.format,
                }),
                multiplicity: Multiplicity::Many,
                required: true,
            }],
            ExecutionPartition::BlockingWorker,
            SafetyContract::AllocationAllowed,
        )?;
        Ok(Arc::new(Self {
            manifest,
            sample_spec: config.sample_spec,
            frame_samples_per_channel: config.frame_samples_per_channel,
            pending: Arc::new(Mutex::new(BTreeMap::new())),
        }))
    }

    pub(crate) fn reserve(
        &self,
        config: AudioInputConfig,
    ) -> Result<AudioInputReservation, AudioInputError> {
        if config.sample_spec != self.sample_spec
            || config.frame_samples_per_channel != self.frame_samples_per_channel
        {
            return Err(AudioInputError::IncompatibleContract);
        }
        let writer_id = NEXT_AUDIO_INPUT_WRITER_ID
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| AudioInputError::InstanceIdentityExhausted)?;
        let (sender, receiver) = SignalEdge::bounded(config.capacity_frames);
        let pool = AudioBufferPool::new(
            config.capacity_frames + 1,
            config.interleaved_samples_per_frame(),
        );
        let state = Arc::new(AudioInputState::default());
        let writer = AudioInputWriter {
            writer_id,
            config,
            pool,
            sender: Some(sender),
            state: Arc::clone(&state),
            next_sequence: 0,
            next_timestamp_ns: None,
            discontinuity_epoch: 0,
        };
        Ok(AudioInputReservation {
            pending: PendingAudioInput { receiver, state },
            writer,
        })
    }

    pub(crate) fn bind(
        &self,
        source_id: crate::SourceId,
        reservation: AudioInputReservation,
    ) -> AudioInputWriter {
        let mut pending = self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let replaced = pending.insert(source_id.get(), reservation.pending);
        debug_assert!(replaced.is_none());
        reservation.writer
    }

    pub(crate) fn cancel(&self, source_id: crate::SourceId) {
        self.pending
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&source_id.get());
    }
}

impl SourceDriver for AudioInputDriver {
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        let session = context.session.clone().ok_or_else(|| {
            SourceDriverError::Failed("audio input requires a Session identity".to_owned())
        })?;
        let pending = self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&session.source_id.get())
            .ok_or_else(|| {
                SourceDriverError::Failed(
                    "audio input reservation is absent or consumed".to_owned(),
                )
            })?;
        self.receiver = Some(pending.receiver);
        self.state = Some(pending.state);
        self.session = Some(session);
        Ok(())
    }

    fn next(
        &mut self,
        cancellation: &SourceCancellation,
    ) -> Result<Option<SourceEmission>, SourceDriverError> {
        if !self.receiver_thread_registered {
            self.receiver_thread_registered = self
                .receiver
                .as_ref()
                .ok_or_else(|| {
                    SourceDriverError::Failed("audio input receiver is not prepared".to_owned())
                })?
                .register_current_thread();
        }
        loop {
            if cancellation.is_cancelled() {
                self.state
                    .as_ref()
                    .ok_or_else(|| {
                        SourceDriverError::Failed("audio input state is not prepared".to_owned())
                    })?
                    .cancelled
                    .store(true, Ordering::Release);
                return Ok(None);
            }
            let receiver = self.receiver.as_mut().ok_or_else(|| {
                SourceDriverError::Failed("audio input receiver is not prepared".to_owned())
            })?;
            if let Some(queued) = receiver.recv() {
                return self.emission(queued).map(Some);
            }
            if receiver.is_abandoned() {
                if let Some(queued) = receiver.recv() {
                    return self.emission(queued).map(Some);
                }
                return Ok(None);
            }
            std::thread::park_timeout(DRIVER_IDLE_WAIT);
        }
    }

    fn close(&mut self) -> Result<(), SourceDriverError> {
        if let Some(state) = &self.state {
            state.closed.store(true, Ordering::Release);
        }
        Ok(())
    }
}

impl AudioInputDriver {
    fn emission(&self, queued: QueuedAudioInputFrame) -> Result<SourceEmission, SourceDriverError> {
        let session = self.session.as_ref().ok_or_else(|| {
            SourceDriverError::Failed("audio input Session context is absent".to_owned())
        })?;
        let output = session.output(OUTPUT_PORT).ok_or_else(|| {
            SourceDriverError::Failed("audio input output identity is absent".to_owned())
        })?;
        let frame = AudioFrame::try_new(
            output.stream_id,
            session.source_id,
            queued.sequence_number,
            queued.timestamp_ns,
            self.sample_spec,
            queued.buffer,
        )
        .map_err(|error| SourceDriverError::Failed(error.to_string()))?;
        let envelope = SignalEnvelope::from_audio(frame, None).with_lineage(
            SignalLineage {
                session_id: session.session_id,
                stream_id: output.stream_id,
                source_id: session.source_id,
                clock_id: PROCESS_MONOTONIC_CLOCK_DOMAIN_ID,
                sequence_number: queued.sequence_number,
                source_generation: 1,
                discontinuity_epoch: queued.discontinuity_epoch,
                policy_epoch: 0,
            },
            SignalTiming {
                source_timestamp_ns: Some(queued.timestamp_ns),
                observed_timestamp_ns: monotonic_timestamp_ns(),
                session_timestamp_ns: Some(queued.timestamp_ns),
                duration_ns: Some(queued.duration_ns),
            },
        );
        Ok(SourceEmission {
            output_port: OUTPUT_PORT.to_owned(),
            envelope,
            terminal: false,
        })
    }
}

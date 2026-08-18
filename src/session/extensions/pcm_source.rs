use std::collections::BTreeMap;
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::frame::{AudioBufferHandle, AudioBufferPool, AudioBufferWriteError, AudioFrame};
use crate::graph::{
    AudioCaps, ChannelLayout, ConfigError, ExecutionPartition, MediaCaps, Multiplicity,
    PortDirection, PortSpec, SafetyContract, SignalEnvelope, SignalLineage, SignalSpec,
    SignalTiming,
};
use crate::runtime::{SignalEdge, SignalEdgeReceiver, SignalEdgeSender};
use crate::session::declaration::{SourceInstanceHandle, SourceOutputHandle};
use crate::timing::{monotonic_timestamp_ns, PROCESS_MONOTONIC_CLOCK_DOMAIN_ID};
use crate::{SampleFormat, SampleSpec, SessionError};

use super::source::{
    SourceCancellation, SourceConfiguration, SourceDriver, SourceDriverError, SourceEmission,
    SourceFactory, SourceManifest, SourceManifestError, SourcePrepareContext, SourceSessionContext,
    SourceTypeId,
};

const OUTPUT_PORT: &str = "audio";
const MAX_CAPACITY_FRAMES: usize = 63;
const DRIVER_IDLE_WAIT: Duration = Duration::from_millis(2);

static NEXT_PCM_WRITER_ID: AtomicU64 = AtomicU64::new(1);

pub const PCM_SOURCE_TYPE_ID: &str = "io.pocketstation.source.pcm.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcmSourceConfig {
    sample_spec: SampleSpec,
    capacity_frames: usize,
    frame_samples_per_channel: usize,
}

impl PcmSourceConfig {
    pub fn new(
        sample_spec: SampleSpec,
        capacity_frames: usize,
        frame_samples_per_channel: usize,
    ) -> Result<Self, PcmSourceConfigError> {
        if sample_spec.sample_rate_hz == 0 {
            return Err(PcmSourceConfigError::ZeroSampleRate);
        }
        if !matches!(sample_spec.channels, 1 | 2) {
            return Err(PcmSourceConfigError::UnsupportedChannelCount);
        }
        if sample_spec.format != SampleFormat::F32Interleaved {
            return Err(PcmSourceConfigError::UnsupportedSampleFormat);
        }
        if !(1..=MAX_CAPACITY_FRAMES).contains(&capacity_frames) {
            return Err(PcmSourceConfigError::InvalidCapacity);
        }
        if frame_samples_per_channel == 0 {
            return Err(PcmSourceConfigError::ZeroFrameSamples);
        }
        frame_samples_per_channel
            .checked_mul(usize::from(sample_spec.channels))
            .ok_or(PcmSourceConfigError::FrameSampleCountOverflow)?;
        Ok(Self {
            sample_spec,
            capacity_frames,
            frame_samples_per_channel,
        })
    }

    pub const fn sample_spec(self) -> SampleSpec {
        self.sample_spec
    }

    pub const fn capacity_frames(self) -> usize {
        self.capacity_frames
    }

    pub const fn frame_samples_per_channel(self) -> usize {
        self.frame_samples_per_channel
    }

    fn interleaved_samples_per_frame(self) -> usize {
        self.frame_samples_per_channel * usize::from(self.sample_spec.channels)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PcmSourceConfigError {
    #[error("PCM source sample rate must be non-zero")]
    ZeroSampleRate,
    #[error("PCM source currently supports mono or stereo PCM")]
    UnsupportedChannelCount,
    #[error("PCM source currently supports F32 interleaved PCM")]
    UnsupportedSampleFormat,
    #[error("PCM source capacity must be between 1 and 63 frames")]
    InvalidCapacity,
    #[error("PCM source frame sample count must be non-zero")]
    ZeroFrameSamples,
    #[error("PCM source frame sample count overflows the platform size")]
    FrameSampleCountOverflow,
}

pub struct PcmSource {
    source: SourceInstanceHandle,
    output: SourceOutputHandle,
    writer: PcmSourceWriter,
}

impl PcmSource {
    pub(crate) fn new(
        source: SourceInstanceHandle,
        output: SourceOutputHandle,
        writer: PcmSourceWriter,
    ) -> Self {
        Self {
            source,
            output,
            writer,
        }
    }

    pub const fn source(&self) -> &SourceInstanceHandle {
        &self.source
    }

    pub const fn output(&self) -> &SourceOutputHandle {
        &self.output
    }

    pub const fn writer(&self) -> &PcmSourceWriter {
        &self.writer
    }

    pub fn writer_mut(&mut self) -> &mut PcmSourceWriter {
        &mut self.writer
    }

    pub fn into_parts(self) -> (SourceInstanceHandle, SourceOutputHandle, PcmSourceWriter) {
        (self.source, self.output, self.writer)
    }
}

impl fmt::Debug for PcmSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PcmSource")
            .field("source", &self.source)
            .field("output", &self.output)
            .field("writer", &self.writer)
            .finish()
    }
}

pub struct PcmBuffer {
    writer_id: u64,
    buffer: AudioBufferHandle,
    sample_capacity: usize,
    discontinuity: bool,
}

impl PcmBuffer {
    pub fn sample_capacity(&self) -> usize {
        self.sample_capacity
    }

    pub fn sample_count(&self) -> usize {
        self.buffer.len()
    }

    pub fn try_set_sample_count(
        &mut self,
        sample_count: usize,
    ) -> Result<(), AudioBufferWriteError> {
        self.buffer.try_set_len(sample_count)
    }

    pub fn try_copy_from_slice(&mut self, samples: &[f32]) -> Result<(), AudioBufferWriteError> {
        self.buffer.try_copy_from_slice(samples)
    }

    pub fn samples(&self) -> &[f32] {
        self.buffer.as_slice()
    }

    pub fn samples_mut(&mut self) -> &mut [f32] {
        self.buffer.as_mut_slice()
    }

    pub fn mark_discontinuity(&mut self) {
        self.discontinuity = true;
    }
}

impl fmt::Debug for PcmBuffer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PcmBuffer")
            .field("sample_count", &self.sample_count())
            .field("sample_capacity", &self.sample_capacity())
            .field("discontinuity", &self.discontinuity)
            .finish()
    }
}

#[derive(Default)]
struct PcmSourceState {
    cancelled: AtomicBool,
    closed: AtomicBool,
    accepted_total: AtomicU64,
    full_total: AtomicU64,
    invalid_total: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcmSourceObservations {
    pub capacity_frames: u64,
    pub buffer_slots: u64,
    pub available_buffers: u64,
    pub accepted_total: u64,
    pub full_total: u64,
    pub invalid_total: u64,
    pub cancelled: bool,
    pub closed: bool,
}

pub struct PcmSourceWriter {
    writer_id: u64,
    config: PcmSourceConfig,
    pool: Arc<AudioBufferPool>,
    sender: Option<SignalEdgeSender<QueuedPcmFrame>>,
    state: Arc<PcmSourceState>,
    next_sequence: u64,
    next_timestamp_ns: Option<u64>,
    discontinuity_epoch: u64,
}

impl PcmSourceWriter {
    pub fn try_acquire(&self) -> Result<PcmBuffer, PcmBufferAcquireError> {
        if self.state.cancelled.load(Ordering::Acquire) {
            return Err(PcmBufferAcquireError::Cancelled);
        }
        if self.sender.is_none()
            || self.state.closed.load(Ordering::Acquire)
            || self
                .sender
                .as_ref()
                .is_some_and(|sender| sender.is_abandoned())
        {
            return Err(PcmBufferAcquireError::Closed);
        }
        let mut buffer = self.pool.acquire().ok_or(PcmBufferAcquireError::Full)?;
        buffer
            .try_set_len(0)
            .map_err(|_| PcmBufferAcquireError::Full)?;
        Ok(PcmBuffer {
            writer_id: self.writer_id,
            buffer,
            sample_capacity: self.config.interleaved_samples_per_frame(),
            discontinuity: false,
        })
    }

    pub fn try_write(&mut self, samples: &[f32]) -> Result<(), PcmWriteError> {
        let mut buffer = self.try_acquire().map_err(PcmWriteError::from)?;
        if let Err(error) = buffer.try_copy_from_slice(samples) {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(PcmWriteError::new(
                PcmWriteErrorKind::InvalidBuffer(PcmBufferError::Capacity(error)),
                Some(buffer),
            ));
        }
        self.try_send(buffer)
    }

    pub fn try_send(&mut self, buffer: PcmBuffer) -> Result<(), PcmWriteError> {
        if self.state.cancelled.load(Ordering::Acquire) {
            return Err(PcmWriteError::new(
                PcmWriteErrorKind::Cancelled,
                Some(buffer),
            ));
        }
        let Some(sender) = self.sender.as_mut() else {
            return Err(PcmWriteError::new(PcmWriteErrorKind::Closed, Some(buffer)));
        };
        if self.state.closed.load(Ordering::Acquire) || sender.is_abandoned() {
            return Err(PcmWriteError::new(PcmWriteErrorKind::Closed, Some(buffer)));
        }
        if buffer.writer_id != self.writer_id {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(PcmWriteError::new(
                PcmWriteErrorKind::InvalidBuffer(PcmBufferError::WrongSource),
                Some(buffer),
            ));
        }
        let sample_count = buffer.buffer.len();
        let channels = usize::from(self.config.sample_spec.channels);
        if sample_count == 0 {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(PcmWriteError::new(
                PcmWriteErrorKind::InvalidBuffer(PcmBufferError::Empty),
                Some(buffer),
            ));
        }
        if !sample_count.is_multiple_of(channels) {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(PcmWriteError::new(
                PcmWriteErrorKind::InvalidBuffer(PcmBufferError::MisalignedChannels),
                Some(buffer),
            ));
        }
        let expected_samples = self.config.interleaved_samples_per_frame();
        if sample_count != expected_samples {
            self.state.invalid_total.fetch_add(1, Ordering::Relaxed);
            return Err(PcmWriteError::new(
                PcmWriteErrorKind::InvalidBuffer(PcmBufferError::WrongFrameLength {
                    expected_samples,
                    actual_samples: sample_count,
                }),
                Some(buffer),
            ));
        }

        let sample_frames = sample_count / channels;
        let duration_ns = sample_duration_ns(sample_frames, self.config.sample_spec.sample_rate_hz);
        let timestamp_ns = self
            .next_timestamp_ns
            .unwrap_or_else(monotonic_timestamp_ns);
        let next_discontinuity_epoch = if buffer.discontinuity {
            self.discontinuity_epoch.saturating_add(1)
        } else {
            self.discontinuity_epoch
        };
        let queued = QueuedPcmFrame {
            buffer: buffer.buffer,
            sequence_number: self.next_sequence,
            timestamp_ns,
            duration_ns,
            discontinuity_epoch: next_discontinuity_epoch,
        };
        match sender.try_send(queued) {
            Ok(()) => {
                self.next_sequence = self.next_sequence.saturating_add(1);
                self.next_timestamp_ns = Some(timestamp_ns.saturating_add(duration_ns));
                self.discontinuity_epoch = next_discontinuity_epoch;
                self.state.accepted_total.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(error) => {
                self.state.full_total.fetch_add(1, Ordering::Relaxed);
                let queued = error.into_rejected();
                Err(PcmWriteError::new(
                    PcmWriteErrorKind::Full,
                    Some(PcmBuffer {
                        writer_id: self.writer_id,
                        buffer: queued.buffer,
                        sample_capacity: self.config.interleaved_samples_per_frame(),
                        discontinuity: queued.discontinuity_epoch > self.discontinuity_epoch,
                    }),
                ))
            }
        }
    }

    pub fn close(&mut self) {
        self.sender = None;
    }

    pub fn observations(&self) -> PcmSourceObservations {
        PcmSourceObservations {
            capacity_frames: self.config.capacity_frames as u64,
            buffer_slots: self.config.capacity_frames.saturating_add(1) as u64,
            available_buffers: self.pool.available_slots() as u64,
            accepted_total: self.state.accepted_total.load(Ordering::Relaxed),
            full_total: self.state.full_total.load(Ordering::Relaxed),
            invalid_total: self.state.invalid_total.load(Ordering::Relaxed),
            cancelled: self.state.cancelled.load(Ordering::Acquire),
            closed: self.sender.is_none() || self.state.closed.load(Ordering::Acquire),
        }
    }
}

impl fmt::Debug for PcmSourceWriter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PcmSourceWriter")
            .field("config", &self.config)
            .field("observations", &self.observations())
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PcmBufferAcquireError {
    #[error("PCM source is full")]
    Full,
    #[error("PCM source is closed")]
    Closed,
    #[error("PCM source Session was cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PcmBufferError {
    #[error("audio buffer belongs to another PCM source")]
    WrongSource,
    #[error("audio buffer contains no samples")]
    Empty,
    #[error("interleaved sample count is not divisible by the channel count")]
    MisalignedChannels,
    #[error("audio buffer contains {actual_samples} samples; expected exactly {expected_samples}")]
    WrongFrameLength {
        expected_samples: usize,
        actual_samples: usize,
    },
    #[error("audio buffer capacity rejected the samples: {0}")]
    Capacity(AudioBufferWriteError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcmWriteErrorKind {
    Full,
    Closed,
    Cancelled,
    InvalidBuffer(PcmBufferError),
}

pub struct PcmWriteError {
    kind: PcmWriteErrorKind,
    rejected: Option<PcmBuffer>,
}

impl PcmWriteError {
    fn new(kind: PcmWriteErrorKind, rejected: Option<PcmBuffer>) -> Self {
        Self { kind, rejected }
    }

    pub const fn kind(&self) -> PcmWriteErrorKind {
        self.kind
    }

    pub fn into_rejected(self) -> Option<PcmBuffer> {
        self.rejected
    }
}

impl From<PcmBufferAcquireError> for PcmWriteError {
    fn from(error: PcmBufferAcquireError) -> Self {
        let kind = match error {
            PcmBufferAcquireError::Full => PcmWriteErrorKind::Full,
            PcmBufferAcquireError::Closed => PcmWriteErrorKind::Closed,
            PcmBufferAcquireError::Cancelled => PcmWriteErrorKind::Cancelled,
        };
        Self::new(kind, None)
    }
}

impl fmt::Debug for PcmWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PcmWriteError")
            .field("kind", &self.kind)
            .field("has_rejected_buffer", &self.rejected.is_some())
            .finish()
    }
}

impl fmt::Display for PcmWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            PcmWriteErrorKind::Full => formatter.write_str("PCM source is full"),
            PcmWriteErrorKind::Closed => formatter.write_str("PCM source is closed"),
            PcmWriteErrorKind::Cancelled => formatter.write_str("PCM source Session was cancelled"),
            PcmWriteErrorKind::InvalidBuffer(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for PcmWriteError {}

#[derive(Debug, thiserror::Error)]
pub enum PcmSourceError {
    #[error("invalid PCM source configuration: {0}")]
    Configuration(#[from] PcmSourceConfigError),
    #[error("PCM source manifest failed: {0}")]
    Manifest(#[from] SourceManifestError),
    #[error("PCM source Session declaration failed: {0}")]
    Session(#[from] SessionError),
    #[error("PCM source registration state is unavailable")]
    RegistrationStateUnavailable,
    #[error("all PCM sources in one Session must use the same concrete sample and frame contract")]
    IncompatibleContract,
    #[error("PCM source instance identity space is exhausted")]
    InstanceIdentityExhausted,
}

struct QueuedPcmFrame {
    buffer: AudioBufferHandle,
    sequence_number: u64,
    timestamp_ns: u64,
    duration_ns: u64,
    discontinuity_epoch: u64,
}

struct PendingPcmSource {
    receiver: SignalEdgeReceiver<QueuedPcmFrame>,
    state: Arc<PcmSourceState>,
}

pub(crate) struct PcmSourceReservation {
    pending: PendingPcmSource,
    writer: PcmSourceWriter,
}

pub(crate) struct PcmSourceFactory {
    manifest: SourceManifest,
    sample_spec: SampleSpec,
    frame_samples_per_channel: usize,
    pending: Arc<Mutex<BTreeMap<u64, PendingPcmSource>>>,
}

struct PcmSourceDriver {
    pending: Arc<Mutex<BTreeMap<u64, PendingPcmSource>>>,
    receiver: Option<SignalEdgeReceiver<QueuedPcmFrame>>,
    state: Option<Arc<PcmSourceState>>,
    sample_spec: SampleSpec,
    session: Option<SourceSessionContext>,
    receiver_thread_registered: bool,
}

impl SourceFactory for PcmSourceFactory {
    fn manifest(&self) -> &SourceManifest {
        &self.manifest
    }

    fn validate_config(&self, configuration: &SourceConfiguration) -> Result<(), ConfigError> {
        if configuration.iter().next().is_some() {
            Err(ConfigError::Invalid {
                key: "<configuration>".to_owned(),
                reason: "PCM source does not accept public configuration fields".to_owned(),
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
        Ok(Box::new(PcmSourceDriver {
            pending: Arc::clone(&self.pending),
            receiver: None,
            state: None,
            sample_spec: self.sample_spec,
            session: None,
            receiver_thread_registered: false,
        }))
    }
}

impl PcmSourceFactory {
    pub(crate) fn new(config: PcmSourceConfig) -> Result<Arc<Self>, PcmSourceError> {
        let channel_layout = match config.sample_spec.channels {
            1 => ChannelLayout::Mono,
            2 => ChannelLayout::Stereo,
            _ => return Err(PcmSourceConfigError::UnsupportedChannelCount.into()),
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
        config: PcmSourceConfig,
    ) -> Result<PcmSourceReservation, PcmSourceError> {
        if config.sample_spec != self.sample_spec
            || config.frame_samples_per_channel != self.frame_samples_per_channel
        {
            return Err(PcmSourceError::IncompatibleContract);
        }
        let writer_id = NEXT_PCM_WRITER_ID
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| PcmSourceError::InstanceIdentityExhausted)?;
        let (sender, receiver) = SignalEdge::bounded(config.capacity_frames);
        let pool = AudioBufferPool::new(
            config.capacity_frames + 1,
            config.interleaved_samples_per_frame(),
        );
        let state = Arc::new(PcmSourceState::default());
        let writer = PcmSourceWriter {
            writer_id,
            config,
            pool,
            sender: Some(sender),
            state: Arc::clone(&state),
            next_sequence: 0,
            next_timestamp_ns: None,
            discontinuity_epoch: 0,
        };
        Ok(PcmSourceReservation {
            pending: PendingPcmSource { receiver, state },
            writer,
        })
    }

    pub(crate) fn bind(
        &self,
        source_id: crate::SourceId,
        reservation: PcmSourceReservation,
    ) -> PcmSourceWriter {
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

impl SourceDriver for PcmSourceDriver {
    fn prepare(&mut self, context: &SourcePrepareContext) -> Result<(), SourceDriverError> {
        let session = context.session.clone().ok_or_else(|| {
            SourceDriverError::Failed("PCM source requires a Session identity".to_owned())
        })?;
        let pending = self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .remove(&session.source_id.get())
            .ok_or_else(|| {
                SourceDriverError::Failed("PCM source reservation is absent or consumed".to_owned())
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
                    SourceDriverError::Failed("PCM source receiver is not prepared".to_owned())
                })?
                .register_current_thread();
        }
        loop {
            if cancellation.is_cancelled() {
                self.state
                    .as_ref()
                    .ok_or_else(|| {
                        SourceDriverError::Failed("PCM source state is not prepared".to_owned())
                    })?
                    .cancelled
                    .store(true, Ordering::Release);
                return Ok(None);
            }
            let receiver = self.receiver.as_mut().ok_or_else(|| {
                SourceDriverError::Failed("PCM source receiver is not prepared".to_owned())
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

impl PcmSourceDriver {
    fn emission(&self, queued: QueuedPcmFrame) -> Result<SourceEmission, SourceDriverError> {
        let session = self.session.as_ref().ok_or_else(|| {
            SourceDriverError::Failed("PCM source Session context is absent".to_owned())
        })?;
        let output = session.output(OUTPUT_PORT).ok_or_else(|| {
            SourceDriverError::Failed("PCM source output identity is absent".to_owned())
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

fn sample_duration_ns(sample_frames: usize, sample_rate_hz: u32) -> u64 {
    (sample_frames as u128)
        .saturating_mul(1_000_000_000)
        .checked_div(u128::from(sample_rate_hz))
        .unwrap_or(0)
        .min(u128::from(u64::MAX)) as u64
}

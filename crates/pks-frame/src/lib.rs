use std::cell::UnsafeCell;
use std::fmt;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

pub const SAMPLE_RATE_HZ: u32 = 48_000;
pub const FRAME_DURATION_MS: u32 = 20;
pub const POOL_SLOT_SAMPLES: usize = 960; // 20ms × 48kHz
pub const POOL_MAX_SLOTS: usize = 64; // AtomicU64 bitset ceiling

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Macos,
    Windows,
    Linux,
    Ios,
    Android,
    Web,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StemId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndpointId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectorId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteId(pub u64);

/// Identifies a named audio bus within a Session. None = pre-graph capture path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BusId(pub u64);

/// Opaque handles for Phase 3+ fields on SourceIdentity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelProviderId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClockDomainId(pub u32);

/// Compact hot-path lineage carried by a captured or derived frame.
///
/// Descriptive source, stem, and permission data live in immutable registries;
/// frames carry only stable numeric references and monotonic time values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameLineage {
    pub session_id: SessionId,
    pub source_id: SourceId,
    pub stem_id: StemId,
    pub clock_id: ClockDomainId,
    pub sequence_num: u64,
    pub timestamp_start_ns: u64,
    pub duration_ns: u64,
    pub source_generation: u32,
    pub discontinuity_epoch: u64,
    pub permission_epoch: u64,
}

impl FrameLineage {
    pub fn timestamp_end_ns(self) -> u64 {
        self.timestamp_start_ns.saturating_add(self.duration_ns)
    }
}

/// Route-specific delivery lineage. One `FrameLineage` may produce several
/// records when a stem fans out to independent destinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeliveryLineage {
    pub endpoint_id: EndpointId,
    pub connector_id: Option<ConnectorId>,
    pub route_id: RouteId,
    pub enqueued_at_ns: u64,
    pub delivered_at_ns: Option<u64>,
    pub delivery_discontinuity_epoch: u64,
}

impl DeliveryLineage {
    pub fn queue_latency_ns(self) -> Option<u64> {
        self.delivered_at_ns
            .map(|delivered_at_ns| delivered_at_ns.saturating_sub(self.enqueued_at_ns))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyClass {
    Public,
    Private,      // not forwarded to model nodes
    Confidential, // E2EE required end-to-end (Phase 5)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleFormat {
    F32Interleaved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleSpec {
    pub sample_rate_hz: u32,
    pub channels: u8,
    pub format: SampleFormat,
}

impl SampleSpec {
    pub fn new(sample_rate_hz: u32, channels: u8, format: SampleFormat) -> Self {
        Self {
            sample_rate_hz,
            channels,
            format,
        }
    }

    pub fn frame_samples_for_duration_ms(&self, duration_ms: u32) -> usize {
        (self.sample_rate_hz * duration_ms / 1000) as usize * self.channels as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioMode {
    Voice,
    Music,
    Broadcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioSourceTag {
    #[default]
    Captured,
    AiTts, // EU AI Act §50: watermark required before relay delivery (AUDIO-017)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncryptionMode {
    #[default]
    None,
    SFrame, // RFC 9605 frame-level E2EE; relay forwards opaque, SDK decrypts
}

pub struct AudioBufferPool {
    slots: Box<[UnsafeCell<Box<[f32]>>]>,
    shared_ref_counts: Box<[AtomicUsize]>,
    slot_size: usize,     // samples per slot, fixed at creation
    free_mask: AtomicU64, // bitset: 1 = free; 64-slot cap
    acquire_failures: AtomicUsize,
}

// SAFETY: Each slot is guarded by its free_mask bit and shared reference count.
// An exclusive handle exists only while the free bit is clear and the shared
// count is zero. A frozen slot has a non-zero shared count and exposes immutable
// access only. The final shared Drop returns the free bit.
unsafe impl Sync for AudioBufferPool {}

impl AudioBufferPool {
    pub fn new(slot_count: usize, slot_size: usize) -> Arc<Self> {
        assert!((1..=POOL_MAX_SLOTS).contains(&slot_count));
        assert!(slot_size > 0);
        let slots: Vec<_> = (0..slot_count)
            .map(|_| UnsafeCell::new(vec![0.0f32; slot_size].into_boxed_slice()))
            .collect();
        let full_mask = if slot_count == 64 {
            u64::MAX
        } else {
            (1u64 << slot_count) - 1
        };
        Arc::new(Self {
            slots: slots.into_boxed_slice(),
            shared_ref_counts: (0..slot_count)
                .map(|_| AtomicUsize::new(0))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            slot_size,
            free_mask: AtomicU64::new(full_mask),
            acquire_failures: AtomicUsize::new(0),
        })
    }

    pub fn slot_size(&self) -> usize {
        self.slot_size
    }
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }
    pub fn acquire_failures(&self) -> usize {
        self.acquire_failures.load(Ordering::Relaxed)
    }
    pub fn available_slots(&self) -> usize {
        self.free_mask.load(Ordering::Acquire).count_ones() as usize
    }

    pub fn acquire(self: &Arc<Self>) -> Option<AudioBufferHandle> {
        loop {
            let mask = self.free_mask.load(Ordering::Acquire);
            if mask == 0 {
                self.acquire_failures.fetch_add(1, Ordering::Relaxed);
                return None;
            }
            let idx = mask.trailing_zeros() as usize;
            let bit = 1u64 << idx;
            if self
                .free_mask
                .compare_exchange_weak(mask, mask & !bit, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return Some(AudioBufferHandle {
                    pool: Arc::clone(self),
                    index: idx as u32,
                    len: self.slot_size as u32,
                });
            }
        }
    }

    pub fn is_in_use(&self, index: u32) -> bool {
        self.free_mask.load(Ordering::Acquire) & (1u64 << index) == 0
    }

    pub fn shared_ref_count(&self, index: u32) -> usize {
        self.shared_ref_counts
            .get(index as usize)
            .map_or(0, |count| count.load(Ordering::Acquire))
    }

    fn begin_shared(&self, index: u32) -> bool {
        self.shared_ref_counts
            .get(index as usize)
            .is_some_and(|count| {
                count
                    .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
            })
    }

    fn try_retain_shared(&self, index: u32) -> bool {
        let Some(count) = self.shared_ref_counts.get(index as usize) else {
            return false;
        };
        let mut current = count.load(Ordering::Acquire);
        loop {
            if current == 0 || current == usize::MAX {
                return false;
            }
            match count.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(observed) => current = observed,
            }
        }
    }

    fn release_shared(&self, index: u32) -> bool {
        let Some(count) = self.shared_ref_counts.get(index as usize) else {
            return false;
        };
        let mut current = count.load(Ordering::Acquire);
        loop {
            if current == 0 {
                return false;
            }
            match count.compare_exchange_weak(
                current,
                current - 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    if current == 1 {
                        self.release(index);
                    }
                    return true;
                }
                Err(observed) => current = observed,
            }
        }
    }

    fn release(&self, index: u32) {
        self.free_mask.fetch_or(1u64 << index, Ordering::Release);
    }

    fn slot(&self, index: u32, len: u32) -> &[f32] {
        assert!((index as usize) < self.slots.len() && (len as usize) <= self.slot_size);
        // SAFETY: immutable borrow; exclusive mutable access is held by the owning handle.
        let cell = unsafe { &*self.slots[index as usize].get() };
        &cell[..len as usize]
    }

    #[allow(clippy::mut_from_ref)]
    fn slot_mut(&self, index: u32, len: u32) -> &mut [f32] {
        assert!((index as usize) < self.slots.len() && (len as usize) <= self.slot_size);
        // SAFETY: acquisition protocol ensures exactly one live handle per slot.
        let cell = unsafe { &mut *self.slots[index as usize].get() };
        &mut cell[..len as usize]
    }
}

pub struct AudioBufferHandle {
    pool: Arc<AudioBufferPool>,
    index: u32,
    len: u32,
}

impl AudioBufferHandle {
    pub fn len(&self) -> usize {
        self.len as usize
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn index(&self) -> u32 {
        self.index
    }
    pub fn as_slice(&self) -> &[f32] {
        self.pool.slot(self.index, self.len)
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        self.pool.slot_mut(self.index, self.len)
    }

    pub fn set_len(&mut self, len: usize) {
        assert!(len <= self.pool.slot_size());
        self.len = len as u32;
    }

    pub fn copy_from_slice(&mut self, data: &[f32]) {
        assert!(data.len() <= self.pool.slot_size());
        self.set_len(data.len());
        self.as_mut_slice().copy_from_slice(data);
    }

    pub fn freeze(self) -> Result<SharedAudioBufferHandle, Self> {
        if !self.pool.begin_shared(self.index) {
            return Err(self);
        }
        let this = std::mem::ManuallyDrop::new(self);
        // SAFETY: `this` will not be dropped. Moving its Arc into the immutable
        // handle transfers the sole exclusive ownership without changing the
        // pool's allocation or releasing the slot.
        let pool = unsafe { std::ptr::read(&this.pool) };
        Ok(SharedAudioBufferHandle {
            pool,
            index: this.index,
            len: this.len,
        })
    }
}

/// Drop contract — must stay forever: lock-free · panic-free · alloc-free · log-free.
impl Drop for AudioBufferHandle {
    fn drop(&mut self) {
        if self.pool.shared_ref_count(self.index) == 0 {
            self.pool.release(self.index);
        }
    }
}

impl fmt::Debug for AudioBufferHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioBufferHandle")
            .field("index", &self.index)
            .field("len", &self.len)
            .finish()
    }
}

pub struct SharedAudioBufferHandle {
    pool: Arc<AudioBufferPool>,
    index: u32,
    len: u32,
}

impl SharedAudioBufferHandle {
    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn as_slice(&self) -> &[f32] {
        self.pool.slot(self.index, self.len)
    }

    pub fn try_clone(&self) -> Option<Self> {
        if !self.pool.try_retain_shared(self.index) {
            return None;
        }
        Some(Self {
            pool: Arc::clone(&self.pool),
            index: self.index,
            len: self.len,
        })
    }

    pub fn shared_ref_count(&self) -> usize {
        self.pool.shared_ref_count(self.index)
    }
}

/// Drop contract — must stay forever: lock-free · panic-free · alloc-free · log-free.
impl Drop for SharedAudioBufferHandle {
    fn drop(&mut self) {
        self.pool.release_shared(self.index);
    }
}

impl fmt::Debug for SharedAudioBufferHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedAudioBufferHandle")
            .field("index", &self.index)
            .field("len", &self.len)
            .field("shared_ref_count", &self.shared_ref_count())
            .finish()
    }
}

/// Semantic identity of an audio source node in the graph (v3.0 addition — Phase 0).
#[derive(Debug, Clone)]
pub struct SourceIdentity {
    pub source_id: SourceId,
    pub display_name: String,
    pub kind: SourceKind,
    pub platform: Platform,
    pub app_bundle_id: Option<String>,
    pub device_id: Option<String>,
    pub human_owner: Option<UserId>,          // Phase 3
    pub agent_owner: Option<AgentId>,         // Phase 3
    pub model_owner: Option<ModelProviderId>, // Phase 3
    pub clock_domain: ClockDomainId,
    pub privacy_class: PrivacyClass,
}

/// Identifies an audio source kind for graph node discrimination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Mic,
    SystemOutput,
    Application,
    Device,
    NetworkStream,
    VirtualInput,
    ModelOutput, // TTS or agent output fed back into the graph
    Synthetic,   // testing / Phase 0 sine sources
}

/// Describes a named audio bus in a Session (v3.0 addition -- Phase 0).
#[derive(Debug, Clone)]
pub struct BusDescriptor {
    pub bus_id: BusId,
    pub name: String,
    pub mode: AudioMode,     // Voice | Music | Broadcast
    pub channels: u8,        // 1 or 2
    pub sample_rate_hz: u32, // always 48_000 in Phase 0–1
}

#[derive(Debug)]
pub struct AudioFrame {
    pub stream_id: StreamId,
    pub source_id: SourceId,
    pub bus_id: Option<BusId>, // semantic bus identity (v3.0)
    pub sample_rate_hz: u32,   // always 48_000 internally; see DOCS-013
    pub channels: u8,          // 1 = voice, 2 = music/broadcast
    pub format: SampleFormat,
    pub timestamp_ns: u64, // monotonic, never wall clock
    pub sequence_number: u64,
    pub buffer: AudioBufferHandle,
    pub source_tag: AudioSourceTag, // AiTts triggers AUDIO-017 watermark
    pub speaker_id: Option<u32>,    // assigned by diarization node (Phase 6)
    pub encryption_mode: EncryptionMode,
}

impl AudioFrame {
    pub fn new(
        stream_id: StreamId,
        source_id: SourceId,
        sequence_number: u64,
        timestamp_ns: u64,
        channels: u8,
        buffer: AudioBufferHandle,
    ) -> Self {
        Self {
            stream_id,
            source_id,
            bus_id: None,
            sample_rate_hz: SAMPLE_RATE_HZ,
            channels,
            format: SampleFormat::F32Interleaved,
            timestamp_ns,
            sequence_number,
            buffer,
            source_tag: AudioSourceTag::Captured,
            speaker_id: None,
            encryption_mode: EncryptionMode::None,
        }
    }

    pub fn freeze(self) -> Option<SharedAudioFrame> {
        let Self {
            stream_id,
            source_id,
            bus_id,
            sample_rate_hz,
            channels,
            format,
            timestamp_ns,
            sequence_number,
            buffer,
            source_tag,
            speaker_id,
            encryption_mode,
        } = self;
        let buffer = buffer.freeze().ok()?;
        Some(SharedAudioFrame {
            stream_id,
            source_id,
            bus_id,
            sample_rate_hz,
            channels,
            format,
            timestamp_ns,
            sequence_number,
            buffer,
            source_tag,
            speaker_id,
            encryption_mode,
        })
    }
}

#[derive(Debug)]
pub struct SharedAudioFrame {
    pub stream_id: StreamId,
    pub source_id: SourceId,
    pub bus_id: Option<BusId>,
    pub sample_rate_hz: u32,
    pub channels: u8,
    pub format: SampleFormat,
    pub timestamp_ns: u64,
    pub sequence_number: u64,
    pub buffer: SharedAudioBufferHandle,
    pub source_tag: AudioSourceTag,
    pub speaker_id: Option<u32>,
    pub encryption_mode: EncryptionMode,
}

impl SharedAudioFrame {
    pub fn try_clone(&self) -> Option<Self> {
        Some(Self {
            stream_id: self.stream_id,
            source_id: self.source_id,
            bus_id: self.bus_id,
            sample_rate_hz: self.sample_rate_hz,
            channels: self.channels,
            format: self.format,
            timestamp_ns: self.timestamp_ns,
            sequence_number: self.sequence_number,
            buffer: self.buffer.try_clone()?,
            source_tag: self.source_tag,
            speaker_id: self.speaker_id,
            encryption_mode: self.encryption_mode,
        })
    }

    pub fn copy_to_pool(&self, pool: &Arc<AudioBufferPool>) -> Option<AudioFrame> {
        let mut buffer = pool.acquire()?;
        buffer.copy_from_slice(self.buffer.as_slice());
        Some(AudioFrame {
            stream_id: self.stream_id,
            source_id: self.source_id,
            bus_id: self.bus_id,
            sample_rate_hz: self.sample_rate_hz,
            channels: self.channels,
            format: self.format,
            timestamp_ns: self.timestamp_ns,
            sequence_number: self.sequence_number,
            buffer,
            source_tag: self.source_tag,
            speaker_id: self.speaker_id,
            encryption_mode: self.encryption_mode,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodedCodec {
    Opus,    // RFC 6716 single Opus payload
    OpusRed, // RFC 2198 redundancy wrapping Opus; loss-resilient (ADR-021)
}

impl EncodedCodec {
    pub fn is_redundant(self) -> bool {
        matches!(self, Self::OpusRed)
    }
}

/// Encoder output; produced on the encoder/processing thread, never the audio callback,
/// so the owned payload_bytes Vec does not violate the hot-path purity rule (LAW 15).
#[derive(Debug, Clone)]
pub struct EncodedFrame {
    pub stream_id: StreamId,
    pub source_id: SourceId,
    pub bus_id: Option<BusId>,
    pub codec: EncodedCodec,
    pub sample_rate_hz: u32,
    pub channels: u8,
    pub timestamp_ns: u64,
    pub sequence_number: u64,
    pub payload_bytes: Vec<u8>,
    pub source_tag: AudioSourceTag,
    pub encryption_mode: EncryptionMode,
}

impl EncodedFrame {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        stream_id: StreamId,
        source_id: SourceId,
        bus_id: Option<BusId>,
        codec: EncodedCodec,
        sample_rate_hz: u32,
        channels: u8,
        timestamp_ns: u64,
        sequence_number: u64,
        payload_bytes: Vec<u8>,
        source_tag: AudioSourceTag,
        encryption_mode: EncryptionMode,
    ) -> Self {
        Self {
            stream_id,
            source_id,
            bus_id,
            codec,
            sample_rate_hz,
            channels,
            timestamp_ns,
            sequence_number,
            payload_bytes,
            source_tag,
            encryption_mode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventPayload {
    VoiceActivity(bool),
    Transcript { text: String, is_final: bool },
    Metadata { key: String, value: String },
}

/// Control/event-plane frame; produced off the audio callback thread, so the owned
/// String payloads in EventPayload are acceptable here (LAW 15 hot-path purity).
#[derive(Debug, Clone)]
pub struct EventFrame {
    pub stream_id: StreamId,
    pub source_id: SourceId,
    pub bus_id: Option<BusId>,
    pub timestamp_ns: u64,
    pub sequence_number: u64,
    pub payload: EventPayload,
}

impl EventFrame {
    pub fn new(
        stream_id: StreamId,
        source_id: SourceId,
        bus_id: Option<BusId>,
        timestamp_ns: u64,
        sequence_number: u64,
        payload: EventPayload,
    ) -> Self {
        Self {
            stream_id,
            source_id,
            bus_id,
            timestamp_ns,
            sequence_number,
            payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_pool_with_4_slots_when_all_acquired_then_next_acquire_returns_none() {
        // Given
        let pool = AudioBufferPool::new(4, 16);

        // When
        let a = pool.acquire().unwrap();
        let b = pool.acquire().unwrap();
        let c = pool.acquire().unwrap();
        let d = pool.acquire().unwrap();

        // Then
        assert!(pool.acquire().is_none());
        drop(a);
        drop(b);
        drop(c);
        drop(d);
        assert!(pool.acquire().is_some());
    }

    #[test]
    fn given_pool_acquisition_and_release_when_observed_then_available_slots_are_exact() {
        let pool = AudioBufferPool::new(2, 4);
        assert_eq!(pool.available_slots(), 2);

        let handle = pool.acquire().unwrap();
        assert_eq!(pool.available_slots(), 1);

        drop(handle);
        assert_eq!(pool.available_slots(), 2);
    }

    #[test]
    fn given_acquired_handle_when_copy_from_slice_then_length_matches_data() {
        // Given
        let pool = AudioBufferPool::new(1, 8);
        let mut h = pool.acquire().unwrap();

        // When
        h.copy_from_slice(&[1.0, 2.0, 3.0]);

        // Then
        assert_eq!(h.len(), 3);
        assert_eq!(h.as_slice(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn given_full_64_slot_pool_when_acquire_then_returns_none_and_increments_failures() {
        // Given
        let pool = AudioBufferPool::new(64, 4);

        // When
        let handles: Vec<_> = (0..64).map(|_| pool.acquire().unwrap()).collect();
        let extra = pool.acquire();

        // Then
        assert_eq!(pool.acquire_failures(), 1);
        assert!(extra.is_none());
        drop(handles);
    }

    #[test]
    fn given_exhausted_pool_when_handle_dropped_then_reacquire_succeeds() {
        // Given
        let pool = AudioBufferPool::new(1, 4);
        let h = pool.acquire().unwrap();
        assert!(pool.acquire().is_none());

        // When
        drop(h);

        // Then
        assert!(pool.acquire().is_some());
    }

    #[test]
    fn given_pool_when_acquire_and_release_then_in_use_flag_tracks_state() {
        // Given
        let pool = AudioBufferPool::new(2, 4);
        let h = pool.acquire().unwrap();
        let slot = h.index();

        // When / Then
        assert!(pool.is_in_use(slot));
        drop(h);
        assert!(!pool.is_in_use(slot));
    }

    #[test]
    fn given_48khz_mono_spec_when_frame_samples_for_20ms_then_returns_960() {
        // Given
        let spec = SampleSpec::new(SAMPLE_RATE_HZ, 1, SampleFormat::F32Interleaved);

        // When
        let samples = spec.frame_samples_for_duration_ms(FRAME_DURATION_MS);

        // Then
        assert_eq!(samples, 960);
    }

    #[test]
    fn given_48khz_stereo_spec_when_frame_samples_for_20ms_then_returns_1920() {
        // Given
        let spec = SampleSpec::new(SAMPLE_RATE_HZ, 2, SampleFormat::F32Interleaved);

        // When
        let samples = spec.frame_samples_for_duration_ms(FRAME_DURATION_MS);

        // Then
        assert_eq!(samples, 1920);
    }

    #[test]
    fn given_encoded_codec_variants_when_is_redundant_then_only_opus_red_is_true() {
        // Given / When / Then
        assert!(!EncodedCodec::Opus.is_redundant());
        assert!(EncodedCodec::OpusRed.is_redundant());
    }

    #[test]
    fn given_encoded_frame_fields_when_new_then_fields_round_trip() {
        // Given
        let payload_bytes = vec![1u8, 2, 3, 4];

        // When
        let frame = EncodedFrame::new(
            StreamId(7),
            SourceId(11),
            Some(BusId(3)),
            EncodedCodec::OpusRed,
            SAMPLE_RATE_HZ,
            2,
            123_456_789,
            42,
            payload_bytes.clone(),
            AudioSourceTag::AiTts,
            EncryptionMode::SFrame,
        );

        // Then
        assert_eq!(frame.stream_id, StreamId(7));
        assert_eq!(frame.source_id, SourceId(11));
        assert_eq!(frame.bus_id, Some(BusId(3)));
        assert_eq!(frame.codec, EncodedCodec::OpusRed);
        assert_eq!(frame.sample_rate_hz, SAMPLE_RATE_HZ);
        assert_eq!(frame.channels, 2);
        assert_eq!(frame.timestamp_ns, 123_456_789);
        assert_eq!(frame.sequence_number, 42);
        assert_eq!(frame.payload_bytes, payload_bytes);
        assert_eq!(frame.source_tag, AudioSourceTag::AiTts);
        assert_eq!(frame.encryption_mode, EncryptionMode::SFrame);
    }

    #[test]
    fn given_voice_activity_payload_when_event_frame_new_then_payload_round_trips() {
        // Given / When
        let frame = EventFrame::new(
            StreamId(1),
            SourceId(2),
            None,
            10,
            1,
            EventPayload::VoiceActivity(true),
        );

        // Then
        assert_eq!(frame.payload, EventPayload::VoiceActivity(true));
    }

    #[test]
    fn given_transcript_payload_when_event_frame_new_then_payload_round_trips() {
        // Given / When
        let frame = EventFrame::new(
            StreamId(1),
            SourceId(2),
            Some(BusId(9)),
            20,
            2,
            EventPayload::Transcript {
                text: "hello".to_string(),
                is_final: true,
            },
        );

        // Then
        assert_eq!(
            frame.payload,
            EventPayload::Transcript {
                text: "hello".to_string(),
                is_final: true,
            }
        );
    }

    #[test]
    fn given_metadata_payload_when_event_frame_new_then_payload_round_trips() {
        // Given / When
        let frame = EventFrame::new(
            StreamId(1),
            SourceId(2),
            None,
            30,
            3,
            EventPayload::Metadata {
                key: "lang".to_string(),
                value: "en".to_string(),
            },
        );

        // Then
        assert_eq!(
            frame.payload,
            EventPayload::Metadata {
                key: "lang".to_string(),
                value: "en".to_string(),
            }
        );
    }

    #[test]
    fn given_frame_lineage_when_timestamp_end_requested_then_duration_is_saturating() {
        // Given
        let lineage = FrameLineage {
            session_id: SessionId(1),
            source_id: SourceId(2),
            stem_id: StemId(3),
            clock_id: ClockDomainId(4),
            sequence_num: 5,
            timestamp_start_ns: u64::MAX - 2,
            duration_ns: 10,
            source_generation: 6,
            discontinuity_epoch: 7,
            permission_epoch: 8,
        };

        // When / Then
        assert_eq!(lineage.timestamp_end_ns(), u64::MAX);
    }

    #[test]
    fn given_delivery_lineage_when_delivered_then_queue_latency_has_nanosecond_units() {
        // Given
        let delivery = DeliveryLineage {
            endpoint_id: EndpointId(11),
            connector_id: Some(ConnectorId(12)),
            route_id: RouteId(13),
            enqueued_at_ns: 20,
            delivered_at_ns: Some(55),
            delivery_discontinuity_epoch: 0,
        };

        // When / Then
        assert_eq!(delivery.queue_latency_ns(), Some(35));
    }

    #[test]
    fn given_delivery_lineage_when_not_delivered_then_queue_latency_is_absent() {
        // Given
        let delivery = DeliveryLineage {
            endpoint_id: EndpointId(11),
            connector_id: None,
            route_id: RouteId(13),
            enqueued_at_ns: 20,
            delivered_at_ns: None,
            delivery_discontinuity_epoch: 1,
        };

        // When / Then
        assert_eq!(delivery.queue_latency_ns(), None);
    }

    #[test]
    fn given_frozen_buffer_with_many_consumers_when_final_handle_drops_then_slot_is_reused() {
        // Given
        let pool = AudioBufferPool::new(1, 4);
        let mut exclusive = pool.acquire().unwrap();
        exclusive.copy_from_slice(&[0.1, 0.2, 0.3, 0.4]);
        let shared = exclusive.freeze().unwrap();
        let second = shared.try_clone().unwrap();
        let third = shared.try_clone().unwrap();

        // When / Then
        assert_eq!(shared.shared_ref_count(), 3);
        assert!(pool.acquire().is_none());
        drop(second);
        drop(shared);
        assert!(pool.acquire().is_none());
        drop(third);
        assert!(pool.acquire().is_some());
    }

    #[test]
    fn given_rejected_shared_branch_when_handle_drops_then_reference_is_released() {
        // Given
        let pool = AudioBufferPool::new(1, 2);
        let shared = pool.acquire().unwrap().freeze().unwrap();
        let rejected_branch = shared.try_clone().unwrap();

        // When
        drop(rejected_branch);

        // Then
        assert_eq!(shared.shared_ref_count(), 1);
    }

    #[test]
    fn given_queued_shared_branches_when_queue_drops_then_all_references_are_released() {
        // Given
        let pool = AudioBufferPool::new(1, 2);
        let shared = pool.acquire().unwrap().freeze().unwrap();
        let queued = vec![shared.try_clone().unwrap(), shared.try_clone().unwrap()];
        assert_eq!(shared.shared_ref_count(), 3);

        // When
        drop(queued);

        // Then
        assert_eq!(shared.shared_ref_count(), 1);
    }

    #[test]
    fn given_shared_frame_when_copied_to_branch_pool_then_samples_are_independent() {
        // Given
        let source_pool = AudioBufferPool::new(1, 3);
        let branch_pool = AudioBufferPool::new(1, 3);
        let mut buffer = source_pool.acquire().unwrap();
        buffer.copy_from_slice(&[0.25, -0.5, 0.75]);
        let shared = AudioFrame::new(StreamId(11), SourceId(2), 5, 6, 1, buffer)
            .freeze()
            .unwrap();

        // When
        let mut branch = shared.copy_to_pool(&branch_pool).unwrap();
        branch.buffer.as_mut_slice()[0] = 1.0;

        // Then
        assert_eq!(shared.buffer.as_slice(), &[0.25, -0.5, 0.75]);
        assert_eq!(branch.buffer.as_slice(), &[1.0, -0.5, 0.75]);
        assert_eq!(branch.source_id, shared.source_id);
        assert_eq!(branch.sequence_number, shared.sequence_number);
        assert_eq!(branch.timestamp_ns, shared.timestamp_ns);
        assert!(shared.copy_to_pool(&branch_pool).is_none());
    }

    #[test]
    fn given_shared_reference_at_max_when_clone_attempted_then_clone_fails_without_wraparound() {
        // Given
        let pool = AudioBufferPool::new(1, 1);
        let shared = pool.acquire().unwrap().freeze().unwrap();
        pool.shared_ref_counts[0].store(usize::MAX, Ordering::Release);

        // When / Then
        assert!(shared.try_clone().is_none());
        assert_eq!(pool.shared_ref_count(0), usize::MAX);
    }

    #[test]
    fn given_zero_shared_references_when_release_attempted_then_underflow_is_rejected() {
        // Given
        let pool = AudioBufferPool::new(1, 1);

        // When / Then
        assert!(!pool.release_shared(0));
        assert_eq!(pool.shared_ref_count(0), 0);
        assert!(pool.acquire().is_some());
    }
}

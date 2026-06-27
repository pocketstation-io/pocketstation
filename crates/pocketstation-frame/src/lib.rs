use std::cell::UnsafeCell;
use std::fmt;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

pub const SAMPLE_RATE_HZ:    u32   = 48_000;
pub const FRAME_DURATION_MS: u32   = 20;
pub const POOL_SLOT_SAMPLES: usize = 960;    // 20ms × 48kHz
pub const POOL_MAX_SLOTS:    usize = 64;     // AtomicU64 bitset ceiling

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleFormat {
    F32Interleaved,
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
    AiTts,    // EU AI Act §50: watermark required before relay delivery (AUDIO-017)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncryptionMode {
    #[default]
    None,
    SFrame,   // RFC 9605 frame-level E2EE; relay forwards opaque, SDK decrypts
}

pub struct AudioBufferPool {
    slots:            Box<[UnsafeCell<Box<[f32]>>]>,
    slot_size:        usize,      // samples per slot, fixed at creation
    free_mask:        AtomicU64,  // bitset: 1 = free; 64-slot cap
    acquire_failures: AtomicUsize,
}

// SAFETY: Each slot is guarded by its free_mask bit. A slot is exclusively
// accessible only through the AudioBufferHandle that cleared its bit.
// Release sets the bit again exactly once in Drop.
unsafe impl Sync for AudioBufferPool {}

impl AudioBufferPool {
    pub fn new(slot_count: usize, slot_size: usize) -> Arc<Self> {
        assert!((1..=POOL_MAX_SLOTS).contains(&slot_count));
        assert!(slot_size > 0);
        let slots: Vec<_> = (0..slot_count)
            .map(|_| UnsafeCell::new(vec![0.0f32; slot_size].into_boxed_slice()))
            .collect();
        let full_mask = if slot_count == 64 { u64::MAX } else { (1u64 << slot_count) - 1 };
        Arc::new(Self {
            slots:            slots.into_boxed_slice(),
            slot_size,
            free_mask:        AtomicU64::new(full_mask),
            acquire_failures: AtomicUsize::new(0),
        })
    }

    pub fn slot_size(&self) -> usize        { self.slot_size }
    pub fn slot_count(&self) -> usize       { self.slots.len() }
    pub fn acquire_failures(&self) -> usize { self.acquire_failures.load(Ordering::Relaxed) }

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
                    pool:  Arc::clone(self),
                    index: idx as u32,
                    len:   self.slot_size as u32,
                });
            }
        }
    }

    pub fn is_in_use(&self, index: u32) -> bool {
        self.free_mask.load(Ordering::Acquire) & (1u64 << index) == 0
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
    pool:  Arc<AudioBufferPool>,
    index: u32,
    len:   u32,
}

impl AudioBufferHandle {
    pub fn len(&self) -> usize          { self.len as usize }
    pub fn is_empty(&self) -> bool      { self.len == 0 }
    pub fn index(&self) -> u32          { self.index }
    pub fn as_slice(&self) -> &[f32]    { self.pool.slot(self.index, self.len) }

    pub fn as_mut_slice(&mut self) -> &mut [f32] { self.pool.slot_mut(self.index, self.len) }

    pub fn set_len(&mut self, len: usize) {
        assert!(len <= self.pool.slot_size());
        self.len = len as u32;
    }

    pub fn copy_from_slice(&mut self, data: &[f32]) {
        assert!(data.len() <= self.pool.slot_size());
        self.set_len(data.len());
        self.as_mut_slice().copy_from_slice(data);
    }
}

/// Drop contract — must stay forever: wait-free · panic-free · alloc-free · log-free.
impl Drop for AudioBufferHandle {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        debug_assert!(self.pool.is_in_use(self.index), "double-release of slot {}", self.index);
        self.pool.release(self.index); // single atomic fetch_or
    }
}

impl fmt::Debug for AudioBufferHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioBufferHandle")
            .field("index", &self.index)
            .field("len",   &self.len)
            .finish()
    }
}

#[derive(Debug)]
pub struct AudioFrame {
    pub stream_id:       StreamId,
    pub source_id:       SourceId,
    pub sample_rate_hz:  u32,             // always 48_000 internally; see DOCS-013
    pub channels:        u8,              // 1 = voice, 2 = music/broadcast
    pub format:          SampleFormat,
    pub timestamp_ns:    u64,             // monotonic, never wall clock
    pub sequence_number: u64,
    pub buffer:          AudioBufferHandle,
    pub source_tag:      AudioSourceTag,  // AiTts triggers AUDIO-017 watermark
    pub speaker_id:      Option<u32>,     // assigned by diarization node (Phase 6)
    pub encryption_mode: EncryptionMode,
}

impl AudioFrame {
    pub fn new(
        stream_id:       StreamId,
        source_id:       SourceId,
        sequence_number: u64,
        timestamp_ns:    u64,
        channels:        u8,
        buffer:          AudioBufferHandle,
    ) -> Self {
        Self {
            stream_id,
            source_id,
            sample_rate_hz:  SAMPLE_RATE_HZ,
            channels,
            format:          SampleFormat::F32Interleaved,
            timestamp_ns,
            sequence_number,
            buffer,
            source_tag:      AudioSourceTag::Captured,
            speaker_id:      None,
            encryption_mode: EncryptionMode::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acquire_all_slots_then_none() {
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
    fn handle_copy_sets_length() {
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
    fn acquire_all_64_slots_then_65th_returns_none() {
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
    fn drop_releases_slot_and_reacquire_succeeds() {
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
    fn is_in_use_tracks_acquire_and_release() {
        // Given
        let pool = AudioBufferPool::new(2, 4);
        let h = pool.acquire().unwrap();
        let slot = h.index();

        // When / Then
        assert!(pool.is_in_use(slot));
        drop(h);
        assert!(!pool.is_in_use(slot));
    }
}

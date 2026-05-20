use std::cell::UnsafeCell;
use std::fmt;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

pub const DEFAULT_SAMPLE_RATE: u32 = 48_000;
pub const DEFAULT_FRAME_MS: u32 = 20;
pub const DEFAULT_SLOT_SAMPLES_MONO_20MS: usize = 960;
pub const MAX_POOL_SLOTS: usize = 64;

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

pub struct AudioBufferPool {
    slots: Box<[UnsafeCell<Box<[f32]>>]>,
    slot_size: usize,
    free_mask: AtomicU64,
    acquire_failures: AtomicUsize,
}

// SAFETY: Each slot is protected by the `free_mask` bitset. A slot can only be
// mutably accessed through an `AudioBufferHandle` obtained by successfully
// clearing its free bit. Release sets the bit again exactly once in Drop.
unsafe impl Sync for AudioBufferPool {}

impl AudioBufferPool {
    pub fn new(slot_count: usize, slot_size: usize) -> Arc<Self> {
        assert!((1..=MAX_POOL_SLOTS).contains(&slot_count));
        assert!(slot_size > 0);
        let mut slots = Vec::with_capacity(slot_count);
        for _ in 0..slot_count {
            slots.push(UnsafeCell::new(vec![0.0f32; slot_size].into_boxed_slice()));
        }
        let mask = if slot_count == 64 {
            u64::MAX
        } else {
            (1u64 << slot_count) - 1
        };
        Arc::new(Self {
            slots: slots.into_boxed_slice(),
            slot_size,
            free_mask: AtomicU64::new(mask),
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

    pub fn acquire(self: &Arc<Self>) -> Option<AudioBufferHandle> {
        loop {
            let mask = self.free_mask.load(Ordering::Acquire);
            if mask == 0 {
                self.acquire_failures.fetch_add(1, Ordering::Relaxed);
                return None;
            }
            let idx = mask.trailing_zeros() as usize;
            let bit = 1u64 << idx;
            let next = mask & !bit;
            if self
                .free_mask
                .compare_exchange_weak(mask, next, Ordering::AcqRel, Ordering::Acquire)
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

    fn release(&self, index: u32) {
        let idx = index as usize;
        if idx >= self.slots.len() {
            return;
        }
        let bit = 1u64 << idx;
        #[cfg(debug_assertions)]
        {
            let old = self.free_mask.fetch_or(bit, Ordering::Release);
            debug_assert_eq!(old & bit, 0, "double release of buffer slot {}", idx);
        }
        #[cfg(not(debug_assertions))]
        {
            self.free_mask.fetch_or(bit, Ordering::Release);
        }
    }

    pub fn is_in_use(&self, index: u32) -> bool {
        let bit = 1u64 << index;
        self.free_mask.load(Ordering::Acquire) & bit == 0
    }

    fn slot(&self, index: u32, len: u32) -> &[f32] {
        let idx = index as usize;
        let len = len as usize;
        assert!(idx < self.slots.len());
        assert!(len <= self.slot_size);
        // SAFETY: immutable access for read-only slice; unique mutable access is
        // only exposed through the owning handle.
        let slot = unsafe { &*self.slots[idx].get() };
        &slot[..len]
    }

    #[allow(clippy::mut_from_ref)]
    fn slot_mut(&self, index: u32, len: u32) -> &mut [f32] {
        let idx = index as usize;
        let len = len as usize;
        assert!(idx < self.slots.len());
        assert!(len <= self.slot_size);
        // SAFETY: acquisition protocol ensures exactly one live handle per slot.
        let slot = unsafe { &mut *self.slots[idx].get() };
        &mut slot[..len]
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
}

impl Drop for AudioBufferHandle {
    fn drop(&mut self) {
        self.pool.release(self.index);
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

#[derive(Debug)]
pub struct AudioFrame {
    pub stream_id: StreamId,
    pub source_id: SourceId,
    pub sample_rate: u32,
    pub channels: u8,
    pub format: SampleFormat,
    pub timestamp_ns: u64,
    pub sequence_number: u64,
    pub buffer: AudioBufferHandle,
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
            sample_rate: DEFAULT_SAMPLE_RATE,
            channels,
            format: SampleFormat::F32Interleaved,
            timestamp_ns,
            sequence_number,
            buffer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acquire_all_slots_then_none() {
        let pool = AudioBufferPool::new(4, 16);
        let a = pool.acquire().unwrap();
        let b = pool.acquire().unwrap();
        let c = pool.acquire().unwrap();
        let d = pool.acquire().unwrap();
        assert!(pool.acquire().is_none());
        drop(a);
        drop(b);
        drop(c);
        drop(d);
        assert!(pool.acquire().is_some());
    }

    #[test]
    fn handle_copy_sets_length() {
        let pool = AudioBufferPool::new(1, 8);
        let mut h = pool.acquire().unwrap();
        h.copy_from_slice(&[1.0, 2.0, 3.0]);
        assert_eq!(h.len(), 3);
        assert_eq!(h.as_slice(), &[1.0, 2.0, 3.0]);
    }
}

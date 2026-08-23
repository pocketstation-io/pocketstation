//! Fixed-capacity realtime audio storage and ownership handles.
//!
//! Allocation happens only when the pool is constructed. Acquire, share, and
//! final `Drop` use atomics and never allocate, lock, block, or log.

use std::cell::UnsafeCell;
use std::fmt;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

#[doc = "Defines the public pool max slots value."]
pub const POOL_MAX_SLOTS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as audio buffer write error."]
pub enum AudioBufferWriteError {
    #[error(
        "audio buffer write of {requested_samples} samples exceeds capacity {capacity_samples}"
    )]
    #[doc = "Reports capacity exceeded."]
    CapacityExceeded {
        #[doc = "Stores the requested samples used by `CapacityExceeded`."]
        requested_samples: usize,
        #[doc = "Sets the capacity samples available to `CapacityExceeded`."]
        capacity_samples: usize,
    },
}

#[doc = "Owns fixed-capacity reusable audio slots and reports acquisition pressure without allocating per frame."]
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
    #[doc = "Creates a new `AudioBufferPool`."]
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

    #[doc = "Returns the slot size held by `AudioBufferPool`."]
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }
    #[doc = "Returns the slot count held by `AudioBufferPool`."]
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }
    #[doc = "Returns the acquire failures held by `AudioBufferPool`."]
    pub fn acquire_failures(&self) -> usize {
        self.acquire_failures.load(Ordering::Relaxed)
    }
    #[doc = "Returns the available slots held by `AudioBufferPool`."]
    pub fn available_slots(&self) -> usize {
        self.free_mask.load(Ordering::Acquire).count_ones() as usize
    }

    #[doc = "Attempts to acquire an available buffer slot from `AudioBufferPool`."]
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

    #[doc = "Returns whether in use applies to `AudioBufferPool`."]
    pub fn is_in_use(&self, index: u32) -> bool {
        self.free_mask.load(Ordering::Acquire) & (1u64 << index) == 0
    }

    #[doc = "Returns the shared ref count held by `AudioBufferPool`."]
    pub fn shared_ref_count(&self, index: u32) -> usize {
        self.shared_ref_counts
            .get(index as usize)
            .map_or(0, |count| count.load(Ordering::Acquire))
    }

    #[cfg(test)]
    pub(super) fn set_shared_ref_count_for_testing(&self, index: u32, count: usize) {
        self.shared_ref_counts[index as usize].store(count, Ordering::Release);
    }

    #[cfg(test)]
    pub(super) fn release_shared_for_testing(&self, index: u32) -> bool {
        self.release_shared(index)
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

    /// Returns a raw mutable view so the pool itself never manufactures a
    /// mutable reference from shared access. The exclusive handle is the only
    /// authority allowed to turn this pointer into `&mut [f32]`.
    unsafe fn slot_mut_ptr(&self, index: u32, len: u32) -> *mut [f32] {
        assert!((index as usize) < self.slots.len() && (len as usize) <= self.slot_size);
        // SAFETY: the caller guarantees the acquisition protocol has exactly
        // one exclusive handle for this slot. Bounds were checked above.
        let cell = unsafe { &mut *self.slots[index as usize].get() };
        std::ptr::slice_from_raw_parts_mut(cell.as_mut_ptr(), len as usize)
    }
}

#[doc = "Owns bounded access to audio buffer."]
pub struct AudioBufferHandle {
    pool: Arc<AudioBufferPool>,
    index: u32,
    len: u32,
}

impl AudioBufferHandle {
    #[doc = "Returns the number of values held by `AudioBufferHandle`."]
    pub fn len(&self) -> usize {
        self.len as usize
    }
    #[doc = "Returns whether `AudioBufferHandle` contains no values."]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    #[doc = "Returns the index held by `AudioBufferHandle`."]
    pub fn index(&self) -> u32 {
        self.index
    }
    #[doc = "Borrows `AudioBufferHandle` as slice."]
    pub fn as_slice(&self) -> &[f32] {
        self.pool.slot(self.index, self.len)
    }

    #[doc = "Borrows `AudioBufferHandle` as mut slice."]
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        // SAFETY: `&mut self` proves exclusive access to the only mutable
        // handle. Frozen/shared handles cannot coexist with this handle.
        unsafe { &mut *self.pool.slot_mut_ptr(self.index, self.len) }
    }

    /// Changes the visible sample length without panicking.
    ///
    /// Capture callbacks and realtime partitions must use this method instead
    /// of the assertion-based compatibility setter.
    pub fn try_set_len(&mut self, len: usize) -> Result<(), AudioBufferWriteError> {
        if len > self.pool.slot_size() {
            return Err(AudioBufferWriteError::CapacityExceeded {
                requested_samples: len,
                capacity_samples: self.pool.slot_size(),
            });
        }
        self.len = len as u32;
        Ok(())
    }

    /// Copies samples into this fixed-capacity slot without panicking.
    pub fn try_copy_from_slice(&mut self, data: &[f32]) -> Result<(), AudioBufferWriteError> {
        self.try_set_len(data.len())?;
        self.as_mut_slice().copy_from_slice(data);
        Ok(())
    }

    #[doc = "Freezes mutable storage owned by `AudioBufferHandle` into its shared immutable form."]
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
    #[doc = "Releases resources owned by `AudioBufferHandle`."]
    fn drop(&mut self) {
        if self.pool.shared_ref_count(self.index) == 0 {
            self.pool.release(self.index);
        }
    }
}

impl fmt::Debug for AudioBufferHandle {
    #[doc = "Formats `AudioBufferHandle` with the requested formatter."]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioBufferHandle")
            .field("index", &self.index)
            .field("len", &self.len)
            .finish()
    }
}

#[doc = "Owns bounded access to shared audio buffer."]
pub struct SharedAudioBufferHandle {
    pool: Arc<AudioBufferPool>,
    index: u32,
    len: u32,
}

impl SharedAudioBufferHandle {
    #[doc = "Returns the number of values held by `SharedAudioBufferHandle`."]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    #[doc = "Returns whether `SharedAudioBufferHandle` contains no values."]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[doc = "Returns the index held by `SharedAudioBufferHandle`."]
    pub fn index(&self) -> u32 {
        self.index
    }

    #[doc = "Borrows `SharedAudioBufferHandle` as slice."]
    pub fn as_slice(&self) -> &[f32] {
        self.pool.slot(self.index, self.len)
    }

    #[doc = "Attempts to clone through `SharedAudioBufferHandle`."]
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

    #[doc = "Returns the shared ref count held by `SharedAudioBufferHandle`."]
    pub fn shared_ref_count(&self) -> usize {
        self.pool.shared_ref_count(self.index)
    }
}

/// Drop contract — must stay forever: lock-free · panic-free · alloc-free · log-free.
impl Drop for SharedAudioBufferHandle {
    #[doc = "Releases resources owned by `SharedAudioBufferHandle`."]
    fn drop(&mut self) {
        self.pool.release_shared(self.index);
    }
}

impl fmt::Debug for SharedAudioBufferHandle {
    #[doc = "Formats `SharedAudioBufferHandle` with the requested formatter."]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedAudioBufferHandle")
            .field("index", &self.index)
            .field("len", &self.len)
            .field("shared_ref_count", &self.shared_ref_count())
            .finish()
    }
}

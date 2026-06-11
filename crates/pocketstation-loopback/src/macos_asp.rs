//! Reads from the PocketStation HAL plugin POSIX shared memory ring.

use std::ffi::c_float;
use std::ffi::c_uint;

extern "C" {
    fn pks_asp_is_installed() -> std::ffi::c_int;
    fn pks_asp_open_reader() -> *mut std::ffi::c_void;
    fn pks_asp_sample_rate(r: *mut std::ffi::c_void) -> c_uint;
    fn pks_asp_channels(r: *mut std::ffi::c_void) -> c_uint;
    fn pks_asp_drop_count(r: *mut std::ffi::c_void) -> u64;
    fn pks_asp_read_frames(r: *mut std::ffi::c_void, out: *mut c_float, frame_count: c_uint) -> c_uint;
    fn pks_asp_close_reader(r: *mut std::ffi::c_void);
}

/// Returns `true` if the PocketStation HAL plugin is active (shared memory ring exists).
pub fn asp_is_installed() -> bool {
    // SAFETY: pure C function, no side effects, no pointer args.
    unsafe { pks_asp_is_installed() != 0 }
}

/// RAII wrapper around the C reader handle.
pub struct AspReader(*mut std::ffi::c_void);

// SAFETY: AspReader owns the C heap-allocated PksReader. The ring is PROT_READ mmap —
// read-only shared memory. No mutation through the pointer from Rust side.
unsafe impl Send for AspReader {}

impl AspReader {
    /// Open the shared memory ring. Returns `None` if the plugin is not running.
    pub fn open() -> Option<Self> {
        // SAFETY: returns NULL on failure; we check before wrapping.
        let ptr = unsafe { pks_asp_open_reader() };
        if ptr.is_null() { None } else { Some(Self(ptr)) }
    }

    pub fn sample_rate(&self) -> u32 {
        // SAFETY: self.0 is valid (checked in open()).
        unsafe { pks_asp_sample_rate(self.0) }
    }

    pub fn channels(&self) -> u32 {
        // SAFETY: self.0 is valid (checked in open()).
        unsafe { pks_asp_channels(self.0) }
    }

    pub fn drop_count(&self) -> u64 {
        // SAFETY: self.0 is valid (checked in open()).
        unsafe { pks_asp_drop_count(self.0) }
    }

    /// Read up to `frame_count` interleaved f32 frames into `buf`.
    /// Returns the number of frames actually read.
    ///
    /// # Safety contract
    /// `buf.len()` must be >= `frame_count * channels()`.
    pub fn read_frames(&mut self, buf: &mut [f32], frame_count: u32) -> u32 {
        // SAFETY: buf is valid for buf.len() f32 values; caller ensures
        // frame_count * channels ≤ buf.len().
        unsafe { pks_asp_read_frames(self.0, buf.as_mut_ptr(), frame_count) }
    }
}

impl Drop for AspReader {
    fn drop(&mut self) {
        // SAFETY: self.0 is the only owner; close_reader frees it.
        unsafe { pks_asp_close_reader(self.0) }
    }
}

//! Reads from the PocketStation HAL plugin POSIX shared memory ring.

use std::ffi::c_float;
use std::ffi::c_uint;

extern "C" {
    fn pks_asp_is_installed() -> std::ffi::c_int;
    fn pks_asp_open_reader() -> *mut std::ffi::c_void;
    fn pks_asp_sample_rate(r: *mut std::ffi::c_void) -> c_uint;
    fn pks_asp_channels(r: *mut std::ffi::c_void) -> c_uint;
    // Monitoring metric: cumulative frames rejected before publication.
    #[allow(dead_code)]
    fn pks_asp_drop_count(r: *mut std::ffi::c_void) -> u64;
    fn pks_asp_timeline_reject_callback_count(r: *mut std::ffi::c_void) -> u64;
    fn pks_asp_read_frames(
        r: *mut std::ffi::c_void,
        out: *mut c_float,
        frame_count: c_uint,
        out_source_frame_position: *mut u64,
    ) -> c_uint;
    fn pks_asp_close_reader(r: *mut std::ffi::c_void);
}

/// Returns `true` if the PocketStation HAL plugin is active (shared memory ring exists).
pub fn asp_is_installed() -> bool {
    // SAFETY: pure C function, no side effects, no pointer args.
    unsafe { pks_asp_is_installed() != 0 }
}

/// RAII wrapper around the C reader handle.
pub struct AspReader(*mut std::ffi::c_void);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AspReadBatch {
    pub frame_count: u32,
    pub source_frame_position_frames: u64,
}

// SAFETY: AspReader owns the C heap-allocated PksReader. The C layer enforces a
// single attached consumer and mutates only the consumer-owned read head.
unsafe impl Send for AspReader {}

impl AspReader {
    /// Open the shared memory ring. Returns `None` if the plugin is not running.
    pub fn open() -> Option<Self> {
        // SAFETY: returns NULL on failure; we check before wrapping.
        let ptr = unsafe { pks_asp_open_reader() };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    pub fn sample_rate(&self) -> u32 {
        // SAFETY: self.0 is valid (checked in open()).
        unsafe { pks_asp_sample_rate(self.0) }
    }

    pub fn channels(&self) -> u32 {
        // SAFETY: self.0 is valid (checked in open()).
        unsafe { pks_asp_channels(self.0) }
    }

    /// Returns the cumulative number of frames rejected before publication.
    #[allow(dead_code)]
    pub fn drop_count(&self) -> u64 {
        // SAFETY: self.0 is valid (checked in open()).
        unsafe { pks_asp_drop_count(self.0) }
    }

    /// Returns callbacks rejected for invalid native source positions.
    pub fn timeline_reject_callback_count(&self) -> u64 {
        // SAFETY: self.0 is valid (checked in open()).
        unsafe { pks_asp_timeline_reject_callback_count(self.0) }
    }

    /// Read up to `frame_count` interleaved f32 frames into `buf`.
    /// Returns the frame count and native position of the first returned frame.
    ///
    /// # Safety contract
    /// `buf.len()` must be >= `frame_count * channels()`.
    pub fn read_frames(&mut self, buf: &mut [f32], frame_count: u32) -> AspReadBatch {
        let mut source_frame_position = 0u64;
        // SAFETY: buf is valid for buf.len() f32 values; caller ensures
        // frame_count * channels ≤ buf.len().
        let frame_count = unsafe {
            pks_asp_read_frames(
                self.0,
                buf.as_mut_ptr(),
                frame_count,
                &mut source_frame_position,
            )
        };
        AspReadBatch {
            frame_count,
            source_frame_position_frames: source_frame_position,
        }
    }
}

impl Drop for AspReader {
    fn drop(&mut self) {
        // SAFETY: self.0 is the only owner; close_reader frees it.
        unsafe { pks_asp_close_reader(self.0) }
    }
}

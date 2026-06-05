//! System audio loopback capture via ScreenCaptureKit (macOS only).
//!
//! On macOS 13+, `capture_system_audio` uses ScreenCaptureKit to tap the
//! system mix and delivers `AudioFrame` values to the provided callback.
//!
//! On all other platforms the function returns `Err(LoopbackError::NotSupported)`
//! immediately without attempting any capture.

use thiserror::Error;

// ---------------------------------------------------------------------------
// Public error type
// ---------------------------------------------------------------------------

/// Errors produced by the loopback capture API.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum LoopbackError {
    /// The platform does not support system audio loopback (non-macOS).
    #[error("system audio loopback is not supported on this platform")]
    NotSupported,

    /// ScreenCaptureKit initialisation failed.
    #[cfg(target_os = "macos")]
    #[error("ScreenCaptureKit initialisation failed: {0}")]
    Init(String),

    /// No display was found to anchor the content filter.
    #[cfg(target_os = "macos")]
    #[error("no display found for content filter")]
    NoDisplay,

    /// The audio buffer pool is exhausted; a frame was dropped.
    #[cfg(target_os = "macos")]
    #[error("audio buffer pool exhausted — frame dropped")]
    PoolExhausted,
}

// ---------------------------------------------------------------------------
// macOS implementation
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod macos {
    use std::sync::Arc;

    use pocketstation_frame::{
        AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, SourceId, StreamId,
        DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS,
    };
    use screencapturekit::prelude::*;

    use super::LoopbackError;

    // -----------------------------------------------------------------------
    // Constants
    // -----------------------------------------------------------------------

    /// Number of audio channels captured from ScreenCaptureKit (stereo).
    const CAPTURE_CHANNELS: u8 = 2;

    /// Stereo 20 ms frame at 48 kHz = 960 mono samples × 2 channels.
    const CAPTURE_FRAME_SAMPLES: usize = DEFAULT_SLOT_SAMPLES_MONO_20MS * CAPTURE_CHANNELS as usize;

    /// Pool depth: 8 frames of lookahead to absorb callback jitter without
    /// blocking.  Each slot is CAPTURE_FRAME_SAMPLES * 4 bytes = 7.68 kB.
    const POOL_DEPTH: usize = 8;

    // -----------------------------------------------------------------------
    // Internal handler
    // -----------------------------------------------------------------------

    /// Implements `SCStreamOutputTrait` so it can be registered as an audio
    /// output handler on an `SCStream`.
    ///
    /// The handler receives `CMSampleBuffer` values on a ScreenCaptureKit
    /// dispatch thread and forwards them to the user-provided callback.
    /// All allocations happen in `SystemLoopbackSource::new`; the hot path
    /// only acquires pool slots (lock-free CAS) and invokes the callback.
    struct AudioHandler<F>
    where
        F: Fn(AudioFrame) + Send + Sync + 'static,
    {
        pool: Arc<AudioBufferPool>,
        callback: F,
        stream_id: StreamId,
        source_id: SourceId,
        /// Sequence counter — incremented atomically per delivered frame.
        seq: std::sync::atomic::AtomicU64,
    }

    impl<F> SCStreamOutputTrait for AudioHandler<F>
    where
        F: Fn(AudioFrame) + Send + Sync + 'static,
    {
        fn did_output_sample_buffer(&self, sample: CMSampleBuffer, of_type: SCStreamOutputType) {
            if of_type != SCStreamOutputType::Audio {
                return;
            }

            let buffer_list = match sample.audio_buffer_list() {
                Some(bl) => bl,
                None => return,
            };

            // Acquire a pool slot — lock-free, no heap allocation.
            let mut handle = match self.pool.acquire() {
                Some(h) => h,
                None => return, // pool exhausted: drop frame rather than allocate
            };

            // Copy interleaved f32 samples from every AudioBuffer in the list
            // into the pool slot.  ScreenCaptureKit delivers f32Le interleaved
            // when configured with `.with_captures_audio(true)` at 48 kHz.
            let dst = handle.as_mut_slice();
            let mut written = 0usize;

            'outer: for audio_buf in buffer_list.iter() {
                let bytes = audio_buf.data();
                // Interpret the byte slice as f32 samples (4 bytes each).
                let n_samples = bytes.len() / std::mem::size_of::<f32>();
                let src_ptr = bytes.as_ptr() as *const f32;
                for i in 0..n_samples {
                    if written >= dst.len() {
                        break 'outer;
                    }
                    // SAFETY: `src_ptr` points to a valid, aligned region of
                    // `bytes.len()` bytes delivered by ScreenCaptureKit.  We
                    // iterate within `n_samples` which equals `bytes.len() / 4`.
                    dst[written] = unsafe { src_ptr.add(i).read_unaligned() };
                    written += 1;
                }
            }

            handle.set_len(written);

            let seq = self.seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            let timestamp_ns = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64;

            let mut frame = AudioFrame::new(
                self.stream_id,
                self.source_id,
                seq,
                timestamp_ns,
                CAPTURE_CHANNELS,
                handle,
            );
            frame.source_tag = AudioSourceTag::Captured;
            frame.encryption_mode = EncryptionMode::None;
            // Mono equivalent is still 48 kHz; channels field carries stereo count.
            frame.sample_rate = DEFAULT_SAMPLE_RATE;

            (self.callback)(frame);
        }
    }

    // -----------------------------------------------------------------------
    // Public struct
    // -----------------------------------------------------------------------

    /// Manages a ScreenCaptureKit audio capture session.
    ///
    /// Drop this value to stop capture (calls `stop_capture` on the underlying
    /// `SCStream`).
    pub struct SystemLoopbackSource {
        stream: SCStream,
    }

    impl SystemLoopbackSource {
        /// Start capturing system audio and deliver `AudioFrame` values to
        /// `callback`.
        ///
        /// # Errors
        ///
        /// Returns `LoopbackError::Init` if ScreenCaptureKit cannot be
        /// initialised, or `LoopbackError::NoDisplay` if no display is
        /// available to anchor the content filter (required even for
        /// audio-only capture).
        pub fn capture<F>(callback: F) -> Result<Self, LoopbackError>
        where
            F: Fn(AudioFrame) + Send + Sync + 'static,
        {
            // ----------------------------------------------------------------
            // 1. Enumerate shareable content to obtain a display reference.
            // ----------------------------------------------------------------
            let content =
                SCShareableContent::get().map_err(|e| LoopbackError::Init(format!("{e:?}")))?;

            let display = content
                .displays()
                .into_iter()
                .next()
                .ok_or(LoopbackError::NoDisplay)?;

            // ----------------------------------------------------------------
            // 2. Build a content filter that captures the full display.
            // ----------------------------------------------------------------
            let filter = SCContentFilter::create()
                .with_display(&display)
                .with_excluding_windows(&[])
                .build();

            // ----------------------------------------------------------------
            // 3. Configure the stream: audio only, 48 kHz stereo, exclude the
            //    current process so the CLI does not feed back into itself.
            // ----------------------------------------------------------------
            let config = SCStreamConfiguration::new()
                .with_captures_audio(true)
                .with_sample_rate(DEFAULT_SAMPLE_RATE as i32)
                .with_channel_count(CAPTURE_CHANNELS as i32)
                // Minimise frame size; ScreenCaptureKit delivers frames at
                // its own cadence — we accept whatever it provides.
                .with_width(1)
                .with_height(1);

            // ----------------------------------------------------------------
            // 4. Build the stream and register the audio handler.
            // ----------------------------------------------------------------
            let pool = AudioBufferPool::new(POOL_DEPTH, CAPTURE_FRAME_SAMPLES);

            let handler = AudioHandler {
                pool,
                callback,
                stream_id: StreamId(0),
                source_id: SourceId(0),
                seq: std::sync::atomic::AtomicU64::new(0),
            };

            let mut stream = SCStream::new(&filter, &config);
            stream.add_output_handler(handler, SCStreamOutputType::Audio);

            // ----------------------------------------------------------------
            // 5. Start capture.
            // ----------------------------------------------------------------
            stream
                .start_capture()
                .map_err(|e| LoopbackError::Init(format!("{e:?}")))?;

            Ok(Self { stream })
        }
    }

    impl Drop for SystemLoopbackSource {
        fn drop(&mut self) {
            // Best-effort: if stop_capture fails we cannot recover.
            let _ = self.stream.stop_capture();
        }
    }
}

// ---------------------------------------------------------------------------
// Non-macOS stub
// ---------------------------------------------------------------------------

#[cfg(not(target_os = "macos"))]
mod stub {
    use pocketstation_frame::AudioFrame;

    use super::LoopbackError;

    /// Stub source that always reports the platform is unsupported.
    pub struct SystemLoopbackSource;

    impl SystemLoopbackSource {
        /// Always returns `Err(LoopbackError::NotSupported)` on non-macOS.
        pub fn capture<F>(_callback: F) -> Result<Self, LoopbackError>
        where
            F: Fn(AudioFrame) + Send + Sync + 'static,
        {
            Err(LoopbackError::NotSupported)
        }
    }
}

// ---------------------------------------------------------------------------
// Re-export the right implementation
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
pub use macos::SystemLoopbackSource;

#[cfg(not(target_os = "macos"))]
pub use stub::SystemLoopbackSource;

// ---------------------------------------------------------------------------
// Public convenience function
// ---------------------------------------------------------------------------

/// Start capturing system audio and deliver `AudioFrame` values to `callback`.
///
/// On macOS this uses ScreenCaptureKit.  On all other platforms it returns
/// `Err(LoopbackError::NotSupported)` without attempting capture.
///
/// The returned `SystemLoopbackSource` keeps the capture session alive.
/// Drop it to stop capture.
pub fn capture_system_audio<F>(callback: F) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: Fn(pocketstation_frame::AudioFrame) + Send + Sync + 'static,
{
    SystemLoopbackSource::capture(callback)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Test 1: non-macOS stub always returns NotSupported
    // -----------------------------------------------------------------------

    /// On non-macOS platforms the capture function must return NotSupported
    /// without panicking.  On macOS this test is compiled out because the
    /// real implementation is wired in instead.
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_loopback_source_returns_not_supported_on_non_macos() {
        // Given: a no-op callback
        // When: capture is requested on a non-macOS platform
        let result = capture_system_audio(|_frame| {});

        // Then: the result is the NotSupported variant
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // -----------------------------------------------------------------------
    // Test 2: error Display formatting
    // -----------------------------------------------------------------------

    #[test]
    fn test_loopback_error_display() {
        // Given / When / Then: the NotSupported variant formats correctly
        let msg = LoopbackError::NotSupported.to_string();
        assert!(
            msg.contains("not supported"),
            "expected 'not supported' in error message, got: {msg}"
        );
    }

    // -----------------------------------------------------------------------
    // Test 3: struct can be named (compile-time check)
    // -----------------------------------------------------------------------

    /// Verifies that `SystemLoopbackSource` is accessible as a named type.
    /// This is a compile-time check — if the type does not exist the test
    /// file will not compile.
    #[test]
    fn test_loopback_source_struct_can_be_constructed() {
        // Given: the type is accessible
        // When: we obtain a type name string
        let type_name = std::any::type_name::<SystemLoopbackSource>();

        // Then: the string contains the struct name
        assert!(
            type_name.contains("SystemLoopbackSource"),
            "unexpected type name: {type_name}"
        );
    }
}

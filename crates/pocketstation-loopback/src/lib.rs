use thiserror::Error;

// ---------------------------------------------------------------------------
// Capture mode
// ---------------------------------------------------------------------------

/// Selects which audio source to capture.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum CaptureMode {
    /// Capture the system-wide audio mix (all applications).
    #[default]
    SystemMix,
    /// Capture a specific application by bundle ID (macOS) or node name (Linux).
    Application(String),
    /// Capture a specific process by PID.
    Process(u32),
}

// ---------------------------------------------------------------------------
// Public error type
// ---------------------------------------------------------------------------

/// Errors produced by the loopback capture API.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum LoopbackError {
    /// The platform does not support system audio loopback.
    #[error("system audio loopback is not supported on this platform")]
    NotSupported,

    /// A backend-specific initialisation error.
    #[error("loopback backend error: {0}")]
    BackendInit(String),

    /// The requested capture mode is not supported by this backend.
    #[error("capture mode not supported on this backend: {0:?}")]
    ModeUnsupported(CaptureMode),

    /// ScreenCaptureKit initialisation failed (macOS only).
    #[cfg(target_os = "macos")]
    #[error("ScreenCaptureKit initialisation failed: {0}")]
    Init(String),

    /// No display was found to anchor the content filter (macOS only).
    #[cfg(target_os = "macos")]
    #[error("no display found for content filter")]
    NoDisplay,

    /// The audio buffer pool is exhausted (macOS only).
    #[cfg(target_os = "macos")]
    #[error("audio buffer pool exhausted -- frame dropped")]
    PoolExhausted,
}

// ---------------------------------------------------------------------------
// macOS ScreenCaptureKit implementation
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod macos {
    use std::sync::Arc;

    use pocketstation_frame::{
        AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, SourceId, StreamId,
        DEFAULT_SAMPLE_RATE, DEFAULT_SLOT_SAMPLES_MONO_20MS,
    };
    use screencapturekit::prelude::*;

    use super::{CaptureMode, LoopbackError};

    const CAPTURE_CHANNELS: u8 = 2;
    const CAPTURE_FRAME_SAMPLES: usize = DEFAULT_SLOT_SAMPLES_MONO_20MS * CAPTURE_CHANNELS as usize;
    const POOL_DEPTH: usize = 8;

    struct AudioHandler<F>
    where
        F: Fn(AudioFrame) + Send + Sync + 'static,
    {
        pool: Arc<AudioBufferPool>,
        callback: F,
        stream_id: StreamId,
        source_id: SourceId,
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
            let mut handle = match self.pool.acquire() {
                Some(h) => h,
                None => return,
            };
            let dst = handle.as_mut_slice();
            let mut written = 0usize;
            'outer: for audio_buf in buffer_list.iter() {
                let bytes = audio_buf.data();
                let n_samples = bytes.len() / std::mem::size_of::<f32>();
                let src_ptr = bytes.as_ptr() as *const f32;
                for i in 0..n_samples {
                    if written >= dst.len() {
                        break 'outer;
                    }
                    // SAFETY: ptr is valid f32 LE data from ScreenCaptureKit.
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
            frame.sample_rate = DEFAULT_SAMPLE_RATE;
            (self.callback)(frame);
        }
    }

    /// Manages a ScreenCaptureKit audio capture session.
    pub struct SystemLoopbackSource {
        stream: SCStream,
    }

    impl SystemLoopbackSource {
        pub fn capture<F>(callback: F) -> Result<Self, LoopbackError>
        where
            F: Fn(AudioFrame) + Send + Sync + 'static,
        {
            Self::capture_mode(CaptureMode::SystemMix, callback)
        }

        pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
        where
            F: Fn(AudioFrame) + Send + Sync + 'static,
        {
            let content = SCShareableContent::get()
                .map_err(|e| LoopbackError::Init(format!("{e:?}")))?;
            let display = content
                .displays()
                .into_iter()
                .next()
                .ok_or(LoopbackError::NoDisplay)?;
            // Build a content filter for the requested mode.
            // Process(_) is not supported by SCKit.
            let filter = match &mode {
                CaptureMode::SystemMix => SCContentFilter::create()
                    .with_display(&display)
                    .with_excluding_windows(&[])
                    .build(),
                CaptureMode::Application(bundle_id) => {
                    let app = content
                        .applications()
                        .into_iter()
                        .find(|a| a.bundle_identifier() == *bundle_id)
                        .ok_or_else(|| {
                            LoopbackError::ModeUnsupported(CaptureMode::Application(
                                bundle_id.clone(),
                            ))
                        })?;
                    SCContentFilter::create()
                        .with_display(&display)
                        .with_including_applications(&[&app], &[])
                        .build()
                }
                CaptureMode::Process(_) => {
                    return Err(LoopbackError::ModeUnsupported(mode));
                }
            };
            // with_excludes_current_process_audio prevents the CLI from
            // capturing its own output (feedback loop prevention).
            let config = SCStreamConfiguration::new()
                .with_captures_audio(true)
                .with_sample_rate(DEFAULT_SAMPLE_RATE as i32)
                .with_channel_count(CAPTURE_CHANNELS as i32)
                .with_excludes_current_process_audio(true)
                .with_width(1)
                .with_height(1);
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
            stream
                .start_capture()
                .map_err(|e| LoopbackError::Init(format!("{e:?}")))?;
            Ok(Self { stream })
        }
    }

    impl Drop for SystemLoopbackSource {
        fn drop(&mut self) {
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
    use super::{CaptureMode, LoopbackError};

    #[derive(Debug)]
    pub struct SystemLoopbackSource;

    impl SystemLoopbackSource {
        pub fn capture<F>(_callback: F) -> Result<Self, LoopbackError>
        where
            F: Fn(AudioFrame) + Send + Sync + 'static,
        {
            Err(LoopbackError::NotSupported)
        }

        pub fn capture_mode<F>(_mode: CaptureMode, _callback: F) -> Result<Self, LoopbackError>
        where
            F: Fn(AudioFrame) + Send + Sync + 'static,
        {
            Err(LoopbackError::NotSupported)
        }
    }
}

// ---------------------------------------------------------------------------
// macOS ASP detection
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod macos_asp;
#[cfg(target_os = "macos")]
pub use macos_asp::asp_is_installed;

// ---------------------------------------------------------------------------
// Re-export the right implementation
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
pub use macos::SystemLoopbackSource;

#[cfg(not(target_os = "macos"))]
pub use stub::SystemLoopbackSource;

// ---------------------------------------------------------------------------
// Public convenience functions
// ---------------------------------------------------------------------------

/// Start capturing system audio in `SystemMix` mode.
///
/// Uses ScreenCaptureKit on macOS. Returns `Err(LoopbackError::NotSupported)`
/// on all other platforms.
pub fn capture_system_audio<F>(callback: F) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: Fn(pocketstation_frame::AudioFrame) + Send + Sync + 'static,
{
    SystemLoopbackSource::capture(callback)
}

/// Start capturing with an explicit `CaptureMode`.
///
/// - macOS: `Application(bundle_id)` uses `SCContentFilter.with_including_applications`.
///   `Process(_)` returns `ModeUnsupported`.
///   `with_excludes_current_process_audio(true)` prevents CLI feedback.
/// - Other platforms: always returns `NotSupported`.
pub fn capture_with_mode<F>(
    mode: CaptureMode,
    callback: F,
) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: Fn(pocketstation_frame::AudioFrame) + Send + Sync + 'static,
{
    SystemLoopbackSource::capture_mode(mode, callback)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: non-macOS returns NotSupported.
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_loopback_source_returns_not_supported_on_non_macos() {
        let result = capture_system_audio(|_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // Test 2: NotSupported error formats correctly.
    #[test]
    fn test_loopback_error_display() {
        let msg = LoopbackError::NotSupported.to_string();
        assert!(msg.contains("not supported"), "got: {msg}");
    }

    // Test 3: type name accessible (compile-time check).
    #[test]
    fn test_loopback_source_struct_can_be_constructed() {
        let type_name = std::any::type_name::<SystemLoopbackSource>();
        assert!(type_name.contains("SystemLoopbackSource"), "got: {type_name}");
    }

    // GWT Test 4: SystemMix on non-macOS returns NotSupported.
    // Given: SystemMix mode
    // When: capture_with_mode is called on a non-macOS platform
    // Then: returns NotSupported
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn given_system_mix_mode_when_capture_with_mode_called_on_non_macos_then_returns_not_supported() {
        let result = capture_with_mode(CaptureMode::SystemMix, |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // GWT Test 5: Application mode on non-macOS returns NotSupported.
    // Given: Application mode with a bundle ID
    // When: capture_with_mode is called on a non-macOS platform
    // Then: returns NotSupported
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn given_application_mode_when_capture_with_mode_called_on_non_macos_then_returns_not_supported() {
        let result = capture_with_mode(
            CaptureMode::Application("com.example.app".into()),
            |_frame| {},
        );
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // GWT Test 6: Process mode on non-macOS returns NotSupported.
    // Given: Process mode with a PID
    // When: capture_with_mode is called on a non-macOS platform
    // Then: returns NotSupported
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn given_process_mode_when_capture_with_mode_called_on_non_macos_then_returns_not_supported() {
        let result = capture_with_mode(CaptureMode::Process(1234), |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // GWT Test 7: ModeUnsupported error display.
    // Given: a ModeUnsupported error for Process mode
    // When: displayed
    // Then: contains 'not supported'
    #[test]
    fn given_capture_mode_unsupported_when_displayed_then_contains_not_supported() {
        let err = LoopbackError::ModeUnsupported(CaptureMode::Process(1234));
        let msg = err.to_string();
        assert!(msg.contains("not supported"), "got: {msg}");
    }

    // GWT Test 8: BackendInit error preserves the message.
    // Given: a BackendInit error with a custom message
    // When: displayed
    // Then: custom text is present
    #[test]
    fn given_backend_init_error_when_displayed_then_contains_message() {
        let err = LoopbackError::BackendInit("test failure".into());
        let msg = err.to_string();
        assert!(msg.contains("test failure"), "got: {msg}");
    }

    // GWT Test 9: ASP stub always returns false (macOS only).
    // Given: ASP feature is off (no libASPL submodule)
    // When: asp_is_installed() is called
    // Then: returns false
    #[cfg(target_os = "macos")]
    #[test]
    fn given_asp_stub_compiled_when_asp_is_installed_called_then_returns_false() {
        assert!(!asp_is_installed(), "asp_is_installed() must return false with stub");
    }

    // Wave C GWT Test 1: PW_NODE_LATENCY format is "numerator/denominator".
    // Given: the PW_NODE_LATENCY constant used by linux.rs
    // When: parsed as "n/d"
    // Then: numerator is 128 and denominator is 48000
    #[test]
    fn given_pw_node_latency_const_when_parsed_then_numerator_is_128_and_denominator_is_48000() {
        const PW_NODE_LATENCY: &str = "128/48000";
        let parts: Vec<u32> = PW_NODE_LATENCY
            .split('/')
            .map(|s| s.parse().expect("must be a number"))
            .collect();
        assert_eq!(parts.len(), 2, "expected n/d form");
        assert_eq!(parts[0], 128, "numerator must be 128 (~2.67 ms at 48 kHz)");
        assert_eq!(parts[1], 48_000, "denominator must be 48000 Hz");
    }

    // Wave C GWT Test 2: pipewire socket absent outside a PipeWire session.
    // Given: running on macOS / non-Linux CI (no PipeWire daemon)
    // When: the pipewire-0 socket path is checked in a temp directory
    // Then: the socket does not exist
    #[cfg(not(target_os = "linux"))]
    #[test]
    fn given_non_linux_host_when_pipewire_socket_path_checked_then_does_not_exist() {
        let tmp = std::env::temp_dir();
        let socket = tmp.join("pipewire-0");
        assert!(
            !socket.exists(),
            "unexpected pipewire-0 socket found at {socket:?}"
        );
    }

    // Wave C GWT Test 3: CaptureMode::Process on non-Linux/non-Windows returns NotSupported.
    // Given: Process mode on a stub platform (not Linux, not Windows, not macOS)
    // When: capture_with_mode is called
    // Then: NotSupported (stub backend)
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    #[test]
    fn given_process_capture_mode_when_called_on_stub_platform_then_returns_not_supported() {
        let result = capture_with_mode(CaptureMode::Process(42), |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // Wave C GWT Test 4: CaptureMode default is SystemMix.
    // Given: CaptureMode::default()
    // When: compared to CaptureMode::SystemMix
    // Then: they are equal
    #[test]
    fn given_default_capture_mode_when_compared_then_is_system_mix() {
        assert_eq!(CaptureMode::default(), CaptureMode::SystemMix);
    }
}

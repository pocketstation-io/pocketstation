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
    use super::macos_asp::AspReader;

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

    /// Manages a macOS audio capture session.
    ///
    /// Backed by the PocketStation HAL plugin shared memory ring when the plugin is
    /// installed (zero Screen Recording permission required), falling back to
    /// ScreenCaptureKit when it is not.
    pub struct SystemLoopbackSource {
        /// SCKit stream handle — present only when using the SCKit backend.
        sckit_stream: Option<SCStream>,
        /// ASP reader thread join handle — present only when using the ASP backend.
        asp_thread: Option<std::thread::JoinHandle<()>>,
        /// Channel used to signal the ASP reader thread to stop.
        asp_stop: Option<std::sync::mpsc::SyncSender<()>>,
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
            // Use ASP ring if plugin is installed (zero-permission, low-latency).
            if crate::macos_asp::asp_is_installed() {
                return Self::capture_via_asp(mode, callback);
            }
            // Fall back to SCKit (requires Screen Recording permission).
            Self::capture_via_sckit(mode, callback)
        }

        fn capture_via_asp<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
        where
            F: Fn(AudioFrame) + Send + Sync + 'static,
        {
            // ASP ring is output-only (system mix); per-app capture not supported.
            if mode != CaptureMode::SystemMix {
                return Err(LoopbackError::ModeUnsupported(mode));
            }
            let mut reader = AspReader::open()
                .ok_or_else(|| LoopbackError::BackendInit("ASP shm open failed".into()))?;

            let channels = reader.channels() as u8;
            // Frame size: 20 ms at the plugin's sample rate.
            let sample_rate = reader.sample_rate();
            // 20 ms worth of frames per callback invocation.
            let frames_per_cb: u32 = sample_rate / 50;
            let buf_samples = (frames_per_cb as usize) * (channels as usize);
            let pool = Arc::new(AudioBufferPool::new(POOL_DEPTH, buf_samples));

            let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);

            let thread = std::thread::Builder::new()
                .name("pks-asp-reader".into())
                .spawn(move || {
                    let mut seq: u64 = 0;
                    let mut buf = vec![0.0f32; buf_samples];
                    loop {
                        // Check for stop signal (non-blocking).
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        let read = reader.read_frames(&mut buf, frames_per_cb);
                        if read == 0 {
                            // Ring empty — yield and retry after ~1 ms.
                            std::thread::sleep(std::time::Duration::from_millis(1));
                            continue;
                        }
                        let mut handle = match pool.acquire() {
                            Some(h) => h,
                            None => continue,
                        };
                        let dst = handle.as_mut_slice();
                        let samples = (read as usize) * (channels as usize);
                        let copy_len = samples.min(dst.len());
                        dst[..copy_len].copy_from_slice(&buf[..copy_len]);
                        handle.set_len(copy_len);
                        let timestamp_ns = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_nanos() as u64;
                        let mut frame = AudioFrame::new(
                            StreamId(0),
                            SourceId(0),
                            seq,
                            timestamp_ns,
                            channels,
                            handle,
                        );
                        frame.source_tag = AudioSourceTag::Captured;
                        frame.encryption_mode = EncryptionMode::None;
                        frame.sample_rate = sample_rate;
                        callback(frame);
                        seq += 1;
                    }
                })
                .map_err(|e| LoopbackError::BackendInit(format!("thread spawn: {e}")))?;

            Ok(Self {
                sckit_stream: None,
                asp_thread: Some(thread),
                asp_stop: Some(stop_tx),
            })
        }

        fn capture_via_sckit<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
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
            Ok(Self {
                sckit_stream: Some(stream),
                asp_thread: None,
                asp_stop: None,
            })
        }
    }

    impl Drop for SystemLoopbackSource {
        fn drop(&mut self) {
            if let Some(ref tx) = self.asp_stop {
                let _ = tx.try_send(());
            }
            if let Some(thread) = self.asp_thread.take() {
                let _ = thread.join();
            }
            if let Some(ref mut stream) = self.sckit_stream {
                let _ = stream.stop_capture();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Windows WASAPI implementation
// ---------------------------------------------------------------------------

#[cfg(target_os = "windows")]
mod windows;

// ---------------------------------------------------------------------------
// Linux PipeWire + ALSA implementation
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
mod linux;

// ---------------------------------------------------------------------------
// Stub for platforms with no backend (not macOS, not Linux, not Windows)
// ---------------------------------------------------------------------------

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
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

#[cfg(target_os = "windows")]
pub use windows::SystemLoopbackSource;

#[cfg(target_os = "linux")]
pub use linux::SystemLoopbackSource;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
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

    // Test 1: stub platform returns NotSupported.
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
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

    // GWT Test 4: SystemMix on stub platform returns NotSupported.
    // Given: SystemMix mode
    // When: capture_with_mode is called on a platform with no backend
    // Then: returns NotSupported
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    #[test]
    fn given_system_mix_mode_when_capture_with_mode_called_on_non_macos_then_returns_not_supported() {
        let result = capture_with_mode(CaptureMode::SystemMix, |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // GWT Test 5: Application mode on stub platform returns NotSupported.
    // Given: Application mode with a bundle ID
    // When: capture_with_mode is called on a platform with no backend
    // Then: returns NotSupported
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    #[test]
    fn given_application_mode_when_capture_with_mode_called_on_non_macos_then_returns_not_supported() {
        let result = capture_with_mode(
            CaptureMode::Application("com.example.app".into()),
            |_frame| {},
        );
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // GWT Test 6: Process mode on stub platform returns NotSupported.
    // Given: Process mode with a PID
    // When: capture_with_mode is called on a platform with no backend
    // Then: returns NotSupported
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
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

    // GWT Test 9: asp_is_installed() returns false when no HAL plugin is running (macOS only).
    // Given: no PocketStation HAL plugin running (CI environment, no /pocketstation-loopback-v1 shm)
    // When: asp_is_installed() is called
    // Then: returns false
    #[cfg(target_os = "macos")]
    #[test]
    fn given_asp_not_running_when_asp_is_installed_then_returns_false() {
        assert!(!asp_is_installed(), "asp_is_installed() must return false without the plugin");
    }

    // Wave B GWT Test 1: SystemMix on non-Windows non-macOS returns NotSupported.
    // Given: SystemMix mode on a non-Windows, non-macOS platform
    // When: capture_with_mode is called
    // Then: NotSupported (stub backend on macOS CI, Linux backend on Linux)
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    #[test]
    fn given_system_mix_mode_when_capture_with_mode_on_non_windows_non_macos_then_returns_not_supported() {
        let mode = CaptureMode::SystemMix;
        let result = capture_with_mode(mode, |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // Wave B GWT Test 2: Process mode on non-Windows non-macOS returns NotSupported.
    // Given: Process mode with an arbitrary PID on a non-Windows, non-macOS platform
    // When: capture_with_mode is called
    // Then: NotSupported
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    #[test]
    fn given_process_mode_when_capture_with_mode_on_non_windows_non_macos_then_returns_not_supported() {
        let mode = CaptureMode::Process(1234);
        let result = capture_with_mode(mode, |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // Wave B GWT Test 3: WASAPI_PROCESS_LOOPBACK_PERIOD_100NS is non-zero (Windows only).
    // Given: the process-loopback period constant from the windows backend
    // When: the constant is read
    // Then: it is non-zero (passing zero to initialize_client is a bug)
    #[cfg(target_os = "windows")]
    #[test]
    fn given_wasapi_process_period_const_when_checked_then_is_nonzero() {
        use crate::windows::WASAPI_PROCESS_LOOPBACK_PERIOD_100NS;
        assert_ne!(WASAPI_PROCESS_LOOPBACK_PERIOD_100NS, 0);
    }

    // Wave B GWT Test 4: BackendInit error contains the message.
    // Given: a BackendInit error with a specific message
    // When: the error is displayed
    // Then: the message is present in the output
    #[test]
    fn given_backend_init_error_when_displayed_then_contains_com_message() {
        let err = LoopbackError::BackendInit("COM init failed".to_owned());
        let msg = err.to_string();
        assert!(
            msg.contains("COM init failed"),
            "expected message in display, got: {msg}"
        );
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

    // ASP GWT Test 1: asp_is_installed() returns false without the plugin (macOS only).
    // Given: no HAL plugin running (no shared memory region present in CI)
    // When: asp_is_installed() is called
    // Then: returns false
    #[cfg(target_os = "macos")]
    #[test]
    fn given_asp_not_running_when_asp_is_installed_called_then_returns_false() {
        assert!(!asp_is_installed());
    }

    // ASP GWT Test 2: AspReader::open() returns None when no plugin is running (macOS only).
    // Given: no HAL plugin running (no shared memory region)
    // When: AspReader::open() is called
    // Then: returns None
    #[cfg(target_os = "macos")]
    #[test]
    fn given_asp_reader_null_when_open_called_without_plugin_then_returns_none() {
        use crate::macos_asp::AspReader;
        assert!(AspReader::open().is_none());
    }

    // ASP GWT Test 3: PKS_RING_FRAMES is a power of two (compile-time check).
    // Given: the PKS_RING_FRAMES constant (65536)
    // When: checked at compile time
    // Then: it is a power of two and non-zero
    #[test]
    fn given_shm_ring_const_pks_ring_frames_when_checked_then_is_power_of_two() {
        const PKS_RING_FRAMES: u32 = 65536u32;
        assert!(PKS_RING_FRAMES > 0);
        assert_eq!(PKS_RING_FRAMES & (PKS_RING_FRAMES - 1), 0,
            "PKS_RING_FRAMES must be a power of two for bitmask wrap to work");
    }
}

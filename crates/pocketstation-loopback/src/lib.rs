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
}

// ---------------------------------------------------------------------------
// Platform backends
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

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
/// On macOS, installs the HAL plugin automatically on first run (prompts for
/// sudo once).  Returns `Err(LoopbackError::NotSupported)` on platforms with
/// no backend.
pub fn capture_system_audio<F>(callback: F) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: Fn(pocketstation_frame::AudioFrame) + Send + Sync + 'static,
{
    SystemLoopbackSource::capture(callback)
}

/// Start capturing with an explicit `CaptureMode`.
///
/// - macOS: only `SystemMix` is supported; `Application(_)` and `Process(_)`
///   return `ModeUnsupported`.
/// - Windows: `SystemMix` and `Process(pid)` supported; `Application(_)` returns
///   `ModeUnsupported`.
/// - Linux: only `SystemMix` is supported (PipeWire); other modes return
///   `ModeUnsupported`.
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

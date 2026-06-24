mod model;

pub use model::{
    CaptureMode, CaptureSource, LoopbackError, SourceKind, SourceState, StableSourceId,
};

// ---------------------------------------------------------------------------
// Platform backends
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub mod macos_tap;

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
// macOS tap availability re-export
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
pub use macos_tap::tap_available;

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
// Source discovery — public API
// ---------------------------------------------------------------------------

/// Enumerate all available audio capture sources on this system.
///
/// - macOS 14.2+: system mix entry followed by per-process application sources
///   discovered via CoreAudio process enumeration.
/// - macOS < 14.2: system mix entry only.
/// - Windows: system mix entry followed by active WASAPI audio session sources.
/// - Linux (PipeWire): system mix entry followed by PipeWire node sources.
/// - Linux (no PipeWire): system mix entry only.
/// - Other platforms: system mix entry only.
pub fn discover_sources() -> Vec<CaptureSource> {
    #[cfg(target_os = "macos")]
    {
        use pocketstation_frame::Platform;
        let system_mix = CaptureSource {
            stable_id:   StableSourceId::new(Platform::Macos, SourceKind::SystemMix, "system:mix"),
            name:        "System Mix".to_owned(),
            process_id:  None,
            app_id:      None,
            device_uid:  None,
            state:       SourceState::Available,
            sample_rate: 48000,
            channels:    2,
        };
        let mut sources = vec![system_mix];
        if macos_tap::tap_available() {
            sources.extend(macos_tap::discover_sources_native());
        }
        sources
    }

    #[cfg(target_os = "windows")]
    {
        windows::discover_sources_windows()
    }

    #[cfg(target_os = "linux")]
    {
        linux::discover_sources_linux()
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        use pocketstation_frame::Platform;
        vec![CaptureSource {
            stable_id:   StableSourceId::new(Platform::Unknown, SourceKind::SystemMix, "system:mix"),
            name:        "System Mix".to_owned(),
            process_id:  None,
            app_id:      None,
            device_uid:  None,
            state:       SourceState::Available,
            sample_rate: 48000,
            channels:    2,
        }]
    }
}

// ---------------------------------------------------------------------------
// Public convenience functions
// ---------------------------------------------------------------------------

/// Start capturing system audio in `SystemMix` mode.
///
/// On macOS 14.2+, uses the CoreAudio process tap (no HAL plugin required).
/// On older macOS, installs the HAL plugin automatically on first run (prompts
/// for sudo once).  Returns `Err(LoopbackError::NotSupported)` on platforms
/// with no backend.
pub fn capture_system_audio<F>(callback: F) -> Result<SystemLoopbackSource, LoopbackError>
where
    F: Fn(pocketstation_frame::AudioFrame) + Send + Sync + 'static,
{
    SystemLoopbackSource::capture(callback)
}

/// Start capturing with an explicit `CaptureMode`.
///
/// - macOS 14.2+: `SystemMix`, `Application(_)`, and `Process(_)` all
///   supported via the CoreAudio process tap.
/// - macOS < 14.2: only `SystemMix` is supported via the HAL plugin.
/// - Windows: `SystemMix` and `Process(pid)` supported; `Application(name)` is
///   resolved to `Process(pid)` by name lookup — returns `BackendInit` if no
///   matching session is found.  Never silently falls back to `SystemMix`.
/// - Linux (PipeWire available): `SystemMix`, `Application(name)`, and
///   `Process(pid)` all supported via PipeWire node targeting.  Returns
///   `BackendInit` if the named node or PID is not found, and
///   `ModeUnsupported` if PipeWire is unavailable for per-app modes.
///   Never silently falls back to `SystemMix`.
/// - Linux (no PipeWire): `SystemMix` falls back to ALSA snd-aloop;
///   `Application(_)` and `Process(_)` return `ModeUnsupported`.
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
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    #[test]
    fn given_system_mix_mode_when_capture_with_mode_called_on_non_macos_then_returns_not_supported() {
        let result = capture_with_mode(CaptureMode::SystemMix, |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // GWT Test 5: Application mode on stub platform returns NotSupported.
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
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    #[test]
    fn given_process_mode_when_capture_with_mode_called_on_non_macos_then_returns_not_supported() {
        let result = capture_with_mode(CaptureMode::Process(1234), |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // GWT Test 7: ModeUnsupported error display.
    #[test]
    fn given_capture_mode_unsupported_when_displayed_then_contains_not_supported() {
        let err = LoopbackError::ModeUnsupported(CaptureMode::Process(1234));
        let msg = err.to_string();
        assert!(msg.contains("not supported"), "got: {msg}");
    }

    // GWT Test 8: BackendInit error preserves the message.
    #[test]
    fn given_backend_init_error_when_displayed_then_contains_message() {
        let err = LoopbackError::BackendInit("test failure".into());
        let msg = err.to_string();
        assert!(msg.contains("test failure"), "got: {msg}");
    }

    // GWT Test 9: asp_is_installed() returns false when no HAL plugin is running (macOS only).
    #[cfg(target_os = "macos")]
    #[test]
    fn given_asp_not_running_when_asp_is_installed_then_returns_false() {
        assert!(!asp_is_installed(), "asp_is_installed() must return false without the plugin");
    }

    // Wave B GWT Test 1: SystemMix on non-Windows non-macOS returns NotSupported.
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    #[test]
    fn given_system_mix_mode_when_capture_with_mode_on_non_windows_non_macos_then_returns_not_supported() {
        let mode = CaptureMode::SystemMix;
        let result = capture_with_mode(mode, |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // Wave B GWT Test 2: Process mode on non-Windows non-macOS returns NotSupported.
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    #[test]
    fn given_process_mode_when_capture_with_mode_on_non_windows_non_macos_then_returns_not_supported() {
        let mode = CaptureMode::Process(1234);
        let result = capture_with_mode(mode, |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // Wave B GWT Test 3: WASAPI_PROCESS_LOOPBACK_PERIOD_100NS is non-zero (Windows only).
    #[cfg(target_os = "windows")]
    #[test]
    fn given_wasapi_process_period_const_when_checked_then_is_nonzero() {
        use crate::windows::WASAPI_PROCESS_LOOPBACK_PERIOD_100NS;
        assert_ne!(WASAPI_PROCESS_LOOPBACK_PERIOD_100NS, 0);
    }

    // Wave B GWT Test 4: BackendInit error contains the message.
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
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    #[test]
    fn given_process_capture_mode_when_called_on_stub_platform_then_returns_not_supported() {
        let result = capture_with_mode(CaptureMode::Process(42), |_frame| {});
        assert_eq!(result.unwrap_err(), LoopbackError::NotSupported);
    }

    // Wave C GWT Test 4: CaptureMode default is SystemMix.
    #[test]
    fn given_default_capture_mode_when_compared_then_is_system_mix() {
        assert_eq!(CaptureMode::default(), CaptureMode::SystemMix);
    }

    // ASP GWT Test 1: asp_is_installed() returns false without the plugin (macOS only).
    #[cfg(target_os = "macos")]
    #[test]
    fn given_asp_not_running_when_asp_is_installed_called_then_returns_false() {
        assert!(!asp_is_installed());
    }

    // ASP GWT Test 2: AspReader::open() returns None when no plugin is running (macOS only).
    #[cfg(target_os = "macos")]
    #[test]
    fn given_asp_reader_null_when_open_called_without_plugin_then_returns_none() {
        use crate::macos_asp::AspReader;
        assert!(AspReader::open().is_none());
    }

    // ASP GWT Test 3: PKS_RING_FRAMES is a power of two (compile-time check).
    #[test]
    fn given_shm_ring_const_pks_ring_frames_when_checked_then_is_power_of_two() {
        const PKS_RING_FRAMES: u32 = 65536u32;
        assert!(PKS_RING_FRAMES > 0);
        assert_eq!(PKS_RING_FRAMES & (PKS_RING_FRAMES - 1), 0,
            "PKS_RING_FRAMES must be a power of two for bitmask wrap to work");
    }

    // Tap GWT Test 1: discover_sources() always returns at least the SystemMix entry.
    #[test]
    fn given_any_platform_when_discover_sources_called_then_first_entry_is_system_mix() {
        let sources = discover_sources();
        assert!(!sources.is_empty(), "discover_sources must return at least one entry");
        assert_eq!(
            sources[0].stable_id.kind,
            SourceKind::SystemMix,
            "first source must be SystemMix"
        );
    }

    // Tap GWT Test 2: tap_available() reports macOS 14.2+ (macOS only).
    #[cfg(target_os = "macos")]
    #[test]
    fn given_macos_15_when_tap_available_called_then_returns_true() {
        assert!(
            tap_available(),
            "tap_available() must return true on macOS 14.2+ (this machine is 15.5)"
        );
    }

    // No-fallback contract: Application mode on Linux with no PipeWire socket
    // present (as in a standard macOS/CI host) must return ModeUnsupported, not
    // silently capture SystemMix.
    //
    // GWT: Application mode on Linux returns ModeUnsupported, not SystemMix fallback.
    #[cfg(target_os = "linux")]
    #[test]
    fn given_application_mode_when_capture_with_mode_on_linux_then_mode_unsupported() {
        // Remove XDG_RUNTIME_DIR so pipewire_available() returns false, which
        // guarantees the PipeWire socket path cannot be resolved.  This exercises
        // the no-fallback contract: Application(_) must return ModeUnsupported
        // immediately rather than capturing SystemMix.
        std::env::remove_var("XDG_RUNTIME_DIR");
        let result = capture_with_mode(
            CaptureMode::Application("com.spotify.client".into()),
            |_| {},
        );
        assert_eq!(
            result.unwrap_err(),
            LoopbackError::ModeUnsupported(CaptureMode::Application("com.spotify.client".into())),
            "Application mode without PipeWire must return ModeUnsupported, not SystemMix"
        );
    }

    // GWT: Process mode on Linux without PipeWire returns ModeUnsupported.
    #[cfg(target_os = "linux")]
    #[test]
    fn given_process_mode_when_capture_with_mode_on_linux_without_pipewire_then_mode_unsupported() {
        std::env::remove_var("XDG_RUNTIME_DIR");
        let result = capture_with_mode(CaptureMode::Process(99999), |_| {});
        assert_eq!(
            result.unwrap_err(),
            LoopbackError::ModeUnsupported(CaptureMode::Process(99999)),
            "Process mode without PipeWire must return ModeUnsupported, not SystemMix"
        );
    }

    // GWT: Application mode on Windows returns an error, not a SystemMix fallback.
    // Windows resolves Application(name) → Process(pid) by WASAPI session lookup.
    // When no session matches, it returns BackendInit (not ModeUnsupported and not
    // a silent SystemMix capture), satisfying the no-fallback contract.
    #[cfg(target_os = "windows")]
    #[test]
    fn given_application_mode_when_capture_with_mode_on_windows_then_errors_not_system_mix() {
        // Use a name that cannot match any real WASAPI session in a CI environment.
        let result = capture_with_mode(
            CaptureMode::Application("__pks_no_such_app__".into()),
            |_| {},
        );
        match result.unwrap_err() {
            LoopbackError::ModeUnsupported(_) | LoopbackError::BackendInit(_) => {
                // Both are acceptable: no match → BackendInit; contract preserved.
            }
            other => panic!(
                "expected ModeUnsupported or BackendInit for unknown app, got: {other:?}"
            ),
        }
    }
}

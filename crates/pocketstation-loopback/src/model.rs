use pocketstation_frame::Platform;

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
// Source kind
// ---------------------------------------------------------------------------

/// Classifies the type of audio capture source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    /// An application-level audio source (identified by bundle ID, node name, etc.).
    Application,
    /// A hardware or virtual audio output device.
    OutputDevice,
    /// A hardware or virtual audio input device.
    InputDevice,
    /// The system-wide audio mix (all output combined).
    SystemMix,
}

// ---------------------------------------------------------------------------
// Source state
// ---------------------------------------------------------------------------

/// Playback state of a discoverable capture source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceState {
    /// The source is actively producing audio.
    Active,
    /// The source exists but is currently silent or paused.
    Idle,
}

// ---------------------------------------------------------------------------
// Stable source identity
// ---------------------------------------------------------------------------

/// A stable, typed identity for a discoverable capture source.
///
/// Unlike a raw u64 (PID / CoreAudio object ID / PipeWire node ID), the stable_key
/// is constructed from information that survives process restarts and device
/// reconnects where possible.
///
/// Examples of stable_key values:
///   "com.spotify.client"      (macOS app by bundle ID)
///   "device:<CoreAudio UID>"  (macOS audio device)
///   "pid:1234"                (fallback — PID, not stable across restarts)
///   "pw-node:<node_serial>"   (Linux PipeWire by serial, more stable than node ID)
///   "wasapi:<device-id>"      (Windows WASAPI device ID)
///   "system:mix"              (system-wide mix, always stable)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StableSourceId {
    pub platform: Platform,
    pub kind: SourceKind,
    pub stable_key: String,
}

impl StableSourceId {
    pub fn new(platform: Platform, kind: SourceKind, stable_key: impl Into<String>) -> Self {
        Self {
            platform,
            kind,
            stable_key: stable_key.into(),
        }
    }

    /// Derive a deterministic u64 for use as a frame-level `SourceId`.
    /// Stable across calls for the same (platform, kind, stable_key) triple.
    pub fn to_frame_source_id(&self) -> pocketstation_frame::SourceId {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        pocketstation_frame::SourceId(h.finish())
    }
}

// ---------------------------------------------------------------------------
// Capture source
// ---------------------------------------------------------------------------

/// A discoverable audio capture source returned by the discovery layer.
pub struct CaptureSource {
    /// Typed stable identity. Use `stable_id.to_frame_source_id()` when
    /// constructing AudioFrame for this source.
    pub stable_id: StableSourceId,
    /// Human-readable display name (app name, device name, etc.).
    pub name: String,
    /// OS process ID of the application (None for device/system sources).
    pub process_id: Option<u32>,
    /// Current playback state.
    pub state: SourceState,
    /// Native sample rate reported by the source (Hz).
    pub sample_rate: u32,
    /// Number of audio channels.
    pub channels: u16,
}

impl CaptureSource {
    /// Convenience: frame-level SourceId derived from this source's stable identity.
    pub fn frame_source_id(&self) -> pocketstation_frame::SourceId {
        self.stable_id.to_frame_source_id()
    }
}

// ---------------------------------------------------------------------------
// Public error type
// ---------------------------------------------------------------------------

/// Errors produced by the loopback capture API.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
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
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::Platform;

    // Given: a StableSourceId for a macOS app
    // When: to_frame_source_id() is called twice
    // Then: both calls return the same value (deterministic hash)
    #[test]
    fn given_stable_source_id_when_hashed_twice_then_same_frame_source_id() {
        let id = StableSourceId::new(Platform::Macos, SourceKind::Application, "com.spotify.client");
        let a = id.to_frame_source_id();
        let b = id.to_frame_source_id();
        assert_eq!(a, b);
    }

    // Given: two StableSourceIds with different keys
    // When: to_frame_source_id() is called on each
    // Then: the results differ
    #[test]
    fn given_two_different_stable_ids_when_hashed_then_different_frame_source_ids() {
        let a = StableSourceId::new(Platform::Macos, SourceKind::Application, "com.spotify.client");
        let b = StableSourceId::new(Platform::Macos, SourceKind::Application, "com.apple.music");
        assert_ne!(a.to_frame_source_id(), b.to_frame_source_id());
    }

    // Given: a StableSourceId for system mix
    // When: stable_key is inspected
    // Then: it equals "system:mix"
    #[test]
    fn given_system_mix_stable_id_when_key_checked_then_is_system_mix() {
        let id = StableSourceId::new(Platform::Linux, SourceKind::SystemMix, "system:mix");
        assert_eq!(id.stable_key, "system:mix");
        assert_eq!(id.kind, SourceKind::SystemMix);
        assert_eq!(id.platform, Platform::Linux);
    }

    // Given: a CaptureSource wrapping a StableSourceId
    // When: frame_source_id() is called
    // Then: the result matches to_frame_source_id() on the stable_id
    #[test]
    fn given_capture_source_when_frame_source_id_called_then_matches_stable_id_hash() {
        let stable_id = StableSourceId::new(Platform::Windows, SourceKind::OutputDevice, "wasapi:dev-001");
        let expected = stable_id.to_frame_source_id();
        let source = CaptureSource {
            stable_id,
            name: "Speakers".into(),
            process_id: None,
            state: SourceState::Active,
            sample_rate: 48_000,
            channels: 2,
        };
        assert_eq!(source.frame_source_id(), expected);
    }

    // Given: CaptureMode default
    // When: compared to CaptureMode::SystemMix
    // Then: they are equal
    #[test]
    fn given_default_capture_mode_when_compared_then_is_system_mix() {
        assert_eq!(CaptureMode::default(), CaptureMode::SystemMix);
    }

    // Given: LoopbackError::NotSupported
    // When: displayed
    // Then: contains "not supported"
    #[test]
    fn given_not_supported_error_when_displayed_then_contains_not_supported() {
        let msg = LoopbackError::NotSupported.to_string();
        assert!(msg.contains("not supported"), "got: {msg}");
    }
}

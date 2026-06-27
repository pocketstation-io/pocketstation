use pocketstation_frame::{AudioFrame, Platform};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum CaptureMode {
    #[default]
    SystemMix,
    Application(String),
    Process(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Application,
    OutputDevice,
    InputDevice,
    SystemMix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceState {
    Available,
    Playing,
    Silent,
    Unavailable,
    PermissionBlocked,
}

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

    pub fn to_frame_source_id(&self) -> pocketstation_frame::SourceId {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        pocketstation_frame::SourceId(h.finish())
    }
}

#[derive(Debug, Clone)]
pub struct CaptureSource {
    pub stable_id: StableSourceId,
    pub name: String,
    pub process_id: Option<u32>,
    pub app_id: Option<String>,
    pub device_uid: Option<String>,
    pub state: SourceState,
    pub sample_rate_hz: u32,
    pub channels: u16,
}

impl CaptureSource {
    pub fn frame_source_id(&self) -> pocketstation_frame::SourceId {
        self.stable_id.to_frame_source_id()
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CaptureError {
    #[error("system audio loopback is not supported on this platform")]
    NotSupported,
    #[error("loopback backend error: {0}")]
    BackendInit(String),
    #[error("capture mode not supported on this backend: {0:?}")]
    ModeUnsupported(CaptureMode),
}

// Backwards-compat alias — existing callers that use LoopbackError still compile.
pub use CaptureError as LoopbackError;

pub trait AudioSourceStream: Send {
    fn sample_rate_hz(&self) -> u32;
    fn channel_count(&self) -> u8;
    fn read_frame(&mut self) -> Result<AudioFrame, CaptureError>;
}

pub trait AudioOutputSink: Send {
    fn write_frame(&mut self, frame: AudioFrame) -> Result<(), CaptureError>;
    fn flush(&mut self) -> Result<(), CaptureError>;
}

#[derive(Debug)]
pub enum AdapterError {
    Unavailable,
    PermissionDenied,
    Unsupported,
    Io(String),
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterError::Unavailable => write!(f, "adapter: source or output is unavailable"),
            AdapterError::PermissionDenied => write!(f, "adapter: permission denied"),
            AdapterError::Unsupported => {
                write!(f, "adapter: operation not supported on this platform")
            }
            AdapterError::Io(msg) => write!(f, "adapter I/O error: {msg}"),
        }
    }
}

impl std::error::Error for AdapterError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceCapability {
    Microphone,
    OwnAppAudio,
    DesktopSystemLoopback,
    EligibleAppPlayback,
    ScreenProjectionMix,
    PluginHostAudio,
    BroadcastExtensionAudio,
    ExternalRouteInput,
    VirtualDeviceInput,
    NetworkStreamInput,
    FileOrBuffer,
    HardwareInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformId {
    Ios,
    Android,
    Windows,
    Macos,
    Linux,
    Web,
    Server,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LatencyClass {
    Realtime,
    LowLatency,
    Buffered,
}

impl LatencyClass {
    pub fn rank(self) -> u8 {
        match self {
            Self::Realtime => 0,
            Self::LowLatency => 1,
            Self::Buffered => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReliabilityClass {
    AlwaysAvailable,
    UserPermission,
    UserAction,
    Experimental,
    PolicyGated,
    FutureAPI,
}

impl ReliabilityClass {
    pub fn rank(self) -> u8 {
        match self {
            Self::AlwaysAvailable => 0,
            Self::UserPermission => 1,
            Self::UserAction => 2,
            Self::PolicyGated => 3,
            Self::Experimental => 4,
            Self::FutureAPI => 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioSourceDescriptor {
    pub id: pocketstation_frame::SourceId,
    pub name: String,
    pub platform: PlatformId,
    pub capability: SourceCapability,
    pub latency_class: LatencyClass,
    pub reliability_class: ReliabilityClass,
    pub requires_user_action: bool,
    pub available_now: bool,
    pub policy_notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AudioOutputDescriptor {
    pub id: pocketstation_frame::SourceId,
    pub name: String,
    pub platform: PlatformId,
    pub target: OutputTarget,
    pub latency_class: LatencyClass,
    pub available_now: bool,
}

pub struct SourceRequest {
    pub capability: SourceCapability,
    pub preferred_latency: LatencyClass,
}

pub struct OutputRequest {
    pub target: OutputTarget,
}

#[derive(Debug, Clone)]
pub enum OutputTarget {
    LocalSpeaker,
    BluetoothDevice,
    WebListener,
    MobileApp,
    DesktopApp,
    VoiceAgentBackend,
    RecordingFile,
    VirtualMicrophone,
    PublicRoom,
    PrivateRoom,
    HardwareAccessory,
}

pub enum SourcePreference {
    Voice,
    Music,
    Broadcast,
}

pub trait PlatformAdapter: Send + Sync {
    fn platform(&self) -> PlatformId;
    fn source_capabilities(&self) -> Vec<AudioSourceDescriptor>;
    fn output_capabilities(&self) -> Vec<AudioOutputDescriptor>;
    fn open_source(
        &self,
        request: SourceRequest,
    ) -> Result<Box<dyn AudioSourceStream>, AdapterError>;
    fn open_output(&self, request: OutputRequest)
        -> Result<Box<dyn AudioOutputSink>, AdapterError>;
}

fn is_preferred(cap: &SourceCapability, preference: &SourcePreference) -> bool {
    match preference {
        SourcePreference::Voice => matches!(
            cap,
            SourceCapability::Microphone | SourceCapability::OwnAppAudio
        ),
        SourcePreference::Music => matches!(
            cap,
            SourceCapability::OwnAppAudio | SourceCapability::DesktopSystemLoopback
        ),
        SourcePreference::Broadcast => true,
    }
}

pub fn open_best_source(
    adapter: &dyn PlatformAdapter,
    preference: SourcePreference,
) -> Result<Box<dyn AudioSourceStream>, AdapterError> {
    let mut candidates: Vec<AudioSourceDescriptor> = adapter
        .source_capabilities()
        .into_iter()
        .filter(|d| d.available_now)
        .collect();
    if candidates.is_empty() {
        return Err(AdapterError::Unavailable);
    }
    candidates.sort_by_key(|d| {
        let pref_rank: u8 = if is_preferred(&d.capability, &preference) {
            0
        } else {
            1
        };
        (
            pref_rank,
            d.latency_class.clone().rank(),
            d.reliability_class.clone().rank(),
        )
    });
    let best = candidates.remove(0);
    adapter.open_source(SourceRequest {
        capability: best.capability,
        preferred_latency: best.latency_class,
    })
}

pub struct SystemLoopbackSource;

impl SystemLoopbackSource {
    pub fn capture<F>(_cb: F) -> Result<Self, CaptureError>
    where
        F: Fn(pocketstation_frame::AudioFrame) + Send + Sync + 'static,
    {
        Err(CaptureError::NotSupported)
    }
    pub fn capture_mode<F>(_mode: CaptureMode, _cb: F) -> Result<Self, CaptureError>
    where
        F: Fn(pocketstation_frame::AudioFrame) + Send + Sync + 'static,
    {
        Err(CaptureError::NotSupported)
    }
}

pub fn discover_sources() -> Vec<CaptureSource> {
    #[cfg(target_os = "macos")]
    let platform = Platform::Macos;
    #[cfg(target_os = "windows")]
    let platform = Platform::Windows;
    #[cfg(target_os = "linux")]
    let platform = Platform::Linux;
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    let platform = Platform::Unknown;

    vec![CaptureSource {
        stable_id: StableSourceId::new(platform, SourceKind::SystemMix, "system:mix"),
        name: "System Mix".to_owned(),
        process_id: None,
        app_id: None,
        device_uid: None,
        state: SourceState::Available,
        sample_rate_hz: 48_000,
        channels: 2,
    }]
}

pub fn capture_system_audio<F>(callback: F) -> Result<SystemLoopbackSource, CaptureError>
where
    F: Fn(pocketstation_frame::AudioFrame) + Send + Sync + 'static,
{
    SystemLoopbackSource::capture(callback)
}

pub fn capture_with_mode<F>(
    mode: CaptureMode,
    callback: F,
) -> Result<SystemLoopbackSource, CaptureError>
where
    F: Fn(pocketstation_frame::AudioFrame) + Send + Sync + 'static,
{
    SystemLoopbackSource::capture_mode(mode, callback)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::Platform;

    #[test]
    fn given_stable_source_id_when_hashed_twice_then_same_frame_source_id() {
        let id = StableSourceId::new(
            Platform::Macos,
            SourceKind::Application,
            "com.spotify.client",
        );
        assert_eq!(id.to_frame_source_id(), id.to_frame_source_id());
    }

    #[test]
    fn given_two_different_stable_ids_when_hashed_then_different_frame_source_ids() {
        let a = StableSourceId::new(
            Platform::Macos,
            SourceKind::Application,
            "com.spotify.client",
        );
        let b = StableSourceId::new(Platform::Macos, SourceKind::Application, "com.apple.music");
        assert_ne!(a.to_frame_source_id(), b.to_frame_source_id());
    }

    #[test]
    fn given_any_platform_when_discover_sources_called_then_first_entry_is_system_mix() {
        let sources = discover_sources();
        assert!(!sources.is_empty());
        assert_eq!(sources[0].stable_id.kind, SourceKind::SystemMix);
    }

    #[test]
    fn given_capture_error_not_supported_when_displayed_then_contains_not_supported() {
        let msg = CaptureError::NotSupported.to_string();
        assert!(msg.contains("not supported"), "got: {msg}");
    }

    #[test]
    fn given_loopback_error_alias_when_used_then_compiles_and_equals() {
        let _e: LoopbackError = CaptureError::NotSupported;
    }

    #[test]
    fn given_default_capture_mode_when_compared_then_is_system_mix() {
        assert_eq!(CaptureMode::default(), CaptureMode::SystemMix);
    }

    #[test]
    fn given_stub_capture_when_called_then_returns_not_supported() {
        let result = capture_system_audio(|_| {});
        assert!(matches!(result, Err(CaptureError::NotSupported)));
    }

    #[test]
    fn given_mode_unsupported_error_when_displayed_then_contains_not_supported() {
        let err = CaptureError::ModeUnsupported(CaptureMode::Process(1234));
        assert!(err.to_string().contains("not supported"));
    }

    #[test]
    fn latency_class_rank_order_is_realtime_low_buffered() {
        assert!(LatencyClass::Realtime.rank() < LatencyClass::LowLatency.rank());
        assert!(LatencyClass::LowLatency.rank() < LatencyClass::Buffered.rank());
    }

    #[test]
    fn reliability_class_rank_always_available_is_lowest() {
        assert_eq!(ReliabilityClass::AlwaysAvailable.rank(), 0);
        assert!(ReliabilityClass::AlwaysAvailable.rank() < ReliabilityClass::FutureAPI.rank());
    }
}

use pocketstation_frame::SourceId;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReliabilityClass {
    AlwaysAvailable,
    UserPermission,
    UserAction,
    Experimental,
    PolicyGated,
    FutureAPI,
}

#[derive(Debug, Clone)]
pub struct AudioSourceDescriptor {
    pub id: SourceId,
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
pub enum RouteKind {
    LocalPlayback,
    LanDirect,
    CloudRelay,
    VoiceAgentBackend,
    RecordingFile,
    VirtualMicrophone,
    PeerToPeer,
    HardwareBridge,
    PublicChannel,
    PrivateRoom,
}
#[derive(Debug, Clone)]
pub enum TransportKind {
    Local,
    WebRtc,
    RtpUdp,
    File,
}
#[derive(Debug, Clone)]
// Renamed from EncryptionMode to avoid ambiguous glob re-export with
// pocketstation_frame::EncryptionMode in pocketstation-audio (Phase 5).
pub enum RouteEncryptionMode {
    TransportOnly,
    SFrameE2EE,
    EnterpriseKeyManager,
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

#[derive(Debug, Clone)]
pub struct RoutePlan {
    pub source: SourceId,
    pub outputs: Vec<OutputTarget>,
    pub transport: TransportKind,
    pub encryption: RouteEncryptionMode,
    pub latency_budget_ms: u32,
    pub fallback_routes: Vec<RouteKind>,
}

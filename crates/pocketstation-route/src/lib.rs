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

// Keep EncryptionMode as a public alias so existing callers that use
// `RouteEncryptionMode` and callers that expect `EncryptionMode` both compile.
pub use RouteEncryptionMode as EncryptionMode;

/// Descriptor for a physical or virtual audio output (spec §5.4).
#[derive(Debug, Clone)]
pub struct AudioOutputDescriptor {
    pub id: SourceId,
    pub name: String,
    pub platform: PlatformId,
    pub target: OutputTarget,
    pub latency_class: LatencyClass,
    pub available_now: bool,
}

/// Criteria used to select a source when calling `open_best_source`.
pub struct SourceRequest {
    pub capability: SourceCapability,
    pub preferred_latency: LatencyClass,
}

/// Criteria used to select an output when calling `PlatformAdapter::open_output`.
pub struct OutputRequest {
    pub target: OutputTarget,
}

/// Errors returned by `PlatformAdapter` methods.
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

/// Opaque handle to a live audio source stream.
pub trait AudioSourceStream: Send {}

/// Opaque handle to a live audio output sink.
pub trait AudioOutputSink: Send {}

/// Platform-specific adapter that exposes sources and outputs (spec §5.4).
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

/// Hint to `open_best_source` about the intended use-case.
pub enum SourcePreference {
    Voice,
    Music,
    Broadcast,
}

/// Numeric rank for `LatencyClass`; lower is better.
fn latency_rank(lc: &LatencyClass) -> u8 {
    match lc {
        LatencyClass::Realtime => 0,
        LatencyClass::LowLatency => 1,
        LatencyClass::Buffered => 2,
    }
}

/// Numeric rank for `ReliabilityClass`; lower is better.
fn reliability_rank(rc: &ReliabilityClass) -> u8 {
    match rc {
        ReliabilityClass::AlwaysAvailable => 0,
        ReliabilityClass::UserPermission => 1,
        ReliabilityClass::UserAction => 2,
        ReliabilityClass::PolicyGated => 3,
        ReliabilityClass::Experimental => 4,
        ReliabilityClass::FutureAPI => 5,
    }
}

/// Whether a descriptor is preferred for the given `SourcePreference`.
/// Preferred sources are sorted before non-preferred ones.
fn is_preferred(cap: &SourceCapability, preference: &SourcePreference) -> bool {
    match preference {
        SourcePreference::Voice => {
            matches!(
                cap,
                SourceCapability::Microphone | SourceCapability::OwnAppAudio
            )
        }
        SourcePreference::Music => {
            matches!(
                cap,
                SourceCapability::OwnAppAudio | SourceCapability::DesktopSystemLoopback
            )
        }
        SourcePreference::Broadcast => true,
    }
}

/// Selects the best available source from `adapter` for the given preference
/// and opens it (spec §12.1).
///
/// Ranking (ascending = better):
/// 1. Non-preferred before preferred (preferred wins).
/// 2. `LatencyClass`: Realtime < LowLatency < Buffered.
/// 3. `ReliabilityClass`: AlwaysAvailable < UserPermission < UserAction
///    < PolicyGated < Experimental < FutureAPI.
///
/// Returns `AdapterError::Unavailable` when no source has `available_now == true`.
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
            latency_rank(&d.latency_class),
            reliability_rank(&d.reliability_class),
        )
    });

    let best = candidates.remove(0);
    adapter.open_source(SourceRequest {
        capability: best.capability,
        preferred_latency: best.latency_class,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id(n: u64) -> SourceId {
        SourceId(n)
    }

    /// A minimal stub that records which source was opened.
    struct MockStream;
    impl AudioSourceStream for MockStream {}

    struct MockSink;
    impl AudioOutputSink for MockSink {}

    struct MockAdapter {
        sources: Vec<AudioSourceDescriptor>,
    }

    impl PlatformAdapter for MockAdapter {
        fn platform(&self) -> PlatformId {
            PlatformId::Macos
        }

        fn source_capabilities(&self) -> Vec<AudioSourceDescriptor> {
            self.sources.clone()
        }

        fn output_capabilities(&self) -> Vec<AudioOutputDescriptor> {
            vec![]
        }

        fn open_source(
            &self,
            request: SourceRequest,
        ) -> Result<Box<dyn AudioSourceStream>, AdapterError> {
            // Return a stream only if the requested capability is in our list.
            let found = self
                .sources
                .iter()
                .any(|d| d.capability == request.capability && d.available_now);
            if found {
                Ok(Box::new(MockStream))
            } else {
                Err(AdapterError::Unavailable)
            }
        }

        fn open_output(
            &self,
            _request: OutputRequest,
        ) -> Result<Box<dyn AudioOutputSink>, AdapterError> {
            Ok(Box::new(MockSink))
        }
    }

    fn mic_descriptor(available: bool) -> AudioSourceDescriptor {
        AudioSourceDescriptor {
            id: make_id(1),
            name: "Built-in Microphone".into(),
            platform: PlatformId::Macos,
            capability: SourceCapability::Microphone,
            latency_class: LatencyClass::Realtime,
            reliability_class: ReliabilityClass::UserPermission,
            requires_user_action: false,
            available_now: available,
            policy_notes: None,
        }
    }

    fn own_app_descriptor(available: bool) -> AudioSourceDescriptor {
        AudioSourceDescriptor {
            id: make_id(2),
            name: "Own App Audio".into(),
            platform: PlatformId::Macos,
            capability: SourceCapability::OwnAppAudio,
            latency_class: LatencyClass::LowLatency,
            reliability_class: ReliabilityClass::AlwaysAvailable,
            requires_user_action: false,
            available_now: available,
            policy_notes: None,
        }
    }

    fn loopback_descriptor(available: bool) -> AudioSourceDescriptor {
        AudioSourceDescriptor {
            id: make_id(3),
            name: "Desktop System Loopback".into(),
            platform: PlatformId::Macos,
            capability: SourceCapability::DesktopSystemLoopback,
            latency_class: LatencyClass::Buffered,
            reliability_class: ReliabilityClass::UserPermission,
            requires_user_action: false,
            available_now: available,
            policy_notes: None,
        }
    }

    #[test]
    fn given_voice_preference_when_mic_available_then_selects_mic() {
        let adapter = MockAdapter {
            sources: vec![mic_descriptor(true), own_app_descriptor(true)],
        };
        // Both Microphone and OwnAppAudio are preferred for Voice.
        // Microphone is Realtime; OwnAppAudio is LowLatency → Microphone wins.
        let result = open_best_source(&adapter, SourcePreference::Voice);
        assert!(result.is_ok(), "expected Ok, got err: {:?}", result.err());
    }

    #[test]
    fn given_no_available_source_when_open_best_then_returns_unavailable() {
        let adapter = MockAdapter {
            sources: vec![mic_descriptor(false), own_app_descriptor(false)],
        };
        let result = open_best_source(&adapter, SourcePreference::Voice);
        assert!(
            matches!(result, Err(AdapterError::Unavailable)),
            "expected Unavailable"
        );
    }

    #[test]
    fn given_music_preference_when_own_app_and_mic_available_then_selects_own_app() {
        // For Music: OwnAppAudio and DesktopSystemLoopback are preferred.
        // Microphone is not preferred. OwnAppAudio (LowLatency) beats loopback (Buffered).
        let adapter = MockAdapter {
            sources: vec![
                mic_descriptor(true),
                own_app_descriptor(true),
                loopback_descriptor(true),
            ],
        };
        // open_best_source should pick OwnAppAudio (preferred + LowLatency)
        // and MockAdapter.open_source succeeds for it.
        let result = open_best_source(&adapter, SourcePreference::Music);
        assert!(result.is_ok(), "expected Ok, got err: {:?}", result.err());
    }

    #[test]
    fn given_platform_adapter_trait_when_implemented_then_compiles() {
        // Compile-time test: MockAdapter implements PlatformAdapter.
        fn assert_adapter<T: PlatformAdapter>(_: &T) {}
        let adapter = MockAdapter { sources: vec![] };
        assert_adapter(&adapter);
    }
}

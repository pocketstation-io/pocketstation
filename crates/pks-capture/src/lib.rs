use pks_frame::{AudioFrame, Platform};
use serde::Serialize;
use std::num::NonZeroU32;

mod frame_stream;

pub use frame_stream::{
    captured_frame_stream, CapturedFrameDelivery, CapturedFrameSender, CapturedFrameStream,
    CapturedFrameStreamStats,
};

/// Process-wide monotonic timestamp domain used by every capture adapter.
/// The value is non-zero and comparable across PocketStation crates in the
/// same process; it is never derived from a wall clock and cannot jump.
pub fn monotonic_timestamp_ns() -> u64 {
    pks_timing::monotonic_timestamp_ns()
}

/// Source-time clock for capture streams whose media cadence is defined by
/// the number of sample frames produced by the device.
///
/// Callback arrival time is scheduler time, not audio presentation time. This
/// clock anchors the first observed sample frame to the process monotonic clock
/// and advances only by represented sample count. Callers must advance it even
/// when a captured buffer is dropped so downstream gaps remain observable.
#[derive(Debug)]
pub struct CaptureSampleTimeline {
    sample_rate_hz: NonZeroU32,
    origin_timestamp_ns: Option<u64>,
    elapsed_sample_frames: u64,
}

impl CaptureSampleTimeline {
    pub fn new(sample_rate_hz: NonZeroU32) -> Self {
        Self {
            sample_rate_hz,
            origin_timestamp_ns: None,
            elapsed_sample_frames: 0,
        }
    }

    /// Returns this buffer's source-time start and advances the next start.
    pub fn advance(&mut self, sample_frames: u64) -> u64 {
        let origin_timestamp_ns = *self
            .origin_timestamp_ns
            .get_or_insert_with(monotonic_timestamp_ns);
        let elapsed_ns = u128::from(self.elapsed_sample_frames)
            .saturating_mul(1_000_000_000)
            .checked_div(u128::from(self.sample_rate_hz.get()))
            .unwrap_or(0)
            .min(u128::from(u64::MAX)) as u64;
        self.elapsed_sample_frames = self.elapsed_sample_frames.saturating_add(sample_frames);
        origin_timestamp_ns.saturating_add(elapsed_ns)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum InputDeviceSelector {
    #[default]
    Default,
    StableId(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum CaptureMode {
    #[default]
    SystemMix,
    Application(String),
    Process(u32),
    InputDevice(InputDeviceSelector),
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

    pub fn to_frame_source_id(&self) -> pks_frame::SourceId {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        pks_frame::SourceId(h.finish())
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
    pub fn frame_source_id(&self) -> pks_frame::SourceId {
        self.stable_id.to_frame_source_id()
    }
}

/// Point-in-time authorization evidence for opening one exact capture source.
///
/// This is control-plane evidence, not callback state. Backends must use
/// `NotObservable` when the operating system does not expose an authoritative
/// permission or policy result; a successful stream open is recorded
/// separately and must not be relabeled as an OS permission preflight.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CaptureAuthorizationSnapshot {
    pub capability: CaptureCapabilityState,
    pub os_permission: PermissionObservation,
    pub application_policy: ApplicationPolicyObservation,
    pub session_grant: CaptureSessionGrant,
    pub capture_scope: CaptureScope,
    pub identity_strength: SourceIdentityStrength,
    pub permission_epoch: PermissionEpoch,
    pub observed_at_ns: u64,
    pub open_outcome: CaptureOpenOutcome,
}

impl CaptureAuthorizationSnapshot {
    /// Records the evidence available after an explicitly selected source opens.
    pub fn after_successful_open(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
    ) -> Self {
        Self::from_open_outcome(
            source,
            session_grant,
            permission_epoch,
            CaptureOpenOutcome::Succeeded,
        )
    }

    /// Records a backend open failure without guessing that permission was denied.
    pub fn after_failed_open(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
    ) -> Self {
        Self::from_open_outcome(
            source,
            session_grant,
            permission_epoch,
            CaptureOpenOutcome::BackendFailed,
        )
    }

    /// Records a source that was resolved but not opened because another
    /// required source failed first.
    pub fn before_open(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
    ) -> Self {
        Self::from_open_outcome(
            source,
            session_grant,
            permission_epoch,
            CaptureOpenOutcome::NotAttempted,
        )
    }

    /// Records platform authorization observations without inferring them from
    /// a generic backend result. Callers must pass `NotObservable` when their
    /// platform has no authoritative query for the requested capture class.
    pub fn from_open_observations(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
        os_permission: PermissionObservation,
        application_policy: ApplicationPolicyObservation,
        open_outcome: CaptureOpenOutcome,
    ) -> Self {
        let mut snapshot =
            Self::from_open_outcome(source, session_grant, permission_epoch, open_outcome);
        snapshot.os_permission = os_permission;
        snapshot.application_policy = application_policy;
        snapshot
    }

    fn from_open_outcome(
        source: &CaptureSource,
        session_grant: CaptureSessionGrant,
        permission_epoch: PermissionEpoch,
        open_outcome: CaptureOpenOutcome,
    ) -> Self {
        let (capture_scope, identity_strength, application_policy) = match source.stable_id.kind {
            SourceKind::Application => (
                CaptureScope::ExactApplication {
                    stable_id: source.stable_id.stable_key.clone(),
                },
                if source.app_id.is_some() && source.process_id.is_some() {
                    SourceIdentityStrength::ApplicationIdAndProcessId
                } else if source.app_id.is_some() {
                    SourceIdentityStrength::StableApplicationId
                } else {
                    SourceIdentityStrength::PlatformStableId
                },
                ApplicationPolicyObservation::NotObservable,
            ),
            SourceKind::InputDevice => (
                CaptureScope::ExactInputDevice {
                    stable_id: source.stable_id.stable_key.clone(),
                },
                if source.device_uid.is_some() {
                    SourceIdentityStrength::StableDeviceUid
                } else {
                    SourceIdentityStrength::PlatformStableId
                },
                ApplicationPolicyObservation::NotApplicable,
            ),
            SourceKind::OutputDevice => (
                CaptureScope::ExactOutputDevice {
                    stable_id: source.stable_id.stable_key.clone(),
                },
                SourceIdentityStrength::PlatformStableId,
                ApplicationPolicyObservation::NotApplicable,
            ),
            SourceKind::SystemMix => (
                CaptureScope::SystemMix,
                SourceIdentityStrength::PlatformStableId,
                ApplicationPolicyObservation::NotApplicable,
            ),
        };
        Self {
            capability: if source.state == SourceState::Unavailable {
                CaptureCapabilityState::Unavailable
            } else {
                CaptureCapabilityState::Available
            },
            os_permission: PermissionObservation::NotObservable,
            application_policy,
            session_grant,
            capture_scope,
            identity_strength,
            permission_epoch,
            observed_at_ns: monotonic_timestamp_ns(),
            open_outcome,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureCapabilityState {
    Available,
    Unavailable,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionObservation {
    Allowed,
    Denied,
    Restricted,
    NotDetermined,
    Revoked,
    NotObservable,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationPolicyObservation {
    Allowed,
    Denied,
    NotObservable,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureSessionGrant {
    GrantedByExplicitSelection,
    Denied,
    NotEvaluated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum CaptureScope {
    ExactApplication { stable_id: String },
    ExactInputDevice { stable_id: String },
    ExactOutputDevice { stable_id: String },
    SystemMix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceIdentityStrength {
    ApplicationIdAndProcessId,
    StableApplicationId,
    StableDeviceUid,
    PlatformStableId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct PermissionEpoch(pub u64);

impl PermissionEpoch {
    pub const INITIAL: Self = Self(1);

    /// Advances the local evidence epoch after an observed authorization change
    /// or an explicit source reopen.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureOpenOutcome {
    NotAttempted,
    Succeeded,
    PermissionDenied,
    BackendFailed,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CaptureError {
    #[error("system audio loopback is not supported on this platform")]
    NotSupported,
    #[error("loopback backend error: {0}")]
    BackendInit(String),
    #[error("capture permission denied while {operation}")]
    PermissionDenied { operation: &'static str },
    #[error("capture backend failed while {operation}: status {status}")]
    BackendStatus {
        operation: &'static str,
        status: i32,
    },
    #[error("capture mode not supported on this backend: {0:?}")]
    ModeUnsupported(CaptureMode),
    #[error("captured-frame stream capacity must be greater than zero")]
    InvalidStreamCapacity,
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
    pub id: pks_frame::SourceId,
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
    pub id: pks_frame::SourceId,
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
        F: FnMut(pks_frame::AudioFrame) + Send + 'static,
    {
        Err(CaptureError::NotSupported)
    }
    pub fn capture_mode<F>(_mode: CaptureMode, _cb: F) -> Result<Self, CaptureError>
    where
        F: FnMut(pks_frame::AudioFrame) + Send + 'static,
    {
        Err(CaptureError::NotSupported)
    }
}

/// A request for sources, resolved against discovered `CaptureSource`s. `App("Discord")`
/// is a query — the provider resolves it to concrete sources, not a source itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceQuery {
    Any,
    App(String),         // case-insensitive match on app_id or display name
    ByKind(SourceKind),  // microphone, output device, system mix, …
    ByStableKey(String), // exact stable-id key
    Playing,             // only sources currently producing audio
}

impl SourceQuery {
    pub fn matches(&self, source: &CaptureSource) -> bool {
        match self {
            Self::Any => true,
            Self::App(name) => {
                let needle = name.to_lowercase();
                source
                    .app_id
                    .as_deref()
                    .is_some_and(|a| a.to_lowercase().contains(&needle))
                    || source.name.to_lowercase().contains(&needle)
            }
            Self::ByKind(kind) => source.stable_id.kind == *kind,
            Self::ByStableKey(key) => &source.stable_id.stable_key == key,
            Self::Playing => source.state == SourceState::Playing,
        }
    }
}

/// Resolve a query against an already-discovered source list.
pub fn resolve_query(query: &SourceQuery, sources: &[CaptureSource]) -> Vec<CaptureSource> {
    sources
        .iter()
        .filter(|s| query.matches(s))
        .cloned()
        .collect()
}

/// Discovers sources and resolves queries against them. The local provider lists
/// what the platform exposes; a remote/test provider can implement this differently.
pub trait SourceProvider {
    fn discover(&self, query: &SourceQuery) -> Vec<CaptureSource>;
}

pub struct LocalSourceProvider;

impl SourceProvider for LocalSourceProvider {
    fn discover(&self, query: &SourceQuery) -> Vec<CaptureSource> {
        resolve_query(query, &discover_sources())
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
    F: FnMut(pks_frame::AudioFrame) + Send + 'static,
{
    SystemLoopbackSource::capture(callback)
}

pub fn capture_with_mode<F>(
    mode: CaptureMode,
    callback: F,
) -> Result<SystemLoopbackSource, CaptureError>
where
    F: FnMut(pks_frame::AudioFrame) + Send + 'static,
{
    SystemLoopbackSource::capture_mode(mode, callback)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_frame::Platform;

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
    fn given_stable_input_selector_when_wrapped_then_capture_mode_preserves_identity() {
        let selector = InputDeviceSelector::StableId("coreaudio:device-7".to_owned());
        assert_eq!(
            CaptureMode::InputDevice(selector.clone()),
            CaptureMode::InputDevice(selector)
        );
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
    fn given_monotonic_capture_clock_when_sampled_then_never_moves_backwards() {
        let first = monotonic_timestamp_ns();
        let second = monotonic_timestamp_ns();
        assert!(first > 0);
        assert!(second >= first);
    }

    #[test]
    fn given_capture_sample_timeline_when_callback_arrival_varies_then_media_cadence_is_exact() {
        let mut timeline = CaptureSampleTimeline::new(NonZeroU32::new(48_000).unwrap());

        let first_timestamp_ns = timeline.advance(960);
        std::thread::sleep(std::time::Duration::from_millis(1));
        let second_timestamp_ns = timeline.advance(480);
        let third_timestamp_ns = timeline.advance(480);

        assert_eq!(second_timestamp_ns - first_timestamp_ns, 20_000_000);
        assert_eq!(third_timestamp_ns - second_timestamp_ns, 10_000_000);
    }

    #[test]
    fn given_small_capture_chunks_when_one_second_elapses_then_rounding_does_not_drift() {
        let mut timeline = CaptureSampleTimeline::new(NonZeroU32::new(48_000).unwrap());
        let origin_timestamp_ns = timeline.advance(128);
        for _ in 1..375 {
            timeline.advance(128);
        }

        let one_second_timestamp_ns = timeline.advance(128);

        assert_eq!(one_second_timestamp_ns - origin_timestamp_ns, 1_000_000_000);
    }

    #[test]
    fn given_exact_application_after_open_when_authorization_snapshotted_then_scope_stays_exact() {
        let mut source = fake_source(
            SourceKind::Application,
            "com.acme.meeting",
            Some("com.acme.meeting"),
            SourceState::Playing,
        );
        source.process_id = Some(42);

        let snapshot = CaptureAuthorizationSnapshot::after_successful_open(
            &source,
            CaptureSessionGrant::GrantedByExplicitSelection,
            PermissionEpoch::INITIAL,
        );

        assert_eq!(snapshot.capability, CaptureCapabilityState::Available);
        assert_eq!(
            snapshot.capture_scope,
            CaptureScope::ExactApplication {
                stable_id: "com.acme.meeting".to_owned()
            }
        );
        assert_eq!(
            snapshot.identity_strength,
            SourceIdentityStrength::ApplicationIdAndProcessId
        );
        assert_eq!(
            snapshot.application_policy,
            ApplicationPolicyObservation::NotObservable
        );
        assert_eq!(snapshot.os_permission, PermissionObservation::NotObservable);
        assert_eq!(snapshot.open_outcome, CaptureOpenOutcome::Succeeded);
    }

    #[test]
    fn given_exact_microphone_after_open_when_authorization_snapshotted_then_device_uid_is_retained(
    ) {
        let mut source = fake_source(
            SourceKind::InputDevice,
            "coreaudio:built-in-mic",
            None,
            SourceState::Available,
        );
        source.device_uid = Some("coreaudio:built-in-mic".to_owned());

        let snapshot = CaptureAuthorizationSnapshot::after_successful_open(
            &source,
            CaptureSessionGrant::GrantedByExplicitSelection,
            PermissionEpoch::INITIAL,
        );

        assert_eq!(
            snapshot.capture_scope,
            CaptureScope::ExactInputDevice {
                stable_id: "coreaudio:built-in-mic".to_owned()
            }
        );
        assert_eq!(
            snapshot.identity_strength,
            SourceIdentityStrength::StableDeviceUid
        );
        assert_eq!(
            snapshot.application_policy,
            ApplicationPolicyObservation::NotApplicable
        );
    }

    #[test]
    fn given_unclassified_backend_failure_when_snapshotted_then_permission_is_not_guessed() {
        let source = fake_source(
            SourceKind::Application,
            "com.acme.meeting",
            Some("com.acme.meeting"),
            SourceState::Available,
        );

        let snapshot = CaptureAuthorizationSnapshot::after_failed_open(
            &source,
            CaptureSessionGrant::GrantedByExplicitSelection,
            PermissionEpoch::INITIAL,
        );

        assert_eq!(snapshot.os_permission, PermissionObservation::NotObservable);
        assert_eq!(snapshot.open_outcome, CaptureOpenOutcome::BackendFailed);
    }

    #[test]
    fn given_authoritative_permission_when_snapshotted_then_platform_state_is_preserved() {
        let source = fake_source(
            SourceKind::InputDevice,
            "coreaudio:built-in-mic",
            None,
            SourceState::Available,
        );

        let snapshot = CaptureAuthorizationSnapshot::from_open_observations(
            &source,
            CaptureSessionGrant::GrantedByExplicitSelection,
            PermissionEpoch::INITIAL,
            PermissionObservation::Denied,
            ApplicationPolicyObservation::NotApplicable,
            CaptureOpenOutcome::BackendFailed,
        );

        assert_eq!(snapshot.os_permission, PermissionObservation::Denied);
        assert!(snapshot.observed_at_ns > 0);
    }

    #[test]
    fn given_unavailable_source_when_snapshotted_then_capability_is_unavailable() {
        let source = fake_source(
            SourceKind::Application,
            "com.acme.meeting",
            Some("com.acme.meeting"),
            SourceState::Unavailable,
        );

        let snapshot = CaptureAuthorizationSnapshot::before_open(
            &source,
            CaptureSessionGrant::GrantedByExplicitSelection,
            PermissionEpoch::INITIAL,
        );

        assert_eq!(snapshot.capability, CaptureCapabilityState::Unavailable);
    }

    #[test]
    fn given_authorization_transition_when_epoch_advanced_then_previous_snapshot_is_not_reused() {
        assert_eq!(PermissionEpoch::INITIAL.next(), PermissionEpoch(2));
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

    fn fake_source(
        kind: SourceKind,
        name: &str,
        app_id: Option<&str>,
        state: SourceState,
    ) -> CaptureSource {
        CaptureSource {
            stable_id: StableSourceId::new(Platform::Macos, kind, name),
            name: name.to_owned(),
            process_id: None,
            app_id: app_id.map(|a| a.to_owned()),
            device_uid: None,
            state,
            sample_rate_hz: 48_000,
            channels: 2,
        }
    }

    #[test]
    fn given_app_query_when_matched_against_matching_app_id_then_true() {
        let src = fake_source(
            SourceKind::Application,
            "Discord",
            Some("com.hnc.Discord"),
            SourceState::Playing,
        );
        assert!(SourceQuery::App("discord".to_owned()).matches(&src));
        assert!(!SourceQuery::App("spotify".to_owned()).matches(&src));
    }

    #[test]
    fn given_kind_query_when_resolved_then_only_matching_kind_returned() {
        let sources = vec![
            fake_source(
                SourceKind::Application,
                "Discord",
                None,
                SourceState::Playing,
            ),
            fake_source(
                SourceKind::SystemMix,
                "System",
                None,
                SourceState::Available,
            ),
        ];
        let out = resolve_query(&SourceQuery::ByKind(SourceKind::SystemMix), &sources);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].stable_id.kind, SourceKind::SystemMix);
    }

    #[test]
    fn given_playing_query_when_resolved_then_only_playing_sources_returned() {
        let sources = vec![
            fake_source(SourceKind::Application, "A", None, SourceState::Playing),
            fake_source(SourceKind::Application, "B", None, SourceState::Silent),
        ];
        let out = resolve_query(&SourceQuery::Playing, &sources);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].name, "A");
    }

    #[test]
    fn given_any_query_when_resolved_then_all_sources_returned() {
        let sources = vec![
            fake_source(SourceKind::Application, "A", None, SourceState::Playing),
            fake_source(SourceKind::SystemMix, "B", None, SourceState::Available),
        ];
        assert_eq!(resolve_query(&SourceQuery::Any, &sources).len(), 2);
    }

    #[test]
    fn given_local_provider_when_discover_any_then_does_not_panic() {
        let provider = LocalSourceProvider;
        let _ = provider.discover(&SourceQuery::Any);
    }
}

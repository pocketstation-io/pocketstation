//! Stable source identity and source discovery state.

use crate::frame::Platform;

use super::authorization::SourceIdentityStrength;
use super::selection::{ProcessTreeScope, SelectorPersistenceScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Selects the source kind used by PocketStation."]
pub enum SourceKind {
    #[doc = "Classifies a capture source as application."]
    Application,
    #[doc = "Classifies a capture source as output device."]
    OutputDevice,
    #[doc = "Classifies a capture source as input device."]
    InputDevice,
    #[doc = "Classifies a capture source as system mix."]
    SystemMix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Selects the source state used by PocketStation."]
pub enum SourceState {
    #[doc = "Indicates the available state for `SourceState`."]
    Available,
    #[doc = "Indicates the playing state for `SourceState`."]
    Playing,
    #[doc = "Indicates the silent state for `SourceState`."]
    Silent,
    #[doc = "Reports that the requested resource is unavailable."]
    Unavailable,
    #[doc = "Indicates the permission blocked state for `SourceState`."]
    PermissionBlocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[doc = "Uniquely identifies stable source."]
pub struct StableSourceId {
    #[doc = "Stores the platform as a `Platform` value in `StableSourceId`."]
    pub platform: Platform,
    #[doc = "Records the kind selected for `StableSourceId`."]
    pub kind: SourceKind,
    #[doc = "Stores the stable source key associated with `StableSourceId`."]
    pub stable_key: String,
}

impl StableSourceId {
    #[doc = "Creates a new `StableSourceId`."]
    pub fn new(platform: Platform, kind: SourceKind, stable_key: impl Into<String>) -> Self {
        Self {
            platform,
            kind,
            stable_key: stable_key.into(),
        }
    }

    /// Derives the immutable captured-frame identity for this resolved source.
    ///
    /// The algorithm is explicit and version-stable; it does not depend on
    /// Rust's implementation-defined default hasher. Platform capture owners
    /// call this exactly once after resolving the selected native source.
    pub fn source_id(&self) -> crate::frame::SourceId {
        const FNV1A_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
        const FNV1A_PRIME: u64 = 0x0000_0100_0000_01b3;

        fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
            for byte in bytes {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(FNV1A_PRIME);
            }
            hash
        }

        let platform = match self.platform {
            Platform::Macos => b"macos".as_slice(),
            Platform::Windows => b"windows".as_slice(),
            Platform::Linux => b"linux".as_slice(),
            Platform::Ios => b"ios".as_slice(),
            Platform::Android => b"android".as_slice(),
            Platform::Web => b"web".as_slice(),
            Platform::Unknown => b"unknown".as_slice(),
        };
        let kind = match self.kind {
            SourceKind::Application => b"application".as_slice(),
            SourceKind::OutputDevice => b"output-device".as_slice(),
            SourceKind::InputDevice => b"input-device".as_slice(),
            SourceKind::SystemMix => b"system-mix".as_slice(),
        };
        let hash = hash_bytes(FNV1A_OFFSET_BASIS, platform);
        let hash = hash_bytes(hash, &[0]);
        let hash = hash_bytes(hash, kind);
        let hash = hash_bytes(hash, &[0]);
        crate::frame::SourceId(hash_bytes(hash, self.stable_key.as_bytes()))
    }
}

#[derive(Debug, Clone)]
#[doc = "Owns production of capture values and its lifecycle state."]
pub struct CaptureSource {
    #[doc = "Identifies the stable identifier recorded by `CaptureSource`."]
    pub stable_id: StableSourceId,
    #[doc = "Stores the human-readable name used to identify `CaptureSource`."]
    pub name: String,
    #[doc = "Identifies the process identifier recorded by `CaptureSource`."]
    pub process_id: Option<u32>,
    #[doc = "Identifies the app identifier recorded by `CaptureSource`."]
    pub app_id: Option<String>,
    #[doc = "Stores the device uid component of `CaptureSource`."]
    pub device_uid: Option<String>,
    #[doc = "Records the state selected for `CaptureSource`."]
    pub state: SourceState,
    #[doc = "Stores the sample rate value for `CaptureSource`, in hertz."]
    pub sample_rate_hz: u32,
    #[doc = "Contains the channels owned or reported by `CaptureSource`."]
    pub channels: u16,
}

impl CaptureSource {
    #[doc = "Returns the identity strength held by `CaptureSource`."]
    pub fn identity_strength(&self) -> SourceIdentityStrength {
        match self.stable_id.kind {
            SourceKind::Application if self.app_id.is_some() && self.process_id.is_some() => {
                SourceIdentityStrength::ApplicationIdAndProcessId
            }
            SourceKind::Application if self.app_id.is_some() => {
                SourceIdentityStrength::StableApplicationId
            }
            SourceKind::Application if self.process_id.is_some() => {
                SourceIdentityStrength::ProcessId
            }
            SourceKind::InputDevice | SourceKind::OutputDevice if self.device_uid.is_some() => {
                SourceIdentityStrength::StableDeviceUid
            }
            _ => SourceIdentityStrength::PlatformStableId,
        }
    }

    /// Reports how long this discovered selector can be reused without
    /// rediscovery. The capture owner remains authoritative for opening it.
    pub fn selector_persistence_scope(&self) -> Option<SelectorPersistenceScope> {
        match self.stable_id.kind {
            SourceKind::SystemMix => Some(SelectorPersistenceScope::PlatformIdentity),
            SourceKind::InputDevice if self.device_uid.is_some() => {
                Some(SelectorPersistenceScope::DeviceIdentity)
            }
            SourceKind::InputDevice => Some(SelectorPersistenceScope::SessionDefaultDevice),
            SourceKind::Application if self.app_id.is_some() => {
                Some(SelectorPersistenceScope::ApplicationIdentity)
            }
            SourceKind::Application
                if self.stable_id.platform == Platform::Linux
                    && (self.stable_id.stable_key.starts_with("pw-app:")
                        || self.stable_id.stable_key.starts_with("pw-node:")) =>
            {
                Some(SelectorPersistenceScope::ApplicationIdentity)
            }
            SourceKind::Application if self.process_id.is_some() => {
                Some(SelectorPersistenceScope::ProcessLifetime)
            }
            SourceKind::Application | SourceKind::OutputDevice => None,
        }
    }

    /// Reports the native process boundary represented by this discovery
    /// result without making the CLI reconstruct a private capture mode.
    pub fn process_tree_scope(&self) -> Option<ProcessTreeScope> {
        match self.stable_id.kind {
            SourceKind::Application if self.app_id.is_some() => {
                Some(ProcessTreeScope::ApplicationIdentity)
            }
            SourceKind::Application
                if self.stable_id.platform == Platform::Linux
                    && (self.stable_id.stable_key.starts_with("pw-app:")
                        || self.stable_id.stable_key.starts_with("pw-node:")) =>
            {
                Some(ProcessTreeScope::ApplicationIdentity)
            }
            SourceKind::Application if self.process_id.is_some() => {
                if self.stable_id.platform == Platform::Windows {
                    Some(ProcessTreeScope::SelectedProcessAndDescendants)
                } else {
                    Some(ProcessTreeScope::SelectedProcessOnly)
                }
            }
            SourceKind::Application => None,
            SourceKind::SystemMix | SourceKind::InputDevice | SourceKind::OutputDevice => {
                Some(ProcessTreeScope::NotApplicable)
            }
        }
    }
}

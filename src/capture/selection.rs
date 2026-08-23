//! Capture selection semantics; control-plane only.

use crate::frame::Platform;
use serde::Serialize;

use super::identity::StableSourceId;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[doc = "Enumerates the supported input device selector cases."]
pub enum InputDeviceSelector {
    #[default]
    #[doc = "Selects default behavior for `InputDeviceSelector`."]
    Default,
    #[doc = "Selects stable identifier behavior for `InputDeviceSelector`."]
    StableId(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[doc = "Selects the capture mode used by PocketStation."]
pub enum CaptureMode {
    #[default]
    #[doc = "Selects system mix behavior for `CaptureMode`."]
    SystemMix,
    #[doc = "Selects application behavior for `CaptureMode`."]
    Application(String),
    #[doc = "Selects process behavior for `CaptureMode`."]
    Process(u32),
    #[doc = "Selects exact application behavior for `CaptureMode`."]
    ExactApplication {
        #[doc = "Identifies the process associated with `ExactApplication`."]
        process_id: u32,
        #[doc = "Identifies the stable associated with `ExactApplication`."]
        stable_id: StableSourceId,
    },
    #[doc = "Selects exact application stable behavior for `CaptureMode`."]
    ExactApplicationStable {
        #[doc = "Identifies the stable associated with `ExactApplicationStable`."]
        stable_id: StableSourceId,
    },
    #[doc = "Selects input device behavior for `CaptureMode`."]
    InputDevice(InputDeviceSelector),
}

impl CaptureMode {
    /// Describes how long the selector may be reused without rediscovery.
    ///
    /// This is control-plane metadata. It never authorizes a backend to follow
    /// a replacement process or substitute a default device.
    pub fn selector_persistence_scope(&self) -> SelectorPersistenceScope {
        match self {
            Self::SystemMix => SelectorPersistenceScope::PlatformIdentity,
            Self::Application(_) | Self::ExactApplicationStable { .. } => {
                SelectorPersistenceScope::ApplicationIdentity
            }
            Self::Process(_) | Self::ExactApplication { .. } => {
                SelectorPersistenceScope::ProcessLifetime
            }
            Self::InputDevice(InputDeviceSelector::Default) => {
                SelectorPersistenceScope::SessionDefaultDevice
            }
            Self::InputDevice(InputDeviceSelector::StableId(_)) => {
                SelectorPersistenceScope::DeviceIdentity
            }
        }
    }

    /// Reports the process boundary requested from the native backend.
    pub fn process_tree_scope(&self, platform: Platform) -> ProcessTreeScope {
        match self {
            Self::Process(_) | Self::ExactApplication { .. } if platform == Platform::Windows => {
                ProcessTreeScope::SelectedProcessAndDescendants
            }
            Self::Process(_) | Self::ExactApplication { .. } => {
                ProcessTreeScope::SelectedProcessOnly
            }
            Self::Application(_) | Self::ExactApplicationStable { .. } => {
                ProcessTreeScope::ApplicationIdentity
            }
            Self::SystemMix | Self::InputDevice(_) => ProcessTreeScope::NotApplicable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Selects the selector persistence scope used by PocketStation."]
pub enum SelectorPersistenceScope {
    #[doc = "Selects process lifetime behavior for `SelectorPersistenceScope`."]
    ProcessLifetime,
    #[doc = "Selects application identity behavior for `SelectorPersistenceScope`."]
    ApplicationIdentity,
    #[doc = "Selects device identity behavior for `SelectorPersistenceScope`."]
    DeviceIdentity,
    #[doc = "Selects session default device behavior for `SelectorPersistenceScope`."]
    SessionDefaultDevice,
    #[doc = "Selects platform identity behavior for `SelectorPersistenceScope`."]
    PlatformIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Selects the process tree scope used by PocketStation."]
pub enum ProcessTreeScope {
    #[doc = "Selects selected process only behavior for `ProcessTreeScope`."]
    SelectedProcessOnly,
    #[doc = "Selects selected process and descendants behavior for `ProcessTreeScope`."]
    SelectedProcessAndDescendants,
    #[doc = "Selects application identity behavior for `ProcessTreeScope`."]
    ApplicationIdentity,
    #[doc = "Selects not applicable behavior for `ProcessTreeScope`."]
    NotApplicable,
}

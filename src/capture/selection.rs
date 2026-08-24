//! Capture selection semantics; control-plane only.

use crate::frame::Platform;
use serde::Serialize;

use super::identity::StableSourceId;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[doc = "Selects either the default input device or one exact device identity."]
pub enum InputDeviceSelector {
    #[default]
    #[doc = "Selects an input device by default."]
    Default,
    #[doc = "Selects an input device by stable identifier."]
    StableId(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[doc = "Selects the capture mode used by PocketStation."]
pub enum CaptureMode {
    #[default]
    #[doc = "Requests capture in system mix mode."]
    SystemMix,
    #[doc = "Requests capture in application mode."]
    Application(String),
    #[doc = "Requests capture in process mode."]
    Process(u32),
    #[doc = "Requests capture in exact application mode."]
    ExactApplication {
        #[doc = "Identifies the process identifier recorded by `ExactApplication`."]
        process_id: u32,
        #[doc = "Identifies the stable identifier recorded by `ExactApplication`."]
        stable_id: StableSourceId,
    },
    #[doc = "Requests capture in exact application stable mode."]
    ExactApplicationStable {
        #[doc = "Identifies the stable identifier recorded by `ExactApplicationStable`."]
        stable_id: StableSourceId,
    },
    #[doc = "Requests capture in input device mode."]
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
    #[doc = "Limits selector persistence to the process lifetime scope."]
    ProcessLifetime,
    #[doc = "Limits selector persistence to the application identity scope."]
    ApplicationIdentity,
    #[doc = "Limits selector persistence to the device identity scope."]
    DeviceIdentity,
    #[doc = "Limits selector persistence to the session default device scope."]
    SessionDefaultDevice,
    #[doc = "Limits selector persistence to the platform identity scope."]
    PlatformIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Selects the process tree scope used by PocketStation."]
pub enum ProcessTreeScope {
    #[doc = "Limits process capture to selected process only."]
    SelectedProcessOnly,
    #[doc = "Limits process capture to selected process and descendants."]
    SelectedProcessAndDescendants,
    #[doc = "Limits process capture to application identity."]
    ApplicationIdentity,
    #[doc = "Limits process capture to not applicable."]
    NotApplicable,
}

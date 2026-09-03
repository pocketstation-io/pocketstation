//! Capture selection semantics; control-plane only.

use crate::frame::Platform;
use serde::Serialize;

use super::identity::StableSourceId;

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
    ExactApplication {
        process_id: u32,
        stable_id: StableSourceId,
    },
    ExactApplicationStable {
        stable_id: StableSourceId,
    },
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

    /// Reports the process scope requested from the native backend.
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
pub enum SelectorPersistenceScope {
    ProcessLifetime,
    ApplicationIdentity,
    DeviceIdentity,
    SessionDefaultDevice,
    PlatformIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessTreeScope {
    SelectedProcessOnly,
    SelectedProcessAndDescendants,
    ApplicationIdentity,
    NotApplicable,
}

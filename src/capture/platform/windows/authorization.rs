//! Non-prompting Windows microphone capability observation.

use crate::capture::PermissionObservation;
use windows::Security::Authorization::AppCapabilityAccess::{
    AppCapability, AppCapabilityAccessStatus,
};
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_MULTITHREADED};
use windows_core::{Error, HRESULT, HSTRING};

const RPC_E_CHANGED_MODE: HRESULT = HRESULT(0x8001_0106_u32 as i32);

/// Observes the current process' Windows microphone capability without
/// requesting access or displaying user interface.
pub fn microphone_permission_observation() -> PermissionObservation {
    let Ok(_apartment) = WinRtApartment::initialize() else {
        return PermissionObservation::NotObservable;
    };
    let Ok(capability) = AppCapability::Create(&HSTRING::from("Microphone")) else {
        return PermissionObservation::NotObservable;
    };
    capability
        .CheckAccess()
        .map(permission_observation)
        .unwrap_or(PermissionObservation::NotObservable)
}

fn permission_observation(status: AppCapabilityAccessStatus) -> PermissionObservation {
    match status {
        AppCapabilityAccessStatus::Allowed => PermissionObservation::Allowed,
        AppCapabilityAccessStatus::DeniedByUser => PermissionObservation::Denied,
        AppCapabilityAccessStatus::DeniedBySystem | AppCapabilityAccessStatus::NotDeclaredByApp => {
            PermissionObservation::Restricted
        }
        AppCapabilityAccessStatus::UserPromptRequired => PermissionObservation::NotDetermined,
        _ => PermissionObservation::NotObservable,
    }
}

struct WinRtApartment {
    owns_initialization: bool,
}

impl WinRtApartment {
    fn initialize() -> Result<Self, Error> {
        // SAFETY: this control-plane call initializes only the current thread.
        // A changed-mode result means the host already initialized a different
        // apartment, which is still usable for the agile AppCapability API.
        match unsafe { RoInitialize(RO_INIT_MULTITHREADED) } {
            Ok(()) => Ok(Self {
                owns_initialization: true,
            }),
            Err(error) if error.code() == RPC_E_CHANGED_MODE => Ok(Self {
                owns_initialization: false,
            }),
            Err(error) => Err(error),
        }
    }
}

impl Drop for WinRtApartment {
    fn drop(&mut self) {
        if self.owns_initialization {
            // SAFETY: this balances the successful RoInitialize on this same
            // control thread. It never runs on an audio callback.
            unsafe { RoUninitialize() };
        }
    }
}

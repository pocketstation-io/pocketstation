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
    if !winrt_apartment_available() {
        return PermissionObservation::NotObservable;
    }
    let Ok(capability) = AppCapability::Create(&HSTRING::from("Microphone")) else {
        return PermissionObservation::NotObservable;
    };
    capability
        .CheckAccess()
        .map(permission_observation)
        .unwrap_or(PermissionObservation::NotObservable)
}

thread_local! {
    // Keep library-owned WinRT initialization alive for the thread lifetime.
    // The Windows projection caches activation factories, so balancing
    // RoInitialize after every query can leave cached runtime state behind a
    // torn-down apartment. Host-owned apartments are never uninitialized here.
    static WINRT_APARTMENT: Option<WinRtApartment> = WinRtApartment::initialize().ok();
}

fn winrt_apartment_available() -> bool {
    WINRT_APARTMENT.with(Option::is_some)
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

#[cfg(test)]
mod tests {
    use super::microphone_permission_observation;

    #[test]
    fn given_repeated_permission_observation_when_queried_then_winrt_state_remains_valid() {
        let _first = microphone_permission_observation();
        let _second = microphone_permission_observation();
    }
}

use crate::capture::PermissionObservation;

const NOT_DETERMINED: i32 = 0;
const RESTRICTED: i32 = 1;
const DENIED: i32 = 2;
const AUTHORIZED: i32 = 3;

extern "C" {
    fn pks_microphone_authorization_status() -> i32;
}

/// Reads the current macOS microphone authorization state without prompting.
/// This control-path query must never run from an audio callback.
pub fn microphone_permission_observation() -> PermissionObservation {
    // SAFETY: the Objective-C function takes no pointers, performs a read-only
    // AVFoundation authorization query, and returns an integer value.
    permission_observation(unsafe { pks_microphone_authorization_status() })
}

fn permission_observation(status: i32) -> PermissionObservation {
    match status {
        NOT_DETERMINED => PermissionObservation::NotDetermined,
        RESTRICTED => PermissionObservation::Restricted,
        DENIED => PermissionObservation::Denied,
        AUTHORIZED => PermissionObservation::Allowed,
        _ => PermissionObservation::NotObservable,
    }
}

#[cfg(test)]
mod tests {
    use super::{permission_observation, AUTHORIZED, DENIED, NOT_DETERMINED, RESTRICTED};
    use crate::capture::PermissionObservation;

    #[test]
    fn given_authorization_values_when_mapped_then_every_state_remains_distinct() {
        assert_eq!(
            permission_observation(NOT_DETERMINED),
            PermissionObservation::NotDetermined
        );
        assert_eq!(
            permission_observation(RESTRICTED),
            PermissionObservation::Restricted
        );
        assert_eq!(
            permission_observation(DENIED),
            PermissionObservation::Denied
        );
        assert_eq!(
            permission_observation(AUTHORIZED),
            PermissionObservation::Allowed
        );
        assert_eq!(
            permission_observation(i32::MAX),
            PermissionObservation::NotObservable
        );
    }
}

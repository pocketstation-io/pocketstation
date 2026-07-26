/// HRESULT returned when the audio endpoint backing an active WASAPI stream
/// is invalidated. This is the only WASAPI status classified as authoritative
/// source disappearance here.
pub(crate) const AUDCLNT_E_DEVICE_INVALIDATED_STATUS_CODE: i32 = 0x8889_0004_u32 as i32;

/// A related but distinct WASAPI status. Resource invalidation does not prove
/// that the selected source disappeared, so it must remain a backend failure.
#[cfg(test)]
const AUDCLNT_E_RESOURCES_INVALIDATED_STATUS_CODE: i32 = 0x8889_0026_u32 as i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WindowsRuntimeFailureDisposition {
    SourceUnavailable,
    BackendFailure,
}

pub(crate) fn classify_platform_status(status_code: i32) -> WindowsRuntimeFailureDisposition {
    if status_code == AUDCLNT_E_DEVICE_INVALIDATED_STATUS_CODE {
        WindowsRuntimeFailureDisposition::SourceUnavailable
    } else {
        WindowsRuntimeFailureDisposition::BackendFailure
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_device_invalidated_hresult_when_classified_then_source_is_unavailable() {
        assert_eq!(
            classify_platform_status(AUDCLNT_E_DEVICE_INVALIDATED_STATUS_CODE),
            WindowsRuntimeFailureDisposition::SourceUnavailable
        );
    }

    #[test]
    fn given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance(
    ) {
        assert_eq!(
            classify_platform_status(AUDCLNT_E_RESOURCES_INVALIDATED_STATUS_CODE),
            WindowsRuntimeFailureDisposition::BackendFailure
        );
    }

    #[test]
    fn given_generic_hresult_when_classified_then_exact_failure_remains_backend_failure() {
        assert_eq!(
            classify_platform_status(0x8000_4005_u32 as i32),
            WindowsRuntimeFailureDisposition::BackendFailure
        );
    }
}

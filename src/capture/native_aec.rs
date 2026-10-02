use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::capture::StableSourceId;
use crate::frame::SourceId;
use crate::DeviceId;

/// Explicit request to pair microphone processing with one playback device.
///
/// This is setup data, not evidence that the backend supports the route. The
/// backend must attest the exact opened route before Session admits delivery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeAecRequest {
    playback_device: DeviceId,
}

impl NativeAecRequest {
    pub fn new(playback_device: DeviceId) -> Self {
        Self { playback_device }
    }

    pub fn playback_device(&self) -> &DeviceId {
        &self.playback_device
    }
}

/// Backend-observed configuration of one opened native microphone route.
///
/// These facts describe configuration, not acoustic qualification. An enabled,
/// non-bypassed route with a confirmed reference does not establish cancellation
/// quality or coverage of audio outside the declared playback device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeAecRoute {
    source_id: SourceId,
    microphone: StableSourceId,
    playback_device: DeviceId,
    enabled: bool,
    bypassed: bool,
    reference_confirmed: bool,
}

impl NativeAecRoute {
    /// Records facts obtained from the actual opened input and reference route.
    ///
    /// Callers must not infer these facts from names, build flags, or requested
    /// options. Session independently checks identity and configuration.
    pub fn new(
        source_id: SourceId,
        microphone: StableSourceId,
        playback_device: DeviceId,
        enabled: bool,
        bypassed: bool,
        reference_confirmed: bool,
    ) -> Self {
        Self {
            source_id,
            microphone,
            playback_device,
            enabled,
            bypassed,
            reference_confirmed,
        }
    }

    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    pub fn microphone(&self) -> &StableSourceId {
        &self.microphone
    }

    pub fn playback_device(&self) -> &DeviceId {
        &self.playback_device
    }

    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    pub const fn bypassed(&self) -> bool {
        self.bypassed
    }

    pub const fn reference_confirmed(&self) -> bool {
        self.reference_confirmed
    }
}

#[derive(Debug)]
struct NativeAecRouteState {
    route: NativeAecRoute,
    invalidated: AtomicBool,
}

/// Immutable route metadata and allocation-free, lock-free validity observation.
///
/// Validity means the backend has not revoked these configuration facts. It
/// does not by itself mean the route is suitable; admission must check its
/// identities and enabled, bypassed, and reference-confirmed fields.
#[derive(Debug, Clone)]
pub struct NativeAecRouteHandle {
    state: Arc<NativeAecRouteState>,
}

impl NativeAecRouteHandle {
    /// Performs one atomic load, without querying the operating system.
    pub fn is_valid(&self) -> bool {
        !self.state.invalidated.load(Ordering::Acquire)
    }

    /// Returns the original facts, including after irrevocable invalidation.
    ///
    /// Inspect this metadata on control/error paths. Always check validity
    /// independently before using it to admit capture delivery.
    pub fn route(&self) -> &NativeAecRoute {
        &self.state.route
    }
}

/// Backend-owned revocation of facts for exactly one opened capture.
///
/// The backend must invalidate before delivering PCM after an effect, input,
/// reference, or playback-route change. Invalidation does not stop a native
/// stream itself; its capture owner must still stop and reclaim the resources.
#[derive(Debug, Clone)]
pub struct NativeAecRouteReporter {
    state: Arc<NativeAecRouteState>,
}

impl NativeAecRouteReporter {
    /// Irrevocably invalidates the opened route; recovery requires a new open
    /// and a new reporter/handle pair. Repeated calls remain invalid.
    pub fn invalidate(&self) {
        self.state.invalidated.store(true, Ordering::Release);
    }
}

/// Creates backend-owned configuration evidence for one opened capture.
///
/// Allocate this pair on a control thread. The reporter cannot mutate facts
/// or restore validity; a new route requires a new opened capture.
pub fn native_aec_route(route: NativeAecRoute) -> (NativeAecRouteReporter, NativeAecRouteHandle) {
    let state = Arc::new(NativeAecRouteState {
        route,
        invalidated: AtomicBool::new(false),
    });
    (
        NativeAecRouteReporter {
            state: Arc::clone(&state),
        },
        NativeAecRouteHandle { state },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::SourceKind;
    use crate::frame::Platform;

    fn route(enabled: bool, bypassed: bool, reference_confirmed: bool) -> NativeAecRoute {
        let microphone = StableSourceId::new(Platform::Macos, SourceKind::InputDevice, "mic");
        NativeAecRoute::new(
            microphone.source_id(),
            microphone,
            DeviceId::new("speaker"),
            enabled,
            bypassed,
            reference_confirmed,
        )
    }

    #[test]
    fn given_opened_route_when_invalidated_repeatedly_then_all_handles_remain_invalid() {
        let expected = route(true, false, true);
        let (reporter, handle) = native_aec_route(expected.clone());
        let clone = handle.clone();
        assert!(handle.is_valid());
        reporter.invalidate();
        reporter.clone().invalidate();
        assert!(!handle.is_valid());
        assert!(!clone.is_valid());
        assert_eq!(handle.route(), &expected);
    }

    #[test]
    fn given_unusable_configuration_when_observed_then_validity_does_not_imply_suitability() {
        let expected = route(false, true, false);
        let (_, handle) = native_aec_route(expected.clone());
        assert!(handle.is_valid());
        assert!(!handle.route().enabled());
        assert!(handle.route().bypassed());
        assert!(!handle.route().reference_confirmed());
        assert_eq!(
            handle.route().source_id(),
            expected.microphone().source_id()
        );
        assert_eq!(handle.route().playback_device().as_str(), "speaker");
    }

    #[test]
    fn given_old_route_when_a_new_pair_is_created_then_old_revocation_is_preserved() {
        let (old_reporter, old_handle) = native_aec_route(route(true, false, true));
        old_reporter.invalidate();
        let (_, new_handle) = native_aec_route(route(true, false, true));
        assert!(!old_handle.is_valid());
        assert!(new_handle.is_valid());
    }
}

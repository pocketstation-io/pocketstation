#[cfg(target_os = "macos")]
mod authorization;
#[cfg(target_os = "macos")]
mod input;
#[cfg(target_os = "macos")]
mod loopback;
#[cfg(target_os = "macos")]
pub mod macos_asp;
#[cfg(target_os = "macos")]
pub mod macos_tap;
#[cfg(target_os = "macos")]
mod session_backend;

#[cfg(target_os = "macos")]
pub(crate) use authorization::microphone_permission_observation;
#[cfg(target_os = "macos")]
pub use input::{discover_input_sources_native, MacosInputSource};
#[cfg(target_os = "macos")]
use loopback::SystemLoopbackSource;
#[cfg(all(target_os = "macos", feature = "internal-testing"))]
pub use loopback::SystemLoopbackSource as InternalSystemLoopbackSource;
#[cfg(target_os = "macos")]
pub use macos_tap::{discover_sources_native, tap_available};
#[cfg(target_os = "macos")]
pub(crate) use session_backend::DesktopCaptureBackend;
#[cfg(all(target_os = "macos", feature = "internal-testing"))]
pub use session_backend::DesktopCaptureBackend as InternalDesktopCaptureBackend;
#[cfg(all(target_os = "macos", feature = "internal-testing"))]
pub use DesktopCaptureSource as InternalDesktopCaptureSource;

#[cfg(target_os = "macos")]
// The implementation is held for RAII; dropping it stops the selected capture source.
#[doc = "Owns production of desktop capture values and its lifecycle state."]
pub struct DesktopCaptureSource(DesktopCaptureImplementation);

#[cfg(target_os = "macos")]
enum DesktopCaptureImplementation {
    Input(MacosInputSource),
    Loopback(SystemLoopbackSource),
}

#[cfg(target_os = "macos")]
impl DesktopCaptureSource {
    #[cfg(any(test, feature = "internal-testing"))]
    #[doc = "Returns the capture mode held by `DesktopCaptureSource`."]
    pub fn capture_mode<F>(
        mode: crate::capture::CaptureMode,
        callback: F,
    ) -> Result<Self, crate::capture::CaptureError>
    where
        F: FnMut(crate::frame::AudioFrame) + Send + 'static,
    {
        Self::capture_mode_with_runtime_event_sender(mode, callback, None)
    }

    pub(crate) fn capture_mode_with_runtime_event_sender<F>(
        mode: crate::capture::CaptureMode,
        callback: F,
        runtime_event_sender: Option<crate::capture::SourceRuntimeEventSender>,
    ) -> Result<Self, crate::capture::CaptureError>
    where
        F: FnMut(crate::frame::AudioFrame) + Send + 'static,
    {
        match mode {
            crate::capture::CaptureMode::InputDevice(selector) => {
                MacosInputSource::capture_with_runtime_event_sender(
                    selector,
                    callback,
                    runtime_event_sender,
                )
                .map(DesktopCaptureImplementation::Input)
                .map(Self)
            }
            loopback_mode => SystemLoopbackSource::capture_mode_with_runtime_event_sender(
                loopback_mode,
                callback,
                runtime_event_sender,
            )
            .map(DesktopCaptureImplementation::Loopback)
            .map(Self),
        }
    }

    #[doc = "Returns the observations exposed by `DesktopCaptureSource`."]
    pub fn observations(&self) -> crate::capture::CaptureObservations {
        match &self.0 {
            DesktopCaptureImplementation::Input(source) => source.observations(),
            DesktopCaptureImplementation::Loopback(source) => source.observations(),
        }
    }

    #[doc = "Returns the source identifier held by `DesktopCaptureSource`."]
    pub fn source_id(&self) -> crate::frame::SourceId {
        match &self.0 {
            DesktopCaptureImplementation::Input(source) => source.source_id(),
            DesktopCaptureImplementation::Loopback(source) => source.source_id(),
        }
    }

    #[doc = "Returns a handle for reading observations from `DesktopCaptureSource`."]
    pub fn observation_handle(&self) -> crate::capture::CaptureObservationHandle {
        match &self.0 {
            DesktopCaptureImplementation::Input(source) => source.observation_handle(),
            DesktopCaptureImplementation::Loopback(source) => source.observation_handle(),
        }
    }

    #[doc = "Stops `DesktopCaptureSource`, joins its worker, and returns the terminal result."]
    pub fn stop_and_join(
        self,
    ) -> Result<crate::capture::CaptureObservations, crate::capture::CaptureError> {
        match self.0 {
            DesktopCaptureImplementation::Input(source) => source.stop_and_join(),
            DesktopCaptureImplementation::Loopback(source) => source.stop_and_join(),
        }
    }
}

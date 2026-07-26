#[cfg(target_os = "macos")]
mod authorization;
#[cfg(target_os = "macos")]
mod input;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub mod macos_asp;
#[cfg(target_os = "macos")]
pub mod macos_tap;
#[cfg(target_os = "macos")]
mod session_backend;

#[cfg(target_os = "macos")]
pub use authorization::microphone_permission_observation;
#[cfg(target_os = "macos")]
pub use input::{discover_input_sources_native, MacosInputSource};
#[cfg(target_os = "macos")]
pub use macos::SystemLoopbackSource;
#[cfg(target_os = "macos")]
pub use macos_asp::{asp_is_installed, AspReader};
#[cfg(target_os = "macos")]
pub use macos_tap::{discover_sources_native, tap_available, TapLoopbackSource};
#[cfg(target_os = "macos")]
pub use session_backend::DesktopCaptureBackend;

#[cfg(target_os = "macos")]
// The implementation is held for RAII; dropping it stops the selected capture source.
#[allow(dead_code)]
pub struct DesktopCaptureSource(DesktopCaptureImplementation);

#[cfg(target_os = "macos")]
#[allow(dead_code)]
enum DesktopCaptureImplementation {
    Input(MacosInputSource),
    Loopback(SystemLoopbackSource),
}

#[cfg(target_os = "macos")]
impl DesktopCaptureSource {
    pub fn capture_mode<F>(
        mode: pks_capture::CaptureMode,
        callback: F,
    ) -> Result<Self, pks_capture::CaptureError>
    where
        F: FnMut(pks_frame::AudioFrame) + Send + 'static,
    {
        Self::capture_mode_with_runtime_event_sender(mode, callback, None)
    }

    pub(crate) fn capture_mode_with_runtime_event_sender<F>(
        mode: pks_capture::CaptureMode,
        callback: F,
        runtime_event_sender: Option<pks_capture::SourceRuntimeEventSender>,
    ) -> Result<Self, pks_capture::CaptureError>
    where
        F: FnMut(pks_frame::AudioFrame) + Send + 'static,
    {
        match mode {
            pks_capture::CaptureMode::InputDevice(selector) => {
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

    pub fn observations(&self) -> pks_capture::CaptureObservations {
        match &self.0 {
            DesktopCaptureImplementation::Input(source) => source.observations(),
            DesktopCaptureImplementation::Loopback(source) => source.observations(),
        }
    }

    pub fn stop_and_join(
        self,
    ) -> Result<pks_capture::CaptureObservations, pks_capture::CaptureError> {
        match self.0 {
            DesktopCaptureImplementation::Input(source) => source.stop_and_join(),
            DesktopCaptureImplementation::Loopback(source) => source.stop_and_join(),
        }
    }
}

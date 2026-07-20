#[cfg(target_os = "macos")]
mod input;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub mod macos_asp;
#[cfg(target_os = "macos")]
pub mod macos_tap;

#[cfg(target_os = "macos")]
pub use input::{discover_input_sources_native, InputCaptureObservations, MacosInputSource};
#[cfg(target_os = "macos")]
pub use macos::SystemLoopbackSource;
#[cfg(target_os = "macos")]
pub use macos_asp::{asp_is_installed, AspReader};
#[cfg(target_os = "macos")]
pub use macos_tap::{discover_sources_native, tap_available, TapLoopbackSource};

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
        match mode {
            pks_capture::CaptureMode::InputDevice(selector) => {
                MacosInputSource::capture(selector, callback)
                    .map(DesktopCaptureImplementation::Input)
                    .map(Self)
            }
            loopback_mode => SystemLoopbackSource::capture_mode(loopback_mode, callback)
                .map(DesktopCaptureImplementation::Loopback)
                .map(Self),
        }
    }
}

#[cfg(target_os = "linux")]
mod pipewire;
#[cfg(target_os = "linux")]
mod session_backend;

#[cfg(all(target_os = "linux", any(test, feature = "internal-testing")))]
pub use pipewire::SystemLoopbackSource;
#[cfg(target_os = "linux")]
pub use pipewire::{discover_sources_linux, DesktopCaptureSource};
#[cfg(target_os = "linux")]
pub use session_backend::DesktopCaptureBackend;

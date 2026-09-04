#[cfg(target_os = "windows")]
mod authorization;
#[cfg(any(target_os = "windows", test))]
mod open_lifecycle;
#[cfg(any(target_os = "windows", test))]
mod packet_delivery;
#[cfg(any(target_os = "windows", test))]
mod process_identity;
#[cfg(any(target_os = "windows", test))]
mod runtime_lifecycle;

#[cfg(target_os = "windows")]
mod session_backend;
#[cfg(target_os = "windows")]
mod source;

#[cfg(target_os = "windows")]
pub(crate) use authorization::microphone_permission_observation;
#[cfg(target_os = "windows")]
pub use session_backend::DesktopCaptureBackend;
#[cfg(any(test, feature = "internal-testing"))]
pub use source::SystemLoopbackSource;
#[cfg(target_os = "windows")]
pub use source::{discover_sources_windows, DesktopCaptureSource};

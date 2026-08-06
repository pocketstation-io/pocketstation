#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
mod session_backend;

#[cfg(target_os = "linux")]
pub use linux::{discover_sources_linux, DesktopCaptureSource, SystemLoopbackSource};
#[cfg(target_os = "linux")]
pub use session_backend::DesktopCaptureBackend;

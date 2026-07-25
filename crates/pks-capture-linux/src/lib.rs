#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::{discover_sources_linux, DesktopCaptureSource, SystemLoopbackSource};

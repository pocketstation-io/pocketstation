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
mod windows;

#[cfg(target_os = "windows")]
pub use session_backend::DesktopCaptureBackend;
#[cfg(target_os = "windows")]
pub use windows::{
    discover_sources_windows, DesktopCaptureSource, SystemLoopbackSource,
    WindowsAudioThreadPriorityGuard, WASAPI_PROCESS_LOOPBACK_PERIOD_100NS,
};

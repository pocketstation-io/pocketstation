#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::{SystemLoopbackSource, discover_sources_windows, WASAPI_PROCESS_LOOPBACK_PERIOD_100NS};

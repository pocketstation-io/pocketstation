#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::{
    discover_sources_windows, SystemLoopbackSource, WASAPI_PROCESS_LOOPBACK_PERIOD_100NS,
};

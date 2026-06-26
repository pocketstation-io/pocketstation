#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub mod macos_asp;
#[cfg(target_os = "macos")]
pub mod macos_tap;

#[cfg(target_os = "macos")]
pub use macos::SystemLoopbackSource;
#[cfg(target_os = "macos")]
pub use macos_asp::{asp_is_installed, AspReader};
#[cfg(target_os = "macos")]
pub use macos_tap::{discover_sources_native, tap_available, TapLoopbackSource};

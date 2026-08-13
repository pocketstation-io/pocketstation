#[cfg(all(target_os = "linux", feature = "native-capture"))]
pub(crate) mod linux;
#[cfg(all(target_os = "macos", feature = "native-capture"))]
pub(crate) mod macos;
#[cfg(all(target_os = "windows", feature = "native-capture"))]
pub(crate) mod windows;

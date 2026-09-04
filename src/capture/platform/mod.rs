#[cfg(all(
    target_os = "linux",
    any(feature = "pipewire-capture", feature = "alsa-fallback")
))]
pub(crate) mod linux;
#[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
pub(crate) mod macos;
#[cfg(all(target_os = "windows", feature = "wasapi-capture"))]
pub(crate) mod windows;

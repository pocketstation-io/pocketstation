extern "C" {
    fn pks_asp_is_installed() -> std::ffi::c_int;
}

/// Returns `true` if the PocketStation AudioServerPlugin is installed in
/// `/Library/Audio/Plug-Ins/HAL/`.
///
/// When compiled without the `asp` Cargo feature the bridge stub always
/// returns `false`.
pub fn asp_is_installed() -> bool {
    // SAFETY: pure C function; no side effects; always returns 0 or 1.
    unsafe { pks_asp_is_installed() != 0 }
}

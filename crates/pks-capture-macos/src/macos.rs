//! macOS audio loopback capture backend.
//!
//! On macOS 14.4+ (public support), uses `AudioHardwareCreateProcessTap` / `CATapDescription`
//! (the process tap path).  No HAL plugin installation, no Screen Recording
//! permission, no deadlock.
//!
//! On older macOS, falls back to the AudioServerPlugin + POSIX SHM ring.
//! On first use, if the HAL driver is not installed, this module prompts for
//! the administrator password, installs the driver, and restarts `coreaudiod`
//! automatically.
//!
//! # Hot-path invariants
//!
//! No allocation, locking, logging, or panicking on the audio delivery path.
//! Pool slot acquisition is lock-free (CAS bitset in `AudioBufferPool::acquire`).

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use pks_frame::{AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, SourceId, StreamId};

use crate::macos_asp::AspReader;
use pks_capture::{
    CaptureError as LoopbackError, CaptureMode, CaptureObservationCounters, CaptureObservations,
    CaptureSampleTimeline,
};

// These paths are emitted as `cargo:rustc-env` by build.rs.
const DRIVER_DYLIB_BYTES: &[u8] = include_bytes!(env!("PKS_DRIVER_DYLIB"));
const DRIVER_PLIST_BYTES: &[u8] = include_bytes!(env!("PKS_DRIVER_PLIST"));

const DRIVER_INSTALL_DIR: &str = "/Library/Audio/Plug-Ins/HAL";
const DRIVER_BUNDLE_NAME: &str = "PocketStationLoopback.driver";

/// Milliseconds to sleep before the first poll after restarting coreaudiod.
const COREAUDIOD_RESTART_INITIAL_SLEEP_MS: u64 = 2_000;

/// Milliseconds between subsequent SHM-ring poll attempts.
const COREAUDIOD_POLL_INTERVAL_MS: u64 = 200;

/// Total number of poll attempts after the initial sleep (~12 s window, 14 s total).
const COREAUDIOD_POLL_ATTEMPTS: u32 = 60;

/// Pool depth: 8 frames absorb callback jitter without unbounded growth.
const POOL_CAPACITY_FRAMES: usize = 8;

enum Impl {
    // TapLoopbackSource is held for RAII; it stops capture on Drop.
    // The inner value is never read — only dropped.
    #[allow(dead_code)]
    Tap(crate::macos_tap::TapLoopbackSource),
    Asp {
        _thread: std::thread::JoinHandle<()>,
        stop_tx: std::sync::mpsc::SyncSender<()>,
        counters: CaptureObservationCounters,
    },
}

/// Manages a macOS loopback capture session.
///
/// On macOS 14.4+ (public support), uses the CoreAudio process tap (no HAL plugin required).
/// On older macOS, uses the PocketStation HAL plugin + POSIX SHM ring.
///
/// Drop this value to stop capture.
// The inner Impl is read-only from Rust; it is kept alive for Drop/RAII and
// accessed exclusively through the C FFI callbacks.
#[allow(dead_code)]
pub struct SystemLoopbackSource(Impl);

impl SystemLoopbackSource {
    pub fn capture<F>(callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        Self::capture_mode(CaptureMode::SystemMix, callback)
    }

    pub fn capture_mode<F>(mode: CaptureMode, mut callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        // macOS 14.4+ (public support): use the process tap path (no HAL plugin, no routing change).
        if crate::macos_tap::tap_available() {
            return crate::macos_tap::TapLoopbackSource::capture_mode(mode, callback)
                .map(|t| Self(Impl::Tap(t)));
        }

        // Older macOS: ASP fallback only supports SystemMix.
        match mode {
            CaptureMode::SystemMix => {}
            other => return Err(LoopbackError::ModeUnsupported(other)),
        }

        ensure_asp_driver_active()?;

        let mut reader = AspReader::open().ok_or_else(|| {
            LoopbackError::BackendInit(
                "SHM ring unavailable after driver install — try running again".into(),
            )
        })?;

        let channel_count = reader.channels() as u8;
        let sample_rate_hz = reader.sample_rate();
        let sample_rate_nonzero = std::num::NonZeroU32::new(sample_rate_hz).ok_or_else(|| {
            LoopbackError::BackendInit("ASP reader reported a zero sample rate".to_owned())
        })?;
        let callback_frame_count: u32 = sample_rate_hz / 50; // 20 ms
        let buffer_capacity_samples = callback_frame_count as usize * channel_count as usize;
        let pool = Arc::new(AudioBufferPool::new(
            POOL_CAPACITY_FRAMES,
            buffer_capacity_samples,
        ));

        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);
        let counters = CaptureObservationCounters::default();
        let capture_counters = counters.clone();
        let initial_drop_count = reader.drop_count();

        let thread = std::thread::Builder::new()
            .name("pks-asp-reader".into())
            .spawn(move || {
                let mut sequence_num: u64 = 0;
                let mut timeline = CaptureSampleTimeline::new(sample_rate_nonzero);
                let mut buffer = vec![0.0f32; buffer_capacity_samples];
                let mut observed_drop_count = initial_drop_count;
                loop {
                    if stop_rx.try_recv().is_ok() {
                        break;
                    }
                    let frame_count = reader.read_frames(&mut buffer, callback_frame_count);
                    let drop_count = reader.drop_count();
                    capture_counters.observe_dispatch_queue_full_frames(
                        drop_count.saturating_sub(observed_drop_count),
                    );
                    observed_drop_count = drop_count;
                    if frame_count == 0 {
                        std::thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                    capture_counters.observe_callback_buffer();
                    let timestamp_ns = timeline.advance(u64::from(frame_count));
                    let frame_sequence_number = sequence_num;
                    sequence_num = sequence_num.saturating_add(1);
                    let mut handle = match pool.acquire() {
                        Some(h) => h,
                        None => {
                            capture_counters.observe_pool_exhaustion();
                            continue;
                        }
                    };
                    let dst = handle.as_mut_slice();
                    let sample_count = frame_count as usize * channel_count as usize;
                    if sample_count > dst.len() {
                        capture_counters.observe_oversized_buffer();
                        continue;
                    }
                    dst[..sample_count].copy_from_slice(&buffer[..sample_count]);
                    handle.set_len(sample_count);
                    let mut frame = AudioFrame::new(
                        StreamId(0),
                        SourceId(0),
                        frame_sequence_number,
                        timestamp_ns,
                        channel_count,
                        handle,
                    );
                    frame.source_tag = AudioSourceTag::Captured;
                    frame.encryption_mode = EncryptionMode::None;
                    frame.sample_rate_hz = sample_rate_hz;
                    capture_counters.observe_enqueued_frame();
                    callback(frame);
                }
            })
            .map_err(|e| LoopbackError::BackendInit(format!("thread spawn: {e}")))?;

        Ok(Self(Impl::Asp {
            _thread: thread,
            stop_tx,
            counters,
        }))
    }

    pub fn observations(&self) -> CaptureObservations {
        match &self.0 {
            Impl::Tap(source) => source.observations(),
            Impl::Asp { counters, .. } => counters.snapshot(),
        }
    }
}

/// Drop contract — this is a control-thread-only owner:
///   non-blocking stop request · panic-free · allocation-free · log-free
impl Drop for SystemLoopbackSource {
    fn drop(&mut self) {
        match &self.0 {
            Impl::Tap(_) => {} // TapLoopbackSource::Drop handles stop
            Impl::Asp { stop_tx, .. } => {
                let _ = stop_tx.try_send(());
            }
        }
    }
}

/// Ensures the HAL plugin is installed and `coreaudiod` has loaded it.
fn ensure_asp_driver_active() -> Result<(), LoopbackError> {
    let install_path = format!("{DRIVER_INSTALL_DIR}/{DRIVER_BUNDLE_NAME}");

    if Path::new(&install_path).exists() {
        if !crate::macos_asp::asp_is_installed() {
            wait_for_shm_ring()?;
        }
        return Ok(());
    }

    let tmp = extract_driver_to_temp()?;

    println!(
        "PocketStation audio driver not detected.\n\
         Installing to {install_path} — you may be prompted for your password."
    );

    install_driver(&tmp, &install_path)?;
    restart_coreaudiod()?;
    wait_for_shm_ring()?;

    println!("Driver active. Ultra-low latency capture ready.");
    Ok(())
}

fn extract_driver_to_temp() -> Result<std::path::PathBuf, LoopbackError> {
    let tmp = std::env::temp_dir().join("pocketstation-driver-install");
    let macos_dir = tmp.join(DRIVER_BUNDLE_NAME).join("Contents").join("MacOS");
    std::fs::create_dir_all(&macos_dir)
        .map_err(|e| LoopbackError::BackendInit(format!("temp dir create: {e}")))?;

    std::fs::write(macos_dir.join("PocketStationLoopback"), DRIVER_DYLIB_BYTES)
        .map_err(|e| LoopbackError::BackendInit(format!("dylib write: {e}")))?;

    std::fs::write(
        tmp.join(DRIVER_BUNDLE_NAME)
            .join("Contents")
            .join("Info.plist"),
        DRIVER_PLIST_BYTES,
    )
    .map_err(|e| LoopbackError::BackendInit(format!("plist write: {e}")))?;

    set_executable(&macos_dir.join("PocketStationLoopback"))?;
    Ok(tmp.join(DRIVER_BUNDLE_NAME))
}

fn set_executable(path: &Path) -> Result<(), LoopbackError> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)
        .map_err(|e| LoopbackError::BackendInit(format!("metadata: {e}")))?
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)
        .map_err(|e| LoopbackError::BackendInit(format!("chmod: {e}")))?;
    Ok(())
}

fn install_driver(bundle: &Path, install_path: &str) -> Result<(), LoopbackError> {
    let status = std::process::Command::new("sudo")
        .args(["cp", "-R", bundle.to_str().unwrap(), DRIVER_INSTALL_DIR])
        .status()
        .map_err(|e| LoopbackError::BackendInit(format!("sudo cp: {e}")))?;

    if !status.success() {
        return Err(LoopbackError::BackendInit(
            "Driver installation cancelled or failed. \
             Ensure you have administrator privileges."
                .into(),
        ));
    }

    let installed_dylib = format!("{install_path}/Contents/MacOS/PocketStationLoopback");
    let _ = std::process::Command::new("sudo")
        .args(["chmod", "755", &installed_dylib])
        .status();

    Ok(())
}

fn restart_coreaudiod() -> Result<(), LoopbackError> {
    println!("Restarting audio daemon to load driver (audio will briefly interrupt)...");
    let status = std::process::Command::new("sudo")
        .args(["killall", "coreaudiod"])
        .status()
        .map_err(|e| LoopbackError::BackendInit(format!("killall coreaudiod: {e}")))?;

    if !status.success() && status.code() != Some(1) {
        return Err(LoopbackError::BackendInit(
            "coreaudiod restart failed — try running again with sudo".into(),
        ));
    }
    Ok(())
}

fn wait_for_shm_ring() -> Result<(), LoopbackError> {
    std::thread::sleep(Duration::from_millis(COREAUDIOD_RESTART_INITIAL_SLEEP_MS));

    for _ in 0..COREAUDIOD_POLL_ATTEMPTS {
        if crate::macos_asp::asp_is_installed() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(COREAUDIOD_POLL_INTERVAL_MS));
    }

    Err(LoopbackError::BackendInit(
        "Timed out waiting for PocketStation audio driver to become active. \
         Try running `pks source system` again, or reboot if the problem persists."
            .into(),
    ))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::assertions_on_constants)]
    use super::*;

    // Given: driver install path constant
    // When: inspected at test time
    // Then: points to the canonical HAL directory
    #[test]
    fn given_driver_install_dir_when_checked_then_is_hal_directory() {
        assert_eq!(DRIVER_INSTALL_DIR, "/Library/Audio/Plug-Ins/HAL");
    }

    // Given: driver bundle name constant
    // When: inspected
    // Then: ends with .driver extension
    #[test]
    fn given_driver_bundle_name_when_checked_then_has_driver_extension() {
        assert!(
            DRIVER_BUNDLE_NAME.ends_with(".driver"),
            "bundle name must end with .driver, got: {DRIVER_BUNDLE_NAME}"
        );
    }

    // Given: embedded driver bytes
    // When: length checked
    // Then: both are non-empty (would be zero if build.rs failed to embed)
    #[test]
    fn given_embedded_driver_bytes_when_checked_then_are_non_empty() {
        assert!(
            !DRIVER_DYLIB_BYTES.is_empty(),
            "DRIVER_DYLIB_BYTES must be non-empty"
        );
        assert!(
            !DRIVER_PLIST_BYTES.is_empty(),
            "DRIVER_PLIST_BYTES must be non-empty"
        );
    }

    // Given: embedded Info.plist
    // When: inspected as UTF-8
    // Then: contains the expected bundle identifier
    #[test]
    fn given_embedded_info_plist_when_parsed_then_contains_bundle_id() {
        let plist =
            std::str::from_utf8(DRIVER_PLIST_BYTES).expect("Info.plist must be valid UTF-8");
        assert!(
            plist.contains("io.pocketstation.loopback"),
            "Info.plist must contain the bundle identifier"
        );
    }

    // Given: restart initial sleep constant
    // When: checked
    // Then: provides enough time for coreaudiod to restart (~800 ms floor)
    #[test]
    fn given_restart_sleep_constant_when_checked_then_is_at_least_500ms() {
        assert!(
            COREAUDIOD_RESTART_INITIAL_SLEEP_MS >= 500,
            "restart sleep must be at least 500 ms to let coreaudiod initialise"
        );
    }

    // Given: no ASP plugin running in CI
    // When: asp_is_installed() is called
    // Then: returns false without panicking
    #[test]
    fn given_no_plugin_running_when_asp_is_installed_called_then_returns_false() {
        assert!(!crate::macos_asp::asp_is_installed());
    }
}

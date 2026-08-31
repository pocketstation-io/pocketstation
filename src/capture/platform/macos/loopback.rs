//! macOS audio loopback capture backend.
//!
//! On macOS 14.4+ (public support), uses `AudioHardwareCreateProcessTap` / `CATapDescription`
//! (the process tap path).  No HAL plugin installation, no Screen Recording
//! permission, no deadlock.
//!
//! On older macOS, an externally provisioned AudioServerPlugin can expose the
//! POSIX SHM ring. The SDK never installs privileged components or restarts
//! Core Audio.
//!
//! # Hot-path invariants
//!
//! No allocation, locking, logging, or panicking on the audio delivery path.
//! Pool slot acquisition is lock-free (CAS bitset in `AudioBufferPool::acquire`).

use std::time::Duration;

use crate::frame::{AudioBufferPool, AudioFrame, StreamId};

use crate::capture::platform::macos::macos_asp::AspReader;
use crate::capture::{
    CaptureError as LoopbackError, CaptureMode, CaptureObservationCounters,
    CaptureObservationHandle, CaptureObservations, CaptureSampleTimeline,
};

const ASP_BACKEND_NAME: &str = "PocketStation macOS Audio Server Plug-in";
const ASP_SETUP_ACTION: &str = "provision PocketStationLoopback.driver outside the SDK, \
    verify its shared-memory ABI is active, then retry; macOS 14.4+ can use the built-in \
    Core Audio process-tap backend";

/// Pool depth: 8 frames absorb callback jitter without unbounded growth.
const POOL_CAPACITY_FRAMES: usize = 8;

enum Impl {
    // TapLoopbackSource is held for RAII; it stops capture on Drop.
    // The inner value is never read — only dropped.
    Tap(crate::capture::platform::macos::macos_tap::TapLoopbackSource),
    Asp {
        reader_thread: Option<std::thread::JoinHandle<()>>,
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
pub struct SystemLoopbackSource(Impl);

impl SystemLoopbackSource {
    #[cfg(feature = "internal-testing")]
    pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        Self::capture_mode_with_runtime_event_sender(
            mode,
            crate::frame::AudioFrameDuration::default(),
            callback,
            None,
        )
    }

    pub(crate) fn capture_mode_with_runtime_event_sender<F>(
        mode: CaptureMode,
        audio_frame_duration: crate::frame::AudioFrameDuration,
        mut callback: F,
        runtime_event_sender: Option<crate::capture::SourceRuntimeEventSender>,
    ) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        // macOS 14.4+ (public support): use the process tap path (no HAL plugin, no routing change).
        if crate::capture::platform::macos::macos_tap::tap_available() {
            return crate::capture::platform::macos::macos_tap::TapLoopbackSource::capture_mode_with_runtime_event_sender(
                mode,
                audio_frame_duration,
                callback,
                runtime_event_sender,
            )
            .map(|t| Self(Impl::Tap(t)));
        }

        // Older macOS: ASP fallback only supports SystemMix.
        match mode {
            CaptureMode::SystemMix => {}
            other => return Err(LoopbackError::ModeUnsupported(other)),
        }

        require_asp_driver_active(crate::capture::platform::macos::macos_asp::asp_is_installed())?;

        let mut reader = AspReader::open().ok_or_else(|| {
            LoopbackError::BackendInit(
                "compatible ASP shared-memory ring disappeared during capture open".into(),
            )
        })?;

        let channel_count = reader.channels() as u8;
        let sample_rate_hz = reader.sample_rate();
        let sample_rate_nonzero = std::num::NonZeroU32::new(sample_rate_hz).ok_or_else(|| {
            LoopbackError::BackendInit("ASP reader reported a zero sample rate".to_owned())
        })?;
        let source_id = crate::capture::StableSourceId::new(
            crate::frame::Platform::Macos,
            crate::capture::SourceKind::SystemMix,
            "system:mix",
        )
        .source_id();
        let callback_frame_count =
            u32::try_from(audio_frame_duration.samples_per_channel(sample_rate_hz))
                .unwrap_or(u32::MAX)
                .max(1);
        let buffer_capacity_samples = callback_frame_count as usize * channel_count as usize;
        let pool = AudioBufferPool::new(POOL_CAPACITY_FRAMES, buffer_capacity_samples);

        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);
        let counters = CaptureObservationCounters::default();
        let capture_counters = counters.clone();
        let failure_counters = counters.clone();
        let initial_drop_count = reader.drop_count();
        let initial_timeline_reject_count = reader.timeline_reject_callback_count();

        let thread = std::thread::Builder::new()
            .name("pks-asp-reader".into())
            .spawn(move || {
                let worker = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut sequence_num: u64 = 0;
                    let mut timeline = CaptureSampleTimeline::new(sample_rate_nonzero);
                    let mut buffer = vec![0.0f32; buffer_capacity_samples];
                    let mut observed_drop_count = initial_drop_count;
                    loop {
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        let batch = reader.read_frames(&mut buffer, callback_frame_count);
                        let drop_count = reader.drop_count();
                        capture_counters.observe_dispatch_queue_full_frames(
                            drop_count.saturating_sub(observed_drop_count),
                        );
                        observed_drop_count = drop_count;
                        let timeline_reject_count = reader.timeline_reject_callback_count();
                        if timeline_reject_count != initial_timeline_reject_count {
                            capture_counters.observe_stream_error();
                            if let Some(sender) = runtime_event_sender.as_ref() {
                                let _ = crate::capture::publish_backend_failure(
                                    sender,
                                    crate::capture::StableSourceId::new(
                                        crate::frame::Platform::Macos,
                                        crate::capture::SourceKind::SystemMix,
                                        "system:mix",
                                    ),
                                    crate::capture::SourceGeneration::INITIAL,
                                    "macOS ASP reader",
                                    crate::capture::CaptureRuntimeFailureClass::BackendClass {
                                        class: "native-timeline-rejected".to_owned(),
                                    },
                                );
                            }
                            break;
                        }
                        if batch.frame_count == 0 {
                            std::thread::sleep(Duration::from_millis(1));
                            continue;
                        }
                        capture_counters.observe_callback_buffer();
                        let timestamp_ns = match timeline.advance_from_source_position(
                            batch.source_frame_position_frames,
                            u64::from(batch.frame_count),
                        ) {
                            Ok(timestamp_ns) => timestamp_ns,
                            Err(_) => {
                                capture_counters.observe_stream_error();
                                if let Some(sender) = runtime_event_sender.as_ref() {
                                    let _ = crate::capture::publish_backend_failure(
                                        sender,
                                        crate::capture::StableSourceId::new(
                                            crate::frame::Platform::Macos,
                                            crate::capture::SourceKind::SystemMix,
                                            "system:mix",
                                        ),
                                        crate::capture::SourceGeneration::INITIAL,
                                        "macOS ASP reader",
                                        crate::capture::CaptureRuntimeFailureClass::BackendClass {
                                            class: "native-timeline-invalid".to_owned(),
                                        },
                                    );
                                }
                                break;
                            }
                        };
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
                        let sample_count = batch.frame_count as usize * channel_count as usize;
                        if sample_count > dst.len() {
                            capture_counters.observe_oversized_buffer();
                            continue;
                        }
                        dst[..sample_count].copy_from_slice(&buffer[..sample_count]);
                        if handle.try_set_len(sample_count).is_err() {
                            capture_counters.observe_oversized_buffer();
                            continue;
                        }
                        let mut frame = AudioFrame::new(
                            StreamId(0),
                            source_id,
                            frame_sequence_number,
                            timestamp_ns,
                            channel_count,
                            handle,
                        );
                        frame.sample_rate_hz = sample_rate_hz;
                        capture_counters.observe_enqueued_frame();
                        callback(frame);
                    }
                }));
                if let Err(payload) = worker {
                    failure_counters.observe_stream_error();
                    if let Some(sender) = runtime_event_sender.as_ref() {
                        let _ = crate::capture::publish_backend_failure(
                            sender,
                            crate::capture::StableSourceId::new(
                                crate::frame::Platform::Macos,
                                crate::capture::SourceKind::SystemMix,
                                "system:mix",
                            ),
                            crate::capture::SourceGeneration::INITIAL,
                            "macOS ASP reader",
                            crate::capture::CaptureRuntimeFailureClass::BackendClass {
                                class: "reader-panicked".to_owned(),
                            },
                        );
                    }
                    std::panic::resume_unwind(payload);
                }
            })
            .map_err(|e| LoopbackError::BackendInit(format!("thread spawn: {e}")))?;

        Ok(Self(Impl::Asp {
            reader_thread: Some(thread),
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

    pub fn source_id(&self) -> crate::frame::SourceId {
        match &self.0 {
            Impl::Tap(source) => source.source_id(),
            Impl::Asp { .. } => crate::capture::StableSourceId::new(
                crate::frame::Platform::Macos,
                crate::capture::SourceKind::SystemMix,
                "system:mix",
            )
            .source_id(),
        }
    }

    pub fn observation_handle(&self) -> CaptureObservationHandle {
        match &self.0 {
            Impl::Tap(source) => source.observation_handle(),
            Impl::Asp { counters, .. } => counters.observation_handle(),
        }
    }

    pub fn stop_and_join(mut self) -> Result<CaptureObservations, LoopbackError> {
        self.stop_reader()
    }

    fn stop_reader(&mut self) -> Result<CaptureObservations, LoopbackError> {
        match &mut self.0 {
            Impl::Tap(source) => source.stop_and_join(),
            Impl::Asp {
                reader_thread,
                stop_tx,
                counters,
            } => {
                let counters = counters.clone();
                let _ = stop_tx.try_send(());
                if let Some(reader_thread) = reader_thread.take() {
                    crate::capture::join_capture_worker(reader_thread, "macOS ASP reader")?;
                }
                Ok(counters.snapshot())
            }
        }
    }
}

/// Drop contract — control thread only: signal and join the owned reader.
impl Drop for SystemLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_reader();
    }
}

fn require_asp_driver_active(active: bool) -> Result<(), LoopbackError> {
    if active {
        Ok(())
    } else {
        Err(LoopbackError::BackendSetupRequired {
            backend: ASP_BACKEND_NAME,
            action: ASP_SETUP_ACTION,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_active_asp_when_required_then_sdk_accepts_external_provisioning() {
        assert_eq!(require_asp_driver_active(true), Ok(()));
    }

    #[test]
    fn given_missing_asp_when_required_then_sdk_returns_actionable_typed_error() {
        assert_eq!(
            require_asp_driver_active(false),
            Err(LoopbackError::BackendSetupRequired {
                backend: ASP_BACKEND_NAME,
                action: ASP_SETUP_ACTION,
            })
        );
    }
}

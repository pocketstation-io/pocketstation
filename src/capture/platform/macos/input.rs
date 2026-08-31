//! Physical input-device capture through CoreAudio via CPAL.

use std::num::NonZeroU32;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::capture::frame_normalizer::CaptureFrameNormalizer;
use crate::capture::{
    initialize_monotonic_timestamp_domain, monotonic_timestamp_ns, CaptureError,
    CaptureObservationCounters, CaptureObservationHandle, CaptureObservations,
    CaptureRuntimeFailure, CaptureRuntimeFailureClass, CaptureSampleTimeline, CaptureSource,
    InputDeviceSelector, PermissionObservation, SourceGeneration, SourceKind, SourceRuntimeEvent,
    SourceRuntimeEventSender, SourceState, StableSourceId,
};
use crate::frame::{AudioBufferPool, AudioFrame, AudioFrameDuration, Platform, StreamId};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, SupportedBufferSize};

const QUEUE_CAPACITY_FRAMES: usize = 8;
const POOL_CAPACITY_FRAMES: usize = QUEUE_CAPACITY_FRAMES + 2;
const FALLBACK_MAX_CALLBACK_DURATION_MS: u32 = 200;

fn require_microphone_permission(permission: PermissionObservation) -> Result<(), CaptureError> {
    match permission {
        PermissionObservation::Denied
        | PermissionObservation::Restricted
        | PermissionObservation::Revoked => Err(CaptureError::PermissionDenied {
            operation: "opening the macOS microphone input stream",
        }),
        PermissionObservation::Allowed
        | PermissionObservation::NotDetermined
        | PermissionObservation::NotObservable
        | PermissionObservation::NotApplicable => Ok(()),
    }
}

struct InputCaptureTimestamp {
    timestamp_ns: u64,
    epoch_clamped: bool,
}

fn input_capture_timestamp(
    callback_observed_at_ns: u64,
    callback_info: &cpal::InputCallbackInfo,
) -> InputCaptureTimestamp {
    let timestamp = callback_info.timestamp();
    let capture_before_callback_ns = timestamp
        .callback
        .saturating_duration_since(timestamp.capture)
        .as_nanos()
        .min(u128::from(u64::MAX)) as u64;
    match callback_observed_at_ns.checked_sub(capture_before_callback_ns) {
        Some(timestamp_ns) if timestamp_ns != 0 => InputCaptureTimestamp {
            timestamp_ns,
            epoch_clamped: false,
        },
        _ => InputCaptureTimestamp {
            timestamp_ns: 1,
            epoch_clamped: true,
        },
    }
}

pub struct MacosInputSource {
    stream: Option<cpal::Stream>,
    reader_thread: Option<std::thread::JoinHandle<()>>,
    running: Arc<AtomicBool>,
    counters: CaptureObservationCounters,
    source_id: crate::frame::SourceId,
}

impl MacosInputSource {
    pub(crate) fn capture_with_runtime_event_sender<F>(
        selector: InputDeviceSelector,
        audio_frame_duration: AudioFrameDuration,
        mut callback: F,
        runtime_event_sender: Option<SourceRuntimeEventSender>,
    ) -> Result<Self, CaptureError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        require_microphone_permission(super::microphone_permission_observation())?;
        let host = cpal::default_host();
        let device = select_input_device(&host, &selector)?;
        let device_id = device
            .id()
            .map_err(|error| capture_backend_error("read input device id", error))?;
        let stable_device_id = device_id.to_string();
        let supported_config = select_f32_input_config(&device)?;
        let sample_rate_hz = supported_config.sample_rate();
        let sample_rate = NonZeroU32::new(sample_rate_hz).ok_or_else(|| {
            CaptureError::BackendInit("input device sample rate is zero".to_owned())
        })?;
        let channels = u8::try_from(supported_config.channels())
            .ok()
            .filter(|channels| *channels > 0)
            .ok_or_else(|| {
                CaptureError::BackendInit("input device channel count is invalid".to_owned())
            })?;
        let target_callback_frames =
            u32::try_from(audio_frame_duration.samples_per_channel(sample_rate_hz))
                .unwrap_or(u32::MAX)
                .max(1);
        let fallback_max_callback_frames =
            (sample_rate_hz / (1_000 / FALLBACK_MAX_CALLBACK_DURATION_MS)).max(1);
        let mut stream_config = supported_config.config();
        let maximum_callback_frames = match supported_config.buffer_size() {
            SupportedBufferSize::Range { min, max }
                if target_callback_frames >= *min && target_callback_frames <= *max =>
            {
                stream_config.buffer_size = BufferSize::Fixed(target_callback_frames);
                fallback_max_callback_frames.max(*min).min(*max)
            }
            SupportedBufferSize::Range { min, max } => {
                fallback_max_callback_frames.max(*min).min(*max)
            }
            SupportedBufferSize::Unknown => fallback_max_callback_frames,
        };
        let maximum_callback_samples = usize::try_from(maximum_callback_frames)
            .ok()
            .and_then(|frames| frames.checked_mul(usize::from(channels)))
            .ok_or_else(|| CaptureError::BackendInit("input pool size overflow".to_owned()))?;
        let mut frame_normalizer = CaptureFrameNormalizer::new(
            usize::try_from(target_callback_frames).unwrap_or(usize::MAX),
            channels,
            sample_rate_hz,
        );
        let pool =
            AudioBufferPool::new(POOL_CAPACITY_FRAMES, frame_normalizer.frame_sample_count());
        let (mut producer, mut consumer) = rtrb::RingBuffer::new(QUEUE_CAPACITY_FRAMES);
        let running = Arc::new(AtomicBool::new(true));
        let counters = CaptureObservationCounters::default();
        initialize_monotonic_timestamp_domain();
        let stable_id =
            StableSourceId::new(Platform::Macos, SourceKind::InputDevice, stable_device_id);
        let source_id = stable_id.source_id();
        let callback_pool = Arc::clone(&pool);
        let callback_counters = counters.clone();
        let mut sequence_number = 0u64;
        let mut sample_timeline = None;
        let data_callback = move |data: &[f32], callback_info: &cpal::InputCallbackInfo| {
            callback_counters.observe_callback_buffer();
            if data.len() > maximum_callback_samples {
                callback_counters.observe_oversized_buffer();
                return;
            }
            let samples_per_channel = data.len() / usize::from(channels);
            let timeline = sample_timeline.get_or_insert_with(|| {
                let timestamp = input_capture_timestamp(monotonic_timestamp_ns(), callback_info);
                if timestamp.epoch_clamped {
                    callback_counters.observe_timestamp_epoch_clamp();
                }
                CaptureSampleTimeline::anchored(sample_rate, timestamp.timestamp_ns)
            });
            let timestamp_ns =
                timeline.advance(u64::try_from(samples_per_channel).unwrap_or(u64::MAX));
            let normalized = frame_normalizer.push(data, timestamp_ns, |timestamp_ns, samples| {
                let frame_sequence_number = sequence_number;
                sequence_number = sequence_number.saturating_add(1);
                let Some(mut handle) = callback_pool.acquire() else {
                    callback_counters.observe_pool_exhaustion();
                    return;
                };
                if handle.try_copy_from_slice(samples).is_err() {
                    callback_counters.observe_oversized_buffer();
                    return;
                }
                let mut frame = AudioFrame::new(
                    StreamId(source_id.0),
                    source_id,
                    frame_sequence_number,
                    timestamp_ns,
                    channels,
                    handle,
                );
                frame.sample_rate_hz = sample_rate_hz;
                if producer.push(frame).is_err() {
                    callback_counters.observe_dispatch_queue_full();
                    return;
                }
                callback_counters.observe_enqueued_frame();
            });
            if !normalized {
                callback_counters.observe_invalid_buffer();
            }
        };
        let error_counters = counters.clone();
        let mut runtime_failure_event =
            runtime_event_sender
                .as_ref()
                .map(|_| SourceRuntimeEvent::BackendFailure {
                    stable_id,
                    generation: SourceGeneration::INITIAL,
                    failure: CaptureRuntimeFailure {
                        operation: "macOS input stream callback",
                        error_class: CaptureRuntimeFailureClass::BackendClass {
                            class: "cpal-stream-error".to_owned(),
                        },
                    },
                });
        let error_callback = move |_error: cpal::Error| {
            error_counters.observe_stream_error();
            if let (Some(sender), Some(event)) =
                (runtime_event_sender.as_ref(), runtime_failure_event.take())
            {
                let _ = sender.try_send(event);
            }
        };
        let stream = device
            .build_input_stream(stream_config, data_callback, error_callback, None)
            .map_err(|error| capture_backend_error("build input stream", error))?;

        let reader_running = Arc::clone(&running);
        let reader_thread = std::thread::Builder::new()
            .name("pks-input-reader".to_owned())
            .spawn(move || {
                while reader_running.load(Ordering::Acquire) {
                    match consumer.pop() {
                        Ok(frame) => callback(frame),
                        Err(_) => std::thread::sleep(Duration::from_millis(1)),
                    }
                }
                while let Ok(frame) = consumer.pop() {
                    callback(frame);
                }
            })
            .map_err(|error| CaptureError::BackendInit(format!("input reader thread: {error}")))?;

        if let Err(error) = stream.play() {
            running.store(false, Ordering::Release);
            let _ = reader_thread.join();
            return Err(capture_backend_error("start input stream", error));
        }

        Ok(Self {
            stream: Some(stream),
            reader_thread: Some(reader_thread),
            running,
            counters,
            source_id,
        })
    }

    pub fn source_id(&self) -> crate::frame::SourceId {
        self.source_id
    }

    pub fn observations(&self) -> CaptureObservations {
        self.counters.snapshot()
    }

    pub fn observation_handle(&self) -> CaptureObservationHandle {
        self.counters.observation_handle()
    }

    pub fn stop_and_join(mut self) -> Result<CaptureObservations, CaptureError> {
        let counters = self.counters.clone();
        self.stop_reader()?;
        Ok(counters.snapshot())
    }

    fn stop_reader(&mut self) -> Result<(), CaptureError> {
        self.running.store(false, Ordering::Release);
        self.stream.take();
        self.reader_thread.take().map_or(Ok(()), |thread| {
            crate::capture::join_capture_worker(thread, "macOS input reader")
        })
    }
}

impl Drop for MacosInputSource {
    fn drop(&mut self) {
        let _ = self.stop_reader();
    }
}

pub fn discover_input_sources_native() -> Vec<CaptureSource> {
    let host = cpal::default_host();
    let default_id = host
        .default_input_device()
        .and_then(|device| device.id().ok())
        .map(|id| id.to_string());
    let Ok(devices) = host.input_devices() else {
        return Vec::new();
    };
    let mut sources = devices
        .filter_map(|device| {
            let id = device.id().ok()?.to_string();
            let description = device.description().ok()?;
            let config = select_f32_input_config(&device).ok()?;
            Some(CaptureSource {
                stable_id: StableSourceId::new(
                    Platform::Macos,
                    SourceKind::InputDevice,
                    id.clone(),
                ),
                name: description.name().to_owned(),
                process_id: None,
                app_id: None,
                device_uid: Some(id),
                state: SourceState::Available,
                sample_rate_hz: config.sample_rate(),
                channels: config.channels(),
            })
        })
        .collect::<Vec<_>>();
    sources.sort_by_key(|source| {
        let is_default = default_id.as_deref() == source.device_uid.as_deref();
        (!is_default, source.name.clone())
    });
    sources
}

fn select_input_device(
    host: &cpal::Host,
    selector: &InputDeviceSelector,
) -> Result<cpal::Device, CaptureError> {
    match selector {
        InputDeviceSelector::Default => host.default_input_device().ok_or_else(|| {
            CaptureError::BackendInit("no default physical input device is available".to_owned())
        }),
        InputDeviceSelector::StableId(expected_id) => host
            .input_devices()
            .map_err(|error| capture_backend_error("enumerate input devices", error))?
            .find(|device| {
                device
                    .id()
                    .is_ok_and(|device_id| device_id.to_string() == *expected_id)
            })
            .ok_or_else(|| {
                CaptureError::BackendInit(format!(
                    "physical input device is unavailable: {expected_id}"
                ))
            }),
    }
}

fn select_f32_input_config(
    device: &cpal::Device,
) -> Result<cpal::SupportedStreamConfig, CaptureError> {
    let configs = device
        .supported_input_configs()
        .map_err(|error| capture_backend_error("query input formats", error))?
        .filter(|config| config.sample_format() == SampleFormat::F32)
        .collect::<Vec<_>>();
    configs
        .iter()
        .copied()
        .filter_map(|config| config.try_with_sample_rate(48_000))
        .max_by_key(|config| config.channels() == 1)
        .or_else(|| {
            configs
                .iter()
                .copied()
                .filter_map(cpal::SupportedStreamConfigRange::try_with_standard_sample_rate)
                .max_by_key(|config| config.channels() == 1)
        })
        .ok_or_else(|| {
            CaptureError::BackendInit(
                "input device exposes no supported f32 48 kHz or 44.1 kHz format".to_owned(),
            )
        })
}

fn capture_backend_error(context: &str, error: impl std::fmt::Display) -> CaptureError {
    CaptureError::BackendInit(format!("{context}: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpal::{InputCallbackInfo, InputStreamTimestamp, StreamInstant};

    #[test]
    fn given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay() {
        let callback_info = InputCallbackInfo::new(InputStreamTimestamp {
            callback: StreamInstant::new(10, 20_000_000),
            capture: StreamInstant::new(10, 0),
        });

        let timestamp = input_capture_timestamp(1_000_000_000, &callback_info);

        assert_eq!(timestamp.timestamp_ns, 980_000_000);
        assert!(!timestamp.epoch_clamped);
    }

    #[test]
    fn given_capture_before_process_epoch_when_mapped_then_timestamp_is_earliest_representable() {
        let callback_info = InputCallbackInfo::new(InputStreamTimestamp {
            callback: StreamInstant::new(10, 40_000_000),
            capture: StreamInstant::new(10, 0),
        });

        let timestamp = input_capture_timestamp(20_000_000, &callback_info);

        assert_eq!(timestamp.timestamp_ns, 1);
        assert!(timestamp.epoch_clamped);
    }

    #[test]
    fn given_denied_permission_when_opening_input_then_capture_fails_closed() {
        for permission in [
            PermissionObservation::Denied,
            PermissionObservation::Restricted,
            PermissionObservation::Revoked,
        ] {
            assert_eq!(
                require_microphone_permission(permission),
                Err(CaptureError::PermissionDenied {
                    operation: "opening the macOS microphone input stream",
                })
            );
        }
    }

    #[test]
    fn given_promptable_or_observable_permission_when_opening_input_then_native_open_decides() {
        for permission in [
            PermissionObservation::Allowed,
            PermissionObservation::NotDetermined,
            PermissionObservation::NotObservable,
            PermissionObservation::NotApplicable,
        ] {
            assert_eq!(require_microphone_permission(permission), Ok(()));
        }
    }
}

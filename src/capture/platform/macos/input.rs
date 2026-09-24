//! Physical input-device capture through CoreAudio via CPAL.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::capture::frame_normalizer::CaptureFrameNormalizer;
use crate::capture::platform::macos::input_format::{
    decode_interleaved_to_mono, maximum_resampled_frames, select_input_config,
    StreamingLinearResampler, CANONICAL_INPUT_CHANNEL_COUNT, CANONICAL_INPUT_SAMPLE_RATE_HZ,
};
use crate::capture::{
    initialize_monotonic_timestamp_domain, monotonic_timestamp_ns, CaptureError,
    CaptureNativeFormat, CaptureObservationCounters, CaptureObservationHandle, CaptureObservations,
    CaptureRuntimeFailure, CaptureRuntimeFailureClass, CaptureSource, InputDeviceSelector,
    PermissionObservation, SourceGeneration, SourceKind, SourceRuntimeEvent,
    SourceRuntimeEventSender, SourceState, StableSourceId,
};
use crate::frame::{AudioBufferPool, AudioFrame, AudioFrameDuration, Platform, StreamId};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{ErrorKind, SupportedBufferSize};
use rtrb::PushError;

const QUEUE_CAPACITY_FRAMES: usize = 8;
const POOL_CAPACITY_FRAMES: usize = QUEUE_CAPACITY_FRAMES + 2;
const FALLBACK_MAX_CALLBACK_DURATION_MS: u32 = 200;
const CPAL_ERROR_CLASS_CAPACITY: usize = 32;

fn cpal_error_class(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::DeviceBusy => "cpal-device-busy",
        ErrorKind::DeviceChanged => "cpal-device-changed",
        ErrorKind::DeviceNotAvailable => "cpal-device-not-available",
        ErrorKind::HostUnavailable => "cpal-host-unavailable",
        ErrorKind::InvalidInput => "cpal-invalid-input",
        ErrorKind::PermissionDenied => "cpal-permission-denied",
        ErrorKind::RealtimeDenied => "cpal-realtime-denied",
        ErrorKind::ResourceExhausted => "cpal-resource-exhausted",
        ErrorKind::StreamInvalidated => "cpal-stream-invalidated",
        ErrorKind::UnsupportedConfig => "cpal-unsupported-config",
        ErrorKind::UnsupportedOperation => "cpal-unsupported-operation",
        ErrorKind::Xrun => "cpal-xrun",
        ErrorKind::BackendError => "cpal-backend-error",
        ErrorKind::Other => "cpal-other",
        _ => "cpal-unrecognized-error",
    }
}

fn cpal_stream_continues(kind: ErrorKind) -> bool {
    matches!(
        kind,
        ErrorKind::DeviceChanged | ErrorKind::RealtimeDenied | ErrorKind::Xrun
    )
}

fn cpal_stream_has_discontinuity(kind: ErrorKind) -> bool {
    matches!(kind, ErrorKind::DeviceChanged | ErrorKind::Xrun)
}

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

struct NativeInputPacket {
    storage: Box<[u8]>,
    length_bytes: usize,
    timestamp_ns: u64,
    starts_after_discontinuity: bool,
}

impl NativeInputPacket {
    fn new(capacity_bytes: usize) -> Self {
        Self {
            storage: vec![0; capacity_bytes].into_boxed_slice(),
            length_bytes: 0,
            timestamp_ns: 1,
            starts_after_discontinuity: false,
        }
    }

    fn bytes(&self) -> &[u8] {
        &self.storage[..self.length_bytes]
    }
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
    native_format: CaptureNativeFormat,
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
        let selected_format = select_input_config(
            device
                .supported_input_configs()
                .map_err(|error| capture_backend_error("query input formats", error))?,
        )
        .ok_or_else(|| {
            CaptureError::BackendInit(
                "input device exposes no supported PCM input format".to_owned(),
            )
        })?;
        let native_format = selected_format.native;
        let native_sample_rate_hz = native_format.sample_rate_hz;
        let native_channels = u8::try_from(native_format.channel_count)
            .ok()
            .filter(|channels| *channels > 0)
            .ok_or_else(|| {
                CaptureError::BackendInit("input device channel count is invalid".to_owned())
            })?;
        let target_frame_samples =
            u32::try_from(audio_frame_duration.samples_per_channel(CANONICAL_INPUT_SAMPLE_RATE_HZ))
                .unwrap_or(u32::MAX)
                .max(1);
        let fallback_max_callback_frames =
            (native_sample_rate_hz / (1_000 / FALLBACK_MAX_CALLBACK_DURATION_MS)).max(1);
        let stream_config = selected_format.config.config();
        let native_sample_format = selected_format.config.sample_format();
        let maximum_callback_frames = match selected_format.config.buffer_size() {
            SupportedBufferSize::Range { min, max } => {
                fallback_max_callback_frames.max(*min).min(*max)
            }
            SupportedBufferSize::Unknown => fallback_max_callback_frames,
        };
        let maximum_callback_samples = usize::try_from(maximum_callback_frames)
            .ok()
            .and_then(|frames| frames.checked_mul(usize::from(native_channels)))
            .ok_or_else(|| CaptureError::BackendInit("input pool size overflow".to_owned()))?;
        let maximum_callback_bytes = maximum_callback_samples
            .checked_mul(selected_format.sample_size_bytes)
            .ok_or_else(|| CaptureError::BackendInit("input packet size overflow".to_owned()))?;
        let maximum_native_frames = maximum_callback_samples / usize::from(native_channels);
        let maximum_canonical_frames =
            maximum_resampled_frames(maximum_native_frames, native_sample_rate_hz).ok_or_else(
                || CaptureError::BackendInit("input resampling pool size overflow".to_owned()),
            )?;
        let frame_normalizer = CaptureFrameNormalizer::new(
            usize::try_from(target_frame_samples).unwrap_or(usize::MAX),
            CANONICAL_INPUT_CHANNEL_COUNT,
            CANONICAL_INPUT_SAMPLE_RATE_HZ,
        );
        let frame_pool =
            AudioBufferPool::new(POOL_CAPACITY_FRAMES, frame_normalizer.frame_sample_count());
        let (mut packet_producer, packet_consumer) = rtrb::RingBuffer::new(QUEUE_CAPACITY_FRAMES);
        let (mut free_packet_producer, mut free_packet_consumer) =
            rtrb::RingBuffer::new(POOL_CAPACITY_FRAMES);
        for _ in 0..POOL_CAPACITY_FRAMES {
            free_packet_producer
                .push(NativeInputPacket::new(maximum_callback_bytes))
                .map_err(|_| {
                    CaptureError::BackendInit("input packet pool initialization failed".to_owned())
                })?;
        }
        let running = Arc::new(AtomicBool::new(true));
        let counters = CaptureObservationCounters::default();
        initialize_monotonic_timestamp_domain();
        let stable_id =
            StableSourceId::new(Platform::Macos, SourceKind::InputDevice, stable_device_id);
        let source_id = stable_id.source_id();
        let callback_counters = counters.clone();
        let callback_discontinuity_pending = Arc::new(AtomicBool::new(false));
        let error_discontinuity_pending = Arc::clone(&callback_discontinuity_pending);
        let mut pending_packet = None;
        let data_callback = move |data: &cpal::Data, callback_info: &cpal::InputCallbackInfo| {
            callback_counters.observe_callback_buffer();
            if let Some(packet) = pending_packet.take() {
                if let Err(PushError::Full(packet)) = packet_producer.push(packet) {
                    pending_packet = Some(packet);
                    callback_counters.observe_dispatch_queue_full();
                    callback_discontinuity_pending.store(true, Ordering::Release);
                    return;
                }
            }
            let bytes = data.bytes();
            if data.sample_format() != native_sample_format
                || data.len() > maximum_callback_samples
                || bytes.len() > maximum_callback_bytes
            {
                callback_counters.observe_oversized_buffer();
                callback_discontinuity_pending.store(true, Ordering::Release);
                return;
            }
            let Ok(mut packet) = free_packet_consumer.pop() else {
                callback_counters.observe_pool_exhaustion();
                callback_discontinuity_pending.store(true, Ordering::Release);
                return;
            };
            packet.storage[..bytes.len()].copy_from_slice(bytes);
            packet.length_bytes = bytes.len();
            let timestamp = input_capture_timestamp(monotonic_timestamp_ns(), callback_info);
            if timestamp.epoch_clamped {
                callback_counters.observe_timestamp_epoch_clamp();
            }
            packet.timestamp_ns = timestamp.timestamp_ns;
            packet.starts_after_discontinuity =
                callback_discontinuity_pending.swap(false, Ordering::AcqRel);
            if let Err(PushError::Full(packet)) = packet_producer.push(packet) {
                pending_packet = Some(packet);
                callback_counters.observe_dispatch_queue_full();
                callback_discontinuity_pending.store(true, Ordering::Release);
            }
        };
        let error_counters = counters.clone();
        let mut runtime_failure_event = runtime_event_sender.as_ref().map(|_| {
            let mut class = String::with_capacity(CPAL_ERROR_CLASS_CAPACITY);
            class.push_str("cpal-unrecognized-error");
            SourceRuntimeEvent::BackendFailure {
                stable_id,
                generation: SourceGeneration::INITIAL,
                failure: CaptureRuntimeFailure {
                    operation: "macOS input stream callback",
                    error_class: CaptureRuntimeFailureClass::BackendClass { class },
                },
            }
        });
        let error_callback = move |error: cpal::Error| {
            error_counters.observe_stream_error();
            let error_kind = error.kind();
            if cpal_stream_has_discontinuity(error_kind) {
                error_discontinuity_pending.store(true, Ordering::Release);
            }
            if cpal_stream_continues(error_kind) {
                return;
            }
            if let Some(SourceRuntimeEvent::BackendFailure { failure, .. }) =
                runtime_failure_event.as_mut()
            {
                let CaptureRuntimeFailureClass::BackendClass { class } = &mut failure.error_class
                else {
                    return;
                };
                class.clear();
                class.push_str(cpal_error_class(error_kind));
            }
            if let (Some(sender), Some(event)) =
                (runtime_event_sender.as_ref(), runtime_failure_event.take())
            {
                let _ = sender.try_send(event);
            }
        };
        let stream = device
            .build_input_stream_raw(
                stream_config,
                native_sample_format,
                data_callback,
                error_callback,
                None,
            )
            .map_err(|error| capture_backend_error("build input stream", error))?;

        let reader_running = Arc::clone(&running);
        let reader_counters = counters.clone();
        let reader_thread = std::thread::Builder::new()
            .name("pks-input-reader".to_owned())
            .spawn(move || {
                let mut packet_consumer = packet_consumer;
                let mut frame_normalizer = frame_normalizer;
                let mut rate_converter = StreamingLinearResampler::new(
                    native_sample_rate_hz,
                    CANONICAL_INPUT_SAMPLE_RATE_HZ,
                );
                let mut native_mono = vec![0.0; maximum_native_frames].into_boxed_slice();
                let mut canonical_mono = vec![0.0; maximum_canonical_frames].into_boxed_slice();
                let mut sequence_number = 0u64;
                let mut conversion_discontinuity_pending = false;
                let mut process_packet = |packet: &NativeInputPacket| {
                    if packet.starts_after_discontinuity || conversion_discontinuity_pending {
                        frame_normalizer.reset();
                        rate_converter.reset();
                        sequence_number = sequence_number.saturating_add(1);
                        conversion_discontinuity_pending = false;
                    }
                    let native_frame_count = match decode_interleaved_to_mono(
                        packet.bytes(),
                        native_format,
                        &mut native_mono,
                    ) {
                        Ok(frame_count) => frame_count,
                        Err(_) => {
                            reader_counters.observe_invalid_buffer();
                            conversion_discontinuity_pending = true;
                            return;
                        }
                    };
                    let canonical_frame_count = match rate_converter
                        .process(&native_mono[..native_frame_count], &mut canonical_mono)
                    {
                        Ok(frame_count) => frame_count,
                        Err(_) => {
                            reader_counters.observe_invalid_buffer();
                            conversion_discontinuity_pending = true;
                            return;
                        }
                    };
                    let normalized = frame_normalizer.push(
                        &canonical_mono[..canonical_frame_count],
                        packet.timestamp_ns,
                        |timestamp_ns, samples| {
                            let frame_sequence_number = sequence_number;
                            sequence_number = sequence_number.saturating_add(1);
                            let Some(mut handle) = frame_pool.acquire() else {
                                reader_counters.observe_pool_exhaustion();
                                return;
                            };
                            if handle.try_copy_from_slice(samples).is_err() {
                                reader_counters.observe_oversized_buffer();
                                return;
                            }
                            let frame = AudioFrame::new(
                                StreamId(source_id.0),
                                source_id,
                                frame_sequence_number,
                                timestamp_ns,
                                CANONICAL_INPUT_CHANNEL_COUNT,
                                handle,
                            );
                            callback(frame);
                            reader_counters.observe_enqueued_frame();
                        },
                    );
                    if !normalized {
                        reader_counters.observe_invalid_buffer();
                        conversion_discontinuity_pending = true;
                    }
                };
                while reader_running.load(Ordering::Acquire) {
                    match packet_consumer.pop() {
                        Ok(packet) => {
                            process_packet(&packet);
                            if let Err(PushError::Full(_)) = free_packet_producer.push(packet) {
                                reader_counters.observe_invalid_buffer();
                            }
                        }
                        Err(_) => std::thread::sleep(Duration::from_millis(1)),
                    }
                }
                while let Ok(packet) = packet_consumer.pop() {
                    process_packet(&packet);
                    if let Err(PushError::Full(_)) = free_packet_producer.push(packet) {
                        reader_counters.observe_invalid_buffer();
                    }
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
            native_format,
        })
    }

    pub fn source_id(&self) -> crate::frame::SourceId {
        self.source_id
    }

    pub fn native_format(&self) -> CaptureNativeFormat {
        self.native_format
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
            let config = select_input_config(device.supported_input_configs().ok()?)?;
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
                sample_rate_hz: config.native.sample_rate_hz,
                channels: config.native.channel_count,
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

fn capture_backend_error(context: &str, error: impl std::fmt::Display) -> CaptureError {
    CaptureError::BackendInit(format!("{context}: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpal::{InputCallbackInfo, InputStreamTimestamp, StreamInstant};

    #[test]
    fn given_cpal_error_kinds_when_classified_then_names_fit_preallocated_storage() {
        let cases = [
            (ErrorKind::DeviceBusy, "cpal-device-busy"),
            (ErrorKind::DeviceChanged, "cpal-device-changed"),
            (ErrorKind::DeviceNotAvailable, "cpal-device-not-available"),
            (ErrorKind::HostUnavailable, "cpal-host-unavailable"),
            (ErrorKind::InvalidInput, "cpal-invalid-input"),
            (ErrorKind::PermissionDenied, "cpal-permission-denied"),
            (ErrorKind::RealtimeDenied, "cpal-realtime-denied"),
            (ErrorKind::ResourceExhausted, "cpal-resource-exhausted"),
            (ErrorKind::StreamInvalidated, "cpal-stream-invalidated"),
            (ErrorKind::UnsupportedConfig, "cpal-unsupported-config"),
            (
                ErrorKind::UnsupportedOperation,
                "cpal-unsupported-operation",
            ),
            (ErrorKind::Xrun, "cpal-xrun"),
            (ErrorKind::BackendError, "cpal-backend-error"),
            (ErrorKind::Other, "cpal-other"),
        ];

        for (kind, expected) in cases {
            assert_eq!(cpal_error_class(kind), expected);
            assert!(expected.len() <= CPAL_ERROR_CLASS_CAPACITY);
        }
        assert!("cpal-unrecognized-error".len() <= CPAL_ERROR_CLASS_CAPACITY);
    }

    #[test]
    fn given_nonfatal_cpal_notifications_when_classified_then_stream_stays_active() {
        assert!(cpal_stream_continues(ErrorKind::DeviceChanged));
        assert!(cpal_stream_continues(ErrorKind::RealtimeDenied));
        assert!(cpal_stream_continues(ErrorKind::Xrun));
        assert!(cpal_stream_has_discontinuity(ErrorKind::DeviceChanged));
        assert!(!cpal_stream_has_discontinuity(ErrorKind::RealtimeDenied));
        assert!(cpal_stream_has_discontinuity(ErrorKind::Xrun));
        assert!(!cpal_stream_continues(ErrorKind::DeviceNotAvailable));
        assert!(!cpal_stream_continues(ErrorKind::StreamInvalidated));
    }

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

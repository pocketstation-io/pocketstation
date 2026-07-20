//! Physical input-device capture through CoreAudio via CPAL.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, SupportedBufferSize};
use pks_capture::{
    monotonic_timestamp_ns, CaptureError, CaptureSource, InputDeviceSelector, SourceKind,
    SourceState, StableSourceId,
};
use pks_frame::{AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, Platform, StreamId};

const QUEUE_CAPACITY_FRAMES: usize = 8;
const POOL_CAPACITY_FRAMES: usize = QUEUE_CAPACITY_FRAMES + 2;
const TARGET_FRAME_DURATION_MS: u32 = 20;
const FALLBACK_MAX_CALLBACK_DURATION_MS: u32 = 200;

fn input_capture_timestamp_ns(
    callback_observed_at_ns: u64,
    callback_info: &cpal::InputCallbackInfo,
) -> u64 {
    let timestamp = callback_info.timestamp();
    let capture_before_callback_ns = timestamp
        .callback
        .saturating_duration_since(timestamp.capture)
        .as_nanos()
        .min(u128::from(u64::MAX)) as u64;
    callback_observed_at_ns.saturating_sub(capture_before_callback_ns)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InputCaptureObservations {
    pub frames_captured_total: u64,
    pub pool_exhausted_total: u64,
    pub queue_full_total: u64,
    pub oversized_callback_total: u64,
    pub stream_errors_total: u64,
}

#[derive(Default)]
struct InputCaptureCounters {
    frames_captured_total: AtomicU64,
    pool_exhausted_total: AtomicU64,
    queue_full_total: AtomicU64,
    oversized_callback_total: AtomicU64,
    stream_errors_total: AtomicU64,
}

impl InputCaptureCounters {
    fn snapshot(&self) -> InputCaptureObservations {
        InputCaptureObservations {
            frames_captured_total: self.frames_captured_total.load(Ordering::Relaxed),
            pool_exhausted_total: self.pool_exhausted_total.load(Ordering::Relaxed),
            queue_full_total: self.queue_full_total.load(Ordering::Relaxed),
            oversized_callback_total: self.oversized_callback_total.load(Ordering::Relaxed),
            stream_errors_total: self.stream_errors_total.load(Ordering::Relaxed),
        }
    }
}

pub struct MacosInputSource {
    stream: Option<cpal::Stream>,
    reader_thread: Option<std::thread::JoinHandle<()>>,
    running: Arc<AtomicBool>,
    counters: Arc<InputCaptureCounters>,
}

impl MacosInputSource {
    pub fn capture<F>(selector: InputDeviceSelector, mut callback: F) -> Result<Self, CaptureError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        let host = cpal::default_host();
        let device = select_input_device(&host, &selector)?;
        let device_id = device
            .id()
            .map_err(|error| capture_backend_error("read input device id", error))?;
        let stable_device_id = device_id.to_string();
        let supported_config = select_f32_input_config(&device)?;
        let sample_rate_hz = supported_config.sample_rate();
        let channels = u8::try_from(supported_config.channels())
            .ok()
            .filter(|channels| *channels > 0)
            .ok_or_else(|| {
                CaptureError::BackendInit("input device channel count is invalid".to_owned())
            })?;
        let target_callback_frames = (sample_rate_hz / (1_000 / TARGET_FRAME_DURATION_MS)).max(1);
        let fallback_max_callback_frames =
            (sample_rate_hz / (1_000 / FALLBACK_MAX_CALLBACK_DURATION_MS)).max(1);
        let mut stream_config = supported_config.config();
        let slot_frames = match supported_config.buffer_size() {
            SupportedBufferSize::Range { min, max }
                if target_callback_frames >= *min && target_callback_frames <= *max =>
            {
                stream_config.buffer_size = BufferSize::Fixed(target_callback_frames);
                target_callback_frames
            }
            SupportedBufferSize::Range { min, max } => {
                fallback_max_callback_frames.max(*min).min(*max)
            }
            SupportedBufferSize::Unknown => fallback_max_callback_frames,
        };
        let slot_samples = usize::try_from(slot_frames)
            .ok()
            .and_then(|frames| frames.checked_mul(usize::from(channels)))
            .ok_or_else(|| CaptureError::BackendInit("input pool size overflow".to_owned()))?;
        let pool = Arc::new(AudioBufferPool::new(POOL_CAPACITY_FRAMES, slot_samples));
        let (mut producer, mut consumer) = rtrb::RingBuffer::new(QUEUE_CAPACITY_FRAMES);
        let running = Arc::new(AtomicBool::new(true));
        let counters = Arc::new(InputCaptureCounters::default());
        let frame_source_id =
            StableSourceId::new(Platform::Macos, SourceKind::InputDevice, stable_device_id)
                .to_frame_source_id();
        let callback_pool = Arc::clone(&pool);
        let callback_counters = Arc::clone(&counters);
        let mut sequence_number = 0u64;
        let data_callback = move |data: &[f32], callback_info: &cpal::InputCallbackInfo| {
            let timestamp_ns = input_capture_timestamp_ns(monotonic_timestamp_ns(), callback_info);
            let frame_sequence_number = sequence_number;
            sequence_number = sequence_number.saturating_add(1);
            if data.len() > callback_pool.slot_size() {
                callback_counters
                    .oversized_callback_total
                    .fetch_add(1, Ordering::Relaxed);
                return;
            }
            let Some(mut handle) = callback_pool.acquire() else {
                callback_counters
                    .pool_exhausted_total
                    .fetch_add(1, Ordering::Relaxed);
                return;
            };
            handle.as_mut_slice()[..data.len()].copy_from_slice(data);
            handle.set_len(data.len());
            let mut frame = AudioFrame::new(
                StreamId(frame_source_id.0),
                frame_source_id,
                frame_sequence_number,
                timestamp_ns,
                channels,
                handle,
            );
            frame.source_tag = AudioSourceTag::Captured;
            frame.encryption_mode = EncryptionMode::None;
            frame.sample_rate_hz = sample_rate_hz;
            if producer.push(frame).is_err() {
                callback_counters
                    .queue_full_total
                    .fetch_add(1, Ordering::Relaxed);
                return;
            }
            callback_counters
                .frames_captured_total
                .fetch_add(1, Ordering::Relaxed);
        };
        let error_counters = Arc::clone(&counters);
        let error_callback = move |_| {
            error_counters
                .stream_errors_total
                .fetch_add(1, Ordering::Relaxed);
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
        })
    }

    pub fn observations(&self) -> InputCaptureObservations {
        self.counters.snapshot()
    }
}

impl Drop for MacosInputSource {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Release);
        self.stream.take();
        if let Some(reader_thread) = self.reader_thread.take() {
            let _ = reader_thread.join();
        }
    }
}

pub fn discover_input_sources_native() -> Vec<CaptureSource> {
    let host = cpal::default_host();
    let diagnostics_enabled = std::env::var_os("PKS_INPUT_DISCOVERY_DIAG").is_some();
    let default_id = host
        .default_input_device()
        .and_then(|device| device.id().ok())
        .map(|id| id.to_string());
    if diagnostics_enabled {
        eprintln!(
            "input_discovery_diag: default_device_id={}",
            default_id.as_deref().unwrap_or("none")
        );
    }
    let devices = match host.input_devices() {
        Ok(devices) => devices,
        Err(error) => {
            if diagnostics_enabled {
                eprintln!("input_discovery_diag: enumerate_error={error}");
            }
            return Vec::new();
        }
    };
    let mut sources = Vec::new();
    for device in devices {
        let id = match device.id() {
            Ok(id) => id.to_string(),
            Err(error) => {
                if diagnostics_enabled {
                    eprintln!("input_discovery_diag: device_id_error={error}");
                }
                continue;
            }
        };
        let description = match device.description() {
            Ok(description) => description,
            Err(error) => {
                if diagnostics_enabled {
                    eprintln!("input_discovery_diag: device_id={id} description_error={error}");
                }
                continue;
            }
        };
        let config = match select_f32_input_config(&device) {
            Ok(config) => config,
            Err(error) => {
                if diagnostics_enabled {
                    eprintln!("input_discovery_diag: device_id={id} config_error={error}");
                }
                continue;
            }
        };
        if diagnostics_enabled {
            eprintln!(
                "input_discovery_diag: device_id={id} name={} sample_rate_hz={} channels={} default={}",
                description.name(),
                config.sample_rate(),
                config.channels(),
                default_id.as_deref() == Some(id.as_str())
            );
        }
        sources.push(CaptureSource {
            stable_id: StableSourceId::new(Platform::Macos, SourceKind::InputDevice, id.clone()),
            name: description.name().to_owned(),
            process_id: None,
            app_id: None,
            device_uid: Some(id),
            state: SourceState::Available,
            sample_rate_hz: config.sample_rate(),
            channels: config.channels(),
        });
    }
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

        assert_eq!(
            input_capture_timestamp_ns(1_000_000_000, &callback_info),
            980_000_000
        );
    }
}

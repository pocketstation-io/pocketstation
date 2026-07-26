//! Windows WASAPI loopback capture backend.
//!
//! Supports two modes:
//!
//! - `CaptureMode::SystemMix` -- captures the system-wide audio mix using
//!   `AUDCLNT_STREAMFLAGS_LOOPBACK` on the default render endpoint.
//!   Available on Windows Vista+.
//!
//! - `CaptureMode::Process(pid)` -- captures audio from a single process
//!   using `ActivateAudioInterfaceAsync` with process-loopback params.
//!   Available on Windows 10 2004+ (build 19041).
//!
//! # Hot-path invariants
//!
//! No allocation, locking, logging, or panicking on the audio delivery path.
//! Pool slot acquisition is lock-free (atomic CAS in `AudioBufferPool::acquire`).
//!
//! # Known WASAPI bug -- process-loopback period
//!
//! When an `AudioClient` is created via `new_application_loopback_client`,
//! `get_device_period` returns `Not implemented` and `get_buffer_size` returns
//! garbage (e.g. 3 131 961 357).  The period passed to `initialize_client`
//! is documented to be irrelevant in this mode, so we use
//! `WASAPI_PROCESS_LOOPBACK_PERIOD_100NS` (10 ms in 100-ns units) as a safe
//! placeholder that avoids passing zero.
//! Reference: wasapi crate 0.23 doc comment on `new_application_loopback_client`.

use std::mem::size_of;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::open_lifecycle::{
    report_open, wait_for_open, OpenCancellation, OpenReportError, OpenWaitOutcome,
};
use crate::packet_delivery::{plan_packet_read, PacketReadPlan};
use crate::process_identity::ProcessInstanceFingerprint;
use crate::runtime_lifecycle::{classify_platform_status, WindowsRuntimeFailureDisposition};
use pks_frame::{
    AudioBufferPool, AudioFrame, AudioSourceTag, EncryptionMode, Platform, SourceId, StreamId,
    SAMPLE_RATE_HZ,
};
use wasapi::{AudioClient, DeviceEnumerator, Direction, SampleType, StreamMode, WaveFormat};

use pks_capture::{
    monotonic_timestamp_ns, source_runtime_event_channel, CaptureError as LoopbackError,
    CaptureMode, CaptureObservationCounters, CaptureObservations, CaptureRuntimeFailure,
    CaptureRuntimeFailureClass, InputDeviceSelector, SourceGeneration, SourceKind,
    SourceRecoveryRequirement, SourceRuntimeEvent, SourceRuntimeEventObservations,
    SourceRuntimeEventReceive, SourceRuntimeEventReceiver, SourceRuntimeEventSender,
    StableSourceId,
};

const CAPTURE_CHANNEL_COUNT: u8 = 2;
const CAPTURE_POOL_CAPACITY_FRAMES: usize = 8;
const WASAPI_CALLBACK_MAX_SAMPLES: usize = 4096;

/// Buffer duration hint (20 ms in 100-ns units).  Ignored for loopback modes.
const BUFFER_DURATION_100NS: i64 = 200_000;

/// Hardcoded period for process-loopback mode (10 ms in 100-ns units).
///
/// `get_device_period` returns `Not implemented` in process-loopback mode.
/// The period value is also documented as irrelevant.  We use this non-zero
/// constant as a safe placeholder.
pub const WASAPI_PROCESS_LOOPBACK_PERIOD_100NS: i64 = 100_000;

/// `wait_for_event` timeout in milliseconds.
const WAIT_TIMEOUT_MS: u32 = 200;

const BACKEND_OPEN_TIMEOUT_DURATION: Duration = Duration::from_secs(5);

const DISPATCH_QUEUE_CAPACITY_FRAMES: usize = 16;
const RUNTIME_EVENT_CHANNEL_CAPACITY_EVENTS: usize = 8;

pub struct SystemLoopbackSource {
    capture_thread: Option<std::thread::JoinHandle<()>>,
    dispatch_thread: Option<std::thread::JoinHandle<()>>,
    stop_tx: std::sync::mpsc::SyncSender<()>,
    counters: CaptureObservationCounters,
    runtime_event_rx: Option<SourceRuntimeEventReceiver>,
}

pub struct DesktopCaptureSource {
    source: SystemLoopbackSource,
}

impl DesktopCaptureSource {
    pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        SystemLoopbackSource::capture_mode(mode, callback).map(|source| Self { source })
    }

    pub(crate) fn capture_mode_with_runtime_event_sender<F>(
        mode: CaptureMode,
        callback: F,
        runtime_event_sender: SourceRuntimeEventSender,
    ) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        SystemLoopbackSource::capture_mode_with_runtime_event_sender(
            mode,
            callback,
            runtime_event_sender,
        )
        .map(|source| Self { source })
    }

    pub fn observations(&self) -> CaptureObservations {
        self.source.observations()
    }

    pub fn try_recv_runtime_event(&self) -> SourceRuntimeEventReceive {
        self.source.try_recv_runtime_event()
    }

    pub fn runtime_event_observations(&self) -> SourceRuntimeEventObservations {
        self.source.runtime_event_observations()
    }

    pub fn stop_and_join(self) -> Result<CaptureObservations, LoopbackError> {
        self.source.stop_and_join()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessLoopbackScope {
    TargetProcessTree,
}

impl ProcessLoopbackScope {
    fn open_client(
        self,
        process_id: u32,
        expected_instance: Option<ProcessInstanceFingerprint>,
        stable_key: &str,
    ) -> Result<(AudioClient, ProcessInstanceFingerprint), LoopbackError> {
        let opening_instance = verify_process_instance(process_id, expected_instance, stable_key)?;
        let client_result = match self {
            Self::TargetProcessTree => {
                AudioClient::new_application_loopback_client(process_id, true)
            }
        };
        match client_result {
            Ok(client) => {
                verify_process_instance(process_id, Some(opening_instance), stable_key)?;
                Ok((client, opening_instance))
            }
            Err(activation_error) => {
                if let Err(query_error @ LoopbackError::SourceUnavailable { .. }) =
                    verify_process_instance(process_id, Some(opening_instance), stable_key)
                {
                    return Err(query_error);
                }
                Err(LoopbackError::BackendInit(activation_error.to_string()))
            }
        }
    }
}

fn verify_process_instance(
    process_id: u32,
    expected_instance: Option<ProcessInstanceFingerprint>,
    stable_key: &str,
) -> Result<ProcessInstanceFingerprint, LoopbackError> {
    let current_instance = query_process_instance(process_id, stable_key)?;
    if expected_instance.is_some_and(|expected| {
        !expected.matches(
            current_instance.process_id,
            current_instance.creation_time_100ns,
        )
    }) {
        return Err(LoopbackError::SourceUnavailable {
            stable_key: stable_key.to_owned(),
        });
    }
    Ok(current_instance)
}

fn query_process_instance(
    process_id: u32,
    stable_key: &str,
) -> Result<ProcessInstanceFingerprint, LoopbackError> {
    use windows::Win32::Foundation::ERROR_INVALID_PARAMETER;
    use windows_core::HRESULT;

    match query_process_creation_time_100ns(process_id) {
        Ok(creation_time_100ns) => Ok(ProcessInstanceFingerprint::new(
            process_id,
            creation_time_100ns,
        )),
        Err(error) if error.code() == HRESULT::from_win32(ERROR_INVALID_PARAMETER.0) => {
            Err(LoopbackError::SourceUnavailable {
                stable_key: stable_key.to_owned(),
            })
        }
        Err(error) => {
            if query_process_creation_time_100ns(process_id).is_err_and(|retry_error| {
                retry_error.code() == HRESULT::from_win32(ERROR_INVALID_PARAMETER.0)
            }) {
                return Err(LoopbackError::SourceUnavailable {
                    stable_key: stable_key.to_owned(),
                });
            }
            Err(LoopbackError::BackendInit(format!(
                "process {process_id} is inaccessible for exact-application capture: {error}"
            )))
        }
    }
}

fn query_process_creation_time_100ns(process_id: u32) -> windows_core::Result<u64> {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

    // SAFETY: OpenProcess receives a concrete PID and requests query-only
    // access. A successful handle is closed before returning.
    let process_handle =
        unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) }?;
    let query_result = query_process_creation_time_from_handle(process_handle);
    // SAFETY: process_handle came from OpenProcess and has not been transferred
    // or closed.
    let close_result = unsafe { CloseHandle(process_handle) };
    let creation_time_100ns = query_result?;
    close_result?;
    Ok(creation_time_100ns)
}

fn query_process_creation_time_from_handle(
    process_handle: windows::Win32::Foundation::HANDLE,
) -> windows_core::Result<u64> {
    use windows::Win32::Foundation::FILETIME;
    use windows::Win32::System::Threading::GetProcessTimes;

    let mut creation_time = FILETIME::default();
    let mut exit_time = FILETIME::default();
    let mut kernel_time = FILETIME::default();
    let mut user_time = FILETIME::default();
    // SAFETY: every FILETIME pointer is valid for writes during this call and
    // process_handle remains owned by this function until CloseHandle below.
    unsafe {
        GetProcessTimes(
            process_handle,
            &mut creation_time,
            &mut exit_time,
            &mut kernel_time,
            &mut user_time,
        )
    }?;
    Ok((u64::from(creation_time.dwHighDateTime) << 32) | u64::from(creation_time.dwLowDateTime))
}

struct ProcessInstanceWatch {
    process_handle: windows::Win32::Foundation::HANDLE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessInstanceWatchObservation {
    Running,
    Exited,
    Failed { status_code: i32 },
}

impl ProcessInstanceWatch {
    fn open(
        process_id: u32,
        expected_instance: ProcessInstanceFingerprint,
        stable_key: &str,
    ) -> Result<Self, LoopbackError> {
        use windows::Win32::System::Threading::{
            OpenProcess, PROCESS_ACCESS_RIGHTS, PROCESS_QUERY_LIMITED_INFORMATION,
        };
        const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(0x0010_0000);

        // SAFETY: OpenProcess receives a concrete PID and requests only query
        // and synchronization access. The returned handle is owned by Self.
        let process_handle = unsafe {
            OpenProcess(
                PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_SYNCHRONIZE,
                false,
                process_id,
            )
        }
        .map_err(|error| LoopbackError::BackendStatus {
            operation: "open selected process lifecycle handle",
            status_code: error.code().0,
        })?;
        let actual_creation_time = match query_process_creation_time_from_handle(process_handle) {
            Ok(creation_time_100ns) => creation_time_100ns,
            Err(error) => {
                // SAFETY: process_handle was returned by OpenProcess above and
                // has not been transferred.
                let _ = unsafe { windows::Win32::Foundation::CloseHandle(process_handle) };
                return Err(LoopbackError::BackendStatus {
                    operation: "verify selected process lifecycle handle",
                    status_code: error.code().0,
                });
            }
        };
        if !expected_instance.matches(process_id, actual_creation_time) {
            // SAFETY: process_handle was returned by OpenProcess above and has
            // not been transferred.
            let _ = unsafe { windows::Win32::Foundation::CloseHandle(process_handle) };
            return Err(LoopbackError::SourceUnavailable {
                stable_key: stable_key.to_owned(),
            });
        }
        Ok(Self { process_handle })
    }

    fn poll(&self) -> ProcessInstanceWatchObservation {
        use windows::Win32::Foundation::{GetLastError, WAIT_FAILED, WAIT_OBJECT_0, WAIT_TIMEOUT};
        use windows::Win32::System::Threading::WaitForSingleObject;
        use windows_core::HRESULT;

        // SAFETY: process_handle remains valid for Self's lifetime; a zero-ms
        // wait is nonblocking and does not mutate the handle.
        match unsafe { WaitForSingleObject(self.process_handle, 0) } {
            WAIT_OBJECT_0 => ProcessInstanceWatchObservation::Exited,
            WAIT_TIMEOUT => ProcessInstanceWatchObservation::Running,
            WAIT_FAILED => {
                // SAFETY: called immediately on the same thread after the
                // failed WaitForSingleObject.
                let win32_error = unsafe { GetLastError() };
                ProcessInstanceWatchObservation::Failed {
                    status_code: HRESULT::from_win32(win32_error.0).0,
                }
            }
            unexpected => ProcessInstanceWatchObservation::Failed {
                status_code: unexpected.0 as i32,
            },
        }
    }
}

/// Drop contract — capture worker only: release one kernel process handle;
/// no allocation, lock, block, async operation, logging, or panic.
impl Drop for ProcessInstanceWatch {
    fn drop(&mut self) {
        // SAFETY: process_handle is uniquely owned by Self and dropped once.
        let _ = unsafe { windows::Win32::Foundation::CloseHandle(self.process_handle) };
    }
}

struct ComApartmentGuard;

impl ComApartmentGuard {
    fn initialize_mta() -> Result<Self, LoopbackError> {
        let result = wasapi::initialize_mta();
        if result.0 < 0 {
            return Err(LoopbackError::BackendInit(format!(
                "COM MTA initialisation failed: {result:?}"
            )));
        }
        Ok(Self)
    }
}

/// Drop contract — capture setup thread only: deinitialize its COM apartment;
/// remain allocation-free, lock-free, blocking-free, log-free, and panic-free.
impl Drop for ComApartmentGuard {
    fn drop(&mut self) {
        wasapi::deinitialize()
    }
}

impl SystemLoopbackSource {
    pub fn capture<F>(callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        Self::capture_mode(CaptureMode::SystemMix, callback)
    }

    pub fn capture_mode<F>(mode: CaptureMode, callback: F) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        let (runtime_event_sender, runtime_event_receiver) =
            source_runtime_event_channel(RUNTIME_EVENT_CHANNEL_CAPACITY_EVENTS)?;
        Self::capture_mode_with_runtime_events(
            mode,
            callback,
            runtime_event_sender,
            Some(runtime_event_receiver),
        )
    }

    pub(crate) fn capture_mode_with_runtime_event_sender<F>(
        mode: CaptureMode,
        callback: F,
        runtime_event_sender: SourceRuntimeEventSender,
    ) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        Self::capture_mode_with_runtime_events(mode, callback, runtime_event_sender, None)
    }

    fn capture_mode_with_runtime_events<F>(
        mode: CaptureMode,
        callback: F,
        runtime_event_tx: SourceRuntimeEventSender,
        runtime_event_rx: Option<SourceRuntimeEventReceiver>,
    ) -> Result<Self, LoopbackError>
    where
        F: FnMut(AudioFrame) + Send + 'static,
    {
        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);
        let (open_tx, open_rx) = std::sync::mpsc::sync_channel::<Result<(), LoopbackError>>(1);
        let (mut frame_producer, mut frame_consumer) =
            rtrb::RingBuffer::<AudioFrame>::new(DISPATCH_QUEUE_CAPACITY_FRAMES);
        let pool = AudioBufferPool::new(CAPTURE_POOL_CAPACITY_FRAMES, WASAPI_CALLBACK_MAX_SAMPLES);
        let sequence_number = Arc::new(AtomicU64::new(0));
        let counters = CaptureObservationCounters::default();
        let capture_counters = counters.clone();
        let open_cancellation = OpenCancellation::default();
        let worker_cancellation = open_cancellation.clone();
        let capture_thread = std::thread::Builder::new()
            .name("pks-wasapi-capture".into())
            .spawn(move || {
                let _com_guard = match ComApartmentGuard::initialize_mta() {
                    Ok(guard) => guard,
                    Err(error) => {
                        let _ = open_tx.try_send(Err(error));
                        return;
                    }
                };
                if worker_cancellation.is_cancelled() {
                    return;
                }
                apply_mmcss_audio_thread();

                let resolved_mode = match resolve_application_mode(mode) {
                    Ok(mode) => mode,
                    Err(error) => {
                        let _ = open_tx.try_send(Err(error));
                        return;
                    }
                };
                if worker_cancellation.is_cancelled() {
                    return;
                }

                let dispatch_counters = capture_counters.clone();
                let capture_callback = move |frame| {
                    if frame_producer.push(frame).is_err() {
                        dispatch_counters.observe_dispatch_queue_full();
                    } else {
                        dispatch_counters.observe_enqueued_frame();
                    }
                };

                let result = match resolved_mode {
                    CaptureMode::SystemMix => run_system_loopback(
                        CaptureWorkerContext {
                            pool,
                            sequence: sequence_number,
                            stop_rx,
                            open_tx: open_tx.clone(),
                            counters: capture_counters.clone(),
                            open_cancellation: worker_cancellation.clone(),
                            runtime_event_tx: runtime_event_tx.clone(),
                        },
                        capture_callback,
                    ),
                    CaptureMode::Process(process_id) => {
                        let stable_key = format!("wasapi:pid:{process_id}");
                        run_process_loopback(
                            process_id,
                            None,
                            &stable_key,
                            SourceId(0),
                            CaptureWorkerContext {
                                pool,
                                sequence: sequence_number,
                                stop_rx,
                                open_tx: open_tx.clone(),
                                counters: capture_counters.clone(),
                                open_cancellation: worker_cancellation.clone(),
                                runtime_event_tx: runtime_event_tx.clone(),
                            },
                            capture_callback,
                        )
                    }
                    CaptureMode::ExactApplication {
                        process_id,
                        stable_id,
                    } => {
                        let source_id = stable_id.to_frame_source_id();
                        if let Some(expected_instance) =
                            ProcessInstanceFingerprint::parse(&stable_id.stable_key)
                        {
                            run_process_loopback(
                                process_id,
                                Some(expected_instance),
                                &stable_id.stable_key,
                                source_id,
                                CaptureWorkerContext {
                                    pool,
                                    sequence: sequence_number,
                                    stop_rx,
                                    open_tx: open_tx.clone(),
                                    counters: capture_counters.clone(),
                                    open_cancellation: worker_cancellation.clone(),
                                    runtime_event_tx: runtime_event_tx.clone(),
                                },
                                capture_callback,
                            )
                        } else {
                            Err(LoopbackError::BackendInit(format!(
                                "exact Windows application identity '{}' has no process-instance fingerprint",
                                stable_id.stable_key
                            )))
                        }
                    }
                    CaptureMode::InputDevice(selector) => run_input_capture(
                        selector,
                        CaptureWorkerContext {
                            pool,
                            sequence: sequence_number,
                            stop_rx,
                            open_tx: open_tx.clone(),
                            counters: capture_counters.clone(),
                            open_cancellation: worker_cancellation.clone(),
                            runtime_event_tx: runtime_event_tx.clone(),
                        },
                        capture_callback,
                    ),
                    // Application(_) is resolved to Process before thread spawn.
                    other => Err(LoopbackError::ModeUnsupported(other)),
                };
                if let Err(error) = result {
                    if !worker_cancellation.is_cancelled() {
                        capture_counters.observe_stream_error();
                        let _ = open_tx.try_send(Err(error));
                    }
                }
            })
            .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;

        match wait_for_open(&open_rx, BACKEND_OPEN_TIMEOUT_DURATION) {
            OpenWaitOutcome::Opened => {}
            OpenWaitOutcome::Failed(error) => {
                open_cancellation.cancel();
                let _ = stop_tx.try_send(());
                drop(capture_thread);
                return Err(error);
            }
            OpenWaitOutcome::TimedOut => {
                open_cancellation.cancel();
                let _ = stop_tx.try_send(());
                drop(capture_thread);
                return Err(LoopbackError::BackendInit(format!(
                    "WASAPI stream did not open within {} ms",
                    BACKEND_OPEN_TIMEOUT_DURATION.as_millis()
                )));
            }
            OpenWaitOutcome::WorkerExited => {
                open_cancellation.cancel();
                let _ = stop_tx.try_send(());
                drop(capture_thread);
                return Err(LoopbackError::BackendInit(
                    "WASAPI capture worker exited before reporting open status".to_owned(),
                ));
            }
        }

        let dispatch_thread = match std::thread::Builder::new()
            .name("pks-wasapi-dispatch".into())
            .spawn(move || {
                let mut callback = callback;
                loop {
                    while let Ok(frame) = frame_consumer.pop() {
                        callback(frame);
                    }
                    if frame_consumer.is_abandoned() {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(1));
                }
            }) {
            Ok(thread) => thread,
            Err(error) => {
                open_cancellation.cancel();
                let _ = stop_tx.try_send(());
                drop(capture_thread);
                return Err(LoopbackError::BackendInit(format!(
                    "WASAPI dispatch thread: {error}"
                )));
            }
        };

        Ok(Self {
            capture_thread: Some(capture_thread),
            dispatch_thread: Some(dispatch_thread),
            stop_tx,
            counters,
            runtime_event_rx,
        })
    }

    pub fn observations(&self) -> CaptureObservations {
        self.counters.snapshot()
    }

    pub fn try_recv_runtime_event(&self) -> SourceRuntimeEventReceive {
        self.runtime_event_rx
            .as_ref()
            .map_or(SourceRuntimeEventReceive::Closed, |receiver| {
                receiver.try_recv()
            })
    }

    pub fn runtime_event_observations(&self) -> SourceRuntimeEventObservations {
        self.runtime_event_rx
            .as_ref()
            .map_or_else(SourceRuntimeEventObservations::default, |receiver| {
                receiver.observations()
            })
    }

    pub fn stop_and_join(mut self) -> Result<CaptureObservations, LoopbackError> {
        let counters = self.counters.clone();
        self.stop_workers()?;
        Ok(counters.snapshot())
    }

    fn stop_workers(&mut self) -> Result<(), LoopbackError> {
        let _ = self.stop_tx.try_send(());
        let capture_join = self.capture_thread.take().map_or(Ok(()), |thread| {
            pks_capture::join_capture_worker(thread, "Windows capture")
        });
        let dispatch_join = self.dispatch_thread.take().map_or(Ok(()), |thread| {
            pks_capture::join_capture_worker(thread, "Windows dispatch")
        });
        capture_join.and(dispatch_join)
    }
}

fn resolve_application_mode(mode: CaptureMode) -> Result<CaptureMode, LoopbackError> {
    let CaptureMode::Application(name) = mode else {
        return Ok(mode);
    };
    let name_lower = name.to_ascii_lowercase();
    let sources = discover_sources_windows();
    let source = sources
        .iter()
        .find(|source| {
            source.name.to_ascii_lowercase() == name_lower
                || source.app_id.as_deref().map(str::to_ascii_lowercase) == Some(name_lower.clone())
        })
        .ok_or_else(|| {
            LoopbackError::BackendInit(format!(
                "no audio session found for '{name}' — run `pks sources list`"
            ))
        })?;
    let process_id = source.process_id.ok_or_else(|| {
        LoopbackError::BackendInit(format!(
            "audio source '{name}' has no process identity — run `pks sources list`"
        ))
    })?;
    Ok(CaptureMode::ExactApplication {
        process_id,
        stable_id: source.stable_id.clone(),
    })
}

/// Drop contract — control thread only: signal once, join both owned workers,
/// never execute from a capture callback or realtime partition.
impl Drop for SystemLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_workers();
    }
}

fn apply_mmcss_audio_thread() {
    use windows::Win32::Media::timeBeginPeriod;
    use windows::Win32::System::Threading::AvSetMmThreadCharacteristicsW;
    use windows_core::w;

    // SAFETY: both API calls are safe to call from any thread.
    // Failure is non-fatal -- capture continues at normal priority.
    unsafe {
        timeBeginPeriod(1);
        let mut task_index: u32 = 0;
        let _ = AvSetMmThreadCharacteristicsW(w!("Audio"), &mut task_index);
    }
}

fn target_wave_format() -> WaveFormat {
    WaveFormat::new(
        32,
        32,
        &SampleType::Float,
        SAMPLE_RATE_HZ as usize,
        CAPTURE_CHANNEL_COUNT as usize,
        None,
    )
}

#[inline(always)]
fn deliver_packet(
    raw: &[u8],
    source_id: SourceId,
    pool: &Arc<AudioBufferPool>,
    sequence_number: &Arc<AtomicU64>,
    counters: &CaptureObservationCounters,
    callback: &mut (impl FnMut(AudioFrame) + Send + 'static),
) {
    counters.observe_callback_buffer();
    if raw.is_empty() || !raw.len().is_multiple_of(size_of::<f32>()) {
        counters.observe_invalid_buffer();
        return;
    }
    let sample_count = raw.len() / size_of::<f32>();
    let mut handle = match pool.acquire() {
        Some(h) => h,
        None => {
            counters.observe_pool_exhaustion();
            return;
        }
    };
    let dst = handle.as_mut_slice();
    if sample_count > dst.len() {
        counters.observe_oversized_buffer();
        return;
    }
    // SAFETY: `raw` is a valid WASAPI buffer; 4-byte groups are f32.
    let src_ptr = raw.as_ptr() as *const f32;
    for (sample_index, sample) in dst.iter_mut().take(sample_count).enumerate() {
        // SAFETY: sample_index is bounded by raw.len() / size_of::<f32>().
        *sample = unsafe { src_ptr.add(sample_index).read_unaligned() };
    }
    handle.set_len(sample_count);

    let frame_sequence_number = sequence_number.fetch_add(1, Ordering::Relaxed);
    let timestamp_ns = monotonic_timestamp_ns();
    let mut frame = AudioFrame::new(
        StreamId(0),
        source_id,
        frame_sequence_number,
        timestamp_ns,
        CAPTURE_CHANNEL_COUNT,
        handle,
    );
    frame.source_tag = AudioSourceTag::Captured;
    frame.encryption_mode = EncryptionMode::None;
    frame.sample_rate_hz = SAMPLE_RATE_HZ;
    callback(frame);
}

struct CaptureLoopState {
    source_id: SourceId,
    stable_id: StableSourceId,
    pool: Arc<AudioBufferPool>,
    sequence: Arc<AtomicU64>,
    stop_rx: std::sync::mpsc::Receiver<()>,
    counters: CaptureObservationCounters,
    open_cancellation: OpenCancellation,
    runtime_event_tx: SourceRuntimeEventSender,
    process_watch: Option<ProcessInstanceWatch>,
}

struct CaptureWorkerContext {
    pool: Arc<AudioBufferPool>,
    sequence: Arc<AtomicU64>,
    stop_rx: std::sync::mpsc::Receiver<()>,
    open_tx: std::sync::mpsc::SyncSender<Result<(), LoopbackError>>,
    counters: CaptureObservationCounters,
    open_cancellation: OpenCancellation,
    runtime_event_tx: SourceRuntimeEventSender,
}

impl CaptureWorkerContext {
    fn into_loop_state(
        self,
        source_id: SourceId,
        stable_id: StableSourceId,
        process_watch: Option<ProcessInstanceWatch>,
    ) -> CaptureLoopState {
        CaptureLoopState {
            source_id,
            stable_id,
            pool: self.pool,
            sequence: self.sequence,
            stop_rx: self.stop_rx,
            counters: self.counters,
            open_cancellation: self.open_cancellation,
            runtime_event_tx: self.runtime_event_tx,
            process_watch,
        }
    }
}

#[derive(Debug)]
enum CaptureLoopFailure {
    ProcessExited,
    ProcessWatchFailed {
        status_code: i32,
    },
    Wasapi {
        operation: &'static str,
        error: wasapi::WasapiError,
    },
    BackendClass {
        operation: &'static str,
        class: &'static str,
    },
}

impl CaptureLoopFailure {
    fn operation(&self) -> &'static str {
        match self {
            Self::ProcessExited => "observe selected process lifetime",
            Self::ProcessWatchFailed { .. } => "observe selected process lifetime",
            Self::Wasapi { operation, .. } | Self::BackendClass { operation, .. } => operation,
        }
    }

    fn error_class(&self) -> CaptureRuntimeFailureClass {
        match self {
            Self::ProcessExited => CaptureRuntimeFailureClass::SourceInstanceExited,
            Self::ProcessWatchFailed { status_code } => {
                CaptureRuntimeFailureClass::PlatformStatus {
                    status_code: *status_code,
                }
            }
            Self::Wasapi {
                error: wasapi::WasapiError::Windows(error),
                ..
            } => CaptureRuntimeFailureClass::PlatformStatus {
                status_code: error.code().0,
            },
            Self::Wasapi { error, .. } => CaptureRuntimeFailureClass::BackendClass {
                class: format!("{error:?}"),
            },
            Self::BackendClass { class, .. } => CaptureRuntimeFailureClass::BackendClass {
                class: (*class).to_owned(),
            },
        }
    }

    fn disposition(&self) -> WindowsRuntimeFailureDisposition {
        match self {
            Self::ProcessExited => WindowsRuntimeFailureDisposition::SourceUnavailable,
            Self::Wasapi {
                error: wasapi::WasapiError::Windows(error),
                ..
            } => classify_platform_status(error.code().0),
            Self::ProcessWatchFailed { .. } | Self::Wasapi { .. } | Self::BackendClass { .. } => {
                WindowsRuntimeFailureDisposition::BackendFailure
            }
        }
    }

    fn runtime_event(&self, stable_id: &StableSourceId) -> SourceRuntimeEvent {
        let failure = CaptureRuntimeFailure {
            operation: self.operation(),
            error_class: self.error_class(),
        };
        match self.disposition() {
            WindowsRuntimeFailureDisposition::SourceUnavailable => {
                SourceRuntimeEvent::SourceUnavailable {
                    stable_id: stable_id.clone(),
                    generation: SourceGeneration::INITIAL,
                    recovery_requirement:
                        SourceRecoveryRequirement::ExplicitRediscoveryAndNewSession,
                    failure,
                }
            }
            WindowsRuntimeFailureDisposition::BackendFailure => {
                SourceRuntimeEvent::BackendFailure {
                    stable_id: stable_id.clone(),
                    generation: SourceGeneration::INITIAL,
                    failure,
                }
            }
        }
    }

    fn into_capture_error(self, stable_key: String) -> LoopbackError {
        if self.disposition() == WindowsRuntimeFailureDisposition::SourceUnavailable {
            return LoopbackError::SourceUnavailable { stable_key };
        }
        match self {
            Self::ProcessWatchFailed { status_code } => LoopbackError::BackendStatus {
                operation: "observe selected process lifetime",
                status_code,
            },
            Self::Wasapi {
                operation,
                error: wasapi::WasapiError::Windows(error),
            } => LoopbackError::BackendStatus {
                operation,
                status_code: error.code().0,
            },
            Self::Wasapi { operation, error } => {
                LoopbackError::BackendInit(format!("{operation}: {error:?}"))
            }
            Self::BackendClass { operation, class } => {
                LoopbackError::BackendInit(format!("{operation}: {class}"))
            }
            Self::ProcessExited => LoopbackError::SourceUnavailable { stable_key },
        }
    }
}

fn capture_loop(
    audio_client: &AudioClient,
    capture_client: &wasapi::AudioCaptureClient,
    h_event: &wasapi::Handle,
    state: CaptureLoopState,
    mut callback: impl FnMut(AudioFrame) + Send + 'static,
) -> Result<(), LoopbackError> {
    const CALLBACK_MAX_BYTES: usize = WASAPI_CALLBACK_MAX_SAMPLES * size_of::<f32>();
    let mut raw_buf = [0u8; CALLBACK_MAX_BYTES];

    let capture_result: Result<(), CaptureLoopFailure> = (|| {
        loop {
            if state.open_cancellation.is_cancelled() || state.stop_rx.try_recv().is_ok() {
                break;
            }
            if let Some(process_watch) = &state.process_watch {
                match process_watch.poll() {
                    ProcessInstanceWatchObservation::Running => {}
                    ProcessInstanceWatchObservation::Exited => {
                        return Err(CaptureLoopFailure::ProcessExited);
                    }
                    ProcessInstanceWatchObservation::Failed { status_code } => {
                        return Err(CaptureLoopFailure::ProcessWatchFailed { status_code });
                    }
                }
            }
            match h_event.wait_for_event(WAIT_TIMEOUT_MS) {
                Ok(()) => {}
                Err(wasapi::WasapiError::EventTimeout) => {
                    if state.stop_rx.try_recv().is_ok() {
                        break;
                    }
                    continue;
                }
                Err(error) => {
                    return Err(CaptureLoopFailure::Wasapi {
                        operation: "wait for WASAPI capture event",
                        error,
                    });
                }
            }
            loop {
                let next = match capture_client.get_next_packet_size() {
                    Ok(Some(packet_frames)) => packet_frames,
                    Ok(None) => 0,
                    Err(error) => {
                        return Err(CaptureLoopFailure::Wasapi {
                            operation: "query next WASAPI packet size",
                            error,
                        });
                    }
                };
                match plan_packet_read(next, CAPTURE_CHANNEL_COUNT, size_of::<f32>(), raw_buf.len())
                {
                    PacketReadPlan::Empty => break,
                    PacketReadPlan::Read { .. } => {}
                    PacketReadPlan::Oversized { .. } => {
                        state.counters.observe_oversized_buffer();
                        return Err(CaptureLoopFailure::BackendClass {
                            operation: "validate announced WASAPI packet size",
                            class: "announced-packet-oversized",
                        });
                    }
                }
                match capture_client.read_from_device(&mut raw_buf) {
                    Ok((frames, info)) => {
                        if frames == 0 || info.flags.silent {
                            continue;
                        }
                        let bytes = match plan_packet_read(
                            frames,
                            CAPTURE_CHANNEL_COUNT,
                            size_of::<f32>(),
                            raw_buf.len(),
                        ) {
                            PacketReadPlan::Read { packet_bytes } => packet_bytes,
                            PacketReadPlan::Empty => continue,
                            PacketReadPlan::Oversized { .. } => {
                                state.counters.observe_oversized_buffer();
                                return Err(CaptureLoopFailure::BackendClass {
                                    operation: "validate delivered WASAPI packet size",
                                    class: "delivered-packet-oversized",
                                });
                            }
                        };
                        deliver_packet(
                            &raw_buf[..bytes],
                            state.source_id,
                            &state.pool,
                            &state.sequence,
                            &state.counters,
                            &mut callback,
                        );
                    }
                    Err(error) => {
                        return Err(CaptureLoopFailure::Wasapi {
                            operation: "read WASAPI packet",
                            error,
                        });
                    }
                }
            }
        }
        Ok(())
    })();
    let _ = audio_client.stop_stream();
    capture_result.map_err(|failure| {
        let _ = state
            .runtime_event_tx
            .try_send(failure.runtime_event(&state.stable_id));
        failure.into_capture_error(state.stable_id.stable_key.clone())
    })
}

fn signal_open(
    audio_client: &AudioClient,
    open_tx: &std::sync::mpsc::SyncSender<Result<(), LoopbackError>>,
    open_cancellation: &OpenCancellation,
) -> Result<(), LoopbackError> {
    report_open(open_tx, open_cancellation).map_err(|error| {
        let _ = audio_client.stop_stream();
        let message = match error {
            OpenReportError::Cancelled => "WASAPI open was cancelled before activation completed",
            OpenReportError::ReceiverUnavailable => "WASAPI open result receiver is unavailable",
        };
        LoopbackError::BackendInit(message.to_owned())
    })
}

fn run_system_loopback(
    context: CaptureWorkerContext,
    callback: impl FnMut(AudioFrame) + Send + 'static,
) -> Result<(), LoopbackError> {
    let enumerator =
        DeviceEnumerator::new().map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let device = enumerator
        .get_default_device(&Direction::Render)
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let mut audio_client = device
        .get_iaudioclient()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let wave_fmt = target_wave_format();
    audio_client
        .initialize_client(
            &wave_fmt,
            &Direction::Capture, // wasapi adds LOOPBACK flag automatically
            &StreamMode::EventsShared {
                autoconvert: true,
                buffer_duration_hns: BUFFER_DURATION_100NS,
            },
        )
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let h_event = audio_client
        .set_get_eventhandle()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let capture_client = audio_client
        .get_audiocaptureclient()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    audio_client
        .start_stream()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    signal_open(&audio_client, &context.open_tx, &context.open_cancellation)?;
    capture_loop(
        &audio_client,
        &capture_client,
        &h_event,
        context.into_loop_state(
            SourceId(0),
            StableSourceId::new(Platform::Windows, SourceKind::SystemMix, "system:mix"),
            None,
        ),
        callback,
    )
}

/// Enumerate all audio capture sources visible on this Windows system.
///
/// Always returns at least one entry (the system-wide mix at id=0).
/// Per-process application sources are appended via WASAPI session enumeration.
pub fn discover_sources_windows() -> Vec<pks_capture::CaptureSource> {
    use pks_capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use pks_frame::Platform;

    let system_mix = CaptureSource {
        stable_id: StableSourceId::new(Platform::Windows, SourceKind::SystemMix, "system:mix"),
        name: "System Mix".to_owned(),
        process_id: None,
        app_id: None,
        device_uid: None,
        state: SourceState::Available,
        sample_rate_hz: 48_000,
        channels: 2,
    };

    let Ok(_com_guard) = ComApartmentGuard::initialize_mta() else {
        return vec![system_mix];
    };

    let mut sources = vec![system_mix];
    sources.extend(enumerate_wasapi_input_devices());
    // SAFETY: COM is initialised above; all COM objects are released before return.
    let app_sources = unsafe { enumerate_wasapi_sessions() };
    sources.extend(app_sources);
    sources
}

fn enumerate_wasapi_input_devices() -> Vec<pks_capture::CaptureSource> {
    use pks_capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use pks_frame::Platform;

    let Ok(enumerator) = DeviceEnumerator::new() else {
        return Vec::new();
    };
    let default_device_id = enumerator
        .get_default_device(&Direction::Capture)
        .ok()
        .and_then(|device| device.get_id().ok());
    let Ok(devices) = enumerator.get_device_collection(&Direction::Capture) else {
        return Vec::new();
    };
    let Ok(device_count) = devices.get_nbr_devices() else {
        return Vec::new();
    };
    let mut sources = Vec::with_capacity(device_count as usize);
    for device_index in 0..device_count {
        let Ok(device) = devices.get_device_at_index(device_index) else {
            continue;
        };
        let Ok(device_id) = device.get_id() else {
            continue;
        };
        let name = device
            .get_friendlyname()
            .unwrap_or_else(|_| device_id.clone());
        let format = device.get_device_format().ok();
        let sample_rate_hz = format
            .as_ref()
            .map_or(SAMPLE_RATE_HZ, WaveFormat::get_samplespersec);
        let channels = format
            .as_ref()
            .map_or(u16::from(CAPTURE_CHANNEL_COUNT), WaveFormat::get_nchannels);
        sources.push(CaptureSource {
            stable_id: StableSourceId::new(
                Platform::Windows,
                SourceKind::InputDevice,
                device_id.clone(),
            ),
            name,
            process_id: None,
            app_id: None,
            device_uid: Some(device_id.clone()),
            state: SourceState::Available,
            sample_rate_hz,
            channels,
        });
    }
    sources.sort_by_key(|source| {
        let is_default = default_device_id.as_deref() == source.device_uid.as_deref();
        (!is_default, source.name.clone())
    });
    sources
}

/// Enumerate active WASAPI audio sessions on the default render endpoint.
///
/// Returns per-process `Application` sources.  On any error, returns an empty
/// `Vec` — missing sessions are not fatal.
///
/// # Safety
///
/// Caller must have initialised COM (MTA) before calling this function.
unsafe fn enumerate_wasapi_sessions() -> Vec<pks_capture::CaptureSource> {
    use pks_capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use pks_frame::Platform;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Media::Audio::{
        eMultimedia, eRender, AudioSessionStateActive, IAudioSessionControl, IAudioSessionControl2,
        IAudioSessionEnumerator, IAudioSessionManager2, IMMDeviceEnumerator, MMDeviceEnumerator,
    };
    use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};
    use windows::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
        PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows_core::{Interface, PWSTR};

    let current_pid = std::process::id();

    let enumerator: IMMDeviceEnumerator =
        match CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };

    let device = match enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    let session_manager: IAudioSessionManager2 = match device.Activate(CLSCTX_ALL, None) {
        Ok(sm) => sm,
        Err(_) => return Vec::new(),
    };

    let session_enum: IAudioSessionEnumerator = match session_manager.GetSessionEnumerator() {
        Ok(se) => se,
        Err(_) => return Vec::new(),
    };

    let count = match session_enum.GetCount() {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut sources: Vec<CaptureSource> = Vec::with_capacity(count as usize);

    for i in 0..count {
        let ctrl: IAudioSessionControl = match session_enum.GetSession(i) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let ctrl2: IAudioSessionControl2 = match ctrl.cast() {
            Ok(c) => c,
            Err(_) => continue,
        };

        let pid = match ctrl2.GetProcessId() {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Skip system/idle sessions and our own process.
        if pid == 0 || pid == current_pid {
            continue;
        }
        let process_instance = match query_process_creation_time_100ns(pid) {
            Ok(creation_time_100ns) => ProcessInstanceFingerprint::new(pid, creation_time_100ns),
            Err(_) => continue,
        };

        let state = match ctrl.GetState() {
            Ok(s) => s,
            Err(_) => continue,
        };
        let source_state = if state == AudioSessionStateActive {
            SourceState::Playing
        } else {
            SourceState::Silent
        };

        // Resolve process name from PID.
        let name = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                let mut buf = vec![0u16; 260];
                let mut len = buf.len() as u32;
                let name_result = QueryFullProcessImageNameW(
                    handle,
                    PROCESS_NAME_WIN32,
                    PWSTR(buf.as_mut_ptr()),
                    &mut len,
                );
                let _ = CloseHandle(handle);
                match name_result {
                    Ok(()) => {
                        let path = String::from_utf16_lossy(&buf[..len as usize]);
                        std::path::Path::new(&path)
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or(&path)
                            .to_owned()
                    }
                    Err(_) => format!("pid-{pid}"),
                }
            }
            Err(_) => format!("pid-{pid}"),
        };
        match query_process_creation_time_100ns(pid) {
            Ok(creation_time_100ns)
                if creation_time_100ns == process_instance.creation_time_100ns => {}
            _ => continue,
        }

        sources.push(CaptureSource {
            stable_id: StableSourceId::new(
                Platform::Windows,
                SourceKind::Application,
                process_instance.stable_key(),
            ),
            name,
            process_id: Some(pid),
            // The StableSourceId identifies this PID incarnation through its
            // creation FILETIME; it is not an application ID across restarts.
            app_id: None,
            device_uid: None,
            state: source_state,
            sample_rate_hz: 48_000,
            channels: 2,
        });
    }

    sources
}

fn run_process_loopback(
    process_id: u32,
    expected_instance: Option<ProcessInstanceFingerprint>,
    stable_key: &str,
    source_id: SourceId,
    context: CaptureWorkerContext,
    callback: impl FnMut(AudioFrame) + Send + 'static,
) -> Result<(), LoopbackError> {
    let (mut audio_client, opening_instance) = ProcessLoopbackScope::TargetProcessTree
        .open_client(process_id, expected_instance, stable_key)?;
    let process_watch = ProcessInstanceWatch::open(process_id, opening_instance, stable_key)?;
    if context.open_cancellation.is_cancelled() {
        return Err(LoopbackError::BackendInit(
            "WASAPI process-loopback activation completed after open cancellation".to_owned(),
        ));
    }
    let wave_fmt = target_wave_format();
    // Period is irrelevant in process-loopback mode; use a safe non-zero value.
    audio_client
        .initialize_client(
            &wave_fmt,
            &Direction::Capture,
            &StreamMode::EventsShared {
                autoconvert: true,
                buffer_duration_hns: WASAPI_PROCESS_LOOPBACK_PERIOD_100NS,
            },
        )
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let h_event = audio_client
        .set_get_eventhandle()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    let capture_client = audio_client
        .get_audiocaptureclient()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    audio_client
        .start_stream()
        .map_err(|e| LoopbackError::BackendInit(e.to_string()))?;
    signal_open(&audio_client, &context.open_tx, &context.open_cancellation)?;
    capture_loop(
        &audio_client,
        &capture_client,
        &h_event,
        context.into_loop_state(
            source_id,
            StableSourceId::new(Platform::Windows, SourceKind::Application, stable_key),
            Some(process_watch),
        ),
        callback,
    )
}

fn run_input_capture(
    selector: InputDeviceSelector,
    context: CaptureWorkerContext,
    callback: impl FnMut(AudioFrame) + Send + 'static,
) -> Result<(), LoopbackError> {
    let enumerator =
        DeviceEnumerator::new().map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    let device = match selector {
        InputDeviceSelector::Default => enumerator
            .get_default_device(&Direction::Capture)
            .map_err(|error| LoopbackError::BackendInit(error.to_string()))?,
        InputDeviceSelector::StableId(device_id) => {
            ensure_input_device_is_available(&enumerator, &device_id)?;
            match enumerator.get_device(&device_id) {
                Ok(device) => device,
                Err(open_error) => {
                    if let Err(unavailable @ LoopbackError::SourceUnavailable { .. }) =
                        ensure_input_device_is_available(&enumerator, &device_id)
                    {
                        return Err(unavailable);
                    }
                    return Err(LoopbackError::BackendInit(open_error.to_string()));
                }
            }
        }
    };
    if device.get_direction() != Direction::Capture {
        return Err(LoopbackError::BackendInit(
            "selected Windows endpoint is not an input device".to_owned(),
        ));
    }
    let device_id = device
        .get_id()
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    let stable_id = StableSourceId::new(Platform::Windows, SourceKind::InputDevice, device_id);
    let source_id = stable_id.to_frame_source_id();
    let mut audio_client = device
        .get_iaudioclient()
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    let wave_fmt = target_wave_format();
    audio_client
        .initialize_client(
            &wave_fmt,
            &Direction::Capture,
            &StreamMode::EventsShared {
                autoconvert: true,
                buffer_duration_hns: BUFFER_DURATION_100NS,
            },
        )
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    let h_event = audio_client
        .set_get_eventhandle()
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    let capture_client = audio_client
        .get_audiocaptureclient()
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    audio_client
        .start_stream()
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    signal_open(&audio_client, &context.open_tx, &context.open_cancellation)?;
    capture_loop(
        &audio_client,
        &capture_client,
        &h_event,
        context.into_loop_state(source_id, stable_id, None),
        callback,
    )
}

fn ensure_input_device_is_available(
    enumerator: &DeviceEnumerator,
    stable_key: &str,
) -> Result<(), LoopbackError> {
    let devices = enumerator
        .get_device_collection(&Direction::Capture)
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    let device_count = devices
        .get_nbr_devices()
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    for device_index in 0..device_count {
        let device = devices
            .get_device_at_index(device_index)
            .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
        let device_id = device
            .get_id()
            .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
        if device_id == stable_key {
            return Ok(());
        }
    }
    Err(LoopbackError::SourceUnavailable {
        stable_key: stable_key.to_owned(),
    })
}

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
//! # Process-loopback activation and period
//!
//! The third-party `AudioClient::new_application_loopback_client` waits without
//! a cancellation or deadline. This backend therefore owns the narrow
//! `ActivateAudioInterfaceAsync` process-loopback boundary and retains the
//! activation inputs with the Windows-owned completion handler. System-mix and
//! input-device capture continue to use the third-party wrapper.
//!
//! Process loopback does not expose a useful device period. The period passed
//! to `IAudioClient::Initialize` is irrelevant in this mode, so we use
//! `WASAPI_PROCESS_LOOPBACK_PERIOD_100NS` (10 ms in 100-ns units) as a safe
//! placeholder that avoids passing zero.

use std::mem::size_of;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::capture::platform::windows::open_lifecycle::{
    report_open, wait_for_completion, wait_for_open, CancellableWaitOutcome, OpenCancellation,
    OpenReportError, OpenWaitOutcome,
};
use crate::capture::platform::windows::packet_delivery::{
    plan_packet_read, sample_buffer_capacity_bytes, PacketReadPlan,
};
use crate::capture::platform::windows::process_identity::ProcessInstanceFingerprint;
use crate::capture::platform::windows::runtime_lifecycle::{
    classify_platform_status, WindowsRuntimeFailureDisposition,
};
use crate::frame::{
    AudioBufferPool, AudioFrame, Platform, SourceId, StreamId, POOL_MAX_SLOTS, SAMPLE_RATE_HZ,
};
use wasapi::{AudioClient, DeviceEnumerator, Direction, SampleType, StreamMode, WaveFormat};
use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

use crate::capture::frame_normalizer::CaptureFrameNormalizer;
use crate::capture::{
    initialize_monotonic_timestamp_domain, monotonic_timestamp_ns, source_runtime_event_channel,
    CaptureError as LoopbackError, CaptureMode, CaptureObservationCounters,
    CaptureObservationHandle, CaptureObservations, CaptureRuntimeFailure,
    CaptureRuntimeFailureClass, InputDeviceSelector, SourceGeneration, SourceKind,
    SourceRecoveryRequirement, SourceRuntimeEvent, SourceRuntimeEventObservations,
    SourceRuntimeEventReceive, SourceRuntimeEventReceiver, SourceRuntimeEventSender,
    StableSourceId,
};

const CAPTURE_CHANNEL_COUNT: u8 = 2;
const CAPTURE_FRAME_SAMPLES_PER_CHANNEL: usize = 960;
const CAPTURE_FRAME_SAMPLES: usize =
    CAPTURE_FRAME_SAMPLES_PER_CHANNEL * CAPTURE_CHANNEL_COUNT as usize;
// A delivered WASAPI frame remains backed by this pool while it crosses the
// 16-frame platform dispatch ring and the Session's bounded capture queue
// (32 frames by default). Eight slots could therefore exhaust before either
// bounded queue was full, which made real loss appear under ordinary guest
// scheduling jitter. The architecture-wide fixed maximum is still bounded
// (one MiB per 4096-sample pool) and covers both queues plus in-flight frames.
const CAPTURE_POOL_CAPACITY_FRAMES: usize = POOL_MAX_SLOTS;
const WASAPI_CALLBACK_MAX_DURATION_MS: usize = 200;
const WASAPI_CALLBACK_MAX_FRAMES: usize =
    SAMPLE_RATE_HZ as usize * WASAPI_CALLBACK_MAX_DURATION_MS / 1_000;
const WASAPI_CALLBACK_MAX_SAMPLES: usize =
    WASAPI_CALLBACK_MAX_FRAMES * CAPTURE_CHANNEL_COUNT as usize;

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
const PROCESS_ACTIVATION_TIMEOUT_DURATION: Duration = Duration::from_secs(4);
const PROCESS_ACTIVATION_CANCELLATION_POLL_DURATION: Duration = Duration::from_millis(10);
const OPEN_WORKER_CANCELLATION_GRACE_DURATION: Duration = Duration::from_millis(250);

// WASAPI commonly delivers 10 ms packets. Twenty-four slots provide 240 ms
// of bounded scheduler headroom without promoting the dispatch thread above
// its downstream graph consumers. Together with the Standard CLI's 16-frame
// input channel and 16-frame graph ring, this remains below the source pool's
// 64-slot ownership bound with room for callback and dispatch in-flight frames.
const DISPATCH_QUEUE_CAPACITY_FRAMES: usize = 24;
const RUNTIME_EVENT_CHANNEL_CAPACITY_EVENTS: usize = 8;

const _: () = assert!(CAPTURE_POOL_CAPACITY_FRAMES >= DISPATCH_QUEUE_CAPACITY_FRAMES + 2);

#[derive(Debug, Clone, Copy)]
struct WasapiTimestampMapping {
    qpc_origin_ns: u64,
    monotonic_origin_ns: u64,
}

impl WasapiTimestampMapping {
    fn new() -> Option<Self> {
        initialize_monotonic_timestamp_domain();
        let monotonic_before_ns = monotonic_timestamp_ns();
        let mut counter = 0i64;
        let mut frequency_hz = 0i64;
        // SAFETY: both output pointers are valid for the duration of each call.
        unsafe {
            QueryPerformanceFrequency(&mut frequency_hz).ok()?;
            QueryPerformanceCounter(&mut counter).ok()?;
        }
        let monotonic_after_ns = monotonic_timestamp_ns();
        let counter = u64::try_from(counter).ok()?;
        let frequency_hz = u64::try_from(frequency_hz).ok()?.max(1);
        let qpc_origin_ns = u128::from(counter)
            .saturating_mul(1_000_000_000)
            .checked_div(u128::from(frequency_hz))?
            .min(u128::from(u64::MAX)) as u64;
        let monotonic_origin_ns = monotonic_before_ns
            .saturating_add(monotonic_after_ns.saturating_sub(monotonic_before_ns) / 2);
        Some(Self {
            qpc_origin_ns,
            monotonic_origin_ns,
        })
    }

    fn to_monotonic_ns(self, qpc_timestamp_ns: u64) -> Option<u64> {
        let delta_ns = qpc_timestamp_ns.abs_diff(self.qpc_origin_ns);
        let timestamp_ns = if qpc_timestamp_ns >= self.qpc_origin_ns {
            self.monotonic_origin_ns.checked_add(delta_ns)?
        } else {
            self.monotonic_origin_ns.checked_sub(delta_ns)?
        };
        (timestamp_ns > 0).then_some(timestamp_ns)
    }
}

fn wasapi_qpc_100ns_to_ns(qpc_position: u64) -> Option<u64> {
    qpc_position.checked_mul(100)
}

pub struct SystemLoopbackSource {
    capture_thread: Option<std::thread::JoinHandle<()>>,
    dispatch_thread: Option<std::thread::JoinHandle<()>>,
    stop_tx: std::sync::mpsc::SyncSender<()>,
    counters: CaptureObservationCounters,
    runtime_event_rx: Option<SourceRuntimeEventReceiver>,
    source_id: SourceId,
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

    pub fn source_id(&self) -> SourceId {
        self.source.source_id()
    }

    pub fn observation_handle(&self) -> CaptureObservationHandle {
        self.source.observation_handle()
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
        open_cancellation: &OpenCancellation,
    ) -> Result<
        (
            windows::Win32::Media::Audio::IAudioClient,
            ProcessInstanceFingerprint,
        ),
        LoopbackError,
    > {
        let opening_instance = verify_process_instance(process_id, expected_instance, stable_key)?;
        let client_result = match self {
            Self::TargetProcessTree => {
                activate_process_loopback_client(process_id, open_cancellation)
            }
        };
        match client_result {
            Ok(client) => {
                verify_process_instance(process_id, Some(opening_instance), stable_key)?;
                Ok((client, opening_instance))
            }
            Err(activation_error) if open_cancellation.is_cancelled() => {
                Err(LoopbackError::BackendInit(activation_error.to_string()))
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

struct WorkerExitNotifier {
    exit_tx: Option<std::sync::mpsc::SyncSender<()>>,
}

impl WorkerExitNotifier {
    fn new(exit_tx: std::sync::mpsc::SyncSender<()>) -> Self {
        Self {
            exit_tx: Some(exit_tx),
        }
    }
}

/// Drop contract — setup worker only: one nonblocking exit notification;
/// never runs on an audio callback or realtime partition.
impl Drop for WorkerExitNotifier {
    fn drop(&mut self) {
        if let Some(exit_tx) = self.exit_tx.take() {
            let _ = exit_tx.try_send(());
        }
    }
}

#[windows_core::implement(windows::Win32::Media::Audio::IActivateAudioInterfaceCompletionHandler)]
struct ProcessLoopbackActivationHandler {
    completion_tx: std::sync::mpsc::SyncSender<()>,
    _activation_parameters: ProcessLoopbackActivationParameters,
}

struct ProcessLoopbackActivationParameters {
    _parameters: Box<windows::Win32::Media::Audio::AUDIOCLIENT_ACTIVATION_PARAMS>,
    property: Box<windows_core::imp::PROPVARIANT>,
}

impl ProcessLoopbackActivationParameters {
    fn new(process_id: u32) -> Self {
        use windows::Win32::Media::Audio::{
            AUDIOCLIENT_ACTIVATION_PARAMS, AUDIOCLIENT_ACTIVATION_PARAMS_0,
            AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK, AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS,
            PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE,
        };
        use windows::Win32::System::Variant::VT_BLOB;

        let mut parameters = Box::new(AUDIOCLIENT_ACTIVATION_PARAMS {
            ActivationType: AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK,
            Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0 {
                ProcessLoopbackParams: AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
                    TargetProcessId: process_id,
                    ProcessLoopbackMode: PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE,
                },
            },
        });
        let property = Box::new(windows_core::imp::PROPVARIANT {
            Anonymous: windows_core::imp::PROPVARIANT_0 {
                Anonymous: windows_core::imp::PROPVARIANT_0_0 {
                    vt: VT_BLOB.0,
                    wReserved1: 0,
                    wReserved2: 0,
                    wReserved3: 0,
                    Anonymous: windows_core::imp::PROPVARIANT_0_0_0 {
                        blob: windows_core::imp::BLOB {
                            cbSize: size_of::<AUDIOCLIENT_ACTIVATION_PARAMS>() as u32,
                            pBlobData: std::ptr::from_mut(parameters.as_mut()).cast(),
                        },
                    },
                },
            },
        });
        Self {
            _parameters: parameters,
            property,
        }
    }

    fn property_ptr(&self) -> *const windows_core::PROPVARIANT {
        std::ptr::from_ref(self.property.as_ref()).cast()
    }
}

// SAFETY: both allocations are immutable after construction. The only raw
// pointer in `property` targets `_parameters`, whose Box has a stable address
// and exactly the same lifetime. Windows only reads these activation inputs.
unsafe impl Send for ProcessLoopbackActivationParameters {}
// SAFETY: see Send; concurrent access is read-only and both allocations remain
// owned by the Windows-retained completion handler.
unsafe impl Sync for ProcessLoopbackActivationParameters {}

impl windows::Win32::Media::Audio::IActivateAudioInterfaceCompletionHandler_Impl
    for ProcessLoopbackActivationHandler_Impl
{
    fn ActivateCompleted(
        &self,
        _activate_operation: Option<
            &windows::Win32::Media::Audio::IActivateAudioInterfaceAsyncOperation,
        >,
    ) -> windows_core::Result<()> {
        let _ = self.completion_tx.try_send(());
        Ok(())
    }
}

fn activate_process_loopback_client(
    process_id: u32,
    open_cancellation: &OpenCancellation,
) -> windows_core::Result<windows::Win32::Media::Audio::IAudioClient> {
    use windows::Win32::Foundation::{ERROR_TIMEOUT, E_ABORT, E_FAIL, E_POINTER};
    use windows::Win32::Media::Audio::{
        ActivateAudioInterfaceAsync, IActivateAudioInterfaceCompletionHandler, IAudioClient,
        VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK,
    };
    use windows_core::{IUnknown, Interface};

    let (completion_tx, completion_rx) = std::sync::mpsc::sync_channel(1);
    let activation_parameters = ProcessLoopbackActivationParameters::new(process_id);
    let property_ptr = activation_parameters.property_ptr();
    let completion_handler: IActivateAudioInterfaceCompletionHandler =
        ProcessLoopbackActivationHandler {
            completion_tx,
            _activation_parameters: activation_parameters,
        }
        .into();

    // SAFETY: Windows retains completion_handler until the callback executes;
    // it owns both activation structures at stable heap addresses.
    let activation_operation = unsafe {
        ActivateAudioInterfaceAsync(
            VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK,
            &IAudioClient::IID,
            Some(property_ptr),
            &completion_handler,
        )
    }?;

    match wait_for_completion(
        &completion_rx,
        open_cancellation,
        PROCESS_ACTIVATION_TIMEOUT_DURATION,
        PROCESS_ACTIVATION_CANCELLATION_POLL_DURATION,
    ) {
        CancellableWaitOutcome::Completed => {}
        CancellableWaitOutcome::Cancelled => {
            return Err(windows_core::Error::new(
                E_ABORT,
                "WASAPI process-loopback activation cancelled",
            ));
        }
        CancellableWaitOutcome::TimedOut => {
            return Err(windows_core::Error::new(
                windows_core::HRESULT::from_win32(ERROR_TIMEOUT.0),
                format!(
                    "WASAPI process-loopback activation did not complete within {} ms",
                    PROCESS_ACTIVATION_TIMEOUT_DURATION.as_millis()
                ),
            ));
        }
        CancellableWaitOutcome::ProducerExited => {
            return Err(windows_core::Error::new(
                E_FAIL,
                "WASAPI activation completion handler exited before reporting a result",
            ));
        }
    }

    let mut activation_result = windows_core::HRESULT::default();
    let mut activated_interface: Option<IUnknown> = None;
    // SAFETY: completion was reported for activation_operation and both out
    // parameters remain valid for the duration of the call.
    unsafe {
        activation_operation.GetActivateResult(&mut activation_result, &mut activated_interface)
    }?;
    activation_result.ok()?;
    activated_interface
        .ok_or_else(|| windows_core::Error::from_hresult(E_POINTER))
        .and_then(|interface| interface.cast())
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
        let (open_tx, open_rx) =
            std::sync::mpsc::sync_channel::<Result<SourceId, LoopbackError>>(1);
        let (worker_exit_tx, worker_exit_rx) = std::sync::mpsc::sync_channel::<()>(1);
        let (mut frame_producer, mut frame_consumer) =
            rtrb::RingBuffer::<AudioFrame>::new(DISPATCH_QUEUE_CAPACITY_FRAMES);
        let pool = AudioBufferPool::new(CAPTURE_POOL_CAPACITY_FRAMES, CAPTURE_FRAME_SAMPLES);
        let sequence_number = Arc::new(AtomicU64::new(0));
        let counters = CaptureObservationCounters::default();
        let capture_counters = counters.clone();
        let open_cancellation = OpenCancellation::default();
        let worker_cancellation = open_cancellation.clone();
        let capture_thread = std::thread::Builder::new()
            .name("pks-wasapi-capture".into())
            .spawn(move || {
                let _exit_notifier = WorkerExitNotifier::new(worker_exit_tx);
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
                let _audio_thread_priority = WindowsAudioThreadPriorityGuard::enter();

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
                        let source_id = StableSourceId::new(
                            Platform::Windows,
                            SourceKind::Application,
                            stable_key.clone(),
                        )
                        .source_id();
                        run_process_loopback(
                            process_id,
                            None,
                            &stable_key,
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
                    }
                    CaptureMode::ExactApplication {
                        process_id,
                        stable_id,
                    } => {
                        let source_id = stable_id.source_id();
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

        let source_id = match wait_for_open(&open_rx, BACKEND_OPEN_TIMEOUT_DURATION) {
            OpenWaitOutcome::Opened(source_id) => source_id,
            OpenWaitOutcome::Failed(error) => {
                open_cancellation.cancel();
                let _ = stop_tx.try_send(());
                crate::capture::join_capture_worker(capture_thread, "Windows capture open")?;
                return Err(error);
            }
            OpenWaitOutcome::TimedOut => {
                open_cancellation.cancel();
                let _ = stop_tx.try_send(());
                match worker_exit_rx.recv_timeout(OPEN_WORKER_CANCELLATION_GRACE_DURATION) {
                    Ok(()) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        crate::capture::join_capture_worker(
                            capture_thread,
                            "Windows cancelled capture open",
                        )?;
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        drop(capture_thread);
                    }
                }
                return Err(LoopbackError::BackendInit(format!(
                    "WASAPI stream did not open within {} ms",
                    BACKEND_OPEN_TIMEOUT_DURATION.as_millis()
                )));
            }
            OpenWaitOutcome::WorkerExited => {
                open_cancellation.cancel();
                let _ = stop_tx.try_send(());
                crate::capture::join_capture_worker(capture_thread, "Windows capture open")?;
                return Err(LoopbackError::BackendInit(
                    "WASAPI capture worker exited before reporting open status".to_owned(),
                ));
            }
        };

        let dispatch_thread = match std::thread::Builder::new()
            .name("pks-wasapi-dispatch".into())
            .spawn(move || {
                let _audio_thread_priority = WindowsAudioThreadPriorityGuard::enter();
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
                crate::capture::join_capture_worker(capture_thread, "Windows capture open")?;
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
            source_id,
        })
    }

    pub fn observations(&self) -> CaptureObservations {
        self.counters.snapshot()
    }

    pub fn observation_handle(&self) -> CaptureObservationHandle {
        self.counters.observation_handle()
    }

    pub fn source_id(&self) -> SourceId {
        self.source_id
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
                receiver.observation_handle().observations()
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
            crate::capture::join_capture_worker(thread, "Windows capture")
        });
        let dispatch_join = self.dispatch_thread.take().map_or(Ok(()), |thread| {
            crate::capture::join_capture_worker(thread, "Windows dispatch")
        });
        capture_join.and(dispatch_join)
    }
}

fn select_process_source(
    sources: &[crate::capture::CaptureSource],
    process_id: u32,
) -> Result<StableSourceId, LoopbackError> {
    let mut matches = sources.iter().filter(|source| {
        source.stable_id.kind == SourceKind::Application && source.process_id == Some(process_id)
    });
    let source = matches
        .next()
        .ok_or_else(|| LoopbackError::SourceUnavailable {
            stable_key: format!("wasapi:pid:{process_id}"),
        })?;
    if matches.any(|candidate| candidate.stable_id != source.stable_id) {
        return Err(LoopbackError::BackendInit(format!(
            "process '{process_id}' exposes more than one audio source identity"
        )));
    }
    Ok(source.stable_id.clone())
}

fn resolve_application_mode(mode: CaptureMode) -> Result<CaptureMode, LoopbackError> {
    match mode {
        CaptureMode::Application(name) => {
            let sources = discover_sources_windows();
            let mut matches = sources.iter().filter(|source| {
                source.name.eq_ignore_ascii_case(&name)
                    || source
                        .app_id
                        .as_deref()
                        .is_some_and(|app_id| app_id.eq_ignore_ascii_case(&name))
            });
            let source = matches.next().ok_or_else(|| {
                LoopbackError::BackendInit(format!(
                    "no running audio source matches application '{name}'"
                ))
            })?;
            if matches.any(|candidate| candidate.stable_id != source.stable_id) {
                return Err(LoopbackError::BackendInit(format!(
                    "application '{name}' matches multiple audio sessions — select one from source discovery"
                )));
            }
            let process_id = source.process_id.ok_or_else(|| {
                LoopbackError::BackendInit(format!(
                    "audio source for application '{name}' has no process identity"
                ))
            })?;
            Ok(CaptureMode::ExactApplication {
                process_id,
                stable_id: source.stable_id.clone(),
            })
        }
        CaptureMode::Process(process_id) => {
            let stable_id = select_process_source(&discover_sources_windows(), process_id)?;
            Ok(CaptureMode::ExactApplication {
                process_id,
                stable_id,
            })
        }
        CaptureMode::ExactApplicationStable { stable_id } => {
            if stable_id.platform != Platform::Windows || stable_id.kind != SourceKind::Application
            {
                return Err(LoopbackError::SourceUnavailable {
                    stable_key: stable_id.stable_key,
                });
            }
            let Some(process_instance) = ProcessInstanceFingerprint::parse(&stable_id.stable_key)
            else {
                return Err(LoopbackError::SourceUnavailable {
                    stable_key: stable_id.stable_key,
                });
            };
            if process_instance.process_id == 0 || process_instance.creation_time_100ns == 0 {
                return Err(LoopbackError::SourceUnavailable {
                    stable_key: stable_id.stable_key,
                });
            }
            Ok(CaptureMode::ExactApplication {
                process_id: process_instance.process_id,
                stable_id,
            })
        }
        other => Ok(other),
    }
}

#[cfg(test)]
mod application_selection_tests {
    use super::*;
    use crate::capture::{CaptureSource, SourceState};

    fn application_source(process_id: u32, stable_id: StableSourceId) -> CaptureSource {
        CaptureSource {
            stable_id,
            name: "Application".to_owned(),
            process_id: Some(process_id),
            app_id: None,
            device_uid: None,
            state: SourceState::Playing,
            sample_rate_hz: 48_000,
            channels: 2,
        }
    }

    #[test]
    fn given_process_id_when_resolved_then_discovered_instance_identity_is_preserved() {
        let stable_id = StableSourceId::new(
            Platform::Windows,
            SourceKind::Application,
            ProcessInstanceFingerprint::new(42, 133_980_144_000_000_000).stable_key(),
        );
        let sources = vec![application_source(42, stable_id.clone())];

        assert_eq!(select_process_source(&sources, 42), Ok(stable_id));
    }

    #[test]
    fn given_discovered_stable_identity_when_resolved_then_exact_process_instance_is_retained() {
        let fingerprint = ProcessInstanceFingerprint::new(42, 133_980_144_000_000_000);
        let stable_id = StableSourceId::new(
            Platform::Windows,
            SourceKind::Application,
            fingerprint.stable_key(),
        );

        assert_eq!(
            resolve_application_mode(CaptureMode::ExactApplicationStable {
                stable_id: stable_id.clone(),
            }),
            Ok(CaptureMode::ExactApplication {
                process_id: 42,
                stable_id,
            })
        );
    }

    #[test]
    fn given_foreign_stable_identity_when_resolved_then_selection_fails_closed() {
        let stable_id =
            StableSourceId::new(Platform::Macos, SourceKind::Application, "com.acme.meeting");

        assert!(matches!(
            resolve_application_mode(CaptureMode::ExactApplicationStable { stable_id }),
            Err(LoopbackError::SourceUnavailable { stable_key })
                if stable_key == "com.acme.meeting"
        ));
    }
}

/// Drop contract — control thread only: signal once, join both owned workers,
/// never execute from a capture callback or realtime partition.
impl Drop for SystemLoopbackSource {
    fn drop(&mut self) {
        let _ = self.stop_workers();
    }
}

/// Registers one owned Windows audio worker with MMCSS for its lifetime.
///
/// This is an internal worker primitive, not a public product extension API.
/// Unsupported or denied priority registration remains non-fatal, while the
/// existing bounded-queue observations still fail closed on real loss.
pub struct WindowsAudioThreadPriorityGuard {
    mmcss_handle: Option<windows::Win32::Foundation::HANDLE>,
    timer_period_active: bool,
}

impl WindowsAudioThreadPriorityGuard {
    pub fn enter() -> Self {
        use windows::Win32::Media::timeBeginPeriod;
        use windows::Win32::System::Threading::AvSetMmThreadCharacteristicsW;
        use windows_core::w;

        // SAFETY: both APIs affect only the calling thread/process timer
        // request. Successful effects are paired in Drop on this same owned
        // worker thread.
        unsafe {
            let timer_period_active = timeBeginPeriod(1) == 0;
            let mut task_index: u32 = 0;
            let mmcss_handle = AvSetMmThreadCharacteristicsW(w!("Audio"), &mut task_index).ok();
            Self {
                mmcss_handle,
                timer_period_active,
            }
        }
    }
}

impl Drop for WindowsAudioThreadPriorityGuard {
    fn drop(&mut self) {
        use windows::Win32::Media::timeEndPeriod;
        use windows::Win32::System::Threading::AvRevertMmThreadCharacteristics;

        // SAFETY: each handle and timer request was created by `enter` on this
        // worker and is released exactly once during owned worker teardown.
        unsafe {
            if let Some(handle) = self.mmcss_handle.take() {
                let _ = AvRevertMmThreadCharacteristics(handle);
            }
            if self.timer_period_active {
                let _ = timeEndPeriod(1);
            }
        }
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
    callback_samples: &[f32],
    callback_timestamp_ns: u64,
    source_id: SourceId,
    pool: &Arc<AudioBufferPool>,
    sequence_number: &Arc<AtomicU64>,
    counters: &CaptureObservationCounters,
    frame_normalizer: &mut CaptureFrameNormalizer,
    callback: &mut (impl FnMut(AudioFrame) + Send + 'static),
) {
    counters.observe_callback_buffer();
    if callback_samples.is_empty() {
        counters.observe_invalid_buffer();
        return;
    }
    let normalized = frame_normalizer.push(
        callback_samples,
        callback_timestamp_ns,
        |timestamp_ns, samples| {
            let frame_sequence_number = sequence_number.fetch_add(1, Ordering::Relaxed);
            let mut handle = match pool.acquire() {
                Some(handle) => handle,
                None => {
                    counters.observe_pool_exhaustion();
                    return;
                }
            };
            if handle.try_copy_from_slice(samples).is_err() {
                counters.observe_oversized_buffer();
                return;
            }
            let mut frame = AudioFrame::new(
                StreamId(0),
                source_id,
                frame_sequence_number,
                timestamp_ns,
                CAPTURE_CHANNEL_COUNT,
                handle,
            );
            frame.sample_rate_hz = SAMPLE_RATE_HZ;
            callback(frame);
        },
    );
    if !normalized {
        counters.observe_invalid_buffer();
    }
}

fn f32_samples_as_bytes_mut(samples: &mut [f32]) -> &mut [u8] {
    let byte_count = std::mem::size_of_val(samples);
    // SAFETY: `u8` has alignment one and the byte slice covers exactly the
    // initialized f32 storage for the duration of this exclusive borrow.
    unsafe { std::slice::from_raw_parts_mut(samples.as_mut_ptr().cast::<u8>(), byte_count) }
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
    open_tx: std::sync::mpsc::SyncSender<Result<SourceId, LoopbackError>>,
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
    Windows {
        operation: &'static str,
        error: windows_core::Error,
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
            Self::Wasapi { operation, .. }
            | Self::Windows { operation, .. }
            | Self::BackendClass { operation, .. } => operation,
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
            Self::Windows { error, .. } => CaptureRuntimeFailureClass::PlatformStatus {
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
            Self::Windows { error, .. } => classify_platform_status(error.code().0),
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
            Self::Windows { operation, error } => LoopbackError::BackendStatus {
                operation,
                status_code: error.code().0,
            },
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
    let mut callback_samples = vec![0.0f32; WASAPI_CALLBACK_MAX_SAMPLES].into_boxed_slice();
    let mut frame_normalizer = CaptureFrameNormalizer::new(
        CAPTURE_FRAME_SAMPLES_PER_CHANNEL,
        CAPTURE_CHANNEL_COUNT,
        SAMPLE_RATE_HZ,
    );

    let capture_result: Result<(), CaptureLoopFailure> = (|| {
        let timestamp_mapping =
            WasapiTimestampMapping::new().ok_or(CaptureLoopFailure::BackendClass {
                operation: "initialize WASAPI QPC timestamp mapping",
                class: "qpc-clock-unavailable",
            })?;
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
                match plan_packet_read(
                    next,
                    CAPTURE_CHANNEL_COUNT,
                    size_of::<f32>(),
                    sample_buffer_capacity_bytes(&callback_samples),
                ) {
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
                match capture_client
                    .read_from_device(f32_samples_as_bytes_mut(&mut callback_samples))
                {
                    Ok((frames, info)) => {
                        if frames == 0 {
                            continue;
                        }
                        let bytes = match plan_packet_read(
                            frames,
                            CAPTURE_CHANNEL_COUNT,
                            size_of::<f32>(),
                            sample_buffer_capacity_bytes(&callback_samples),
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
                        let sample_count = bytes / size_of::<f32>();
                        if info.flags.silent {
                            callback_samples[..sample_count].fill(0.0);
                        }
                        if info.flags.timestamp_error {
                            state.counters.observe_invalid_buffer();
                            continue;
                        }
                        let qpc_timestamp_ns = wasapi_qpc_100ns_to_ns(info.timestamp).ok_or(
                            CaptureLoopFailure::BackendClass {
                                operation: "convert WASAPI packet timestamp",
                                class: "qpc-timestamp-out-of-range",
                            },
                        )?;
                        let callback_timestamp_ns = timestamp_mapping
                            .to_monotonic_ns(qpc_timestamp_ns)
                            .ok_or(CaptureLoopFailure::BackendClass {
                                operation: "map WASAPI packet timestamp",
                                class: "qpc-timestamp-out-of-range",
                            })?;
                        deliver_packet(
                            &callback_samples[..sample_count],
                            callback_timestamp_ns,
                            state.source_id,
                            &state.pool,
                            &state.sequence,
                            &state.counters,
                            &mut frame_normalizer,
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

struct ProcessLoopbackStream {
    audio_client: windows::Win32::Media::Audio::IAudioClient,
    capture_client: windows::Win32::Media::Audio::IAudioCaptureClient,
    event_handle: windows::Win32::Foundation::HANDLE,
}

impl ProcessLoopbackStream {
    fn initialize(
        audio_client: windows::Win32::Media::Audio::IAudioClient,
    ) -> windows_core::Result<Self> {
        use windows::Win32::Media::Audio::{
            IAudioCaptureClient, AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM,
            AUDCLNT_STREAMFLAGS_EVENTCALLBACK, AUDCLNT_STREAMFLAGS_LOOPBACK,
            AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY, WAVEFORMATEX,
        };
        use windows::Win32::System::Threading::CreateEventW;
        use windows_core::PCWSTR;

        const IEEE_FLOAT_FORMAT_TAG: u16 = 3;
        const BITS_PER_SAMPLE: u16 = 32;
        const BYTES_PER_SAMPLE: u16 = BITS_PER_SAMPLE / 8;
        let block_alignment = u16::from(CAPTURE_CHANNEL_COUNT) * BYTES_PER_SAMPLE;
        let wave_format = WAVEFORMATEX {
            wFormatTag: IEEE_FLOAT_FORMAT_TAG,
            nChannels: u16::from(CAPTURE_CHANNEL_COUNT),
            nSamplesPerSec: SAMPLE_RATE_HZ,
            nAvgBytesPerSec: SAMPLE_RATE_HZ * u32::from(block_alignment),
            nBlockAlign: block_alignment,
            wBitsPerSample: BITS_PER_SAMPLE,
            cbSize: 0,
        };
        let stream_flags = AUDCLNT_STREAMFLAGS_LOOPBACK
            | AUDCLNT_STREAMFLAGS_EVENTCALLBACK
            | AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM
            | AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY;
        // SAFETY: wave_format is a complete WAVEFORMATEX that remains alive
        // throughout Initialize; process loopback is always shared mode.
        unsafe {
            audio_client.Initialize(
                AUDCLNT_SHAREMODE_SHARED,
                stream_flags,
                WASAPI_PROCESS_LOOPBACK_PERIOD_100NS,
                0,
                &wave_format,
                None,
            )
        }?;
        // SAFETY: unnamed auto-reset event with default security attributes.
        let event_handle = unsafe { CreateEventW(None, false, false, PCWSTR::null()) }?;
        // SAFETY: event_handle remains owned by the returned stream.
        if let Err(error) = unsafe { audio_client.SetEventHandle(event_handle) } {
            // SAFETY: SetEventHandle failed, so ownership was not transferred.
            let _ = unsafe { windows::Win32::Foundation::CloseHandle(event_handle) };
            return Err(error);
        }
        // SAFETY: the initialized client exposes IAudioCaptureClient.
        let capture_client = match unsafe { audio_client.GetService::<IAudioCaptureClient>() } {
            Ok(client) => client,
            Err(error) => {
                // SAFETY: event_handle remains uniquely owned here.
                let _ = unsafe { windows::Win32::Foundation::CloseHandle(event_handle) };
                return Err(error);
            }
        };
        Ok(Self {
            audio_client,
            capture_client,
            event_handle,
        })
    }

    fn start(&self) -> windows_core::Result<()> {
        // SAFETY: audio_client is initialized and the event handle is set.
        unsafe { self.audio_client.Start() }
    }

    fn stop(&self) {
        // SAFETY: Stop is valid after successful initialization and is
        // deliberately best-effort during teardown.
        let _ = unsafe { self.audio_client.Stop() };
    }
}

/// Drop contract — capture worker only: close one kernel event;
/// no allocation, lock, block, async operation, logging, or panic.
impl Drop for ProcessLoopbackStream {
    fn drop(&mut self) {
        // SAFETY: event_handle is uniquely owned by Self and closed once.
        let _ = unsafe { windows::Win32::Foundation::CloseHandle(self.event_handle) };
    }
}

fn capture_process_loopback(
    stream: &ProcessLoopbackStream,
    state: CaptureLoopState,
    mut callback: impl FnMut(AudioFrame) + Send + 'static,
) -> Result<(), LoopbackError> {
    use windows::Win32::Foundation::{GetLastError, WAIT_FAILED, WAIT_OBJECT_0, WAIT_TIMEOUT};
    use windows::Win32::Media::Audio::AUDCLNT_BUFFERFLAGS_SILENT;
    use windows::Win32::System::Threading::WaitForSingleObject;
    use windows_core::HRESULT;

    let mut callback_samples = vec![0.0f32; WASAPI_CALLBACK_MAX_SAMPLES].into_boxed_slice();
    let mut frame_normalizer = CaptureFrameNormalizer::new(
        CAPTURE_FRAME_SAMPLES_PER_CHANNEL,
        CAPTURE_CHANNEL_COUNT,
        SAMPLE_RATE_HZ,
    );
    let capture_result: Result<(), CaptureLoopFailure> = (|| {
        let timestamp_mapping =
            WasapiTimestampMapping::new().ok_or(CaptureLoopFailure::BackendClass {
                operation: "initialize WASAPI process-loopback QPC timestamp mapping",
                class: "qpc-clock-unavailable",
            })?;
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
            // SAFETY: event_handle remains valid for stream's lifetime.
            match unsafe { WaitForSingleObject(stream.event_handle, WAIT_TIMEOUT_MS) } {
                WAIT_OBJECT_0 => {}
                WAIT_TIMEOUT => continue,
                WAIT_FAILED => {
                    // SAFETY: called immediately after failed wait.
                    let error = windows_core::Error::from_hresult(HRESULT::from_win32(
                        unsafe { GetLastError() }.0,
                    ));
                    return Err(CaptureLoopFailure::Windows {
                        operation: "wait for WASAPI process-loopback capture event",
                        error,
                    });
                }
                status => {
                    return Err(CaptureLoopFailure::BackendClass {
                        operation: "wait for WASAPI process-loopback capture event",
                        class: if status.0 == WAIT_FAILED.0 {
                            "wait-failed"
                        } else {
                            "unexpected-wait-status"
                        },
                    });
                }
            }
            loop {
                // SAFETY: capture_client belongs to the initialized stream.
                let announced_frames = unsafe { stream.capture_client.GetNextPacketSize() }
                    .map_err(|error| CaptureLoopFailure::Windows {
                        operation: "query next WASAPI process-loopback packet size",
                        error,
                    })?;
                match plan_packet_read(
                    announced_frames,
                    CAPTURE_CHANNEL_COUNT,
                    size_of::<f32>(),
                    sample_buffer_capacity_bytes(&callback_samples),
                ) {
                    PacketReadPlan::Empty => break,
                    PacketReadPlan::Read { .. } => {}
                    PacketReadPlan::Oversized { .. } => {
                        state.counters.observe_oversized_buffer();
                        return Err(CaptureLoopFailure::BackendClass {
                            operation: "validate announced WASAPI process-loopback packet size",
                            class: "announced-packet-oversized",
                        });
                    }
                }

                let mut data = std::ptr::null_mut();
                let mut delivered_frames = 0;
                let mut flags = 0;
                let mut device_position = 0u64;
                let mut qpc_position = 0u64;
                // SAFETY: all out pointers are valid and capture_client owns
                // the returned packet until ReleaseBuffer below.
                unsafe {
                    stream.capture_client.GetBuffer(
                        &mut data,
                        &mut delivered_frames,
                        &mut flags,
                        Some(&mut device_position),
                        Some(&mut qpc_position),
                    )
                }
                .map_err(|error| CaptureLoopFailure::Windows {
                    operation: "read WASAPI process-loopback packet",
                    error,
                })?;
                let read_plan = plan_packet_read(
                    delivered_frames,
                    CAPTURE_CHANNEL_COUNT,
                    size_of::<f32>(),
                    sample_buffer_capacity_bytes(&callback_samples),
                );
                let packet_bytes = match read_plan {
                    PacketReadPlan::Empty => 0,
                    PacketReadPlan::Read { packet_bytes } => packet_bytes,
                    PacketReadPlan::Oversized { .. } => {
                        // SAFETY: GetBuffer succeeded and this releases the
                        // exact packet before returning the validation error.
                        let _ = unsafe { stream.capture_client.ReleaseBuffer(delivered_frames) };
                        state.counters.observe_oversized_buffer();
                        return Err(CaptureLoopFailure::BackendClass {
                            operation: "validate delivered WASAPI process-loopback packet size",
                            class: "delivered-packet-oversized",
                        });
                    }
                };
                if packet_bytes > 0 {
                    let sample_count = packet_bytes / size_of::<f32>();
                    if flags & AUDCLNT_BUFFERFLAGS_SILENT.0 as u32 != 0 {
                        callback_samples[..sample_count].fill(0.0);
                    } else if data.is_null() {
                        // SAFETY: GetBuffer succeeded and this releases the
                        // packet before reporting the invalid pointer.
                        let _ = unsafe { stream.capture_client.ReleaseBuffer(delivered_frames) };
                        return Err(CaptureLoopFailure::BackendClass {
                            operation: "read WASAPI process-loopback packet",
                            class: "null-packet-data",
                        });
                    } else {
                        // SAFETY: WASAPI guarantees at least delivered frame
                        // count times nBlockAlign bytes until ReleaseBuffer.
                        let source = unsafe { std::slice::from_raw_parts(data, packet_bytes) };
                        f32_samples_as_bytes_mut(&mut callback_samples)[..packet_bytes]
                            .copy_from_slice(source);
                    }
                }
                // SAFETY: GetBuffer succeeded and the packet is released once.
                unsafe { stream.capture_client.ReleaseBuffer(delivered_frames) }.map_err(
                    |error| CaptureLoopFailure::Windows {
                        operation: "release WASAPI process-loopback packet",
                        error,
                    },
                )?;
                if packet_bytes > 0 {
                    let sample_count = packet_bytes / size_of::<f32>();
                    if flags
                        & windows::Win32::Media::Audio::AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR.0 as u32
                        != 0
                    {
                        state.counters.observe_invalid_buffer();
                        continue;
                    }
                    let qpc_timestamp_ns = wasapi_qpc_100ns_to_ns(qpc_position).ok_or(
                        CaptureLoopFailure::BackendClass {
                            operation: "convert WASAPI process-loopback packet timestamp",
                            class: "qpc-timestamp-out-of-range",
                        },
                    )?;
                    let callback_timestamp_ns = timestamp_mapping
                        .to_monotonic_ns(qpc_timestamp_ns)
                        .ok_or(CaptureLoopFailure::BackendClass {
                            operation: "map WASAPI process-loopback packet timestamp",
                            class: "qpc-timestamp-out-of-range",
                        })?;
                    deliver_packet(
                        &callback_samples[..sample_count],
                        callback_timestamp_ns,
                        state.source_id,
                        &state.pool,
                        &state.sequence,
                        &state.counters,
                        &mut frame_normalizer,
                        &mut callback,
                    );
                }
            }
        }
        Ok(())
    })();
    stream.stop();
    capture_result.map_err(|failure| {
        let _ = state
            .runtime_event_tx
            .try_send(failure.runtime_event(&state.stable_id));
        failure.into_capture_error(state.stable_id.stable_key.clone())
    })
}

fn signal_open(
    audio_client: &AudioClient,
    source_id: SourceId,
    open_tx: &std::sync::mpsc::SyncSender<Result<SourceId, LoopbackError>>,
    open_cancellation: &OpenCancellation,
) -> Result<(), LoopbackError> {
    report_open(open_tx, source_id, open_cancellation).map_err(|error| {
        let _ = audio_client.stop_stream();
        let message = match error {
            OpenReportError::Cancelled => "WASAPI open was cancelled before activation completed",
            OpenReportError::ReceiverUnavailable => "WASAPI open result receiver is unavailable",
        };
        LoopbackError::BackendInit(message.to_owned())
    })
}

fn signal_process_open(
    stream: &ProcessLoopbackStream,
    source_id: SourceId,
    open_tx: &std::sync::mpsc::SyncSender<Result<SourceId, LoopbackError>>,
    open_cancellation: &OpenCancellation,
) -> Result<(), LoopbackError> {
    report_open(open_tx, source_id, open_cancellation).map_err(|error| {
        stream.stop();
        let message = match error {
            OpenReportError::Cancelled => {
                "WASAPI process-loopback open was cancelled before activation completed"
            }
            OpenReportError::ReceiverUnavailable => {
                "WASAPI process-loopback open result receiver is unavailable"
            }
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
    let stable_id = StableSourceId::new(Platform::Windows, SourceKind::SystemMix, "system:mix");
    let source_id = stable_id.source_id();
    signal_open(
        &audio_client,
        source_id,
        &context.open_tx,
        &context.open_cancellation,
    )?;
    capture_loop(
        &audio_client,
        &capture_client,
        &h_event,
        context.into_loop_state(source_id, stable_id, None),
        callback,
    )
}

/// Enumerate all audio capture sources visible on this Windows system.
///
/// Always returns at least one entry (the system-wide mix at id=0).
/// Per-process application sources are appended via WASAPI session enumeration.
pub fn discover_sources_windows() -> Vec<crate::capture::CaptureSource> {
    use crate::capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use crate::frame::Platform;

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

fn enumerate_wasapi_input_devices() -> Vec<crate::capture::CaptureSource> {
    use crate::capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use crate::frame::Platform;

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
unsafe fn enumerate_wasapi_sessions() -> Vec<crate::capture::CaptureSource> {
    use crate::capture::{CaptureSource, SourceKind, SourceState, StableSourceId};
    use crate::frame::Platform;
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

        // Resolve the display name and the native application identity from
        // the same process incarnation used to create the source identity.
        let (name, application_id) =
            match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                Ok(handle) => {
                    let mut buf = vec![0u16; 260];
                    let mut len = buf.len() as u32;
                    let application_id = query_process_application_id(handle);
                    let name_result = QueryFullProcessImageNameW(
                        handle,
                        PROCESS_NAME_WIN32,
                        PWSTR(buf.as_mut_ptr()),
                        &mut len,
                    );
                    let _ = CloseHandle(handle);
                    let name = match name_result {
                        Ok(()) => {
                            let path = String::from_utf16_lossy(&buf[..len as usize]);
                            std::path::Path::new(&path)
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or(&path)
                                .to_owned()
                        }
                        Err(_) => format!("pid-{pid}"),
                    };
                    (name, application_id)
                }
                Err(_) => (format!("pid-{pid}"), None),
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
            app_id: application_id,
            device_uid: None,
            state: source_state,
            sample_rate_hz: 48_000,
            channels: 2,
        });
    }

    sources
}

unsafe fn query_process_application_id(
    process: windows::Win32::Foundation::HANDLE,
) -> Option<String> {
    use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS};
    use windows::Win32::Storage::Packaging::Appx::GetApplicationUserModelId;
    use windows_core::PWSTR;

    let mut length_chars = 0u32;
    // SAFETY: this first call requests only the required finite buffer size.
    let status = unsafe {
        GetApplicationUserModelId(process, &mut length_chars, PWSTR(std::ptr::null_mut()))
    };
    if status != ERROR_INSUFFICIENT_BUFFER || length_chars <= 1 {
        return None;
    }
    let mut application_id = vec![0u16; usize::try_from(length_chars).ok()?];
    // SAFETY: the buffer contains exactly the capacity requested by the first
    // call and remains live for the complete call.
    let status = unsafe {
        GetApplicationUserModelId(
            process,
            &mut length_chars,
            PWSTR(application_id.as_mut_ptr()),
        )
    };
    if status != ERROR_SUCCESS || length_chars <= 1 {
        return None;
    }
    let string_length = usize::try_from(length_chars.saturating_sub(1)).ok()?;
    String::from_utf16(&application_id[..string_length]).ok()
}

fn run_process_loopback(
    process_id: u32,
    expected_instance: Option<ProcessInstanceFingerprint>,
    stable_key: &str,
    source_id: SourceId,
    context: CaptureWorkerContext,
    callback: impl FnMut(AudioFrame) + Send + 'static,
) -> Result<(), LoopbackError> {
    let (audio_client, opening_instance) = ProcessLoopbackScope::TargetProcessTree.open_client(
        process_id,
        expected_instance,
        stable_key,
        &context.open_cancellation,
    )?;
    let process_watch = ProcessInstanceWatch::open(process_id, opening_instance, stable_key)?;
    if context.open_cancellation.is_cancelled() {
        return Err(LoopbackError::BackendInit(
            "WASAPI process-loopback activation completed after open cancellation".to_owned(),
        ));
    }
    let stream = ProcessLoopbackStream::initialize(audio_client)
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    stream
        .start()
        .map_err(|error| LoopbackError::BackendInit(error.to_string()))?;
    signal_process_open(
        &stream,
        source_id,
        &context.open_tx,
        &context.open_cancellation,
    )?;
    capture_process_loopback(
        &stream,
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
    let source_id = stable_id.source_id();
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
    signal_open(
        &audio_client,
        source_id,
        &context.open_tx,
        &context.open_cancellation,
    )?;
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

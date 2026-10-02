//! One WASAPI microphone owner with same-open processing evidence.
//!
//! All COM setup and effect inspection runs on the capture worker before it
//! reports open. Notifications atomically invalidate route and processing
//! facts; they do not query COM, allocate, or process PCM in the callback.

use std::mem::size_of;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use windows::core::{implement, Interface, GUID, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, BOOL, HANDLE};
use windows::Win32::Media::Audio::{
    eCapture, eMultimedia, eRender, AudioCategory_Communications, AudioClientProperties, EDataFlow,
    ERole, IAcousticEchoCancellationControl, IAudioCaptureClient, IAudioClient, IAudioClient2,
    IAudioEffectsChangedNotificationClient, IAudioEffectsChangedNotificationClient_Impl,
    IAudioEffectsManager, IMMDevice, IMMDeviceEnumerator, IMMNotificationClient,
    IMMNotificationClient_Impl, MMDeviceEnumerator, AUDCLNT_SHAREMODE_SHARED,
    AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM, AUDCLNT_STREAMFLAGS_EVENTCALLBACK,
    AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY, AUDCLNT_STREAMOPTIONS_NONE, AUDIO_EFFECT,
    AUDIO_EFFECT_STATE_ON, DEVICE_STATE_ACTIVE, WAVEFORMATEX, WAVEFORMATEXTENSIBLE,
    WAVEFORMATEXTENSIBLE_0,
};
use windows::Win32::Media::KernelStreaming::{
    AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION, WAVE_FORMAT_EXTENSIBLE,
};
use windows::Win32::System::Com::{CoCreateInstance, CoTaskMemFree, CLSCTX_ALL};
use windows::Win32::System::Threading::CreateEventW;
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

use crate::capture::{
    capture_processing_observations, native_aec_route, CaptureError,
    CaptureProcessingObservationHandle, CaptureProcessingObservations, CaptureProcessingReporter,
    InputDeviceSelector, NativeAecRequest, NativeAecRoute, NativeAecRouteHandle,
    NativeAecRouteReporter, SourceKind, StableSourceId,
};
use crate::frame::{Platform, SourceId, SAMPLE_RATE_HZ};

use super::source::{BUFFER_DURATION_100NS, CAPTURE_CHANNEL_COUNT};

const AEC_UNAVAILABLE: &str =
    "selected Windows capture endpoint has no active controllable AEC effect";
// KSDATAFORMAT_SUBTYPE_IEEE_FLOAT is exposed by Windows 0.58's Multimedia
// feature; use the published GUID without adding another crate feature.
const IEEE_FLOAT_SUBFORMAT: GUID = GUID::from_u128(0x00000003_0000_0010_8000_00aa00389b71);

/// The same initialized IAudioClient supplies effects evidence and PCM.
/// A native request also binds and verifies the exact render endpoint.
pub(super) struct WindowsInputStream {
    pub(super) audio_client: IAudioClient,
    pub(super) capture_client: IAudioCaptureClient,
    pub(super) event_handle: HANDLE,
    monitor: Option<NativeRouteMonitor>,
    stable_id: StableSourceId,
    route_handle: Option<NativeAecRouteHandle>,
    processing_reporter: CaptureProcessingReporter,
    processing_handle: CaptureProcessingObservationHandle,
}

impl WindowsInputStream {
    pub(super) fn open(
        selector: InputDeviceSelector,
        request: Option<&NativeAecRequest>,
    ) -> Result<Self, CaptureError> {
        // SAFETY: COM MTA is initialized by the dedicated Windows capture worker.
        let enumerator: IMMDeviceEnumerator =
            unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }
                .map_err(|error| backend_error("enumerate Windows audio endpoints", error))?;
        let input = match selector {
            InputDeviceSelector::Default => {
                // SAFETY: enumerator is live in this COM apartment.
                unsafe { enumerator.GetDefaultAudioEndpoint(eCapture, eMultimedia) }
                    .map_err(|error| backend_error("open default Windows microphone", error))?
            }
            InputDeviceSelector::StableId(id) => {
                let wide = wide_id(&id);
                // SAFETY: wide is null-terminated and live for the COM call.
                unsafe { enumerator.GetDevice(PCWSTR(wide.as_ptr())) }
                    .map_err(|_| CaptureError::SourceUnavailable { stable_key: id })?
            }
        };
        // SAFETY: input is a live endpoint in this COM apartment.
        if unsafe { input.GetState() }
            .map_err(|error| backend_error("read Windows microphone state", error))?
            != DEVICE_STATE_ACTIVE
        {
            return Err(CaptureError::BackendInit(
                "selected Windows microphone is inactive".into(),
            ));
        }
        let microphone_id = device_id(&input)?;
        ensure_active_flow(&enumerator, eCapture, &microphone_id)?;
        let render = if let Some(request) = request {
            let render_id = request.playback_device().as_str();
            let render_wide = wide_id(render_id);
            // SAFETY: render_wide is null-terminated and live for the COM call.
            let render = unsafe { enumerator.GetDevice(PCWSTR(render_wide.as_ptr())) }
                .map_err(|error| backend_error("open exact Windows AEC render endpoint", error))?;
            // SAFETY: render is a live endpoint in this COM apartment.
            if unsafe { render.GetState() }
                .map_err(|error| backend_error("read Windows AEC render state", error))?
                != DEVICE_STATE_ACTIVE
                || device_id(&render)? != render_id
            {
                return Err(CaptureError::BackendInit(
                    "requested Windows AEC render endpoint is not active and exact".into(),
                ));
            }
            ensure_active_flow(&enumerator, eRender, render_id)?;
            Some(render)
        } else {
            None
        };

        let stable_id =
            StableSourceId::new(Platform::Windows, SourceKind::InputDevice, microphone_id);
        // SAFETY: input is an active capture endpoint and COM is initialized.
        let client: IAudioClient = unsafe { input.Activate(CLSCTX_ALL, None) }
            .map_err(|error| backend_error("activate Windows microphone audio client", error))?;
        if request.is_some() {
            let client2: IAudioClient2 = client.cast().map_err(|error| {
                backend_error("obtain Windows microphone client properties", error)
            })?;
            let properties = AudioClientProperties {
                cbSize: size_of::<AudioClientProperties>() as u32,
                bIsOffload: BOOL(0),
                eCategory: AudioCategory_Communications,
                Options: AUDCLNT_STREAMOPTIONS_NONE,
            };
            // SAFETY: properties has a valid size and remains live for this call.
            unsafe { client2.SetClientProperties(&properties) }.map_err(|error| {
                backend_error("select Windows communications processing", error)
            })?;
        }

        let block_align = u16::from(CAPTURE_CHANNEL_COUNT) * size_of::<f32>() as u16;
        // Match wasapi::WaveFormat::new(32, 32, Float, 48k, 2, None),
        // previously used by the ordinary microphone path.
        let format = WAVEFORMATEXTENSIBLE {
            Format: WAVEFORMATEX {
                wFormatTag: WAVE_FORMAT_EXTENSIBLE as u16,
                nChannels: u16::from(CAPTURE_CHANNEL_COUNT),
                nSamplesPerSec: SAMPLE_RATE_HZ,
                nAvgBytesPerSec: SAMPLE_RATE_HZ * u32::from(block_align),
                nBlockAlign: block_align,
                wBitsPerSample: 32,
                cbSize: 22,
            },
            Samples: WAVEFORMATEXTENSIBLE_0 {
                wValidBitsPerSample: 32,
            },
            dwChannelMask: (1u32 << CAPTURE_CHANNEL_COUNT) - 1,
            SubFormat: IEEE_FLOAT_SUBFORMAT,
        };
        // SAFETY: format is complete and remains live through initialization.
        unsafe {
            client.Initialize(
                AUDCLNT_SHAREMODE_SHARED,
                AUDCLNT_STREAMFLAGS_EVENTCALLBACK
                    | AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM
                    | AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY,
                BUFFER_DURATION_100NS,
                0,
                &format.Format,
                None,
            )
        }
        .map_err(|error| backend_error("initialize Windows microphone capture", error))?;

        if let Some(request) = request {
            // SAFETY: the audio client is initialized in the current COM apartment.
            let control: IAcousticEchoCancellationControl = unsafe { client.GetService() }
                .map_err(|_| CaptureError::BackendInit(AEC_UNAVAILABLE.into()))?;
            // Keep the UTF-16 allocation alive until the COM call returns.
            let render_wide = wide_id(request.playback_device().as_str());
            // SAFETY: render_wide remains allocated until this COM call returns.
            unsafe { control.SetEchoCancellationRenderEndpoint(PCWSTR(render_wide.as_ptr())) }
                .map_err(|error| backend_error("set exact Windows AEC render endpoint", error))?;
        }

        let (processing_reporter, processing_handle) = capture_processing_observations();
        // SAFETY: the audio client is initialized in the current COM apartment.
        let effects: Option<IAudioEffectsManager> = match unsafe { client.GetService() } {
            Ok(effects) => Some(effects),
            Err(_) if request.is_none() => None,
            Err(_) => return Err(CaptureError::BackendInit(AEC_UNAVAILABLE.into())),
        };
        let (route_reporter, route_handle) = if let Some(request) = request {
            let effects = effects
                .as_ref()
                .ok_or_else(|| CaptureError::BackendInit(AEC_UNAVAILABLE.into()))?;
            if !aec_effect_on(effects)? {
                return Err(CaptureError::BackendInit(AEC_UNAVAILABLE.into()));
            }
            let route = NativeAecRoute::new(
                stable_id.source_id(),
                stable_id.clone(),
                request.playback_device().clone(),
                true,
                false,
                true,
            );
            let (reporter, handle) = native_aec_route(route);
            (Some(reporter), Some(handle))
        } else {
            (None, None)
        };
        let monitor = if let Some(effects) = effects.as_ref() {
            let registration = NativeRouteMonitor::register(
                &enumerator,
                effects,
                &stable_id.stable_key,
                request.map(|request| request.playback_device().as_str()),
                route_reporter,
                processing_reporter.clone(),
            );
            match registration {
                Ok(monitor) => Some(monitor),
                Err(_) if request.is_none() => None,
                Err(error) => return Err(error),
            }
        } else {
            None
        };
        if request.is_some() {
            // Close the setup/notification-registration race. Any change in
            // this interval either invalidated the route or is caught here.
            // SAFETY: input remains a live endpoint in this COM apartment.
            let input_active = unsafe { input.GetState() }.ok() == Some(DEVICE_STATE_ACTIVE);
            // SAFETY: render remains a live endpoint in this COM apartment.
            let render = render
                .as_ref()
                .ok_or_else(|| CaptureError::BackendInit(AEC_UNAVAILABLE.into()))?;
            let render_active = unsafe { render.GetState() }.ok() == Some(DEVICE_STATE_ACTIVE);
            let effects = effects
                .as_ref()
                .ok_or_else(|| CaptureError::BackendInit(AEC_UNAVAILABLE.into()))?;
            if !route_handle
                .as_ref()
                .is_some_and(NativeAecRouteHandle::is_valid)
                || !aec_effect_on(effects)?
                || !input_active
                || !render_active
            {
                return Err(CaptureError::BackendInit(
                    "Windows AEC route changed during open".into(),
                ));
            }
            processing_reporter.replace(CaptureProcessingObservations {
                native_aec_supported: Some(true),
                echo_processed: Some(true),
                raw_audio_available: Some(false),
            });
            if monitor
                .as_ref()
                .is_some_and(NativeRouteMonitor::was_invalidated)
            {
                processing_reporter.replace(CaptureProcessingObservations::default());
                return Err(CaptureError::BackendInit(
                    "Windows AEC route changed during open".into(),
                ));
            }
        } else if let (Some(effects), Some(monitor)) = (effects.as_ref(), monitor.as_ref()) {
            // A registered callback makes later changes sticky. Without that
            // subscription, an effect snapshot would go stale during capture.
            if let Ok(state) = aec_effect_state(effects) {
                processing_reporter.replace(CaptureProcessingObservations {
                    native_aec_supported: Some(state.present),
                    echo_processed: Some(state.on),
                    raw_audio_available: None,
                });
                if monitor.was_invalidated() {
                    processing_reporter.replace(CaptureProcessingObservations::default());
                }
            }
        }

        // SAFETY: unnamed auto-reset event uses default security attributes.
        let event_handle = unsafe { CreateEventW(None, false, false, PCWSTR::null()) }
            .map_err(|error| backend_error("create Windows AEC capture event", error))?;
        // SAFETY: event_handle is valid until stream teardown or this error path.
        if let Err(error) = unsafe { client.SetEventHandle(event_handle) } {
            // SAFETY: SetEventHandle failed; this owner closes its unique handle.
            let _ = unsafe { CloseHandle(event_handle) };
            return Err(backend_error("set Windows AEC capture event", error));
        }
        // SAFETY: the initialized capture stream owns this service.
        let capture_client = match unsafe { client.GetService() } {
            Ok(capture_client) => capture_client,
            Err(error) => {
                // SAFETY: stream setup failed; this owner closes its unique handle.
                let _ = unsafe { CloseHandle(event_handle) };
                return Err(backend_error("obtain Windows AEC capture client", error));
            }
        };
        Ok(Self {
            audio_client: client,
            capture_client,
            event_handle,
            monitor,
            stable_id,
            route_handle,
            processing_reporter,
            processing_handle,
        })
    }

    pub(super) fn source_id(&self) -> SourceId {
        self.stable_id.source_id()
    }
    pub(super) fn stable_id(&self) -> StableSourceId {
        self.stable_id.clone()
    }
    pub(super) fn route_handle(&self) -> Option<NativeAecRouteHandle> {
        self.route_handle.clone()
    }
    pub(super) fn processing_handle(&self) -> CaptureProcessingObservationHandle {
        self.processing_handle.clone()
    }
    pub(super) fn route_is_valid(&self) -> bool {
        self.route_handle
            .as_ref()
            .is_some_and(NativeAecRouteHandle::is_valid)
    }
    pub(super) fn start(&self) -> Result<(), CaptureError> {
        // SAFETY: the client is initialized and owns a live event handle.
        unsafe { self.audio_client.Start() }
            .map_err(|error| backend_error("start Windows AEC capture", error))
    }
    pub(super) fn stop(&self) {
        self.invalidate();
        // SAFETY: Stop is allowed after successful initialization and is best-effort.
        let _ = unsafe { self.audio_client.Stop() };
    }
}

impl Drop for WindowsInputStream {
    fn drop(&mut self) {
        self.invalidate();
        // SAFETY: this stream uniquely owns the event handle until drop.
        let _ = unsafe { CloseHandle(self.event_handle) };
    }
}

impl WindowsInputStream {
    fn invalidate(&self) {
        if let Some(monitor) = &self.monitor {
            monitor.invalidate();
        }
        self.processing_reporter
            .replace(CaptureProcessingObservations::default());
    }
}

#[derive(Clone, Copy)]
struct AecEffectState {
    present: bool,
    on: bool,
}

fn aec_effect_on(manager: &IAudioEffectsManager) -> Result<bool, CaptureError> {
    Ok(aec_effect_state(manager)?.on)
}

fn aec_effect_state(manager: &IAudioEffectsManager) -> Result<AecEffectState, CaptureError> {
    let mut effects_ptr: *mut AUDIO_EFFECT = std::ptr::null_mut();
    let mut count = 0u32;
    // SAFETY: both output pointers remain valid for the COM call.
    unsafe { manager.GetAudioEffects(&mut effects_ptr, &mut count) }
        .map_err(|error| backend_error("query Windows capture effects", error))?;
    if count > 0 && effects_ptr.is_null() {
        return Err(CaptureError::BackendInit(
            "Windows returned a null audio-effect list".into(),
        ));
    }
    let state = if count == 0 {
        AecEffectState {
            present: false,
            on: false,
        }
    } else {
        // SAFETY: GetAudioEffects returned count elements at non-null effects_ptr.
        let effects = unsafe { std::slice::from_raw_parts(effects_ptr, count as usize) };
        AecEffectState {
            present: effects
                .iter()
                .any(|effect| effect.id == AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION),
            on: effects.iter().any(|effect| {
                effect.id == AUDIO_EFFECT_TYPE_ACOUSTIC_ECHO_CANCELLATION
                    && effect.state == AUDIO_EFFECT_STATE_ON
            }),
        }
    };
    if !effects_ptr.is_null() {
        // SAFETY: GetAudioEffects allocated this array with CoTaskMemAlloc.
        unsafe { CoTaskMemFree(Some(effects_ptr.cast())) };
    }
    Ok(state)
}

fn device_id(device: &IMMDevice) -> Result<String, CaptureError> {
    // SAFETY: device is a live endpoint in the current COM apartment.
    let id = unsafe { device.GetId() }
        .map_err(|error| backend_error("query Windows audio endpoint identity", error))?;
    // SAFETY: the endpoint ID is a live null-terminated Windows string.
    let value = unsafe { id.to_string() }.map_err(|error| {
        CaptureError::BackendInit(format!("decode Windows audio endpoint identity: {error}"))
    });
    // SAFETY: IMMDevice::GetId allocated this pointer with CoTaskMemAlloc.
    unsafe { CoTaskMemFree(Some(id.0.cast())) };
    value
}

fn ensure_active_flow(
    enumerator: &IMMDeviceEnumerator,
    flow: EDataFlow,
    exact_id: &str,
) -> Result<(), CaptureError> {
    // SAFETY: enumerator is live and DEVICE_STATE_ACTIVE is a valid mask.
    let devices = unsafe { enumerator.EnumAudioEndpoints(flow, DEVICE_STATE_ACTIVE) }
        .map_err(|error| backend_error("enumerate active Windows audio endpoints", error))?;
    // SAFETY: devices is a live COM collection in the current apartment.
    let count = unsafe { devices.GetCount() }
        .map_err(|error| backend_error("count active Windows audio endpoints", error))?;
    for index in 0..count {
        // SAFETY: index is below the collection's returned item count.
        let device = unsafe { devices.Item(index) }
            .map_err(|error| backend_error("inspect active Windows audio endpoint", error))?;
        if device_id(&device)? == exact_id {
            return Ok(());
        }
    }
    Err(CaptureError::BackendInit(
        "Windows AEC requires active input and render endpoints of the declared kinds".into(),
    ))
}

fn wide_id(id: &str) -> Vec<u16> {
    id.encode_utf16().chain(std::iter::once(0)).collect()
}

fn backend_error(operation: &'static str, error: windows::core::Error) -> CaptureError {
    CaptureError::BackendStatus {
        operation,
        status_code: error.code().0,
    }
}

#[implement(IAudioEffectsChangedNotificationClient)]
struct EffectsChanged {
    route_reporter: Option<NativeAecRouteReporter>,
    processing_reporter: CaptureProcessingReporter,
    invalidated: Arc<AtomicBool>,
}

#[allow(non_snake_case)]
impl IAudioEffectsChangedNotificationClient_Impl for EffectsChanged_Impl {
    fn OnAudioEffectsChanged(&self) -> windows::core::Result<()> {
        self.invalidated.store(true, Ordering::Release);
        self.processing_reporter
            .replace(CaptureProcessingObservations::default());
        if let Some(route_reporter) = &self.route_reporter {
            route_reporter.invalidate();
        }
        Ok(())
    }
}

#[implement(IMMNotificationClient)]
struct EndpointChanged {
    route_reporter: Option<NativeAecRouteReporter>,
    processing_reporter: CaptureProcessingReporter,
    invalidated: Arc<AtomicBool>,
    microphone_id: Vec<u16>,
    render_id: Option<Vec<u16>>,
}

impl EndpointChanged {
    fn invalidate(&self) {
        self.invalidated.store(true, Ordering::Release);
        self.processing_reporter
            .replace(CaptureProcessingObservations::default());
        if let Some(route_reporter) = &self.route_reporter {
            route_reporter.invalidate();
        }
    }

    fn invalidate_if_selected(&self, device_id: &PCWSTR) {
        if matches_endpoint(device_id, &self.microphone_id)
            || self
                .render_id
                .as_ref()
                .is_some_and(|id| matches_endpoint(device_id, id))
        {
            self.invalidate();
        }
    }

    fn invalidate_on_default_change(&self, flow: EDataFlow) {
        // A default render switch can redirect the selected application's
        // audible output while the chosen reference remains active. A default
        // capture switch can change the meaning of input selection. Without
        // routing evidence, withdraw processing facts and the native route.
        if flow == eRender || flow == eCapture {
            self.invalidate();
        }
    }
}

#[allow(non_snake_case)]
impl IMMNotificationClient_Impl for EndpointChanged_Impl {
    fn OnDeviceStateChanged(
        &self,
        id: &PCWSTR,
        _: windows::Win32::Media::Audio::DEVICE_STATE,
    ) -> windows::core::Result<()> {
        self.invalidate_if_selected(id);
        Ok(())
    }
    fn OnDeviceAdded(&self, _: &PCWSTR) -> windows::core::Result<()> {
        Ok(())
    }
    fn OnDeviceRemoved(&self, id: &PCWSTR) -> windows::core::Result<()> {
        self.invalidate_if_selected(id);
        Ok(())
    }
    fn OnDefaultDeviceChanged(
        &self,
        flow: EDataFlow,
        _: ERole,
        _: &PCWSTR,
    ) -> windows::core::Result<()> {
        self.invalidate_on_default_change(flow);
        Ok(())
    }
    fn OnPropertyValueChanged(&self, id: &PCWSTR, _: &PROPERTYKEY) -> windows::core::Result<()> {
        self.invalidate_if_selected(id);
        Ok(())
    }
}

fn matches_endpoint(actual: &PCWSTR, expected: &[u16]) -> bool {
    if actual.is_null() {
        return false;
    }
    for (index, expected_unit) in expected.iter().enumerate() {
        // SAFETY: Windows supplies a live, null-terminated endpoint ID for
        // this callback. Stop at its terminator and never scan past the
        // preallocated expected ID length.
        let actual_unit = unsafe { *actual.0.add(index) };
        if actual_unit != *expected_unit {
            return false;
        }
        if actual_unit == 0 {
            return true;
        }
    }
    false
}

struct NativeRouteMonitor {
    enumerator: IMMDeviceEnumerator,
    effects: IAudioEffectsManager,
    effects_changed: IAudioEffectsChangedNotificationClient,
    endpoint_changed: IMMNotificationClient,
    route_reporter: Option<NativeAecRouteReporter>,
    processing_reporter: CaptureProcessingReporter,
    invalidated: Arc<AtomicBool>,
}

impl NativeRouteMonitor {
    fn register(
        enumerator: &IMMDeviceEnumerator,
        effects: &IAudioEffectsManager,
        microphone_id: &str,
        render_id: Option<&str>,
        route_reporter: Option<NativeAecRouteReporter>,
        processing_reporter: CaptureProcessingReporter,
    ) -> Result<Self, CaptureError> {
        let invalidated = Arc::new(AtomicBool::new(false));
        let effects_changed: IAudioEffectsChangedNotificationClient = EffectsChanged {
            route_reporter: route_reporter.clone(),
            processing_reporter: processing_reporter.clone(),
            invalidated: invalidated.clone(),
        }
        .into();
        let endpoint_changed: IMMNotificationClient = EndpointChanged {
            route_reporter: route_reporter.clone(),
            processing_reporter: processing_reporter.clone(),
            invalidated: invalidated.clone(),
            microphone_id: wide_id(microphone_id),
            render_id: render_id.map(wide_id),
        }
        .into();
        // SAFETY: callback is a live COM object retained by this monitor.
        unsafe { effects.RegisterAudioEffectsChangedNotificationCallback(&effects_changed) }
            .map_err(|error| backend_error("subscribe to Windows AEC effect changes", error))?;
        // SAFETY: callback is a live COM object retained by this monitor.
        if let Err(error) =
            unsafe { enumerator.RegisterEndpointNotificationCallback(&endpoint_changed) }
        {
            // SAFETY: effect callback was registered above and remains live.
            let _ = unsafe {
                effects.UnregisterAudioEffectsChangedNotificationCallback(&effects_changed)
            };
            return Err(backend_error(
                "subscribe to Windows audio endpoint changes",
                error,
            ));
        }
        Ok(Self {
            enumerator: enumerator.clone(),
            effects: effects.clone(),
            effects_changed,
            endpoint_changed,
            route_reporter,
            processing_reporter,
            invalidated,
        })
    }

    fn invalidate(&self) {
        self.invalidated.store(true, Ordering::Release);
        self.processing_reporter
            .replace(CaptureProcessingObservations::default());
        if let Some(route_reporter) = &self.route_reporter {
            route_reporter.invalidate();
        }
    }

    fn was_invalidated(&self) -> bool {
        self.invalidated.load(Ordering::Acquire)
    }
}

impl Drop for NativeRouteMonitor {
    fn drop(&mut self) {
        self.invalidate();
        // SAFETY: the callback and enumerator are live; this is a control-thread drop.
        let _ = unsafe {
            self.enumerator
                .UnregisterEndpointNotificationCallback(&self.endpoint_changed)
        };
        // SAFETY: the callback and effects manager are live until this drop ends.
        let _ = unsafe {
            self.effects
                .UnregisterAudioEffectsChangedNotificationCallback(&self.effects_changed)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_endpoint_ids_when_compared_then_only_exact_utf16_matches() {
        let id = wide_id("{render-device}");
        let shorter = wide_id("{render}");
        let other = wide_id("{other-device}");
        assert!(matches_endpoint(&PCWSTR(id.as_ptr()), &id));
        assert!(!matches_endpoint(&PCWSTR(shorter.as_ptr()), &id));
        assert!(!matches_endpoint(&PCWSTR(other.as_ptr()), &id));
        assert!(!matches_endpoint(&PCWSTR::null(), &id));
    }

    #[test]
    fn given_native_route_when_unrelated_endpoint_changes_then_route_stays_valid() {
        let microphone = StableSourceId::new(Platform::Windows, SourceKind::InputDevice, "mic");
        let (reporter, handle) = native_aec_route(NativeAecRoute::new(
            microphone.source_id(),
            microphone,
            crate::DeviceId::new("render"),
            true,
            false,
            true,
        ));
        let (processing_reporter, _) = capture_processing_observations();
        let notification = EndpointChanged {
            route_reporter: Some(reporter),
            processing_reporter,
            invalidated: Arc::new(AtomicBool::new(false)),
            microphone_id: wide_id("mic"),
            render_id: Some(wide_id("render")),
        };
        let unrelated = wide_id("unrelated");
        notification.invalidate_if_selected(&PCWSTR(unrelated.as_ptr()));
        assert!(handle.is_valid());
        let render = wide_id("render");
        notification.invalidate_if_selected(&PCWSTR(render.as_ptr()));
        assert!(!handle.is_valid());
    }

    #[test]
    fn given_ordinary_input_when_opened_endpoint_changes_then_effect_facts_invalidate() {
        let (processing_reporter, processing_handle) = capture_processing_observations();
        processing_reporter.replace(CaptureProcessingObservations {
            native_aec_supported: Some(true),
            echo_processed: Some(false),
            raw_audio_available: None,
        });
        let notification = EndpointChanged {
            route_reporter: None,
            processing_reporter,
            invalidated: Arc::new(AtomicBool::new(false)),
            microphone_id: wide_id("opened-microphone"),
            render_id: None,
        };
        let unrelated = wide_id("different-microphone");
        notification.invalidate_if_selected(&PCWSTR(unrelated.as_ptr()));
        assert_eq!(processing_handle.observations().echo_processed, Some(false));
        let selected = wide_id("opened-microphone");
        notification.invalidate_if_selected(&PCWSTR(selected.as_ptr()));
        assert_eq!(
            processing_handle.observations(),
            CaptureProcessingObservations::default()
        );
        assert!(notification.invalidated.load(Ordering::Acquire));
        assert_eq!(processing_handle.admission_snapshot().1, 1);
    }

    #[test]
    fn given_native_route_when_default_render_changes_then_route_and_processing_invalidate() {
        let microphone = StableSourceId::new(Platform::Windows, SourceKind::InputDevice, "mic");
        let (route_reporter, route_handle) = native_aec_route(NativeAecRoute::new(
            microphone.source_id(),
            microphone,
            crate::DeviceId::new("render"),
            true,
            false,
            true,
        ));
        let (processing_reporter, processing_handle) = capture_processing_observations();
        processing_reporter.replace(CaptureProcessingObservations {
            native_aec_supported: Some(true),
            echo_processed: Some(true),
            raw_audio_available: Some(false),
        });
        let notification = EndpointChanged {
            route_reporter: Some(route_reporter),
            processing_reporter,
            invalidated: Arc::new(AtomicBool::new(false)),
            microphone_id: wide_id("mic"),
            render_id: Some(wide_id("render")),
        };
        notification.invalidate_on_default_change(eRender);
        assert!(!route_handle.is_valid());
        assert_eq!(
            processing_handle.observations(),
            CaptureProcessingObservations::default()
        );
    }
}

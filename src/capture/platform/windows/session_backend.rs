use crate::capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationHandle, CaptureObservations, CaptureProcessingObservationHandle,
    NativeAecRequest, NativeAecRouteHandle, PreparedCaptureBackend,
};

use crate::capture::platform::windows::DesktopCaptureSource;
use crate::frame::AudioFrameDuration;

/// Windows adapter from the platform-neutral Session capture API to the
/// existing synchronously-opened WASAPI RAII owner.
#[derive(Debug)]
pub struct DesktopCaptureBackend {
    audio_frame_duration: AudioFrameDuration,
}

impl DesktopCaptureBackend {
    pub(crate) const fn new(audio_frame_duration: AudioFrameDuration) -> Self {
        Self {
            audio_frame_duration,
        }
    }
}

impl Default for DesktopCaptureBackend {
    fn default() -> Self {
        Self::new(AudioFrameDuration::default())
    }
}

struct PreparedDesktopCapture {
    mode: CaptureMode,
    native_aec_request: Option<NativeAecRequest>,
    audio_frame_duration: AudioFrameDuration,
}

struct ActiveDesktopCapture {
    source: DesktopCaptureSource,
}

impl CallbackCaptureBackend for DesktopCaptureBackend {
    fn prepare(&self, mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        Ok(Box::new(PreparedDesktopCapture {
            mode,
            native_aec_request: None,
            audio_frame_duration: self.audio_frame_duration,
        }))
    }

    fn prepare_native_aec(
        &self,
        mode: CaptureMode,
        request: &NativeAecRequest,
    ) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        if !matches!(mode, CaptureMode::InputDevice(_)) {
            return Err(CaptureError::ModeUnsupported(mode));
        }
        if request.playback_device().as_str().is_empty() {
            return Err(CaptureError::BackendInit(
                "Windows native AEC requires an exact playback endpoint".to_owned(),
            ));
        }
        Ok(Box::new(PreparedDesktopCapture {
            mode,
            native_aec_request: Some(request.clone()),
            audio_frame_duration: self.audio_frame_duration,
        }))
    }
}

impl PreparedCaptureBackend for PreparedDesktopCapture {
    fn open(
        self: Box<Self>,
        delivery: CaptureDelivery,
    ) -> Result<Box<dyn ActiveCaptureBackend>, CaptureError> {
        let CaptureDelivery {
            frame_sender,
            runtime_event_sender,
        } = delivery;
        let source = match (self.mode, self.native_aec_request) {
            (CaptureMode::InputDevice(selector), Some(request)) => {
                DesktopCaptureSource::capture_native_input_with_runtime_event_sender(
                    selector,
                    request,
                    self.audio_frame_duration,
                    frame_sender.into_callback(),
                    runtime_event_sender,
                )?
            }
            (mode, None) => DesktopCaptureSource::capture_mode_with_runtime_event_sender(
                mode,
                self.audio_frame_duration,
                frame_sender.into_callback(),
                runtime_event_sender,
            )?,
            (mode, Some(_)) => return Err(CaptureError::ModeUnsupported(mode)),
        };
        Ok(Box::new(ActiveDesktopCapture { source }))
    }
}

impl ActiveCaptureBackend for ActiveDesktopCapture {
    fn source_id(&self) -> crate::frame::SourceId {
        self.source.source_id()
    }

    fn observation_handle(&self) -> CaptureObservationHandle {
        self.source.observation_handle()
    }

    fn native_aec_route_handle(&self) -> Option<NativeAecRouteHandle> {
        self.source.native_aec_route_handle()
    }

    fn processing_observation_handle(&self) -> Option<CaptureProcessingObservationHandle> {
        self.source.processing_observation_handle()
    }

    fn observations(&self) -> CaptureObservations {
        self.source.observations()
    }

    fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
        self.source.stop_and_join()
    }
}

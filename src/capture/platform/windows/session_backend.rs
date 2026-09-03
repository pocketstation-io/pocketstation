use crate::capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationHandle, CaptureObservations, PreparedCaptureBackend,
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
    audio_frame_duration: AudioFrameDuration,
}

struct ActiveDesktopCapture {
    source: DesktopCaptureSource,
}

impl CallbackCaptureBackend for DesktopCaptureBackend {
    fn prepare(&self, mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        Ok(Box::new(PreparedDesktopCapture {
            mode,
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
        let source = DesktopCaptureSource::capture_mode_with_runtime_event_sender(
            self.mode,
            self.audio_frame_duration,
            frame_sender.into_callback(),
            runtime_event_sender,
        )?;
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

    fn observations(&self) -> CaptureObservations {
        self.source.observations()
    }

    fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
        self.source.stop_and_join()
    }
}

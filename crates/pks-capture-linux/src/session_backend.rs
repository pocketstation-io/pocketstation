use pks_capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservations, PreparedCaptureBackend,
};

use crate::DesktopCaptureSource;

/// Linux adapter from the platform-neutral Session capture contract to the
/// existing PipeWire RAII owner.
#[derive(Debug, Default)]
pub struct DesktopCaptureBackend;

struct PreparedDesktopCapture {
    mode: CaptureMode,
}

struct ActiveDesktopCapture {
    source: DesktopCaptureSource,
}

impl CallbackCaptureBackend for DesktopCaptureBackend {
    fn prepare(&self, mode: CaptureMode) -> Result<Box<dyn PreparedCaptureBackend>, CaptureError> {
        Ok(Box::new(PreparedDesktopCapture { mode }))
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
            frame_sender.into_callback(),
            Some(runtime_event_sender),
        )?;
        Ok(Box::new(ActiveDesktopCapture { source }))
    }
}

impl ActiveCaptureBackend for ActiveDesktopCapture {
    fn observations(&self) -> CaptureObservations {
        self.source.observations()
    }

    fn stop_and_join(self: Box<Self>) -> Result<CaptureObservations, CaptureError> {
        self.source.stop_and_join()
    }
}

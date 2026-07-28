use pks_capture::{
    ActiveCaptureBackend, CallbackCaptureBackend, CaptureDelivery, CaptureError, CaptureMode,
    CaptureObservationHandle, CaptureObservations, PreparedCaptureBackend,
};

use crate::DesktopCaptureSource;

/// macOS adapter from the platform-neutral Session capture contract to the
/// existing CoreAudio/input RAII owner.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_macos_backend_when_type_checked_then_callback_contract_is_implemented() {
        fn require_callback_backend<T: CallbackCaptureBackend>() {}

        require_callback_backend::<DesktopCaptureBackend>();
    }
}

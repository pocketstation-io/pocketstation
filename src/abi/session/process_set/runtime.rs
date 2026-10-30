//! Control-thread assembly of the existing exact-set Source factory.

use super::*;
use crate::abi::session::process_set::{process_error, records::*};
use crate::session::extensions::process_set::ProcessSetFactory;
use crate::{ApplicationProcessSet, AudioFrameDuration, SampleFormat, SampleSpec};

impl SessionRuntime {
    pub(super) fn create_process_set_session(
        &mut self,
        selection: ApplicationProcessSet,
    ) -> Result<PksSessionHandle, AbiError> {
        if self.session_created || self.sessions.active_count() != 0 || self.host.is_some() {
            return Err(AbiError::NoCapacity);
        }
        check_platform(&selection)?;
        let mut options = self.options;
        options.sample_spec = SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved);
        let duration = AudioFrameDuration::Ms20;
        let factory =
            ProcessSetFactory::new(options.sample_spec, duration).map_err(process_error)?;
        let mut builder = SessionEngineHostBuilder::native_with_audio_frame_duration(
            options, duration,
        )
        .map_err(|error| match error {
            crate::session::SessionEngineHostBuildError::UnsupportedPlatform => {
                AbiError::ProcessSetFailure {
                    detail_code: PROCESS_SET_UNSUPPORTED,
                }
            }
            _ => AbiError::ProcessSetFailure {
                detail_code: PROCESS_SET_BACKEND_FAILED,
            },
        })?;
        builder
            .engine_builder()
            .register_source_factory(Arc::clone(&factory) as Arc<dyn crate::SourceFactory>)
            .map_err(|_| AbiError::InvalidProcessSet)?;
        let declaration = Session::new();
        let output = factory
            .declare(&declaration, selection)
            .map_err(process_error)?;
        let audio = declaration
            .polled_audio()
            .map_err(|_| AbiError::InvalidProcessSet)?;
        output
            .send(audio)
            .map_err(|_| AbiError::InvalidProcessSet)?;
        builder
            .register_polled_audio_endpoint(options.polled_audio_endpoint)
            .map_err(|_| AbiError::InvalidProcessSet)?;
        let host = builder.build().map_err(|_| AbiError::BackendFailure)?;
        let handle = self.sessions.insert(SessionObject::Draft(declaration))?;
        self.host = Some(host);
        self.process_set_factory = Some(factory);
        self.session_created = true;
        Ok(handle)
    }

    pub(super) fn classify_start_failure(
        &mut self,
        error: &crate::session::SessionEngineStartError,
    ) -> AbiError {
        let Some(failure) = error.start_failure() else {
            return AbiError::BackendFailure;
        };
        let code = crate::session_start_failure_code(failure.error()).as_str();
        self.startup_cleanup_unconfirmed = !failure.rollback_failures().is_empty()
            || failure.error().rollback_failures_total() != 0
            || code == "capture.cleanup_unconfirmed";
        if self.factory_cleanup_unconfirmed() || self.startup_cleanup_unconfirmed {
            return AbiError::ProcessSetFailure {
                detail_code: PROCESS_SET_CLEANUP_UNCONFIRMED,
            };
        }
        if self.process_set_factory.is_none() {
            return AbiError::BackendFailure;
        }
        let detail_code = match code {
            "capture.source_unavailable" => PROCESS_SET_SOURCE_UNAVAILABLE,
            "capture.permission_denied" => PROCESS_SET_PERMISSION_DENIED,
            "capture.unsupported" | "session.unsupported_platform" => PROCESS_SET_UNSUPPORTED,
            _ => PROCESS_SET_BACKEND_FAILED,
        };
        AbiError::ProcessSetFailure { detail_code }
    }
}

fn check_platform(selection: &ApplicationProcessSet) -> Result<(), AbiError> {
    #[cfg(all(target_os = "macos", feature = "coreaudio-capture"))]
    if selection.platform() == crate::Platform::Macos
        && crate::capture::platform::macos::macos_tap::exact_process_set_available()
    {
        return Ok(());
    }
    #[cfg(not(all(target_os = "macos", feature = "coreaudio-capture")))]
    let _ = selection;
    Err(AbiError::ProcessSetFailure {
        detail_code: PROCESS_SET_UNSUPPORTED,
    })
}

impl RuntimeState {
    pub fn create_process_set_session(
        &self,
        engine_handle: PksSessionHandle,
        selection: ApplicationProcessSet,
    ) -> Result<PksSessionHandle, AbiError> {
        self.create_guarded_session(engine_handle, |runtime| {
            runtime.create_process_set_session(selection)
        })
    }

    pub(super) fn create_guarded_session(
        &self,
        engine_handle: PksSessionHandle,
        operation: impl FnOnce(&mut SessionRuntime) -> Result<PksSessionHandle, AbiError>,
    ) -> Result<PksSessionHandle, AbiError> {
        let engine = self.engine(engine_handle)?;
        // Preserve the existing one-Session-ever capacity result after destroy.
        if engine.session_handle.get().is_some() {
            return Err(AbiError::NoCapacity);
        }
        engine.check_open()?;
        let mut runtime = engine
            .runtime
            .lock()
            .map_err(|_| AbiError::BackendFailure)?;
        engine.check_open()?;
        let handle = operation(&mut runtime)?;
        engine
            .session_handle
            .set(handle)
            .map_err(|_| AbiError::NoCapacity)?;
        engine.session_live.store(true, Ordering::Release);
        engine.set_state(PksSessionLifecycleState::Draft);
        Ok(handle)
    }
}

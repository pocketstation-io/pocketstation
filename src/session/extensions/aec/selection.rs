use crate::capture::NativeAecRequest;
use crate::{DeviceId, Session, SessionError, StemHandle};

/// Explicit permission to use one playback device's output mix as a native reference.
/// No additional application or output recording is created by this declaration.
#[derive(Clone, Debug)]
pub struct NativePlaybackReference {
    playback_device: DeviceId,
}

impl NativePlaybackReference {
    /// Selects an exact playback device; an empty identifier fails at declaration.
    pub fn output(playback_device: DeviceId) -> Self {
        Self { playback_device }
    }

    pub fn playback_device(&self) -> &DeviceId {
        &self.playback_device
    }
}

impl Session {
    /// Requests native AEC on one captured microphone using an authorized output mix.
    ///
    /// This records intent, not device support or active processing. Session start
    /// must establish the requested device reference and processing state, or fail.
    /// No portable engine is required or selected as an automatic fallback.
    /// The delivered microphone may already be processed; do not label it raw.
    pub fn native_aec(
        &self,
        microphone: &StemHandle,
        reference: NativePlaybackReference,
    ) -> Result<(), SessionError> {
        self.declaration
            .request_native_aec(microphone, NativeAecRequest::new(reference.playback_device))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Source;

    #[test]
    fn given_native_request_when_frozen_then_exact_device_intent_survives_without_an_operator() {
        let session = Session::new();
        let microphone = session.capture(Source::microphone_default()).unwrap();
        microphone.send(session.polled_audio().unwrap()).unwrap();
        session
            .native_aec(
                &microphone,
                NativePlaybackReference::output(DeviceId::new("output-exact")),
            )
            .unwrap();
        assert_eq!(
            session
                .echo_input_processing(&microphone)
                .unwrap()
                .echo_processed,
            None
        );
        let spec = session.declaration.freeze().unwrap();
        assert_eq!(
            spec.stems()[0]
                .native_aec_request()
                .unwrap()
                .playback_device()
                .as_str(),
            "output-exact"
        );
        assert!(spec.operators().is_empty());
    }

    #[test]
    fn given_invalid_or_duplicate_native_requests_when_declared_then_original_intent_is_preserved()
    {
        let session = Session::new();
        let microphone = session.capture(Source::microphone_default()).unwrap();
        let playback = session.capture(Source::SystemAudio).unwrap();
        let foreign = Session::new()
            .capture(Source::microphone_default())
            .unwrap();
        let reference = || NativePlaybackReference::output(DeviceId::new("output-exact"));
        assert!(session.native_aec(&foreign, reference()).is_err());
        assert!(session.native_aec(&playback, reference()).is_err());
        assert!(session
            .native_aec(
                &microphone,
                NativePlaybackReference::output(DeviceId::new(" "))
            )
            .is_err());
        session.native_aec(&microphone, reference()).unwrap();
        assert!(session.native_aec(&microphone, reference()).is_err());
        assert!(session
            .native_aec(
                &microphone,
                NativePlaybackReference::output(DeviceId::new("other-output"))
            )
            .is_err());
        microphone.send(session.polled_audio().unwrap()).unwrap();
        playback.send(session.polled_audio().unwrap()).unwrap();
        let spec = session.declaration.freeze().unwrap();
        assert_eq!(
            spec.stems()[0]
                .native_aec_request()
                .unwrap()
                .playback_device()
                .as_str(),
            "output-exact"
        );
        assert!(spec.stems()[1].native_aec_request().is_none());
    }

    #[cfg(feature = "aec")]
    #[test]
    fn given_native_and_portable_requests_when_order_changes_then_second_request_is_rejected() {
        for native_first in [true, false] {
            let session = Session::new();
            let microphone = session.capture(Source::microphone_default()).unwrap();
            let playback = session.capture(Source::SystemAudio).unwrap();
            let native = || NativePlaybackReference::output(DeviceId::new("output-exact"));
            if native_first {
                session.native_aec(&microphone, native()).unwrap();
                assert!(session
                    .echo_cancel(
                        &microphone,
                        super::super::PlaybackReference::output_mix(&playback)
                    )
                    .is_err());
                microphone.send(session.polled_audio().unwrap()).unwrap();
                playback.send(session.polled_audio().unwrap()).unwrap();
                let spec = session.declaration.freeze().unwrap();
                assert!(spec.stems()[0].native_aec_request().is_some());
                assert!(spec.operators().is_empty());
            } else {
                let processed = session
                    .echo_cancel(
                        &microphone,
                        super::super::PlaybackReference::output_mix(&playback),
                    )
                    .unwrap();
                assert!(session.native_aec(&microphone, native()).is_err());
                processed
                    .audio()
                    .send(session.polled_audio().unwrap())
                    .unwrap();
                let spec = session.declaration.freeze().unwrap();
                assert!(spec.stems()[0].native_aec_request().is_none());
                assert_eq!(spec.operators().len(), 1);
            }
        }
    }

    #[cfg(feature = "aec")]
    #[test]
    fn given_native_request_when_generic_input_connected_late_then_freeze_rejects_portable_processing(
    ) {
        use crate::{Operator, OperatorConfiguration, OperatorId};

        let session = Session::new();
        let microphone = session.capture(Source::microphone_default()).unwrap();
        let playback = session.capture(Source::SystemAudio).unwrap();
        session
            .native_aec(
                &microphone,
                NativePlaybackReference::output(DeviceId::new("output-exact")),
            )
            .unwrap();
        let intermediate = session
            .operator(Operator::new(
                OperatorId::new("example.operator.intermediate.v1"),
                OperatorConfiguration::new(),
            ))
            .unwrap();
        let output = intermediate.output("audio").unwrap();
        let processed = session
            .declaration
            .connected_audio_operator(
                Operator::new(
                    OperatorId::new("test.aec.portable"),
                    OperatorConfiguration::new(),
                )
                .with_echo_processing(),
                &[
                    (session.id(), output.signal_origin().1, "microphone"),
                    (session.id(), playback.signal_origin().1, "reference"),
                ],
                "microphone",
            )
            .unwrap();
        processed.send(session.polled_audio().unwrap()).unwrap();
        microphone
            .connect(intermediate.input("audio").unwrap())
            .unwrap();
        assert!(session.declaration.freeze().is_err());
    }
}

use pocketstation::{AudioInputConfig, EchoAudioInput, SampleFormat, SampleSpec, Session, Source};

fn input_config() -> AudioInputConfig {
    AudioInputConfig::new(
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        8,
        960,
    )
    .unwrap()
}

#[test]
fn given_unobserved_or_declared_input_when_inspected_then_evidence_is_explicit() {
    let session = Session::new();
    let microphone = session.capture(Source::microphone_default()).unwrap();
    let unknown = session.echo_input_processing(&microphone).unwrap();
    assert_eq!(unknown.echo_processed, None);
    assert_eq!(unknown.evidence, "unknown");
    let pcm = session.audio_input(input_config()).unwrap();
    for value in [false, true] {
        let input = EchoAudioInput::from(pcm.output()).with_echo_processing(value);
        let state = session.echo_input_processing(input).unwrap();
        assert_eq!(state.echo_processed, Some(value));
        assert_eq!(state.evidence, "caller");
    }
    assert_eq!(
        session
            .echo_input_processing(pcm.output())
            .unwrap()
            .echo_processed,
        None
    );
    let foreign = Session::new();
    assert!(foreign.echo_input_processing(pcm.output()).is_err());
}

#[cfg(feature = "aec")]
#[test]
fn given_processed_ancestry_when_cloned_transformed_or_reentered_then_it_cannot_be_erased() {
    use pocketstation::{Operator, OperatorConfiguration, OperatorId, PlaybackReference};
    let session = Session::new();
    let mic = session.audio_input(input_config()).unwrap();
    let reference = session.audio_input(input_config()).unwrap();
    let processed = session
        .echo_cancel(
            mic.output(),
            PlaybackReference::rendered_audio(reference.output()),
        )
        .unwrap();
    let cloned = processed.audio().clone();
    let declared_raw = EchoAudioInput::from(&cloned).with_echo_processing(false);
    let state = session.echo_input_processing(declared_raw.clone()).unwrap();
    assert_eq!(state.echo_processed, Some(true));
    assert_eq!(state.evidence, "session");
    assert!(session
        .echo_cancel(
            declared_raw,
            PlaybackReference::rendered_audio(reference.output())
        )
        .is_err());
    let derived = cloned
        .through(Operator::new(
            OperatorId::new("example.operator.passthrough.v1"),
            OperatorConfiguration::new(),
        ))
        .unwrap();
    let reentered = derived.reenter_audio().unwrap();
    for input in [
        EchoAudioInput::from(&cloned),
        EchoAudioInput::from(&derived),
        EchoAudioInput::from(&reentered),
    ] {
        assert_eq!(
            session
                .echo_input_processing(input.clone())
                .unwrap()
                .echo_processed,
            Some(true)
        );
        let error = match session
            .echo_cancel(input, PlaybackReference::rendered_audio(reference.output()))
        {
            Err(error) => error,
            Ok(_) => panic!("known cancellation ancestry was processed again"),
        };
        assert!(error
            .to_string()
            .contains("already passes through cancellation"));
    }
}

#[cfg(feature = "aec")]
#[test]
fn given_rejected_second_stage_when_session_runs_then_raw_and_first_stage_still_deliver() {
    use pocketstation::PlaybackReference;
    use std::{
        thread,
        time::{Duration, Instant},
    };
    let session = Session::new();
    let mut mic = session.audio_input(input_config()).unwrap();
    let mut reference = session.audio_input(input_config()).unwrap();
    let processed = session
        .echo_cancel(
            mic.output(),
            PlaybackReference::rendered_audio(reference.output()),
        )
        .unwrap();
    assert!(session
        .echo_cancel(
            processed.audio(),
            PlaybackReference::rendered_audio(reference.output())
        )
        .is_err());
    let declared_processed = EchoAudioInput::from(mic.output()).with_echo_processing(true);
    assert!(session
        .echo_cancel(
            declared_processed,
            PlaybackReference::rendered_audio(reference.output())
        )
        .is_err());
    let endpoint = session.polled_audio().unwrap();
    mic.output().send(endpoint).unwrap();
    processed.audio().send(endpoint).unwrap();
    let mut running = session.start().unwrap();
    for index in 0..20 {
        let pcm = vec![0.001 * (index + 1) as f32; 960];
        reference.try_write(&vec![0.0; 960]).unwrap();
        mic.try_write(&pcm).unwrap();
        let deadline = Instant::now() + Duration::from_secs(2);
        let (mut raw, mut aec) = (false, false);
        while !(raw && aec) {
            if let Ok(batch) = running.try_poll_audio() {
                for offset in 0..batch.len() {
                    let frame = batch.frame(offset).unwrap();
                    if frame.lineage().stem_id() == processed.audio().id() {
                        assert!(frame.processing().is_some());
                        aec = true;
                    } else {
                        assert_eq!(frame.samples(), pcm.as_slice());
                        raw = true;
                    }
                }
            }
            assert!(
                Instant::now() < deadline,
                "first-stage or raw delivery stalled"
            );
            if !(raw && aec) {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
    mic.close();
    reference.close();
    assert!(running.stop().is_success());
    assert_eq!(
        processed.observations().processed_microphone_frames_total,
        20
    );
}

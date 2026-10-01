use pocketstation::aec_available;

#[test]
fn given_build_features_when_queried_then_availability_matches() {
    assert_eq!(aec_available(), cfg!(feature = "aec"));
}

#[cfg(not(feature = "aec"))]
#[test]
fn given_unavailable_aec_when_requested_then_independent_audio_still_arrives() {
    use pocketstation::{AudioInputConfig, PlaybackReference, SampleFormat, SampleSpec, Session};
    let session = Session::new();
    let spec = SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved);
    let mut application = session
        .audio_input(AudioInputConfig::new(spec, 8, 960).unwrap())
        .unwrap();
    let output = application.output();
    let error = match session.echo_cancel(output, PlaybackReference::selected_application(output)) {
        Ok(_) => panic!("a lean build must not claim to cancel echo"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("AEC is unavailable"));
    output.send(session.polled_audio().unwrap()).unwrap();
    let mut running = session.start().unwrap();
    let samples = vec![0.125; 960];
    application.try_write(&samples).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    let mut received = false;
    while std::time::Instant::now() < deadline {
        if let Ok(batch) = running.try_poll_audio() {
            if let Some(frame) = batch.frame(0) {
                assert_eq!(frame.samples(), samples.as_slice());
                received = true;
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert!(running.stop().is_success());
    assert!(
        received,
        "independent application audio must survive unavailable AEC"
    );
}

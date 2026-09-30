use pocketstation::{
    AudioFrameDuration, AudioInputConfig, AudioInputWriteErrorKind, PolledAudioPollError,
    SampleFormat, SampleSpec, Session,
};

#[test]
fn given_preaccepted_pcm_when_session_stops_immediately_then_all_frames_drain_with_identity() {
    for producer_closed in [false, true] {
        for frame_count in [1, 8] {
            let spec = SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved);
            let session = Session::builder()
                .sample_spec(spec)
                .audio_frame_duration(AudioFrameDuration::Ms10)
                .build();
            let mut input = session
                .audio_input(AudioInputConfig::new(spec, 8, 480).unwrap())
                .unwrap();
            let source_id = input.source().source_id();
            let stream_id = input.output().stream_id();
            input
                .output()
                .send(session.polled_audio().unwrap())
                .unwrap();
            for sequence in 0..frame_count {
                input.try_write(&[sequence as f32 / 16.0; 480]).unwrap();
            }
            let mut held = input.try_acquire().unwrap();
            held.try_copy_from_slice(&[0.75; 480]).unwrap();
            if producer_closed {
                input.close();
            }

            let mut running = session.start().unwrap();
            // There is no delivery wait between starting and stopping.
            let outcome = running.stop();
            assert!(outcome.is_success(), "{outcome:?}");
            let receipt = running.audio_receipt();
            let mut delivered = 0;
            loop {
                match receipt.try_poll() {
                    Ok(batch) => {
                        for index in 0..batch.len() {
                            let frame = batch.frame(index).unwrap();
                            assert_eq!(frame.lineage().source_id(), source_id);
                            assert_eq!(frame.stream_id(), stream_id);
                            assert_eq!(frame.lineage().sequence_number(), delivered);
                            assert_eq!(frame.samples(), &[delivered as f32 / 16.0; 480]);
                            delivered += 1;
                        }
                    }
                    Err(PolledAudioPollError::Empty) => break,
                    Err(error) => panic!("unexpected terminal read: {error:?}"),
                }
            }
            assert_eq!(delivered, frame_count);
            let observations = input.observations();
            assert!(observations.closed);
            assert!(!observations.cancelled);
            assert_eq!(observations.accepted_total, frame_count);

            let rejected = input
                .try_send(held)
                .expect_err("stop closes live writer admission");
            assert_eq!(rejected.kind(), AudioInputWriteErrorKind::Closed);
            assert_eq!(rejected.into_rejected().unwrap().samples(), &[0.75; 480]);
            assert_eq!(input.observations().accepted_total, frame_count);
        }
    }
}

#[test]
fn given_accepted_pcm_when_session_is_cancelled_then_pending_receipt_is_discarded() {
    let spec = SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved);
    let session = Session::builder()
        .sample_spec(spec)
        .audio_frame_duration(AudioFrameDuration::Ms10)
        .build();
    let mut input = session
        .audio_input(AudioInputConfig::new(spec, 2, 480).unwrap())
        .unwrap();
    input
        .output()
        .send(session.polled_audio().unwrap())
        .unwrap();
    input.try_write(&[0.25; 480]).unwrap();
    let mut running = session.start().unwrap();
    let receipt = running.audio_receipt();
    assert!(running.cancel().is_success());
    assert!(matches!(
        receipt.try_poll(),
        Err(PolledAudioPollError::Empty)
    ));
    assert!(input.observations().closed);
    assert!(input.observations().cancelled);
    assert_eq!(
        input.try_write(&[0.5; 480]).unwrap_err().kind(),
        AudioInputWriteErrorKind::Cancelled
    );
    assert_eq!(input.observations().accepted_total, 1);
}

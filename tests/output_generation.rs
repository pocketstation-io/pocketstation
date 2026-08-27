use std::time::{Duration, Instant};

use pocketstation::{
    AudioInputConfig, AudioInputWriteErrorKind, OutputCancelResult, SampleFormat, SampleSpec,
    Session,
};

const FRAME_SAMPLES: usize = 960;

fn audio_input_config() -> AudioInputConfig {
    AudioInputConfig::new(
        SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
        4,
        FRAME_SAMPLES,
    )
    .expect("valid generated-audio input")
}

#[test]
fn given_replaced_output_when_session_starts_then_only_active_frames_are_delivered() {
    let session = Session::builder()
        .sample_spec(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
        .build();
    let mut output = session
        .audio_input(audio_input_config())
        .expect("application-generated audio input");
    let polled_audio = session.polled_audio().expect("polled audio endpoint");
    output
        .output()
        .send(polled_audio)
        .expect("generated-audio polling route");
    let mut running = session.start().expect("running Session");

    let first = output
        .begin_output_generation()
        .expect("first output generation");
    output
        .try_write_for_output(&first, &vec![-0.5; FRAME_SAMPLES])
        .expect("first accepted frame");
    output
        .try_write_for_output(&first, &vec![-0.25; FRAME_SAMPLES])
        .expect("second accepted frame");
    let first_delivery_deadline = Instant::now() + Duration::from_secs(3);
    while running.audio_observations().frames_delivered_total < 2 {
        assert!(
            Instant::now() < first_delivery_deadline,
            "first output did not reach the bounded endpoint queue"
        );
        std::thread::yield_now();
    }
    assert_eq!(first.cancel(), OutputCancelResult::Cancelled);
    assert_eq!(
        output
            .try_write_for_output(&first, &vec![-0.75; FRAME_SAMPLES])
            .expect_err("inactive output must reject new samples")
            .kind(),
        AudioInputWriteErrorKind::OutputGenerationInactive(first.id())
    );

    let replacement = output
        .begin_output_generation()
        .expect("replacement output generation");
    output
        .try_write_for_output(&replacement, &vec![0.5; FRAME_SAMPLES])
        .expect("replacement frame");

    let deadline = Instant::now() + Duration::from_secs(3);
    let delivered = loop {
        if let Ok(batch) = running.try_poll_audio() {
            if let Some(frame) = batch.frame(0) {
                break (
                    frame.output_generation_id(),
                    frame.samples().first().copied(),
                );
            }
        }
        assert!(
            Instant::now() < deadline,
            "replacement output was not delivered"
        );
        std::thread::yield_now();
    };

    assert_eq!(delivered.0, Some(replacement.id()));
    assert_eq!(delivered.1, Some(0.5));
    assert!(running.try_poll_audio().is_err());

    output.close();
    assert!(running.stop().is_success());
    let observations = output.observations();
    assert_eq!(observations.discarded_output_frames_total, 0);
    assert_eq!(observations.inactive_output_writes_total, 1);
    assert_eq!(
        running.audio_observations().discarded_output_frames_total,
        2
    );
}

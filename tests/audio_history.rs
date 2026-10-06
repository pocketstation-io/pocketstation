use pocketstation::{
    AudioHistory, AudioHistoryConfig, AudioHistoryError, AudioHistoryState, AudioInput,
    AudioInputConfig, RecordingClipWindow, SampleFormat, SampleSpec, Session,
};
use std::time::{Duration, Instant};

fn wait_received(history: &AudioHistory, buffer_count: u64) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while history.observations().unwrap().received_buffers_total < buffer_count {
        assert!(
            Instant::now() < deadline,
            "history worker did not receive expected PCM"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn write(input: &mut AudioInput, value: f32) {
    let mut samples = vec![value; 1920];
    for pair in samples.chunks_exact_mut(2) {
        pair[1] = -value;
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    while input.try_write(&samples).is_err() {
        assert!(Instant::now() < deadline);
        std::thread::yield_now();
    }
}

fn samples(clip: pocketstation::RecordingClip) -> Vec<f32> {
    hound::WavReader::new(std::io::Cursor::new(clip.wav))
        .unwrap()
        .into_samples::<f32>()
        .map(Result::unwrap)
        .collect()
}

#[test]
fn given_live_independent_stems_when_delayed_trigger_reads_then_exact_audio_and_recording_fanout_survive(
) {
    let directory = tempfile::tempdir().unwrap();
    let spec = SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved);
    let session = Session::builder()
        .sample_spec(spec)
        .recording_root(directory.path())
        .build();
    let history = session
        .audio_history(AudioHistoryConfig::default())
        .unwrap();
    let mut app = session
        .audio_input(AudioInputConfig::new(spec, 16, 960).unwrap())
        .unwrap();
    let mut mic = session
        .audio_input(AudioInputConfig::new(spec, 16, 960).unwrap())
        .unwrap();
    let app_id = app.source().source_id();
    let mic_id = mic.source().source_id();
    app.output().retain_audio().unwrap();
    mic.output().retain_audio().unwrap();
    app.output().record("application").unwrap();
    mic.output().record("microphone").unwrap();
    let mut running = session.start().unwrap();
    for index in 0..5 {
        write(&mut app, index as f32 / 10.0);
        write(&mut mic, 0.125);
        wait_received(&history, (index + 1) * 2);
    }
    let stems = history.stems().unwrap();
    let app_stem = stems.iter().find(|stem| stem.source_id == app_id).unwrap();
    let mic_stem = stems.iter().find(|stem| stem.source_id == mic_id).unwrap();
    assert_eq!(app_stem.session_id, running.session_id());
    let origin = app_stem.first_timestamp_ns;
    let window = RecordingClipWindow::around(
        origin + 40_000_000,
        origin + 60_000_000,
        20_000_000,
        60_000_000,
    )
    .unwrap();
    assert!(matches!(
        history.read_clip(app_stem.stem_id, window),
        Err(AudioHistoryError::NotReady)
    ));
    write(&mut app, 0.5);
    write(&mut mic, 0.125);
    wait_received(&history, 12);
    let clip = history.read_clip(app_stem.stem_id, window).unwrap();
    assert_eq!(clip.first_sample_frame, 960);
    assert_eq!(clip.sample_frames, 4800);
    assert_eq!(clip.stem.source_id, app_id);
    assert!(clip.discontinuities.is_empty());
    let pcm = samples(clip);
    for (index, block) in pcm.chunks_exact(1920).enumerate() {
        let value = (index + 1) as f32 / 10.0;
        assert!(block.chunks_exact(2).all(|pair| pair == [value, -value]));
    }
    assert!(samples(
        history
            .read_clip(
                mic_stem.stem_id,
                RecordingClipWindow::new(mic_stem.first_timestamp_ns, mic_stem.final_timestamp_ns)
                    .unwrap()
            )
            .unwrap()
    )
    .chunks_exact(2)
    .all(|pair| pair == [0.125, -0.125]));
    app.close();
    mic.close();
    assert!(running.stop().is_success());
    assert_eq!(
        history.observations().unwrap().state,
        AudioHistoryState::Complete
    );
    let recording = running.recording_outcome().unwrap();
    assert!(recording.stems.iter().all(|stem| stem.written_frames == 6));
    assert!(matches!(
        history.read_clip(
            app_stem.stem_id,
            RecordingClipWindow::new(origin, origin + 500_000_000).unwrap()
        ),
        Err(AudioHistoryError::Ended)
    ));
    assert_eq!(history.observations().unwrap().rejected_buffers_total, 0);
}

#[test]
fn given_long_running_capture_when_limits_evict_then_expiry_and_clear_do_not_stop_capture() {
    let spec = SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved);
    let session = Session::builder().sample_spec(spec).build();
    let history = session
        .audio_history(AudioHistoryConfig {
            retention_ns: 60_000_000,
            max_pcm_bytes: 23_040,
            max_buffers: 3,
        })
        .unwrap();
    let mut input = session
        .audio_input(AudioInputConfig::new(spec, 16, 960).unwrap())
        .unwrap();
    input.output().retain_audio().unwrap();
    let mut running = session.start().unwrap();
    for index in 0..100 {
        write(&mut input, index as f32 / 100.0);
        wait_received(&history, index + 1);
    }
    let stem = history.stems().unwrap().remove(0);
    let observations = history.observations().unwrap();
    assert_eq!(observations.retained_buffers, 3);
    assert_eq!(observations.retained_pcm_bytes, 23040);
    assert_eq!(observations.evicted_buffers_total, 97);
    assert!(matches!(
        history.read_clip(
            stem.stem_id,
            RecordingClipWindow::new(
                stem.first_timestamp_ns,
                stem.first_timestamp_ns + 20_000_000
            )
            .unwrap()
        ),
        Err(AudioHistoryError::Expired)
    ));
    let window = RecordingClipWindow::new(
        stem.final_timestamp_ns - 60_000_000,
        stem.final_timestamp_ns,
    )
    .unwrap();
    assert_eq!(
        history
            .read_clip(stem.stem_id, window)
            .unwrap()
            .sample_frames,
        2880
    );
    history.clear().unwrap();
    assert_eq!(history.observations().unwrap().retained_buffers, 0);
    write(&mut input, 0.5);
    wait_received(&history, 101);
    assert_eq!(history.observations().unwrap().retained_buffers, 1);
    input.close();
    assert!(running.stop().is_success());
}

#[test]
fn given_live_capture_when_session_cancelled_then_retained_audio_is_discarded() {
    let spec = SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved);
    let session = Session::builder().sample_spec(spec).build();
    let history = session
        .audio_history(AudioHistoryConfig::default())
        .unwrap();
    let mut input = session
        .audio_input(AudioInputConfig::new(spec, 16, 960).unwrap())
        .unwrap();
    input.output().retain_audio().unwrap();
    let mut running = session.start().unwrap();
    write(&mut input, 0.25);
    wait_received(&history, 1);
    let stem = history.stems().unwrap().remove(0);
    input.close();
    running.cancel();
    assert_eq!(
        history.observations().unwrap().state,
        AudioHistoryState::Cancelled
    );
    assert_eq!(history.observations().unwrap().retained_pcm_bytes, 0);
    assert!(matches!(
        history.read_clip(
            stem.stem_id,
            RecordingClipWindow::new(stem.first_timestamp_ns, stem.final_timestamp_ns).unwrap()
        ),
        Err(AudioHistoryError::Cancelled)
    ));
}

#[test]
fn given_oversized_pcm_when_history_worker_fails_then_failure_is_visible_before_stop() {
    let spec = SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved);
    let session = Session::builder().sample_spec(spec).build();
    let history = session
        .audio_history(AudioHistoryConfig {
            max_pcm_bytes: 1,
            ..Default::default()
        })
        .unwrap();
    let mut input = session
        .audio_input(AudioInputConfig::new(spec, 16, 960).unwrap())
        .unwrap();
    input.output().retain_audio().unwrap();
    let mut running = session.start().unwrap();
    write(&mut input, 0.25);
    let deadline = Instant::now() + Duration::from_secs(5);
    while history.observations().unwrap().state != AudioHistoryState::Failed {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    let observations = history.observations().unwrap();
    assert_eq!(observations.rejected_buffers_total, 1);
    assert_eq!(observations.retained_pcm_bytes, 0);
    assert!(matches!(
        history.read_clip(
            pocketstation::StemId::new(1),
            RecordingClipWindow::new(0, 20_000_000).unwrap()
        ),
        Err(AudioHistoryError::Failed)
    ));
    input.close();
    assert!(!running.stop().is_success());
}

#[test]
fn given_invalid_or_duplicate_history_when_declared_then_session_fails_before_capture() {
    let session = Session::new();
    assert!(session
        .audio_history(AudioHistoryConfig {
            retention_ns: 0,
            ..Default::default()
        })
        .is_err());
    session
        .audio_history(AudioHistoryConfig::default())
        .unwrap();
    assert!(session
        .audio_history(AudioHistoryConfig::default())
        .is_err());
}

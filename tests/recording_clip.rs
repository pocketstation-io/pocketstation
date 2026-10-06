use pocketstation::{RecordedAudio, RecordingClipWindow};

#[test]
fn given_signal_interval_when_context_requested_then_core_bounds_the_window() {
    let window =
        RecordingClipWindow::around(2_000_000_000, 3_000_000_000, 1_000_000_000, 1_000_000_000)
            .unwrap();
    assert_eq!(window.start_ns(), 1_000_000_000);
    assert_eq!(window.end_ns(), 4_000_000_000);
    let _ = std::mem::size_of::<RecordedAudio>();
}

fn recording_fixture() -> (
    tempfile::TempDir,
    std::path::PathBuf,
    pocketstation::SessionId,
    pocketstation::SourceId,
) {
    use pocketstation::{
        AudioInputConfig, SampleFormat, SampleSpec, Session, SessionRecordingState,
    };
    let dir = tempfile::tempdir().unwrap();
    let spec = SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved);
    let session = Session::builder()
        .recording_root(dir.path())
        .sample_spec(spec)
        .build();
    let mut first = session
        .audio_input(AudioInputConfig::new(spec, 16, 960).unwrap())
        .unwrap();
    let mut second = session
        .audio_input(AudioInputConfig::new(spec, 16, 960).unwrap())
        .unwrap();
    let source_id = first.source().source_id();
    first.output().record("application").unwrap();
    second.output().record("microphone").unwrap();
    let mut running = session.start().unwrap();
    let id = running.session_id();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    for index in 0..10 {
        let mut samples = vec![0.0; 1920];
        for frame in 0..960 {
            let value = (index * 960 + frame) as f32 / 10000.0;
            samples[frame * 2] = value;
            samples[frame * 2 + 1] = -value;
        }
        while first.try_write(&samples).is_err() {
            assert!(std::time::Instant::now() < deadline);
            std::thread::yield_now();
        }
        while second.try_write(&vec![0.125; 1920]).is_err() {
            assert!(std::time::Instant::now() < deadline);
            std::thread::yield_now();
        }
    }
    first.close();
    second.close();
    assert!(running.stop().is_success());
    let outcome = running.recording_outcome().unwrap();
    assert_eq!(outcome.state, SessionRecordingState::Complete);
    assert_eq!(outcome.stems.len(), 2);
    let path = outcome.session_dir.clone();
    (dir, path, id, source_id)
}

#[test]
fn given_two_session_stems_when_trigger_clip_requested_then_samples_and_provenance_stay_independent(
) {
    let (_dir, path, id, source_id) = recording_fixture();
    let recording = RecordedAudio::open(&path, id).unwrap();
    let stem = recording
        .stems()
        .iter()
        .find(|s| s.label == "application")
        .unwrap();
    assert_eq!(stem.source_id, source_id);
    let origin = stem.first_timestamp_ns;
    let window = RecordingClipWindow::around(
        origin + 60_000_000,
        origin + 100_000_000,
        20_000_000,
        20_000_000,
    )
    .unwrap();
    let clip = recording.read_clip(stem.stem_id, window).unwrap();
    assert_eq!(clip.first_sample_frame, 1920);
    assert_eq!(clip.sample_frames, 3840);
    assert_eq!(clip.stem.channels, 2);
    assert_eq!(clip.stem.session_id, id);
    assert_eq!(clip.stem.source_id, source_id);
    let samples: Vec<f32> = hound::WavReader::new(std::io::Cursor::new(clip.wav))
        .unwrap()
        .into_samples::<f32>()
        .map(Result::unwrap)
        .collect();
    assert_eq!(&samples[..4], &[0.192, -0.192, 0.1921, -0.1921]);
    assert_eq!(&samples[samples.len() - 2..], &[0.5759, -0.5759]);
    // Sub-sample request bounds retain exactly one stereo sample frame rather
    // than a buffer, and expose the actual rounded Session-time bounds.
    let tiny = recording
        .read_clip(
            stem.stem_id,
            RecordingClipWindow::new(origin + 20_834, origin + 41_665).unwrap(),
        )
        .unwrap();
    assert_eq!(tiny.first_sample_frame, 1);
    assert_eq!(tiny.sample_frames, 1);
    assert_eq!(tiny.actual.start_ns(), origin + 20_833);
    assert_eq!(tiny.actual.end_ns(), origin + 41_666);
    let mic = recording
        .stems()
        .iter()
        .find(|s| s.label == "microphone")
        .unwrap();
    let mic_clip = recording
        .read_clip(
            mic.stem_id,
            RecordingClipWindow::new(mic.first_timestamp_ns, mic.final_timestamp_ns).unwrap(),
        )
        .unwrap();
    assert!(hound::WavReader::new(std::io::Cursor::new(mic_clip.wav))
        .unwrap()
        .into_samples::<f32>()
        .all(|s| s.unwrap() == 0.125));
}

#[test]
fn given_recording_bounds_when_context_overflows_then_shortening_and_empty_intervals_are_explicit()
{
    let (_dir, path, id, _) = recording_fixture();
    let recording = RecordedAudio::open(&path, id).unwrap();
    let stem = &recording.stems()[0];
    let clip = recording
        .read_clip(
            stem.stem_id,
            RecordingClipWindow::new(0, stem.final_timestamp_ns + 1_000_000_000).unwrap(),
        )
        .unwrap();
    assert_eq!(clip.actual.start_ns(), stem.first_timestamp_ns);
    assert_eq!(clip.sample_frames, 9600);
    assert_eq!(clip.actual.end_ns(), stem.first_timestamp_ns + 200_000_000);
    assert!(matches!(
        recording.read_clip(pocketstation::StemId::new(u64::MAX), clip.requested),
        Err(pocketstation::RecordingClipError::UnknownStem)
    ));
    assert!(matches!(
        recording.read_clip(
            stem.stem_id,
            RecordingClipWindow::new(stem.final_timestamp_ns + 1, stem.final_timestamp_ns + 2)
                .unwrap()
        ),
        Err(pocketstation::RecordingClipError::NoAudio)
    ));
    assert!(RecordingClipWindow::around(u64::MAX - 2, u64::MAX - 1, 0, 9).is_err());
    assert!(RecordingClipWindow::new(0, 120_000_000_001).is_err());
    assert!(RecordedAudio::open(&path, pocketstation::SessionId::new(id.get() + 1)).is_err());
}

#[test]
fn given_inspected_recording_when_pcm_or_manifest_changes_then_replay_fails_closed() {
    let (_dir, path, id, _) = recording_fixture();
    let recording = RecordedAudio::open(&path, id).unwrap();
    let stem = &recording.stems()[0];
    let window =
        RecordingClipWindow::new(stem.first_timestamp_ns, stem.final_timestamp_ns).unwrap();
    let wav = path.join("stems").join(format!("{}.wav", stem.label));
    let bytes = std::fs::read(&wav).unwrap();
    let mut modified = bytes.clone();
    *modified.last_mut().unwrap() ^= 1;
    std::fs::write(&wav, modified).unwrap();
    assert!(matches!(
        recording.read_clip(stem.stem_id, window),
        Err(pocketstation::RecordingClipError::ChangedRecording)
    ));
    std::fs::write(wav, bytes).unwrap();
    let manifest = path.join("manifest.json");
    let mut bytes = std::fs::read(&manifest).unwrap();
    bytes.push(b' ');
    std::fs::write(manifest, bytes).unwrap();
    assert!(matches!(
        recording.read_clip(stem.stem_id, window),
        Err(pocketstation::RecordingClipError::ChangedRecording)
    ));
}

#[test]
fn given_manifest_with_foreign_paths_or_lineage_when_opened_then_core_rejects_it() {
    let (_dir, path, id, _) = recording_fixture();
    let manifest = path.join("manifest.json");
    let original = std::fs::read(&manifest).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&original).unwrap();
    for (field, bad) in [
        ("session_id", serde_json::json!(id.get() + 1)),
        ("sample_format", serde_json::json!("int16")),
        ("source_id", serde_json::Value::Null),
    ] {
        let mut altered = value.clone();
        altered["stems"][0][field] = bad;
        std::fs::write(&manifest, serde_json::to_vec(&altered).unwrap()).unwrap();
        assert!(RecordedAudio::open(&path, id).is_err());
    }
    let mut altered = value.clone();
    altered["stems"][0]["wav_path"] = serde_json::json!("../outside.wav");
    std::fs::write(&manifest, serde_json::to_vec(&altered).unwrap()).unwrap();
    let recording = RecordedAudio::open(&path, id).unwrap();
    let stem = &recording.stems()[0];
    assert!(recording
        .read_clip(
            stem.stem_id,
            RecordingClipWindow::new(stem.first_timestamp_ns, stem.final_timestamp_ns).unwrap()
        )
        .is_err());
    let mut altered = value;
    altered["state"] = serde_json::json!("recording");
    std::fs::write(&manifest, serde_json::to_vec(&altered).unwrap()).unwrap();
    assert!(RecordedAudio::open(&path, id).is_err());
}

#[test]
fn given_invalid_public_clip_interval_when_classified_then_sdk_error_code_is_stable() {
    let error = pocketstation::RecordingClipWindow::new(7, 7).unwrap_err();
    assert_eq!(error.code(), "recording.clip_invalid_window");
    assert_eq!(
        pocketstation::RecordingClipError::UnknownStem.code(),
        "recording.clip_unknown_stem"
    );
    assert_eq!(
        pocketstation::RecordingClipError::ChangedRecording.code(),
        "recording.clip_changed_recording"
    );
}

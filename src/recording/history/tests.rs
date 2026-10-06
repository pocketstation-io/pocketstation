use super::state::HistoryState;
use super::*;
use crate::frame::{ClockDomainId, FrameLineage, SampleFormat, SampleSpec, SessionId, SourceId};

fn spec() -> SampleSpec {
    SampleSpec::new(48_000, 2, SampleFormat::F32Interleaved)
}
fn lineage(sequence_num: u64, timestamp_ns: u64, generation: u32) -> FrameLineage {
    FrameLineage::try_new(
        SessionId::new(8),
        SourceId::new(9),
        StemId::new(10),
        ClockDomainId::new(11),
        sequence_num,
        timestamp_ns,
        20_000_000,
        generation,
        0,
        1,
    )
    .unwrap()
}
fn history() -> HistoryState {
    let mut state = HistoryState::new(AudioHistoryConfig::default());
    state.observations.state = AudioHistoryState::Running;
    state
}

#[test]
fn given_sequence_gap_when_window_crosses_gap_then_missing_context_is_explicit() {
    let mut state = history();
    state
        .push(lineage(0, 0, 1), spec(), 0, &[0.25; 1920])
        .unwrap();
    state
        .push(lineage(2, 40_000_000, 1), spec(), 40_000_000, &[0.5; 1920])
        .unwrap();
    assert!(matches!(
        state.snapshot(
            StemId::new(10),
            RecordingClipWindow::new(0, 60_000_000).unwrap()
        ),
        Err(AudioHistoryError::MissingContext)
    ));
    assert_eq!(state.observations.discontinuities_total, 1);
    let clip = state
        .snapshot(
            StemId::new(10),
            RecordingClipWindow::new(41_000_000, 59_000_000).unwrap(),
        )
        .unwrap();
    assert!(clip.samples.iter().all(|sample| *sample == 0.5));
}

#[test]
fn given_source_reset_when_new_generation_arrives_then_old_audio_is_discarded() {
    let mut state = history();
    state
        .push(lineage(0, 0, 1), spec(), 0, &[0.25; 1920])
        .unwrap();
    state
        .push(lineage(0, 40_000_000, 2), spec(), 40_000_000, &[0.5; 1920])
        .unwrap();
    assert!(matches!(
        state.snapshot(
            StemId::new(10),
            RecordingClipWindow::new(0, 20_000_000).unwrap()
        ),
        Err(AudioHistoryError::Expired)
    ));
    assert_eq!(state.metadata()[0].source_generation, 2);
    assert_eq!(state.observations.source_resets_total, 1);
    assert_eq!(state.observations.retained_pcm_bytes, 7680);
}

#[test]
fn given_foreign_or_nonfinite_audio_when_delivered_then_history_rejects_it() {
    let mut state = history();
    state
        .push(lineage(0, 0, 1), spec(), 0, &[0.0; 1920])
        .unwrap();
    assert!(matches!(
        state.push(
            lineage(1, 20_000_000, 1),
            spec(),
            20_000_000,
            &[f32::NAN; 1920]
        ),
        Err(AudioHistoryError::InvalidFrame)
    ));
    let foreign = FrameLineage::try_new(
        SessionId::new(81),
        SourceId::new(9),
        StemId::new(10),
        ClockDomainId::new(11),
        1,
        20_000_000,
        20_000_000,
        1,
        0,
        1,
    )
    .unwrap();
    assert!(matches!(
        state.push(foreign, spec(), 20_000_000, &[0.0; 1920]),
        Err(AudioHistoryError::InvalidFrame)
    ));
    assert_eq!(state.observations.rejected_buffers_total, 2);
}

#[test]
fn given_fractional_sample_duration_when_many_buffers_arrive_then_clock_does_not_accumulate_rounding_drift(
) {
    let mut state = history();
    let spec = SampleSpec::new(44_100, 1, SampleFormat::F32Interleaved);
    for index in 0..44_100 {
        let timestamp_ns = index * 1_000_000_000 / 44_100;
        state
            .push(lineage(index, timestamp_ns, 1), spec, timestamp_ns, &[0.1])
            .unwrap();
    }
    assert_eq!(state.metadata()[0].final_timestamp_ns, 1_000_000_000);
    assert!(state.observations.retained_buffers <= 4096);
}

#[test]
fn given_fractional_timestamp_when_reading_second_sample_then_index_is_exact() {
    let mut state = history();
    state
        .push(lineage(0, 0, 1), spec(), 0, &[0.1, 0.2, 0.3, 0.4])
        .unwrap();
    let clip = state
        .snapshot(
            StemId::new(10),
            RecordingClipWindow::new(20_834, 41_665).unwrap(),
        )
        .unwrap();
    assert_eq!(clip.first_sample_frame, 1);
    assert_eq!(clip.sample_frames, 1);
    assert_eq!(clip.samples, [0.3, 0.4]);
    assert_eq!(clip.actual.start_ns(), 20_833);
    assert_eq!(clip.actual.end_ns(), 41_666);
}

#[test]
fn given_pcm_and_buffer_limits_when_history_rolls_then_both_limits_remain_hard() {
    let mut state = HistoryState::new(AudioHistoryConfig {
        retention_ns: 60_000_000,
        max_pcm_bytes: 15_360,
        max_buffers: 1,
    });
    for index in 0..100 {
        state
            .push(
                lineage(index, index * 20_000_000, 1),
                spec(),
                index * 20_000_000,
                &[0.0; 1920],
            )
            .unwrap();
        assert!(state.observations.retained_buffers <= 1);
        assert!(state.observations.retained_pcm_bytes <= 15360);
    }
    assert_eq!(state.observations.evicted_buffers_total, 99);
    assert!(matches!(
        state.snapshot(
            StemId::new(10),
            RecordingClipWindow::new(0, 20_000_000).unwrap()
        ),
        Err(AudioHistoryError::Expired)
    ));
    state.clear();
    assert_eq!(state.observations.retained_pcm_bytes, 0);
}

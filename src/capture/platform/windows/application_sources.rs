use std::collections::HashMap;

use crate::capture::{CaptureSource, SourceState, StableSourceId};

pub(super) fn collect_application_sources(
    endpoint_sessions: impl IntoIterator<Item = Vec<CaptureSource>>,
) -> Vec<CaptureSource> {
    let mut sources: HashMap<StableSourceId, CaptureSource> = HashMap::new();
    for source in endpoint_sessions.into_iter().flatten() {
        sources
            .entry(source.stable_id.clone())
            .and_modify(|existing| {
                // One process can render on several endpoints; activity is not endpoint identity.
                if source.state == SourceState::Playing {
                    existing.state = SourceState::Playing;
                }
            })
            .or_insert(source);
    }
    let mut sources: Vec<_> = sources.into_values().collect();
    sources.sort_by(|left, right| left.stable_id.stable_key.cmp(&right.stable_id.stable_key));
    sources
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::SourceKind;
    use crate::frame::Platform;

    fn application(process_id: u32, creation_time_100ns: u64, state: SourceState) -> CaptureSource {
        CaptureSource {
            stable_id: StableSourceId::new(
                Platform::Windows,
                SourceKind::Application,
                format!("wasapi:pid:{process_id}:creation-100ns:{creation_time_100ns}"),
            ),
            name: "Audio application".to_owned(),
            process_id: Some(process_id),
            app_id: Some("org.example.Audio".to_owned()),
            device_uid: None,
            state,
            sample_rate_hz: 48_000,
            channels: 2,
        }
    }

    #[test]
    fn given_two_render_endpoints_when_sessions_are_collected_then_both_processes_are_visible() {
        let sources = collect_application_sources([
            vec![application(41, 100, SourceState::Playing)],
            vec![application(42, 200, SourceState::Playing)],
        ]);
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].process_id, Some(41));
        assert_eq!(sources[1].process_id, Some(42));
    }

    #[test]
    fn given_failed_endpoint_when_other_endpoints_have_sessions_then_results_are_retained() {
        let sources = collect_application_sources([
            vec![application(41, 100, SourceState::Playing)],
            Vec::new(),
            vec![application(42, 200, SourceState::Playing)],
        ]);
        assert_eq!(sources.len(), 2);
    }

    #[test]
    fn given_silent_then_playing_session_when_identity_repeats_then_one_playing_source_is_returned()
    {
        let sources = collect_application_sources([
            vec![application(41, 100, SourceState::Silent)],
            vec![application(41, 100, SourceState::Playing)],
        ]);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].state, SourceState::Playing);
    }

    #[test]
    fn given_playing_then_silent_session_when_identity_repeats_then_activity_is_not_downgraded() {
        let sources = collect_application_sources([
            vec![application(41, 100, SourceState::Playing)],
            vec![application(41, 100, SourceState::Silent)],
        ]);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].state, SourceState::Playing);
    }

    #[test]
    fn given_reused_pid_when_creation_time_differs_then_process_instances_stay_distinct() {
        let sources = collect_application_sources([
            vec![application(41, 100, SourceState::Silent)],
            vec![application(41, 101, SourceState::Playing)],
        ]);
        assert_eq!(sources.len(), 2);
        assert_ne!(sources[0].stable_id, sources[1].stable_id);
    }

    #[test]
    fn given_silent_sessions_when_identity_repeats_then_one_silent_source_is_returned() {
        let sources = collect_application_sources([
            vec![application(41, 100, SourceState::Silent)],
            vec![application(41, 100, SourceState::Silent)],
        ]);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].state, SourceState::Silent);
    }

    #[test]
    fn given_no_endpoint_sessions_when_discovery_is_collected_then_no_process_is_invented() {
        assert!(collect_application_sources([Vec::new(), Vec::new()]).is_empty());
    }
}

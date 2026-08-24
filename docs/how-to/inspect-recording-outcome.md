# Inspect recording outcomes

<!-- claims: CLM-GUIDE-010-SCOPE-001,CLM-GUIDE-010-TEXT-001,CLM-GUIDE-010-TEXT-002,CLM-GUIDE-010-TEXT-003,CLM-GUIDE-010-TEXT-004,CLM-GUIDE-010-TEXT-005,CLM-GUIDE-010-TEXT-006,CLM-GUIDE-010-SOURCE-001 -->

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Inspect recording outcomes** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A stopped Session with its structured `SessionStopOutcome` still available.

## Procedure

1. Retain RunningSession until stop returns.
2. Preserve SessionStopOutcome.
3. Read recording_outcome after stop and locate the schema-versioned recording manifest using the exported file-name constant.
4. Check overall state plus completed and failed stem counts.
5. Use error codes and per-stem results to diagnose partial finalization.

## Concrete repository example

The executable repository test `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` (`test-8c7b0f326da2b4760c28`) shows the concrete API sequence and asserted outcome at `src/recording/endpoint/tests.rs:287`.

```rust
}

#[test]
fn given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed() {
    let temp_dir = TempDir::new().unwrap();
    let coordinator =
        SessionMultistemEndpointCoordinator::new(temp_dir.path(), EndpointGroupId::new(GROUP_ID));
    let receipt = coordinator.receipt();
    let (registry, operator_id, node_type_id) = session_endpoint_registry(coordinator);
    let (mut router, mut receivers, source_nodes, _edge_ids) = router_with_sources(1);
    let prepared = registry
        .prepare_batch(
            &operator_id,
            &node_type_id,
            vec![session_input(
                receivers.pop().unwrap(),
                EndpointId(101),
                StemId(11),
                RouteId(21),
                "application",
                0,
            )],
        )
        .unwrap();
    let (gate_controller, gate) = endpoint_start_gate();
    let mut running = prepared.start(gate).unwrap();
    gate_controller.open();

    router.dispatch_from(
        source_nodes[0],
        "out",
        lineaged_frame_with_permission(31, 11, 0, 4, 0.25),
        1,
    );
    wait_for_received(&running, 1);
    router.dispatch_from(
        source_nodes[0],
        "out",
        lineaged_frame_with_permission(31, 11, 1, 5, 0.5),
        20_000_001,
    );
    wait_for_failure(&running);
    running.request_stop();
    let finalization = running.join_and_finalize();

    assert!(!finalization.is_success());
    assert_eq!(finalization.observations.frames_received_total, 2);
    assert_eq!(finalization.observations.failures_total, 1);
    let outcome = receipt
        .result()
        .expect("failed recording receipt must finalize");
    assert_eq!(outcome.state, RecordingState::Incomplete);
    assert!(outcome.stems[0]
        .error
        .as_deref()
        .is_some_and(|error| error.contains("PermissionEpoch")));
}
```

```bash
cargo test --all-features given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed
```

## Important consequence

A partial recording can coexist with useful completed stems; preserve both sides of the outcome.

## Verify the outcome

Overall state, completed stems, failed stems, and per-stem errors have all been inspected before the outcome is discarded.

Executable evidence selected for **Inspect recording outcomes** is limited to each test's recorded setup and assertions:

- `given_derived_permission_epoch_when_later_frame_changes_it_then_recording_fails_closed` — given derived permission epoch when later frame changes it then recording fails closed (`src/recording/endpoint/tests.rs:287`; `test-8c7b0f326da2b4760c28`).
- `given_recording_codes_when_serialized_then_values_are_exact_and_unique` — given recording codes when serialized then values are exact and unique (`src/recording/error_code.rs:95`; `test-921c1c5c1fdb60c7bf78`).
- `given_queued_audio_when_recording_cancelled_then_wav_header_is_playable_and_manifest_incomplete` — given queued audio when recording cancelled then wav header is playable and manifest incomplete (`src/recording/writer/tests.rs:221`; `test-7f4ab2688e1c16ab56c2`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` — given session context and two first frames when recorded then manifest derives capture lineage and common origin (`src/recording/endpoint/tests.rs:187`; `test-1d7c657b57a9c71d6591`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` — given session recorder input without audio stem origin when prepared then it is rejected (`src/recording/endpoint/tests.rs:262`; `test-a2e8d174434f9a88bf9e`).
- `given_terminal_failure_when_projected_then_code_is_typed` — given terminal failure when projected then code is typed (`src/recording/error_code.rs:158`; `test-41c72d3a0caba393eac7`).
- `given_failed_recorder_branch_when_more_frames_dispatched_then_healthy_branch_continues` — given failed recorder branch when more frames dispatched then healthy branch continues (`src/recording/writer/tests.rs:253`; `test-668e2a246f514118dc91`).
- `given_fractional_stereo_gap_when_silence_is_sized_then_channels_remain_aligned` — given fractional stereo gap when silence is sized then channels remain aligned (`src/recording/writer/tests.rs:99`; `test-991391bffbd7771b8674`).
- `given_timestamp_and_sequence_gap_when_finished_then_silence_and_events_preserve_time` — given timestamp and sequence gap when finished then silence and events preserve time (`src/recording/writer/tests.rs:174`; `test-c0bc88b64402027ef6d4`).
- `given_two_clock_mapped_stems_when_finished_then_two_aligned_playable_wavs_are_written` — given two clock mapped stems when finished then two aligned playable wavs are written (`src/recording/writer/tests.rs:107`; `test-13b59c3a2ed9350468eb`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-21f8c08b6457bb762def`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` — given derived stream chain when compiled then operator output feeds next named input (`src/session/compile/tests.rs:587`; `test-081f9254eabd3bfeaad1`).

## Failure signals

- `pocketstation::recording::writer::RecorderError` — `error-9339dedf4c84c38da0e2`
- `pocketstation::recording::writer::RecorderError` / `DuplicateStemLabel` — `error-d49408f9053c2456c283`
- `pocketstation::recording::writer::RecorderError` / `FrameSpecMismatch` — `error-578da524e06cea0e8e00`
- `pocketstation::recording::writer::RecorderError` / `GapTooLarge` — `error-061391a6f6d35c982334`
- `pocketstation::recording::writer::RecorderError` / `InvalidSampleSpec` — `error-d05cd71eb7b744b378a5`
- `pocketstation::recording::writer::RecorderError` / `InvalidStemLabel` — `error-d435376c2427ddf5f115`
- `pocketstation::recording::writer::RecorderError` / `Io` — `error-c553215c0c3b0de2f46e`
- `pocketstation::recording::writer::RecorderError` / `Json` — `error-2d48aa8fde935c4cbdb5`
- `pocketstation::recording::writer::RecorderError` / `LineageMismatch` — `error-88f9c83dd92f31e2cd0d`
- `pocketstation::recording::writer::RecorderError` / `OutputExists` — `error-3e33d869c826f0e7c489`

## API reference

- [Multistem Recording](/docs/concepts/multistem-recording.md)
- [Recording](/docs/errors/recording.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::recording::config::RecorderStemConfig` | struct | Configures recorder stem behavior at its owning API boundary. | `src/recording/config.rs:55` |
| `pocketstation::recording::config::StemLabel` | struct | Stores the validated human-readable label used for one recording stem. | `src/recording/config.rs:20` |
| `pocketstation::recording::endpoint::MultistemRecordingReceipt` | struct | Retains the identity and observation access returned for multistem recording. | `src/recording/endpoint.rs:28` |
| `pocketstation::recording::endpoint::SessionMultistemEndpointCoordinator` | struct | Canonical Session-owned multistem recorder declaration. | `src/recording/endpoint.rs:49` |
| `pocketstation::recording::writer::DiscontinuityRecord` | struct | Records one immutable discontinuity observation. | `src/recording/writer.rs:93` |
| `pocketstation::recording::writer::MultistemRecording` | struct | Owns the per-stem recording workers and coordinates their terminal finalization outcome. | `src/recording/writer.rs:139` |
| `pocketstation::recording::writer::RecordingObservations` | struct | Reports the recording observations collected at an observation boundary. | `src/recording/writer.rs:131` |
| `pocketstation::recording::writer::RecordingOutcome` | struct | Reports the structured recording outcome. | `src/recording/writer.rs:112` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A recording is incomplete](/docs/troubleshooting/recording-incomplete.md)
- [Recording failures](/docs/errors/recording.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)

## Evidence boundary

The claims on **Inspect recording outcomes** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/recording/writer.rs:17-17` (`DIRECT`)
- `src/recording/writer.rs:18-18` (`DIRECT`)
- `src/recording/writer.rs:19-19` (`DIRECT`)
- `src/recording/writer.rs:20-20` (`DIRECT`)
- `src/recording/writer.rs:21-21` (`DIRECT`)
- `src/recording/writer.rs:23-23` (`DIRECT`)
- `src/recording/writer.rs:23-23` (`DIRECT`)
- `src/recording/writer.rs:23-23` (`DIRECT`)
- `src/recording/writer.rs:23-23` (`DIRECT`)
- `src/recording/writer.rs:23-23` (`DIRECT`)
- `src/recording/writer.rs:23-23` (`DIRECT`)
- `src/recording/writer.rs:24-82` (`DIRECT`)
- `src/recording/writer.rs:26-26` (`DIRECT`)
- `src/recording/writer.rs:26-26` (`DIRECT`)
- `src/recording/writer.rs:28-28` (`DIRECT`)
- `src/recording/writer.rs:28-28` (`DIRECT`)
- `src/recording/writer.rs:30-30` (`DIRECT`)
- `src/recording/writer.rs:30-30` (`DIRECT`)
- `src/recording/writer.rs:32-36` (`DIRECT`)
- `src/recording/writer.rs:33-33` (`DIRECT`)
- `src/recording/writer.rs:34-34` (`DIRECT`)
- `src/recording/writer.rs:35-35` (`DIRECT`)
- `src/recording/writer.rs:38-38` (`DIRECT`)
- `src/recording/writer.rs:38-38` (`DIRECT`)
- `src/session/extensions/recording.rs:1-4` (`DECLARED`)

For **Inspect recording outcomes**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

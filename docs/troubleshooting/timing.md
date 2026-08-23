# Timestamps diverge or discontinuities appear

<!-- claims: CLM-TRBL-016-CAP-001,CLM-TRBL-016-CAP-002,CLM-TRBL-016-CAP-003,CLM-TRBL-016-CAP-004,CLM-TRBL-016-SOURCE-001 -->

## Symptom

Mapped timestamps regress, drift grows, or frame lineage reports a discontinuity.

## Evidenced causes

- Source and Session timestamps were compared without the correct clock-domain mapping.
- The source clock rate changed or accumulated drift.
- Source generation, permission epoch, or discontinuity epoch advanced.
- Timestamp arithmetic is unrepresentable.

## Distinguish the causes

Compare clock-domain IDs, raw source and Session times, mapping generation, drift snapshot, correction, and discontinuity records.

## Diagnostic signals

- `pocketstation::capture::timeline::CaptureSampleTimelineError` (`error-dbe5e36cead65d64cbf7`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `MixedAdvanceModes` (`error-671379ddd61206ef198e`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionMovedBackward` (`error-012287cf4e78fb89426b`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionOverflow` (`error-c17db2e1686bee7e86be`)

## Executable evidence

- `given_large_absolute_timestamps_when_observed_then_relative_drift_stays_precise` exercises given large absolute timestamps when observed then relative drift stays precise under its recorded setup (`test-7eab6b2397d7f488801e`).
- `given_aligned_clocks_when_observed_then_drift_is_near_zero` exercises given aligned clocks when observed then drift is near zero under its recorded setup (`test-6157a8f188bed0df7cf2`).
- `given_faster_runtime_clock_when_observed_then_drift_is_positive` exercises given faster runtime clock when observed then drift is positive under its recorded setup (`test-369a4cb1f110b73815ed`).
- `given_observations_when_snapshotted_then_lineage_metrics_are_reported` exercises given observations when snapshotted then lineage metrics are reported under its recorded setup (`test-ddeaa98b011c7a32a19f`).
- `given_slower_runtime_clock_when_observed_then_drift_is_negative` exercises given slower runtime clock when observed then drift is negative under its recorded setup (`test-b6b0a57d7df5bfbceeee`).
- `given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance` exercises given resources invalidated hresult when classified then failure is not guessed as disappearance under its recorded setup (`test-f7437f6b9062abefafe0`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-805d755d4acd2257ba9b`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-a3a0d044f02b7f664bb9`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` exercises given zero frame capacity when preparing then backend is not prepared under its recorded setup (`test-0afbec4242ea2fad4582`).
- `given_available_capacity_when_frame_is_sent_then_stream_preserves_frame` exercises given available capacity when frame is sent then stream preserves frame under its recorded setup (`test-8f4bb6c6c11e1d2947a7`).
- `given_closed_start_gate_when_frame_is_sent_then_frame_is_discarded_and_counted` exercises given closed start gate when frame is sent then frame is discarded and counted under its recorded setup (`test-a59f34c85fd9d74e587e`).
- `given_full_stream_when_frame_is_sent_then_newest_is_dropped_and_counted` exercises given full stream when frame is sent then newest is dropped and counted under its recorded setup (`test-bb4e6d290a21c545166a`).

## Corrective action

Rebuild the mapping at a real observation boundary and apply only the controller's evidenced bounds while retaining original lineage.

## Retry and incomplete state

Do not retry by rewriting old timestamps. Downstream data around a discontinuity may need an explicit gap or new segment.

## Related reference

- [Timing And Clocks](/docs/concepts/timing-and-clocks.md)
- [Timing](/docs/reference/timing.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)
- [A capture source disappears](/docs/troubleshooting/source-loss.md)
- [No application audio arrives](/docs/troubleshooting/no-application-audio.md)
- [No microphone audio arrives](/docs/troubleshooting/no-microphone-audio.md)

## Evidence boundary

The claims on **Timestamps diverge or discontinuities appear** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/timing/clock_drift.rs:1-175` (`DIRECT`)
- `src/capture/timeline.rs:1-130` (`DIRECT`)

For **Timestamps diverge or discontinuities appear**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

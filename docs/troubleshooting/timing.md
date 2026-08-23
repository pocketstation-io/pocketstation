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

- `pocketstation::capture::timeline::CaptureSampleTimelineError` (`error-5f1691a6289a2b6fdd03`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `MixedAdvanceModes` (`error-cc2b1a61e9dee283b627`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionMovedBackward` (`error-3812c0f164d9b3c8357b`)
- `pocketstation::capture::timeline::CaptureSampleTimelineError` / `SourcePositionOverflow` (`error-51de207a9656d496b4d1`)

## Executable evidence

- `given_large_absolute_timestamps_when_observed_then_relative_drift_stays_precise` exercises given large absolute timestamps when observed then relative drift stays precise under its recorded setup (`test-62316896388e623801b8`).
- `given_aligned_clocks_when_observed_then_drift_is_near_zero` exercises given aligned clocks when observed then drift is near zero under its recorded setup (`test-5c4146a598e6cdc11175`).
- `given_faster_runtime_clock_when_observed_then_drift_is_positive` exercises given faster runtime clock when observed then drift is positive under its recorded setup (`test-24a29769eb9c240f93a1`).
- `given_observations_when_snapshotted_then_lineage_metrics_are_reported` exercises given observations when snapshotted then lineage metrics are reported under its recorded setup (`test-da38283fb00d196f31c4`).
- `given_slower_runtime_clock_when_observed_then_drift_is_negative` exercises given slower runtime clock when observed then drift is negative under its recorded setup (`test-eff93c107acb8107fb7d`).
- `given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance` exercises given resources invalidated hresult when classified then failure is not guessed as disappearance under its recorded setup (`test-acc6963aea9a1e14e631`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-8de0974346f9110044c2`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` exercises given zero frame capacity when preparing then backend is not prepared under its recorded setup (`test-f42d54d3bd1632c2ccfa`).
- `given_available_capacity_when_frame_is_sent_then_stream_preserves_frame` exercises given available capacity when frame is sent then stream preserves frame under its recorded setup (`test-82e1dcd18071b5ef2f92`).
- `given_closed_start_gate_when_frame_is_sent_then_frame_is_discarded_and_counted` exercises given closed start gate when frame is sent then frame is discarded and counted under its recorded setup (`test-fd2a36cb4c8a774eb98b`).
- `given_full_stream_when_frame_is_sent_then_newest_is_dropped_and_counted` exercises given full stream when frame is sent then newest is dropped and counted under its recorded setup (`test-bce087e83b434cd19363`).

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

The claims on **Timestamps diverge or discontinuities appear** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/timing/clock_drift.rs:1-175` (`DIRECT`)
- `src/capture/timeline.rs:1-120` (`DIRECT`)

For **Timestamps diverge or discontinuities appear**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

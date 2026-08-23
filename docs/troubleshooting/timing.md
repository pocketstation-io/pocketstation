# Timestamps diverge or discontinuities appear

<!-- claims: CLM-TRBL-016-CAP-001,CLM-TRBL-016-CAP-002,CLM-TRBL-016-CAP-003,CLM-TRBL-016-CAP-004,CLM-TRBL-016-SOURCE-001 -->

Use this page when you observe **timestamps diverge or discontinuities appear**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Compare clock-domain IDs, source and Session timestamps, drift snapshots, corrections, generations, and discontinuity records before altering a mapping.

## Diagnostic signals

- `pocketstation::capture::events::CaptureRuntimeFailure` (`error-11b972ad42d5de880e06`)
- `pocketstation::frame::pool::AudioBufferWriteError` / `CapacityExceeded` (`error-2317926ecc3df1fe0485`)
- `pocketstation::frame::lineage::FrameLineageBuildError` / `ZeroSourceGeneration` (`error-2333fb8ed9ffc64dfe3d`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `BackendClass` (`error-29e952ae7432566a9e95`)
- `pocketstation::frame::lineage::FrameLineageBuildError` / `ZeroDuration` (`error-36112cc71bb577df5cc6`)
- `pocketstation::capture::authorization::CaptureError` / `CaptureWorkerPanicked` (`error-365f9b6fbda74eb0d631`)
- `pocketstation::capture::authorization::CaptureError` / `PermissionDenied` (`error-38030156125346a8e892`)
- `pocketstation::capture::authorization::CaptureError` / `NotSupported` (`error-3b4b5393164d9f6f12a5`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `PlatformStatus` (`error-3c6fcc22deb2f54788ba`)
- `pocketstation::frame::audio::AudioFrameBuildError` / `ZeroSampleRate` (`error-3d530ffcc82f2ae60152`)
- `pocketstation::frame::pool::AudioBufferWriteError` (`error-44d619f15116bb8d5f0e`)
- `pocketstation::frame::audio::AudioFrameBuildError` (`error-47bd33a1cf3d0c5fa264`)
- `pocketstation::capture::authorization::CaptureError` / `SourceUnavailable` (`error-71c87f975acc9e22a402`)
- `pocketstation::frame::lineage::FrameLineageBuildError` (`error-886f021bf510039ccdbb`)
- `pocketstation::capture::authorization::CaptureError` / `BackendSetupRequired` (`error-8db0fec69a9c7158ffdf`)
- `pocketstation::capture::authorization::CaptureError` (`error-96ffe4bc4254583d1e17`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` / `SourceInstanceExited` (`error-a9c0f7dfff744e9ba6b7`)
- `pocketstation::capture::authorization::CaptureError` / `BackendInit` (`error-b320ea1cba2b3c8dc4c7`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidStreamCapacity` (`error-bcf5d4d897b6bd0784bf`)
- `pocketstation::frame::lineage::FrameLineageBuildError` / `TimestampOverflow` (`error-bd9d2580f5c500ca2920`)
- `pocketstation::capture::authorization::CaptureError` / `ModeUnsupported` (`error-bf1be2fb486df6136dc5`)
- `pocketstation::capture::authorization::CaptureError` / `InvalidRuntimeEventCapacity` (`error-ceedf8c06740748c9bd5`)
- `pocketstation::capture::authorization::CaptureError` / `BackendStatus` (`error-e8046b5b5989518ee482`)
- `pocketstation::capture::events::CaptureRuntimeFailureClass` (`error-ea2d5a94280522f41764`)
- `pocketstation::frame::audio::AudioFrameBuildError` / `ZeroChannels` (`error-ec0790bb6edfcc3d5058`)
- `pocketstation::frame::audio::AudioFrameBuildError` / `MisalignedSamples` (`error-fd6606b3c0707d21bb0f`)

## Executable evidence

- `given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance` exercises given resources invalidated hresult when classified then failure is not guessed as disappearance under its recorded setup (`test-acc6963aea9a1e14e631`).
- `given_large_absolute_timestamps_when_observed_then_relative_drift_stays_precise` exercises given large absolute timestamps when observed then relative drift stays precise under its recorded setup (`test-62316896388e623801b8`).
- `frame_stream_closed` exercises frame stream closed under its recorded setup (`test-3ab763bff0cd08d4b4e1`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` exercises given active capture when owner is dropped then backend is reclaimed under its recorded setup (`test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` exercises given active capture when stopped then backend is joined under its recorded setup (`test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` exercises given backend frame when source differs from open identity then lineage fails closed under its recorded setup (`test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` exercises given panicking capture worker when joined then typed failure is returned under its recorded setup (`test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` exercises given prepared capture when opened then bounded delivery is owned under its recorded setup (`test-8de0974346f9110044c2`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` exercises given zero frame capacity when preparing then backend is not prepared under its recorded setup (`test-f42d54d3bd1632c2ccfa`).
- `join_capture_worker` exercises join capture worker under its recorded setup (`test-89b10abefa1f5c9a47e2`).
- `observations` exercises observations under its recorded setup (`test-6c30e98c2843011d2b2e`).
- `prepare_capture` exercises prepare capture under its recorded setup (`test-59d7e50bbae31896948a`).
- `observations` exercises observations under its recorded setup (`test-09066e0a4bfc4d299258`).
- `publish_backend_failure` exercises publish backend failure under its recorded setup (`test-d6ee1878cb3cf2d3f452`).
- `capacity_frames` exercises capacity frames under its recorded setup (`test-22dcc753009aea57ca51`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/timing/clock_drift.rs:1-175` (`DIRECT`)
- `src/capture/timeline.rs:1-120` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

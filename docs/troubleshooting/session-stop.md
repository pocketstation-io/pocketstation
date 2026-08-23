# Session stop reports component failures

<!-- claims: CLM-TRBL-012-CAP-001,CLM-TRBL-012-CAP-002,CLM-TRBL-012-CAP-003,CLM-TRBL-012-CAP-004,CLM-TRBL-012-CAP-005,CLM-TRBL-012-CAP-006,CLM-TRBL-012-SOURCE-001 -->

Use this page when you observe **session stop reports component failures**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Read component, rollback, endpoint, source, recording, sidecar, and finalization failures from the terminal outcome instead of reducing stop to one status flag.

## Diagnostic signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` (`error-00e5716261eba0f8cf3d`)
- `pocketstation::session::error::SessionError` / `UnknownStem` (`error-00f6e798d158df66c847`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` (`error-01d3fc855e2a00319076`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` (`error-023d6ab0b23a50a614ff`)
- `pocketstation::endpoint::runtime::EndpointFailureStage` / `CancelPreparation` (`error-0265bb447764629fa47b`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` (`error-0279b2b6b0cb3b5801bc`)
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroLeaseCapacity` (`error-0370b7ecbdf2b9d6fbdb`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` (`error-037ddc3e193da74177f8`)
- `pocketstation::recording::error_code::RecordingErrorCode` / `PermissionDenied` (`error-059bf10da1dcb4446e68`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` (`error-05c60389efcb84311921`)
- `pocketstation::connector::error::ConnectorErrorCodeError` / `TooLong` (`error-06f5c52aa07c86ca5062`)
- `pocketstation::session::prepare::error::SessionPrepareError` (`error-085082b521c14e5ecd1e`)
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` (`error-08a7536094bfb2242b17`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `InvalidSampleCount` (`error-093c41e2489cf1bb258d`)
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` (`error-09837185c7fca0f70618`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` (`error-0b1f3a3357a77fcef185`)
- `pocketstation::connector::error::ConnectorErrorCodeError` / `Empty` (`error-0b71c9f1b1489e0d4f9a`)
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` (`error-0bc2f7c0b9f9dbf8ddd7`)
- `pocketstation::connector::error::ConnectorErrorBuildError` (`error-0bc8adb0641971704f74`)
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `ZeroCapacity` (`error-0bd6f58be40ade9a01fe`)
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `QueueCapacityTooLarge` (`error-0bed26cd5cd9ccfe0b20`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `SequenceGap` (`error-0c04a3eedb823da29323`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `TooManyFields` (`error-0c83ebde568152ad3edf`)
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingExternalAudioIngress` (`error-0cc0ae8a8cc4f1e05996`)
- `pocketstation::session::lifecycle::engine::SessionEngineBuildError` / `DuplicateSidecarId` (`error-0ce1015c73b65576cbeb`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `TimestampRegression` (`error-0d567cf627daa0adfee1`)
- `pocketstation::endpoint::registry::EndpointDriverRegistryError` / `OperatorNodeTypeConflict` (`error-0db6114718e1d213362f`)
- `pocketstation::session::error_code::SessionStartErrorCode` / `MissingEndpointDeclaration` (`error-0e46a3d13215bfc3898f`)
- `pocketstation::connector::error::ConnectorErrorStage` / `Startup` (`error-0e62627edef059ecab22`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `InvalidManifestRevision` (`error-10517744910e14c23fc4`)

## Executable evidence

- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` exercises given stop and join failures when finalized then both failures and observations remain true under its recorded setup (`test-20a4c27d70a60c9bc881`).
- `given_no_failures_when_terminal_then_state_is_stopped` exercises given no failures when terminal then state is stopped under its recorded setup (`test-6dd97c870cc349f825f9`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` exercises given grouped connector when session stops then one worker is joined and observed under its recorded setup (`test-aa2345c7b9339f742b48`).
- `given_worker_failure_or_panic_when_session_stops_then_endpoint_finalization_is_terminal` exercises given worker failure or panic when session stops then endpoint finalization is terminal under its recorded setup (`test-e56e88c9e99290ea720a`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` exercises given stopped public session when new session starts then capture restarts cleanly under its recorded setup (`test-cbee0768bffa592adde2`).
- `given_closed_start_gate_when_endpoint_starts_then_delivery_waits_until_session_opens_gate` exercises given closed start gate when endpoint starts then delivery waits until session opens gate under its recorded setup (`test-cd73c609f0b99f88ac58`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` exercises given session context and two first frames when recorded then manifest derives capture lineage and common origin under its recorded setup (`test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` exercises given session recorder input without audio stem origin when prepared then it is rejected under its recorded setup (`test-497452363244c581f9e6`).
- `session_dir` exercises session dir under its recorded setup (`test-6d6ced88f99690c75bed`).
- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` exercises given compiled lineaged edge when worker runs then exact session stem is preserved under its recorded setup (`test-6615dcd3b3105010af0b`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` exercises given cloned stem when session frozen then mutation is rejected under its recorded setup (`test-1682e00b3166c4846a92`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` exercises given derived stream when through called again then chain is preserved in session spec under its recorded setup (`test-837dd73be7d5c552ef15`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` exercises given operator when declared then session scoped instance and routes are preserved under its recorded setup (`test-69203660038a41959c14`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` exercises given unrouted stem when session frozen then validation fails closed under its recorded setup (`test-1633b6167eec91db04e2`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` exercises given endpoint operator id when imported from session then endpoint contract type is reexported under its recorded setup (`test-c1047cbdeb5a7bf9bc3b`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Treat stop outcomes as data](/docs/best-practices/terminal-outcomes.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Lifecycle evidence index](/docs/reference/lifecycle-evidence.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/running.rs:1-2612` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

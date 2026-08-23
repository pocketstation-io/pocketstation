# Session stop reports component failures

<!-- claims: CLM-TRBL-012-CAP-001,CLM-TRBL-012-CAP-002,CLM-TRBL-012-CAP-003,CLM-TRBL-012-CAP-004,CLM-TRBL-012-CAP-005,CLM-TRBL-012-CAP-006,CLM-TRBL-012-SOURCE-001 -->

## Symptom

`RunningSession::stop` returns one or more component or finalization failures.

## Evidenced causes

- A source, route, operator, endpoint, connector, or sidecar fails during stop.
- Recording or trace finalization fails after runtime work completed.
- A worker panics, misses a deadline, or cannot join.

## Distinguish the causes

Inspect the terminal outcome by component and stage. Then inspect recording and trace outcomes separately from the top-level Session stop state.

## Diagnostic signals

- `pocketstation::endpoint::runtime::EndpointFailureStage` / `RequestStop` (`error-abbdadb6f9b78cb87e0d`)
- `pocketstation::session::error_code::SessionStopFailureCode` (`error-c819beb61426d30b0bca`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `CaptureFinalizationFailed` (`error-53f7d816c0e63e1f527a`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `EndpointFinalizationFailed` (`error-c7df5634de451ca08e81`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `LineageFailed` (`error-250795b1b91e249ef18e`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `OperatorFinalizationFailed` (`error-77598e8a7dd0e452b692`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `RuntimeFailed` (`error-98eb7a5a8f957d81e458`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `RuntimeWorkerPanicked` (`error-abdc2a4eb41a3179fc4a`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `SourceSendRejected` (`error-615419d7cd667075362a`)
- `pocketstation::connector::ConnectorDeclarationError` / `Configuration` (`error-79dde5b66dc8ba41f246`)
- `pocketstation::connector::ConnectorDeclarationError` / `Session` (`error-036c74e429b61ebdf7cd`)
- `pocketstation::connector::ConnectorDeclarationError` / `WrongSession` (`error-29964fc5e23bb4431977`)

## Executable evidence

- `given_process_instance_selector_when_capture_mode_built_then_exact_identity_is_preserved` exercises given process instance selector when capture mode built then exact identity is preserved under its recorded setup (`test-dac823b98be9f727652f`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` exercises given stop and join failures when finalized then both failures and observations remain true under its recorded setup (`test-20a4c27d70a60c9bc881`).
- `given_no_failures_when_terminal_then_state_is_stopped` exercises given no failures when terminal then state is stopped under its recorded setup (`test-6dd97c870cc349f825f9`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` exercises given grouped connector when session stops then one worker is joined and observed under its recorded setup (`test-92f5704ec6ee88e59fd8`).
- `given_worker_failure_or_panic_when_session_stops_then_endpoint_finalization_is_terminal` exercises given worker failure or panic when session stops then endpoint finalization is terminal under its recorded setup (`test-40ed6b8bb8e60b0fd01a`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` exercises given stopped public session when new session starts then capture restarts cleanly under its recorded setup (`test-6a5d630191363f7e4442`).
- `given_closed_start_gate_when_endpoint_starts_then_delivery_waits_until_session_opens_gate` exercises given closed start gate when endpoint starts then delivery waits until session opens gate under its recorded setup (`test-cd73c609f0b99f88ac58`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` exercises given session context and two first frames when recorded then manifest derives capture lineage and common origin under its recorded setup (`test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` exercises given session recorder input without audio stem origin when prepared then it is rejected under its recorded setup (`test-497452363244c581f9e6`).
- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` exercises given compiled lineaged edge when worker runs then exact session stem is preserved under its recorded setup (`test-6615dcd3b3105010af0b`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` exercises given cloned stem when session frozen then mutation is rejected under its recorded setup (`test-1682e00b3166c4846a92`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` exercises given derived stream when through called again then chain is preserved in session spec under its recorded setup (`test-837dd73be7d5c552ef15`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` exercises given operator when declared then session scoped instance and routes are preserved under its recorded setup (`test-69203660038a41959c14`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` exercises given unrouted stem when session frozen then validation fails closed under its recorded setup (`test-1633b6167eec91db04e2`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` exercises given endpoint operator id when imported from session then endpoint contract type is reexported under its recorded setup (`test-c1047cbdeb5a7bf9bc3b`).

## Corrective action

Preserve every failure record and any completed outputs. Correct the owning component before starting a new Session.

## Retry and incomplete state

A stop failure does not establish a safe restart from the same resources. Frames, files, acknowledgements, and traces can be independently complete or partial.

## Related reference

- [Terminal Outcomes](/docs/lifecycle/terminal-outcomes.md)
- [Session](/docs/errors/session.md)

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

The claims on **Session stop reports component failures** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/running.rs:1-2612` (`DIRECT`)

For **Session stop reports component failures**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

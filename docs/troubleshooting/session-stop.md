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

- `pocketstation::endpoint::runtime::EndpointFailureStage` / `RequestStop` (`error-68d1b02f8922523bb0a5`)
- `pocketstation::session::error_code::SessionStopFailureCode` (`error-c5feda3f1742df7b5b6a`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `CaptureFinalizationFailed` (`error-88f42bd7ff73f9430d0a`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `EndpointFinalizationFailed` (`error-fd6b81edd5d97b10d1bb`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `LineageFailed` (`error-8077d8550164a6f6913c`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `OperatorFinalizationFailed` (`error-8967cd79defb3016add3`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `RuntimeFailed` (`error-18b7ce3e762d2be66a3b`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `RuntimeWorkerPanicked` (`error-bfc126bdcbfa75e3ef6d`)
- `pocketstation::session::error_code::SessionStopFailureCode` / `SourceSendRejected` (`error-499f393474f74d1fef7d`)
- `pocketstation::connector::ConnectorDeclarationError` / `Configuration` (`error-f85c1437daae474702be`)
- `pocketstation::connector::ConnectorDeclarationError` / `Session` (`error-3d1f996891195230a51b`)
- `pocketstation::connector::ConnectorDeclarationError` / `WrongSession` (`error-4fedfe91825f69358fac`)

## Executable evidence

- `given_process_instance_selector_when_capture_mode_built_then_exact_identity_is_preserved` exercises given process instance selector when capture mode built then exact identity is preserved under its recorded setup (`test-284127121760cbb5874f`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` exercises given stop and join failures when finalized then both failures and observations remain true under its recorded setup (`test-da6484ed83753b351441`).
- `given_no_failures_when_terminal_then_state_is_stopped` exercises given no failures when terminal then state is stopped under its recorded setup (`test-74456aeb5a4f8bda5b30`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` exercises given grouped connector when session stops then one worker is joined and observed under its recorded setup (`test-2a1b6ff7d4015d418fc1`).
- `given_worker_failure_or_panic_when_session_stops_then_endpoint_finalization_is_terminal` exercises given worker failure or panic when session stops then endpoint finalization is terminal under its recorded setup (`test-8ebafe380e15caf3c545`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` exercises given stopped public session when new session starts then capture restarts cleanly under its recorded setup (`test-7d6c18ed486400271167`).
- `given_closed_start_gate_when_endpoint_starts_then_delivery_waits_until_session_opens_gate` exercises given closed start gate when endpoint starts then delivery waits until session opens gate under its recorded setup (`test-20a233e58338864c5c2f`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` exercises given session context and two first frames when recorded then manifest derives capture lineage and common origin under its recorded setup (`test-1d7c657b57a9c71d6591`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` exercises given session recorder input without audio stem origin when prepared then it is rejected under its recorded setup (`test-a2e8d174434f9a88bf9e`).
- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` exercises given compiled lineaged edge when worker runs then exact session stem is preserved under its recorded setup (`test-9e1c8ad04d302a8bf88b`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` exercises given cloned stem when session frozen then mutation is rejected under its recorded setup (`test-2cf3d98ffa38e0f5ee68`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` exercises given derived stream when through called again then chain is preserved in session spec under its recorded setup (`test-aec2c4ee7ff8efede00a`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` exercises given operator when declared then session scoped instance and routes are preserved under its recorded setup (`test-e84db4efcd6a7145550a`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` exercises given unrouted stem when session frozen then validation fails closed under its recorded setup (`test-8e301580cdd23a244478`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` exercises given endpoint operator id when imported from session then endpoint contract type is reexported under its recorded setup (`test-0b2dadbe3265dde022e4`).

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

The claims on **Session stop reports component failures** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/running.rs:1-2625` (`DIRECT`)

For **Session stop reports component failures**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

# Stop a Session and inspect failures

<!-- claims: CLM-GUIDE-022-CAP-001,CLM-GUIDE-022-CAP-002,CLM-GUIDE-022-CAP-003,CLM-GUIDE-022-CAP-004,CLM-GUIDE-022-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Stop a Session and inspect failures** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

The `RunningSession` owner and any observation, recording, or trace handles whose terminal state you need.

## Procedure

1. Retain RunningSession as runtime owner.
2. Request stop once application work ends.
3. Read component failures in SessionStopOutcome.
4. Read recording and trace finalization separately.
5. Preserve diagnostics before releasing ownership.

## Important consequence

Request stop once; do not discard partial outcomes because the top-level state is unsuccessful.

## Verify the outcome

The stop outcome, component failures, recording result, and trace finalization have all been retained and classified.

Executable evidence selected for **Stop a Session and inspect failures** is limited to each test's recorded setup and assertions:

- `given_process_instance_selector_when_capture_mode_built_then_exact_identity_is_preserved` — given process instance selector when capture mode built then exact identity is preserved (`src/session/lifecycle/running.rs:2589`; `test-dac823b98be9f727652f`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-20a4c27d70a60c9bc881`).
- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-6dd97c870cc349f825f9`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` — given stopped public session when new session starts then capture restarts cleanly (`tests/session_facade.rs:122`; `test-6a5d630191363f7e4442`).
- `given_closed_start_gate_when_endpoint_starts_then_delivery_waits_until_session_opens_gate` — given closed start gate when endpoint starts then delivery waits until session opens gate (`src/endpoint/registry/tests.rs:250`; `test-cd73c609f0b99f88ac58`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` — given session context and two first frames when recorded then manifest derives capture lineage and common origin (`src/recording/endpoint/tests.rs:187`; `test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` — given session recorder input without audio stem origin when prepared then it is rejected (`src/recording/endpoint/tests.rs:262`; `test-497452363244c581f9e6`).
- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` — given compiled lineaged edge when worker runs then exact session stem is preserved (`src/runtime/signal/operator.rs:2208`; `test-6615dcd3b3105010af0b`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1263`; `test-1682e00b3166c4846a92`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` — given derived stream when through called again then chain is preserved in session spec (`src/session/declaration/draft.rs:1179`; `test-837dd73be7d5c552ef15`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` — given operator when declared then session scoped instance and routes are preserved (`src/session/declaration/draft.rs:1317`; `test-69203660038a41959c14`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` — given unrouted stem when session frozen then validation fails closed (`src/session/declaration/draft.rs:1305`; `test-1633b6167eec91db04e2`).

## Failure signals

- `pocketstation::endpoint::runtime::EndpointFailureStage` / `RequestStop` — `error-abbdadb6f9b78cb87e0d`
- `pocketstation::session::error_code::SessionStopFailureCode` — `error-c819beb61426d30b0bca`
- `pocketstation::session::error_code::SessionStopFailureCode` / `CaptureFinalizationFailed` — `error-53f7d816c0e63e1f527a`
- `pocketstation::session::error_code::SessionStopFailureCode` / `EndpointFinalizationFailed` — `error-c7df5634de451ca08e81`
- `pocketstation::session::error_code::SessionStopFailureCode` / `LineageFailed` — `error-250795b1b91e249ef18e`
- `pocketstation::session::error_code::SessionStopFailureCode` / `OperatorFinalizationFailed` — `error-77598e8a7dd0e452b692`
- `pocketstation::session::error_code::SessionStopFailureCode` / `RuntimeFailed` — `error-98eb7a5a8f957d81e458`
- `pocketstation::session::error_code::SessionStopFailureCode` / `RuntimeWorkerPanicked` — `error-abdc2a4eb41a3179fc4a`
- `pocketstation::session::error_code::SessionStopFailureCode` / `SourceSendRejected` — `error-615419d7cd667075362a`
- `pocketstation::endpoint::runtime::EndpointFailureStage` — `error-7133dd16a1f2e78d47e7`

## API reference

- [Session Lifecycle](/docs/concepts/session-lifecycle.md)
- [Terminal Outcomes](/docs/lifecycle/terminal-outcomes.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::error_code::SessionStopCode` | enum | Stable language-neutral status for an idempotent Session stop. | `src/session/error_code.rs:150` |
| `pocketstation::session::error_code::SessionStopFailureCode` | enum | Stable language-neutral cause retained by a failed Session stop. | `src/session/error_code.rs:171` |
| `pocketstation::session::error_code::session_stop_failure_codes` | function | Returns every stable failure code carried by a Session stop result. | `src/session/error_code.rs:265` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::RequestStop` | variant | Identifies the request stop state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:160` |
| `pocketstation::session::error_code::SessionStopCode::AlreadyStopped` | variant | Indicates that the operation had already stopped. | `src/session/error_code.rs:152` |
| `pocketstation::session::error_code::SessionStopCode::StopFailed` | variant | Represents the stop failed alternative defined by `SessionStopCode`. | `src/session/error_code.rs:153` |
| `pocketstation::session::error_code::SessionStopCode::Stopped` | variant | Indicates that the operation stopped normally. | `src/session/error_code.rs:151` |
| `pocketstation::session::error_code::SessionStopFailureCode::CaptureFinalizationFailed` | variant | Reported when the owning operation encounters capture finalization failed. | `src/session/error_code.rs:173` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)
- [Treat stop outcomes as data](/docs/best-practices/terminal-outcomes.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [A recording is incomplete](/docs/troubleshooting/recording-incomplete.md)

## Evidence boundary

The claims on **Stop a Session and inspect failures** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/running.rs:1-2612` (`DIRECT`)

For **Stop a Session and inspect failures**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

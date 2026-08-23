# Stop a Session and inspect failures

<!-- claims: CLM-GUIDE-022-CAP-001,CLM-GUIDE-022-CAP-002,CLM-GUIDE-022-CAP-003,CLM-GUIDE-022-CAP-004,CLM-GUIDE-022-SOURCE-001 -->

## Scope

- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Retain RunningSession as runtime owner.
2. Request stop once application work ends.
3. Read component failures in SessionStopOutcome.
4. Read recording and trace finalization separately.
5. Preserve diagnostics before releasing ownership.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::error_code::SessionStopCode` | enum | Stable language-neutral status for an idempotent Session stop. | `src/session/error_code.rs:150` |
| `pocketstation::session::error_code::SessionStopFailureCode` | enum | Stable language-neutral cause retained by a failed Session stop. | `src/session/error_code.rs:171` |
| `pocketstation::session::lifecycle::start_contract::SessionStopOutcome` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:308` |
| `pocketstation::endpoint::runtime::EndpointFailureStage::RequestStop` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/endpoint/runtime.rs:160` |
| `pocketstation::session::error_code::SessionStopCode::AlreadyStopped` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:152` |
| `pocketstation::session::error_code::SessionStopCode::StopFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:153` |
| `pocketstation::session::error_code::SessionStopCode::Stopped` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:151` |
| `pocketstation::session::error_code::SessionStopFailureCode::CaptureFinalizationFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:173` |
| `pocketstation::session::error_code::SessionStopFailureCode::EndpointFinalizationFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:175` |
| `pocketstation::session::error_code::SessionStopFailureCode::LineageFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:177` |
| `pocketstation::session::error_code::SessionStopFailureCode::OperatorFinalizationFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:174` |
| `pocketstation::session::error_code::SessionStopFailureCode::RuntimeFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:176` |
| `pocketstation::session::error_code::SessionStopFailureCode::RuntimeWorkerPanicked` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:172` |
| `pocketstation::session::error_code::SessionStopFailureCode::SourceSendRejected` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/error_code.rs:178` |
| `pocketstation::session::lifecycle::events::SessionFinalizationStage::RequestEndpointStop` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/events.rs:43` |
| `pocketstation::session::lifecycle::events::SessionFinalizationStage::StopCapture` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/events.rs:40` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-20a4c27d70a60c9bc881`).
- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-6dd97c870cc349f825f9`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` — given stopped public session when new session starts then capture restarts cleanly (`tests/session_facade.rs:124`; `test-cbee0768bffa592adde2`).
- `given_closed_start_gate_when_endpoint_starts_then_delivery_waits_until_session_opens_gate` — given closed start gate when endpoint starts then delivery waits until session opens gate (`src/endpoint/registry/tests.rs:250`; `test-cd73c609f0b99f88ac58`).
- `given_session_context_and_two_first_frames_when_recorded_then_manifest_derives_capture_lineage_and_common_origin` — given session context and two first frames when recorded then manifest derives capture lineage and common origin (`src/recording/endpoint/tests.rs:187`; `test-9352f7a742c1f649857a`).
- `given_session_recorder_input_without_audio_stem_origin_when_prepared_then_it_is_rejected` — given session recorder input without audio stem origin when prepared then it is rejected (`src/recording/endpoint/tests.rs:262`; `test-497452363244c581f9e6`).
- `session_dir` — session dir (`src/recording/writer.rs:243`; `test-6d6ced88f99690c75bed`).
- `given_compiled_lineaged_edge_when_worker_runs_then_exact_session_stem_is_preserved` — given compiled lineaged edge when worker runs then exact session stem is preserved (`src/runtime/signal/operator.rs:2208`; `test-6615dcd3b3105010af0b`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1263`; `test-1682e00b3166c4846a92`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` — given derived stream when through called again then chain is preserved in session spec (`src/session/declaration/draft.rs:1179`; `test-837dd73be7d5c552ef15`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` — given operator when declared then session scoped instance and routes are preserved (`src/session/declaration/draft.rs:1317`; `test-69203660038a41959c14`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` — given unrouted stem when session frozen then validation fails closed (`src/session/declaration/draft.rs:1305`; `test-1633b6167eec91db04e2`).

## Failure signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` — `error-00e5716261eba0f8cf3d`
- `pocketstation::session::error::SessionError` / `UnknownStem` — `error-00f6e798d158df66c847`
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` — `error-01d3fc855e2a00319076`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` — `error-023d6ab0b23a50a614ff`
- `pocketstation::endpoint::runtime::EndpointFailureStage` / `CancelPreparation` — `error-0265bb447764629fa47b`
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` — `error-0279b2b6b0cb3b5801bc`
- `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` / `ZeroLeaseCapacity` — `error-0370b7ecbdf2b9d6fbdb`
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` — `error-037ddc3e193da74177f8`
- `pocketstation::recording::error_code::RecordingErrorCode` / `PermissionDenied` — `error-059bf10da1dcb4446e68`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` — `error-05c60389efcb84311921`
- `pocketstation::session::prepare::error::SessionPrepareError` — `error-085082b521c14e5ecd1e`
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` — `error-08a7536094bfb2242b17`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/running.rs:1-2612` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

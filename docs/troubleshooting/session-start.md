# Session fails before start

<!-- claims: CLM-TRBL-001-SCOPE-001,CLM-TRBL-001-TEXT-001,CLM-TRBL-001-TEXT-002,CLM-TRBL-001-TEXT-003,CLM-TRBL-001-TEXT-004,CLM-TRBL-001-TEXT-005,CLM-TRBL-001-TEXT-006,CLM-TRBL-001-SOURCE-001 -->

## Symptom

`Session::start` never returns a running owner.

## Evidenced causes

- The declaration cannot compile because a required route, binding, or compatible port is absent.
- A source, operator, bridge, endpoint, or trace resource fails during preparation.
- Start is cancelled or a prepared component fails its start gate; rollback can add further failures.

## Distinguish the causes

Compare the returned stage with declaration, compile, prepare, and start error types. Then inspect component identity and any rollback-failure count.

## Diagnostic signals

- `pocketstation::session::compile::error::SessionCompileError` (`error-a8b427bb67646e07d161`)
- `pocketstation::session::compile::error::SessionCompileError` / `AmbiguousEndpointInput` (`error-de8dedb8048857de2b1a`)
- `pocketstation::session::compile::error::SessionCompileError` / `AmbiguousOperatorPort` (`error-54e630e85aad971e5bab`)
- `pocketstation::session::compile::error::SessionCompileError` / `AudioBridgeOutputNotExclusive` (`error-455e5bda7c2d08e2f47d`)
- `pocketstation::session::compile::error::SessionCompileError` / `DuplicateOperatorInputConnection` (`error-8d82b1ede97797f60e26`)
- `pocketstation::session::compile::error::SessionCompileError` / `GraphCompile` (`error-b7ede33a45bca9ecd350`)
- `pocketstation::session::compile::error::SessionCompileError` / `InvalidAudioBridgeOutput` (`error-05658b1653627f20c5c4`)
- `pocketstation::session::compile::error::SessionCompileError` / `InvalidExternalSourceConfiguration` (`error-1be7c159405620caebf8`)
- `pocketstation::session::compile::error::SessionCompileError` / `InvalidSpec` (`error-c495841904e86732a0cd`)
- `pocketstation::session::compile::error::SessionCompileError` / `MissingRequiredOperatorInput` (`error-5903fce05dd27adde84a`)
- `pocketstation::session::compile::error::SessionCompileError` / `OperatorNodeTypeMismatch` (`error-56bd6d234e0432a776e9`)
- `pocketstation::session::compile::error::SessionCompileError` / `RuntimePlan` (`error-fdbe70184156fb1366ae`)

## Executable evidence

- `given_graph_mismatch_when_start_fails_then_diagnostic_is_retained` exercises given graph mismatch when start fails then diagnostic is retained under its recorded setup (`test-604c0e001a7dcb5f87ae`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` exercises given unrouted stem when session frozen then validation fails closed under its recorded setup (`test-8e301580cdd23a244478`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` exercises given foreign input handle when connected then declaration fails before freeze under its recorded setup (`test-0098e9bf5859cd4840f9`).
- `given_16khz_session_when_started_then_compiled_endpoint_contexts_preserve_declared_rate` exercises given 16khz session when started then compiled endpoint contexts preserve declared rate under its recorded setup (`test-b7bd22defab20a579dd6`).
- `given_capture_backlog_when_session_starts_then_no_destination_edge_overflows` exercises given capture backlog when session starts then no destination edge overflows under its recorded setup (`test-6d6c3692eac547e8fa36`).
- `given_existing_output_when_started_then_recorder_fails_closed` exercises given existing output when started then recorder fails closed under its recorded setup (`test-dbf6e802afe21cf4b197`).
- `given_zero_capacity_when_started_then_recorder_fails_closed` exercises given zero capacity when started then recorder fails closed under its recorded setup (`test-0ad7a1a182e71b0727e9`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` exercises given stopped public session when new session starts then capture restarts cleanly under its recorded setup (`test-7d6c18ed486400271167`).
- `given_linear_graph_when_compiled_then_topo_orders_source_before_sink` exercises given linear graph when compiled then topo orders source before sink under its recorded setup (`test-544fd9e4c3485f21700f`).
- `given_echo_async_node_when_process_before_prepare_then_error_is_returned` exercises given echo async node when process before prepare then error is returned under its recorded setup (`test-60c6c7ddba591b2f3fbf`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` exercises given receive before enqueue when observed then latency sample is rejected under its recorded setup (`test-0f111134a8c8ddb61a69`).
- `given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed` exercises given core extension oversized sidecar payload when encoded then fails closed under its recorded setup (`test-6c04502719d9fc0cdb98`).
- `given_capacity_above_global_bound_when_fanout_built_then_setup_fails` exercises given capacity above global bound when fanout built then setup fails under its recorded setup (`test-c8da19b53530dc618e8e`).
- `given_missing_or_zero_payload_limit_when_fanout_built_then_setup_fails` exercises given missing or zero payload limit when fanout built then setup fails under its recorded setup (`test-d9d74fd1b5a23d91b838`).
- `given_payload_above_branch_limit_when_published_then_all_branches_reject_before_fanout` exercises given payload above branch limit when published then all branches reject before fanout under its recorded setup (`test-26e5374e5c20a32afac4`).

## Corrective action

Correct the exact declaration or resource precondition at the failing stage. Preserve rollback failures alongside the primary error.

## Retry and incomplete state

Retry only from a newly valid declaration or newly prepared resource state. No frames should be assumed delivered before a running owner is returned.

## Related reference

- [Build Prepare Start](/docs/lifecycle/build-prepare-start.md)
- [Session](/docs/errors/session.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Session API](/docs/reference/session.md)
- [Session failures](/docs/errors/session.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)

## Evidence boundary

The claims on **Session fails before start** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/compile/error.rs:1-1` (`DECLARED`)
- `src/session/prepare/error.rs:1-1` (`DECLARED`)

For **Session fails before start**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

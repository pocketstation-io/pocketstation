# Session fails before start

<!-- claims: CLM-TRBL-001-CAP-001,CLM-TRBL-001-CAP-002,CLM-TRBL-001-CAP-003,CLM-TRBL-001-CAP-004,CLM-TRBL-001-CAP-005,CLM-TRBL-001-SOURCE-001 -->

## Symptom

`Session::start` never returns a running owner.

## Evidenced causes

- The declaration cannot compile because a required route, binding, or compatible port is absent.
- A source, operator, bridge, endpoint, or trace resource fails during preparation.
- Start is cancelled or a prepared component fails its start gate; rollback can add further failures.

## Distinguish the causes

Compare the returned stage with declaration, compile, prepare, and start error types. Then inspect component identity and any rollback-failure count.

## Diagnostic signals

- `pocketstation::session::compile::error::SessionCompileError` (`error-9e087654a58d57db0600`)
- `pocketstation::session::compile::error::SessionCompileError` / `AmbiguousEndpointInput` (`error-eaadb6203402e4c6cd3a`)
- `pocketstation::session::compile::error::SessionCompileError` / `AmbiguousOperatorPort` (`error-aad4749f884d05381ab7`)
- `pocketstation::session::compile::error::SessionCompileError` / `AudioBridgeOutputNotExclusive` (`error-756808af75b7a76405bd`)
- `pocketstation::session::compile::error::SessionCompileError` / `DuplicateOperatorInputConnection` (`error-967401df89c3468e7bd0`)
- `pocketstation::session::compile::error::SessionCompileError` / `GraphCompile` (`error-f828fbbe93ec476f38c5`)
- `pocketstation::session::compile::error::SessionCompileError` / `InvalidAudioBridgeOutput` (`error-1e1c426aed779be65563`)
- `pocketstation::session::compile::error::SessionCompileError` / `InvalidExternalSourceConfiguration` (`error-f39e1d5ca300f380beb9`)
- `pocketstation::session::compile::error::SessionCompileError` / `InvalidSpec` (`error-029e54f07e027450b5ea`)
- `pocketstation::session::compile::error::SessionCompileError` / `MissingRequiredOperatorInput` (`error-88aaa28af18ebdcb8225`)
- `pocketstation::session::compile::error::SessionCompileError` / `OperatorNodeTypeMismatch` (`error-c43073e442de4510a95f`)
- `pocketstation::session::compile::error::SessionCompileError` / `RuntimePlan` (`error-b1fed564ca50e53202f1`)

## Executable evidence

- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` exercises given unrouted stem when session frozen then validation fails closed under its recorded setup (`test-1633b6167eec91db04e2`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` exercises given foreign input handle when connected then declaration fails before freeze under its recorded setup (`test-766194f5939b3ddb896d`).
- `given_16khz_session_when_started_then_compiled_endpoint_contexts_preserve_declared_rate` exercises given 16khz session when started then compiled endpoint contexts preserve declared rate under its recorded setup (`test-036ad7eaba4f64ee56a4`).
- `given_capture_backlog_when_session_starts_then_no_destination_edge_overflows` exercises given capture backlog when session starts then no destination edge overflows under its recorded setup (`test-3c7b5a91848359ab3ef8`).
- `given_existing_output_when_started_then_recorder_fails_closed` exercises given existing output when started then recorder fails closed under its recorded setup (`test-3b4ddbb231cb8f0182e8`).
- `given_zero_capacity_when_started_then_recorder_fails_closed` exercises given zero capacity when started then recorder fails closed under its recorded setup (`test-fcc195708afeec03a930`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` exercises given stopped public session when new session starts then capture restarts cleanly under its recorded setup (`test-6a5d630191363f7e4442`).
- `given_linear_graph_when_compiled_then_topo_orders_source_before_sink` exercises given linear graph when compiled then topo orders source before sink under its recorded setup (`test-7ece727a2fa318f311df`).
- `given_echo_async_node_when_process_before_prepare_then_error_is_returned` exercises given echo async node when process before prepare then error is returned under its recorded setup (`test-bfea57e87a139988d3b9`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` exercises given receive before enqueue when observed then latency sample is rejected under its recorded setup (`test-904f86f2ad1e1369647c`).
- `given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed` exercises given core extension oversized sidecar payload when encoded then fails closed under its recorded setup (`test-f94228781a9717656566`).
- `given_capacity_above_global_bound_when_fanout_built_then_setup_fails` exercises given capacity above global bound when fanout built then setup fails under its recorded setup (`test-f0893eafe636572bd65e`).
- `given_missing_or_zero_payload_limit_when_fanout_built_then_setup_fails` exercises given missing or zero payload limit when fanout built then setup fails under its recorded setup (`test-6c3a749ca1eaac051f1f`).
- `given_payload_above_branch_limit_when_published_then_all_branches_reject_before_fanout` exercises given payload above branch limit when published then all branches reject before fanout under its recorded setup (`test-5f2124d7ed0e92c73799`).
- `given_payload_limit_above_global_bound_when_fanout_built_then_setup_fails` exercises given payload limit above global bound when fanout built then setup fails under its recorded setup (`test-dd6a20d29e9e95a48939`).

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

The claims on **Session fails before start** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/compile/error.rs:1-92` (`DIRECT`)
- `src/session/prepare/error.rs:1-95` (`DIRECT`)

For **Session fails before start**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

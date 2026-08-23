# A sidecar misses a deadline

<!-- claims: CLM-TRBL-010-CAP-001,CLM-TRBL-010-CAP-002,CLM-TRBL-010-CAP-003,CLM-TRBL-010-CAP-004,CLM-TRBL-010-SOURCE-001 -->

## Symptom

A sidecar fails to start, exchange a message, drain, or stop before its deadline.

## Evidenced causes

- The child command cannot start or exits early.
- A message kind, framing rule, or byte limit is invalid.
- Startup, request, drain, or abort exceeds its configured deadline.
- The host or child closes while work remains.

## Distinguish the causes

Inspect sidecar state, process error, last accepted message kind, protocol limit, and the exact deadline stage.

## Diagnostic signals

- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` (`error-4a7bbf78f1eef4f31cda`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `AlreadyReaped` (`error-e3a2e354214fc48a985a`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Closed` (`error-edccf83596e248e6faba`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `ControlQueueFull` (`error-bdb3331ea4ef7fe66d05`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `DataQueueFull` (`error-bc92e00331c1093a3a5f`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `FrameTooLarge` (`error-d2843fac19f48d7e718e`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `InvalidConfiguration` (`error-7f2632443ff0a1b229d7`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `InvalidDataKind` (`error-a4c5bffd5a950224b28e`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `InvalidState` (`error-964a79d02e303c7ceefd`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Io` (`error-464e0427ba81044140fb`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Kill` (`error-b5e61d9a83637a8b0a55`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `MissingPipe` (`error-44c57f1bf9e040992e47`)

## Executable evidence

- `given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned` exercises given empty input group when sidecar prepares then structured error is returned under its recorded setup (`test-49bd18fb96d67fdba9bf`).
- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` exercises given sidecar host errors when classified then retryability is preserved under its recorded setup (`test-98ad8a10ce6f978fe856`).
- `given_slow_future_when_deadline_expires_then_host_returns_typed_timeout` exercises given slow future when deadline expires then host returns typed timeout under its recorded setup (`test-9e709c123ea73a9a1332`).
- `given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed` exercises given core extension oversized sidecar payload when encoded then fails closed under its recorded setup (`test-f94228781a9717656566`).
- `given_core_extension_sidecar_message_when_round_tripped_then_identity_is_stable` exercises given core extension sidecar message when round tripped then identity is stable under its recorded setup (`test-647c9ae6972c9c36f722`).
- `given_instance_deadline_when_worker_runs_then_configured_timeout_is_authoritative` exercises given instance deadline when worker runs then configured timeout is authoritative under its recorded setup (`test-53f77875f4c688209091`).
- `given_slow_operator_when_deadline_expires_then_timeout_cancel_and_join_are_observed` exercises given slow operator when deadline expires then timeout cancel and join are observed under its recorded setup (`test-1bec17e5820e3c9ada80`).
- `given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal` exercises given connector never ready when startup deadline expires then failure is terminal under its recorded setup (`test-c926ac54e34e42c44877`).
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` exercises given provider owned field name when resolved then core preserves it opaquely under its recorded setup (`test-d9078fd01d0271720b30`).
- `given_audio_record_when_round_tripped_then_transport_and_lineage_identity_are_preserved` exercises given audio record when round tripped then transport and lineage identity are preserved under its recorded setup (`test-907dfffa894311945559`).
- `given_invalid_audio_record_when_decoded_then_trailing_and_oversized_payloads_are_rejected` exercises given invalid audio record when decoded then trailing and oversized payloads are rejected under its recorded setup (`test-3f130e9d460c8fbeeeee`).
- `given_invalid_configuration_record_when_decoded_then_unknown_kinds_and_trailing_bytes_are_rejected` exercises given invalid configuration record when decoded then unknown kinds and trailing bytes are rejected under its recorded setup (`test-943083fa631d93d1b735`).
- `given_typed_configuration_when_round_tripped_then_types_and_secret_redaction_are_preserved` exercises given typed configuration when round tripped then types and secret redaction are preserved under its recorded setup (`test-214a9dfd247b433b190b`).
- `given_drain_then_abort_when_requested_then_shutdown_intent_upgrades_monotonically` exercises given drain then abort when requested then shutdown intent upgrades monotonically under its recorded setup (`test-50a5f1631531f3816b13`).
- `given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output` exercises given connected gain plan when executed then only connected nodes run and worker receives output under its recorded setup (`test-cd64bb966db1f193ea6f`).

## Corrective action

Correct the process or protocol contract. Choose drain only when pending work may complete; choose abort when application policy accepts discarding it.

## Retry and incomplete state

Restart is not automatically safe because external work may have occurred before timeout. Treat acknowledgements and output as partial until terminal state confirms them.

## Related reference

- [Sidecars](/docs/concepts/sidecars.md)
- [Sidecar Protocol](/docs/reference/sidecar-protocol.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [Honor connector retryability](/docs/best-practices/connector-retries.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)
- [Host a managed-process sidecar](/docs/how-to/host-sidecar.md)

## Evidence boundary

The claims on **A sidecar misses a deadline** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/lifecycle/sidecar_host.rs:1-734` (`DIRECT`)

For **A sidecar misses a deadline**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

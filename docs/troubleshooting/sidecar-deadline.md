# A sidecar misses a deadline

<!-- claims: CLM-TRBL-010-SCOPE-001,CLM-TRBL-010-TEXT-001,CLM-TRBL-010-TEXT-002,CLM-TRBL-010-TEXT-003,CLM-TRBL-010-TEXT-004,CLM-TRBL-010-TEXT-005,CLM-TRBL-010-TEXT-006,CLM-TRBL-010-SOURCE-001 -->

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

- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` (`error-216966e028c93292ad0e`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `AlreadyReaped` (`error-c2bafd8527c7d490c2ad`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Closed` (`error-bf9b3507356148f9eff7`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `ControlQueueFull` (`error-a6aa6d1e23c8870d9145`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `DataQueueFull` (`error-9e4d5596e287983ab54d`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `FrameTooLarge` (`error-e4b7949af0ed720f36ab`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `InvalidConfiguration` (`error-8e7ab792d7358cda4dff`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `InvalidDataKind` (`error-292e4b4d52b5beaf0eef`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `InvalidState` (`error-0bce996f947679936756`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Io` (`error-6fe24a493d557602e0e7`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Kill` (`error-0ae879a99c0152bb21b1`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `MissingPipe` (`error-927c8454c7a40a1c31af`)

## Executable evidence

- `given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned` exercises given empty input group when sidecar prepares then structured error is returned under its recorded setup (`test-a819c552a02a127c977d`).
- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` exercises given sidecar host errors when classified then retryability is preserved under its recorded setup (`test-72a5c76707ff849957fa`).
- `given_slow_future_when_deadline_expires_then_host_returns_typed_timeout` exercises given slow future when deadline expires then host returns typed timeout under its recorded setup (`test-61cb0441698a1984dac6`).
- `given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed` exercises given core extension oversized sidecar payload when encoded then fails closed under its recorded setup (`test-6c04502719d9fc0cdb98`).
- `given_core_extension_sidecar_message_when_round_tripped_then_identity_is_stable` exercises given core extension sidecar message when round tripped then identity is stable under its recorded setup (`test-0d42eb96d05d1140bbc3`).
- `given_instance_deadline_when_worker_runs_then_configured_timeout_is_authoritative` exercises given instance deadline when worker runs then configured timeout is authoritative under its recorded setup (`test-afd27ecbd8c5e4b793e8`).
- `given_slow_operator_when_deadline_expires_then_timeout_cancel_and_join_are_observed` exercises given slow operator when deadline expires then timeout cancel and join are observed under its recorded setup (`test-d23b7905d713383361c0`).
- `given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal` exercises given connector never ready when startup deadline expires then failure is terminal under its recorded setup (`test-6b9e356534c04d2e2c3a`).
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` exercises given provider owned field name when resolved then core preserves it opaquely under its recorded setup (`test-c7a1a4edccbfbf6d9c04`).
- `given_audio_record_when_round_tripped_then_transport_and_lineage_identity_are_preserved` exercises given audio record when round tripped then transport and lineage identity are preserved under its recorded setup (`test-415f6fa4b5693884f093`).
- `given_invalid_audio_record_when_decoded_then_trailing_and_oversized_payloads_are_rejected` exercises given invalid audio record when decoded then trailing and oversized payloads are rejected under its recorded setup (`test-ea1a0a780bd315ebcbb6`).
- `given_invalid_configuration_record_when_decoded_then_unknown_kinds_and_trailing_bytes_are_rejected` exercises given invalid configuration record when decoded then unknown kinds and trailing bytes are rejected under its recorded setup (`test-61989b4b80b2e98a9e25`).
- `given_typed_configuration_when_round_tripped_then_types_and_secret_redaction_are_preserved` exercises given typed configuration when round tripped then types and secret redaction are preserved under its recorded setup (`test-615c91bbd90e08b449bb`).
- `given_drain_then_abort_when_requested_then_shutdown_intent_upgrades_monotonically` exercises given drain then abort when requested then shutdown intent upgrades monotonically under its recorded setup (`test-01257fc4936a2d7e629a`).
- `given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output` exercises given connected gain plan when executed then only connected nodes run and worker receives output under its recorded setup (`test-3f9281677e5af26dc9ad`).

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

The claims on **A sidecar misses a deadline** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/runtime/lifecycle/sidecar_host.rs:13-13` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:14-14` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:15-15` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:16-16` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:17-17` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:19-19` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:19-19` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:19-19` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:21-33` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:22-22` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:23-23` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:24-24` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:25-25` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:26-26` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:27-27` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:28-28` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:29-29` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:30-30` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:31-31` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:32-32` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:36-50` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:53-53` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:53-53` (`DIRECT`)
- `src/runtime/lifecycle/sidecar_host.rs:53-53` (`DIRECT`)

For **A sidecar misses a deadline**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

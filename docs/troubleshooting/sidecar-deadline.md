# A sidecar misses a deadline

<!-- claims: CLM-TRBL-010-CAP-001,CLM-TRBL-010-CAP-002,CLM-TRBL-010-CAP-003,CLM-TRBL-010-CAP-004,CLM-TRBL-010-SOURCE-001 -->

Use this page when you observe **a sidecar misses a deadline**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Inspect sidecar state, message kind, protocol limit, and the deadline that expired before choosing drain, abort, or restart.

## Diagnostic signals

- `pocketstation::connector::error::ConnectorErrorCodeError` / `TooLong` (`error-06f5c52aa07c86ca5062`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `InvalidSampleCount` (`error-093c41e2489cf1bb258d`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` (`error-0b1f3a3357a77fcef185`)
- `pocketstation::connector::error::ConnectorErrorCodeError` / `Empty` (`error-0b71c9f1b1489e0d4f9a`)
- `pocketstation::connector::error::ConnectorErrorBuildError` (`error-0bc8adb0641971704f74`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `TooManyFields` (`error-0c83ebde568152ad3edf`)
- `pocketstation::connector::error::ConnectorErrorStage` / `Startup` (`error-0e62627edef059ecab22`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `InvalidManifestRevision` (`error-10517744910e14c23fc4`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `UnsupportedMinor` (`error-1082687e9dbfd2cadfc5`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `InvalidMagic` (`error-143cce14f0e71f68c4cf`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `InvalidValue` (`error-16fe034657303e4973f8`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `Wait` (`error-19eabd878a9188bf94ce`)
- `pocketstation::connector::ConnectorDeclarationError` (`error-1cafe789f84ff34b7955`)
- `pocketstation::connector::error::ConnectorErrorCodeError` (`error-1d9267787b6c574f3c02`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `ReservedFieldSet` (`error-1d9b879cab06d8598907`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `InvalidTerminal` (`error-201cc7749bdbbd671d69`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `ValueTooLarge` (`error-20e58c6bbc3ac729a8e8`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `FrameLengthOverflow` (`error-23eba8b87dea81473095`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `Truncated` (`error-287ed1c38c6ad2b533ef`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `TooManyManifestEntries` (`error-29230b3395a2c8d86df6`)
- `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` / `UnknownMessageKind` (`error-2b03bbb58bb17d9482da`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `WrongType` (`error-2f295a051ff6d0366ead`)
- `pocketstation::connector::error::ConnectorErrorStage` / `Join` (`error-326d10a69e8bf7fdb781`)
- `pocketstation::runtime::audio::executor::ExecError` / `Node` (`error-3636f110b3c505b0fc87`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `UnknownField` (`error-37775f819a84416494a5`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `LengthOverflow` (`error-3a219a96959e38e2b4d8`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `ProcessingTimeout` (`error-3a3e737bfe0585596712`)
- `pocketstation::connector::ConnectorObservationLookupError` (`error-3a9f4ee91af2cf43237b`)
- `pocketstation::connector::ConnectorRegistrationError` (`error-3d8ad8972e7742e4f68e`)
- `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` / `UnexpectedMessage` (`error-3dbf0292e22bf7695a5b`)

## Executable evidence

- `given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned` exercises given empty input group when sidecar prepares then structured error is returned under its recorded setup (`test-49bd18fb96d67fdba9bf`).
- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` exercises given sidecar host errors when classified then retryability is preserved under its recorded setup (`test-98ad8a10ce6f978fe856`).
- `given_slow_future_when_deadline_expires_then_host_returns_typed_timeout` exercises given slow future when deadline expires then host returns typed timeout under its recorded setup (`test-9e709c123ea73a9a1332`).
- `given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed` exercises given core extension oversized sidecar payload when encoded then fails closed under its recorded setup (`test-f94228781a9717656566`).
- `given_core_extension_sidecar_message_when_round_tripped_then_identity_is_stable` exercises given core extension sidecar message when round tripped then identity is stable under its recorded setup (`test-647c9ae6972c9c36f722`).
- `given_instance_deadline_when_worker_runs_then_configured_timeout_is_authoritative` exercises given instance deadline when worker runs then configured timeout is authoritative under its recorded setup (`test-53f77875f4c688209091`).
- `given_slow_operator_when_deadline_expires_then_timeout_cancel_and_join_are_observed` exercises given slow operator when deadline expires then timeout cancel and join are observed under its recorded setup (`test-1bec17e5820e3c9ada80`).
- `given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal` exercises given connector never ready when startup deadline expires then failure is terminal under its recorded setup (`test-5cad5d93f0205d9f9891`).
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` exercises given provider owned field name when resolved then core preserves it opaquely under its recorded setup (`test-d9078fd01d0271720b30`).
- `given_audio_record_when_round_tripped_then_transport_and_lineage_identity_are_preserved` exercises given audio record when round tripped then transport and lineage identity are preserved under its recorded setup (`test-907dfffa894311945559`).
- `given_invalid_audio_record_when_decoded_then_trailing_and_oversized_payloads_are_rejected` exercises given invalid audio record when decoded then trailing and oversized payloads are rejected under its recorded setup (`test-3f130e9d460c8fbeeeee`).
- `given_invalid_configuration_record_when_decoded_then_unknown_kinds_and_trailing_bytes_are_rejected` exercises given invalid configuration record when decoded then unknown kinds and trailing bytes are rejected under its recorded setup (`test-943083fa631d93d1b735`).
- `given_typed_configuration_when_round_tripped_then_types_and_secret_redaction_are_preserved` exercises given typed configuration when round tripped then types and secret redaction are preserved under its recorded setup (`test-214a9dfd247b433b190b`).
- `given_drain_then_abort_when_requested_then_shutdown_intent_upgrades_monotonically` exercises given drain then abort when requested then shutdown intent upgrades monotonically under its recorded setup (`test-50a5f1631531f3816b13`).
- `given_connected_gain_plan_when_executed_then_only_connected_nodes_run_and_worker_receives_output` exercises given connected gain plan when executed then only connected nodes run and worker receives output under its recorded setup (`test-cd64bb966db1f193ea6f`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)
- [Host a managed-process sidecar](/docs/how-to/host-sidecar.md)
- [Configuration reference](/docs/reference/configuration.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/lifecycle/sidecar_host.rs:1-734` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

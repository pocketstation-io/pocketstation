# A connector is not ready

<!-- claims: CLM-TRBL-008-CAP-001,CLM-TRBL-008-CAP-002,CLM-TRBL-008-CAP-003,CLM-TRBL-008-CAP-004,CLM-TRBL-008-CAP-005,CLM-TRBL-008-SOURCE-001 -->

## Symptom

A connector never reaches readiness or reports a readiness-stage failure.

## Evidenced causes

- Manifest or resolved configuration validation failed.
- The readiness deadline or threshold is invalid or expires.
- Driver preparation or startup failed.
- The provider boundary reports a typed failure before delivery.

## Distinguish the causes

Read connector stage, error code, retryability, readiness snapshot, probe counts, and configuration context without exposing secrets.

## Diagnostic signals

- `pocketstation::connector::readiness::ConnectorReadinessPolicyError` (`error-626bee576556acb470d9`)
- `pocketstation::connector::readiness::ConnectorReadinessPolicyError` / `InvalidDeadline` (`error-6355361a7150f503c29b`)
- `pocketstation::connector::readiness::ConnectorReadinessPolicyError` / `InvalidThreshold` (`error-a4a7c7b9c48716e5720d`)
- `pocketstation::connector::ConnectorDeclarationError` (`error-f52aaa0ab3a57f824f0f`)
- `pocketstation::connector::ConnectorDeclarationError` / `Configuration` (`error-79dde5b66dc8ba41f246`)
- `pocketstation::connector::ConnectorDeclarationError` / `Session` (`error-036c74e429b61ebdf7cd`)
- `pocketstation::connector::ConnectorDeclarationError` / `WrongSession` (`error-29964fc5e23bb4431977`)
- `pocketstation::connector::ConnectorObservationLookupError` (`error-e6fe04d84e66539434f3`)
- `pocketstation::connector::ConnectorObservationLookupError` / `WrongSession` (`error-ee73c462c5fc59286e13`)
- `pocketstation::connector::ConnectorRegistrationError` (`error-673fdb2e6e5753423e37`)
- `pocketstation::connector::ConnectorRegistrationError` / `InvalidManifest` (`error-29c46c16c1ddf5628f60`)
- `pocketstation::connector::ConnectorRegistrationError` / `Session` (`error-5183d1e7630f1e611820`)

## Executable evidence

- `given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal` exercises given connector never ready when startup deadline expires then failure is terminal under its recorded setup (`test-c926ac54e34e42c44877`).
- `given_already_open_start_gate_when_endpoint_start_requested_then_start_fails_recoverably` exercises given already open start gate when endpoint start requested then start fails recoverably under its recorded setup (`test-e8c1d06b58a459a61c14`).
- `given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain` exercises given connector driver when two stems run then core owns typed delivery and drain under its recorded setup (`test-c7deea8505c28b2f4d0d`).
- `given_connector_error_when_inspected_then_code_is_stable_and_machine_readable` exercises given connector error when inspected then code is stable and machine readable under its recorded setup (`test-ae5c25d0ba6141b1a13a`).
- `given_connector_public_surface_when_inspected_then_managed_aliases_are_absent` exercises given connector public surface when inspected then managed aliases are absent under its recorded setup (`test-e1ff05b0ec4b54a78b0b`).
- `given_duplicate_connector_identity_when_registered_then_registration_is_rejected` exercises given duplicate connector identity when registered then registration is rejected under its recorded setup (`test-5576ad13d627ff481e0b`).
- `given_grouped_connector_when_session_is_cancelled_then_abort_intent_reaches_worker` exercises given grouped connector when session is cancelled then abort intent reaches worker under its recorded setup (`test-834e8847ff5d70b04b11`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` exercises given grouped connector when session stops then one worker is joined and observed under its recorded setup (`test-92f5704ec6ee88e59fd8`).
- `given_prior_preparation_when_connector_prepare_fails_then_prior_work_rolls_back` exercises given prior preparation when connector prepare fails then prior work rolls back under its recorded setup (`test-6e2b2556c5e641fda848`).
- `given_registered_connector_when_declared_then_identity_is_session_scoped` exercises given registered connector when declared then identity is session scoped under its recorded setup (`test-da2fb847d5c7f22349e8`).
- `given_saturated_connector_route_when_observed_then_drops_are_visible_in_session_metrics` exercises given saturated connector route when observed then drops are visible in session metrics under its recorded setup (`test-2fa646ca802635256f43`).
- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` exercises given canonical connector vectors when compared then core contract semantics match under its recorded setup (`test-5ccbb97716e582e0a790`).
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` exercises given provider owned field name when resolved then core preserves it opaquely under its recorded setup (`test-d9078fd01d0271720b30`).
- `given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned` exercises given empty input group when sidecar prepares then structured error is returned under its recorded setup (`test-49bd18fb96d67fdba9bf`).
- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` exercises given sidecar host errors when classified then retryability is preserved under its recorded setup (`test-98ad8a10ce6f978fe856`).

## Corrective action

Correct validation failures or provider configuration, then recreate the connector. Follow `ConnectorRetryability`; do not invent a retry budget.

## Retry and incomplete state

Retry is allowed only by the typed retryability and application policy. No delivery acknowledgement should be assumed before readiness.

## Related reference

- [Connector Workers](/docs/concepts/connector-workers.md)
- [Connectors](/docs/reference/connectors.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Honor connector retryability](/docs/best-practices/connector-retries.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)
- [Connector failures](/docs/errors/connectors.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)

## Evidence boundary

The claims on **A connector is not ready** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/connector/readiness.rs:1-67` (`DIRECT`)
- `src/connector/status.rs:1-79` (`DIRECT`)

For **A connector is not ready**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

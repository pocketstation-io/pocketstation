# A connector is not ready

<!-- claims: CLM-TRBL-008-SCOPE-001,CLM-TRBL-008-TEXT-001,CLM-TRBL-008-TEXT-002,CLM-TRBL-008-TEXT-003,CLM-TRBL-008-TEXT-004,CLM-TRBL-008-TEXT-005,CLM-TRBL-008-TEXT-006,CLM-TRBL-008-SOURCE-001 -->

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

- `pocketstation::connector::readiness::ConnectorReadinessPolicyError` (`error-0753f57f39eeda193fd7`)
- `pocketstation::connector::readiness::ConnectorReadinessPolicyError` / `InvalidDeadline` (`error-8a53aa5c4aaf14173550`)
- `pocketstation::connector::readiness::ConnectorReadinessPolicyError` / `InvalidThreshold` (`error-fc4f340f9b56ed8dc516`)
- `pocketstation::connector::ConnectorDeclarationError` (`error-ef429da19499a30febec`)
- `pocketstation::connector::ConnectorDeclarationError` / `Configuration` (`error-f85c1437daae474702be`)
- `pocketstation::connector::ConnectorDeclarationError` / `Session` (`error-3d1f996891195230a51b`)
- `pocketstation::connector::ConnectorDeclarationError` / `WrongSession` (`error-4fedfe91825f69358fac`)
- `pocketstation::connector::ConnectorObservationLookupError` (`error-a53691df2301981b8217`)
- `pocketstation::connector::ConnectorObservationLookupError` / `WrongSession` (`error-e60ba01d1498b3b6afcc`)
- `pocketstation::connector::ConnectorRegistrationError` (`error-9d4e9574523d027696fc`)
- `pocketstation::connector::ConnectorRegistrationError` / `InvalidManifest` (`error-2007e97b628ca3cf2622`)
- `pocketstation::connector::ConnectorRegistrationError` / `Session` (`error-15e79a7790bf8f8fd860`)

## Executable evidence

- `given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal` exercises given connector never ready when startup deadline expires then failure is terminal under its recorded setup (`test-6b9e356534c04d2e2c3a`).
- `given_already_open_start_gate_when_endpoint_start_requested_then_start_fails_recoverably` exercises given already open start gate when endpoint start requested then start fails recoverably under its recorded setup (`test-70431fbc7f2633c86453`).
- `given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain` exercises given connector driver when two stems run then core owns typed delivery and drain under its recorded setup (`test-0226f46b368cc7dec827`).
- `given_connector_error_when_inspected_then_code_is_stable_and_machine_readable` exercises given connector error when inspected then code is stable and machine readable under its recorded setup (`test-15d28406ed0aae973558`).
- `given_connector_public_surface_when_inspected_then_managed_aliases_are_absent` exercises given connector public surface when inspected then managed aliases are absent under its recorded setup (`test-81c56797a2883f88930a`).
- `given_duplicate_connector_identity_when_registered_then_registration_is_rejected` exercises given duplicate connector identity when registered then registration is rejected under its recorded setup (`test-7eabe669b99c6e379ec6`).
- `given_grouped_connector_when_session_is_cancelled_then_abort_intent_reaches_worker` exercises given grouped connector when session is cancelled then abort intent reaches worker under its recorded setup (`test-30eb64a515907397f19f`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` exercises given grouped connector when session stops then one worker is joined and observed under its recorded setup (`test-2a1b6ff7d4015d418fc1`).
- `given_prior_preparation_when_connector_prepare_fails_then_prior_work_rolls_back` exercises given prior preparation when connector prepare fails then prior work rolls back under its recorded setup (`test-125f099fa90218b83809`).
- `given_registered_connector_when_declared_then_identity_is_session_scoped` exercises given registered connector when declared then identity is session scoped under its recorded setup (`test-b500d2adf0e2ce5b5229`).
- `given_saturated_connector_route_when_observed_then_drops_are_visible_in_session_metrics` exercises given saturated connector route when observed then drops are visible in session metrics under its recorded setup (`test-440e0d0f038bd27e531f`).
- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` exercises given canonical connector vectors when compared then core contract semantics match under its recorded setup (`test-2df56b27d49e3e92a1f8`).
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` exercises given provider owned field name when resolved then core preserves it opaquely under its recorded setup (`test-c7a1a4edccbfbf6d9c04`).
- `given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned` exercises given empty input group when sidecar prepares then structured error is returned under its recorded setup (`test-a819c552a02a127c977d`).
- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` exercises given sidecar host errors when classified then retryability is preserved under its recorded setup (`test-72a5c76707ff849957fa`).

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

The claims on **A connector is not ready** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/connector/readiness.rs:3-3` (`DIRECT`)
- `src/connector/readiness.rs:4-4` (`DIRECT`)
- `src/connector/readiness.rs:6-6` (`DIRECT`)
- `src/connector/readiness.rs:6-6` (`DIRECT`)
- `src/connector/readiness.rs:6-6` (`DIRECT`)
- `src/connector/readiness.rs:7-12` (`DIRECT`)
- `src/connector/readiness.rs:8-8` (`DIRECT`)
- `src/connector/readiness.rs:9-9` (`DIRECT`)
- `src/connector/readiness.rs:10-10` (`DIRECT`)
- `src/connector/readiness.rs:11-11` (`DIRECT`)
- `src/connector/readiness.rs:15-41` (`DIRECT`)
- `src/connector/readiness.rs:43-45` (`DIRECT`)
- `src/connector/readiness.rs:47-49` (`DIRECT`)
- `src/connector/readiness.rs:51-53` (`DIRECT`)
- `src/connector/readiness.rs:55-57` (`DIRECT`)
- `src/connector/readiness.rs:60-60` (`DIRECT`)
- `src/connector/readiness.rs:60-60` (`DIRECT`)
- `src/connector/readiness.rs:60-60` (`DIRECT`)
- `src/connector/readiness.rs:60-60` (`DIRECT`)
- `src/connector/readiness.rs:61-66` (`DIRECT`)
- `src/connector/readiness.rs:63-63` (`DIRECT`)
- `src/connector/readiness.rs:65-65` (`DIRECT`)
- `src/connector/status.rs:2-2` (`DIRECT`)
- `src/connector/status.rs:2-2` (`DIRECT`)
- `src/connector/status.rs:2-2` (`DIRECT`)
- `src/connector/status.rs:4-7` (`DIRECT`)
- `src/connector/status.rs:5-5` (`DIRECT`)
- `src/connector/status.rs:6-6` (`DIRECT`)
- `src/connector/status.rs:10-12` (`DIRECT`)
- `src/connector/status.rs:15-15` (`DIRECT`)
- `src/connector/status.rs:15-15` (`DIRECT`)
- `src/connector/status.rs:15-15` (`DIRECT`)
- `src/connector/status.rs:17-20` (`DIRECT`)
- `src/connector/status.rs:18-18` (`DIRECT`)
- `src/connector/status.rs:19-19` (`DIRECT`)
- `src/connector/status.rs:22-22` (`DIRECT`)
- `src/connector/status.rs:22-22` (`DIRECT`)
- `src/connector/status.rs:22-22` (`DIRECT`)
- `src/connector/status.rs:24-27` (`DIRECT`)
- `src/connector/status.rs:25-25` (`DIRECT`)
- `src/connector/status.rs:26-26` (`DIRECT`)
- `src/connector/status.rs:29-29` (`DIRECT`)
- `src/connector/status.rs:29-29` (`DIRECT`)
- `src/connector/status.rs:29-29` (`DIRECT`)
- `src/connector/status.rs:30-39` (`DIRECT`)
- `src/connector/status.rs:31-31` (`DIRECT`)

For **A connector is not ready**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

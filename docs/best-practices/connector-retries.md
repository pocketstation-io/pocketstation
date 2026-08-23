# Honor connector retryability

<!-- claims: CLM-BEST-005-CAP-001,CLM-BEST-005-CAP-002,CLM-BEST-005-CAP-003,CLM-BEST-005-CAP-004,CLM-BEST-005-SOURCE-001 -->

## Problem

Blindly retrying a connector failure can duplicate provider work or repeat a condition that requires reconfiguration.

## Recommendation

Branch on `ConnectorRetryability`: never retry `Never`, require configuration change for `RetryAfterReconfiguration`, and let application policy decide when to repeat `Retryable` work.

## Reason

Retryability is a typed failure property, while observations record attempts. The repository declares no `RetryBudget` type or automatic finite-attempt policy.

## Tradeoff

Application code must own timing, attempt limits, idempotency, and provider-specific consequences.

## When it does not apply

Do not apply connector retryability to another endpoint or sidecar error unless that boundary exposes the same explicit contract.

## Repository evidence

This recommendation is tied directly to the page's source evidence.

## Executable evidence

Executable evidence selected for **Honor connector retryability** is limited to each test's recorded setup and assertions:

- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` — given sidecar host errors when classified then retryability is preserved (`src/connector/sidecar.rs:286`; `test-98ad8a10ce6f978fe856`).
- `given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain` — given connector driver when two stems run then core owns typed delivery and drain (`tests/connector_contract.rs:580`; `test-c7deea8505c28b2f4d0d`).
- `given_connector_error_when_inspected_then_code_is_stable_and_machine_readable` — given connector error when inspected then code is stable and machine readable (`tests/connector_contract.rs:243`; `test-ae5c25d0ba6141b1a13a`).
- `given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal` — given connector never ready when startup deadline expires then failure is terminal (`tests/connector_contract.rs:751`; `test-c926ac54e34e42c44877`).
- `given_connector_public_surface_when_inspected_then_managed_aliases_are_absent` — given connector public surface when inspected then managed aliases are absent (`tests/connector_contract.rs:26`; `test-e1ff05b0ec4b54a78b0b`).
- `given_duplicate_connector_identity_when_registered_then_registration_is_rejected` — given duplicate connector identity when registered then registration is rejected (`tests/connector_contract.rs:229`; `test-5576ad13d627ff481e0b`).
- `given_grouped_connector_when_session_is_cancelled_then_abort_intent_reaches_worker` — given grouped connector when session is cancelled then abort intent reaches worker (`tests/connector_contract.rs:709`; `test-834e8847ff5d70b04b11`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` — given grouped connector when session stops then one worker is joined and observed (`tests/connector_contract.rs:677`; `test-92f5704ec6ee88e59fd8`).
- `given_prior_preparation_when_connector_prepare_fails_then_prior_work_rolls_back` — given prior preparation when connector prepare fails then prior work rolls back (`tests/connector_contract.rs:638`; `test-6e2b2556c5e641fda848`).
- `given_registered_connector_when_declared_then_identity_is_session_scoped` — given registered connector when declared then identity is session scoped (`tests/connector_contract.rs:203`; `test-da2fb847d5c7f22349e8`).
- `given_saturated_connector_route_when_observed_then_drops_are_visible_in_session_metrics` — given saturated connector route when observed then drops are visible in session metrics (`tests/connector_contract.rs:836`; `test-2fa646ca802635256f43`).
- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` — given canonical connector vectors when compared then core contract semantics match (`tests/connector_portable_semantics.rs:167`; `test-5ccbb97716e582e0a790`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)
- [Connector failures](/docs/errors/connectors.md)
- [Author a connector](/docs/guides/connectors.md)

## Evidence boundary

The claims on **Honor connector retryability** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/connector/error.rs:1-190` (`DIRECT`)
- `src/connector/observations.rs:1-261` (`DIRECT`)

For **Honor connector retryability**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

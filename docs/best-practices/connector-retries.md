# Use finite connector retry budgets

<!-- claims: CLM-BEST-005-CAP-001,CLM-BEST-005-CAP-002,CLM-BEST-005-CAP-003,CLM-BEST-005-SOURCE-001 -->

## Recommendation

Declare finite attempts and time in connector retry policy, and expose readiness and exhaustion through observations.

## Why

The repository makes capacity, ownership, identity, lifecycle, and evidence boundaries explicit so failures remain attributable. Bypassing them removes observations and typed outcomes needed for diagnosis.

## Tradeoff

The recommendation requires explicit configuration and result handling. It does not promise that one capacity, retry budget, selector, or shutdown policy fits every workload. Measure within the API's stated scope.

## When it does not apply

Do not apply a realtime, connector, capture, or extension rule to another lane or boundary unless it exposes the same contract. An internal pattern is not automatically a public recommendation.

## Repository evidence

- `sidecar_isolation` at `src/connector/mod.rs` (`pattern-00438ff8d2146688eeaf`).
- `typed_error` at `src/connector/worker/endpoint_adapter.rs` (`pattern-0c264e4ec468e7568a9c`).
- `sidecar_isolation` at `src/connector/transport.rs` (`pattern-139a2b492c98807b410f`).
- `sidecar_isolation` at `src/connector/status.rs` (`pattern-2204e930be728d4ccf21`).
- `typed_error` at `src/connector/transport.rs` (`pattern-2ee2fce6c23e17a0e11e`).
- `transactional_registration` at `src/connector/mod.rs` (`pattern-31b76706228fd84bfc03`).
- `sidecar_isolation` at `src/connector/sidecar.rs` (`pattern-3d974f8a195e8a91eecd`).
- `typed_error` at `tests/connector_contract.rs` (`pattern-4795dd3768d5a3e46d6b`).
- `sidecar_isolation` at `src/connector/worker/coordination.rs` (`pattern-62aec1768a5d595e60b2`).
- `typed_error` at `examples/connector_authoring.rs` (`pattern-6e31e7a838ec8f9bc6b1`).
- `bounded_queue` at `src/connector/transport.rs` (`pattern-7b68019126c4c1540ebc`).
- `typed_error` at `tests/connector_grouping_public_surface.rs` (`pattern-88d3fc2b07df818eff6e`).
- `typed_error` at `src/connector/mod.rs` (`pattern-92384c9a4e85cd5fa74e`).
- `sidecar_isolation` at `src/connector/error.rs` (`pattern-9ea9251c9fbe65c4e367`).
- `sidecar_isolation` at `src/connector/worker/driver.rs` (`pattern-a6a76a8e4974d5910060`).
- `sidecar_isolation` at `tests/connector_portable_semantics.rs` (`pattern-a8e8e29068c207358578`).
- `typed_error` at `src/connector/sidecar.rs` (`pattern-aa4acf0858d935d02808`).
- `sidecar_isolation` at `examples/connector_authoring.rs` (`pattern-b2fbadf2d6a330c083c5`).
- `sidecar_isolation` at `tests/connector_grouping_public_surface.rs` (`pattern-ba010980ee41cec78f8d`).
- `sidecar_isolation` at `src/connector/readiness.rs` (`pattern-bdf45d1b2dace5a77960`).

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_sidecar_host_errors_when_classified_then_retryability_is_preserved` — given sidecar host errors when classified then retryability is preserved (`src/connector/sidecar.rs:286`; `test-98ad8a10ce6f978fe856`).
- `given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain` — given connector driver when two stems run then core owns typed delivery and drain (`tests/connector_contract.rs:582`; `test-eefa0d157754becdb1a2`).
- `given_connector_error_when_inspected_then_code_is_stable_and_machine_readable` — given connector error when inspected then code is stable and machine readable (`tests/connector_contract.rs:243`; `test-ae5c25d0ba6141b1a13a`).
- `given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal` — given connector never ready when startup deadline expires then failure is terminal (`tests/connector_contract.rs:753`; `test-5cad5d93f0205d9f9891`).
- `given_connector_public_surface_when_inspected_then_managed_aliases_are_absent` — given connector public surface when inspected then managed aliases are absent (`tests/connector_contract.rs:26`; `test-e1ff05b0ec4b54a78b0b`).
- `given_duplicate_connector_identity_when_registered_then_registration_is_rejected` — given duplicate connector identity when registered then registration is rejected (`tests/connector_contract.rs:229`; `test-5576ad13d627ff481e0b`).
- `given_grouped_connector_when_session_is_cancelled_then_abort_intent_reaches_worker` — given grouped connector when session is cancelled then abort intent reaches worker (`tests/connector_contract.rs:711`; `test-2e6e7299000cfcf26fc4`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` — given grouped connector when session stops then one worker is joined and observed (`tests/connector_contract.rs:679`; `test-aa2345c7b9339f742b48`).
- `given_prior_preparation_when_connector_prepare_fails_then_prior_work_rolls_back` — given prior preparation when connector prepare fails then prior work rolls back (`tests/connector_contract.rs:640`; `test-867a92c422e2fe7fbb4d`).
- `given_registered_connector_when_declared_then_identity_is_session_scoped` — given registered connector when declared then identity is session scoped (`tests/connector_contract.rs:203`; `test-da2fb847d5c7f22349e8`).
- `given_saturated_connector_route_when_observed_then_drops_are_visible_in_session_metrics` — given saturated connector route when observed then drops are visible in session metrics (`tests/connector_contract.rs:838`; `test-2a3db251e3203d28d4cf`).
- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` — given canonical connector vectors when compared then core contract semantics match (`tests/connector_portable_semantics.rs:167`; `test-5ccbb97716e582e0a790`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [Author a connector](/docs/guides/connectors.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Connector API](/docs/reference/connectors.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/connector/manifest.rs:1-255` (`DIRECT`)
- `src/connector/worker/coordination.rs:1-231` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

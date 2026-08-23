# Test connector conformance

<!-- claims: CLM-GUIDE-017-CAP-001,CLM-GUIDE-017-CAP-002,CLM-GUIDE-017-SOURCE-001 -->

## Scope

- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

The scope of **Test connector conformance** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

The exact versioned connector vector at the sibling path required by the portable-semantics test.

## Procedure

1. Obtain the versioned connector vector file required by portable semantics.
2. Place it at the sibling path expected by the test workflow.
3. Run connector contract and grouping tests.
4. Run portable semantics with the external vector present.
5. Keep local conformance and provider qualification as separate evidence.

## Important consequence

An absent external vector is a prerequisite failure, never passing connector evidence.

## Verify the outcome

Local contract tests and portable vector assertions pass, with fixture revision and execution scope recorded.

Executable evidence selected for **Test connector conformance** is limited to each test's recorded setup and assertions:

- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` — given canonical connector vectors when compared then core contract semantics match (`tests/connector_portable_semantics.rs:167`; `test-5ccbb97716e582e0a790`).
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
- `given_connector_authoring_layer_when_scanned_then_it_does_not_duplicate_core_runtime_policy` — given connector authoring layer when scanned then it does not duplicate core runtime policy (`tests/public_api_boundary.rs:98`; `test-bcf303db85cca8b13bc2`).

## Failure signals

- `pocketstation::connector::ConnectorDeclarationError` — `error-f52aaa0ab3a57f824f0f`
- `pocketstation::connector::ConnectorDeclarationError` / `Configuration` — `error-79dde5b66dc8ba41f246`
- `pocketstation::connector::ConnectorDeclarationError` / `Session` — `error-036c74e429b61ebdf7cd`
- `pocketstation::connector::ConnectorDeclarationError` / `WrongSession` — `error-29964fc5e23bb4431977`
- `pocketstation::connector::ConnectorObservationLookupError` — `error-e6fe04d84e66539434f3`
- `pocketstation::connector::ConnectorObservationLookupError` / `WrongSession` — `error-ee73c462c5fc59286e13`
- `pocketstation::connector::ConnectorRegistrationError` — `error-673fdb2e6e5753423e37`
- `pocketstation::connector::ConnectorRegistrationError` / `InvalidManifest` — `error-29c46c16c1ddf5628f60`
- `pocketstation::connector::ConnectorRegistrationError` / `Session` — `error-5183d1e7630f1e611820`
- `pocketstation::connector::configuration::ConnectorConfigurationError` — `error-67d99d7e897c27488847`

## API reference

- [Conformance](/docs/concepts/conformance.md)
- [Conformance Fixtures](/docs/troubleshooting/conformance-fixtures.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::connector::worker::driver::ConnectorDriver` | trait | Provider-specific behavior executed on Core's bounded connector worker. | `src/connector/worker/driver.rs:92` |
| `pocketstation::connector::worker::driver::ConnectorDriverFactory` | trait | Prepares provider state while Core retains receiver and lifecycle authority. | `src/connector/worker/driver.rs:123` |
| `pocketstation::connector::error::ConnectorError` | struct | Reports a connector error. | `src/connector/error.rs:80` |
| `pocketstation::connector::error::ConnectorErrorCode` | struct | Carries the stable external error code exported for a connector failure. | `src/connector/error.rs:10` |
| `pocketstation::connector::observations::ConnectorObservationHandle` | struct | Owns bounded access to connector observation. | `src/connector/observations.rs:15` |
| `pocketstation::connector::observations::ConnectorObservations` | struct | Reports the connector observations collected at an observation boundary. | `src/connector/observations.rs:158` |
| `pocketstation::connector::observations::ConnectorRuntimeObservations` | struct | Reports the connector runtime observations collected at an observation boundary. | `src/connector/observations.rs:168` |
| `pocketstation::connector::readiness::ConnectorReadinessPolicy` | struct | Configures connector readiness behavior at its owning API boundary. | `src/connector/readiness.rs:7` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Author a connector](/docs/guides/connectors.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [Conformance and qualification](/docs/concepts/conformance.md)

## Evidence boundary

The claims on **Test connector conformance** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/connector_portable_semantics.rs:1-210` (`DIRECT`)

For **Test connector conformance**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

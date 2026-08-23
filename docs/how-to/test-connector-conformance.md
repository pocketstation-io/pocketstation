# Test connector conformance

<!-- claims: CLM-GUIDE-017-CAP-001,CLM-GUIDE-017-CAP-002,CLM-GUIDE-017-SOURCE-001 -->

## Scope

- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

The scope of **Test connector conformance** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

The repository-owned connector vector corpus and permission to materialize its versioned sibling path.

## Procedure

1. Create ../protocol/conformance/connector/v1 from the repository root.
2. Copy scripts/fixtures/connector-v1-vectors.json to ../protocol/conformance/connector/v1/vectors.json.
3. Run connector contract and grouping tests.
4. Run portable semantics with the materialized canonical vector.
5. Keep portable conformance and provider qualification as separate evidence.

## Important consequence

A missing or drifted materialized copy is a prerequisite failure, never passing connector evidence.

## Verify the outcome

Local contract tests and portable vector assertions pass, with fixture revision and execution scope recorded.

Executable evidence selected for **Test connector conformance** is limited to each test's recorded setup and assertions:

- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` — given canonical connector vectors when compared then core contract semantics match (`tests/connector_portable_semantics.rs:167`; `test-2df56b27d49e3e92a1f8`).
- `given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain` — given connector driver when two stems run then core owns typed delivery and drain (`tests/connector_contract.rs:580`; `test-0226f46b368cc7dec827`).
- `given_connector_error_when_inspected_then_code_is_stable_and_machine_readable` — given connector error when inspected then code is stable and machine readable (`tests/connector_contract.rs:243`; `test-15d28406ed0aae973558`).
- `given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal` — given connector never ready when startup deadline expires then failure is terminal (`tests/connector_contract.rs:751`; `test-6b9e356534c04d2e2c3a`).
- `given_connector_public_surface_when_inspected_then_managed_aliases_are_absent` — given connector public surface when inspected then managed aliases are absent (`tests/connector_contract.rs:26`; `test-81c56797a2883f88930a`).
- `given_duplicate_connector_identity_when_registered_then_registration_is_rejected` — given duplicate connector identity when registered then registration is rejected (`tests/connector_contract.rs:229`; `test-7eabe669b99c6e379ec6`).
- `given_grouped_connector_when_session_is_cancelled_then_abort_intent_reaches_worker` — given grouped connector when session is cancelled then abort intent reaches worker (`tests/connector_contract.rs:709`; `test-30eb64a515907397f19f`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` — given grouped connector when session stops then one worker is joined and observed (`tests/connector_contract.rs:677`; `test-2a1b6ff7d4015d418fc1`).
- `given_prior_preparation_when_connector_prepare_fails_then_prior_work_rolls_back` — given prior preparation when connector prepare fails then prior work rolls back (`tests/connector_contract.rs:638`; `test-125f099fa90218b83809`).
- `given_registered_connector_when_declared_then_identity_is_session_scoped` — given registered connector when declared then identity is session scoped (`tests/connector_contract.rs:203`; `test-b500d2adf0e2ce5b5229`).
- `given_saturated_connector_route_when_observed_then_drops_are_visible_in_session_metrics` — given saturated connector route when observed then drops are visible in session metrics (`tests/connector_contract.rs:836`; `test-440e0d0f038bd27e531f`).
- `given_connector_authoring_layer_when_scanned_then_it_does_not_duplicate_core_runtime_policy` — given connector authoring layer when scanned then it does not duplicate core runtime policy (`tests/public_api_boundary.rs:110`; `test-7012bfbc5fb5bdda9c3d`).

## Failure signals

- `pocketstation::connector::ConnectorDeclarationError` — `error-ef429da19499a30febec`
- `pocketstation::connector::ConnectorDeclarationError` / `Configuration` — `error-f85c1437daae474702be`
- `pocketstation::connector::ConnectorDeclarationError` / `Session` — `error-3d1f996891195230a51b`
- `pocketstation::connector::ConnectorDeclarationError` / `WrongSession` — `error-4fedfe91825f69358fac`
- `pocketstation::connector::ConnectorObservationLookupError` — `error-a53691df2301981b8217`
- `pocketstation::connector::ConnectorObservationLookupError` / `WrongSession` — `error-e60ba01d1498b3b6afcc`
- `pocketstation::connector::ConnectorRegistrationError` — `error-9d4e9574523d027696fc`
- `pocketstation::connector::ConnectorRegistrationError` / `InvalidManifest` — `error-2007e97b628ca3cf2622`
- `pocketstation::connector::ConnectorRegistrationError` / `Session` — `error-15e79a7790bf8f8fd860`
- `pocketstation::connector::configuration::ConnectorConfigurationError` — `error-7586a644dfbc54e958bb`

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

The claims on **Test connector conformance** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `tests/connector_portable_semantics.rs:1-210` (`DIRECT`)

For **Test connector conformance**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

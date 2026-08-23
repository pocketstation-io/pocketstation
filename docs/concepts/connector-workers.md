# Connector worker lifecycle

<!-- claims: CLM-DOC-026-CAP-001,CLM-DOC-026-SOURCE-001 -->

## What it is

A connector worker owns delivery, acknowledgement, readiness observation, cancellation, drain or abort, and the connector's terminal outcome.

## Why it exists

Provider delivery can fail independently from Session capture and other endpoints. Worker ownership keeps that failure stage and its observations local to the connector.

## Relationships

- The manifest and resolved configuration are preparation inputs.
- The endpoint lifecycle starts and stops the connector driver.
- `ConnectorRetryability` classifies failure policy while observations count retry attempts; no finite retry budget API is declared.

## Invariants and guarantees

- Delivery outcomes are finite and typed.
- Readiness, delivery, retryability, and shutdown stages remain distinguishable.
- A connector failure does not erase other component and recording outcomes.

## When you encounter it

- **Author a connector** — Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.
- **Host an out-of-process worker** — Spawn a sidecar and enforce bounded messages, deadlines, cancellation, and terminal state.

## Use it

- [Author a connector](/docs/guides/connectors.md)
- [Honor connector retryability](/docs/best-practices/connector-retries.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)

## Scope

- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.

The scope of **Connector worker lifecycle** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::connector::worker::driver::ConnectorDriver` | trait | Provider-specific behavior executed on Core's bounded connector worker. | `src/connector/worker/driver.rs:92` |
| `pocketstation::connector::worker::driver::ConnectorDriverFactory` | trait | Prepares provider state while Core retains receiver and lifecycle authority. | `src/connector/worker/driver.rs:123` |
| `pocketstation::connector::worker::driver::ConnectorInputDescriptor` | struct | Immutable Session and graph metadata for one connector input. | `src/connector/worker/driver.rs:16` |
| `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome` | enum | Explicit delivery result used for Core-owned accounting. | `src/connector/worker/driver.rs:83` |
| `pocketstation::connector::worker::driver::ConnectorItem` | enum | One bounded item delivered by Core to a connector driver. | `src/connector/worker/driver.rs:62` |
| `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome::Delivered` | variant | Identifies the delivered state or stage represented by `ConnectorDeliveryOutcome`. | `src/connector/worker/driver.rs:84` |
| `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome::Dropped` | variant | Identifies the dropped state or stage represented by `ConnectorDeliveryOutcome`. | `src/connector/worker/driver.rs:85` |
| `pocketstation::connector::worker::driver::ConnectorItem::Audio` | variant | Represents the audio alternative defined by `ConnectorItem`. | `src/connector/worker/driver.rs:63` |
| `pocketstation::connector::worker::driver::ConnectorItem::Signal` | variant | Represents the signal alternative defined by `ConnectorItem`. | `src/connector/worker/driver.rs:67` |
| `pocketstation::connector::error::ConnectorError` | struct | Reports a connector error. | `src/connector/error.rs:80` |

## Executable evidence

Executable evidence selected for **Connector worker lifecycle** is limited to each test's recorded setup and assertions:

- `given_grouped_connector_when_session_is_cancelled_then_abort_intent_reaches_worker` — given grouped connector when session is cancelled then abort intent reaches worker (`tests/connector_contract.rs:709`; `test-834e8847ff5d70b04b11`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` — given grouped connector when session stops then one worker is joined and observed (`tests/connector_contract.rs:677`; `test-92f5704ec6ee88e59fd8`).
- `given_connector_driver_when_two_stems_run_then_core_owns_typed_delivery_and_drain` — given connector driver when two stems run then core owns typed delivery and drain (`tests/connector_contract.rs:580`; `test-c7deea8505c28b2f4d0d`).
- `given_connector_error_when_inspected_then_code_is_stable_and_machine_readable` — given connector error when inspected then code is stable and machine readable (`tests/connector_contract.rs:243`; `test-ae5c25d0ba6141b1a13a`).
- `given_connector_never_ready_when_startup_deadline_expires_then_failure_is_terminal` — given connector never ready when startup deadline expires then failure is terminal (`tests/connector_contract.rs:751`; `test-c926ac54e34e42c44877`).
- `given_connector_public_surface_when_inspected_then_managed_aliases_are_absent` — given connector public surface when inspected then managed aliases are absent (`tests/connector_contract.rs:26`; `test-e1ff05b0ec4b54a78b0b`).
- `given_duplicate_connector_identity_when_registered_then_registration_is_rejected` — given duplicate connector identity when registered then registration is rejected (`tests/connector_contract.rs:229`; `test-5576ad13d627ff481e0b`).
- `given_prior_preparation_when_connector_prepare_fails_then_prior_work_rolls_back` — given prior preparation when connector prepare fails then prior work rolls back (`tests/connector_contract.rs:638`; `test-6e2b2556c5e641fda848`).
- `given_registered_connector_when_declared_then_identity_is_session_scoped` — given registered connector when declared then identity is session scoped (`tests/connector_contract.rs:203`; `test-da2fb847d5c7f22349e8`).
- `given_saturated_connector_route_when_observed_then_drops_are_visible_in_session_metrics` — given saturated connector route when observed then drops are visible in session metrics (`tests/connector_contract.rs:836`; `test-2fa646ca802635256f43`).
- `given_worker_failure_or_panic_when_session_stops_then_endpoint_finalization_is_terminal` — given worker failure or panic when session stops then endpoint finalization is terminal (`tests/connector_contract.rs:803`; `test-40ed6b8bb8e60b0fd01a`).
- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` — given canonical connector vectors when compared then core contract semantics match (`tests/connector_portable_semantics.rs:167`; `test-5ccbb97716e582e0a790`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Author a connector](/docs/guides/connectors.md)
- [Host a managed-process sidecar](/docs/how-to/host-sidecar.md)
- [Test connector conformance](/docs/how-to/test-connector-conformance.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)

## Evidence boundary

The claims on **Connector worker lifecycle** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/connector/worker/supervisor.rs:1-150` (`DIRECT`)

For **Connector worker lifecycle**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

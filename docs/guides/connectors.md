# Author a connector

<!-- claims: CLM-GUIDE-015-CAP-001,CLM-GUIDE-015-CAP-002,CLM-GUIDE-015-CAP-003,CLM-GUIDE-015-SOURCE-001 -->

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

The scope of **Author a connector** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A stable connector identity, port and configuration schema, secret ownership plan, and finite delivery outcomes.

## Procedure

1. Build ConnectorManifest with node and configuration schemas.
2. Validate values and keep secrets in ConnectorSecret.
3. Implement finite delivery outcomes.
4. Register, declare, and connect the endpoint.
5. Run conformance before provider qualification.

## Important consequence

Conformance is not live-provider qualification, and the Core API does not declare a retry budget.

## Verify the outcome

The connector registers, validates configuration, reaches readiness, consumes its declared item, and finalizes under conformance.

Executable evidence selected for **Author a connector** is limited to each test's recorded setup and assertions:

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
- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` — given canonical connector vectors when compared then core contract semantics match (`tests/connector_portable_semantics.rs:167`; `test-2df56b27d49e3e92a1f8`).
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` — given provider owned field name when resolved then core preserves it opaquely (`src/connector/configuration.rs:642`; `test-c7a1a4edccbfbf6d9c04`).

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

- [Connectors](/docs/concepts/connectors.md)
- [Connectors](/docs/reference/connectors.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::connector::worker::driver::ConnectorDriver` | trait | Provider-specific behavior executed on Core's bounded connector worker. | `src/connector/worker/driver.rs:92` |
| `pocketstation::connector::worker::driver::ConnectorDriverFactory` | trait | Prepares provider state while Core retains receiver and lifecycle authority. | `src/connector/worker/driver.rs:123` |
| `pocketstation::connector::configuration::ConnectorConfiguration` | struct | Configures connector behavior at its owning API boundary. | `src/connector/configuration.rs:111` |
| `pocketstation::connector::configuration::ConnectorConfigurationError` | struct | Reports a connector configuration error. | `src/connector/configuration.rs:608` |
| `pocketstation::connector::configuration::ConnectorConfigurationField` | struct | Declares one typed connector configuration field and its validation constraints. | `src/connector/configuration.rs:168` |
| `pocketstation::connector::configuration::ConnectorConfigurationSchema` | struct | Validates connector configuration values against the manifest-declared field set. | `src/connector/configuration.rs:232` |
| `pocketstation::connector::configuration::ConnectorSecret` | struct | Owns a connector secret with redacted diagnostics and byte clearing on explicit reset or drop. | `src/connector/configuration.rs:11` |
| `pocketstation::connector::configuration::ResolvedConnectorConfiguration` | struct | Configures resolved connector behavior at its owning API boundary. | `src/connector/configuration.rs:391` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Test connector conformance](/docs/how-to/test-connector-conformance.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Connector API](/docs/reference/connectors.md)

## Evidence boundary

The claims on **Author a connector** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `examples/connector_authoring.rs:1-167` (`DIRECT`)

For **Author a connector**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

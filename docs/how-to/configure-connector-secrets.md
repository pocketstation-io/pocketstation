# Configure connector secrets

<!-- claims: CLM-GUIDE-016-CAP-001,CLM-GUIDE-016-CAP-002,CLM-GUIDE-016-SOURCE-001 -->

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Configure connector secrets** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A manifest field for every accepted value and a decision about which fields contain secret material.

## Procedure

1. Declare every connector configuration field.
2. Use Secret value kind for secret material.
3. Construct ConnectorSecret instead of ordinary text values.
4. Read validated values during preparation.
5. Keep diagnostics on redacted representations.

## Important consequence

Redacted debug output does not prove that every upstream or downstream copy has been erased.

## Verify the outcome

Configuration resolution returns typed values; diagnostics do not reveal the initialized secret representation.

Executable evidence selected for **Configure connector secrets** is limited to each test's recorded setup and assertions:

- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` — given provider owned field name when resolved then core preserves it opaquely (`src/connector/configuration.rs:642`; `test-d9078fd01d0271720b30`).
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
- `given_typed_schema_when_configuration_resolves_then_defaults_apply_and_secrets_redact` — given typed schema when configuration resolves then defaults apply and secrets redact (`tests/connector_contract.rs:168`; `test-20c3cf08187aa1f02fcd`).

## Failure signals

- `pocketstation::connector::configuration::ConnectorConfigurationError` — `error-67d99d7e897c27488847`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` — `error-1f98ce662465497fe9e9`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `ConstraintViolation` — `error-2ce73a91158c4d9b0fdf`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `DuplicateField` — `error-2d0a2d9b54e5680c9285`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `EmptySecret` — `error-69b8c5299a7f611d79f7`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `InvalidSchema` — `error-648c2ca7e569679d349f`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `InvalidValue` — `error-cccaf86d5e8df8dea29d`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `MissingRequiredField` — `error-29933d2716fdeefdaa36`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `SecretClassificationMismatch` — `error-aa018059beaf334fd398`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `SecretDefaultForbidden` — `error-3ea1e2f9b2e4697c2e8a`

## API reference

- [Connectors](/docs/concepts/connectors.md)
- [Boundaries](/docs/security/boundaries.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::connector::configuration::ConnectorConfiguration` | struct | Configures connector behavior at its owning API boundary. | `src/connector/configuration.rs:111` |
| `pocketstation::connector::configuration::ConnectorConfigurationError` | struct | Reports a connector configuration error. | `src/connector/configuration.rs:608` |
| `pocketstation::connector::configuration::ConnectorConfigurationField` | struct | Declares one typed connector configuration field and its validation constraints. | `src/connector/configuration.rs:168` |
| `pocketstation::connector::configuration::ConnectorConfigurationSchema` | struct | Validates connector configuration values against the manifest-declared field set. | `src/connector/configuration.rs:232` |
| `pocketstation::connector::configuration::ConnectorSecret` | struct | Owns a connector secret with redacted diagnostics and byte clearing on explicit reset or drop. | `src/connector/configuration.rs:11` |
| `pocketstation::connector::configuration::ResolvedConnectorConfiguration` | struct | Configures resolved connector behavior at its owning API boundary. | `src/connector/configuration.rs:391` |
| `pocketstation::connector::manifest::ConnectorCapability` | struct | Declares a capability advertised by a connector manifest. | `src/connector/manifest.rs:12` |
| `pocketstation::connector::manifest::ConnectorManifest` | struct | Describes the connector manifest contract. | `src/connector/manifest.rs:75` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [Connector failures](/docs/errors/connectors.md)
- [Honor connector retryability](/docs/best-practices/connector-retries.md)
- [Connector model](/docs/concepts/connectors.md)

## Evidence boundary

The claims on **Configure connector secrets** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/connector/configuration.rs:1-673` (`DIRECT`)
- `src/secret.rs:1-13` (`DIRECT`)

For **Configure connector secrets**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

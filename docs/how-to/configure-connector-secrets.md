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

- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` — given provider owned field name when resolved then core preserves it opaquely (`src/connector/configuration.rs:642`; `test-c7a1a4edccbfbf6d9c04`).
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
- `given_typed_schema_when_configuration_resolves_then_defaults_apply_and_secrets_redact` — given typed schema when configuration resolves then defaults apply and secrets redact (`tests/connector_contract.rs:168`; `test-fad64112037847056015`).

## Failure signals

- `pocketstation::connector::configuration::ConnectorConfigurationError` — `error-7586a644dfbc54e958bb`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` — `error-eb394a41ca14c7c3d902`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `ConstraintViolation` — `error-f991bab1bb0e62a4ddcc`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `DuplicateField` — `error-35280bb059c691469ba4`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `EmptySecret` — `error-b54444b24a83b3d62005`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `InvalidSchema` — `error-2822d51f1f5fa9c5e76a`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `InvalidValue` — `error-4052e2a14ed737e1b05f`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `MissingRequiredField` — `error-b42167d41fffa8689ba2`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `SecretClassificationMismatch` — `error-530db2fc3674010dc252`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `SecretDefaultForbidden` — `error-355239d59ffed6225cfe`

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

The claims on **Configure connector secrets** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/connector/configuration.rs:1-673` (`DIRECT`)
- `src/secret.rs:1-13` (`DIRECT`)

For **Configure connector secrets**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

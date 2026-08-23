# Connector model

<!-- claims: CLM-DOC-025-CAP-001,CLM-DOC-025-SOURCE-001 -->

## What it is

A connector manifest describes a provider endpoint's identity, node and port contracts, configuration schema, secret fields, delivery requirements, and readiness behavior without adding provider protocol details to Core.

## Why it exists

Provider integrations need a stable typed boundary while credentials and network protocols remain connector-owned. The manifest is the declaration and validation authority for that boundary.

## Relationships

- Connector configuration resolves typed and secret values against the manifest.
- A connector driver runs behind the endpoint lifecycle.
- Conformance checks portable contract behavior separately from live provider qualification.

## Invariants and guarantees

- Connector and node identities must be valid and unique.
- Secret values use `ConnectorSecret` and redact their debug representation.
- Delivery and readiness state do not imply a retry budget that the API does not declare.

## When you encounter it

- **Author a connector** — Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.

## Use it

- [Author a connector](/docs/guides/connectors.md)
- [Configure connector secrets](/docs/how-to/configure-connector-secrets.md)
- [Connector reference](/docs/reference/connectors.md)

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.

The scope of **Connector model** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

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
| `pocketstation::connector::manifest::ConnectorRequirement` | struct | Declares a host or configuration requirement that must be satisfied before connector use. | `src/connector/manifest.rs:40` |
| `pocketstation::connector::configuration::ConnectorConfigurationConstraint` | enum | Enumerates the supported connector configuration constraint cases. | `src/connector/configuration.rs:159` |

## Executable evidence

Executable evidence selected for **Connector model** is limited to each test's recorded setup and assertions:

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
- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` — given canonical connector vectors when compared then core contract semantics match (`tests/connector_portable_semantics.rs:167`; `test-2df56b27d49e3e92a1f8`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Author a connector](/docs/guides/connectors.md)
- [Configure connector secrets](/docs/how-to/configure-connector-secrets.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Connector API](/docs/reference/connectors.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)

## Evidence boundary

The claims on **Connector model** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/connector/manifest.rs:1-255` (`DIRECT`)
- `src/connector/configuration.rs:1-673` (`DIRECT`)

For **Connector model**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

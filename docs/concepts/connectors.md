# Connector model

<!-- claims: CLM-DOC-025-CAP-001,CLM-DOC-025-SOURCE-001 -->

Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::connector::worker::driver::ConnectorDriver` | trait | Provider-specific behavior executed on Core's bounded connector worker. | `src/connector/worker/driver.rs:92` |
| `pocketstation::connector::worker::driver::ConnectorDriverFactory` | trait | Prepares provider state while Core retains receiver and lifecycle authority. | `src/connector/worker/driver.rs:123` |
| `pocketstation::connector::sidecar::SidecarConnectorDriverFactory` | struct | Adapts a bounded PocketStation sidecar process to the Connector driver SPI. | `src/connector/sidecar.rs:24` |
| `pocketstation::connector::transport::ConnectorConfigurationRecord` | struct | Canonical typed configuration handed to a connector sidecar during its bounded Configure handshake. Secret classification survives the boundary; Debug output continues to redact secret values. | `src/connector/transport.rs:42` |
| `pocketstation::connector::worker::driver::ConnectorInputDescriptor` | struct | Immutable Session and graph metadata for one connector input. | `src/connector/worker/driver.rs:16` |
| `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome` | enum | Explicit delivery result used for Core-owned accounting. | `src/connector/worker/driver.rs:83` |
| `pocketstation::connector::worker::driver::ConnectorItem` | enum | One bounded item delivered by Core to a connector driver. | `src/connector/worker/driver.rs:62` |
| `pocketstation::connector` | module | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/mod.rs:1` |
| `pocketstation::connector::worker::ConnectorFactory` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/worker/mod.rs:17` |
| `pocketstation::connector::worker::ConnectorWorker` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/worker/mod.rs:32` |
| `pocketstation::connector::Connector` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/mod.rs:61` |
| `pocketstation::connector::RegisteredConnector` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/mod.rs:125` |
| `pocketstation::connector::configuration::ConnectorConfiguration` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/configuration.rs:111` |
| `pocketstation::connector::configuration::ConnectorConfigurationError` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/configuration.rs:608` |
| `pocketstation::connector::configuration::ConnectorConfigurationField` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/configuration.rs:168` |
| `pocketstation::connector::configuration::ConnectorConfigurationSchema` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/configuration.rs:232` |
| `pocketstation::connector::configuration::ConnectorSecret` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/configuration.rs:11` |
| `pocketstation::connector::configuration::ResolvedConnectorConfiguration` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/configuration.rs:391` |
| `pocketstation::connector::error::ConnectorError` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/error.rs:80` |
| `pocketstation::connector::error::ConnectorErrorCode` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/error.rs:10` |

## Where you encounter it

- **Author a connector** — Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

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
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` — given provider owned field name when resolved then core preserves it opaquely (`src/connector/configuration.rs:642`; `test-d9078fd01d0271720b30`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/connector/manifest.rs:1-255` (`DIRECT`)
- `src/connector/configuration.rs:1-673` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

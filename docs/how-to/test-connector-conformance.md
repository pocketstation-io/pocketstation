# Test connector conformance

<!-- claims: CLM-GUIDE-017-CAP-001,CLM-GUIDE-017-CAP-002,CLM-GUIDE-017-SOURCE-001 -->

## Scope

- **Run connector workers.** Supervise connector delivery, acknowledgement, retry budgets, readiness, cancellation, drain, and abort.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Obtain the versioned connector vector file required by portable semantics.
2. Place it at the sibling path expected by the test workflow.
3. Run connector contract and grouping tests.
4. Run portable semantics with the external vector present.
5. Keep local conformance and provider qualification as separate evidence.

## APIs used

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

## Verify the outcome

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
- `given_connector_authoring_layer_when_scanned_then_it_does_not_duplicate_core_runtime_policy` — given connector authoring layer when scanned then it does not duplicate core runtime policy (`tests/public_api_boundary.rs:98`; `test-bcf303db85cca8b13bc2`).

## Failure signals

- `pocketstation::connector::error::ConnectorErrorCodeError` / `TooLong` — `error-06f5c52aa07c86ca5062`
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `InvalidSampleCount` — `error-093c41e2489cf1bb258d`
- `pocketstation::connector::transport::ConnectorAudioRecordError` — `error-0b1f3a3357a77fcef185`
- `pocketstation::connector::error::ConnectorErrorCodeError` / `Empty` — `error-0b71c9f1b1489e0d4f9a`
- `pocketstation::connector::error::ConnectorErrorBuildError` — `error-0bc8adb0641971704f74`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `TooManyFields` — `error-0c83ebde568152ad3edf`
- `pocketstation::connector::error::ConnectorErrorStage` / `Startup` — `error-0e62627edef059ecab22`
- `pocketstation::connector::manifest::ConnectorManifestError` / `InvalidManifestRevision` — `error-10517744910e14c23fc4`
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `UnsupportedMinor` — `error-1082687e9dbfd2cadfc5`
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `InvalidValue` — `error-16fe034657303e4973f8`
- `pocketstation::connector::ConnectorDeclarationError` — `error-1cafe789f84ff34b7955`
- `pocketstation::connector::error::ConnectorErrorCodeError` — `error-1d9267787b6c574f3c02`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/connector_portable_semantics.rs:1-210` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

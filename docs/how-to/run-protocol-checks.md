# Run protocol checks

<!-- claims: CLM-GUIDE-025-CAP-001,CLM-GUIDE-025-CAP-002,CLM-GUIDE-025-SOURCE-001 -->

## Scope

- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.
- **Build and publish repository artifacts.** Run architecture, protocol, package, platform, and release checks used by the repository publication workflow.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Run the repository protocol script.
2. Run C ABI and compatibility tests selected by CI.
3. Provide required sibling or private fixtures.
4. Distinguish absent prerequisites from assertion failures.
5. Record command, target, and fixture revision.

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
| `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| `sidecar` | function | Builds an outbound Connector backed by one bounded sidecar process. | `src/connector/mod.rs:112` |
| `with_driver` | function | Builds a connector whose bounded receiver loop is owned by Core. | `src/connector/mod.rs:88` |
| `pocketstation::connector` | module | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/mod.rs:1` |
| `pocketstation::connector::worker::ConnectorFactory` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/worker/mod.rs:17` |
| `pocketstation::connector::worker::ConnectorWorker` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/worker/mod.rs:32` |
| `pocketstation::abi::executable_extension::PksExtensionCallbacks` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:91` |
| `pocketstation::abi::executable_extension::PksExtensionLibrary` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:123` |
| `pocketstation::abi::executable_extension::PksExtensionPipelineDeclaration` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:168` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `test-publish-recovery` — test publish recovery (`scripts/test-publish-recovery.sh:1`; `test-adb1dc831caefa6dcbaa`).
- `test-session-c-conformance` — test session c conformance (`scripts/test-session-c-conformance.sh:1`; `test-ff28fea505d22371ce41`).
- `test-single-package-publish` — test single package publish (`scripts/test-single-package-publish.sh:1`; `test-2cb6c90c68d9362041c6`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-041037b5b9482d79c8e2`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-d3686b94180b732c8001`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` — given panicking abi bodies when guarded then panics are contained (`src/abi/codec.rs:373`; `test-03d685383aaeadb55cad`).
- `given_rejected_capacity_when_retried_then_encoder_state_is_unchanged` — given rejected capacity when retried then encoder state is unchanged (`src/abi/codec.rs:323`; `test-d02294e14bc1e7d6bfd2`).
- `given_sine_440hz_when_round_trip_then_decoded_has_energy` — given sine 440hz when round trip then decoded has energy (`src/abi/codec.rs:435`; `test-3e20a259ad1a0f55a8c8`).

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
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Compatibility and evidence](/docs/compatibility/README.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Test evidence index](/docs/reference/test-evidence.md)
- [A conformance check cannot find external fixtures](/docs/troubleshooting/conformance-fixtures.md)
- [Keep qualification claims scoped](/docs/best-practices/evidence-boundaries.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `scripts/check_protocol.sh:1-132` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

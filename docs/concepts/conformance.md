# Conformance and qualification

<!-- claims: CLM-DOC-035-CAP-001,CLM-DOC-035-SOURCE-001 -->

Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

## Scope

- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

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
| `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| `sidecar` | function | Builds an outbound Connector backed by one bounded sidecar process. | `src/connector/mod.rs:112` |
| `with_driver` | function | Builds a connector whose bounded receiver loop is owned by Core. | `src/connector/mod.rs:88` |
| `pocketstation::connector` | module | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/mod.rs:1` |
| `pocketstation::connector::worker::ConnectorFactory` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/worker/mod.rs:17` |
| `pocketstation::connector::worker::ConnectorWorker` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/worker/mod.rs:32` |
| `pocketstation::abi::executable_extension::PksExtensionCallbacks` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:91` |
| `pocketstation::abi::executable_extension::PksExtensionLibrary` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:123` |
| `pocketstation::abi::executable_extension::PksExtensionPipelineDeclaration` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:168` |
| `pocketstation::abi::executable_extension::PksExtensionSignalBuffer` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:153` |
| `pocketstation::abi::executable_extension::PksExtensionSignalView` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:138` |
| `pocketstation::abi::extension::PksExtensionAbiVersion` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/extension.rs:14` |
| `pocketstation::abi::extension::PksExtensionDescriptor` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/extension.rs:47` |

## Where you encounter it

- **Author a connector** — Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.
- **Bind through C** — Create and operate a Session through ABI handles, status codes, and versioned callbacks.
- **Validate an integration** — Run protocol, ABI, connector, package, and example checks at the frozen source revision.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `abi_codec_cpp_conformance` — abi codec cpp conformance (`tests/abi_codec_cpp_conformance.cpp:1`; `test-6fbae5633a1419379cd7`).
- `abi_session_c_conformance` — abi session c conformance (`tests/abi_session_c_conformance.c:1`; `test-1ab6697ee6c783b1c41b`).
- `abi_session_c_success_conformance` — abi session c success conformance (`tests/abi_session_c_success_conformance.c:1`; `test-a2314a88cf28de25b331`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-041037b5b9482d79c8e2`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-d3686b94180b732c8001`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` — given panicking abi bodies when guarded then panics are contained (`src/abi/codec.rs:373`; `test-03d685383aaeadb55cad`).
- `given_rejected_capacity_when_retried_then_encoder_state_is_unchanged` — given rejected capacity when retried then encoder state is unchanged (`src/abi/codec.rs:323`; `test-d02294e14bc1e7d6bfd2`).
- `given_sine_440hz_when_round_trip_then_decoded_has_energy` — given sine 440hz when round trip then decoded has energy (`src/abi/codec.rs:435`; `test-3e20a259ad1a0f55a8c8`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [Glossary](/docs/glossary.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Run the examples](/docs/getting-started/examples.md)
- [Author a connector](/docs/guides/connectors.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Run protocol checks](/docs/how-to/run-protocol-checks.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/conformance.rs:1-1252` (`DIRECT`)
- `tests/protocol_compatibility.rs:1-55` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

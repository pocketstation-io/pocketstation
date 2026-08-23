# ABI and conformance model

<!-- claims: CLM-DOC-054-CAP-001,CLM-DOC-054-CAP-002,CLM-DOC-054-SOURCE-001 -->

## Scope

- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Ownership map

- `src/abi/mod.rs` owns part of this boundary.
- `src/conformance.rs` owns part of this boundary.

## Compiler-visible surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::abi::executable_extension::PksExtensionCallbacks` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:91` |
| `pocketstation::abi::executable_extension::PksExtensionLibrary` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:123` |
| `pocketstation::abi::executable_extension::PksExtensionPipelineDeclaration` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:168` |
| `pocketstation::abi::executable_extension::PksExtensionSignalBuffer` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:153` |
| `pocketstation::abi::executable_extension::PksExtensionSignalView` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:138` |
| `pocketstation::abi::extension::PksExtensionAbiVersion` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/extension.rs:14` |
| `pocketstation::abi::extension::PksExtensionDescriptor` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/extension.rs:47` |
| `pocketstation::abi::extension::PksExtensionPort` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/extension.rs:60` |
| `pocketstation::abi::session::abi::PksSessionStatus` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:56` |
| `pocketstation::abi::session::abi::PksSessionUtf8` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:101` |
| `pocketstation::abi::extension::PksExtensionKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/extension.rs:32` |
| `pocketstation::abi::extension::PksExtensionPortDirection` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/extension.rs:40` |
| `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:79` |
| `new` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:69` |
| `ok` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:62` |
| `pocketstation::abi::executable_extension::PksExtensionAcquireRegistrationCallback` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:110` |
| `pocketstation::abi::executable_extension::PksExtensionCreateCallback` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:56` |
| `pocketstation::abi::executable_extension::PksExtensionDestroyCallback` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:87` |
| `pocketstation::abi::executable_extension::PksExtensionEndpointConsumeCallback` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:77` |
| `pocketstation::abi::executable_extension::PksExtensionFinishCallback` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:85` |
| `pocketstation::abi::executable_extension::PksExtensionLibraryEntrypoint` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:133` |
| `pocketstation::abi::executable_extension::PksExtensionOperatorProcessCallback` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:70` |
| `pocketstation::abi::executable_extension::PksExtensionPrepareCallback` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:48` |
| `pocketstation::abi::executable_extension::PksExtensionSourceNextCallback` | type_alias | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/executable_extension.rs:63` |

## Observed implementation patterns

- `bounded_queue` — `tests/conformance_fixture.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `tests/public_api_boundary.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/abi/executable_extension.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `bounded_queue` — `src/abi/executable_extension.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/executable_extension.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/generated_audio_bridge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/session/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/abi/session/conformance_fixture.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/abi/session/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `tests/macos_native_ring_contract.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/session/conformance_fixture.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/session/runtime.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `sidecar_isolation` — `tests/protocol_compatibility.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

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

## Stability boundary

This page explains internals. Public compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts—not private module layout.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [C ABI reference](/docs/reference/c-abi.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [C ABI ownership](/docs/concepts/c-abi-ownership.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/abi/mod.rs:1-5` (`DIRECT`)
- `src/conformance.rs:1-1252` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

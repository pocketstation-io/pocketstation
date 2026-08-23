# Native extension libraries

<!-- claims: CLM-DOC-027-CAP-001,CLM-DOC-027-SOURCE-001 -->

Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.

## Scope

- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::native_extension::NativeExtensionLibrary` | struct | Immutable receipt for registrations imported into one Session. Executable code ownership remains internal to the registered factories and drivers. | `src/native_extension/mod.rs:62` |
| `pocketstation::native_extension::EXTENSION_LIBRARY_ENTRYPOINT_V1` | constant | Exact exported symbol required from a native Extension ABI v1 dynamic library. The suffix follows the ABI major; compatible minor revisions use the same entrypoint. | `src/native_extension/mod.rs:24` |
| `pocketstation::native_extension::NativeExtensionLibraryError` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:124` |
| `pocketstation::native_extension::NativeExtensionRegistration` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:34` |
| `pocketstation::native_extension::NativeExtensionKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:27` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:78` |
| `pocketstation::native_extension::NativeExtensionKind::Endpoint` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:30` |
| `pocketstation::native_extension::NativeExtensionKind::Operator` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:29` |
| `pocketstation::native_extension::NativeExtensionKind::Source` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:28` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::DuplicateRegistration` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:92` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:85` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointMissing` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:83` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointPanicked` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:84` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidLibraryDescriptor` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:88` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidRegistration` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:91` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::LibraryLoadFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:82` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathCanonicalizationFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:80` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotAbsolute` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:79` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotFile` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:81` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationAcquisitionFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:90` |

## Where you encounter it

- **Load a compiled extension** — Load a trusted absolute library path and import its registrations transactionally.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_core_extension_c_descriptor_when_validated_then_version_and_ports_pass` — given core extension c descriptor when validated then version and ports pass (`src/abi/extension.rs:286`; `test-4766cfbb0d0cdab01cdc`).
- `given_native_engine_when_created_then_real_session_declaration_compiles` — given native engine when created then real session declaration compiles (`src/abi/session/mod.rs:929`; `test-ec0f9d1e4ec547217e7b`).
- `given_core_extension_typed_stream_when_composed_then_signal_spec_is_runtime_authority` — given core extension typed stream when composed then signal spec is runtime authority (`tests/core_extension.rs:74`; `test-c7f7a0c2cf17718838d1`).
- `given_core_extension_wire_signals_when_read_then_ids_are_language_neutral` — given core extension wire signals when read then ids are language neutral (`tests/core_extension.rs:105`; `test-67b73fc4fe5bce8cb97f`).
- `macos_native_ring_contract` — macos native ring contract (`tests/macos_native_ring_contract.c:1`; `test-3fbcf9295b42f7dd9bf7`).
- `given_native_ring_contract_when_executed_then_visibility_and_drop_accounting_hold` — given native ring contract when executed then visibility and drop accounting hold (`tests/macos_native_ring_contract.rs:4`; `test-8dc1f35d1fe44a3ada8b`).
- `given_valid_native_library_when_loaded_then_canonical_session_executes_complete_pipeline` — given valid native library when loaded then canonical session executes complete pipeline (`tests/native_extension_library.rs:123`; `test-17108da423c933ddbc98`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-041037b5b9482d79c8e2`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Native extension API](/docs/reference/native-extensions.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/native_extension/library.rs:1-272` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

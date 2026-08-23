# Build and load a native extension

<!-- claims: CLM-GUIDE-018-CAP-001,CLM-GUIDE-018-CAP-002,CLM-GUIDE-018-CAP-003,CLM-GUIDE-018-SOURCE-001 -->

## Scope

- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.
- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Build a dynamic library exporting pks_extension_library_v1.
2. Use a canonical absolute path to a trusted regular file.
3. Return a compatible descriptor and callbacks.
4. Load through Session and retain the receipt.
5. Handle registration rollback and executable-code lifetime.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::LibraryLoadFailed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:82` |
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

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_valid_native_library_when_loaded_then_canonical_session_executes_complete_pipeline` — given valid native library when loaded then canonical session executes complete pipeline (`tests/native_extension_library.rs:123`; `test-17108da423c933ddbc98`).
- `given_core_extension_c_descriptor_when_validated_then_version_and_ports_pass` — given core extension c descriptor when validated then version and ports pass (`src/abi/extension.rs:286`; `test-4766cfbb0d0cdab01cdc`).
- `given_native_engine_when_created_then_real_session_declaration_compiles` — given native engine when created then real session declaration compiles (`src/abi/session/mod.rs:929`; `test-ec0f9d1e4ec547217e7b`).
- `given_core_extension_typed_stream_when_composed_then_signal_spec_is_runtime_authority` — given core extension typed stream when composed then signal spec is runtime authority (`tests/core_extension.rs:74`; `test-c7f7a0c2cf17718838d1`).
- `given_core_extension_wire_signals_when_read_then_ids_are_language_neutral` — given core extension wire signals when read then ids are language neutral (`tests/core_extension.rs:105`; `test-67b73fc4fe5bce8cb97f`).
- `macos_native_ring_contract` — macos native ring contract (`tests/macos_native_ring_contract.c:1`; `test-3fbcf9295b42f7dd9bf7`).
- `given_native_ring_contract_when_executed_then_visibility_and_drop_accounting_hold` — given native ring contract when executed then visibility and drop accounting hold (`tests/macos_native_ring_contract.rs:4`; `test-8dc1f35d1fe44a3ada8b`).
- `given_acquired_malformed_registration_when_loaded_then_context_is_destroyed_once` — given acquired malformed registration when loaded then context is destroyed once (`tests/native_extension_library.rs:106`; `test-47090f2c38cec228990b`).
- `given_duplicate_library_import_when_loaded_then_second_import_is_transactional` — given duplicate library import when loaded then second import is transactional (`tests/native_extension_library.rs:190`; `test-d8a393c10a8e4326265b`).
- `given_library_without_entrypoint_when_loaded_then_typed_error_is_returned` — given library without entrypoint when loaded then typed error is returned (`tests/native_extension_library.rs:77`; `test-67789b29ed2d07d71c3d`).
- `given_relative_library_path_when_loaded_then_ambient_search_is_rejected` — given relative library path when loaded then ambient search is rejected (`tests/native_extension_library.rs:65`; `test-337b49655c0e010a7d5c`).
- `given_unsupported_library_abi_when_loaded_then_registration_never_mutates_session` — given unsupported library abi when loaded then registration never mutates session (`tests/native_extension_library.rs:91`; `test-008f5c468f41d11fc947`).

## Failure signals

- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `UnsupportedAbiMinor` — `error-1a38891e490c637fd1f2`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` — `error-1a6a79718688c3bd3715`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidLibraryDescriptor` — `error-1f5abab34214f5b63b01`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidRegistration` — `error-1fc53703fe6fb00d8fab`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `RegistrationAcquisitionFailed` — `error-21780b72885c9e29a71c`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `DuplicateRegistration` — `error-3ea568577af25a2fc5e6`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointFailed` — `error-480de969fd202f51c549`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointPanicked` — `error-527bf9a9b68ccbda6c90`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathNotAbsolute` — `error-6baafa30dcc94f68cad3`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathCanonicalizationFailed` — `error-735365ce5d41572f06f2`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathNotFile` — `error-76f6466e465a0d7a52d9`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `RegistrationAcquisitionPanicked` — `error-86e449bc7fc67119dab5`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [C ABI reference](/docs/reference/c-abi.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/fixtures/native_extension_plugin.rs:1-427` (`DIRECT`)
- `src/native_extension/library.rs:1-272` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

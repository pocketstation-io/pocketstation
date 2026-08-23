# Build and load a native extension

<!-- claims: CLM-GUIDE-018-CAP-001,CLM-GUIDE-018-CAP-002,CLM-GUIDE-018-CAP-003,CLM-GUIDE-018-SOURCE-001 -->

## Scope

- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.
- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

The scope of **Build and load a native extension** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A dynamic library for the current target, a compatible ABI descriptor, and a host trust decision for its canonical path.

## Procedure

1. Build a dynamic library exporting pks_extension_library_v1.
2. Use a canonical absolute path to a trusted regular file.
3. Return a compatible descriptor and callbacks.
4. Load through Session and retain the receipt.
5. Handle registration rollback and executable-code lifetime.

## Important consequence

Treat path validation, library loading, entry point, ABI, descriptor, and registration as distinct failure stages.

## Verify the outcome

The loader returns a receipt, all registrations appear together, and the library remains owned while callbacks are reachable.

Executable evidence selected for **Build and load a native extension** is limited to each test's recorded setup and assertions:

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

- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `LibraryLoadFailed` — `error-565dce9eae9619340a07`
- `pocketstation::native_extension::NativeExtensionLibraryError` — `error-3f67c43b32f6fcad4623`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` — `error-e8cfe53ac16a24ec271a`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `DuplicateRegistration` — `error-ae300d01d210d603e094`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointFailed` — `error-62cf0cb93419ca1b424f`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointMissing` — `error-8055e200b698a55ee9e7`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointPanicked` — `error-09da0172ff632c40da5e`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidLibraryDescriptor` — `error-32deb305311b19362a8f`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidRegistration` — `error-f891b63a492cbaf2009f`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathCanonicalizationFailed` — `error-39b8d96537c461086e6c`

## API reference

- [Native Extensions](/docs/concepts/native-extensions.md)
- [Native Extensions](/docs/reference/native-extensions.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::LibraryLoadFailed` | variant | Reported when the owning operation encounters library load failed. | `src/native_extension/mod.rs:82` |
| `pocketstation::native_extension::NativeExtensionLibrary` | struct | Immutable receipt for registrations imported into one Session. Executable code ownership remains internal to the registered factories and drivers. | `src/native_extension/mod.rs:62` |
| `pocketstation::native_extension::NativeExtensionLibraryError` | struct | Reports a native extension library error. | `src/native_extension/mod.rs:124` |
| `pocketstation::native_extension::NativeExtensionRegistration` | struct | Identifies one node registration imported transactionally from a native extension. | `src/native_extension/mod.rs:34` |
| `pocketstation::native_extension::NativeExtensionKind` | enum | Selects the native extension kind used by PocketStation. | `src/native_extension/mod.rs:27` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | enum | Enumerates the supported native extension library error code cases. | `src/native_extension/mod.rs:78` |
| `pocketstation::native_extension::EXTENSION_LIBRARY_ENTRYPOINT_V1` | constant | Exact exported symbol required from a native Extension ABI v1 dynamic library. The suffix follows the ABI major; compatible minor revisions use the same entrypoint. | `src/native_extension/mod.rs:24` |
| `pocketstation::native_extension::NativeExtensionKind::Endpoint` | variant | Selects endpoint behavior for `NativeExtensionKind`. | `src/native_extension/mod.rs:30` |

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

The claims on **Build and load a native extension** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/fixtures/native_extension_plugin.rs:1-427` (`DIRECT`)
- `src/native_extension/library.rs:1-272` (`DIRECT`)

For **Build and load a native extension**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

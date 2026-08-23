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

- `given_valid_native_library_when_loaded_then_canonical_session_executes_complete_pipeline` — given valid native library when loaded then canonical session executes complete pipeline (`tests/native_extension_library.rs:123`; `test-25e654305e19fe3fc41a`).
- `given_core_extension_c_descriptor_when_validated_then_version_and_ports_pass` — given core extension c descriptor when validated then version and ports pass (`src/abi/extension.rs:286`; `test-0db570036043ad373bee`).
- `given_native_engine_when_created_then_real_session_declaration_compiles` — given native engine when created then real session declaration compiles (`src/abi/session/mod.rs:929`; `test-c204d11ecd759d78439f`).
- `given_core_extension_typed_stream_when_composed_then_signal_spec_is_runtime_authority` — given core extension typed stream when composed then signal spec is runtime authority (`tests/core_extension.rs:74`; `test-2c31d24f3d3f3a3bbc51`).
- `given_core_extension_wire_signals_when_read_then_ids_are_language_neutral` — given core extension wire signals when read then ids are language neutral (`tests/core_extension.rs:105`; `test-3a38f531a8bffa6ce5ed`).
- `macos_native_ring_contract` — macos native ring contract (`tests/macos_native_ring_contract.c:1`; `test-6913f817419f8c9437ac`).
- `given_native_ring_contract_when_executed_then_visibility_and_drop_accounting_hold` — given native ring contract when executed then visibility and drop accounting hold (`tests/macos_native_ring_contract.rs:4`; `test-7e712cc97661871fb169`).
- `given_acquired_malformed_registration_when_loaded_then_context_is_destroyed_once` — given acquired malformed registration when loaded then context is destroyed once (`tests/native_extension_library.rs:106`; `test-201890b5e1a3afb22ef5`).
- `given_duplicate_library_import_when_loaded_then_second_import_is_transactional` — given duplicate library import when loaded then second import is transactional (`tests/native_extension_library.rs:190`; `test-5d5feb4cbedf509d8146`).
- `given_library_without_entrypoint_when_loaded_then_typed_error_is_returned` — given library without entrypoint when loaded then typed error is returned (`tests/native_extension_library.rs:77`; `test-4678c1b527bd72fa5830`).
- `given_relative_library_path_when_loaded_then_ambient_search_is_rejected` — given relative library path when loaded then ambient search is rejected (`tests/native_extension_library.rs:65`; `test-de03d325dde793e9ec1b`).
- `given_unsupported_library_abi_when_loaded_then_registration_never_mutates_session` — given unsupported library abi when loaded then registration never mutates session (`tests/native_extension_library.rs:91`; `test-527acebb13bb37c6f430`).

## Failure signals

- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `LibraryLoadFailed` — `error-d97432d4b6bf9216a606`
- `pocketstation::native_extension::NativeExtensionLibraryError` — `error-ecf350aa6c55c67611e2`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` — `error-8c56cdf18bee2fc21bae`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `DuplicateRegistration` — `error-971353c5ff9dde1692e9`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointFailed` — `error-f77a66c0a6491320325a`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointMissing` — `error-77bedea908d2b38f8807`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointPanicked` — `error-b6f16e992a8c85677bc8`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidLibraryDescriptor` — `error-23644427ccfe2e45879e`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidRegistration` — `error-30fb4ebaa151784596bc`
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathCanonicalizationFailed` — `error-26b0a33c4952ffa51dc0`

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

The claims on **Build and load a native extension** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `tests/fixtures/native_extension_plugin.rs:1-427` (`DIRECT`)
- `src/native_extension/library.rs:1-272` (`DIRECT`)

For **Build and load a native extension**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

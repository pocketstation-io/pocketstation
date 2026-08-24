# Native extension libraries

<!-- claims: CLM-DOC-027-SCOPE-001,CLM-DOC-027-TEXT-001,CLM-DOC-027-TEXT-002,CLM-DOC-027-TEXT-003,CLM-DOC-027-TEXT-004,CLM-DOC-027-TEXT-005,CLM-DOC-027-TEXT-006,CLM-DOC-027-SOURCE-001 -->

## What it is

A native extension library is trusted executable code that exports the versioned PocketStation entry point and contributes source, operator, or endpoint registrations.

## Why it exists

Dynamic loading crosses file-system, ABI, callback, and executable-lifetime boundaries. An explicit library owner keeps those checks and registrations transactional.

## Relationships

- The C ABI defines descriptor and callback layouts.
- The loader canonicalizes the path and validates the entry point and ABI version.
- A load receipt retains the library while imported callbacks remain reachable.

## Invariants and guarantees

- The input path is absolute, canonical, and a regular file.
- Registrations are rolled back if import fails.
- The host—not PocketStation—decides whether the library is trusted to execute.

## When you encounter it

- **Load a compiled extension** — Load a trusted absolute library path and import its registrations transactionally.

## Use it

- [Build and load a native extension](/docs/guides/extensions.md)
- [Load extensions from trusted paths](/docs/best-practices/native-extension-trust.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)

## Scope

- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.

The scope of **Native extension libraries** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::native_extension::NativeExtensionLibrary` | struct | Immutable receipt for registrations imported into one Session. Executable code ownership remains internal to the registered factories and drivers. | `src/native_extension/mod.rs:62` |
| `pocketstation::native_extension::NativeExtensionLibraryError` | struct | Reports a native extension library error. | `src/native_extension/mod.rs:124` |
| `pocketstation::native_extension::NativeExtensionRegistration` | struct | Identifies one node registration imported transactionally from a native extension. | `src/native_extension/mod.rs:34` |
| `pocketstation::native_extension::NativeExtensionKind` | enum | Selects the native extension kind used by PocketStation. | `src/native_extension/mod.rs:27` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | enum | Provides stable categories for native-extension load and validation failures. | `src/native_extension/mod.rs:78` |
| `pocketstation::native_extension::EXTENSION_LIBRARY_ENTRYPOINT_V1` | constant | Exact exported symbol required from a native Extension ABI v1 dynamic library. The suffix follows the ABI major; compatible minor revisions use the same entrypoint. | `src/native_extension/mod.rs:24` |
| `pocketstation::native_extension::NativeExtensionKind::Endpoint` | variant | Classifies the loaded native extension as endpoint. | `src/native_extension/mod.rs:30` |
| `pocketstation::native_extension::NativeExtensionKind::Operator` | variant | Classifies the loaded native extension as operator. | `src/native_extension/mod.rs:29` |
| `pocketstation::native_extension::NativeExtensionKind::Source` | variant | Classifies the loaded native extension as source. | `src/native_extension/mod.rs:28` |
| `pocketstation::native_extension::NativeExtensionLibraryErrorCode::DuplicateRegistration` | variant | Reports that registration duplicates an existing declaration or record. | `src/native_extension/mod.rs:92` |

## Executable evidence

Executable evidence selected for **Native extension libraries** is limited to each test's recorded setup and assertions:

- `given_core_extension_c_descriptor_when_validated_then_version_and_ports_pass` — given core extension c descriptor when validated then version and ports pass (`src/abi/extension.rs:286`; `test-0db570036043ad373bee`).
- `given_native_engine_when_created_then_real_session_declaration_compiles` — given native engine when created then real session declaration compiles (`src/abi/session/mod.rs:929`; `test-c204d11ecd759d78439f`).
- `given_core_extension_typed_stream_when_composed_then_signal_spec_is_runtime_authority` — given core extension typed stream when composed then signal spec is runtime authority (`tests/core_extension.rs:74`; `test-2c31d24f3d3f3a3bbc51`).
- `given_core_extension_wire_signals_when_read_then_ids_are_language_neutral` — given core extension wire signals when read then ids are language neutral (`tests/core_extension.rs:105`; `test-3a38f531a8bffa6ce5ed`).
- `macos_native_ring_contract` — macos native ring contract (`tests/macos_native_ring_contract.c:1`; `test-6913f817419f8c9437ac`).
- `given_native_ring_contract_when_executed_then_visibility_and_drop_accounting_hold` — given native ring contract when executed then visibility and drop accounting hold (`tests/macos_native_ring_contract.rs:4`; `test-7e712cc97661871fb169`).
- `given_valid_native_library_when_loaded_then_canonical_session_executes_complete_pipeline` — given valid native library when loaded then canonical session executes complete pipeline (`tests/native_extension_library.rs:123`; `test-25e654305e19fe3fc41a`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-f2e13a28f1aa591f3c67`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-8ba7b5b19e9d7dfbc464`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-9fb4684ff29b5ab716fd`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-002ce44230f2b0ac6d7c`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-657d1e2cbdcbd70cf5fa`).

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

The claims on **Native extension libraries** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/native_extension/library.rs:19-19` (`DIRECT`)
- `src/native_extension/library.rs:21-24` (`DIRECT`)
- `src/native_extension/library.rs:22-22` (`DIRECT`)
- `src/native_extension/library.rs:23-23` (`DIRECT`)
- `src/native_extension/library.rs:33-230` (`DIRECT`)
- `src/native_extension/library.rs:232-263` (`DIRECT`)
- `src/native_extension/library.rs:265-271` (`DIRECT`)

For **Native extension libraries**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

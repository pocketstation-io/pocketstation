# A native extension does not load

<!-- claims: CLM-TRBL-009-CAP-001,CLM-TRBL-009-CAP-002,CLM-TRBL-009-CAP-003,CLM-TRBL-009-SOURCE-001 -->

## Symptom

Loading a native extension returns a path, library, ABI, descriptor, or registration error.

## Evidenced causes

- The path is relative, cannot be canonicalized, or is not a regular file.
- The dynamic loader cannot open the library or find the required entry point.
- The ABI version or descriptor is incompatible.
- Registration conflicts and transaction rollback runs.

## Distinguish the causes

Follow the reported stages in order: path validation, library load, entry point, ABI, descriptor, registration, rollback.

## Diagnostic signals

- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `LibraryLoadFailed` (`error-d97432d4b6bf9216a606`)
- `pocketstation::native_extension::NativeExtensionLibraryError` (`error-ecf350aa6c55c67611e2`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` (`error-8c56cdf18bee2fc21bae`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `DuplicateRegistration` (`error-971353c5ff9dde1692e9`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointFailed` (`error-f77a66c0a6491320325a`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointMissing` (`error-77bedea908d2b38f8807`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointPanicked` (`error-b6f16e992a8c85677bc8`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidLibraryDescriptor` (`error-23644427ccfe2e45879e`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidRegistration` (`error-30fb4ebaa151784596bc`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathCanonicalizationFailed` (`error-26b0a33c4952ffa51dc0`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathNotAbsolute` (`error-59bfaa1be8a652b3c7b6`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathNotFile` (`error-c40c70d459d226d51a4a`)

## Executable evidence

- `given_valid_native_library_when_loaded_then_canonical_session_executes_complete_pipeline` exercises given valid native library when loaded then canonical session executes complete pipeline under its recorded setup (`test-25e654305e19fe3fc41a`).
- `given_core_extension_c_descriptor_when_validated_then_version_and_ports_pass` exercises given core extension c descriptor when validated then version and ports pass under its recorded setup (`test-0db570036043ad373bee`).
- `given_native_engine_when_created_then_real_session_declaration_compiles` exercises given native engine when created then real session declaration compiles under its recorded setup (`test-c204d11ecd759d78439f`).
- `given_core_extension_typed_stream_when_composed_then_signal_spec_is_runtime_authority` exercises given core extension typed stream when composed then signal spec is runtime authority under its recorded setup (`test-2c31d24f3d3f3a3bbc51`).
- `given_core_extension_wire_signals_when_read_then_ids_are_language_neutral` exercises given core extension wire signals when read then ids are language neutral under its recorded setup (`test-3a38f531a8bffa6ce5ed`).
- `macos_native_ring_contract` exercises macos native ring contract under its recorded setup (`test-6913f817419f8c9437ac`).
- `given_native_ring_contract_when_executed_then_visibility_and_drop_accounting_hold` exercises given native ring contract when executed then visibility and drop accounting hold under its recorded setup (`test-7e712cc97661871fb169`).
- `given_acquired_malformed_registration_when_loaded_then_context_is_destroyed_once` exercises given acquired malformed registration when loaded then context is destroyed once under its recorded setup (`test-201890b5e1a3afb22ef5`).
- `given_duplicate_library_import_when_loaded_then_second_import_is_transactional` exercises given duplicate library import when loaded then second import is transactional under its recorded setup (`test-5d5feb4cbedf509d8146`).
- `given_library_without_entrypoint_when_loaded_then_typed_error_is_returned` exercises given library without entrypoint when loaded then typed error is returned under its recorded setup (`test-4678c1b527bd72fa5830`).
- `given_relative_library_path_when_loaded_then_ambient_search_is_rejected` exercises given relative library path when loaded then ambient search is rejected under its recorded setup (`test-de03d325dde793e9ec1b`).
- `given_unsupported_library_abi_when_loaded_then_registration_never_mutates_session` exercises given unsupported library abi when loaded then registration never mutates session under its recorded setup (`test-527acebb13bb37c6f430`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` exercises given bitrate change when encode then still produces valid packet under its recorded setup (`test-f2e13a28f1aa591f3c67`).
- `given_encoder_when_destroy_null_then_no_crash` exercises given encoder when destroy null then no crash under its recorded setup (`test-8ba7b5b19e9d7dfbc464`).
- `given_invalid_channel_count_when_create_then_returns_null` exercises given invalid channel count when create then returns null under its recorded setup (`test-9fb4684ff29b5ab716fd`).

## Corrective action

Use a trusted canonical absolute library built for the target and correct the first failing ABI or registration contract.

## Retry and incomplete state

Do not repeatedly execute an untrusted or incompatible library. Failed registration is not partial success unless the receipt and registry prove otherwise.

## Related reference

- [Native Extensions](/docs/concepts/native-extensions.md)
- [Native Extensions](/docs/reference/native-extensions.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Extension and ABI failures](/docs/errors/extensions-and-abi.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Operate a Session through C](/docs/how-to/use-c-session-api.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)

## Evidence boundary

The claims on **A native extension does not load** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/native_extension/library.rs:1-272` (`DIRECT`)

For **A native extension does not load**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

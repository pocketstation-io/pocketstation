# A native extension does not load

<!-- claims: CLM-TRBL-009-CAP-001,CLM-TRBL-009-CAP-002,CLM-TRBL-009-CAP-003,CLM-TRBL-009-SOURCE-001 -->

Use this page when you observe **a native extension does not load**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Check absolute path, canonicalization, regular-file, library-load, entrypoint, ABI-version, descriptor, and registration errors in their reported order.

## Diagnostic signals

- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `UnsupportedAbiMinor` (`error-1a38891e490c637fd1f2`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` (`error-1a6a79718688c3bd3715`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidLibraryDescriptor` (`error-1f5abab34214f5b63b01`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `InvalidRegistration` (`error-1fc53703fe6fb00d8fab`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `RegistrationAcquisitionFailed` (`error-21780b72885c9e29a71c`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `DuplicateRegistration` (`error-3ea568577af25a2fc5e6`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointFailed` (`error-480de969fd202f51c549`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointPanicked` (`error-527bf9a9b68ccbda6c90`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathNotAbsolute` (`error-6baafa30dcc94f68cad3`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathCanonicalizationFailed` (`error-735365ce5d41572f06f2`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `PathNotFile` (`error-76f6466e465a0d7a52d9`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `RegistrationAcquisitionPanicked` (`error-86e449bc7fc67119dab5`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `EntrypointMissing` (`error-b09bbf6a2e20bddb6588`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `UnsupportedAbiMajor` (`error-bbda4884c178e8e9698d`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `RegistrationStateUnavailable` (`error-c7729f534adabf1f25cb`)
- `pocketstation::native_extension::NativeExtensionLibraryError` (`error-c8625ea4d5c0a2384c45`)
- `pocketstation::native_extension::NativeExtensionLibraryErrorCode` / `LibraryLoadFailed` (`error-fda081be8f949890171b`)

## Executable evidence

- `given_valid_native_library_when_loaded_then_canonical_session_executes_complete_pipeline` exercises given valid native library when loaded then canonical session executes complete pipeline under its recorded setup (`test-17108da423c933ddbc98`).
- `given_core_extension_c_descriptor_when_validated_then_version_and_ports_pass` exercises given core extension c descriptor when validated then version and ports pass under its recorded setup (`test-4766cfbb0d0cdab01cdc`).
- `given_native_engine_when_created_then_real_session_declaration_compiles` exercises given native engine when created then real session declaration compiles under its recorded setup (`test-ec0f9d1e4ec547217e7b`).
- `given_core_extension_typed_stream_when_composed_then_signal_spec_is_runtime_authority` exercises given core extension typed stream when composed then signal spec is runtime authority under its recorded setup (`test-c7f7a0c2cf17718838d1`).
- `given_core_extension_wire_signals_when_read_then_ids_are_language_neutral` exercises given core extension wire signals when read then ids are language neutral under its recorded setup (`test-67b73fc4fe5bce8cb97f`).
- `macos_native_ring_contract` exercises macos native ring contract under its recorded setup (`test-3fbcf9295b42f7dd9bf7`).
- `given_native_ring_contract_when_executed_then_visibility_and_drop_accounting_hold` exercises given native ring contract when executed then visibility and drop accounting hold under its recorded setup (`test-8dc1f35d1fe44a3ada8b`).
- `given_acquired_malformed_registration_when_loaded_then_context_is_destroyed_once` exercises given acquired malformed registration when loaded then context is destroyed once under its recorded setup (`test-47090f2c38cec228990b`).
- `given_duplicate_library_import_when_loaded_then_second_import_is_transactional` exercises given duplicate library import when loaded then second import is transactional under its recorded setup (`test-d8a393c10a8e4326265b`).
- `given_library_without_entrypoint_when_loaded_then_typed_error_is_returned` exercises given library without entrypoint when loaded then typed error is returned under its recorded setup (`test-67789b29ed2d07d71c3d`).
- `given_relative_library_path_when_loaded_then_ambient_search_is_rejected` exercises given relative library path when loaded then ambient search is rejected under its recorded setup (`test-337b49655c0e010a7d5c`).
- `given_unsupported_library_abi_when_loaded_then_registration_never_mutates_session` exercises given unsupported library abi when loaded then registration never mutates session under its recorded setup (`test-008f5c468f41d11fc947`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` exercises given bitrate change when encode then still produces valid packet under its recorded setup (`test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` exercises given encoder when destroy null then no crash under its recorded setup (`test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` exercises given invalid channel count when create then returns null under its recorded setup (`test-736ddd354b42f58df4ad`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/native_extension/library.rs:1-272` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

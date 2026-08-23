# Load extensions from trusted absolute paths

<!-- claims: CLM-BEST-006-CAP-001,CLM-BEST-006-CAP-002,CLM-BEST-006-SOURCE-001 -->

## Problem

Loading a native extension executes code and retains callback pointers for the registration lifetime.

## Recommendation

Accept only a host-trusted canonical absolute path to a regular file, then keep the load receipt alive while imported registrations are reachable.

## Reason

Path, ABI, descriptor, transaction, and executable-lifetime checks protect the loader's declared boundary.

## Tradeoff

Canonicalization and trust policy add deployment work and do not sandbox the loaded code.

## When it does not apply

This rule does not apply to a connector that runs exclusively through a separately managed process boundary.

## Repository evidence

This recommendation is tied directly to the page's source evidence.

## Executable evidence

Executable evidence selected for **Load extensions from trusted absolute paths** is limited to each test's recorded setup and assertions:

- `given_acquired_malformed_registration_when_loaded_then_context_is_destroyed_once` — given acquired malformed registration when loaded then context is destroyed once (`tests/native_extension_library.rs:106`; `test-201890b5e1a3afb22ef5`).
- `given_duplicate_library_import_when_loaded_then_second_import_is_transactional` — given duplicate library import when loaded then second import is transactional (`tests/native_extension_library.rs:190`; `test-5d5feb4cbedf509d8146`).
- `given_library_without_entrypoint_when_loaded_then_typed_error_is_returned` — given library without entrypoint when loaded then typed error is returned (`tests/native_extension_library.rs:77`; `test-4678c1b527bd72fa5830`).
- `given_relative_library_path_when_loaded_then_ambient_search_is_rejected` — given relative library path when loaded then ambient search is rejected (`tests/native_extension_library.rs:65`; `test-de03d325dde793e9ec1b`).
- `given_unsupported_library_abi_when_loaded_then_registration_never_mutates_session` — given unsupported library abi when loaded then registration never mutates session (`tests/native_extension_library.rs:91`; `test-527acebb13bb37c6f430`).
- `given_valid_native_library_when_loaded_then_canonical_session_executes_complete_pipeline` — given valid native library when loaded then canonical session executes complete pipeline (`tests/native_extension_library.rs:123`; `test-25e654305e19fe3fc41a`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-f2e13a28f1aa591f3c67`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-8ba7b5b19e9d7dfbc464`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-9fb4684ff29b5ab716fd`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-002ce44230f2b0ac6d7c`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-657d1e2cbdcbd70cf5fa`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-f10bfad1b583316ad6fb`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)
- [Extension and ABI failures](/docs/errors/extensions-and-abi.md)

## Evidence boundary

The claims on **Load extensions from trusted absolute paths** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/native_extension/library.rs:1-272` (`DIRECT`)

For **Load extensions from trusted absolute paths**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

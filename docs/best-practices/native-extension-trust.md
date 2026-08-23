# Load extensions from trusted absolute paths

<!-- claims: CLM-BEST-006-CAP-001,CLM-BEST-006-CAP-002,CLM-BEST-006-SOURCE-001 -->

## Recommendation

Load executable extensions only from a canonical absolute path whose trust decision belongs to the host application.

## Why

The repository makes capacity, ownership, identity, lifecycle, and evidence boundaries explicit so failures remain attributable. Bypassing them removes observations and typed outcomes needed for diagnosis.

## Tradeoff

The recommendation requires explicit configuration and result handling. It does not promise that one capacity, retry budget, selector, or shutdown policy fits every workload. Measure within the API's stated scope.

## When it does not apply

Do not apply a realtime, connector, capture, or extension rule to another lane or boundary unless it exposes the same contract. An internal pattern is not automatically a public recommendation.

## Repository evidence

- `bounded_queue` at `tests/conformance_fixture.rs` (`pattern-095b17abae307638000d`).
- `clock_correlation` at `src/abi/executable_extension.rs` (`pattern-4ba4a78f05d586dd6668`).
- `bounded_queue` at `src/abi/executable_extension.rs` (`pattern-525a976746de10b4f0c9`).
- `typed_error` at `src/abi/executable_extension.rs` (`pattern-5be4ae7df5544c36e922`).
- `typed_error` at `src/abi/session/mod.rs` (`pattern-796af7f8b2fa015d666c`).
- `buffer_pool` at `src/abi/session/conformance_fixture.rs` (`pattern-89c11b4a94980624b4e5`).
- `buffer_pool` at `src/abi/session/mod.rs` (`pattern-96698d9df68517671d53`).
- `typed_error` at `tests/macos_native_ring_contract.rs` (`pattern-c187a3b04f866efe0acb`).
- `typed_error` at `src/abi/session/conformance_fixture.rs` (`pattern-c94cceb556c86ba83b5b`).
- `typed_error` at `src/abi/session/runtime.rs` (`pattern-e533a842654dad617452`).

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_acquired_malformed_registration_when_loaded_then_context_is_destroyed_once` — given acquired malformed registration when loaded then context is destroyed once (`tests/native_extension_library.rs:106`; `test-47090f2c38cec228990b`).
- `given_duplicate_library_import_when_loaded_then_second_import_is_transactional` — given duplicate library import when loaded then second import is transactional (`tests/native_extension_library.rs:190`; `test-d8a393c10a8e4326265b`).
- `given_library_without_entrypoint_when_loaded_then_typed_error_is_returned` — given library without entrypoint when loaded then typed error is returned (`tests/native_extension_library.rs:77`; `test-67789b29ed2d07d71c3d`).
- `given_relative_library_path_when_loaded_then_ambient_search_is_rejected` — given relative library path when loaded then ambient search is rejected (`tests/native_extension_library.rs:65`; `test-337b49655c0e010a7d5c`).
- `given_unsupported_library_abi_when_loaded_then_registration_never_mutates_session` — given unsupported library abi when loaded then registration never mutates session (`tests/native_extension_library.rs:91`; `test-008f5c468f41d11fc947`).
- `given_valid_native_library_when_loaded_then_canonical_session_executes_complete_pipeline` — given valid native library when loaded then canonical session executes complete pipeline (`tests/native_extension_library.rs:123`; `test-17108da423c933ddbc98`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-041037b5b9482d79c8e2`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-d3686b94180b732c8001`).

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/native_extension/library.rs:1-272` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

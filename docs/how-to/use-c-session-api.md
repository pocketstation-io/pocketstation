# Operate a Session through C

<!-- claims: CLM-GUIDE-019-CAP-001,CLM-GUIDE-019-CAP-002,CLM-GUIDE-019-SOURCE-001 -->

## Scope

- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Operate a Session through C** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

The repository header, a compatible ABI version, and correct ownership for every opaque handle and callback context.

## Procedure

1. Include pocketstation.h and use its ABI version.
2. Create handles through exported functions.
3. Check every PksSessionStatus.
4. Stop before releasing runtime ownership.
5. Release each handle with its matching ABI function.

## Important consequence

Do not let a Rust panic, borrowed pointer, or library context escape its declared ABI lifetime.

## Verify the outcome

Every call returns an accepted `PksSessionStatus`, stop completes, and each handle is released by its matching function.

Executable evidence selected for **Operate a Session through C** is limited to each test's recorded setup and assertions:

- `abi_session_c_success_conformance` — abi session c success conformance (`tests/abi_session_c_success_conformance.c:1`; `test-a2314a88cf28de25b331`).
- `given_deterministic_session_when_polled_then_audio_lease_is_bounded_and_stable` — given deterministic session when polled then audio lease is bounded and stable (`src/abi/session/mod.rs:1036`; `test-16c3dd3ac223381ec20a`).
- `given_native_engine_when_created_then_real_session_declaration_compiles` — given native engine when created then real session declaration compiles (`src/abi/session/mod.rs:929`; `test-ec0f9d1e4ec547217e7b`).
- `abi_session_c_conformance` — abi session c conformance (`tests/abi_session_c_conformance.c:1`; `test-1ab6697ee6c783b1c41b`).
- `abi_session_c_metrics_canary` — abi session c metrics canary (`tests/abi_session_c_metrics_canary.c:1`; `test-fe902c5e04fb9d6f128e`).
- `given_fixture_session_when_started_then_two_stems_cross_canonical_engine` — given fixture session when started then two stems cross canonical engine (`tests/conformance_fixture.rs:14`; `test-bec1e3cd7f059a144101`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-041037b5b9482d79c8e2`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-d3686b94180b732c8001`).

## Failure signals

No task-specific public error was resolved for operate a session through c; preserve the owning API's returned error.

## API reference

- [C Abi Ownership](/docs/concepts/c-abi-ownership.md)
- [C Abi](/docs/reference/c-abi.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::abi::session::abi::PksSessionStatus` | struct | Reports the structured session status. | `src/abi/session/abi.rs:56` |
| `pocketstation::abi::session::abi::PksSessionUtf8` | struct | Borrows a UTF-8 byte range across the C Session ABI as a pointer and length. | `src/abi/session/abi.rs:101` |
| `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Enumerates the supported session status code cases. | `src/abi/session/abi.rs:79` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | Identifies the backend failure state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:93` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | Indicates that the operation was cancelled. | `src/abi/session/abi.rs:94` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | Identifies the foreign handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:90` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | Identifies the index out of range state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:95` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | Identifies the internal panic state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:87` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)
- [Extension and ABI failures](/docs/errors/extensions-and-abi.md)
- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [C ABI ownership](/docs/concepts/c-abi-ownership.md)

## Evidence boundary

The claims on **Operate a Session through C** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/abi_session_c_success_conformance.c:1-191` (`DIRECT`)

For **Operate a Session through C**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

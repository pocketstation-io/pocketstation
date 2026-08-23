# C ABI ownership

<!-- claims: CLM-DOC-028-CAP-001,CLM-DOC-028-SOURCE-001 -->

## What it is

The C ABI exposes versioned structures, opaque handles, status values, callbacks, and release functions for Session and extension operations.

## Why it exists

Managed and native SDKs need a stable binary boundary whose ownership and error behavior do not depend on Rust layout or panic propagation.

## Relationships

- `pocketstation.h` is the consumer-facing declaration authority.
- Rust ABI wrappers translate typed outcomes into stable C status structures.
- Conformance executables verify layout and cross-language behavior under recorded fixtures.

## Invariants and guarantees

- Callers set and check the declared ABI version and structure size.
- Every created handle is released through its matching function.
- Rust panics are contained and translated at the ABI boundary.

## When you encounter it

- **Load a compiled extension** — Load a trusted absolute library path and import its registrations transactionally.
- **Bind through C** — Create and operate a Session through ABI handles, status codes, and versioned callbacks.

## Use it

- [Operate a Session through C](/docs/how-to/use-c-session-api.md)
- [C ABI reference](/docs/reference/c-abi.md)
- [Run protocol checks](/docs/how-to/run-protocol-checks.md)

## Scope

- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.

The scope of **C ABI ownership** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::abi::session::abi::PksSessionStatus` | struct | Reports the structured session status. | `src/abi/session/abi.rs:56` |
| `pocketstation::abi::session::abi::PksSessionUtf8` | struct | Borrows a UTF-8 byte range across the C Session ABI as a pointer and length. | `src/abi/session/abi.rs:101` |
| `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Enumerates the supported session status code cases. | `src/abi/session/abi.rs:79` |
| `new` | function | Creates a new `PksSessionStatus`. | `src/abi/session/abi.rs:69` |
| `ok` | function | Creates a successful status value for `PksSessionStatus`. | `src/abi/session/abi.rs:62` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | Identifies the backend failure state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:93` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | Indicates that the operation was cancelled. | `src/abi/session/abi.rs:94` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | Identifies the foreign handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:90` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | Identifies the index out of range state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:95` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | Identifies the internal panic state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:87` |

## Executable evidence

Executable evidence selected for **C ABI ownership** is limited to each test's recorded setup and assertions:

- `given_full_table_when_insert_then_capacity_failure_is_returned` — given full table when insert then capacity failure is returned (`src/abi/session/handle.rs:148`; `test-a76e025cf00946b166f4`).
- `given_other_scope_when_lookup_then_foreign_handle_is_reported` — given other scope when lookup then foreign handle is reported (`src/abi/session/handle.rs:159`; `test-a0dac1b61a0737f50473`).
- `given_removed_handle_when_lookup_then_stale_is_reported` — given removed handle when lookup then stale is reported (`src/abi/session/handle.rs:135`; `test-ee9437f141714a79bbc7`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-041037b5b9482d79c8e2`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-d3686b94180b732c8001`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` — given panicking abi bodies when guarded then panics are contained (`src/abi/codec.rs:373`; `test-03d685383aaeadb55cad`).
- `given_rejected_capacity_when_retried_then_encoder_state_is_unchanged` — given rejected capacity when retried then encoder state is unchanged (`src/abi/codec.rs:323`; `test-d02294e14bc1e7d6bfd2`).
- `given_sine_440hz_when_round_trip_then_decoded_has_energy` — given sine 440hz when round trip then decoded has energy (`src/abi/codec.rs:435`; `test-3e20a259ad1a0f55a8c8`).

## Related documentation

- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Operate a Session through C](/docs/how-to/use-c-session-api.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [C ABI reference](/docs/reference/c-abi.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)

## Evidence boundary

The claims on **C ABI ownership** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `include/pocketstation.h:1-615` (`DIRECT`)
- `src/abi/session/handle.rs:1-173` (`DIRECT`)

For **C ABI ownership**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

# C ABI ownership

<!-- claims: CLM-DOC-028-SCOPE-001,CLM-DOC-028-TEXT-001,CLM-DOC-028-TEXT-002,CLM-DOC-028-TEXT-003,CLM-DOC-028-TEXT-004,CLM-DOC-028-TEXT-005,CLM-DOC-028-TEXT-006,CLM-DOC-028-SOURCE-001 -->

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
| `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Provides stable C ABI status categories returned by Session operations. | `src/abi/session/abi.rs:79` |
| `new` | function | Creates a new `PksSessionStatus`. | `src/abi/session/abi.rs:69` |
| `ok` | function | Creates a successful status value for `PksSessionStatus`. | `src/abi/session/abi.rs:62` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | Identifies the backend failure state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:93` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | Indicates that the operation was cancelled. | `src/abi/session/abi.rs:94` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | Identifies the foreign handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:90` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | Identifies the index out of range state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:95` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | Identifies the internal panic state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:87` |

## Executable evidence

Executable evidence selected for **C ABI ownership** is limited to each test's recorded setup and assertions:

- `given_full_table_when_insert_then_capacity_failure_is_returned` — given full table when insert then capacity failure is returned (`src/abi/session/handle.rs:148`; `test-1fd15e979f3cff1947b5`).
- `given_other_scope_when_lookup_then_foreign_handle_is_reported` — given other scope when lookup then foreign handle is reported (`src/abi/session/handle.rs:159`; `test-206ebd1f399506308ec7`).
- `given_removed_handle_when_lookup_then_stale_is_reported` — given removed handle when lookup then stale is reported (`src/abi/session/handle.rs:135`; `test-3f91a4691a62e3175273`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-f2e13a28f1aa591f3c67`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-8ba7b5b19e9d7dfbc464`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-9fb4684ff29b5ab716fd`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-002ce44230f2b0ac6d7c`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-657d1e2cbdcbd70cf5fa`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-f10bfad1b583316ad6fb`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` — given panicking abi bodies when guarded then panics are contained (`src/abi/codec.rs:373`; `test-a807dc7f3aad831eda7a`).
- `given_rejected_capacity_when_retried_then_encoder_state_is_unchanged` — given rejected capacity when retried then encoder state is unchanged (`src/abi/codec.rs:323`; `test-2f90a0a4149901522180`).
- `given_sine_440hz_when_round_trip_then_decoded_has_energy` — given sine 440hz when round trip then decoded has energy (`src/abi/codec.rs:435`; `test-073998688232a109f6e7`).

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

The claims on **C ABI ownership** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `include/pocketstation.h:7-27` (`DIRECT`)
- `src/abi/session/handle.rs:4-4` (`DIRECT`)
- `src/abi/session/handle.rs:5-8` (`DIRECT`)
- `src/abi/session/handle.rs:6-6` (`DIRECT`)
- `src/abi/session/handle.rs:7-7` (`DIRECT`)
- `src/abi/session/handle.rs:11-16` (`DIRECT`)
- `src/abi/session/handle.rs:19-19` (`DIRECT`)
- `src/abi/session/handle.rs:20-24` (`DIRECT`)
- `src/abi/session/handle.rs:21-21` (`DIRECT`)
- `src/abi/session/handle.rs:22-22` (`DIRECT`)
- `src/abi/session/handle.rs:23-23` (`DIRECT`)
- `src/abi/session/handle.rs:27-35` (`DIRECT`)
- `src/abi/session/handle.rs:37-51` (`DIRECT`)
- `src/abi/session/handle.rs:53-59` (`DIRECT`)
- `src/abi/session/handle.rs:61-67` (`DIRECT`)
- `src/abi/session/handle.rs:69-74` (`DIRECT`)
- `src/abi/session/handle.rs:76-81` (`DIRECT`)
- `src/abi/session/handle.rs:83-89` (`DIRECT`)
- `src/abi/session/handle.rs:91-101` (`DIRECT`)
- `src/abi/session/handle.rs:103-113` (`DIRECT`)
- `src/abi/session/handle.rs:115-126` (`DIRECT`)

For **C ABI ownership**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

# ABI and conformance model

<!-- claims: CLM-DOC-054-CAP-001,CLM-DOC-054-CAP-002,CLM-DOC-054-SOURCE-001 -->

## Scope

- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

The scope of **ABI and conformance model** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Ownership map

- `src/abi/mod.rs` owns part of this boundary.
- `src/conformance.rs` owns part of this boundary.

## Compiler-visible surface

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
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidArgument` | variant | Identifies the invalid argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:89` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidHandle` | variant | Identifies the invalid handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:84` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidLifecycleState` | variant | Identifies the invalid lifecycle state state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:91` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidStructSize` | variant | Identifies the invalid struct size state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:83` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::MisalignedPointer` | variant | Identifies the misaligned pointer state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:88` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::NoCapacity` | variant | Identifies the no capacity state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:86` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::NullArgument` | variant | Identifies the null argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:81` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::Ok` | variant | Indicates that the operation completed successfully. | `src/abi/session/abi.rs:80` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::StaleHandle` | variant | Identifies the stale handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:85` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMajor` | variant | Identifies the unsupported ABI major state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:82` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMinor` | variant | Identifies the unsupported ABI minor state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:96` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::WouldBlock` | variant | Identifies the would block state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:92` |
| `PksSessionStatus::code` | struct_field | Stores the code used by `PksSessionStatus`. | `src/abi/session/abi.rs:57` |
| `PksSessionStatus::detail` | struct_field | Stores the detail used by `PksSessionStatus`. | `src/abi/session/abi.rs:58` |

## Observed implementation patterns

- `typed_error` — `src/abi/extension.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `tests/public_api_boundary.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `src/abi/executable_extension.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `clock_correlation` — `benches/generated_audio_bridge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/executable_extension.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `benches/generated_audio_bridge.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/session/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/abi/session/conformance_fixture.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `buffer_pool` — `src/abi/session/mod.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `tests/public_api_boundary.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/session/conformance_fixture.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/session/error.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/session/handle.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).
- `typed_error` — `src/abi/session/runtime.rs` (`OBSERVED_IMPLEMENTATION_PATTERN`).

## Behavioral evidence

Executable evidence selected for **ABI and conformance model** is limited to each test's recorded setup and assertions:

- `abi_codec_cpp_conformance` — abi codec cpp conformance (`tests/abi_codec_cpp_conformance.cpp:1`; `test-544e66e8f85e1ad9e055`).
- `abi_session_c_conformance` — abi session c conformance (`tests/abi_session_c_conformance.c:1`; `test-9e1beea6279253161031`).
- `abi_session_c_success_conformance` — abi session c success conformance (`tests/abi_session_c_success_conformance.c:1`; `test-fbd5f1d6e0ff13895c92`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-f2e13a28f1aa591f3c67`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-8ba7b5b19e9d7dfbc464`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-9fb4684ff29b5ab716fd`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-002ce44230f2b0ac6d7c`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-657d1e2cbdcbd70cf5fa`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-f10bfad1b583316ad6fb`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` — given panicking abi bodies when guarded then panics are contained (`src/abi/codec.rs:373`; `test-a807dc7f3aad831eda7a`).
- `given_rejected_capacity_when_retried_then_encoder_state_is_unchanged` — given rejected capacity when retried then encoder state is unchanged (`src/abi/codec.rs:323`; `test-2f90a0a4149901522180`).
- `given_sine_440hz_when_round_trip_then_decoded_has_energy` — given sine 440hz when round trip then decoded has energy (`src/abi/codec.rs:435`; `test-073998688232a109f6e7`).

## Stability boundary

**ABI and conformance model** describes internal ownership. Its private module layout is not a compatibility promise; compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [C ABI reference](/docs/reference/c-abi.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [C ABI ownership](/docs/concepts/c-abi-ownership.md)

## Evidence boundary

The claims on **ABI and conformance model** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/abi/mod.rs:1-5` (`DIRECT`)
- `src/conformance.rs:1-1251` (`DIRECT`)

For **ABI and conformance model**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

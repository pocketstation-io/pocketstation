# Operate a Session through C

<!-- claims: CLM-GUIDE-019-CAP-001,CLM-GUIDE-019-CAP-002,CLM-GUIDE-019-SOURCE-001 -->

## Scope

- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Include pocketstation.h and use its ABI version.
2. Create handles through exported functions.
3. Check every PksSessionStatus.
4. Stop before releasing runtime ownership.
5. Release each handle with its matching ABI function.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::abi::session::abi::PksSessionStatus` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:56` |
| `pocketstation::abi::session::abi::PksSessionUtf8` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:101` |
| `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:79` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:93` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:94` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:90` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:95` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:87` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidArgument` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:89` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidHandle` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:84` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidLifecycleState` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:91` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidStructSize` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:83` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::MisalignedPointer` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:88` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::NoCapacity` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:86` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::NullArgument` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:81` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::Ok` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:80` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_deterministic_session_when_polled_then_audio_lease_is_bounded_and_stable` — given deterministic session when polled then audio lease is bounded and stable (`src/abi/session/mod.rs:1036`; `test-16c3dd3ac223381ec20a`).
- `given_native_engine_when_created_then_real_session_declaration_compiles` — given native engine when created then real session declaration compiles (`src/abi/session/mod.rs:929`; `test-ec0f9d1e4ec547217e7b`).
- `abi_session_c_conformance` — abi session c conformance (`tests/abi_session_c_conformance.c:1`; `test-1ab6697ee6c783b1c41b`).
- `abi_session_c_metrics_canary` — abi session c metrics canary (`tests/abi_session_c_metrics_canary.c:1`; `test-fe902c5e04fb9d6f128e`).
- `abi_session_c_success_conformance` — abi session c success conformance (`tests/abi_session_c_success_conformance.c:1`; `test-a2314a88cf28de25b331`).
- `given_fixture_session_when_started_then_two_stems_cross_canonical_engine` — given fixture session when started then two stems cross canonical engine (`tests/conformance_fixture.rs:14`; `test-bec1e3cd7f059a144101`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-041037b5b9482d79c8e2`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-d3686b94180b732c8001`).

## Failure signals

No domain-specific error record is assigned. Preserve the returned error and use the general error index.

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/abi_session_c_success_conformance.c:1-191` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

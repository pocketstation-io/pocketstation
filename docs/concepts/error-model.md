# Error and status model

<!-- claims: CLM-DOC-034-CAP-001,CLM-DOC-034-SOURCE-001 -->

Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

## Scope

- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::authorization::CaptureError::BackendStatus` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:303` |
| `authorization::CaptureError::BackendStatus::operation` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:304` |
| `authorization::CaptureError::BackendStatus::status_code` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:305` |
| `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| `pocketstation::session::error_code::PolledAudioPollErrorCode` | enum | Stable language-neutral code for bounded polled-audio status. | `src/session/error_code.rs:131` |
| `pocketstation::session::error_code::SessionDeclarationErrorCode` | enum | Stable language-neutral code for a Session declaration failure. | `src/session/error_code.rs:10` |
| `pocketstation::session::error_code::SessionRuntimeErrorCode` | enum | Stable language-neutral code for a running-Session projection failure. | `src/session/error_code.rs:116` |
| `pocketstation::session::error_code::SessionStartErrorCode` | enum | Stable language-neutral code for Session startup. | `src/session/error_code.rs:61` |
| `pocketstation::session::error_code::SessionStopCode` | enum | Stable language-neutral status for an idempotent Session stop. | `src/session/error_code.rs:150` |
| `pocketstation::session::error_code::SessionStopFailureCode` | enum | Stable language-neutral cause retained by a failed Session stop. | `src/session/error_code.rs:171` |
| `pocketstation::abi::session::abi::PksSessionStatus` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:56` |
| `pocketstation::connector::configuration::ConnectorConfigurationError` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/configuration.rs:608` |
| `pocketstation::connector::error::ConnectorError` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/error.rs:80` |
| `pocketstation::connector::error::ConnectorErrorCode` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/error.rs:10` |
| `pocketstation::connector::status::ConnectorServiceStatus` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/connector/status.rs:30` |
| `pocketstation::native_extension::NativeExtensionLibraryError` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/native_extension/mod.rs:124` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteError` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/buffer.rs:305` |
| `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/abi/session/abi.rs:79` |
| `pocketstation::capture::authorization::CaptureError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:290` |
| `pocketstation::codec::decoder::OpusDecodeError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/codec/decoder.rs:25` |

## Where you encounter it

- **Load a compiled extension** — Load a trusted absolute library path and import its registrations transactionally.
- **Bind through C** — Create and operate a Session through ABI handles, status codes, and versioned callbacks.
- **Diagnose a running Session** — Correlate events, metrics, trace records, stable error codes, and terminal outcomes.
- **Encode and decode a stream** — Configure Opus state and convert audio frames to packets and back.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_zero_capacity_when_stream_is_created_then_error_is_returned` — given zero capacity when stream is created then error is returned (`src/capture/frame_stream.rs:226`; `test-ca400782cee3b7be9d5b`).
- `given_process_mode_when_node_not_found_then_backend_init_error_not_system_mix` — given process mode when node not found then backend init error not system mix (`src/capture/platform/linux/pipewire.rs:2131`; `test-d95b10aa2227cf4f9ffb`).
- `given_missing_asp_when_required_then_sdk_returns_actionable_typed_error` — given missing asp when required then sdk returns actionable typed error (`src/capture/platform/macos/loopback.rs:322`; `test-c5e3e40b26c76972b601`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` — given core audio permission status when mapped then denial remains typed (`src/capture/platform/macos/macos_tap.rs:699`; `test-afb8b00e7e6a55f6d16d`).
- `given_other_core_audio_status_when_mapped_then_raw_status_is_preserved` — given other core audio status when mapped then raw status is preserved (`src/capture/platform/macos/macos_tap.rs:737`; `test-02f8243c416d85f0ef0c`).
- `given_worker_error_when_waiting_then_exact_failure_is_retained` — given worker error when waiting then exact failure is retained (`src/capture/platform/windows/open_lifecycle.rs:141`; `test-8e32e7225c69ed396e55`).
- `given_capture_error_not_supported_when_displayed_then_contains_not_supported` — given capture error not supported when displayed then contains not supported (`src/capture/tests.rs:193`; `test-676aa99d320d214bccad`).
- `given_mode_unsupported_error_when_displayed_then_contains_not_supported` — given mode unsupported error when displayed then contains not supported (`src/capture/tests.rs:263`; `test-ef57017ec533dcedac64`).
- `given_runtime_event_when_sent_then_exact_identity_and_platform_status_are_retained` — given runtime event when sent then exact identity and platform status are retained (`src/capture/tests.rs:18`; `test-11b326c09cc37ec133a0`).
- `given_source_unavailable_error_when_displayed_then_stable_identity_is_retained` — given source unavailable error when displayed then stable identity is retained (`src/capture/tests.rs:256`; `test-d9515c41464fa15374fd`).
- `given_20ms_decoder_when_60ms_concealment_is_requested_then_typed_bound_error_is_returned` — given 20ms decoder when 60ms concealment is requested then typed bound error is returned (`src/codec/decoder.rs:246`; `test-f2b28e6d34edfbf95af0`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Terminal outcomes](/docs/lifecycle/terminal-outcomes.md)
- [Configure connector secrets](/docs/how-to/configure-connector-secrets.md)
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md)
- [Operate a Session through C](/docs/how-to/use-c-session-api.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/error_code.rs:1-81` (`DIRECT`)
- `src/session/error_code.rs:1-544` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

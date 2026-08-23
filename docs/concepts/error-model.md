# Error and status model

<!-- claims: CLM-DOC-034-CAP-001,CLM-DOC-034-SOURCE-001 -->

## What it is

PocketStation exposes typed Rust failures and stable cross-boundary error or status codes. Error stages and terminal outcomes retain which operation and component failed.

## Why it exists

Text alone cannot support reliable branching or cross-language compatibility. Typed variants and stable codes let callers distinguish declaration, preparation, runtime, stop, and finalization failures.

## Relationships

- Rust APIs return owning error types.
- Error-code mappings preserve selected classes at ABI and persisted boundaries.
- Terminal outcomes can contain multiple component failures after partial progress.

## Invariants and guarantees

- An error name does not establish retry safety.
- The primary failure is preserved when rollback also fails.
- Unknown retry, recovery, or fatality remains explicitly undocumented rather than guessed.

## When you encounter it

- **Load a compiled extension** — Load a trusted absolute library path and import its registrations transactionally.
- **Bind through C** — Create and operate a Session through ABI handles, status codes, and versioned callbacks.
- **Diagnose a running Session** — Correlate events, metrics, trace records, stable error codes, and terminal outcomes.
- **Encode and decode a stream** — Configure Opus state and convert audio frames to packets and back.

## Use it

- [Error code index](/docs/reference/error-codes.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Error references](/docs/errors/session.md)

## Scope

- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Error and status model** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| `pocketstation::session::error_code::PolledAudioPollErrorCode` | enum | Stable language-neutral code for bounded polled-audio status. | `src/session/error_code.rs:131` |
| `pocketstation::session::error_code::SessionDeclarationErrorCode` | enum | Stable language-neutral code for a Session declaration failure. | `src/session/error_code.rs:10` |
| `pocketstation::session::error_code::SessionRuntimeErrorCode` | enum | Stable language-neutral code for a running-Session projection failure. | `src/session/error_code.rs:116` |
| `pocketstation::session::error_code::SessionStartErrorCode` | enum | Stable language-neutral code for Session startup. | `src/session/error_code.rs:61` |
| `pocketstation::session::error_code::SessionStopCode` | enum | Stable language-neutral status for an idempotent Session stop. | `src/session/error_code.rs:150` |
| `pocketstation::session::error_code::SessionStopFailureCode` | enum | Stable language-neutral cause retained by a failed Session stop. | `src/session/error_code.rs:171` |
| `pocketstation::recording::error_code::recording_outcome_error_code` | function | Returns the recording outcome error code held by `error_code`. | `src/recording/error_code.rs:82` |
| `pocketstation::session::error_code::polled_audio_poll_error_code` | function | Returns the polled audio poll error code held by `error_code`. | `src/session/error_code.rs:255` |
| `pocketstation::session::error_code::session_declaration_error_code` | function | Returns the session declaration error code held by `error_code`. | `src/session/error_code.rs:195` |

## Executable evidence

Executable evidence selected for **Error and status model** is limited to each test's recorded setup and assertions:

- `given_facade_errors_when_mapped_then_codes_use_canonical_session_vocabulary` — given facade errors when mapped then codes use canonical session vocabulary (`src/error_code.rs:37`; `test-f698d35649fa52eabf28`).
- `given_declaration_errors_when_mapped_then_every_variant_has_a_stable_code` — given declaration errors when mapped then every variant has a stable code (`src/session/error_code.rs:382`; `test-329bc302c00850a37487`).
- `given_polled_audio_failures_when_mapped_then_every_status_is_preserved` — given polled audio failures when mapped then every status is preserved (`src/session/error_code.rs:524`; `test-36b002bb02a82e639fa5`).
- `given_reexported_codes_when_serialized_then_canonical_values_are_unchanged` — given reexported codes when serialized then canonical values are unchanged (`src/error_code.rs:66`; `test-0168dcd4d2cda0ea4fab`).
- `given_stable_code_vocabulary_when_serialized_then_values_are_unique_and_namespaced` — given stable code vocabulary when serialized then values are unique and namespaced (`src/session/error_code.rs:319`; `test-9dbe226d688b419c44dd`).
- `given_start_and_capture_failures_when_mapped_then_specific_classes_are_preserved` — given start and capture failures when mapped then specific classes are preserved (`src/session/error_code.rs:470`; `test-e5e2a976b704c1bcb17d`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-1e4368d9ab7990c79bd7`).
- `given_zero_capacity_when_stream_is_created_then_error_is_returned` — given zero capacity when stream is created then error is returned (`src/capture/frame_stream.rs:226`; `test-ca400782cee3b7be9d5b`).
- `given_process_mode_when_node_not_found_then_backend_init_error_not_system_mix` — given process mode when node not found then backend init error not system mix (`src/capture/platform/linux/pipewire.rs:2131`; `test-d95b10aa2227cf4f9ffb`).
- `given_missing_asp_when_required_then_sdk_returns_actionable_typed_error` — given missing asp when required then sdk returns actionable typed error (`src/capture/platform/macos/loopback.rs:322`; `test-c5e3e40b26c76972b601`).
- `given_core_audio_permission_status_when_mapped_then_denial_remains_typed` — given core audio permission status when mapped then denial remains typed (`src/capture/platform/macos/macos_tap.rs:699`; `test-afb8b00e7e6a55f6d16d`).
- `given_other_core_audio_status_when_mapped_then_raw_status_is_preserved` — given other core audio status when mapped then raw status is preserved (`src/capture/platform/macos/macos_tap.rs:737`; `test-02f8243c416d85f0ef0c`).

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

The claims on **Error and status model** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/error_code.rs:1-81` (`DIRECT`)
- `src/session/error_code.rs:1-544` (`DIRECT`)

For **Error and status model**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

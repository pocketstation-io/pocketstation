# Recording failures

<!-- claims: CLM-ERR-005-CAP-001,CLM-ERR-005-CAP-002,CLM-ERR-005-SOURCE-001,CLM-ERR-005-ERROR-0001,CLM-ERR-005-ERROR-0002,CLM-ERR-005-ERROR-0003,CLM-ERR-005-ERROR-0004,CLM-ERR-005-ERROR-0005,CLM-ERR-005-ERROR-0006,CLM-ERR-005-ERROR-0007,CLM-ERR-005-ERROR-0008,CLM-ERR-005-ERROR-0009,CLM-ERR-005-ERROR-0010,CLM-ERR-005-ERROR-0011,CLM-ERR-005-ERROR-0012,CLM-ERR-005-ERROR-0013,CLM-ERR-005-ERROR-0014,CLM-ERR-005-ERROR-0015,CLM-ERR-005-ERROR-0016,CLM-ERR-005-ERROR-0017,CLM-ERR-005-ERROR-0018,CLM-ERR-005-ERROR-0019,CLM-ERR-005-ERROR-0020 -->

## Scope

- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Error inventory

| Evidence record | Type | Variant | Retryable | Recoverable | Defined |
|---|---|---|---|---|---|
| error-059bf10da1dcb4446e68 | `pocketstation::recording::error_code::RecordingErrorCode` | `PermissionDenied` | unknown | unknown | `src/recording/error_code.rs:14` |
| error-21234ee4f29a9e34b0bb | `pocketstation::recording::error_code::RecordingErrorCode` | `WorkerPanicked` | unknown | unknown | `src/recording/error_code.rs:23` |
| error-3a471ee2980086cb3c21 | `pocketstation::recording::error_code::RecordingErrorCode` | `InvalidSampleSpec` | unknown | unknown | `src/recording/error_code.rs:15` |
| error-52042798a8fdc6235e75 | `pocketstation::recording::error_code::RecordingErrorCode` | `TimestampOutOfRange` | unknown | unknown | `src/recording/error_code.rs:20` |
| error-55e96f54549d3c6692a0 | `pocketstation::recording::error_code::RecordingErrorCode` | `GapTooLarge` | unknown | unknown | `src/recording/error_code.rs:21` |
| error-5c68bda251d1ed4bf3f4 | `pocketstation::recording::error_code::RecordingErrorCode` | `LineageMismatch` | unknown | unknown | `src/recording/error_code.rs:17` |
| error-5fed2e26c75a54168d44 | `pocketstation::recording::error_code::RecordingErrorCode` | `SessionMismatch` | unknown | unknown | `src/recording/error_code.rs:13` |
| error-62ee16293028c5b5b127 | `pocketstation::recording::error_code::RecordingErrorCode` | `WavFailed` | unknown | unknown | `src/recording/error_code.rs:25` |
| error-6a4ff98828200ff939ad | `pocketstation::recording::error_code::RecordingErrorCode` | `TooManyGaps` | unknown | unknown | `src/recording/error_code.rs:22` |
| error-6b6cbb4eeeaa8aa11dbb | `pocketstation::recording::error_code::RecordingErrorCode` | `SourceMismatch` | unknown | unknown | `src/recording/error_code.rs:16` |
| error-6c1d4fc88f2ff6a85b7e | `pocketstation::recording::error_code::RecordingErrorCode` | `OutputExists` | unknown | unknown | `src/recording/error_code.rs:10` |
| error-72a4fdecd5d7cf9cb8ba | `pocketstation::recording::error_code::RecordingErrorCode` | `DuplicateStemLabel` | unknown | unknown | `src/recording/error_code.rs:12` |
| error-7ade9991dbb5b2589480 | `pocketstation::recording::error_code::RecordingErrorCode` | `IoFailed` | unknown | unknown | `src/recording/error_code.rs:24` |
| error-8a0700f833493202b5e6 | `pocketstation::recording::error_code::RecordingErrorCode` | `JsonFailed` | unknown | unknown | `src/recording/error_code.rs:26` |
| error-a991ad845c6aba7b4daf | `pocketstation::recording::error_code::RecordingErrorCode` | `UnalignedSamples` | unknown | unknown | `src/recording/error_code.rs:19` |
| error-c0f84ef8e6d8a86ea475 | `pocketstation::recording::error_code::RecordingErrorCode` | `FrameSpecMismatch` | unknown | unknown | `src/recording/error_code.rs:18` |
| error-d2887b41b2617f598c09 | `pocketstation::recording::error_code::RecordingErrorCode` | type | unknown | unknown | `src/recording/error_code.rs:9` |
| error-d72ab61be39f8f8a3418 | `pocketstation::recording::error_code::RecordingErrorCode` | `InvalidStemLabel` | unknown | unknown | `src/recording/error_code.rs:11` |
| error-dd3dcbdb55252f9f99d3 | `pocketstation::recording::error_code::RecordingErrorCode` | `NotFinalized` | unknown | unknown | `src/recording/error_code.rs:27` |
| error-f652b40f3d3653825e30 | `pocketstation::recording::error_code::RecordingErrorCode` | `Incomplete` | unknown | unknown | `src/recording/error_code.rs:28` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A recording is incomplete](/docs/troubleshooting/recording-incomplete.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/recording/writer.rs:1-1248` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

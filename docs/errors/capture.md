# Capture failures

<!-- claims: CLM-ERR-002-CAP-001,CLM-ERR-002-CAP-002,CLM-ERR-002-CAP-003,CLM-ERR-002-CAP-004,CLM-ERR-002-CAP-005,CLM-ERR-002-CAP-006,CLM-ERR-002-SOURCE-001,CLM-ERR-002-ERROR-0001,CLM-ERR-002-ERROR-0002,CLM-ERR-002-ERROR-0003,CLM-ERR-002-ERROR-0004,CLM-ERR-002-ERROR-0005,CLM-ERR-002-ERROR-0006,CLM-ERR-002-ERROR-0007,CLM-ERR-002-ERROR-0008,CLM-ERR-002-ERROR-0009,CLM-ERR-002-ERROR-0010,CLM-ERR-002-ERROR-0011,CLM-ERR-002-ERROR-0012,CLM-ERR-002-ERROR-0013,CLM-ERR-002-ERROR-0014,CLM-ERR-002-ERROR-0015,CLM-ERR-002-ERROR-0016 -->

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Error inventory

| Evidence record | Type | Variant | Retryable | Recoverable | Defined |
|---|---|---|---|---|---|
| error-11b972ad42d5de880e06 | `pocketstation::capture::events::CaptureRuntimeFailure` | type | unknown | unknown | `src/capture/events.rs:47` |
| error-29e952ae7432566a9e95 | `pocketstation::capture::events::CaptureRuntimeFailureClass` | `BackendClass` | unknown | unknown | `src/capture/events.rs:43` |
| error-365f9b6fbda74eb0d631 | `pocketstation::capture::authorization::CaptureError` | `CaptureWorkerPanicked` | unknown | unknown | `src/capture/authorization.rs:316` |
| error-38030156125346a8e892 | `pocketstation::capture::authorization::CaptureError` | `PermissionDenied` | unknown | unknown | `src/capture/authorization.rs:301` |
| error-3b4b5393164d9f6f12a5 | `pocketstation::capture::authorization::CaptureError` | `NotSupported` | unknown | unknown | `src/capture/authorization.rs:292` |
| error-3c6fcc22deb2f54788ba | `pocketstation::capture::events::CaptureRuntimeFailureClass` | `PlatformStatus` | unknown | unknown | `src/capture/events.rs:42` |
| error-71c87f975acc9e22a402 | `pocketstation::capture::authorization::CaptureError` | `SourceUnavailable` | unknown | unknown | `src/capture/authorization.rs:308` |
| error-8db0fec69a9c7158ffdf | `pocketstation::capture::authorization::CaptureError` | `BackendSetupRequired` | unknown | unknown | `src/capture/authorization.rs:296` |
| error-96ffe4bc4254583d1e17 | `pocketstation::capture::authorization::CaptureError` | type | unknown | unknown | `src/capture/authorization.rs:290` |
| error-a9c0f7dfff744e9ba6b7 | `pocketstation::capture::events::CaptureRuntimeFailureClass` | `SourceInstanceExited` | unknown | unknown | `src/capture/events.rs:41` |
| error-b320ea1cba2b3c8dc4c7 | `pocketstation::capture::authorization::CaptureError` | `BackendInit` | unknown | unknown | `src/capture/authorization.rs:294` |
| error-bcf5d4d897b6bd0784bf | `pocketstation::capture::authorization::CaptureError` | `InvalidStreamCapacity` | unknown | unknown | `src/capture/authorization.rs:312` |
| error-bf1be2fb486df6136dc5 | `pocketstation::capture::authorization::CaptureError` | `ModeUnsupported` | unknown | unknown | `src/capture/authorization.rs:310` |
| error-ceedf8c06740748c9bd5 | `pocketstation::capture::authorization::CaptureError` | `InvalidRuntimeEventCapacity` | unknown | unknown | `src/capture/authorization.rs:314` |
| error-e8046b5b5989518ee482 | `pocketstation::capture::authorization::CaptureError` | `BackendStatus` | unknown | unknown | `src/capture/authorization.rs:303` |
| error-ea2d5a94280522f41764 | `pocketstation::capture::events::CaptureRuntimeFailureClass` | type | unknown | unknown | `src/capture/events.rs:40` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Capture API](/docs/reference/capture.md)
- [Permission state is denied or unobservable](/docs/troubleshooting/permission-state.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/mod.rs:1-65` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

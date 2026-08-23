# Runtime and sidecar failures

<!-- claims: CLM-ERR-009-CAP-001,CLM-ERR-009-CAP-002,CLM-ERR-009-CAP-003,CLM-ERR-009-CAP-004,CLM-ERR-009-SOURCE-001,CLM-ERR-009-ERROR-0001,CLM-ERR-009-ERROR-0002,CLM-ERR-009-ERROR-0003,CLM-ERR-009-ERROR-0004,CLM-ERR-009-ERROR-0005,CLM-ERR-009-ERROR-0006,CLM-ERR-009-ERROR-0007,CLM-ERR-009-ERROR-0008,CLM-ERR-009-ERROR-0009,CLM-ERR-009-ERROR-0010,CLM-ERR-009-ERROR-0011,CLM-ERR-009-ERROR-0012,CLM-ERR-009-ERROR-0013,CLM-ERR-009-ERROR-0014,CLM-ERR-009-ERROR-0015,CLM-ERR-009-ERROR-0016,CLM-ERR-009-ERROR-0017,CLM-ERR-009-ERROR-0018,CLM-ERR-009-ERROR-0019,CLM-ERR-009-ERROR-0020,CLM-ERR-009-ERROR-0021,CLM-ERR-009-ERROR-0022,CLM-ERR-009-ERROR-0023,CLM-ERR-009-ERROR-0024,CLM-ERR-009-ERROR-0025,CLM-ERR-009-ERROR-0026,CLM-ERR-009-ERROR-0027,CLM-ERR-009-ERROR-0028,CLM-ERR-009-ERROR-0029,CLM-ERR-009-ERROR-0030,CLM-ERR-009-ERROR-0031,CLM-ERR-009-ERROR-0032,CLM-ERR-009-ERROR-0033,CLM-ERR-009-ERROR-0034,CLM-ERR-009-ERROR-0035,CLM-ERR-009-ERROR-0036,CLM-ERR-009-ERROR-0037,CLM-ERR-009-ERROR-0038,CLM-ERR-009-ERROR-0039,CLM-ERR-009-ERROR-0040,CLM-ERR-009-ERROR-0041,CLM-ERR-009-ERROR-0042,CLM-ERR-009-ERROR-0043 -->

## Scope

- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.
- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.
- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Error inventory

| Evidence record | Type | Variant | Retryable | Recoverable | Defined |
|---|---|---|---|---|---|
| error-143cce14f0e71f68c4cf | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `InvalidMagic` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:298` |
| error-19eabd878a9188bf94ce | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Wait` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:726` |
| error-1d9b879cab06d8598907 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `ReservedFieldSet` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:308` |
| error-201cc7749bdbbd671d69 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `InvalidTerminal` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:306` |
| error-23eba8b87dea81473095 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `FrameLengthOverflow` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:320` |
| error-2b03bbb58bb17d9482da | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `UnknownMessageKind` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:304` |
| error-3636f110b3c505b0fc87 | `pocketstation::runtime::audio::executor::ExecError` | `Node` | unknown | unknown | `src/runtime/audio/executor.rs:22` |
| error-3a3e737bfe0585596712 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `ProcessingTimeout` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:717` |
| error-3dbf0292e22bf7695a5b | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `UnexpectedMessage` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:710` |
| error-3f7304dbb0de0fe37726 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `InvalidDataKind` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:724` |
| error-3fbce6034564f1a51e83 | `pocketstation::runtime::audio::runner::PlanRunnerError` | `ZeroSourceCapacity` | unknown | unknown | `src/runtime/audio/runner.rs:258` |
| error-4c396f1ad9633a15e4c4 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `UnknownSidecar` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:732` |
| error-4cd7a5440dde80383b2e | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | type | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:686` |
| error-4f7cc30b74223a1354c0 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Protocol` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:698` |
| error-51f341e5e95d92745cc7 | `pocketstation::runtime::audio::runner::PlanRunnerError` | `DuplicateSource` | unknown | unknown | `src/runtime/audio/runner.rs:260` |
| error-56b76ee666f183f18d1c | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `InvalidConfiguration` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:688` |
| error-59c0b276f329f504019c | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `ControlQueueFull` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:704` |
| error-64b40c037850805370f3 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `UnexpectedEof` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:708` |
| error-6b989a5c2a96463f3bee | `pocketstation::runtime::audio::runner::PlanRunnerError` | `AlreadyFinished` | unknown | unknown | `src/runtime/audio/runner.rs:264` |
| error-7c61de949e6f7c062440 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | type | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:292` |
| error-8c284a4a0efd542eb004 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `FrameTooLarge` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:700` |
| error-8d44c749b736491e2485 | `pocketstation::runtime::audio::runner::PlanRunnerError` | `ZeroWorkBudget` | unknown | unknown | `src/runtime/audio/runner.rs:262` |
| error-9ba468b27c25e5eb7e82 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `AlreadyReaped` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:730` |
| error-9d624a622d7040a1fe0f | `pocketstation::runtime::audio::runner::PlanRunnerError` | type | unknown | unknown | `src/runtime/audio/runner.rs:256` |
| error-a13edd0adb172f70699e | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Spawn` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:690` |
| error-a40cc696fa1557b8b562 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `MissingPipe` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:694` |
| error-a984218b068597ea76b4 | `pocketstation::runtime::audio::runner::PlanRunnerError` | `Execution` | unknown | unknown | `src/runtime/audio/runner.rs:266` |
| error-acee01765fe764bfd55c | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `InvalidState` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:719` |
| error-ae3ce028fb5d1b747b4f | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Timeout` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:715` |
| error-b0bb2cce36e6f3a9aa85 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `UnsupportedMajor` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:300` |
| error-c11beeb8e6aa548db6bf | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `EmptySignalId` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:310` |
| error-c8da30f10b161f3331fa | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `Truncated` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:294` |
| error-d1c44816ab5c6bbb5dd3 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `TrailingBytes` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:296` |
| error-d2b53989cb5e8e167f9c | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `FrameTooLarge` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:322` |
| error-dafcbf188eb0f57e0c65 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Closed` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:706` |
| error-e0093a3c3b8e27256dd2 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `UnsupportedMinor` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:302` |
| error-e4672d7e92fa5cb1d206 | `pocketstation::runtime::audio::executor::ExecError` | type | unknown | unknown | `src/runtime/audio/executor.rs:20` |
| error-e5d0b8eacb45ae5e003a | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Kill` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:728` |
| error-eee3fd27c0031552e3e9 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `Io` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:696` |
| error-efbb8501a0345d5c733e | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `FieldTooLarge` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:312` |
| error-f662eed9eaef230d5e97 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `ThreadSpawn` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:692` |
| error-fd6557db957f52eb4959 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError` | `DataQueueFull` | unknown | unknown | `src/runtime/lifecycle/sidecar_host.rs:702` |
| error-ff0c5d9bc0446db9f66b | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError` | `InvalidUtf8` | unknown | unknown | `src/runtime/lifecycle/sidecar_protocol.rs:318` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Configuration reference](/docs/reference/configuration.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/runtime/lifecycle/sidecar_host.rs:1-734` (`DIRECT`)
- `src/runtime/signal/error.rs:1-56` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

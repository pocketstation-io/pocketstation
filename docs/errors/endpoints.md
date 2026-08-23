# Endpoint failures

<!-- claims: CLM-ERR-004-CAP-001,CLM-ERR-004-CAP-002,CLM-ERR-004-CAP-003,CLM-ERR-004-SOURCE-001,CLM-ERR-004-ERROR-0001,CLM-ERR-004-ERROR-0002,CLM-ERR-004-ERROR-0003,CLM-ERR-004-ERROR-0004,CLM-ERR-004-ERROR-0005,CLM-ERR-004-ERROR-0006,CLM-ERR-004-ERROR-0007,CLM-ERR-004-ERROR-0008,CLM-ERR-004-ERROR-0009,CLM-ERR-004-ERROR-0010,CLM-ERR-004-ERROR-0011,CLM-ERR-004-ERROR-0012,CLM-ERR-004-ERROR-0013,CLM-ERR-004-ERROR-0014,CLM-ERR-004-ERROR-0015,CLM-ERR-004-ERROR-0016,CLM-ERR-004-ERROR-0017,CLM-ERR-004-ERROR-0018,CLM-ERR-004-ERROR-0019,CLM-ERR-004-ERROR-0020,CLM-ERR-004-ERROR-0021,CLM-ERR-004-ERROR-0022,CLM-ERR-004-ERROR-0023,CLM-ERR-004-ERROR-0024,CLM-ERR-004-ERROR-0025,CLM-ERR-004-ERROR-0026,CLM-ERR-004-ERROR-0027,CLM-ERR-004-ERROR-0028,CLM-ERR-004-ERROR-0029,CLM-ERR-004-ERROR-0030,CLM-ERR-004-ERROR-0031,CLM-ERR-004-ERROR-0032,CLM-ERR-004-ERROR-0033,CLM-ERR-004-ERROR-0034,CLM-ERR-004-ERROR-0035 -->

## Scope

- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Error inventory

| Evidence record | Type | Variant | Retryable | Recoverable | Defined |
|---|---|---|---|---|---|
| error-0265bb447764629fa47b | `pocketstation::endpoint::runtime::EndpointFailureStage` | `CancelPreparation` | unknown | unknown | `src/endpoint/runtime.rs:158` |
| error-0370b7ecbdf2b9d6fbdb | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | `ZeroLeaseCapacity` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:46` |
| error-0bed26cd5cd9ccfe0b20 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | `QueueCapacityTooLarge` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:48` |
| error-0db6114718e1d213362f | `pocketstation::endpoint::registry::EndpointDriverRegistryError` | `OperatorNodeTypeConflict` | unknown | unknown | `src/endpoint/registry.rs:31` |
| error-1d54a56031f21d638e8a | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | `LeaseCapacityExhausted` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:78` |
| error-1fdd7e0417ea75e9688a | `pocketstation::endpoint::runtime::EndpointFailureStage` | type | unknown | unknown | `src/endpoint/runtime.rs:156` |
| error-21860e8a08d6660b2cd4 | `pocketstation::endpoint::runtime::EndpointFailure` | type | unknown | unknown | `src/endpoint/runtime.rs:174` |
| error-25cba0c2435c181a17c1 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | `Empty` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:76` |
| error-29386b30a0d8753119c2 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | `StatePoisoned` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:80` |
| error-30d87c511b03582515cf | `pocketstation::endpoint::runtime::EndpointStartFailure` | type | unknown | unknown | `src/endpoint/runtime.rs:443` |
| error-323cf47f273c3dd6cfc8 | `pocketstation::endpoint::registry::EndpointPrepareError` | `Driver` | unknown | unknown | `src/endpoint/registry.rs:50` |
| error-39aa567a66c392fdb792 | `pocketstation::endpoint::runtime::EndpointStartFailureCause` | `Driver` | unknown | unknown | `src/endpoint/runtime.rs:440` |
| error-509735924e9985ade727 | `pocketstation::endpoint::runtime::EndpointStartFailureCause` | `GateAlreadyOpen` | unknown | unknown | `src/endpoint/runtime.rs:439` |
| error-50e99d7e6e503b1ebe48 | `pocketstation::endpoint::registry::EndpointPrepareError` | `NotRegistered` | unknown | unknown | `src/endpoint/registry.rs:45` |
| error-5175539560395b69dd7b | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | `BatchCapacityTooLarge` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:50` |
| error-60c7ad4dd8970fcf2852 | `pocketstation::endpoint::runtime::EndpointFailureRetryability` | `Never` | unknown | unknown | `src/endpoint/runtime.rs:167` |
| error-638df5cc5f4c88e5b73b | `pocketstation::endpoint::runtime::EndpointFailureStage` | `RequestStop` | unknown | unknown | `src/endpoint/runtime.rs:160` |
| error-6632caa79656f7747822 | `pocketstation::endpoint::registry::EndpointDriverRegistryError` | `EmptyNodeTypeId` | unknown | unknown | `src/endpoint/registry.rs:20` |
| error-67b368b13e2c4448fa12 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError` | type | unknown | unknown | `src/endpoint/polled_audio_driver.rs:74` |
| error-8389e526c053c0f4878c | `pocketstation::endpoint::runtime::EndpointFailureStage` | `Start` | unknown | unknown | `src/endpoint/runtime.rs:159` |
| error-8a777f9c916e634dee4f | `pocketstation::endpoint::registry::EndpointPrepareError` | type | unknown | unknown | `src/endpoint/registry.rs:39` |
| error-923a27d9164208d2c8d2 | `pocketstation::endpoint::runtime::EndpointFailureRetryability` | `ReconfigurationRequired` | unknown | unknown | `src/endpoint/runtime.rs:169` |
| error-9db01b1164f402cb50a1 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | `ZeroQueueCapacity` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:42` |
| error-9ebf2d2509e9920eb42d | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | `ZeroBatchCapacity` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:44` |
| error-abc7bb88845d6b422d1e | `pocketstation::endpoint::registry::EndpointDriverRegistryError` | `Duplicate` | unknown | unknown | `src/endpoint/registry.rs:24` |
| error-b190ec77abf54ff75844 | `pocketstation::endpoint::runtime::EndpointFailureStage` | `Prepare` | unknown | unknown | `src/endpoint/runtime.rs:157` |
| error-bca09f58f09981e71ec2 | `pocketstation::endpoint::registry::EndpointDriverRegistryError` | `EmptyOperatorId` | unknown | unknown | `src/endpoint/registry.rs:18` |
| error-bcca5f940d504641a87b | `pocketstation::endpoint::runtime::EndpointStartFailureCause` | type | unknown | unknown | `src/endpoint/runtime.rs:438` |
| error-d76e9a2c6b0bdb9cb30d | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | type | unknown | unknown | `src/endpoint/polled_audio_driver.rs:40` |
| error-d915ebe38a122bcad540 | `pocketstation::endpoint::registry::EndpointDriverRegistryError` | type | unknown | unknown | `src/endpoint/registry.rs:16` |
| error-e154f62273e1c62f6c49 | `pocketstation::endpoint::runtime::EndpointFailureRetryability` | type | unknown | unknown | `src/endpoint/runtime.rs:166` |
| error-e504c315c84e8e9f5177 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError` | `LeaseCapacityTooLarge` | unknown | unknown | `src/endpoint/polled_audio_driver.rs:52` |
| error-f3ec89fbfa429d63d867 | `pocketstation::endpoint::registry::EndpointPrepareError` | `EmptyBatch` | unknown | unknown | `src/endpoint/registry.rs:41` |
| error-fc14dba4d0d801d1be2a | `pocketstation::endpoint::runtime::EndpointFailureRetryability` | `Retryable` | unknown | unknown | `src/endpoint/runtime.rs:168` |
| error-fdca9d071b698a089f5f | `pocketstation::endpoint::runtime::EndpointFailureStage` | `JoinFinalize` | unknown | unknown | `src/endpoint/runtime.rs:161` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Endpoint API](/docs/reference/endpoints.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/endpoint/runtime.rs:1-531` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

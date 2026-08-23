# Connector failures

<!-- claims: CLM-ERR-006-CAP-001,CLM-ERR-006-CAP-002,CLM-ERR-006-CAP-003,CLM-ERR-006-SOURCE-001,CLM-ERR-006-ERROR-0001,CLM-ERR-006-ERROR-0002,CLM-ERR-006-ERROR-0003,CLM-ERR-006-ERROR-0004,CLM-ERR-006-ERROR-0005,CLM-ERR-006-ERROR-0006,CLM-ERR-006-ERROR-0007,CLM-ERR-006-ERROR-0008,CLM-ERR-006-ERROR-0009,CLM-ERR-006-ERROR-0010,CLM-ERR-006-ERROR-0011,CLM-ERR-006-ERROR-0012,CLM-ERR-006-ERROR-0013,CLM-ERR-006-ERROR-0014,CLM-ERR-006-ERROR-0015,CLM-ERR-006-ERROR-0016,CLM-ERR-006-ERROR-0017,CLM-ERR-006-ERROR-0018,CLM-ERR-006-ERROR-0019,CLM-ERR-006-ERROR-0020,CLM-ERR-006-ERROR-0021,CLM-ERR-006-ERROR-0022,CLM-ERR-006-ERROR-0023,CLM-ERR-006-ERROR-0024,CLM-ERR-006-ERROR-0025,CLM-ERR-006-ERROR-0026,CLM-ERR-006-ERROR-0027,CLM-ERR-006-ERROR-0028,CLM-ERR-006-ERROR-0029,CLM-ERR-006-ERROR-0030,CLM-ERR-006-ERROR-0031,CLM-ERR-006-ERROR-0032,CLM-ERR-006-ERROR-0033,CLM-ERR-006-ERROR-0034,CLM-ERR-006-ERROR-0035,CLM-ERR-006-ERROR-0036,CLM-ERR-006-ERROR-0037,CLM-ERR-006-ERROR-0038,CLM-ERR-006-ERROR-0039,CLM-ERR-006-ERROR-0040,CLM-ERR-006-ERROR-0041,CLM-ERR-006-ERROR-0042,CLM-ERR-006-ERROR-0043,CLM-ERR-006-ERROR-0044,CLM-ERR-006-ERROR-0045,CLM-ERR-006-ERROR-0046,CLM-ERR-006-ERROR-0047,CLM-ERR-006-ERROR-0048,CLM-ERR-006-ERROR-0049,CLM-ERR-006-ERROR-0050,CLM-ERR-006-ERROR-0051,CLM-ERR-006-ERROR-0052,CLM-ERR-006-ERROR-0053,CLM-ERR-006-ERROR-0054,CLM-ERR-006-ERROR-0055,CLM-ERR-006-ERROR-0056,CLM-ERR-006-ERROR-0057,CLM-ERR-006-ERROR-0058,CLM-ERR-006-ERROR-0059,CLM-ERR-006-ERROR-0060,CLM-ERR-006-ERROR-0061,CLM-ERR-006-ERROR-0062,CLM-ERR-006-ERROR-0063,CLM-ERR-006-ERROR-0064,CLM-ERR-006-ERROR-0065,CLM-ERR-006-ERROR-0066,CLM-ERR-006-ERROR-0067,CLM-ERR-006-ERROR-0068,CLM-ERR-006-ERROR-0069,CLM-ERR-006-ERROR-0070,CLM-ERR-006-ERROR-0071,CLM-ERR-006-ERROR-0072,CLM-ERR-006-ERROR-0073,CLM-ERR-006-ERROR-0074,CLM-ERR-006-ERROR-0075,CLM-ERR-006-ERROR-0076,CLM-ERR-006-ERROR-0077,CLM-ERR-006-ERROR-0078,CLM-ERR-006-ERROR-0079,CLM-ERR-006-ERROR-0080,CLM-ERR-006-ERROR-0081,CLM-ERR-006-ERROR-0082,CLM-ERR-006-ERROR-0083,CLM-ERR-006-ERROR-0084,CLM-ERR-006-ERROR-0085,CLM-ERR-006-ERROR-0086,CLM-ERR-006-ERROR-0087,CLM-ERR-006-ERROR-0088,CLM-ERR-006-ERROR-0089 -->

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Run connector workers.** Supervise connector delivery, acknowledgement, retry budgets, readiness, cancellation, drain, and abort.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Error inventory

| Evidence record | Type | Variant | Retryable | Recoverable | Defined |
|---|---|---|---|---|---|
| error-06f5c52aa07c86ca5062 | `pocketstation::connector::error::ConnectorErrorCodeError` | `TooLong` | unknown | unknown | `src/connector/error.rs:54` |
| error-093c41e2489cf1bb258d | `pocketstation::connector::transport::ConnectorAudioRecordError` | `InvalidSampleCount` | unknown | unknown | `src/connector/transport.rs:592` |
| error-0b1f3a3357a77fcef185 | `pocketstation::connector::transport::ConnectorAudioRecordError` | type | unknown | unknown | `src/connector/transport.rs:568` |
| error-0b71c9f1b1489e0d4f9a | `pocketstation::connector::error::ConnectorErrorCodeError` | `Empty` | unknown | unknown | `src/connector/error.rs:52` |
| error-0bc8adb0641971704f74 | `pocketstation::connector::error::ConnectorErrorBuildError` | type | unknown | unknown | `src/connector/error.rs:184` |
| error-0c83ebde568152ad3edf | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `TooManyFields` | unknown | unknown | `src/connector/configuration.rs:571` |
| error-0e62627edef059ecab22 | `pocketstation::connector::error::ConnectorErrorStage` | `Startup` | unknown | unknown | `src/connector/error.rs:63` |
| error-10517744910e14c23fc4 | `pocketstation::connector::manifest::ConnectorManifestError` | `InvalidManifestRevision` | unknown | unknown | `src/connector/manifest.rs:235` |
| error-1082687e9dbfd2cadfc5 | `pocketstation::connector::transport::ConnectorAudioRecordError` | `UnsupportedMinor` | unknown | unknown | `src/connector/transport.rs:580` |
| error-16fe034657303e4973f8 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `InvalidValue` | unknown | unknown | `src/connector/configuration.rs:575` |
| error-1cafe789f84ff34b7955 | `pocketstation::connector::ConnectorDeclarationError` | type | unknown | unknown | `src/connector/mod.rs:233` |
| error-1d9267787b6c574f3c02 | `pocketstation::connector::error::ConnectorErrorCodeError` | type | unknown | unknown | `src/connector/error.rs:50` |
| error-20e58c6bbc3ac729a8e8 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `ValueTooLarge` | unknown | unknown | `src/connector/configuration.rs:577` |
| error-287ed1c38c6ad2b533ef | `pocketstation::connector::transport::ConnectorAudioRecordError` | `Truncated` | unknown | unknown | `src/connector/transport.rs:572` |
| error-29230b3395a2c8d86df6 | `pocketstation::connector::manifest::ConnectorManifestError` | `TooManyManifestEntries` | unknown | unknown | `src/connector/manifest.rs:251` |
| error-2f295a051ff6d0366ead | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `WrongType` | unknown | unknown | `src/connector/configuration.rs:574` |
| error-326d10a69e8bf7fdb781 | `pocketstation::connector::error::ConnectorErrorStage` | `Join` | unknown | unknown | `src/connector/error.rs:68` |
| error-37775f819a84416494a5 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `UnknownField` | unknown | unknown | `src/connector/configuration.rs:572` |
| error-3a219a96959e38e2b4d8 | `pocketstation::connector::transport::ConnectorAudioRecordError` | `LengthOverflow` | unknown | unknown | `src/connector/transport.rs:598` |
| error-3a9f4ee91af2cf43237b | `pocketstation::connector::ConnectorObservationLookupError` | type | unknown | unknown | `src/connector/mod.rs:246` |
| error-3d8ad8972e7742e4f68e | `pocketstation::connector::ConnectorRegistrationError` | type | unknown | unknown | `src/connector/mod.rs:225` |
| error-3e4ad6dcbe5b16f5d17a | `pocketstation::connector::observations::ConnectorObservationError` | type | unknown | unknown | `src/connector/observations.rs:175` |
| error-3f3d02bb69a74d3a07ef | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `UnsupportedMajor` | unknown | unknown | `src/connector/transport.rs:259` |
| error-4055838a830f20f7900a | `pocketstation::connector::readiness::ConnectorReadinessPolicyError` | `InvalidDeadline` | unknown | unknown | `src/connector/readiness.rs:63` |
| error-4174c27e2d6508ed5da4 | `pocketstation::connector::manifest::ConnectorManifestError` | type | unknown | unknown | `src/connector/manifest.rs:231` |
| error-421a267b178c062c7edd | `pocketstation::connector::error::ConnectorErrorStage` | type | unknown | unknown | `src/connector/error.rs:60` |
| error-4480b4bd1f780efcd1d4 | `pocketstation::connector::manifest::ConnectorManifestError` | `OutputPortNotSupported` | unknown | unknown | `src/connector/manifest.rs:243` |
| error-44ebfc3c55bcf02bfa81 | `pocketstation::connector::error::ConnectorError` | type | unknown | unknown | `src/connector/error.rs:80` |
| error-45b35bd0aef4aea00cd6 | `pocketstation::connector::manifest::ConnectorManifestError` | `MissingInputPort` | unknown | unknown | `src/connector/manifest.rs:241` |
| error-467b6285f7bce7aa6cb8 | `pocketstation::connector::error::ConnectorErrorStage` | `Configuration` | unknown | unknown | `src/connector/error.rs:61` |
| error-4a046ab28843a5b0e7da | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `MissingRequiredField` | unknown | unknown | `src/connector/configuration.rs:573` |
| error-4a51fb66fefffbe50611 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `TrailingBytes` | unknown | unknown | `src/connector/transport.rs:255` |
| error-4acffde05336329cb8ab | `pocketstation::connector::transport::ConnectorAudioRecordError` | `UnsupportedMajor` | unknown | unknown | `src/connector/transport.rs:578` |
| error-4bfc4d2fa96f8d1ef709 | `pocketstation::connector::transport::ConnectorAudioRecordError` | `TrailingBytes` | unknown | unknown | `src/connector/transport.rs:574` |
| error-4fe54431d9faf19b960f | `pocketstation::connector::ConnectorDeclarationError` | `Session` | unknown | unknown | `src/connector/mod.rs:242` |
| error-50b4e55359dfcdac565b | `pocketstation::connector::manifest::ConnectorManifestError` | `InvalidPackageVersion` | unknown | unknown | `src/connector/manifest.rs:239` |
| error-545cf465ee3605ab2a75 | `pocketstation::connector::transport::ConnectorAudioRecordError` | `NotAudio` | unknown | unknown | `src/connector/transport.rs:570` |
| error-586448a95c1460a5b289 | `pocketstation::connector::error::ConnectorErrorBuildError` | `MessageTooLarge` | unknown | unknown | `src/connector/error.rs:188` |
| error-5a65e48f19f37f45f6a8 | `pocketstation::connector::error::ConnectorErrorStage` | `Retry` | unknown | unknown | `src/connector/error.rs:66` |
| error-5b07b94601e40546dc60 | `pocketstation::connector::error::ConnectorErrorStage` | `Prepare` | unknown | unknown | `src/connector/error.rs:62` |
| error-5b102543d9499772995c | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `LengthOverflow` | unknown | unknown | `src/connector/transport.rs:277` |
| error-616bcdb4c8dff696f1b7 | `pocketstation::connector::ConnectorRegistrationError` | `InvalidManifest` | unknown | unknown | `src/connector/mod.rs:227` |
| error-62699e62ccd0b4d71a86 | `pocketstation::connector::observations::ConnectorObservationError` | `StateUnavailable` | unknown | unknown | `src/connector/observations.rs:177` |
| error-697ee3c52f30d679e76c | `pocketstation::connector::transport::ConnectorAudioRecordError` | `InvalidPortName` | unknown | unknown | `src/connector/transport.rs:586` |
| error-6a12319cb2ab6134781d | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `InvalidValue` | unknown | unknown | `src/connector/transport.rs:273` |
| error-6a2062cb1b9e373ea7fc | `pocketstation::connector::transport::ConnectorAudioRecordError` | `InvalidConnectorId` | unknown | unknown | `src/connector/transport.rs:596` |
| error-6e9769c45e3bcd5db58f | `pocketstation::connector::error::ConnectorErrorBuildError` | `EmptyMessage` | unknown | unknown | `src/connector/error.rs:186` |
| error-709cf38dc8672e9d5c88 | `pocketstation::connector::manifest::ConnectorManifestError` | `EmptyOperatorId` | unknown | unknown | `src/connector/manifest.rs:237` |
| error-772c80dea01fc844fcf8 | `pocketstation::connector::ConnectorDeclarationError` | `WrongSession` | unknown | unknown | `src/connector/mod.rs:235` |
| error-7b77c29136d86b5835f6 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `ValueTooLarge` | unknown | unknown | `src/connector/transport.rs:271` |
| error-7e08e2a1669705da5b62 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `InvalidFieldName` | unknown | unknown | `src/connector/transport.rs:267` |
| error-7f4f2c63d7d9f232f4f5 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `UnknownValueKind` | unknown | unknown | `src/connector/transport.rs:275` |
| error-8494aa774f767ff5c43f | `pocketstation::connector::configuration::ConnectorConfigurationError` | type | unknown | unknown | `src/connector/configuration.rs:608` |
| error-914ae74c05c2036a7e6e | `pocketstation::connector::error::ConnectorErrorStage` | `Delivery` | unknown | unknown | `src/connector/error.rs:65` |
| error-98799eb6b2443feb9b2c | `pocketstation::connector::manifest::ConnectorManifestError` | `ManifestEntryTooLarge` | unknown | unknown | `src/connector/manifest.rs:249` |
| error-98a0da9b477decc01ff1 | `pocketstation::connector::error::ConnectorErrorCodeError` | `InvalidCharacter` | unknown | unknown | `src/connector/error.rs:56` |
| error-a3a1c8010d05ad11da72 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `UnsupportedMinor` | unknown | unknown | `src/connector/transport.rs:261` |
| error-a730e20559890b14a1c2 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `Truncated` | unknown | unknown | `src/connector/transport.rs:253` |
| error-a753ff62a72a421ed184 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `SecretDefaultForbidden` | unknown | unknown | `src/connector/configuration.rs:579` |
| error-a8e458a7b1123416885c | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `ConstraintViolation` | unknown | unknown | `src/connector/configuration.rs:576` |
| error-a9054f4c4fd99d0c5e2a | `pocketstation::connector::ConnectorDeclarationError` | `Configuration` | unknown | unknown | `src/connector/mod.rs:240` |
| error-abe4efbaa3a7aaea13b3 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `TooManyFields` | unknown | unknown | `src/connector/transport.rs:265` |
| error-b0c4698207bc7b03aaea | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | type | unknown | unknown | `src/connector/configuration.rs:568` |
| error-b114ab7356bf999ec9f6 | `pocketstation::connector::error::ConnectorErrorStage` | `Readiness` | unknown | unknown | `src/connector/error.rs:64` |
| error-b740c6d43a7b65a65f4c | `pocketstation::connector::manifest::ConnectorManifestError` | `InvalidManifestEntry` | unknown | unknown | `src/connector/manifest.rs:247` |
| error-b81ae01de00a5851018f | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `SecretClassificationMismatch` | unknown | unknown | `src/connector/configuration.rs:580` |
| error-bc1987453c93e6ee8ac8 | `pocketstation::connector::transport::ConnectorAudioRecordError` | `InvalidHeaderSize` | unknown | unknown | `src/connector/transport.rs:582` |
| error-c0121ae98b5fa0e7b031 | `pocketstation::connector::readiness::ConnectorReadinessPolicyError` | `InvalidThreshold` | unknown | unknown | `src/connector/readiness.rs:65` |
| error-c023c40563b21855ff6e | `pocketstation::connector::ConnectorObservationLookupError` | `WrongSession` | unknown | unknown | `src/connector/mod.rs:248` |
| error-c1a2e0af956fcc7f7c3d | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `DuplicateField` | unknown | unknown | `src/connector/configuration.rs:570` |
| error-c30f7d87adfa17adb931 | `pocketstation::connector::transport::ConnectorAudioRecordError` | `InvalidSampleSpec` | unknown | unknown | `src/connector/transport.rs:588` |
| error-c3f90f97088db1a8a500 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `InvalidMagic` | unknown | unknown | `src/connector/transport.rs:257` |
| error-c43e7d721f296ccf4c97 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `UnexpectedSensitiveValue` | unknown | unknown | `src/connector/configuration.rs:581` |
| error-c8770f17b04a19c1b302 | `pocketstation::connector::manifest::ConnectorManifestError` | `DuplicateManifestEntry` | unknown | unknown | `src/connector/manifest.rs:253` |
| error-c99658b4539b72b3e64b | `pocketstation::connector::transport::ConnectorAudioRecordError` | `InvalidMagic` | unknown | unknown | `src/connector/transport.rs:576` |
| error-d0da1fc3be5e151c12a7 | `pocketstation::connector::manifest::ConnectorManifestError` | `RealtimeExecutionForbidden` | unknown | unknown | `src/connector/manifest.rs:245` |
| error-d273fb39f3c68eacc5f1 | `pocketstation::connector::transport::ConnectorAudioRecordError` | `UnsupportedSampleFormat` | unknown | unknown | `src/connector/transport.rs:590` |
| error-d75924fc2fc0dab06d82 | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `DuplicateField` | unknown | unknown | `src/connector/transport.rs:269` |
| error-db21f6d099d7f100a82f | `pocketstation::connector::transport::ConnectorAudioRecordError` | `ReservedFieldSet` | unknown | unknown | `src/connector/transport.rs:584` |
| error-dcf6c8854ddceff03d7d | `pocketstation::connector::manifest::ConnectorManifestError` | `UnsupportedApiRevision` | unknown | unknown | `src/connector/manifest.rs:233` |
| error-debc94036d83e0ba26d8 | `pocketstation::connector::transport::ConnectorAudioRecordError` | `InvalidLineage` | unknown | unknown | `src/connector/transport.rs:594` |
| error-e1d1aef99be9c81bebdf | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | `ReservedFieldSet` | unknown | unknown | `src/connector/transport.rs:263` |
| error-e33567e15f89b0c1e4b8 | `pocketstation::connector::error::ConnectorErrorCode` | type | unknown | unknown | `src/connector/error.rs:10` |
| error-eaa85c72b64e3f1191dd | `pocketstation::connector::ConnectorRegistrationError` | `Session` | unknown | unknown | `src/connector/mod.rs:229` |
| error-eea403e4ffb890f6ba2e | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `InvalidSchema` | unknown | unknown | `src/connector/configuration.rs:569` |
| error-eef69b49fe108985c4d4 | `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` | `EmptySecret` | unknown | unknown | `src/connector/configuration.rs:578` |
| error-ef20304295eacc3a765a | `pocketstation::connector::transport::ConnectorConfigurationRecordError` | type | unknown | unknown | `src/connector/transport.rs:251` |
| error-efa50c31155dedd95bf3 | `pocketstation::connector::readiness::ConnectorReadinessPolicyError` | type | unknown | unknown | `src/connector/readiness.rs:61` |
| error-f648d9f62ff6bcc1ebe5 | `pocketstation::connector::error::ConnectorErrorStage` | `Shutdown` | unknown | unknown | `src/connector/error.rs:67` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [Author a connector](/docs/guides/connectors.md)
- [Configure connector secrets](/docs/how-to/configure-connector-secrets.md)
- [Configuration reference](/docs/reference/configuration.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/connector/error.rs:1-190` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

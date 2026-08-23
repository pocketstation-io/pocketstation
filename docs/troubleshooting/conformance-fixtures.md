# A conformance check cannot find external fixtures

<!-- claims: CLM-TRBL-013-CAP-001,CLM-TRBL-013-CAP-002,CLM-TRBL-013-SOURCE-001 -->

Use this page when you observe **a conformance check cannot find external fixtures**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.

## Distinguish the cause

Resolve the external fixture path and version first. An absent sibling vector or private artifact is a prerequisite failure, not passing product evidence.

## Diagnostic signals

- `pocketstation::connector::error::ConnectorErrorCodeError` / `TooLong` (`error-06f5c52aa07c86ca5062`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `InvalidSampleCount` (`error-093c41e2489cf1bb258d`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` (`error-0b1f3a3357a77fcef185`)
- `pocketstation::connector::error::ConnectorErrorCodeError` / `Empty` (`error-0b71c9f1b1489e0d4f9a`)
- `pocketstation::connector::error::ConnectorErrorBuildError` (`error-0bc8adb0641971704f74`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `TooManyFields` (`error-0c83ebde568152ad3edf`)
- `pocketstation::connector::error::ConnectorErrorStage` / `Startup` (`error-0e62627edef059ecab22`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `InvalidManifestRevision` (`error-10517744910e14c23fc4`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `UnsupportedMinor` (`error-1082687e9dbfd2cadfc5`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `InvalidValue` (`error-16fe034657303e4973f8`)
- `pocketstation::connector::ConnectorDeclarationError` (`error-1cafe789f84ff34b7955`)
- `pocketstation::connector::error::ConnectorErrorCodeError` (`error-1d9267787b6c574f3c02`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `ValueTooLarge` (`error-20e58c6bbc3ac729a8e8`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `Truncated` (`error-287ed1c38c6ad2b533ef`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `TooManyManifestEntries` (`error-29230b3395a2c8d86df6`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `WrongType` (`error-2f295a051ff6d0366ead`)
- `pocketstation::connector::error::ConnectorErrorStage` / `Join` (`error-326d10a69e8bf7fdb781`)
- `pocketstation::connector::configuration::ConnectorConfigurationErrorCode` / `UnknownField` (`error-37775f819a84416494a5`)
- `pocketstation::connector::transport::ConnectorAudioRecordError` / `LengthOverflow` (`error-3a219a96959e38e2b4d8`)
- `pocketstation::connector::ConnectorObservationLookupError` (`error-3a9f4ee91af2cf43237b`)
- `pocketstation::connector::ConnectorRegistrationError` (`error-3d8ad8972e7742e4f68e`)
- `pocketstation::connector::observations::ConnectorObservationError` (`error-3e4ad6dcbe5b16f5d17a`)
- `pocketstation::connector::transport::ConnectorConfigurationRecordError` / `UnsupportedMajor` (`error-3f3d02bb69a74d3a07ef`)
- `pocketstation::connector::readiness::ConnectorReadinessPolicyError` / `InvalidDeadline` (`error-4055838a830f20f7900a`)
- `pocketstation::connector::manifest::ConnectorManifestError` (`error-4174c27e2d6508ed5da4`)
- `pocketstation::connector::error::ConnectorErrorStage` (`error-421a267b178c062c7edd`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `OutputPortNotSupported` (`error-4480b4bd1f780efcd1d4`)
- `pocketstation::connector::error::ConnectorError` (`error-44ebfc3c55bcf02bfa81`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `MissingInputPort` (`error-45b35bd0aef4aea00cd6`)
- `pocketstation::connector::error::ConnectorErrorStage` / `Configuration` (`error-467b6285f7bce7aa6cb8`)

## Executable evidence

- `given_blocking_start_when_started_twice_then_second_start_cannot_corrupt_state` exercises given blocking start when started twice then second start cannot corrupt state under its recorded setup (`test-2b8e9c0a8c7f0d70b58d`).
- `given_checked_header_layout_when_compared_then_all_public_records_match` exercises given checked header layout when compared then all public records match under its recorded setup (`test-7d7e007ce93466d79bd8`).
- `given_wrong_major_when_checked_then_compatibility_fails` exercises given wrong major when checked then compatibility fails under its recorded setup (`test-7f5494338b4b25e3a131`).
- `abi_codec_cpp_conformance` exercises abi codec cpp conformance under its recorded setup (`test-6fbae5633a1419379cd7`).
- `abi_session_c_conformance` exercises abi session c conformance under its recorded setup (`test-1ab6697ee6c783b1c41b`).
- `abi_session_c_success_conformance` exercises abi session c success conformance under its recorded setup (`test-a2314a88cf28de25b331`).
- `given_external_factory_when_grouping_routes_then_public_shared_group_is_addressable` exercises given external factory when grouping routes then public shared group is addressable under its recorded setup (`test-771b49f6eeae48ddfb9b`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` exercises given bitrate change when encode then still produces valid packet under its recorded setup (`test-60e08c6e7ec6bb4b5978`).
- `given_encoder_when_destroy_null_then_no_crash` exercises given encoder when destroy null then no crash under its recorded setup (`test-c5614104f53b6b245bfd`).
- `given_invalid_channel_count_when_create_then_returns_null` exercises given invalid channel count when create then returns null under its recorded setup (`test-736ddd354b42f58df4ad`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` exercises given invalid frame size when encode then error is typed without writing under its recorded setup (`test-1e4368d9ab7990c79bd7`).
- `given_null_encoder_when_encode_then_returns_minus_one` exercises given null encoder when encode then returns minus one under its recorded setup (`test-041037b5b9482d79c8e2`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` exercises given null encoder when set bitrate then returns minus one under its recorded setup (`test-d3686b94180b732c8001`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` exercises given panicking abi bodies when guarded then panics are contained under its recorded setup (`test-03d685383aaeadb55cad`).
- `given_rejected_capacity_when_retried_then_encoder_state_is_unchanged` exercises given rejected capacity when retried then encoder state is unchanged under its recorded setup (`test-d02294e14bc1e7d6bfd2`).

## Corrective action and retry

Apply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.

## Data and state

Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Run protocol checks](/docs/how-to/run-protocol-checks.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Compatibility and evidence](/docs/compatibility/README.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Test evidence index](/docs/reference/test-evidence.md)
- [Keep qualification claims scoped](/docs/best-practices/evidence-boundaries.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/connector_portable_semantics.rs:1-210` (`DIRECT`)
- `scripts/check_protocol.sh:1-132` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

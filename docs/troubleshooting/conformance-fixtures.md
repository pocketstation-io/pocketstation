# A conformance check cannot find external fixtures

<!-- claims: CLM-TRBL-013-CAP-001,CLM-TRBL-013-CAP-002,CLM-TRBL-013-SOURCE-001 -->

## Symptom

A protocol or connector conformance check cannot locate a required vector or private fixture.

## Evidenced causes

- The connector portable-semantics vector is outside this repository at the required sibling path.
- A private CI fixture is unavailable in the local checkout.
- The fixture revision does not match the test contract.

## Distinguish the causes

Resolve the literal path used by the test and record the fixture revision before interpreting any assertion result.

## Diagnostic signals

- `pocketstation::conformance::ObservedEndpointError` (`error-a7289e7474b91e5a9542`)
- `pocketstation::conformance::ObservedEndpointError` / `ConnectorDeclaration` (`error-6d124bc028b19f8db7fe`)
- `pocketstation::conformance::ObservedEndpointError` / `ConnectorRegistration` (`error-4b6f9e5e88b31f0c8734`)
- `pocketstation::conformance::ObservedEndpointError` / `Contract` (`error-923f7f3dff4303f2c83a`)
- `pocketstation::conformance::ObservedEndpointError` / `Declaration` (`error-6258c07672471ba80a01`)
- `pocketstation::conformance::ObservedEndpointError` / `Registration` (`error-3afbbc9b18e186b82cd7`)
- `pocketstation::connector::error::ConnectorErrorBuildError` / `EmptyMessage` (`error-7391bc39acc101285267`)
- `pocketstation::connector::error::ConnectorErrorCodeError` / `Empty` (`error-922d5b7f9ed05c25b51a`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `EmptyOperatorId` (`error-8abdf7118ecd1dff1b30`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `InvalidPackageVersion` (`error-7911f25325d8ed303ce2`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `OutputPortNotSupported` (`error-a20daf688ba80629cc31`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `RealtimeExecutionForbidden` (`error-3d0ab46d383d572713d5`)

## Executable evidence

- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` exercises given canonical connector vectors when compared then core contract semantics match under its recorded setup (`test-5ccbb97716e582e0a790`).
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

## Corrective action

Provide the exact versioned artifact at its expected location, then rerun the selected check without weakening or skipping the assertion.

## Retry and incomplete state

Absence is a prerequisite failure, not a retryable product failure and not passing conformance evidence. No conformance conclusion exists until the fixture runs.

## Related reference

- [Conformance](/docs/concepts/conformance.md)
- [Test Connector Conformance](/docs/how-to/test-connector-conformance.md)

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

The claims on **A conformance check cannot find external fixtures** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/connector_portable_semantics.rs:1-210` (`DIRECT`)
- `scripts/check_protocol.sh:1-132` (`DIRECT`)

For **A conformance check cannot find external fixtures**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

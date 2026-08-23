# A conformance check cannot find external fixtures

<!-- claims: CLM-TRBL-013-CAP-001,CLM-TRBL-013-CAP-002,CLM-TRBL-013-SOURCE-001 -->

## Symptom

A protocol or connector conformance check cannot locate a required vector or private fixture.

## Evidenced causes

- The repository-owned connector vector has not been copied to the sibling path consumed by the test.
- A private CI fixture is unavailable in the local checkout.
- The materialized fixture revision does not match the repository source.

## Distinguish the causes

Resolve the literal path used by the test, compare it with `scripts/fixtures/connector-v1-vectors.json`, and record the fixture revision before interpreting any assertion result.

## Diagnostic signals

- `pocketstation::conformance::ObservedEndpointError` (`error-c53bb5d1f6823776c79e`)
- `pocketstation::conformance::ObservedEndpointError` / `ConnectorDeclaration` (`error-3bee76716d8c8633dc81`)
- `pocketstation::conformance::ObservedEndpointError` / `ConnectorRegistration` (`error-4fcbb301b1e8b58f52d7`)
- `pocketstation::conformance::ObservedEndpointError` / `Contract` (`error-e2cff9aa7d31d5f4b4ae`)
- `pocketstation::conformance::ObservedEndpointError` / `Declaration` (`error-1525ad42252c76777ba7`)
- `pocketstation::conformance::ObservedEndpointError` / `Registration` (`error-c4d7facc4368d387d1e3`)
- `pocketstation::connector::error::ConnectorErrorBuildError` / `EmptyMessage` (`error-8430bdfe51c4584abdb9`)
- `pocketstation::connector::error::ConnectorErrorCodeError` / `Empty` (`error-25b54ec2eac428710e68`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `EmptyOperatorId` (`error-d042f217cd9be875496e`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `InvalidPackageVersion` (`error-8f9a7ce46894523b0960`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `OutputPortNotSupported` (`error-322972e96e32e4cb290a`)
- `pocketstation::connector::manifest::ConnectorManifestError` / `RealtimeExecutionForbidden` (`error-c9d21d4d6e14222151cb`)

## Executable evidence

- `given_canonical_connector_vectors_when_compared_then_core_contract_semantics_match` exercises given canonical connector vectors when compared then core contract semantics match under its recorded setup (`test-2df56b27d49e3e92a1f8`).
- `given_blocking_start_when_started_twice_then_second_start_cannot_corrupt_state` exercises given blocking start when started twice then second start cannot corrupt state under its recorded setup (`test-b2828ba033d77339dbdd`).
- `given_checked_header_layout_when_compared_then_all_public_records_match` exercises given checked header layout when compared then all public records match under its recorded setup (`test-c2b2f5b6ca3644e0bf5e`).
- `given_wrong_major_when_checked_then_compatibility_fails` exercises given wrong major when checked then compatibility fails under its recorded setup (`test-ee6a03f0db459d8f2e98`).
- `abi_codec_cpp_conformance` exercises abi codec cpp conformance under its recorded setup (`test-544e66e8f85e1ad9e055`).
- `abi_session_c_conformance` exercises abi session c conformance under its recorded setup (`test-9e1beea6279253161031`).
- `abi_session_c_success_conformance` exercises abi session c success conformance under its recorded setup (`test-fbd5f1d6e0ff13895c92`).
- `given_external_factory_when_grouping_routes_then_public_shared_group_is_addressable` exercises given external factory when grouping routes then public shared group is addressable under its recorded setup (`test-a600773ba90dce89ab8e`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` exercises given bitrate change when encode then still produces valid packet under its recorded setup (`test-f2e13a28f1aa591f3c67`).
- `given_encoder_when_destroy_null_then_no_crash` exercises given encoder when destroy null then no crash under its recorded setup (`test-8ba7b5b19e9d7dfbc464`).
- `given_invalid_channel_count_when_create_then_returns_null` exercises given invalid channel count when create then returns null under its recorded setup (`test-9fb4684ff29b5ab716fd`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` exercises given invalid frame size when encode then error is typed without writing under its recorded setup (`test-002ce44230f2b0ac6d7c`).
- `given_null_encoder_when_encode_then_returns_minus_one` exercises given null encoder when encode then returns minus one under its recorded setup (`test-657d1e2cbdcbd70cf5fa`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` exercises given null encoder when set bitrate then returns minus one under its recorded setup (`test-f10bfad1b583316ad6fb`).
- `given_panicking_abi_bodies_when_guarded_then_panics_are_contained` exercises given panicking abi bodies when guarded then panics are contained under its recorded setup (`test-a807dc7f3aad831eda7a`).

## Corrective action

Create the sibling directory, copy the repository-owned vector to its expected path, then rerun the selected check without weakening or skipping the assertion.

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

The claims on **A conformance check cannot find external fixtures** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `tests/connector_portable_semantics.rs:1-210` (`DIRECT`)
- `scripts/check_protocol.sh:1-132` (`DIRECT`)

For **A conformance check cannot find external fixtures**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

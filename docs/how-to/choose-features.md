# Choose crate features

<!-- claims: CLM-GUIDE-027-CAP-001,CLM-GUIDE-027-SOURCE-001 -->

## Scope

- **Install and feature-select the crate.** Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.

The scope of **Choose crate features** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

The consumer role: native capture application, contracts-only library, conformance harness, or repository test.

## Procedure

1. Use defaults for native capture applications.
2. Disable defaults for contract-only consumers.
3. Enable conformance-fixtures only for fixture APIs.
4. Reserve internal-testing for repository checks.
5. Rebuild after feature changes.

## Important consequence

Cargo unifies features across the dependency graph, so inspect the resolved build rather than one dependency line in isolation.

## Verify the outcome

`cargo check` succeeds with the selected feature set and only the intended gated modules are visible.

Executable evidence selected for **Choose crate features** is limited to each test's recorded setup and assertions:

- `given_normal_crate_root_when_scanned_then_implementation_owners_are_private` — given normal crate root when scanned then implementation owners are private (`tests/public_api_boundary.rs:41`; `test-0b0f2295b7da3933b922`).
- `given_supported_contracts_when_named_from_crate_root_then_they_compile` — given supported contracts when named from crate root then they compile (`tests/public_api_boundary.rs:14`; `test-4693b2633263a52e0bdf`).
- `test-publish-recovery` — test publish recovery (`scripts/test-publish-recovery.sh:1`; `test-adb1dc831caefa6dcbaa`).
- `test-session-c-conformance` — test session c conformance (`scripts/test-session-c-conformance.sh:1`; `test-ff28fea505d22371ce41`).
- `test-single-package-publish` — test single package publish (`scripts/test-single-package-publish.sh:1`; `test-2cb6c90c68d9362041c6`).
- `given_pkss_v1_message_kinds_when_projected_then_values_remain_stable` — given pkss v1 message kinds when projected then values remain stable (`tests/protocol_compatibility.rs:35`; `test-3f19ed84be12761963bc`).
- `given_pkss_v1_signal_when_encoded_then_bytes_remain_stable` — given pkss v1 signal when encoded then bytes remain stable (`tests/protocol_compatibility.rs:12`; `test-96b5bb377993cd2e3876`).
- `given_connector_authoring_layer_when_scanned_then_it_does_not_duplicate_core_runtime_policy` — given connector authoring layer when scanned then it does not duplicate core runtime policy (`tests/public_api_boundary.rs:98`; `test-bcf303db85cca8b13bc2`).
- `given_endpoint_spi_when_source_is_scanned_then_connector_policy_never_flows_downward` — given endpoint spi when source is scanned then connector policy never flows downward (`tests/public_api_boundary.rs:85`; `test-8703810e1a604377a54e`).

## Failure signals

No task-specific public error was resolved for choose crate features; preserve the owning API's returned error.

## API reference

- [Cargo Features](/docs/concepts/cargo-features.md)
- [Configuration](/docs/reference/configuration.md)

No intentionally public Rust declaration is owned directly by **Choose crate features**. Its contract is expressed by the linked repository, protocol, or qualification evidence instead.

## Related documentation

- [Cargo features and build surfaces](/docs/concepts/cargo-features.md)
- [Glossary](/docs/glossary.md)
- [Install PocketStation](/docs/getting-started/installation.md)
- [PocketStation](/README.md)
- [PocketStation documentation](/docs/README.md)
- [Release and version information](/RELEASE_NOTES.md)
- [Run the examples](/docs/getting-started/examples.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

The claims on **Choose crate features** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)

For **Choose crate features**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

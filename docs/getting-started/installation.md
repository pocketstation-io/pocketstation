# Install PocketStation

<!-- claims: CLM-DOC-002-CAP-001,CLM-DOC-002-SOURCE-001 -->

## Prerequisites

- Rust 1.95 or newer, as declared by package metadata.
- Cargo for dependency and feature resolution.
- Native platform development dependencies when the default native-capture feature is enabled.

## Add the dependency

```toml
[dependencies]
pocketstation = "1.1.1"
```

For a contracts-only build:

```toml
pocketstation = { version = "1.1.1", default-features = false }
```

Run `cargo check` to verify dependency resolution. Feature selection is compile-time configuration.

## Scope

- **Install and feature-select the crate.** Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.

The scope of **Install PocketStation** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Public entry points

No intentionally public Rust declaration is owned directly by **Install PocketStation**. Its contract is expressed by the linked repository, protocol, or qualification evidence instead.

## Executable evidence

Executable evidence selected for **Install PocketStation** is limited to each test's recorded setup and assertions:

- `test-publish-recovery` — test publish recovery (`scripts/test-publish-recovery.sh:1`; `test-adb1dc831caefa6dcbaa`).
- `test-session-c-conformance` — test session c conformance (`scripts/test-session-c-conformance.sh:1`; `test-ff28fea505d22371ce41`).
- `test-single-package-publish` — test single package publish (`scripts/test-single-package-publish.sh:1`; `test-2cb6c90c68d9362041c6`).
- `given_pkss_v1_message_kinds_when_projected_then_values_remain_stable` — given pkss v1 message kinds when projected then values remain stable (`tests/protocol_compatibility.rs:35`; `test-3f19ed84be12761963bc`).
- `given_pkss_v1_signal_when_encoded_then_bytes_remain_stable` — given pkss v1 signal when encoded then bytes remain stable (`tests/protocol_compatibility.rs:12`; `test-96b5bb377993cd2e3876`).
- `given_connector_authoring_layer_when_scanned_then_it_does_not_duplicate_core_runtime_policy` — given connector authoring layer when scanned then it does not duplicate core runtime policy (`tests/public_api_boundary.rs:98`; `test-bcf303db85cca8b13bc2`).
- `given_endpoint_spi_when_source_is_scanned_then_connector_policy_never_flows_downward` — given endpoint spi when source is scanned then connector policy never flows downward (`tests/public_api_boundary.rs:85`; `test-8703810e1a604377a54e`).
- `given_normal_crate_root_when_scanned_then_implementation_owners_are_private` — given normal crate root when scanned then implementation owners are private (`tests/public_api_boundary.rs:41`; `test-0b0f2295b7da3933b922`).
- `given_supported_contracts_when_named_from_crate_root_then_they_compile` — given supported contracts when named from crate root then they compile (`tests/public_api_boundary.rs:14`; `test-4693b2633263a52e0bdf`).

## Related documentation

- [Cargo features and build surfaces](/docs/concepts/cargo-features.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [PocketStation documentation](/docs/README.md)
- [Release and version information](/RELEASE_NOTES.md)
- [Run the examples](/docs/getting-started/examples.md)
- [Choose crate features](/docs/how-to/choose-features.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

The claims on **Install PocketStation** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)

For **Install PocketStation**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

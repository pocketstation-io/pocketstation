# Cargo features and build surfaces

<!-- claims: CLM-DOC-056-SCOPE-001,CLM-DOC-056-TEXT-001,CLM-DOC-056-TEXT-002,CLM-DOC-056-TEXT-003,CLM-DOC-056-TEXT-004,CLM-DOC-056-TEXT-005,CLM-DOC-056-TEXT-006,CLM-DOC-056-SOURCE-001 -->

## What it is

Cargo features select whether the crate exposes contracts only, the default native-capture implementation, conformance fixtures, or repository-internal testing surfaces.

## Why it exists

Consumers that only need type contracts should not be forced to compile native capture dependencies, while validation builds need deliberate access to feature-gated fixtures.

## Relationships

- The default feature enables native capture.
- `conformance-fixtures` exposes deterministic cross-language fixtures.
- `internal-testing` exposes repository validation surfaces and is not a runtime product recommendation.

## Invariants and guarantees

- Feature selection happens at compile time and follows Cargo feature unification.
- Changing features requires rebuilding the crate.
- All feature-gated public declarations remain part of the documentation inventory.

## When you encounter it

- **Reach first captured frames** — Build a Session that captures an application and microphone and polls their independent frames.

## Use it

- [Choose crate features](/docs/how-to/choose-features.md)
- [Install PocketStation](/docs/getting-started/installation.md)
- [A native-capture build fails](/docs/troubleshooting/native-build.md)

## Scope

- **Install and feature-select the crate.** Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.

The scope of **Cargo features and build surfaces** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

No intentionally public Rust declaration is owned directly by **Cargo features and build surfaces**. Its contract is expressed by the linked repository, protocol, or qualification evidence instead.

## Executable evidence

Executable evidence selected for **Cargo features and build surfaces** is limited to each test's recorded setup and assertions:

- `test-publish-recovery` — test publish recovery (`scripts/test-publish-recovery.sh:1`; `test-ed1499f4e962b2a908d0`).
- `test-session-c-conformance` — test session c conformance (`scripts/test-session-c-conformance.sh:1`; `test-9ccda9c28ea127ec4fe9`).
- `test-single-package-publish` — test single package publish (`scripts/test-single-package-publish.sh:1`; `test-042a4aa4fca762147921`).
- `given_pkss_v1_message_kinds_when_projected_then_values_remain_stable` — given pkss v1 message kinds when projected then values remain stable (`tests/protocol_compatibility.rs:35`; `test-be3fbcfc583f9a784846`).
- `given_pkss_v1_signal_when_encoded_then_bytes_remain_stable` — given pkss v1 signal when encoded then bytes remain stable (`tests/protocol_compatibility.rs:12`; `test-0185a2790c06762d1676`).
- `given_connector_authoring_layer_when_scanned_then_it_does_not_duplicate_core_runtime_policy` — given connector authoring layer when scanned then it does not duplicate core runtime policy (`tests/public_api_boundary.rs:110`; `test-7012bfbc5fb5bdda9c3d`).
- `given_endpoint_spi_when_source_is_scanned_then_connector_policy_never_flows_downward` — given endpoint spi when source is scanned then connector policy never flows downward (`tests/public_api_boundary.rs:97`; `test-720a9195239b89934de9`).
- `given_normal_crate_root_when_scanned_then_implementation_owners_are_private` — given normal crate root when scanned then implementation owners are private (`tests/public_api_boundary.rs:53`; `test-3342f0e573ca9490e3ac`).
- `given_supported_contracts_when_named_from_crate_root_then_they_compile` — given supported contracts when named from crate root then they compile (`tests/public_api_boundary.rs:16`; `test-325470c748c78baabbba`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [Install PocketStation](/docs/getting-started/installation.md)
- [PocketStation](/README.md)
- [PocketStation documentation](/docs/README.md)
- [Release and version information](/RELEASE_NOTES.md)
- [Run the examples](/docs/getting-started/examples.md)
- [Choose crate features](/docs/how-to/choose-features.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)

## Evidence boundary

The claims on **Cargo features and build surfaces** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `Cargo.toml:1-21` (`DIRECT`)
- `build.rs:1-21` (`DIRECT`)

For **Cargo features and build surfaces**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

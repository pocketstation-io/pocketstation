# Release evidence boundary

<!-- claims: CLM-DOC-036-CAP-001,CLM-DOC-036-SOURCE-001 -->

## What it is

The release evidence boundary is the set of package metadata, workflow jobs, protocol checks, tests, examples, and artifacts recorded for a published revision.

## Why it exists

A release note is a declaration, while executable checks establish narrower facts. Keeping the evidence layers separate prevents publication metadata from becoming an unsupported product guarantee.

## Relationships

- `Cargo.toml` declares the crate version and feature surface.
- CI and publish workflows select validation commands.
- Release notes summarize changes and link to the validated revision.

## Invariants and guarantees

- Every release claim remains scoped to its recorded command and environment.
- An implemented platform path is not automatically physically qualified.
- External or private prerequisites remain visible in the completion report.

## When you encounter it

- **Validate an integration** — Run protocol, ABI, connector, package, and example checks at the frozen source revision.

## Use it

- [Release and version information](/RELEASE_NOTES.md)
- [Keep qualification claims scoped](/docs/best-practices/evidence-boundaries.md)
- [Run protocol checks](/docs/how-to/run-protocol-checks.md)

## Scope

- **Build and publish repository artifacts.** Run architecture, protocol, package, platform, and release checks used by the repository publication workflow.

The scope of **Release evidence boundary** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

No intentionally public Rust declaration is owned directly by **Release evidence boundary**. Its contract is expressed by the linked repository, protocol, or qualification evidence instead.

## Executable evidence

Executable evidence selected for **Release evidence boundary** is limited to each test's recorded setup and assertions:

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
- [PocketStation](/README.md)
- [Release and version information](/RELEASE_NOTES.md)
- [Run protocol checks](/docs/how-to/run-protocol-checks.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Compatibility and evidence](/docs/compatibility/README.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Test evidence index](/docs/reference/test-evidence.md)

## Evidence boundary

The claims on **Release evidence boundary** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `.github/workflows/publish.yml:1-171` (`DIRECT`)
- `scripts/publish.sh:1-69` (`DIRECT`)

For **Release evidence boundary**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

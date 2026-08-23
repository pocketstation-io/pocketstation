# Keep qualification claims scoped

<!-- claims: CLM-BEST-007-CAP-001,CLM-BEST-007-CAP-002,CLM-BEST-007-CAP-003,CLM-BEST-007-CAP-004,CLM-BEST-007-CAP-005,CLM-BEST-007-SOURCE-001 -->

## Problem

Build, VM, conformance, and physical-device evidence answer different questions and can be accidentally promoted into broad support claims.

## Recommendation

Label every platform or provider claim with the command, target, fixture, environment, and evidence class that established it.

## Reason

Scoped labels let maintainers distinguish implementation presence from real-device or live-provider qualification.

## Tradeoff

Narrow claims are less convenient than one support badge, but they remain auditable and avoid unsupported guarantees.

## When it does not apply

You may combine scopes only when the published statement explicitly lists every constituent environment and result.

## Repository evidence

This recommendation is tied directly to the page's source evidence.

## Executable evidence

Executable evidence selected for **Keep qualification claims scoped** is limited to each test's recorded setup and assertions:

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
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)

## Evidence boundary

The claims on **Keep qualification claims scoped** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `.github/workflows/ci.yml:1-63` (`DIRECT`)
- `src/capture/platform/mod.rs:1-7` (`DIRECT`)

For **Keep qualification claims scoped**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

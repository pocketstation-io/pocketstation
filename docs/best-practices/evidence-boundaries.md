# Keep qualification claims scoped

<!-- claims: CLM-BEST-007-CAP-001,CLM-BEST-007-CAP-002,CLM-BEST-007-CAP-003,CLM-BEST-007-CAP-004,CLM-BEST-007-CAP-005,CLM-BEST-007-SOURCE-001 -->

## Recommendation

Label build, virtual-machine, conformance, and physical-device evidence separately; never promote one scope into another.

## Why

The repository makes capacity, ownership, identity, lifecycle, and evidence boundaries explicit so failures remain attributable. Bypassing them removes observations and typed outcomes needed for diagnosis.

## Tradeoff

The recommendation requires explicit configuration and result handling. It does not promise that one capacity, retry budget, selector, or shutdown policy fits every workload. Measure within the API's stated scope.

## When it does not apply

Do not apply a realtime, connector, capture, or extension rule to another lane or boundary unless it exposes the same contract. An internal pattern is not automatically a public recommendation.

## Repository evidence

- `buffer_pool` at `tests/public_api_boundary.rs` (`pattern-2973216be17e52591275`).
- `buffer_pool` at `scripts/check_pks_single_engine_boundary.sh` (`pattern-691896feff36ac00f8fd`).
- `buffer_pool` at `benches/generated_audio_bridge.rs` (`pattern-75ee714be751c7038ff7`).
- `sidecar_isolation` at `tests/protocol_compatibility.rs` (`pattern-fe84fb6f35a53c802413`).

## Executable evidence

The following test bodies are evidence only for their recorded setup:

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

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `.github/workflows/ci.yml:1-63` (`DIRECT`)
- `src/capture/platform/mod.rs:1-7` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

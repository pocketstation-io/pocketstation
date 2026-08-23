# Treat stop outcomes as data

<!-- claims: CLM-BEST-004-CAP-001,CLM-BEST-004-CAP-002,CLM-BEST-004-CAP-003,CLM-BEST-004-CAP-004,CLM-BEST-004-CAP-005,CLM-BEST-004-SOURCE-001 -->

## Recommendation

Preserve structured stop, component, recording, sidecar, and trace outcomes before releasing runtime ownership.

## Why

The repository makes capacity, ownership, identity, lifecycle, and evidence boundaries explicit so failures remain attributable. Bypassing them removes observations and typed outcomes needed for diagnosis.

## Tradeoff

The recommendation requires explicit configuration and result handling. It does not promise that one capacity, retry budget, selector, or shutdown policy fits every workload. Measure within the API's stated scope.

## When it does not apply

Do not apply a realtime, connector, capture, or extension rule to another lane or boundary unless it exposes the same contract. An internal pattern is not automatically a public recommendation.

## Repository evidence

- `sidecar_isolation` at `src/connector/mod.rs` (`pattern-00438ff8d2146688eeaf`).
- `buffer_pool` at `src/session/extensions/audio_input/source.rs` (`pattern-02730342ffe5edf1a1a0`).
- `typed_error` at `src/session/declaration/typed_stream.rs` (`pattern-0577caa4e8cabf1f0284`).
- `clock_correlation` at `src/session/lifecycle/running.rs` (`pattern-099903d47c50357c102e`).
- `typed_error` at `src/connector/worker/endpoint_adapter.rs` (`pattern-0c264e4ec468e7568a9c`).
- `typed_error` at `src/session/extensions/tests/runtime.rs` (`pattern-0e401638b1f6b8cd709e`).
- `typed_error` at `src/session/compile/tests.rs` (`pattern-0f5d415b7192b8e10569`).
- `typed_error` at `src/session/extensions/tests/registry.rs` (`pattern-1051f823fb6b54798236`).
- `sidecar_isolation` at `src/connector/transport.rs` (`pattern-139a2b492c98807b410f`).
- `transactional_registration` at `src/session/lifecycle/endpoint_transaction.rs` (`pattern-15c202b8b1da269b86b8`).
- `sidecar_isolation` at `src/session/mod.rs` (`pattern-21f70c88ad1965778a78`).
- `sidecar_isolation` at `src/connector/status.rs` (`pattern-2204e930be728d4ccf21`).
- `typed_error` at `src/recording/endpoint.rs` (`pattern-25297d2b3b32d08b7163`).
- `typed_error` at `src/recording/endpoint/tests.rs` (`pattern-2b4448626799ffae1fce`).
- `buffer_pool` at `src/session/extensions/builtins.rs` (`pattern-2bff6ab2da8c5acd813e`).
- `typed_error` at `src/connector/transport.rs` (`pattern-2ee2fce6c23e17a0e11e`).
- `typed_error` at `src/recording/writer.rs` (`pattern-302861fed4a6e4c5b34c`).
- `typed_error` at `src/session/lifecycle/host.rs` (`pattern-302fd4dd990824448edd`).
- `transactional_registration` at `src/connector/mod.rs` (`pattern-31b76706228fd84bfc03`).
- `typed_error` at `examples/product_quickstart.rs` (`pattern-382a459c91458f67937e`).

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-20a4c27d70a60c9bc881`).
- `given_extension_key_matching_old_metadata_when_compiled_then_value_remains_opaque` — given extension key matching old metadata when compiled then value remains opaque (`src/session/compile/tests.rs:769`; `test-e802cd282ad498db0074`).
- `given_structural_ingress_when_validated_then_session_metadata_is_not_required` — given structural ingress when validated then session metadata is not required (`src/session/extensions/builtins.rs:636`; `test-7406cc23117530680012`).
- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-6dd97c870cc349f825f9`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e7766766c491d2b7101`).
- `given_two_sources_when_started_then_gate_lineage_and_repeated_stop_are_truthful` — given two sources when started then gate lineage and repeated stop are truthful (`src/session/lifecycle/tests/running.rs:1309`; `test-39a6ab1a3e3e6782af3a`).
- `given_typed_operator_routes_when_stopped_then_final_state_and_metrics_are_truthful` — given typed operator routes when stopped then final state and metrics are truthful (`src/session/lifecycle/tests/running.rs:1114`; `test-4a96ceb3ecb843502e07`).
- `given_grouped_connector_when_session_stops_then_one_worker_is_joined_and_observed` — given grouped connector when session stops then one worker is joined and observed (`tests/connector_contract.rs:679`; `test-aa2345c7b9339f742b48`).
- `given_worker_failure_or_panic_when_session_stops_then_endpoint_finalization_is_terminal` — given worker failure or panic when session stops then endpoint finalization is terminal (`tests/connector_contract.rs:805`; `test-e56e88c9e99290ea720a`).
- `given_stopped_public_session_when_new_session_starts_then_capture_restarts_cleanly` — given stopped public session when new session starts then capture restarts cleanly (`tests/session_facade.rs:124`; `test-cbee0768bffa592adde2`).
- `given_provider_owned_field_name_when_resolved_then_core_preserves_it_opaquely` — given provider owned field name when resolved then core preserves it opaquely (`src/connector/configuration.rs:642`; `test-d9078fd01d0271720b30`).
- `given_empty_input_group_when_sidecar_prepares_then_structured_error_is_returned` — given empty input group when sidecar prepares then structured error is returned (`src/connector/sidecar.rs:270`; `test-49bd18fb96d67fdba9bf`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Lifecycle evidence index](/docs/reference/lifecycle-evidence.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/events.rs:1-736` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

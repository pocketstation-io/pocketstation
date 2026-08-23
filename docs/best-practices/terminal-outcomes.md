# Treat stop outcomes as data

<!-- claims: CLM-BEST-004-CAP-001,CLM-BEST-004-CAP-002,CLM-BEST-004-CAP-003,CLM-BEST-004-CAP-004,CLM-BEST-004-CAP-005,CLM-BEST-004-SOURCE-001 -->

## Problem

Reducing Session stop to one success flag hides component, recording, sidecar, and trace finalization results.

## Recommendation

Retain and inspect the complete terminal outcome before releasing runtime ownership or reporting success.

## Reason

Independent components can complete or fail separately after useful work has already occurred.

## Tradeoff

Callers must branch over more structured state and decide how partial outputs are presented.

## When it does not apply

A disposable test that asserts one exact failure can inspect only that field when the fixture proves no other output matters.

## Repository evidence

- `transactional_registration` at `src/session/lifecycle/events.rs` (`pattern-0e732daaf7c441a22fe9`).

## Executable evidence

Executable evidence selected for **Treat stop outcomes as data** is limited to each test's recorded setup and assertions:

- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-6dd97c870cc349f825f9`).
- `given_all_failure_classes_when_terminal_then_each_class_is_preserved` — given all failure classes when terminal then each class is preserved (`src/session/lifecycle/events.rs:679`; `test-070a5fc90aa90f7d986a`).
- `given_all_senders_dropped_when_polled_then_receiver_reports_closed` — given all senders dropped when polled then receiver reports closed (`src/session/lifecycle/events.rs:656`; `test-ed28bb41869db2c16ec2`).
- `given_closed_receiver_when_publishing_then_drop_and_closure_are_counted` — given closed receiver when publishing then drop and closure are counted (`src/session/lifecycle/events.rs:664`; `test-14b780bec81ca78fbbe6`).
- `given_events_when_polled_then_fifo_order_and_depth_are_preserved` — given events when polled then fifo order and depth are preserved (`src/session/lifecycle/events.rs:632`; `test-ef7a999d2659ba14b610`).
- `given_full_queue_when_publishing_then_newest_event_is_dropped_and_counted` — given full queue when publishing then newest event is dropped and counted (`src/session/lifecycle/events.rs:572`; `test-c6f9da58b0231d87b0b9`).
- `given_oversized_session_event_when_published_then_queue_owned_memory_stays_bounded` — given oversized session event when published then queue owned memory stays bounded (`src/session/lifecycle/events.rs:608`; `test-681a72f40c41938b9b0d`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-20a4c27d70a60c9bc881`).
- `given_extension_key_matching_old_metadata_when_compiled_then_value_remains_opaque` — given extension key matching old metadata when compiled then value remains opaque (`src/session/compile/tests.rs:769`; `test-e802cd282ad498db0074`).
- `given_structural_ingress_when_validated_then_session_metadata_is_not_required` — given structural ingress when validated then session metadata is not required (`src/session/extensions/builtins.rs:636`; `test-7406cc23117530680012`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e7766766c491d2b7101`).
- `given_two_sources_when_started_then_gate_lineage_and_repeated_stop_are_truthful` — given two sources when started then gate lineage and repeated stop are truthful (`src/session/lifecycle/tests/running.rs:1309`; `test-39a6ab1a3e3e6782af3a`).

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

The claims on **Treat stop outcomes as data** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/events.rs:1-736` (`DIRECT`)

For **Treat stop outcomes as data**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

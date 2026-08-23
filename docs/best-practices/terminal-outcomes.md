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

This recommendation is tied directly to the page's source evidence.

## Executable evidence

Executable evidence selected for **Treat stop outcomes as data** is limited to each test's recorded setup and assertions:

- `given_no_failures_when_terminal_then_state_is_stopped` — given no failures when terminal then state is stopped (`src/session/lifecycle/events.rs:721`; `test-74456aeb5a4f8bda5b30`).
- `given_all_failure_classes_when_terminal_then_each_class_is_preserved` — given all failure classes when terminal then each class is preserved (`src/session/lifecycle/events.rs:679`; `test-391bc055ecec6559c5ee`).
- `given_all_senders_dropped_when_polled_then_receiver_reports_closed` — given all senders dropped when polled then receiver reports closed (`src/session/lifecycle/events.rs:656`; `test-869cc16c477444c9b6fd`).
- `given_closed_receiver_when_publishing_then_drop_and_closure_are_counted` — given closed receiver when publishing then drop and closure are counted (`src/session/lifecycle/events.rs:664`; `test-a18e7b48053bd0288795`).
- `given_events_when_polled_then_fifo_order_and_depth_are_preserved` — given events when polled then fifo order and depth are preserved (`src/session/lifecycle/events.rs:632`; `test-5346edce6d9a6da2069d`).
- `given_full_queue_when_publishing_then_newest_event_is_dropped_and_counted` — given full queue when publishing then newest event is dropped and counted (`src/session/lifecycle/events.rs:572`; `test-50e2cd9cdbc18d041989`).
- `given_oversized_session_event_when_published_then_queue_owned_memory_stays_bounded` — given oversized session event when published then queue owned memory stays bounded (`src/session/lifecycle/events.rs:608`; `test-9e75be4a362fd68c5951`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-da6484ed83753b351441`).
- `given_extension_key_matching_old_metadata_when_compiled_then_value_remains_opaque` — given extension key matching old metadata when compiled then value remains opaque (`src/session/compile/tests.rs:769`; `test-8a30a347c6b7a008fb97`).
- `given_structural_ingress_when_validated_then_session_metadata_is_not_required` — given structural ingress when validated then session metadata is not required (`src/session/extensions/builtins.rs:636`; `test-9a19e49a87f8cc918b10`).
- `given_host_owned_backends_when_started_then_polled_audio_and_stop_are_real` — given host owned backends when started then polled audio and stop are real (`src/session/lifecycle/host.rs:724`; `test-1e1a4d3810caea030f74`).
- `given_two_sources_when_started_then_gate_lineage_and_repeated_stop_are_truthful` — given two sources when started then gate lineage and repeated stop are truthful (`src/session/lifecycle/tests/running.rs:1309`; `test-74fc5d0f0a325399dd1d`).

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

The claims on **Treat stop outcomes as data** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/events.rs:1-736` (`DIRECT`)

For **Treat stop outcomes as data**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

# Instrument a Session

<!-- claims: CLM-GUIDE-021-CAP-001,CLM-GUIDE-021-CAP-002,CLM-GUIDE-021-SOURCE-001 -->

## Scope

- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.
- **Record and validate Session traces.** Persist lifecycle trace records and validate their structural and terminal consistency.

The scope of **Instrument a Session** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

Observation handles acquired before the interval you need to diagnose and a writable path if you record a trace.

## Procedure

1. Acquire observation handles before the period you need to inspect.
2. Snapshot metrics by stable component IDs.
3. Record a trace for durable lifecycle evidence.
4. Stop and include the terminal outcome.
5. Validate trace structure before diagnosis.

## Important consequence

A missing metric is not automatically zero; keep identity and collection timing with the snapshot.

## Verify the outcome

Metrics resolve by stable component ID, the trace validates, and stop contributes its terminal record.

Executable evidence selected for **Instrument a Session** is limited to each test's recorded setup and assertions:

- `given_route_snapshot_when_drop_observed_then_rate_has_explicit_denominator_and_reasons` — given route snapshot when drop observed then rate has explicit denominator and reasons (`src/session/lifecycle/observations.rs:576`; `test-8235b334c09617394406`).
- `given_route_snapshot_when_latency_observed_then_boundary_units_and_coverage_are_explicit` — given route snapshot when latency observed then boundary units and coverage are explicit (`src/session/lifecycle/observations.rs:604`; `test-964c6ff3404f3a9d5952`).
- `given_complete_trace_when_validated_then_lifecycle_and_terminal_match` — given complete trace when validated then lifecycle and terminal match (`src/session/lifecycle/trace.rs:988`; `test-9dd947375f94d2a4f21f`).
- `given_corrupted_record_when_read_then_checksum_is_rejected` — given corrupted record when read then checksum is rejected (`src/session/lifecycle/trace.rs:1012`; `test-9c9f0e0f6d8934622867`).
- `given_dropped_records_when_validated_then_trace_is_incomplete` — given dropped records when validated then trace is incomplete (`src/session/lifecycle/trace.rs:1115`; `test-7647b415f36fc0ddd5a0`).
- `given_existing_output_when_started_then_recorder_fails_closed` — given existing output when started then recorder fails closed (`src/session/lifecycle/trace.rs:1090`; `test-3b4ddbb231cb8f0182e8`).
- `given_invalid_lifecycle_when_validated_then_validation_fails_closed` — given invalid lifecycle when validated then validation fails closed (`src/session/lifecycle/trace.rs:1057`; `test-ad4fc9ea8d172cd4b678`).
- `given_record_after_terminal_when_validated_then_trace_is_rejected` — given record after terminal when validated then trace is rejected (`src/session/lifecycle/trace.rs:1160`; `test-13dde755fb2a02648705`).
- `given_sequence_gap_when_validated_then_trace_is_rejected` — given sequence gap when validated then trace is rejected (`src/session/lifecycle/trace.rs:1136`; `test-5863853c4bbea9e5db76`).
- `given_timestamp_regression_when_validated_then_trace_is_rejected` — given timestamp regression when validated then trace is rejected (`src/session/lifecycle/trace.rs:1148`; `test-3d881d801a5b93012bc6`).
- `given_truncated_trace_when_read_then_truncation_is_rejected` — given truncated trace when read then truncation is rejected (`src/session/lifecycle/trace.rs:1027`; `test-f5a25488cf2eb39c2921`).
- `given_unknown_version_when_read_then_version_is_rejected` — given unknown version when read then version is rejected (`src/session/lifecycle/trace.rs:1042`; `test-8ba8bc1dda2af18cb6c5`).

## Failure signals

- `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` — `error-244f57c587055bf75eb3`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` / `ChannelClosed` — `error-c62bf3f6cc4392dfd9d8`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` / `Io` — `error-be4cd1948de0bbc85d86`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` / `WorkerPanicked` — `error-161f13dcc08f6e4e3828`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` — `error-3305e685338d403564e6`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `Io` — `error-38569c19585a9b917f3f`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `OutputExists` — `error-07d8590399e61aac56a2`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `ZeroCapacity` — `error-f454c8143178ad251c50`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` — `error-848c45be5f51a946a540`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `IncompleteTrace` — `error-25f6cc4c2eb526799c56`

## API reference

- [Observability](/docs/concepts/observability.md)
- [Session Traces](/docs/concepts/session-traces.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::events::SessionControlFailure` | struct | Typed control-plane failure without exposing an implementation error type. | `src/session/lifecycle/events.rs:70` |
| `pocketstation::session::lifecycle::events::SessionEndpointFailure` | struct | Endpoint failure associated with one stable route and endpoint. | `src/session/lifecycle/events.rs:125` |
| `pocketstation::session::lifecycle::events::SessionEvent` | struct | Event emitted by the session lifecycle authority. | `src/session/lifecycle/events.rs:308` |
| `pocketstation::session::lifecycle::events::SessionEventReceiver` | struct | Sole consumer for a session's bounded control-event queue. | `src/session/lifecycle/events.rs:500` |
| `pocketstation::session::lifecycle::events::SessionFinalizationFailure` | struct | Failure observed while finalizing a stopping session. | `src/session/lifecycle/events.rs:186` |
| `pocketstation::session::lifecycle::events::SessionRollbackFailure` | struct | Failure observed while rolling back a partial session start. | `src/session/lifecycle/events.rs:165` |
| `pocketstation::session::lifecycle::events::SessionSourceFailure` | struct | Source failure associated with one stable session stem. | `src/session/lifecycle/events.rs:104` |
| `pocketstation::session::lifecycle::events::SessionTerminalOutcome` | struct | Complete terminal result. Failure categories remain separate for diagnosis. | `src/session/lifecycle/events.rs:217` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Observation API](/docs/reference/observations.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session trace validation fails](/docs/troubleshooting/session-trace.md)
- [Observations and metrics](/docs/concepts/observability.md)
- [Running ownership](/docs/lifecycle/running.md)

## Evidence boundary

The claims on **Instrument a Session** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/observations.rs:1-636` (`DIRECT`)
- `src/session/lifecycle/trace.rs:1-1179` (`DIRECT`)

For **Instrument a Session**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

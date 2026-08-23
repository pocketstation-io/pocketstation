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
2. Snapshot metrics by typed SessionComponentId values and preserve the observation boundary for each counter or timestamp.
3. Record versioned SessionTraceRecord values for durable lifecycle and component-failure evidence.
4. Stop and include SessionTraceTerminal plus the independent recording outcome.
5. Validate trace structure before diagnosis.

## Important consequence

A missing metric is not automatically zero; keep identity and collection timing with the snapshot.

## Verify the outcome

Metrics resolve by stable component ID, the trace validates, and stop contributes its terminal record.

Executable evidence selected for **Instrument a Session** is limited to each test's recorded setup and assertions:

- `given_route_snapshot_when_drop_observed_then_rate_has_explicit_denominator_and_reasons` — given route snapshot when drop observed then rate has explicit denominator and reasons (`src/session/lifecycle/observations.rs:576`; `test-1402ef12bbb47b0d009a`).
- `given_route_snapshot_when_latency_observed_then_boundary_units_and_coverage_are_explicit` — given route snapshot when latency observed then boundary units and coverage are explicit (`src/session/lifecycle/observations.rs:604`; `test-b5b5d3d4a17670690cae`).
- `given_complete_trace_when_validated_then_lifecycle_and_terminal_match` — given complete trace when validated then lifecycle and terminal match (`src/session/lifecycle/trace.rs:988`; `test-20c537693a458a99f6f9`).
- `given_corrupted_record_when_read_then_checksum_is_rejected` — given corrupted record when read then checksum is rejected (`src/session/lifecycle/trace.rs:1012`; `test-b8839cf2e639dd3e301a`).
- `given_dropped_records_when_validated_then_trace_is_incomplete` — given dropped records when validated then trace is incomplete (`src/session/lifecycle/trace.rs:1115`; `test-5ee80a5b5a1b2ce74da0`).
- `given_existing_output_when_started_then_recorder_fails_closed` — given existing output when started then recorder fails closed (`src/session/lifecycle/trace.rs:1090`; `test-dbf6e802afe21cf4b197`).
- `given_invalid_lifecycle_when_validated_then_validation_fails_closed` — given invalid lifecycle when validated then validation fails closed (`src/session/lifecycle/trace.rs:1057`; `test-10224fcae99d563f600b`).
- `given_record_after_terminal_when_validated_then_trace_is_rejected` — given record after terminal when validated then trace is rejected (`src/session/lifecycle/trace.rs:1160`; `test-3d007d4a819d105d9058`).
- `given_sequence_gap_when_validated_then_trace_is_rejected` — given sequence gap when validated then trace is rejected (`src/session/lifecycle/trace.rs:1136`; `test-71add6b39193d7909de3`).
- `given_timestamp_regression_when_validated_then_trace_is_rejected` — given timestamp regression when validated then trace is rejected (`src/session/lifecycle/trace.rs:1148`; `test-22e90ca672945b18e17f`).
- `given_truncated_trace_when_read_then_truncation_is_rejected` — given truncated trace when read then truncation is rejected (`src/session/lifecycle/trace.rs:1027`; `test-293abf2537b795f160f7`).
- `given_unknown_version_when_read_then_version_is_rejected` — given unknown version when read then version is rejected (`src/session/lifecycle/trace.rs:1042`; `test-7d5245f8ecc878c074a6`).

## Failure signals

- `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` — `error-ffa283db2f8d994a7bb3`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` / `ChannelClosed` — `error-20055d6ea51fc6c90117`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` / `Io` — `error-1fd4ea1ac3006f911ac6`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` / `WorkerPanicked` — `error-590f78f66f3437979d8b`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` — `error-77de2b427998cd0f4d05`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `Io` — `error-283f6632813cc4b9c76a`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `OutputExists` — `error-75ca8a05c1093104ea59`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `ZeroCapacity` — `error-588d0418ebbe33fc1a14`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` — `error-0012f834ccbfd8415156`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `IncompleteTrace` — `error-1028e9754b78cc85646f`

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

The claims on **Instrument a Session** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/observations.rs:1-636` (`DIRECT`)
- `src/session/lifecycle/trace.rs:1-1179` (`DIRECT`)

For **Instrument a Session**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

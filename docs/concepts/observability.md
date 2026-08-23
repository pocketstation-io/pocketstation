# Observations and metrics

<!-- claims: CLM-DOC-030-CAP-001,CLM-DOC-030-SOURCE-001 -->

Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.

## Scope

- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::observations::SessionAudioReentryMetrics` | struct | Exact boundedness and lifecycle accounting for one operator PCM output re-entering the Session audio lane. | `src/session/lifecycle/observations.rs:253` |
| `pocketstation::session::lifecycle::observations::SessionMetricsSnapshot` | struct | Authoritative point-in-time observations for the current Session boundary. | `src/session/lifecycle/observations.rs:36` |
| `pocketstation::session::lifecycle::observations::SessionSidecarMetrics` | struct | Exact bounded-queue and process-lifecycle accounting for one Session-owned language-neutral sidecar. | `src/session/lifecycle/observations.rs:133` |
| `pocketstation::session::lifecycle::observations::SessionDerivedRouteMetrics` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:431` |
| `pocketstation::session::lifecycle::observations::SessionExternalSourceMetrics` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:124` |
| `pocketstation::session::lifecycle::observations::SessionOperatorInputMetrics` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:242` |
| `pocketstation::session::lifecycle::observations::SessionOperatorMetrics` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:382` |
| `pocketstation::session::lifecycle::observations::SessionRouteMetrics` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:139` |
| `pocketstation::session::lifecycle::observations::SessionSourceMetrics` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:117` |
| `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| `pocketstation::session::lifecycle::observations::SessionEventQueueObservations` | struct | Point-in-time observations for a session's bounded control-event queue. | `src/session/lifecycle/observations.rs:17` |
| `pocketstation::session::lifecycle::observations::SessionRouteDropObservations` | struct | Explicit numerator, denominator, interval, and typed reasons for one route. | `src/session/lifecycle/observations.rs:157` |
| `pocketstation::session::lifecycle::observations::SessionRouteLatencyObservations` | struct | Common-clock source timestamp to route-receive latency in nanoseconds. | `src/session/lifecycle/observations.rs:182` |
| `pocketstation::session::lifecycle::observations::SessionRouteObservationInterval` | enum | Interval covered by monotonic route counters. | `src/session/lifecycle/observations.rs:150` |
| `from_open_observations` | function | Records platform authorization observations without inferring them from a generic backend result. Callers must pass `NotObservable` when their platform has no authoritative query for the requested capture class. | `src/capture/authorization.rs:76` |
| `observations` | function | Snapshots the bounded edge counters for this endpoint input. | `src/endpoint/contract.rs:108` |
| `pocketstation::session::lifecycle::observations::SessionRouteObservationInterval::RouteLifetimeToSnapshot` | variant | From route start through the instant of the Session snapshot. | `src/session/lifecycle/observations.rs:152` |
| `SessionOperatorMetrics::input_edge` | struct_field | Sole counter authority for input delivered by the compiled Session plan. | `src/session/lifecycle/observations.rs:389` |
| `SessionOperatorMetrics::input_ports` | struct_field | Exact per-port input accounting. `input_edge` is the compatibility aggregate across this slice. | `src/session/lifecycle/observations.rs:392` |
| `SignalEdgeObservations::delivered_total` | struct_field | Compatibility alias for `enqueued_total`. | `src/runtime/signal/edge.rs:44` |

## Where you encounter it

- **Author a connector** — Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.
- **Host an out-of-process worker** — Spawn a sidecar and enforce bounded messages, deadlines, cancellation, and terminal state.
- **Diagnose a running Session** — Correlate events, metrics, trace records, stable error codes, and terminal outcomes.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `observations` — observations (`src/capture/capture_owner.rs:253`; `test-6c30e98c2843011d2b2e`).
- `observations` — observations (`src/capture/events.rs:314`; `test-09066e0a4bfc4d299258`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-20a4c27d70a60c9bc881`).
- `observations` — observations (`src/runtime/audio/executor.rs:185`; `test-8e5dda8471ef4129edb9`).
- `observations` — observations (`src/runtime/audio/router.rs:849`; `test-75f0a25930a60efd39e9`).
- `observations` — observations (`src/runtime/audio/runner.rs:174`; `test-b1965f6e40d10be0df1e`).
- `source_observations` — source observations (`src/runtime/audio/runner.rs:346`; `test-b098416e730e910b8ece`).
- `observations` — observations (`src/runtime/signal/edge.rs:225`; `test-666d71083357335630fa`).
- `given_typed_operator_routes_when_stopped_then_final_state_and_metrics_are_truthful` — given typed operator routes when stopped then final state and metrics are truthful (`src/session/lifecycle/tests/running.rs:1114`; `test-4a96ceb3ecb843502e07`).
- `receiver_observations` — receiver observations (`src/session/prepare/mappings.rs:260`; `test-7a4e60b11ce4b43df7a7`).
- `sender_observations` — sender observations (`src/session/prepare/mappings.rs:27`; `test-c2c3da3d927ec6c07167`).
- `route_observations` — route observations (`src/session/prepare/prepared.rs:58`; `test-9de2ef861c91c59abd5b`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Running ownership](/docs/lifecycle/running.md)
- [Instrument a Session](/docs/how-to/instrument-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Observation API](/docs/reference/observations.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/observations.rs:1-636` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

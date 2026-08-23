# Observations and metrics

<!-- claims: CLM-DOC-030-CAP-001,CLM-DOC-030-SOURCE-001 -->

## What it is

Observations are stable metrics, events, and snapshots keyed by Session and component identity. They expose queue, drop, latency, readiness, source, operator, endpoint, connector, sidecar, and terminal state.

## Why it exists

Bounded routes and independent components need attributable diagnostics. Aggregate success alone cannot identify which route saturated or which component failed to finalize.

## Relationships

- Observation handles read live counters without taking runtime ownership.
- `SessionComponentId` identifies source, endpoint, operator, sidecar, and runtime observations without string parsing.
- Application policy, frame delivery, source faults, discontinuities, and recording state each retain a typed observation boundary.
- Session events report lifecycle changes.
- Traces persist selected lifecycle evidence for later structural validation.

## Invariants and guarantees

- Counters describe the observation boundary that produced them.
- Missing metrics are not converted into zero unless the type contract says so.
- Observations support diagnosis but do not create performance guarantees.

## When you encounter it

- **Author a connector** — Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.
- **Host an out-of-process worker** — Spawn a sidecar and enforce bounded messages, deadlines, cancellation, and terminal state.
- **Diagnose a running Session** — Correlate events, metrics, trace records, stable error codes, and terminal outcomes.

## Use it

- [Instrument a Session](/docs/how-to/instrument-session.md)
- [Size routes from observations](/docs/best-practices/route-sizing.md)
- [Observations reference](/docs/reference/observations.md)

## Scope

- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.

The scope of **Observations and metrics** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::observations::SessionAudioReentryMetrics` | struct | Exact boundedness and lifecycle accounting for one operator PCM output re-entering the Session audio lane. | `src/session/lifecycle/observations.rs:253` |
| `pocketstation::session::lifecycle::observations::SessionDerivedRouteMetrics` | struct | Reports the session derived route metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:431` |
| `pocketstation::session::lifecycle::observations::SessionExternalSourceMetrics` | struct | Reports the session external source metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:124` |
| `pocketstation::session::lifecycle::observations::SessionMetricsSnapshot` | struct | Authoritative point-in-time observations for the current Session boundary. | `src/session/lifecycle/observations.rs:36` |
| `pocketstation::session::lifecycle::observations::SessionOperatorInputMetrics` | struct | Reports the session operator input metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:242` |
| `pocketstation::session::lifecycle::observations::SessionOperatorMetrics` | struct | Reports the session operator metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:382` |
| `pocketstation::session::lifecycle::observations::SessionRouteMetrics` | struct | Reports the session route metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:139` |
| `pocketstation::session::lifecycle::observations::SessionSidecarMetrics` | struct | Exact bounded-queue and process-lifecycle accounting for one Session-owned language-neutral sidecar. | `src/session/lifecycle/observations.rs:133` |
| `pocketstation::session::lifecycle::observations::SessionSourceMetrics` | struct | Reports the session source metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:117` |
| `pocketstation::runtime::signal::observations::AsyncOperatorObservationHandle` | struct | Owns bounded access to async operator observation. | `src/runtime/signal/observations.rs:47` |

## Executable evidence

Executable evidence selected for **Observations and metrics** is limited to each test's recorded setup and assertions:

- `given_route_snapshot_when_drop_observed_then_rate_has_explicit_denominator_and_reasons` — given route snapshot when drop observed then rate has explicit denominator and reasons (`src/session/lifecycle/observations.rs:576`; `test-1402ef12bbb47b0d009a`).
- `given_route_snapshot_when_latency_observed_then_boundary_units_and_coverage_are_explicit` — given route snapshot when latency observed then boundary units and coverage are explicit (`src/session/lifecycle/observations.rs:604`; `test-b5b5d3d4a17670690cae`).
- `given_stop_and_join_failures_when_finalized_then_both_failures_and_observations_remain_true` — given stop and join failures when finalized then both failures and observations remain true (`src/endpoint/registry/tests.rs:294`; `test-da6484ed83753b351441`).
- `given_typed_operator_routes_when_stopped_then_final_state_and_metrics_are_truthful` — given typed operator routes when stopped then final state and metrics are truthful (`src/session/lifecycle/tests/running.rs:1114`; `test-89251e3206216dbcb480`).
- `given_orthogonal_provider_status_when_reconnecting_then_endpoint_metrics_remain_canonical` — given orthogonal provider status when reconnecting then endpoint metrics remain canonical (`tests/connector_contract.rs:719`; `test-0ebb1434ad7b7a743a83`).
- `given_saturated_connector_route_when_observed_then_drops_are_visible_in_session_metrics` — given saturated connector route when observed then drops are visible in session metrics (`tests/connector_contract.rs:836`; `test-440e0d0f038bd27e531f`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-a3a0d044f02b7f664bb9`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` — given zero frame capacity when preparing then backend is not prepared (`src/capture/capture_owner.rs:590`; `test-0afbec4242ea2fad4582`).

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

The claims on **Observations and metrics** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/observations.rs:1-636` (`DIRECT`)

For **Observations and metrics**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

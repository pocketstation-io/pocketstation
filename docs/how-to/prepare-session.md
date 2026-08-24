# Prepare resources before start

<!-- claims: CLM-GUIDE-030-SCOPE-001,CLM-GUIDE-030-TEXT-001,CLM-GUIDE-030-TEXT-002,CLM-GUIDE-030-TEXT-003,CLM-GUIDE-030-TEXT-004,CLM-GUIDE-030-TEXT-005,CLM-GUIDE-030-TEXT-006,CLM-GUIDE-030-SOURCE-001 -->

## Scope

- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

The scope of **Prepare resources before start** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

A frozen Session declaration that has already passed structural compilation.

## Procedure

1. Compile the immutable Session declaration and retain SessionCompileDiagnostic if validation fails.
2. Prepare resources and retain identity mappings.
3. Handle source and endpoint preparation errors.
4. Start with the intended cancellation option.
5. Preserve rollback failures alongside a primary start failure.

## Concrete repository example

The executable repository test `given_session_without_source_when_validated_then_topology_is_rejected` (`test-3ad011ae6ea2c1d8804b`) shows the concrete API sequence and asserted outcome at `src/session/lifecycle/control.rs:218`.

```rust
    }

    #[test]
    fn given_session_without_source_when_validated_then_topology_is_rejected() {
        assert!(!source_topology_has_input(0, 0, 0));
    }
```

```bash
cargo test --all-features given_session_without_source_when_validated_then_topology_is_rejected
```

## Important consequence

Preparation can perform I/O and resource acquisition; do not treat compilation success as start success.

## Verify the outcome

Every resource mapping is retained, start reaches running state, or a staged failure reports primary and rollback evidence.

Executable evidence selected for **Prepare resources before start** is limited to each test's recorded setup and assertions:

- `given_session_without_source_when_validated_then_topology_is_rejected` — given session without source when validated then topology is rejected (`src/session/lifecycle/control.rs:218`; `test-3ad011ae6ea2c1d8804b`).
- `given_supported_source_compositions_when_validated_then_each_is_accepted` — given supported source compositions when validated then each is accepted (`src/session/lifecycle/control.rs:207`; `test-430adf035ce3d16bc420`).
- `given_endpoint_prepare_failure_when_started_then_every_prior_owner_rolls_back` — given endpoint prepare failure when started then every prior owner rolls back (`src/session/lifecycle/tests/running.rs:2251`; `test-c924643ab2f4f9cf21d1`).
- `given_operator_prepare_failure_when_started_then_all_prior_owners_roll_back` — given operator prepare failure when started then all prior owners roll back (`src/session/lifecycle/tests/running.rs:1218`; `test-175f3b0edc08fa4b02f4`).
- `given_receive_before_enqueue_when_observed_then_latency_sample_is_rejected` — given receive before enqueue when observed then latency sample is rejected (`src/runtime/audio/router.rs:1229`; `test-0f111134a8c8ddb61a69`).
- `given_payload_above_branch_limit_when_published_then_all_branches_reject_before_fanout` — given payload above branch limit when published then all branches reject before fanout (`src/runtime/signal/edge.rs:536`; `test-26e5374e5c20a32afac4`).
- `given_every_nonaudio_signal_class_when_worker_prepares_then_exact_signal_context_is_received` — given every nonaudio signal class when worker prepares then exact signal context is received (`src/runtime/signal/operator.rs:1752`; `test-0769819bf6f85fc4186c`).
- `given_idle_worker_when_cancelled_then_cancel_hook_runs_before_close` — given idle worker when cancelled then cancel hook runs before close (`src/runtime/signal/operator.rs:2343`; `test-d5759eae676d2f1b0131`).
- `given_prepare_context_capacity_disagrees_with_runtime_edge_when_spawned_then_prepare_fails_closed` — given prepare context capacity disagrees with runtime edge when spawned then prepare fails closed (`src/runtime/signal/operator.rs:1808`; `test-94655e5366915899c2bd`).
- `given_prepare_failure_when_readiness_is_awaited_then_waiter_returns_false` — given prepare failure when readiness is awaited then waiter returns false (`src/runtime/signal/operator.rs:2179`; `test-b865f3f1c4ba6e60ac49`).
- `given_compiled_derived_route_when_runtime_prepared_then_compiled_topology_is_preserved` — given compiled derived route when runtime prepared then compiled topology is preserved (`src/session/compile/tests.rs:659`; `test-21f8c08b6457bb762def`).
- `given_graph_mismatch_when_start_fails_then_diagnostic_is_retained` — given graph mismatch when start fails then diagnostic is retained (`src/session/compile/tests.rs:867`; `test-604c0e001a7dcb5f87ae`).

## Failure signals

- `pocketstation::session::lifecycle::control::SessionStartError` / `CapturePrepare` — `error-cd9c5fe2a7f48f75a63f`
- `pocketstation::session::lifecycle::control::SessionStartError` / `EndpointPrepare` — `error-f08c687e1b9a85a431c8`
- `pocketstation::session::lifecycle::control::SessionStartError` / `ExternalSourcePrepare` — `error-59e2d48ac5c22ac45b2a`
- `pocketstation::session::lifecycle::control::SessionStartError` / `OperatorPrepare` — `error-058b95fba882a991d25a`
- `pocketstation::session::lifecycle::control::SessionStartError` — `error-c23c9a3b9e142e613f0e`
- `pocketstation::session::lifecycle::control::SessionStartError` / `Cancelled` — `error-14367931dacc2ea6803e`
- `pocketstation::session::lifecycle::control::SessionStartError` / `CaptureOpen` — `error-b4ace4191cf897863e82`
- `pocketstation::session::lifecycle::control::SessionStartError` / `EndpointStart` — `error-e4ad4265bd642c956528`
- `pocketstation::session::lifecycle::control::SessionStartError` / `ExternalAudioBridge` — `error-3a234ccba235becc2ab7`
- `pocketstation::session::lifecycle::control::SessionStartError` / `ExternalSourceStart` — `error-44e80a6f87ddc7f46d8b`

## API reference

- [Runtime Preparation](/docs/concepts/runtime-preparation.md)
- [Build Prepare Start](/docs/lifecycle/build-prepare-start.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::running::start_prepared_session` | function | Starts prepared session for `running`. | `src/session/lifecycle/running.rs:627` |
| `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | function | Starts prepared session cancellable for `running`. | `src/session/lifecycle/running.rs:643` |
| `pocketstation::session::lifecycle::control::SessionStartError::CapturePrepare` | variant | Classifies a failure at the capture prepare stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:166` |
| `pocketstation::session::lifecycle::control::SessionStartError::EndpointPrepare` | variant | Classifies a failure at the endpoint prepare stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:160` |
| `pocketstation::session::lifecycle::control::SessionStartError::ExternalSourcePrepare` | variant | Classifies a failure at the external source prepare stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:127` |
| `pocketstation::session::lifecycle::control::SessionStartError::OperatorPrepare` | variant | Classifies a failure at the operator prepare stage or component of `SessionStartError`. | `src/session/lifecycle/control.rs:152` |
| `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | variant | Classifies a failure at the prepare stage or component of `SessionEngineStartError`. | `src/session/lifecycle/engine.rs:321` |
| `SessionStartError::CapturePrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `CapturePrepare`. | `src/session/lifecycle/control.rs:170` |

## Related documentation

- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session API](/docs/reference/session.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)
- [Session failures](/docs/errors/session.md)

## Evidence boundary

The claims on **Prepare resources before start** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/prepare/mod.rs:33-92` (`DIRECT`)
- `src/session/prepare/mod.rs:94-175` (`DIRECT`)
- `src/session/prepare/mod.rs:178-446` (`DIRECT`)
- `src/session/prepare/mod.rs:448-466` (`DIRECT`)
- `src/session/prepare/mod.rs:468-503` (`DIRECT`)
- `src/session/prepare/mod.rs:505-511` (`DIRECT`)
- `src/session/prepare/mod.rs:506-506` (`DIRECT`)
- `src/session/prepare/mod.rs:507-507` (`DIRECT`)
- `src/session/prepare/mod.rs:508-508` (`DIRECT`)
- `src/session/prepare/mod.rs:509-509` (`DIRECT`)
- `src/session/prepare/mod.rs:510-510` (`DIRECT`)
- `src/session/prepare/mod.rs:513-1300` (`DIRECT`)
- `src/session/prepare/mod.rs:1302-1318` (`DIRECT`)
- `src/session/prepare/mod.rs:1320-1327` (`DIRECT`)
- `src/session/lifecycle/control.rs:1-4` (`DECLARED`)

For **Prepare resources before start**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

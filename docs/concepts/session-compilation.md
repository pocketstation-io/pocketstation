# Session compilation

<!-- claims: CLM-DOC-014-CAP-001,CLM-DOC-014-SOURCE-001 -->

## What it is

Audio buffer pools own a finite set of reusable sample buffers. Frames borrow or share that storage according to the route's ownership and copy contract.

## Why it exists

Realtime audio cannot rely on unbounded allocation. A finite pool makes storage exhaustion visible and keeps ownership transitions explicit.

## Relationships

- `AudioBufferPool` acquires reusable storage.
- `AudioFrame` freezes initialized storage for downstream consumers.
- Copy policy and fan-out determine whether a route shares or copies frame storage.

## Invariants and guarantees

- A writer cannot submit more samples than the acquired capacity.
- Only initialized samples become part of a frozen frame.
- Pool exhaustion is an observable result, not permission to allocate without bound.

## When you encounter it

- **Add an asynchronous operator** — Declare typed ports, implement an operator factory, and route its output.

## Use it

- [Inject external PCM](/docs/how-to/inject-external-pcm.md)
- [Keep realtime callbacks bounded](/docs/best-practices/realtime-boundaries.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)

## Scope

- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.

The scope of **Session compilation** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::compile::SessionCompiler` | struct | Compiles an immutable Session declaration into a validated graph and runtime plan. | `src/session/compile/mod.rs:41` |
| `pocketstation::graph::compile::resolve::Compiler` | struct | Runs the ordered graph-validation passes that resolve a graph specification into executable IR. | `src/graph/compile/resolve.rs:444` |
| `pocketstation::graph::compile::resolve::CompileError` | enum | Classifies failures reported as compile error. | `src/graph/compile/resolve.rs:26` |
| `compile` | function | Compiles its owned operation for `Compiler`. | `src/graph/compile/resolve.rs:464` |
| `compile` | function | Compiles its owned operation for `SessionCompiler`. | `src/session/compile/mod.rs:103` |
| `default` | function | Returns the default `Compiler` value. | `src/graph/compile/resolve.rs:513` |
| `new` | function | Creates a new `Compiler`. | `src/graph/compile/resolve.rs:449` |
| `new` | function | Creates a new `SessionCompiler`. | `src/session/compile/mod.rs:77` |
| `pocketstation::graph::compile::resolve::CompileError::AdapterUnavailable` | variant | Reported when the owning operation encounters adapter unavailable. | `src/graph/compile/resolve.rs:62` |
| `pocketstation::graph::compile::resolve::CompileError::ClockDomainMismatch` | variant | Reported when the owning operation encounters clock domain mismatch. | `src/graph/compile/resolve.rs:38` |

## Executable evidence

Executable evidence selected for **Session compilation** is limited to each test's recorded setup and assertions:

- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1251`; `test-2cf3d98ffa38e0f5ee68`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` — given derived stream when through called again then chain is preserved in session spec (`src/session/declaration/draft.rs:1167`; `test-aec2c4ee7ff8efede00a`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` — given operator when declared then session scoped instance and routes are preserved (`src/session/declaration/draft.rs:1305`; `test-e84db4efcd6a7145550a`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` — given unrouted stem when session frozen then validation fails closed (`src/session/declaration/draft.rs:1293`; `test-8e301580cdd23a244478`).
- `given_endpoint_operator_id_when_imported_from_session_then_endpoint_contract_type_is_reexported` — given endpoint operator id when imported from session then endpoint contract type is reexported (`src/session/declaration/endpoint.rs:174`; `test-0b2dadbe3265dde022e4`).
- `given_duplicate_type_when_session_nodes_registered_then_registry_is_unchanged` — given duplicate type when session nodes registered then registry is unchanged (`src/session/extensions/builtins.rs:565`; `test-df56c06567959a01bf75`).
- `given_structural_ingress_when_validated_then_session_metadata_is_not_required` — given structural ingress when validated then session metadata is not required (`src/session/extensions/builtins.rs:636`; `test-9a19e49a87f8cc918b10`).
- `given_custom_source_output_when_compiled_then_session_identity_and_typed_plan_are_preserved` — given custom source output when compiled then session identity and typed plan are preserved (`src/session/extensions/tests/composition.rs:300`; `test-b40764b43eebb0bca0ad`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-4d0f3e5a95ea9490a090`).
- `given_one_external_source_failure_when_session_runs_then_unrelated_source_completes` — given one external source failure when session runs then unrelated source completes (`src/session/extensions/tests/runtime.rs:734`; `test-9839e75a34cc80e4b057`).
- `given_session_without_source_when_validated_then_topology_is_rejected` — given session without source when validated then topology is rejected (`src/session/lifecycle/control.rs:218`; `test-3ad011ae6ea2c1d8804b`).
- `given_oversized_session_event_when_published_then_queue_owned_memory_stays_bounded` — given oversized session event when published then queue owned memory stays bounded (`src/session/lifecycle/events.rs:608`; `test-9e75be4a362fd68c5951`).

## Related documentation

- [Architecture overview](/docs/architecture/overview.md)
- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Runtime planner](/docs/internals/runtime-planner.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)

## Evidence boundary

The claims on **Session compilation** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/compile/mod.rs:1-790` (`DIRECT`)

For **Session compilation**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

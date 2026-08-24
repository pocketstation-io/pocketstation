# Session mental model

<!-- claims: CLM-DOC-006-SCOPE-001,CLM-DOC-006-TEXT-001,CLM-DOC-006-TEXT-002,CLM-DOC-006-TEXT-003,CLM-DOC-006-TEXT-004,CLM-DOC-006-TEXT-005,CLM-DOC-006-TEXT-006,CLM-DOC-006-SOURCE-001 -->

## What it is

A `Session` is the declaration-time owner of capture sources, external sources, operators, endpoints, routes, and recording intent. You assemble that model before PocketStation compiles or opens runtime resources.

## Why it exists

Keeping declaration separate from execution lets PocketStation reject cross-Session handles, missing destinations, incompatible ports, and unsupported topology before an active runtime owns devices or workers.

## Relationships

- `SessionSpec` is the frozen declaration produced from the mutable draft.
- `CompiledSession` resolves the declaration into a typed plan; `PreparedSession` owns prepared resources.
- `RunningSession` owns active execution and produces structured stop and recording outcomes.

## Invariants and guarantees

- A declaration handle belongs to exactly one Session.
- Freezing the draft ends mutation; later route changes are rejected.
- A source or derived stream must reach a destination before the declaration can become a valid plan.

## When you encounter it

- **Reach first captured frames** — Build a Session that captures an application and microphone and polls their independent frames.
- **Record separate stems** — Record independent source stems and inspect finalization outcomes after Session stop.

## Use it

- [Run the Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Stop and inspect failures](/docs/how-to/stop-session.md)

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.

The scope of **Session mental model** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::declaration::draft::DerivedStreamHandle` | struct | Holds the ownership or bounded access represented by derived stream handle. | `src/session/declaration/draft.rs:827` |
| `pocketstation::session::declaration::draft::EndpointHandle` | struct | Holds the ownership or bounded access represented by endpoint handle. | `src/session/declaration/draft.rs:580` |
| `pocketstation::session::declaration::draft::Operator` | struct | Declares one operator instance, including its stable operator identity and validated node configuration. | `src/session/declaration/draft.rs:282` |
| `pocketstation::session::declaration::draft::OperatorInputHandle` | struct | Holds the ownership or bounded access represented by operator input handle. | `src/session/declaration/draft.rs:701` |
| `pocketstation::session::declaration::draft::OperatorInstanceHandle` | struct | Holds the ownership or bounded access represented by operator instance handle. | `src/session/declaration/draft.rs:694` |
| `pocketstation::session::declaration::draft::Session` | struct | Owns a mutable Session declaration and the host configuration used to compile, prepare, and start it. | `src/session/declaration/draft.rs:316` |
| `pocketstation::session::declaration::draft::SourceInstanceHandle` | struct | Holds the ownership or bounded access represented by source instance handle. | `src/session/declaration/draft.rs:834` |
| `pocketstation::session::declaration::draft::SourceOutputHandle` | struct | Holds the ownership or bounded access represented by source output handle. | `src/session/declaration/draft.rs:910` |
| `pocketstation::session::declaration::draft::StemHandle` | struct | Holds the ownership or bounded access represented by stem handle. | `src/session/declaration/draft.rs:688` |
| `pocketstation::session::declaration::spec::ConnectionSpec` | struct | The single Session connection declaration used for every stream origin and every operator/endpoint destination. | `src/session/declaration/spec.rs:238` |

## Executable evidence

Executable evidence selected for **Session mental model** is limited to each test's recorded setup and assertions:

- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1251`; `test-2cf3d98ffa38e0f5ee68`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` — given derived stream when through called again then chain is preserved in session spec (`src/session/declaration/draft.rs:1167`; `test-aec2c4ee7ff8efede00a`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` — given operator when declared then session scoped instance and routes are preserved (`src/session/declaration/draft.rs:1305`; `test-e84db4efcd6a7145550a`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` — given unrouted stem when session frozen then validation fails closed (`src/session/declaration/draft.rs:1293`; `test-8e301580cdd23a244478`).
- `given_connector_endpoint_when_declared_then_allocated_identity_is_exposed` — given connector endpoint when declared then allocated identity is exposed (`src/session/declaration/draft.rs:1154`; `test-9f5f29ec97239eab31dd`).
- `given_derived_stream_without_destination_when_frozen_then_validation_fails_closed` — given derived stream without destination when frozen then validation fails closed (`src/session/declaration/draft.rs:1357`; `test-17c702ffaf38dad01e0a`).
- `given_empty_operator_id_when_endpoint_declared_then_descriptor_is_rejected` — given empty operator id when endpoint declared then descriptor is rejected (`src/session/declaration/draft.rs:1282`; `test-acb91c715c26103d2747`).
- `given_foreign_endpoint_when_derived_route_declared_then_error_is_immediate` — given foreign endpoint when derived route declared then error is immediate (`src/session/declaration/draft.rs:1378`; `test-c3e77a402eefae2294d5`).
- `given_foreign_endpoint_when_route_declared_then_error_is_immediate` — given foreign endpoint when route declared then error is immediate (`src/session/declaration/draft.rs:1235`; `test-de5b0287b18425cccf03`).
- `given_stale_handle_after_freeze_when_route_declared_then_mutation_is_rejected` — given stale handle after freeze when route declared then mutation is rejected (`src/session/declaration/draft.rs:1270`; `test-585025b498849099665d`).
- `given_two_record_declarations_when_frozen_then_default_group_identity_is_explicit_and_stable` — given two record declarations when frozen then default group identity is explicit and stable (`src/session/declaration/draft.rs:1212`; `test-79533e9048ae8212983b`).
- `given_two_stems_when_sent_to_one_endpoint_then_routes_are_distinct` — given two stems when sent to one endpoint then routes are distinct (`src/session/declaration/draft.rs:1144`; `test-3597ca95a8f2aa93709b`).

## Related documentation

- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [PocketStation documentation](/docs/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Capture application and microphone stems](/docs/how-to/capture-app-and-mic.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Fan out one source](/docs/how-to/fan-out-source.md)

## Evidence boundary

The claims on **Session mental model** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/declaration/draft.rs:19-19` (`DIRECT`)
- `src/session/declaration/draft.rs:21-21` (`DIRECT`)
- `src/session/declaration/draft.rs:22-25` (`DIRECT`)
- `src/session/declaration/draft.rs:23-23` (`DIRECT`)
- `src/session/declaration/draft.rs:24-24` (`DIRECT`)
- `src/session/declaration/draft.rs:27-27` (`DIRECT`)
- `src/session/declaration/draft.rs:28-33` (`DIRECT`)
- `src/session/declaration/draft.rs:29-29` (`DIRECT`)
- `src/session/declaration/draft.rs:30-30` (`DIRECT`)
- `src/session/declaration/draft.rs:31-31` (`DIRECT`)
- `src/session/declaration/draft.rs:32-32` (`DIRECT`)
- `src/session/declaration/draft.rs:35-35` (`DIRECT`)
- `src/session/declaration/draft.rs:36-40` (`DIRECT`)
- `src/session/declaration/draft.rs:37-37` (`DIRECT`)
- `src/session/declaration/draft.rs:38-38` (`DIRECT`)
- `src/session/declaration/draft.rs:39-39` (`DIRECT`)
- `src/session/declaration/draft.rs:42-42` (`DIRECT`)
- `src/session/declaration/draft.rs:43-47` (`DIRECT`)
- `src/session/declaration/draft.rs:44-44` (`DIRECT`)
- `src/session/declaration/draft.rs:45-45` (`DIRECT`)
- `src/session/declaration/draft.rs:46-46` (`DIRECT`)
- `src/session/declaration/draft.rs:49-49` (`DIRECT`)
- `src/session/declaration/draft.rs:50-53` (`DIRECT`)
- `src/session/declaration/draft.rs:51-51` (`DIRECT`)

For **Session mental model**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

# Session mental model

<!-- claims: CLM-DOC-006-CAP-001,CLM-DOC-006-SOURCE-001 -->

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
| `pocketstation::session::declaration::draft::DerivedStreamHandle` | struct | Owns bounded access to derived stream. | `src/session/declaration/draft.rs:839` |
| `pocketstation::session::declaration::draft::EndpointHandle` | struct | Owns bounded access to endpoint. | `src/session/declaration/draft.rs:592` |
| `pocketstation::session::declaration::draft::Operator` | struct | Declares one operator instance, including its stable operator identity and validated node configuration. | `src/session/declaration/draft.rs:294` |
| `pocketstation::session::declaration::draft::OperatorInputHandle` | struct | Owns bounded access to operator input. | `src/session/declaration/draft.rs:713` |
| `pocketstation::session::declaration::draft::OperatorInstanceHandle` | struct | Owns bounded access to operator instance. | `src/session/declaration/draft.rs:706` |
| `pocketstation::session::declaration::draft::Session` | struct | Owns a mutable Session declaration and the host configuration used to compile, prepare, and start it. | `src/session/declaration/draft.rs:328` |
| `pocketstation::session::declaration::draft::SourceInstanceHandle` | struct | Owns bounded access to source instance. | `src/session/declaration/draft.rs:846` |
| `pocketstation::session::declaration::draft::SourceOutputHandle` | struct | Owns bounded access to source output. | `src/session/declaration/draft.rs:922` |
| `pocketstation::session::declaration::draft::StemHandle` | struct | Owns bounded access to stem. | `src/session/declaration/draft.rs:700` |
| `pocketstation::session::declaration::spec::ConnectionSpec` | struct | The single Session connection declaration used for every stream origin and every operator/endpoint destination. | `src/session/declaration/spec.rs:238` |

## Executable evidence

Executable evidence selected for **Session mental model** is limited to each test's recorded setup and assertions:

- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1263`; `test-1682e00b3166c4846a92`).
- `given_derived_stream_when_through_called_again_then_chain_is_preserved_in_session_spec` — given derived stream when through called again then chain is preserved in session spec (`src/session/declaration/draft.rs:1179`; `test-837dd73be7d5c552ef15`).
- `given_operator_when_declared_then_session_scoped_instance_and_routes_are_preserved` — given operator when declared then session scoped instance and routes are preserved (`src/session/declaration/draft.rs:1317`; `test-69203660038a41959c14`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` — given unrouted stem when session frozen then validation fails closed (`src/session/declaration/draft.rs:1305`; `test-1633b6167eec91db04e2`).
- `given_connector_endpoint_when_declared_then_allocated_identity_is_exposed` — given connector endpoint when declared then allocated identity is exposed (`src/session/declaration/draft.rs:1166`; `test-161b990db51190f37641`).
- `given_derived_stream_without_destination_when_frozen_then_validation_fails_closed` — given derived stream without destination when frozen then validation fails closed (`src/session/declaration/draft.rs:1369`; `test-94e7fa143670693acd86`).
- `given_empty_operator_id_when_endpoint_declared_then_descriptor_is_rejected` — given empty operator id when endpoint declared then descriptor is rejected (`src/session/declaration/draft.rs:1294`; `test-3094497c2340606ba13f`).
- `given_foreign_endpoint_when_derived_route_declared_then_error_is_immediate` — given foreign endpoint when derived route declared then error is immediate (`src/session/declaration/draft.rs:1390`; `test-cdff89219c2eef8f0d45`).
- `given_foreign_endpoint_when_route_declared_then_error_is_immediate` — given foreign endpoint when route declared then error is immediate (`src/session/declaration/draft.rs:1247`; `test-c03c1e7f3591b6b9a633`).
- `given_stale_handle_after_freeze_when_route_declared_then_mutation_is_rejected` — given stale handle after freeze when route declared then mutation is rejected (`src/session/declaration/draft.rs:1282`; `test-11ff80196d57930753ca`).
- `given_two_record_declarations_when_frozen_then_default_group_identity_is_explicit_and_stable` — given two record declarations when frozen then default group identity is explicit and stable (`src/session/declaration/draft.rs:1224`; `test-ad12556b25e1d517daba`).
- `given_two_stems_when_sent_to_one_endpoint_then_routes_are_distinct` — given two stems when sent to one endpoint then routes are distinct (`src/session/declaration/draft.rs:1156`; `test-8ad7b42874649ad3c238`).

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

The claims on **Session mental model** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/declaration/draft.rs:1-1417` (`DIRECT`)

For **Session mental model**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

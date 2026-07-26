# Session API and lineage freeze — 2026-07-20 milestone

Status: **SAFE-TO-TEST / PARTIAL**. The W1 API and compile gate pass in the
protected working tree. Runtime route execution remains explicitly unavailable
until W2–W6 and no real-path product claim is made here.

## Purpose and product line enabled

This slice gives Rust developers one canonical `pks-audio` façade for declaring
application and microphone capture plus connector, browser/remote, and recording
routes. It enables the accepted quickstart to compile and makes invalid
configuration fail visibly.

It does not claim that those routes execute.

## Public façade

`pks-audio` now exports:

```text
Session · SessionState · SessionError · StopHandle
Source · ApplicationSelector · DeviceSelector · ProcessId · DeviceId
StemHandle · EndpointHandle · ConnectorHandle · ConnectorKey
```

The public example is:

```text
crates/pks-audio/examples/product_quickstart.rs
```

`pks` remains the future CLI/proof consumer. No second SDK engine was added.

## Lifecycle freeze

```text
Draft → Compiled → Starting → Running → Stopping → Stopped
  └──────────────────── any state → Failed
```

- `capture`, `send`, and `record` only mutate the Draft description.
- `run(self)` consumes the Session, transitions to Compiled, and validates the
  complete declaration before attempting startup.
- invalid selectors, missing routes, invalid endpoints, and cross-Session
  handles return typed `SessionError` values.
- `StopHandle::stop()` is idempotent; the first request returns `true`, later
  requests return `false`.
- a pre-start stop returns `StoppedBeforeStart` and leaves state `Stopped`.
- until W3 integrates the branched executor, a valid declaration returns
  `RuntimeNotIntegrated` and leaves state `Failed`. This is inventoried as
  `PARTIAL`; there is no fake success.
- failed Sessions are consumed and cannot be restarted.

## Source selectors

Accepted forms:

```rust,ignore
Source::application(ApplicationSelector::bundle_id("com.acme.meeting"))
Source::application(ApplicationSelector::process_id(ProcessId::new(pid)))
Source::application(ApplicationSelector::stable_id(source_id))
Source::application(ApplicationSelector::name("Meeting App"))
Source::microphone(DeviceSelector::default())
Source::microphone(DeviceSelector::id(DeviceId::new(device_id)))
Source::microphone_default()
```

`ProcessId` deliberately has no `From<u32>` implementation. Empty names,
bundle IDs, stable keys, and device IDs fail validation; PID zero fails; an
application stable selector must have `SourceKind::Application`. Selection is
declarative and remains separate from platform authorization. No application
selector falls back to system mix.

Name ambiguity, concrete source resolution, permission generations, and source
restart detection are W6/W7 runtime work; the public vocabulary leaves room for
platform capability reporting without claiming mobile capture parity.

## Lineage freeze

`pks-frame::FrameLineage` contains only compact hot-path fields:

| Field | Type/units | Meaning |
|---|---|---|
| `session_id` | opaque numeric ID | owning Session |
| `source_id` | opaque numeric ID | resolved capture source |
| `stem_id` | opaque numeric ID | source-aware named path |
| `clock_id` | opaque clock-domain ID | timestamp domain |
| `sequence_num` | unitless `u64` | monotonic sequence in the stem |
| `timestamp_start_ns` | monotonic nanoseconds | inclusive source-time start |
| `duration_ns` | nanoseconds | represented interval length |
| `source_generation` | unitless `u32` | concrete source incarnation |
| `discontinuity_epoch` | unitless `u64` | source/stem continuity generation |
| `permission_epoch` | unitless `u64` | authorization generation |

`timestamp_end_ns()` uses saturating addition.

`pks-frame::DeliveryLineage` contains route-specific metadata:

| Field | Type/units | Meaning |
|---|---|---|
| `endpoint_id` | opaque numeric ID | destination endpoint |
| `connector_id` | optional opaque numeric ID | external connector registry entry |
| `route_id` | opaque numeric ID | distinct bounded route |
| `enqueued_at_ns` | monotonic nanoseconds | route enqueue time |
| `delivered_at_ns` | optional monotonic nanoseconds | successful delivery time |
| `delivery_discontinuity_epoch` | unitless `u64` | destination continuity generation |

One frame may produce several delivery records. Strings and descriptive source
metadata remain in immutable registries rather than being copied per frame.
Embedding the lineage into every runtime envelope is W2/W3 work.

## Destination handle decision

`EndpointHandle` and `ConnectorHandle` are lightweight `Copy` values containing
Session-owned numeric IDs. A connector or browser handle can therefore be reused
for application and microphone stems. Every `StemHandle::send()` call allocates
a distinct `RouteId`, even when the endpoint is shared. Cross-Session handle use
is retained as a Draft error and fails at `run()`.

This avoids move-only quickstart friction without sharing mutable endpoint
configuration across the audio callback.

## Rejected alternatives

- A `product_quickstart` that only references a nonexistent future API: rejected;
  the accepted example now compiles.
- A successful no-op `Session::run()`: rejected because it would falsely claim
  route execution.
- Per-language engines: rejected; Rust is the canonical engine and later SDKs
  must own idiomatic lifecycle above a stable boundary.
- Raw `GraphSpec`, partition, pass, or provider enums in the façade: rejected.
- Implicit `u32` PID conversion and application-to-system-mix fallback: rejected
  as ambiguous and unsafe.

## Acceptance evidence

Run from the central repository:

| Command | Result |
|---|---|
| `cargo build -p pks-audio --example product_quickstart` | **PASS** |
| `cargo test -p pks-frame -p pks-capture -p pks-graph -p pks-audio` | **PASS:** 121 tests plus 1 graph doc test; zero failed |
| `cargo fmt --all -- --check` | **PASS** |
| `cargo clippy --workspace --all-targets -- -D warnings` | **PASS** |
| `bash scripts/check_protocol.sh` | **PASS** |
| quickstart without required environment | **EXPECTED FAIL:** names the missing `PKS_APPLICATION_NAME` |
| quickstart with required environment | **EXPECTED FAIL:** typed `RuntimeNotIntegrated` until W3 |

The current worktree contains protected pre-existing graph/runtime work, so an
isolated clean-checkout reproduction remains required before the W7 release
gate. The compile result is not mislabeled as that later proof.

## Remaining state

- `PARTIAL`: Session runtime execution, lineage attachment to real frames,
  concrete source resolution, permission state, and endpoint observations.
- `STUB`: none introduced.
- `MOCKED`: none introduced.
- `LOOPBACK-ONLY`: none introduced by W1.
- `BLOCKED`: W2 requires AUDIO-027 frame ownership; W4 requires AUDIO-028
  multistem format.

Scaffold inventory update: yes, `pks-audio Session execution` is recorded as
`PARTIAL` until W6.

Staff review decision: **PASS for W1 API freeze and W2 start; NEEDS MANUAL
REVIEW before any runtime/product claim.**

# AUDIO-029 — Embedded Session engine boundary

- Status: Accepted
- Date: 2026-07-25
- Owners: `pks-session`, `pks-session-c`, `pks-runtime`
- Serves: W11 canonical cross-language engine boundary

## Context

W10 accepted the frozen product candidate. W11 must now expose the same
capture-once, source-aware Session engine to Rust, Python, Node, and later
mobile façades without creating another media runtime in each SDK.

The central repository already owns compiled graph execution, bounded Bridges,
shared pooled frames, capture capability truth, lifecycle events, and exact
observations. It does not own a process IPC protocol or a signed helper
lifecycle. The codec-specific Opus C surface is isolated in `pks-codec-c`; it
is not the versioned Session control contract owned by `pks-session-c`.

The engine boundary has two viable deployment shapes:

1. an embedded native library loaded or statically linked by the host; or
2. a separately signed local helper reached through IPC.

The choice changes permission identity, packaging, latency, copying, failure
containment, lifecycle ownership, and mobile feasibility. It must be decided
before an ABI or language SDK fixes the wrong process boundary.

## Decision

W11 uses one **embedded native Session engine** owned by `pks-session` and one
sibling portability adapter named `pks-session-c`.

`pks-session` owns the public, host-neutral Session specification and lifecycle
composition. It freezes declarations, resolves exact sources, coordinates
transactional startup, retains the owners needed by a running Session, and
coordinates cancellation, draining, joining, and finalization. It compiles
declarations through `pks-graph`, executes the compiled plan through
`pks-runtime`, and consumes the existing capture, endpoint, frame, metric, and
recording contracts. It does not reimplement graph compilation or execution,
buffer pools, capture backends, connectors, or recorder workers.

`pks-session-c` projects that safe Rust contract into a versioned C ABI. It
owns only ABI records, generational foreign handles, marshalling, bounded
polling and lease projections, panic containment, header generation, and C
conformance fixtures. It must not become another Session engine.

`pks-audio` may temporarily re-export the Rust Session façade for source
compatibility. It must not retain or grow a second Session runtime.
`pks-codec-c` owns the separate codec compatibility ABI and preserves its
existing `pks_*` symbols; it is not the W11 engine contract.

The portable conformance boundary is a versioned C ABI, but languages are not
forced through it when a native Rust binding is safer or more idiomatic.
Python may bind `pks-session` through PyO3 and Node through Node-API. Swift and
Kotlin may use the C projection or generated/platform wrappers around it.
Every adapter must preserve the same Session semantics and must not reimplement
media execution.

## Evidence and W11 outcome

This ownership decision followed the live source at the accepted W10 candidate,
not an aspirational crate diagram. W11 then completed the named migrations:

- `pks-frame` defines `AudioBufferPool`, `AudioFrame`, and
  `SharedAudioFrame`;
- `pks-graph` defines `Compiler`, `RuntimePlanner`, `RuntimePlan`,
  `ExecutionPartition`, and `SafetyContract`;
- `pks-runtime` consumes `RuntimePlan` and defines
  `RealtimePlanExecutor`, `PlanScheduler`, `PlanEdgeRouter`, and bounded edge
  receivers;
- `pks-capture` defines source selectors, authorization and generation
  evidence, runtime source events, and bounded captured-frame streams, while
  target capture crates own native backend implementations;
- `pks-nodes` defines concrete factories and `MultistemRecording`, including
  explicit stop, join, and finalization outcomes;
- the legacy `pks-pipeline` package is retired after caller migration;
- `pocketstation` is the language-owned Rust façade and delegates Session
  semantics to `pks-session`; `pks-audio` is transitional compatibility only
  and owns no duplicate `RuntimeNotIntegrated` implementation;
- `pks-session-c` and `pks-codec-c` are separate, executable conformance
  boundaries; and
- the `pks` proof command consumes the reusable Session engine and endpoint
  contracts while retaining only proof selection and artifact rendering.

The dependency direction established by those facts is:

```text
pks-frame / pks-timing / pks-metrics
        -> pks-caps / pks-capture / pks-graph
        -> pks-runtime -> pks-endpoint -> pks-nodes
pks-capture -> target capture implementations
pks-runtime / pks-endpoint / pks-nodes / target capture implementations
        -> pks-session
        -> Rust façade / CLI / language adapters

pks-session <- pks-session-c
```

The last line is the Cargo dependency direction: `pks-session-c` depends on
`pks-session`; the safe engine never depends on the adapter.

### Why embedded

#### Permission identity

An embedded engine performs capture under the identity of the application or
executable that the user launched. On macOS this keeps TCC grants associated
with the embedding application. On Windows and Linux it keeps platform
authorization, session ownership, portal interaction, and diagnostics attached
to the visible host process.

A helper would become a distinct permission and signing identity. Silently
moving capture into that identity could invalidate existing grants, make
operator prompts name the wrong executable, and separate source ownership from
the application that requested the Session. W11 has no product requirement
that justifies that change.

#### Packaging and mobile

Desktop language packages can load one dynamic library. iOS and Android can
statically link the same engine beneath their platform adapters. A mandatory
helper would add installation, signing, upgrade, discovery, authentication,
and orphan-process policy on desktop and would not be a normal mobile
deployment shape.

The embedding package remains responsible for signing and distributing the
native artifact appropriate to its platform. W11 does not introduce a daemon,
installer service, privileged component, or background startup mechanism.

#### Latency and copies

Embedded control avoids an IPC hop and avoids serializing every audio batch.
It does not imply zero-copy end to end. AUDIO-027 still requires any
external-lifetime consumer to receive a copy from its own preallocated branch
pool so it cannot retain the capture pool.

The binding polls a bounded batch lease whose descriptors point at immutable
engine-owned branch-pool storage. That planned branch copy is the ownership
boundary. No additional copy is required merely to cross the C ABI.

#### Crash containment

Embedding cannot isolate the host process from an abort, native-library fault,
undefined behavior, or exhausted process resource. This is an accepted W11
tradeoff, not a hidden guarantee.

Every exported entry point must prevent a Rust unwind from crossing the ABI,
validate nullability, alignment where applicable, lengths, versions, states,
and handles before constructing Rust views over caller memory, and map a
contained panic to a stable typed error. The caller remains responsible for
supplying memory valid for the documented borrowing duration. Panic containment
does not claim to catch invalid caller addresses, process aborts, signals,
access violations, or native backend crashes.

Process crash isolation remains the principal advantage of a future helper.
The reversal triggers below require evidence before paying its permission,
packaging, lifecycle, latency, and copy costs.

## Authoritative safe engine contract

The safe Rust engine exposes only:

- Session construction;
- capture, stem, destination, and route declaration;
- compile;
- start and cancellation;
- bounded event polling;
- metric snapshot polling;
- idempotent stop; and
- typed terminal outcomes.

Capability/version negotiation, opaque foreign handles, C buffer sizing, and
foreign audio-batch lease polling are adapter projections owned by
`pks-session-c`; they are not internal engine architecture.

The lifecycle is monotonic:

```text
Draft -> Compiled -> Starting -> Running -> Stopping -> Stopped
                                             \------> Failed
```

Any non-terminal state may transition to `Failed` after an exact typed error;
`Failed` is never reported as a clean stop.

Declarations are copied and validated on the control path. They cannot mutate
a compiled Session. Start never reports success until the real runtime and
required endpoint owners have started. Stop is idempotent, prevents new
delivery, drains or explicitly discards bounded queues according to their
contracts, releases outstanding engine ownership, and reports finalization
errors through typed events and status.

Destroying an engine requests stop for its live Sessions and joins or
explicitly abandons only work whose bounded shutdown contract permits it.
Destruction must not report a clean stop while workers or recording
finalization have failed.

## Portable ABI adapter and versioning rules

The `pks-session-c` C ABI is a projection of the authoritative Session
contract, not a dump of Rust layouts and not the universal implementation path
for every language.

- The ABI has explicit major and minor versions. An unsupported major fails
  negotiation before Session construction. Minor-version negotiation and
  advertised capabilities control additive behavior.
- Every public input and output record carries a version and byte size. A
  caller may pass an older known prefix; the engine rejects undersized required
  fields and ignores only documented trailing fields.
- Public scalars use fixed-width integer types. Text is length-delimited UTF-8.
  Arrays and byte regions are pointer-plus-length views whose borrowing
  duration is stated by the function contract.
- Rust `bool`, `char`, `String`, `Vec`, `Option`, enum layout, trait object,
  allocator ownership, and unwinding are never part of the ABI.
- The engine copies declarations before returning. Caller-owned output buffers
  remain caller-owned. Buffer-too-small results return the required element or
  byte count without truncating authoritative data.
- Stable status codes distinguish invalid argument, unsupported version,
  invalid or foreign handle, invalid lifecycle state, empty or would-block,
  buffer too small, backend failure, cancellation, and contained internal
  panic. Optional error records preserve subsystem and native platform status
  without promoting native codes into the portable status family.
- Engine, Session, and batch ownership use opaque generational tokens. Rust
  addresses and internal IDs are not foreign handles. Stale, released, and
  foreign tokens fail as invalid handles instead of dereferencing freed Rust
  objects.
- No callback from Rust into a language runtime occurs on an audio callback or
  realtime partition. Events, metrics, and audio cross by bounded polling.
- Each operation documents its control-thread concurrency. Implementations may
  serialize control-plane mutation, but they may not add locks, allocation,
  blocking, logging, async work, or foreign calls to the audio hot path.

Capabilities and endpoint/operator identifiers are stable strings. The ABI
does not add closed provider, model, policy, or sink enums.

## Bounded audio leases

Audio is not sent as per-frame JSON, protobuf, or a per-frame foreign callback.

Each foreign audio destination is a compiled external partition with an
independent bounded edge and a preallocated branch pool. The runtime applies
`CopyToBranchPool` before foreign retention. The engine may freeze those branch
copies for immutable batch ownership; it never lends mutable buffers or lets a
foreign lease retain the capture pool.

One successful poll returns an opaque batch token plus a bounded array of frame
descriptors. Descriptors carry stable lineage IDs, sample format, sample rate,
channel count, sequence number, monotonic timestamps in nanoseconds, and
length-delimited sample regions. Their pointers remain valid only until the
batch is released or the documented terminal engine teardown completes.

Batch frame count, queue depth, branch-pool slots, and outstanding lease count
are fixed at Session compilation. If the destination queue or lease budget is
full, the newest delivery is dropped according to AUDIO-004 and the exact drop
reason is counted. Polling never creates an unbounded backlog. Releasing a
batch is explicit, one successful release relinquishes the entire batch, and a
second release returns an invalid-handle status without touching reclaimed
storage.

## Events, metrics, and errors

Lifecycle, source availability, permission, discontinuity, destination
failure, cancellation, and finalization events enter one bounded control-plane
projection. Existing owners remain authoritative:

- `pks-capture` owns capture authorization and source-runtime observations;
- `pks-runtime` owns edge, queue, discontinuity, and worker observations;
- `pks-endpoint` owns endpoint driver lifecycle and typed finalization outcome
  contracts;
- `pks-metrics` owns metric primitives and snapshots;
- `pks-nodes` owns concrete endpoint-specific observations and finalization
  behavior.

`pks-session` maps those facts into safe Session events and metric snapshots.
`pks-session-c` maps those values again into versioned foreign records. Neither
layer maintains competing counters or infers permission state from a generic
backend failure. Event-channel overflow and metric snapshot unavailability are
themselves explicit and measured.

Metric names and fields retain measurement units and cumulative counters retain
the `_total` suffix. A snapshot never exposes Rust atomics or references into a
mutable metrics implementation.

## Prohibited exports

The engine boundary must not expose:

- Rust traits, closures, futures, Tokio handles, channels, or task types;
- `GraphSpec`, graph IR, `RuntimePlan`, node factories, registries, or compiler
  internals;
- `AudioBufferPool`, exclusive/shared Rust handles, pool slot indices, or
  allocator ownership;
- platform COM, CoreAudio, PipeWire, JNI, Objective-C, or Swift objects;
- provider SDK clients, credentials, provider enums, model enums, policy
  categories, or closed endpoint taxonomies;
- unversioned Rust struct layout or serialized `Debug` output;
- synchronous per-frame calls into Python, JavaScript, Swift, Kotlin, or Java;
- unbounded event, metric, control, or audio queues;
- implicit source replacement, permission guessing, automatic restart, or
  system-mix fallback; or
- a second SDK-owned graph, timing, codec, recorder, or media scheduler.

Language SDKs own idiomatic syntax, cancellation adapters, package loading, and
conversion to their native value types. A Python or Node adapter may depend
directly on `pks-session`; direct use does not authorize it to own media
semantics. Browser JavaScript remains a relay client and does not embed desktop
native capture.

## Future helper compatibility

The Session specification, lifecycle states, stable identifiers, status family,
events, metric meanings, capabilities, and lease semantics are
transport-neutral. A future signed helper must preserve them and pass the same
conformance fixtures.

This does not make raw C structs an IPC wire format. A helper would need a
separately versioned, authenticated, length-delimited control protocol and a
bounded shared-memory or equivalent media transport. It may project the same
contract, but it must not serialize Rust memory or silently change permission
identity.

## Rejected alternatives

### Signed local helper in W11

Rejected because no current product consumer requires process isolation strongly
enough to justify a new permission identity, signed-helper packaging, process
authentication, orphan cleanup, IPC versioning, and an additional media
ownership boundary before the first language binding exists.

### Separate media runtime in each SDK

Rejected because it would split lifecycle, backpressure, timing, metrics,
errors, and recording semantics and make parity a permanent integration test
problem.

### JSON or protobuf for every audio frame

Rejected because serialization and per-frame allocation are unnecessary,
obscure pool ownership, and encourage unbounded language-runtime queues.

### Expose Rust graph and pool types directly

Rejected because Rust layout and ownership are not a stable cross-language
contract and would let foreign callers bypass compilation and hot-path safety.

## Consequences

- W11 adds `pks-session` as the one safe Session composition owner and
  `pks-session-c` as its sibling portable ABI adapter.
- `pks-graph` remains the sole owner of compilation and `RuntimePlan`;
  `pks-runtime` remains the owner of plan scheduling, execution, bounded edge
  routing, and runtime observations; platform crates remain the sole owners of
  capture implementation.
- The existing `pks-audio` Session `PARTIAL` inventory row remains until the
  real Session runtime migrates and the compatibility façade delegates to it.
- Documentation and contract-only tests cannot claim bindability. W11 exit
  requires a non-Rust harness that executes version negotiation, lifecycle,
  cancellation, error mapping, polling, lease release, and panic containment.
- Mobile adapters may statically link the engine, but their platform callback
  boundaries must still follow DOCS-001: pass preallocated storage once and
  perform no per-frame Swift or JNI call.
- A helper remains a reversible deployment choice only after the triggers
  below are met.

## Acceptance

W11 is not complete until one real engine and the C conformance harness prove:

1. compatible and incompatible version negotiation;
2. exact Session declaration, compile, start, stop, and cancellation states;
3. invalid, stale, released, and foreign handle rejection;
4. bounded event and metric polling with typed backend errors;
5. bounded multi-frame audio leases, stable data until release, pool
   reclamation, and counted lease exhaustion;
6. no Rust unwind crossing the ABI and explicit contained-panic mapping;
7. no per-frame foreign-runtime invocation or unbounded queue;
8. the quickstart/demo compile gate and CODE_PROTOCOL checks; and
9. no second media runtime in the CLI or a language SDK.

The harness proves the portable adapter, not language ergonomics. PyO3,
Node-API, Swift, and Kotlin façades remain later adapter work and must run the
same semantic conformance fixtures when introduced.

## Reversal triggers

Reconsider a signed helper only when at least one of these is demonstrated:

- a supported host requires process crash isolation as an acceptance condition;
- a stable, user-visible helper permission identity is preferable to the
  embedding application's identity and migration behavior is proven;
- packaging can install, authenticate, update, and remove the helper on every
  claimed platform, including orphan and version-skew handling;
- measured IPC latency, copying, bounded shared-memory ownership, and
  discontinuity behavior meet the product budgets; and
- the helper passes the same lifecycle, error, event, metric, lease, and
  cancellation conformance suite as the embedded engine.

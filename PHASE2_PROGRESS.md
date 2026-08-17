# Phase 2 Progress - PocketStation Runtime

## Core 1.0 extension-first freeze activation — 2026-08-13

- Status: `REAL` governance boundary after immutable Core 1.0 publication and
  independent registry consumption; active through 2028-08-13.
- AUDIO-034 is accepted. Provider, customer, model, exporter, storage and
  application-policy behavior must remain outside Core whenever a source,
  operator, endpoint/connector, transport, SDK projection or sidecar can
  express it.
- The pull-request template requires one approved Core-change category plus
  extension-model, compatibility and realtime analysis. CI and
  `CODE_PROTOCOL` run `tools/check-core-freeze-policy.sh` so the dated
  authority and review contract cannot silently disappear.
- This activates an ownership rule, not a new platform or performance claim.
  Windows/Linux evidence keeps its existing classification, and overall
  novelty or superiority still requires separate evidence.
- The first remote CI run found a stale public-boundary assertion that still
  classified `SidecarMessage` as private even though public `RunningSession`
  signal methods and every immutable 1.0 release expose the bounded PKSS
  projection. The allowlist/test now preserve that shipped SemVer contract
  while keeping process-host workers, queues and framing I/O private. No
  runtime, callback, pool, queue or protocol byte changed.
- The next remote run reached the new README doctest and rejected one stale
  `PolledAudioFrame::sequence_number()` call. The example now reads sequence
  from `FrameLineage`, matching the published API. This is documentation-only
  and changes no runtime contract.
- The following Linux CI run passed doctests and then exposed an existing
  recording-test scheduling race: its 200 one-millisecond polls could expire
  before a loaded runner scheduled the nested recorder failure. The test now
  uses a monotonic two-second deadline and retains the exact fail-closed and
  final-accounting assertions. Production timeout, worker, queue and recording
  behavior are unchanged.

## W20 final performance qualification — 2026-08-13

- Status: `SAFE-TO-MERGE` component gate within hash-accepted candidate
  `pks-20260813-final-performance-16`; competitive classification is
  `LOOPBACK-ONLY`.
- The exact candidate passes nine callback/codec/router no-allocation cells,
  the complete boundedness/recovery unit suite, and 20 Criterion cases with
  nanosecond distributions, throughput, process CPU and accepted-baseline
  deltas. The largest non-lifecycle component p99 is 106,499 ns against the
  20,000,000 ns audio-frame budget.
- The protocol naming gate exposed three compatibility/profile tests that did
  not use mandatory `given_when_then` names. Only those test names changed;
  public API, ABI, callbacks, pools, queues and runtime behavior did not.
- No long soak, physical-device rerun, Windows/Linux rerun, remote transport,
  publication, fastest claim, or Core-freeze claim is added.

## W20 API/ABI/package compatibility — 2026-08-13

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260813-w20-api-abi-freeze-15`; all three executor predicates are
  hash-accepted.
- The default Rust surface passes 196 pinned SemVer checks against the accepted
  0.1.2 baseline. The accepted C header remains byte-identical; public type
  layouts, exported symbols, callback-table prefix compatibility and the PKSS
  1.0 golden wire vector pass.
- Previous Session C, Extension ABI 1.0 C, executable Extension ABI 1.1 C and
  codec C++ consumer paths compile against their preserved headers and execute
  against the current library.
- Cargo created and verified the locked crate from an exact clean committed
  snapshot without `--allow-dirty`, `--no-verify` or source patches. This gate
  changes no callback, pool, bounded queue or realtime executor and makes no
  publication, comparative-performance or Core-freeze claim.

## W20 public single-engine CLI projection — 2026-08-13

- Status: `SAFE-TO-TEST` inside active candidate 12; physical acceptance is
  pending.
- Central `CaptureSource` now owns selector-persistence and process-tree scope
  observations, so the CLI no longer reconstructs private capture modes.
- `EndpointAudioReceiver` exposes immutable public edge observations, allowing
  external process and relay connector packages to receive bounded audio
  without importing `PlanEdge*` runtime machinery.
- The protocol gate now runs `scripts/check_pks_single_engine_boundary.sh`,
  which rejects any return of a CLI-owned capture/compiler/runtime/pool/queue
  authority or private connector access.
- The realtime callback, fixed pools, bounded SPSC edges, saturation policy and
  audio execution path are unchanged. No soak or comparative claim was added.

## W20 public source-discovery ownership — 2026-08-13

- Status: `SAFE-TO-TEST` inside the active pks single-engine closure.
- Canonical capture now exposes platform-neutral source discovery and query
  contracts through the public SDK. macOS application/input discovery,
  Windows discovery and Linux discovery are merged and deduplicated in the
  capture owner; the CLI no longer requires `internal-testing` merely to list
  or resolve sources.
- `application_capture_available()` is a control-plane capability query only:
  it opens no capture source and creates no callback, pool, queue, compiler or
  runtime owner.
- The realtime capture implementation, pool sizes, bounded edges, saturation
  semantics and callback code are unchanged. No scaffold, mock, provider path,
  physical claim or soak was added.

## W20 Session-owned sidecar host — 2026-08-12

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260812-w20-sidecar-host-6`; acceptance is executor-manifest bound.
- Public `Session::register_sidecar` retains a versioned process contract and
  the canonical engine transactionally spawns and attaches every child.
- The bounded PKSS host owns separate data and reserved control queues, the
  `Spawned -> Hello -> Manifest -> Configure -> Ready -> Running` handshake,
  close/cancel acknowledgement, deadlines, typed failures, kill/wait/reap and
  final Session observations. Foreign work stays off realtime callbacks.
- The external Python fixture proves typed signal echo, close and cancel state
  transitions, data saturation with control delivery, crash isolation,
  hung-child deadline/kill/reap and malformed-frame failure during Session
  start. Exact acceptance, strict Clippy and release quickstart gates pass.
- No callback, audio pool or hot executor changed, so no endurance gate was
  triggered. Cross-language conformance and Core 1.0 freeze remain pending.

## W20 registered built-in lowering closure — 2026-08-12

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260812-w20-builtin-lowering-closure-5`; acceptance remains bound to
  the executor manifest rather than inferred from code existence.
- Generic Session compilation no longer matches application/microphone source
  variants or owns built-in/external-audio node type IDs. One typed
  `SessionSourceLoweringContext` carries the pipeline, source registry, typed
  node maps, and compiled bindings to component-owned lowerers.
- Session engine bootstrap no longer constructs a fixed structural node list or
  a separate lowerer list. Component registration derives collision checks from
  the registered descriptors and returns the exact lowerers installed for that
  engine.
- Built-in application/microphone capture, registered custom sources, registered
  PCM sources, and generated-audio reentry now lower through the same compiler
  extension seam while retaining their specialized runtime execution paths.
- Gates pass: the static boundary predicate, 443 unit tests plus all targets,
  ABI/integration and allocation tests, every benchmark target, strict Clippy,
  default-feature check, release quickstart, and the full protocol check.
- No callback, frame pool, `rtrb` edge, saturation policy, hot-path `Drop`,
  realtime executor, provider/domain type, physical claim, or soak changed.

## W19 composition fault and bounded-scaling closure — 2026-08-08

- Status: `SAFE-TO-MERGE`; all W19 executor tasks and mandatory predicates are
  `DONE`/`PASS` and execution is deliberately `PAUSED` before W20.
- Ten exact deterministic cells prove Session branch saturation isolation,
  independent source survival after another source fails, operator timeout and
  bounded cancellation, typed-edge branch isolation, recorder-branch failure
  isolation, generated-audio pool/ingress exhaustion accounting, composed
  Session ownership, and transactional endpoint-start rollback.
- The hash-verified artifact binds capacity 8, peak depth 4, final depth 0,
  eight pool slots, 960 samples per frame, and the exact 61,440-byte queued
  audio upper bound. Four signals were enqueued and received with zero
  unexplained loss.
- W19 starts no sidecar child process; process lifecycle remains a gated W20
  concern. No physical-device, remote-network, endurance, or product claim was
  added, and no soak ran.

## W19 Session-owned generated-audio reentry — 2026-08-08

- Status: `SAFE-TO-MERGE` for task `W19-GENERATED-AUDIO-REENTRY`; its three
  mandatory predicates are bound to the hashed acceptance manifest. The later
  deterministic fault/scaling task also passed, so W19 is now `DONE`.
- Public `Session` owns an exclusive bounded typed-PCM receiver, normalization
  bridge, dedicated pool, authoritative lineage projection, existing
  plan-source ingress, cancellation, graceful finish, join, and final metrics.
  Operator PCM output stays on the typed async lane until this explicit bridge;
  captured realtime audio stays on the specialized audio lane.
- The generic typed edge now reports exact capacity, current/peak depth,
  enqueue, receive, and declared-drop counters. Its final fan-out branch moves
  the sole `Arc` rather than retaining a racing publisher reference, making
  exclusive generated-audio ownership deterministic.
- Compilation rejects a second consumer of the same named PCM output, including
  a second reentry. Deterministic tests cover ingress saturation, pool
  exhaustion, graceful drain, source/operator/bridge close, and a downstream
  endpoint-delivery scheduling race; the public Session test passes 25 repeated
  runs.
- Final gates pass: 390 unit tests plus every target/integration, strict
  all-target/all-feature Clippy, no-default check, quickstart, hot-path
  allocation tests, full code protocol, and the external public-API consumer.
- No callback, pool ownership, hot executor, provider/domain type, mock,
  physical-device claim, product-claim upgrade, or soak was introduced.

## W17 central Core 1.0 boundary corrections — 2026-08-08

- Status: `SAFE-TO-MERGE` for task
  `W17-CENTRAL-BOUNDARY-CORRECTIONS`; executor completion is bound to the
  hashed acceptance manifest, not inferred from component test success.
- Public `SignalSpec`, `PortSpec`, `EdgeContract`, `NodeDescriptor`, operator
  manifests, source manifests, envelopes, frames, and lineage records expose
  checked construction and read-only access rather than mutable public
  representations.
- `SessionSpec` owns one `ConnectionSpec` collection. Captured stems, external
  source outputs, and operator outputs delegate through one internal stream
  handle; Rust `Stream<T>` remains declaration-time typing over stable
  `SignalSpec`, not a second runtime or an ABI type.
- Graph preparation has one port-aware record. The asynchronous bridge and
  typed fan-out now use one bounded `SignalEdge` implementation with explicit
  owned versus shared payload ownership; the specialized audio edge remains
  separate.
- The generic Session compiler no longer switches on connector, browser,
  recorder, or generated-audio identities. Endpoint-owned configuration is
  copied generically and the audio-reentry package enters through the
  registered graph-lowering seam.
- Unaccepted DSP, codec profile/mock, capture compatibility, recording
  coordination, runtime-node, and experimental timing scaffolds are absent
  from the shipping package. The empty experimental directory was removed and
  is not recoverable except from version control.
- Acceptance passes: strict no-default dead/unused check, 385 central unit
  tests plus all targets and integrations, strict Clippy, release quickstart,
  protocol/hot-path laws, and the external public-consumer artifact with an
  adversarial verifier self-test.
- No audio callback execution, buffer-pool ownership, or hot executor changed;
  W10 endurance evidence remains applicable and no soak was run.

## W18 Session-owned external source lifecycle — 2026-08-08

- Status: `SAFE-TO-MERGE` for task `W18-SOURCE-LIFECYCLE-EVIDENCE`.
  Central gates and the external public-Session artifact pass; executor
  completion remains hash-binding work and is not inferred from this label.
- `PreparedSession` now owns external source branch mappings. Startup prepares
  every source driver, bounded typed branch, endpoint, and PCM ingress bridge
  behind the common closed start gate; only a fully prepared transaction
  starts workers. `RunningSession` owns cancellation, joins, final source
  observations, and failure accounting.
- Custom typed signals use the shared bounded typed-edge runtime without an
  invented audio `SampleSpec`. External PCM crosses one bounded generated-audio
  ingress boundary into the existing specialized audio plan. The realtime
  callback, audio pool owner, and realtime executor are unchanged.
- `SourcePrepareContext` supplies the exact Session, source, and named-output
  stream identities. Runtime emissions fail closed on identity mismatch and
  observe generation, discontinuity, recovery, and policy transitions.
  Endpoint preparation receives a typed source-route identity; non-audio
  endpoint preparation no longer manufactures an audio context.
- Deterministic tests prove a typed-only Session, external PCM, saturated-branch
  isolation, generation/discontinuity transition, independent-source survival
  after another source fails, transactional gate ordering, cancellation, join,
  and clean driver close. The existing application/microphone suite remains
  green.
- Acceptance passes: 506 central unit tests plus all ABI, allocation,
  conformance, facade, integration, and quickstart targets; strict all-target/
  all-feature Clippy; release `product_quickstart`; and the external Lab
  verifier/artifact using public `Session` only.
- No scaffold, mock, provider/customer/domain type, direct-registry acceptance
  fixture, realtime hot-path change, physical-device claim, product-claim
  upgrade, or soak was introduced.

## W18 Session source declaration and compiler lowering — 2026-08-08

- Status: `SAFE-TO-MERGE` for task `W18-SESSION-SOURCE-COMPILER`; W18 as a
  whole remains `PARTIAL` until the next executor task proves source worker
  lifecycle, cancellation, replacement, failure, saturation, observations,
  and an external public-API consumer.
- The shipping public `Session` now declares an external source by open
  `SourceTypeId`, retains caller-owned `SourceFactory` registrations, selects
  named manifest outputs, and exposes stable Session-assigned source, stream,
  and instance identities. The internal declaration owner freezes those values
  into Session schema 1.3 without adding a closed `Source` variant.
- `SessionEngineBuilder` validates and registers each source manifest as a
  zero-input graph definition. Session compilation resolves the exact source
  factory and configuration, validates every selected output, lowers direct
  and operator-bound routes through the normal graph compiler, and records
  connected root outputs in `RuntimePlan`.
- Audio outputs use the existing bounded audio memory plan; custom typed
  outputs use the existing bounded typed-edge plan. No signal-specific queue,
  invented non-audio `SampleSpec`, worker lifecycle, or alternate graph/runtime
  was introduced. Application and microphone declarations retain their
  optimized path.
- Acceptance passes: 503 unit tests plus every ABI, allocation, conformance,
  façade, and integration target; 8 focused public/Session source tests; the
  neutral graph-root boundedness predicate; strict all-target/all-feature
  Clippy; release `product_quickstart`; formatting; and
  `scripts/check_protocol.sh`.
- No scaffold, mock, loopback path, provider/customer/industrial type,
  realtime callback change, physical-device claim, product-claim upgrade, or
  soak was introduced.

## W17 final signal/API hardening — 2026-08-08

- Status: `SAFE-TO-MERGE`; all central gates and the clean external-contract
  proof pass. Final executor `DONE` remains commit- and hash-binding work, not
  an inferred result from these commands.
- `SignalEnvelope` now has exactly five authorities: `payload`, `spec`,
  `timing`, optional source-independent `lineage`, and optional generic
  `derivation`. The former mutable mirror fields and audio-origin-only derived
  lineage contract are removed.
- `SignalDerivation` references generic upstream `SignalLineage` and
  `SignalTiming`. `FrameLineage` is projected once when specialized
  `AudioFrame` crosses into the typed lane; the realtime audio executor and
  pooled frame representation remain specialized.
- Transcript role constructors/constants moved to the external Whisper example.
  The core event payload is now an open type-id/bytes record, generated audio
  uses the generic `Generated` provenance tag, and the future diarization-only
  frame field is removed. No provider, customer, industrial, `Flight*`, or
  transcript-policy authority remains in the public core implementation.
- The external Whisper package compiles warning-free and passes all 15 tests on
  the canonical contract. The Lab hardening proof compiles a separate public-
  API consumer, checks exact canonical fields and payload coverage, and proves
  the removed duplicate API fails compilation for the intended reason.
- Acceptance passes: 492 central unit tests plus every ABI/allocation/facade
  target, strict all-target/all-feature Clippy, release `product_quickstart`,
  `scripts/check_protocol.sh`, the Lab adversarial verifier, and the exact
  candidate artifact verifier. No mock, scaffold, physical-device claim,
  product-claim upgrade, or soak was introduced.

## W20 Core 1.0 extension completeness — 2026-08-08

- 2026-08-09 ownership-remediation slice: `RUNNING`, not accepted. The central
  package is being reorganized around explicit authorities:
  `session/{declaration,compile,prepare,lifecycle,extensions}`,
  `runtime/{audio,signal,bridge,lifecycle}`, split frame/capture/graph/endpoint/
  recording modules, and restored codec/timing owners. Empty legacy module
  directories are gone; no DSP placeholder was restored.
- `CompiledSessionBindings` is now the sole typed bridge from lowered graph
  nodes to Session declarations. Built-in application/microphone selectors are
  no longer serialized into `NodeConfig`; structural ingress nodes receive an
  empty configuration and runtime preparation consumes the typed Session
  declaration. External source, operator, and endpoint configuration remains
  extension-owned and opaque.
- Connector identity now follows the same typed path: `ConnectorId` is retained
  by the endpoint declaration, frozen into `EndpointSpec`, lowered into the
  compiled endpoint binding, and supplied through `EndpointPrepareContext`.
  The former `"connector_id"` configuration injection and endpoint-side
  string parser are removed.
- Endpoint authoring contracts now have one public Rust location at the crate
  root. The endpoint implementation namespace is private, and root contracts
  are re-exported from their actual graph, frame, declaration, extension, and
  lifecycle owners instead of being presented as Session-owned machinery.
- Realtime/session numeric identities retain their compact scalar layout and
  internal zero-cost field access, but the tuple field is no longer public.
  External consumers use `Id::new(...)` and `.get()`, preventing the scalar
  representation from becoming a frozen public-field contract. The central
  examples, integration targets, active CLI, and Lab fixtures were migrated;
  archived `audio-ml` remains archived rather than being pulled into Core.
- The former mixed graph signal file is now separated into payload,
  timing, lineage/derivation, envelope validation, continuity, asynchronous
  preparation, and operator-contract modules. `SignalTiming` and
  `SignalLineage` now have checked constructors and read-only accessors;
  `SignalDerivation` is read-only after construction. Current external examples
  and Lab fixtures were migrated away from writable representation fields.
- Capture remains an explicit owner in this cleanup. Platform adapters retain
  fixed-capacity audio pools, bounded `rtrb` crossings, nonblocking saturation
  accounting, and the callback prohibition on allocation, locking, blocking,
  async work, logging, and panic. Existing capture/pool Criterion targets are
  preserved. This slice changes no callback, pool, queue, or drop semantics.
- The misleading `capture/source/` namespace was removed. Capture selection,
  authorization, stable native identity, callback observations, runtime events,
  and sample-time mapping now have direct modules under `capture/`; the generic
  extensible Source contract remains exclusively under `session/extensions/`.
- The lifecycle start/stop contract has been separated from the Session
  runtime orchestrator, and language-neutral signal identifiers are opaque
  rather than public tuple representations. No provider/customer/domain type,
  second engine, mock, fallback, or loopback path was introduced.
- Verification is deliberately deferred by user direction. This entry records
  implementation state only and does not upgrade W20, Core 1.0, performance,
  platform, or release acceptance.

- Status: `PARTIAL`; the component/package candidate passes, but Core 1.0 is
  not frozen. The preserved artifact is package `0.1.2`, `LOOPBACK-ONLY`,
  produced with `--allow-dirty --no-verify`, and explicitly records
  `clean_worktree_claimed=false`.
- The public Rust `Stream<T>` façade provides compile-time composition without
  creating a generic runtime. External packages define marker types through
  `StreamSignal`; runtime and cross-language identity remains the stable
  `SignalSpec`, schema, named-port, edge-contract, and plan representation.
- `pocketstation.h` now exposes a versioned source/operator/endpoint descriptor
  ABI. Every record validates ABI version, struct size, pointer alignment,
  lengths, UTF-8, port direction, unique names, and extension shape before any
  caller memory can be retained.
- The versioned `PKSS` sidecar frame carries stable signal identity, role,
  schema, sequence, timestamp, bounded payload, terminal state, and control
  kind. Decode rejects unknown versions, invalid flags, oversized fields,
  invalid UTF-8, truncation, and trailing bytes. It performs no callback work.
- The packaged `pocketstation 0.1.2` proof builds an exact-version external Rust
  consumer, replays W18 open-source and W19 composition contracts, links a C
  descriptor consumer to `libpocketstation`, and round-trips a Python sidecar.
  The independently verified artifact remains `LOOPBACK-ONLY`; it upgrades no
  platform or product claim.
- Full central tests (494 unit tests plus ABI, allocation, façade, and
  integration gates), strict all-target/all-feature Clippy, release quickstart,
  Bench (36 tests), Python SDK (19 tests), and Node SDK (14 tests) pass.
- No provider, customer, industrial/domain payload, second engine, scaffold,
  mock, hot-path change, physical-device claim, or soak was introduced.
- Remaining exit: executable C source/operator/endpoint registration,
  Session-owned bounded sidecar lifecycle, completed W18/W19 Session paths,
  clean installed or published external consumption, compatibility gates, and
  a real `1.0.0` release.

## W19 operator composition and generated-audio reentry — 2026-08-08

- Status: `PARTIAL`; focused low-level composition and generated-audio gates
  pass, but the execution fixture manually assembles workers, fanout,
  plan-source ingress, and bridge outside Session.
- `DerivedStreamHandle` can declare another `through(...)` stage. Explicit
  `through_ports(...)` and `output(...)` selection lower named operator ports
  into the Session graph; simple `through(...)` remains fail-closed 1x1 sugar.
- `AsyncOperatorWorker::spawn_composed(...)` executes multiple named bounded
  typed inputs and outputs. Three independently registered external operators
  execute in sequence through the shared `TypedEdgeFanout` runtime, and
  multi-input/multi-output manifests route by declared `SignalSpec`, schema,
  media, and semantic role.
- Asynchronously produced PCM is accepted only through
  `GeneratedAudioBridge`: an exclusive typed branch validates format and frame
  size, copies into a dedicated bounded pool, restores authoritative timing and
  lineage, then performs a nonblocking send into the existing plan-source
  ingress. The realtime callback/executor contract is unchanged.
- Runtime policy and observations remain signal-generic. No transcript,
  provider, customer, telemetry, industrial, or other domain payload type was
  added; no scaffold, mock, physical-device claim, or soak was introduced.
- Remaining exit: Session-owned execution of derived non-audio chains, one
  operator instance with multiple named upstream inputs/outputs, and bounded
  generated-audio route lifecycle with fault isolation.

## W18 open source registration and bounded typed ingress — 2026-08-08

- Status: `PARTIAL`; focused low-level central and external-consumer gates
  pass, but the fixture constructs `SourceRegistry` directly and does not
  register or declare the external source through Session.
- Public `SourceManifest`, `SourceTypeId`, `SourceFactory`, `SourceDriver`,
  `SourceRegistry`, configuration, emission, lifecycle, cancellation, and
  observation contracts accept externally owned source implementations without
  provider or domain enums in core.
- `TypedEdgeFanout` is the one bounded source-independent fan-out owner for
  external typed sources and async operator outputs. Each branch has explicit
  capacity, delivery/drop counters, and fail-closed `MustDeliverOrFail`
  terminal behavior.
- Source outputs validate declared port, `SignalSpec`, schema, `MediaCaps`,
  lineage, sequence, time, discontinuity, generation, and replacement before
  delivery. External audio remains on the non-callback source boundary and
  retains authoritative `FrameLineage` projection.
- The specialized realtime `AudioFrame` executor and native application and
  microphone capture remain unchanged. No named-port composition, operator
  chaining, generated-audio reentry, sidecar, provider behavior, scaffold,
  mock, or product-claim upgrade is introduced.
- Remaining exit: public Session factory registration and external-source
  declaration compiled into Session start/stop/cancel/replacement/fault and
  observation semantics. Current Session still requires exactly one
  application and one microphone source.

## W17 source-independent signal contract — 2026-08-08

- Historical status: `DONE` at the earlier named `LOOPBACK-ONLY` contract
  boundary; superseded for Core 1.0 acceptance by the final W17 hardening task
  above.
- The public async boundary is now `SignalEnvelope` plus `SignalPayload`, with
  source-independent `SignalLineage` and `SignalTiming`. The optimized
  `AudioFrame` realtime lane is unchanged.
- Schema-backed custom signals, encoded audio, text, events, metrics, control,
  and binary payloads have explicit `SignalSpec`/`MediaCaps` symmetry. Invalid
  payload/specification pairs fail closed.
- `SignalContinuityTracker` validates stable identity, monotonic timestamps,
  sequence continuity, declared discontinuities, source generations, and
  policy epochs deterministically. Generic terminal delivery is
  `MustDeliverOrFail`; the runtime no longer owns transcript-named policy or
  observation concepts.
- The misleading public `SessionFlight*` vocabulary is removed before API
  freeze. Session lifecycle diagnostics now use `SessionTrace*`,
  `session_trace(...)`, `.pkstrace`, and `trace.validate()` consistently.
- Full central acceptance passes: 482 unit tests plus C/C++ ABI, allocation,
  conformance and façade tests; strict all-target/all-feature Clippy; and the
  release `product_quickstart` build.
- No source factory, typed source ingress, operator chaining, named-port API,
  generated-audio bridge, sidecar, provider integration, scaffold, mock, or
  product-claim upgrade is introduced in this step.

## W13 recorder initialization observation repair — 2026-08-02

- Status: `SAFE-TO-TEST`; candidate
  `pks-20260802-w13-operational-trust-3` is frozen as
  `pocketstation-0.1.2.crate` with SHA-256
  `e6943cad4c16af22492d880a28e0e6c10b957d031352c675dd405efd41c42657`.
- The Session multistem recorder now publishes received-frame progress as soon
  as it accepts each authoritative first frame. Previously the outer endpoint
  telemetry remained at zero while filesystem-backed recorder initialization
  ran, so a valid received frame could be temporarily invisible under load.
- The permission-epoch fail-closed test exposed the race during the full
  all-target gate. The focused regression now passes without weakening the
  permission lineage assertion or recording finalization behavior.
- The complete central gate passes: 470 unit tests plus C/C++ ABI,
  allocation, integration, façade, quickstart, package verification, strict
  Clippy, CODE_PROTOCOL, and architecture constraints.
- This changes observation timing only. It adds no queue, sleep, capture
  fallback, scaffold, mock, or product claim.

## Historical W13 `-2` Core Audio timeline candidate — 2026-08-02

- Historical status: `PARTIAL`; candidate
  `pks-20260802-w13-operational-trust-2` is frozen as
  `pocketstation-0.1.2.crate` with SHA-256
  `6d4de7597af33d5ceaba0724ba30420b2f3c170691f769018ba4e59ea01ae5bf`.
- The macOS process-tap callback now publishes the native Core Audio host time
  together with its absolute sample-frame position through a lock-free
  seqlock snapshot. Rust converts that native sample timeline into the shared
  PocketStation process-monotonic clock. It fails closed if the native
  timeline is unavailable instead of anchoring time when Rust first polls.
- The callback remains allocation-free, lock-free, blocking-free, async-free,
  log-free, and panic-free. Focused timeline tests cover reader positions on
  both sides of the callback anchor and native-host to process-clock mapping.
- Central formatting, 470 unit tests plus C/C++ ABI, allocation and façade
  tests, all-target/all-feature strict Clippy, CODE_PROTOCOL, architecture
  constraints, package verification, CLI 195 tests plus strict Clippy,
  neutral Bench 36 tests plus strict Clippy, and relay race tests pass.
- Physical macOS evidence for the exact rebuilt CLI binary proves two distinct
  application process incarnations, disappearance and explicit reselection,
  two complete Sessions, independent application and microphone recording,
  connector and same-host browser routes, browser reconnect, zero route drops,
  zero continuity gaps, and complete common-clock latency sample coverage.
- This does not prove denied/revoked permission transitions, Windows or Linux
  native behavior, the final 3,600-second soak, or clean-source reproduction.
  W13 remains `PARTIAL`; no scaffold, mock, fallback, or loopback-only product
  path was added.

## W16 single-package consolidation — 2026-08-02

- Status: `SAFE-TO-MERGE`; W16 is `DONE` at its local consolidation boundary.
  The central implementation is exactly one Cargo
  package named `pocketstation`, with internal frame, timing, graph, runtime,
  capture, endpoint, recording, codec, DSP, Session, observation, and ABI
  modules. The old `crates/` package tree is removed.
- The package emits Rust, static-library, and dynamic-library forms. Native
  consumers include `pocketstation.h` and link `libpocketstation`; the former
  Session and codec C packages are unified ABI modules. Retained `pks_*`
  symbols are compatibility only.
- The CLI, Python and Node native adapters, external examples, Lab fixtures,
  and neutral benchmark now consume the root package. The CLI's 195 tests,
  benchmark's 36 tests, strict Clippy, relay race tests, and central 463 unit
  tests plus ABI/allocation/integration tests pass locally.
- Fresh independent W12 Rust, W12 Python/Node, W15 real-whisper, and
  session trace artifact verifiers pass after consolidation. Their
  classifications remain `LOOPBACK-ONLY` or `PARTIAL`; this work creates no
  new physical-device claim.
- AUDIO-033 supersedes the historical multi-package topology. `runtime::metrics`
  is only the runtime observation implementation, not a separate product or
  package. New Cargo packages require an independently consumed/shipped or
  unavoidable toolchain boundary.
- The packaged crate hash and W16 evidence events are recorded in the
  workspace `docs/execution/evidence/`. W13 now owns fresh operational
  requalification of this exact candidate.

## W15 typed asynchronous STT local acceptance — 2026-07-30

- Status: `DONE`, `LOOPBACK-ONLY`; the central implementation and strict live
  Lab artifact are accepted at their named local evidence boundary.
- Public `pocketstation::Session` owns typed operator registration,
  `through(operator)`, terminal endpoints, 16 kHz signal propagation,
  graceful finish, explicit cancellation, compiled-input observations, and
  derived-route observations without a duplicate scheduler or counter owner.
- The example-owned Whisper operator runs real whisper.cpp CPU processes and
  records actual argv, PID, timestamps, logs, content hashes, transcript,
  timeout/cancellation, and killed/waited/reaped outcomes.
- The isolated external consumer proves typed partial/final lineage, bounded
  derived pressure, and a healthy raw browser/recording branch. The success raw
  branch delivered 751/751 frames with zero drops.
- Central graph, runtime, endpoint, Session, portable C, façade, Whisper,
  formatting, protocol, and strict Clippy gates pass. The embedded and
  independent Lab verifiers pass against `/private/tmp/pks-w15-live-final`.
- Capture uses a Lab speech fixture. This does not upgrade W15 to
  `REAL-DEVICE-PROVEN`, introduce provider code in core, or start W16/mobile.

## W12 focused recording and foreign-audio ownership — 2026-07-29

- Status: `SAFE-TO-TEST`; concrete multistem WAV implementation now lives in
  `pks-recording`, behind the unchanged `pks-endpoint` lifecycle. The generic
  endpoint contract, graph declarations, runtime delivery, and Session
  lifecycle remain in their existing owners.
- The bounded polled-audio endpoint moved from the grab-bag `pks-nodes`
  package into `pks-session`, which already owns its queue, batch, lease,
  observation, and cross-language projection semantics.
- `pks-session` no longer depends on `pks-nodes`. It registers the canonical
  grouped recorder through `register_multistem_recording`, retains a safe
  `SessionRecordingReceipt`, and exposes only terminal recording outcomes.
  Callers supply an artifact root; Session route context and capture-owned
  frame lineage remain authoritative.
- `pks-nodes` retains a deprecated recording re-export for the 0.1 source
  transition, but it and `pks-dsp` are now deferred non-registry packages.
  The supported `pocketstation` publication closure derives to 14 packages
  instead of 15 and contains `pks-recording`, not `pks-nodes` or `pks-dsp`.
- Focused acceptance passes: 36 transitional-node tests, 15 recording tests,
  59 Session tests, and three non-empty façade tests. The derived publication
  dry run validates all 14 closure packages in dependency order, and strict
  all-target Clippy passes for all four focused packages.
- No recording behavior, queue bound, lineage rule, hot path, provider,
  scaffold, mock, fallback, loopback classification, or product claim changed.
  Full workspace, quickstart, architecture, and CODE_PROTOCOL gates remain
  before the boundary candidate can pass.

## W12 stable public Session error codes — 2026-07-28

- Status: `SAFE-TO-TEST`; canonical `pks-session` now owns stable, namespaced,
  language-neutral codes for declaration, start, runtime, bounded audio-poll,
  stop status, and every retained stop-failure cause. The public Rust façade
  re-exports this contract and owns only mappings from its wrapper errors.
- The codes are additive to the published typed Rust errors. Existing error
  enums and method signatures remain unchanged, while Python and Node can
  normalize against values such as `session.start_cancelled`,
  `session.invalid_selector`, and `audio.lease_capacity_exhausted`.
- The string returned by `as_str()` is the compatibility contract; Rust enum
  variant names and discriminants are not. Exhaustive tables pin every current
  string and validate namespace syntax and uniqueness. Declaration variants,
  nested capture/start classes, audio-poll cases, and stop-cause projections
  have focused mapping coverage.
- AUDIO-032 records the measured package-boundary repair: concrete multistem
  recording moves from the broad `pks-nodes` package to `pks-recording`, while
  bounded foreign-audio projection moves to its canonical Session owner.
  Graph/caps/metrics and native-capture package convergence remain W15 work.
- All 53 `pks-session` tests and three non-empty `pocketstation` façade tests
  pass. Strict focused Clippy, CODE_PROTOCOL, and the recording extraction
  remain the next acceptance steps for the active boundary-repair candidate.
- This changes no engine, capture, recorder, queue, hot path, provider,
  scaffold, mock, fallback, or product claim.

## W12 Session-owned endpoint route and timeline context — 2026-07-28

- Status: `SAFE-TO-TEST`; this is the additive setup-context prerequisite for
  the Session-owned recording reference. Recorder composition and outcome
  projection remain separate follow-up work.
- `pks-endpoint` now defines typed `EndpointRouteContext` and
  `SessionTimelineOrigin` values. Canonical endpoint preparation can consume
  exact stem, route, and monotonic-origin identity without parsing reserved
  string configuration.
- The published `EndpointPrepareContext::new` signature remains unchanged.
  Its additive Session-route context is absent for legacy callers and attached
  explicitly by canonical `pks-session` startup.
- `pks-session` samples the shared monotonic clock exactly once after the
  initial cancellation gate and supplies that same origin to every endpoint
  input in the startup transaction. Each input also receives its compiler-owned
  stem and route identity.
- Focused evidence passes: eight `pks-endpoint` tests and 49 `pks-session`
  tests. The Session regression observes all six product routes, two distinct
  stems, unique route IDs, and one identical nonzero timeline origin.
- This changes no capture callback, realtime router, queue capacity, endpoint
  worker, recorder behavior, public provider surface, scaffold, mock,
  fallback, or loopback-only product claim.

## W12 language-owned Rust Session façade — 2026-07-28

- Status: `PARTIAL`; the central Rust façade and canonical-engine fixture pass,
  while the independent Lab clean-consumer artifact remains the W12 acceptance
  owner.
- Added one Cargo package/library named `pocketstation`. Its public `Session`
  and `RunningSession` are thin owners over the canonical `pks-session`
  declaration, native host, capture/runtime transaction, bounded polled-audio
  endpoint, events, metrics, cancellation, and idempotent stop. No scheduler,
  capture backend, counter, or lifecycle rule was duplicated.
- `Session::new()` is infallible and keeps engine setup internals out of the
  developer declaration. `start()` builds the native host with bounded defaults
  and reports host, compile, startup, missing-receipt, and missing-event states
  through typed errors. The public quickstart contains no `PrepareContext`,
  queue capacity, node ID, `CaptureBackendSet`, or CLI/subprocess delegation.
- The default-disabled `conformance-fixtures` feature supplies deterministic,
  distinct application and microphone frames to the same canonical host. It is
  explicitly `LOOPBACK-ONLY`, inventoried, and cannot upgrade a product claim.
  Three focused tests prove two independent stems cross bounded destinations,
  lifecycle/events/metrics are observable, stop is idempotent, cancellation is
  typed, and an invalid selector fails rather than reporting success.
- The unconsumed `pks-audio` compatibility package is retired. Its allocation
  gate moved to the codec owner, its executable graph example moved to the node
  owner, and its obsolete duplicate quickstart/local-proof/soak helpers were
  deleted rather than preserving a second public identity.
- Every internal path dependency in the public façade's Cargo closure now also
  carries its exact compatible version, and previously anonymous closure crates
  have truthful package descriptions. This is package readiness only; no crate
  was published and no registry-consumer claim is made before the dependency
  closure exists in the registry.
- All 19 workspace packages declare an explicit registry role. The release
  dry-run derives the exact 15-package normal/target dependency closure of the
  public façade, validates it in Cargo dependency order, and keeps
  `pocketstation` last. The codec C ABI, Session C ABI, and Whisper example
  remain explicitly non-publishable rather than leaking into that closure.
- Main/PR CI now runs workspace tests, strict all-target/all-feature Clippy,
  the release quickstart build, architecture and CODE_PROTOCOL checks, and the
  exact 15-package publish dry-run. The actual crates.io job is release-only:
  it rejects prereleases, non-`pocketstation-v<workspace-version>` tags,
  commits outside `main`, dirty checkouts, and any failed validation. It reads
  the scoped token from the job environment and does not run `cargo login`.
  GitHub's `crates-io` environment is the deployment boundary where repository
  owners can require approval and scope the token. The token is explicitly a
  first-release bootstrap: after `0.1.0`, each crate can authorize the
  repository through crates.io trusted-publishing OIDC and the long-lived
  secret must be retired. No crate was published by this work.
- No provider code, first-party connector catalog, browser/relay implementation,
  recording implementation, mock product path, per-frame foreign callback,
  unbounded queue, or new device claim was introduced. Connector/browser/
  recording parity remains outside this central façade slice until their
  reusable owners are extracted from callers.

## W11 transactional capture-delivery start boundary — 2026-07-28

- Status: `SAFE-TO-TEST`; the component correction is complete and the
  product-path lab rerun remains the acceptance gate.
- The W11 product proof exposed a real startup defect: capture backends could
  publish into their bounded streams while endpoint workers were still held
  behind the Session start gate. The runtime then admitted that backlog as one
  burst, overflowing otherwise healthy route edges.
- `pks-capture` now owns a one-way, atomic capture-delivery start gate.
  `pks-session` is its sole controller and opens endpoint workers first, then
  capture delivery, immediately before publishing `Running`. Frames produced
  before that transaction boundary are not admitted as product frames and are
  counted explicitly as
  `frames_discarded_before_start_total`.
- The callback-side check remains allocation-free, lock-free, blocking-free,
  async-free, log-free, and panic-free. No capacity changed; no sleep, retry,
  pacing, endpoint dependency, or consumer coordination entered production
  code.
- A deterministic regression emits sixteen frames per source during backend
  open and delays endpoint consumption. It proves those pre-`Running` frames
  are explicitly accounted, each source delivers its first post-start frame,
  every source-ingress rejection/discard counter stays zero, and every
  destination-edge drop counter stays zero.
- The C conformance fixture now delivers its observable audio after the
  Session start boundary. The pre-start counter is deliberately not appended
  to the ABI 1.1 source record: that output function has no caller-size
  negotiation, so growing its 176-byte record would be unsafe for an older
  compiled caller. The README names this projection gap instead of claiming
  complete captured-stream observations. The ABI 1.0 aggregate canary remains
  unchanged.
- Focused acceptance passes: 51 `pks-capture` tests, 49 `pks-session` tests,
  12 `pks-session-c` tests, two executable C harness tests, the standalone C
  conformance script, formatting, and strict all-target/all-feature Clippy for
  all three owners.
- No production scaffold, mock, fallback, loopback-only path, queue inflation,
  or duplicate lifecycle implementation was introduced.

## W11 codec C ABI ownership extraction — 2026-07-28

- Status: `PARTIAL`, `SAFE-TO-TEST`; the central Rust ownership correction is
  complete. Mobile consumption remains source-level compatibility only until
  the separate iOS and Android package/link migrations and native link tests
  pass in their owning repositories.
- Added sibling `pks-codec-c`, depending only on `pks-codec`, as the sole owner
  of the retained Opus C compatibility ABI. The dynamic/static library retains
  the existing `pks_encode_opus`, `pks_opus_encoder_create`,
  `pks_opus_encoder_destroy`, and `pks_opus_encoder_set_bitrate` symbols.
- Moved all ten ABI behavior tests to the new owner. `pks-audio` is now a Rust
  façade only: it no longer owns C exports, a build script, cbindgen, or
  `cdylib`/`staticlib` artifact types.
- The checked header now lives at `crates/pks-codec-c/include/pks_codec.h`.
  Builds generate only into Cargo `OUT_DIR` unless the caller explicitly sets
  `PKS_CODEC_C_HEADER_OUTPUT`. The repository no longer has an ambiguous root
  `ffi/` directory.
- Replaced `scripts/sync-ffi-header.sh` with
  `scripts/sync-codec-c-header.sh`. It generates outside the source tree and
  targets the canonical SDK paths
  `Sources/PocketStationCodecFFI/pks_codec.h` and
  `sdk/src/main/cpp/pks_codec.h`.
- `scripts/sync-codec-c-header.sh --check` is non-mutating and fails closed
  when the generated header differs from the central checked copy or either
  SDK compatibility copy. Unknown arguments fail instead of silently
  triggering synchronization.
- Focused evidence: all ten `pks-codec-c` tests pass; the checked header is
  byte-identical to a fresh explicit-output build; and the debug dynamic
  library exports exactly the four retained `pks_*` codec symbols.
- The generated header is C++ compatible. A separate C++17 executable now
  includes the checked header, links the actual `pks-codec-c` library, creates
  an encoder, encodes one 20 ms frame, and destroys it. This prevents JNI
  consumers from silently compiling against mangled C symbols.
- This introduces no Session ABI, runtime, provider, scaffold, mock, fallback,
  or loopback-only product path.

## W11 portable Session C lifecycle and conformance — 2026-07-28

- Status: `PARTIAL`, `SAFE-TO-TEST`; this checkpoint supersedes the current-
  state claims in the earlier W11 Session-ownership, host-foundation, and
  initial-C-boundary entries below. Those entries remain as historical
  evidence and must not be read as the current implementation inventory.
- `pks-session-c` now projects the real native `SessionEngineHost` through
  versioned records and opaque generational engine, Session, and audio-batch
  handles. The exported boundary covers the narrow application-plus-default-
  microphone declaration, compile, start, stop, state and event polling,
  bounded metric and audio polling, immutable frame access, explicit batch
  release, Session destruction, and engine destruction.
- Rust-record and checked-in C-header layout parity tests cover the complete
  public record surface. ABI entry points validate output pointers before
  acquiring engine, Session, or lease resources, and panic containment maps an
  unwind to a typed status without crossing the foreign boundary.
- The default executable C harness proves the real native-host failure path
  with a deliberately missing application source. It verifies compiled-to-
  failed lifecycle truth, foreign-handle rejection, stale engine rejection,
  bounded event and metric access, and recovery after the failed Session. It
  does not claim successful capture or audio delivery.
- The `conformance-fixtures` feature is test-only. With that feature,
  `scripts/test-session-c-conformance.sh` builds the adapter and compiles and
  runs a separate C executable against it. The executable proves successful
  application-plus-microphone execution through the canonical runtime, two
  distinct source and stem lineages, bounded lease-exhaustion observations,
  sample pointer and value stability while a lease is retained across Session
  stop, stale double-release rejection, and usable ABI recovery after an
  intentionally contained panic.
- The fixture exports are absent from the default library and public header.
  No synthetic capture symbol, test control, or panic trigger enters the
  production ABI.
- ABI version 1 intentionally permits exactly one Session for an engine's
  lifetime. The engine-scoped polled-audio receipt is consequently isolated by
  the ABI contract; concurrent or sequential Session reuse is not implied.
- A concurrent foreign stop no longer waits for the global engine table or the
  start-held runtime mutex before publishing its request. Each engine owns a
  shared `SessionStartCancellation` token plus atomic C lifecycle state. A
  blocking-open test observes `Starting`, requests stop from another thread,
  observes `Stopping` while the open remains blocked, then requires the start
  call to return `Cancelled`, the stop call to return success, and the terminal
  C state to become `Stopped` within the bounded post-release window. A
  compare-exchange owns the `Compiled` to `Starting` transition; a second
  concurrent start fails typed and cannot overwrite a live transition or
  running state.
- `pks-session` now owns `SessionMetricsSnapshot`; `SessionEngineHost`
  composes it from the authoritative bounded event queue, selected polled-audio
  receipt, and setup-time read-only receipts retained by `RunningSession`.
  Indexed source records expose stable stem identity, capture-owner,
  captured-stream, runtime-event, and source-ingress observations. Indexed
  route records expose stable route and endpoint identities, every
  authoritative runtime-edge observation, endpoint observations, an explicit
  unavailable/live/finalized stage, and endpoint-finalization failure count.
  A defensive endpoint lookup miss is typed `Unavailable`; it cannot
  masquerade as synthetic live zero observations.
- C ABI 1.1 adds source/route count functions and count-indexed source and
  route records while preserving the ABI 1.0 aggregate metrics record at
  exactly 160 bytes. A compiled ABI 1.0 C canary requests minor version 0,
  places a guard immediately after that record, polls metrics, and proves the
  guard is unchanged. Invalid indexes fail with `IndexOutOfRange`; final route
  observations remain readable after stop; destroyed Session handles fail
  indexed access with `StaleHandle`. The integer stage field is safe for
  normal C zero initialization and has named `UNAVAILABLE`, `LIVE`, and
  `FINALIZED` values.
- Focused acceptance passes: 48 `pks-session` tests, 12 `pks-session-c` tests,
  the default executable C harness, the feature-gated successful C conformance
  executable, and strict all-target Clippy for both crates.
- W11 is not accepted by this component checkpoint. ABI v1 metrics deliberately
  retain lower-layer counter ownership: the Session snapshot holds only
  read-only capture, ingress, edge, and endpoint receipts. The frozen W11
  acceptance matrix and evidence hashes remain open; this checkpoint does not
  claim language-package consumption or a real-device product proof.

## W11 bounded application-polled audio endpoint — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-nodes` now owns a concrete external endpoint
  worker that consumes the canonical compiled `PlanEdgeReceiver` path and
  publishes immutable audio through fixed-capacity queues and preallocated
  batch-lease ownership. `pks-session` exposes only the safe composition and
  receipt projection needed by later language adapters.
- The worker accepts only `LineagedAudioFrame` values delivered as
  `PlanEdgeFrame::LineagedExclusive`. Raw exclusive, shared-reference, and
  shared-lineaged variants fail closed and increment explicit ownership-drop
  and endpoint-failure observations. This makes branch-copy plus lineage a
  stored type invariant rather than a getter assertion.
- One factory supports independent application and microphone endpoints.
  Every leased frame retains Session, source, stem, clock, sequence,
  timestamp, permission, endpoint, connector, and route identity. Samples
  remain pool-owned and stable until the bounded lease is dropped.
- Endpoint workers perform only bounded receiver pops, SPSC pushes, and atomic
  observations per frame. Queue saturation drops the newest branch copy and
  counts it. Queue depth is reserved before publication, checked on dequeue,
  bounded under concurrent publish/poll, and reports any impossible underflow
  without wrapping. Untrusted queue, batch, and lease capacities are capped
  before allocation. Foreign polling and lease recycling may lock only on the
  control thread; capture callbacks and realtime routing never call foreign
  code, allocate, lock, block, or log.
- A deterministic capture test proves the real
  `CaptureDelivery → RunningSession → compiled runtime → endpoint worker →
  receipt` path for the required application-plus-microphone topology. Focused
  tests also prove all invalid ownership variants are counted, a held lease
  preserves sample address and data through Session stop, and exhausted lease
  capacity returns a typed result and observation.
- All 52 `pks-nodes` tests and 44 `pks-session` tests pass, as does strict
  all-target Clippy for both packages. No adapter-local injection queue,
  provider implementation, production mock, fallback, hidden scaffold, or
  loopback-only product claim remains in this step. The versioned C projection
  and non-Rust conformance harness remain the next gated W11 task.

## W11 source-failure branch isolation — 2026-07-26

- Status: `SAFE-TO-TEST`; the canonical Session runtime now stops and
  finalizes only the capture owner that emits a typed source failure. Other
  source stems continue through their independent bounded routes until the
  Session owner requests stop.
- The failed source remains represented in terminal source failures and makes
  the final Session outcome failed. Branch isolation does not hide or
  downgrade the fault.
- A focused two-source/six-route test injects an authoritative application
  disappearance before runtime polling and proves the microphone frame still
  reaches all three destinations, exactly one source failure is emitted, all
  owners finalize, and the Session outcome remains failed.
- No retry, selector fallback, source replacement, unbounded queue, mock
  product path, or loopback-only behavior was introduced.

## W11 declared connector identity handoff — 2026-07-26

- Status: `SAFE-TO-TEST`; the `EndpointHandle` returned by
  `Session::connector` now exposes its Session-allocated `ConnectorId`.
  Concrete open connector factories can configure exact endpoint/route
  receipts without guessing allocation order or freezing a second Session
  copy.
- Non-connector endpoints return no connector identity. The ID remains
  Session-owned and is still serialized in the canonical `SessionSpec`.
- All 41 `pks-session` tests and strict all-target Clippy pass, including a
  focused allocation test.
- No compatibility alias, provider enum, global registry, scaffold, fallback,
  or loopback-only path was introduced.

## W11 multistem endpoint completion receipt — 2026-07-26

- Status: `SAFE-TO-TEST`; `MultistemEndpointCoordinator` now exposes a
  cloneable, read-only completion receipt owned by the recording endpoint
  boundary. The receipt publishes the exact finalized `RecordingOutcome`,
  including each stem's written-frame count, discontinuities, error, and
  authoritative edge observations.
- Finalization installs the outcome once. A duplicate installation fails the
  endpoint finalization explicitly instead of replacing evidence or returning
  a false success.
- The receipt does not start, stop, poll, or configure recording and adds no
  process-global registry. `MultistemRecording` remains the sole recording
  lifecycle owner.
- Five focused multistem endpoint tests pass, including a two-stem gated run
  that proves the receipt remains readable after endpoint finalization. Strict
  all-target Clippy for `pks-nodes` passes.
- No scaffold, fallback, unbounded storage, connector/provider behavior, or
  loopback-only product path was introduced.

## W11 per-stem Session media preparation — 2026-07-26

- Status: `SAFE-TO-TEST`; the canonical Session structural graph now declares
  the product formats explicitly: 48 kHz stereo for application capture and
  48 kHz mono for microphone capture. Negotiated edge media therefore sizes
  each bounded fan-out branch pool for its real channel count instead of
  treating an `Any` layout as mono.
- Runtime preparation now derives each endpoint input's `PrepareContext` from
  that route's negotiated graph edge. Connector, relay/browser, and grouped
  recorder drivers receive the application and microphone formats separately;
  one global Session sample format no longer misrepresents both stems.
- The obsolete endpoint-preparation context argument was removed from
  `start_prepared_session`. The engine's setup context remains solely with
  realtime-node preparation, while route-specific endpoint contexts travel
  with the owned `PreparedWorkerMapping`.
- A focused canonical-engine test proves all three application destinations
  prepare as stereo and all three microphone destinations prepare as mono.
  All 40 `pks-session` tests and strict all-target Clippy pass.
- No converter, fallback, unbounded queue, provider behavior, capture
  implementation, or loopback-only path was introduced.

## W11 pks-audio canonical Session facade — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-audio` now re-exports the authoritative
  `pks-session` surface instead of owning a second Session declaration,
  lifecycle state machine, stop handle, and `RuntimeNotIntegrated` result.
- A complete repository search found no internal consumer of the removed
  `ConnectorKey`, `ConnectorHandle`, `SessionState`, or `StopHandle` surface,
  so no speculative compatibility aliases remain. The removed implementation
  and its self-tests were the only consumers.
- The `product_quickstart` compile target now declares the application and
  microphone stems with canonical fallible handles, routes them to open
  connector, browser, and grouped recording boundaries, and calls
  `SessionEngine::start` with host-owned capture backends. It does not construct
  a no-op endpoint, select a provider, or move transport policy into the
  facade.
- A focused facade test proves `pks_audio::Session` and
  `pks_audio::SessionEngine` are the exact canonical types. All ten
  `pks-audio` unit tests, its allocation test, pipeline integration test,
  facade test, all 40 `pks-session` tests, strict all-target Clippy,
  architecture constraints, the full CODE_PROTOCOL gate, and the release
  quickstart build pass.
- No scaffold, mock product path, fallback, helper process, provider
  implementation, or loopback-only behavior was introduced.

## W11 canonical Session engine bootstrap — 2026-07-26

- Status: `SAFE-TO-TEST`; `SessionEngineBuilder` now installs the fixed
  structural node set once, validates open operator-to-node registrations, and
  consumes all setup state before constructing the paired operator and
  endpoint-driver registries. A failed registration or build cannot expose a
  partially usable engine.
- `SessionEngine::start` is the one reusable setup-time composition path from
  a public `Session` declaration to the existing freeze, graph compile,
  bounded runtime preparation, and transactional start owners. Freeze,
  compile, prepare, and start failures remain separate typed variants;
  `SessionStartFailure` remains available with its rollback failures and event
  receiver instead of being converted to text.
- Concrete callback capture backends and endpoint-driver factories remain
  injected through the existing `pks-capture` and `pks-endpoint` contracts.
  No platform capture, relay, recorder, connector, provider, artifact, or
  proof-policy implementation moved into `pks-session`.
- Four focused engine tests prove the complete application plus microphone
  declaration with connector, browser, and grouped recording boundaries
  reaches `Running`, all five prepared endpoint instances start behind the
  closed Session gate, repeated stop is idempotent, duplicate and conflicting
  registrations fail typed, unknown operators remain compile failures, and a
  capture-open failure preserves the transactional start error.
- All 40 `pks-session` tests, strict all-target Clippy, workspace formatting,
  architecture constraints, the full CODE_PROTOCOL gate, and the release
  `product_quickstart` example build pass. The initial architecture acceptance
  command in the execution envelope named a nonexistent legacy path; execution
  stopped, the envelope was corrected to the CI-authoritative
  `scripts/lint/check-architecture-constraints.sh`, and the corrected gate
  passed.
- No scaffold, mock product path, fallback, helper process, provider
  implementation, or loopback-only behavior was introduced. Test-only capture
  and endpoint contract doubles remain under `cfg(test)`.

## W11 local CI correction and candidate gate — 2026-07-26

- Status: `SAFE-TO-TEST`; GitHub PR #43 remains unmerged and must not be
  described as `SAFE-TO-MERGE` until its exact pushed head has every required
  check green.
- Rust 1.97 strict all-target Clippy exposed manual no-op `Wake`
  implementations in two test poll helpers. Both now use the standard
  `Waker::noop()` without changing runtime scheduling, capture, endpoint,
  allocation, or hot-path behavior.
- The exact local Linux candidate passes workspace formatting, strict
  all-target Clippy, all workspace tests and doc tests, allocation tests,
  architecture constraints, quickstart compilation, CODE_PROTOCOL, and the
  complete `pks-audio` benchmark build. The benchmark link was repeated with
  one Cargo job and a 6 GiB ceiling after the initial 4 GiB container was
  killed by its memory limit; the source, locked dependencies, release
  profile, Rust 1.97 toolchain, and benchmark executables were unchanged.
- The native macOS candidate independently passes workspace formatting, strict
  all-target Clippy, all workspace tests and doc tests, allocation tests,
  architecture constraints, quickstart compilation, CODE_PROTOCOL, and the
  complete `pks-audio` benchmark build.
- No CI check was disabled, made advisory, skipped, or wrapped in a success
  override. No scaffold, mock, fallback, provider implementation, or
  loopback-only product path was introduced.

## Linux runtime-event CI compilation — 2026-07-26

- Status: `SAFE-TO-TEST`; the Linux capture module now imports the shared
  `Platform`, `SourceKind`, and `StableSourceId` types used by its typed
  runtime-failure events.
- GitHub's Linux all-target Clippy gate found the missing imports after the
  macOS host gates passed. A current Linux Clippy run also removed an
  immediately dereferenced name borrow in exact application matching. These
  corrections change no capture behavior, queue, callback work, fallback,
  selector, or product claim.

## W11 reusable Session structural node registration — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-session::register_session_structural_nodes`
  installs the fixed application/microphone ingress and
  connector/browser/recording boundary descriptors required by the canonical
  Session compiler and runtime. CLI and SDK adapters no longer need to
  recreate compile-only placeholder factories.
- Application and microphone structural nodes are real allocation-free
  realtime ingress forwarders. Their configuration validation requires exact
  Session/stem identity plus a selector form valid for that source kind,
  including Windows process-instance PID and stable identity fields.
- External destination descriptors remain `AsyncWorker` boundaries. Their
  factories validate route and endpoint metadata, but any accidental
  `RuntimeNode` instantiation returns the dedicated typed
  `ExternalBoundaryExecution` error. Connector, relay/browser, and recording
  work remains in `EndpointDriverFactory` implementations; no no-op endpoint
  can report success.
- Registration preflights all five stable node type IDs before mutation and
  returns a typed duplicate error without partially changing the registry.
  Compiler tests now consume the production registration seam instead of local
  compile-only factories.
- All 36 `pks-session` tests pass. Strict all-target Clippy for `pks-session`
  and `pks-graph` and workspace formatting pass.
- No provider implementation, relay algorithm, product policy, scaffold,
  mock, fallback, helper process, or loopback-only path was introduced.

## W11 endpoint-driver lifecycle contract — 2026-07-26

- Status: `SAFE-TO-TEST`; the new acyclic `pks-endpoint` crate owns the open
  endpoint-driver registry and setup-time lifecycle contract shared by
  `pks-session` orchestration and concrete destination packages.
- `EndpointDriverRegistry` resolves only an exact open `OperatorId` plus
  `NodeTypeId` pair and transfers the route's existing bounded
  `PlanEdgeReceiver` into the selected factory. Unknown, empty, and duplicate
  registrations and driver preparation failures remain typed.
- Prepared endpoints may start only while the shared gate is closed. Starting
  makes the driver ready but does not authorize delivery; only the
  Session-owned `EndpointStartGateController` can open the one-way gate after
  every startup resource is ready. An already-open gate fails start and returns
  the prepared endpoint for rollback.
- Preparation cancellation, idempotent stop request, and join/finalize return
  authoritative endpoint observations and preserve stop and finalization
  failures independently. The contract creates no worker thread and contains
  no concrete connector, relay, recorder, provider, or production no-op
  implementation.
- `OperatorId` moved to this lower contract crate and remains re-exported from
  `pks-session`. Its version-one serialized form is explicitly one transparent
  UTF-8 string; `SessionSpec` retains document migration authority.
- Six contract tests prove exact registry resolution, multi-endpoint prepare
  rollback, closed-gate readiness with no pre-open delivery, fail-closed
  already-open start, and truthful stop/join failure reporting. All 16
  `pks-session` tests, strict Clippy for both packages, focused formatting, the
  architecture dependency lint, and full CODE_PROTOCOL pass.
- No scaffold, mock product path, fallback, provider implementation, helper
  process, or loopback-only product behavior was introduced. The only driver
  implementation is a `cfg(test)` contract double.

## W11 grouped multistem recording endpoint — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-nodes::MultistemEndpointCoordinator` is one
  Session-scoped concrete endpoint driver over the accepted
  `MultistemRecording`. It does not duplicate WAV, timeline, discontinuity,
  manifest, metric, checksum, or finalization algorithms.
- AUDIO-030 adds explicit batch preparation to `pks-endpoint`. Grouping requires
  one Session and an exact `OperatorId`, `NodeTypeId`, `EndpointGroupId`, and
  declared endpoint set. `StemHandle::record` now persists the stable default
  `recording_group_id` `session.multistem.default.v1`; sharing only an operator
  or node type never groups endpoints.
- Preparation validates the complete application/microphone batch, exact
  endpoint IDs, Session, group, stem labels, and sample specifications before
  creating an artifact or worker. Cancellation drops the pending receivers and
  leaves no Session directory.
- One prepared group starts one `MultistemRecording` while the shared gate is
  closed. Setup uses an unpublished pending directory; workers consume zero
  queued frames and no final Session directory or manifest exists before the
  Session opens the gate. Opening publishes the staged directory atomically.
  One `RunningEndpoint` owns both stems and therefore requests stop, joins
  workers, writes one final manifest, and reports one typed outcome exactly
  once. Pre-open rollback removes staging and reports cleanup failure rather
  than hiding it; all workers are joined and typed worker plus cleanup failures
  are preserved together when both occur.
- Recorder observations now expose frames received/written/rejected,
  discontinuities, and failures while running; final endpoint observations also
  preserve each edge's authoritative delivery/drop counters. Incomplete worker
  finalization remains a failed endpoint outcome with an incomplete manifest.
- Five grouped-driver tests prove two stems in one directory/manifest,
  pre-gate zero consumption and zero published artifacts, partial-batch and
  ready-group rollback without artifacts, finalization failure truth, and
  failed-branch isolation. A recorder-level test separately proves aggregate
  pre-gate worker and cleanup failure truth. All 47 `pks-nodes` tests and all
  six `pks-endpoint` tests pass; strict `pks-nodes` Clippy passes.
- No process-global registry, concrete relay/provider logic, production fake,
  fallback, helper process, or loopback-only product behavior was introduced.

## W11 Session compiler and RuntimePlan ownership — 2026-07-26

- Status: `PARTIAL`; immutable Session declarations now lower through the real
  `pks-graph` compiler and runtime planner, while runtime start, capture and
  endpoint ownership, transactional rollback, stopping, and finalization
  remain open.
- `SessionCompiler` consumes a validated `SessionSpec`, the existing
  `NodeRegistry`, and an open `OperatorRegistry` mapping `OperatorId` to
  `NodeTypeId`. Unknown operators, missing source node types, mismatched
  operator/node registrations, reserved configuration keys, graph compile
  errors, and planner errors remain typed.
- Each captured stem lowers to one source node. Each route lowers to its own
  endpoint node and edge, so two stems sharing connector/browser declarations
  still receive independent edge queues and memory plans.
- `CompiledSession` privately owns the immutable specification, verified
  `GraphIr`, and `RuntimePlan`. Its public surface exposes declarations and
  summary counts, not graph IR, runtime plans, factories, pools, or executor
  internals.
- The focused product topology compiles two sources plus six route endpoints
  into eight nodes and six independent graph/planned edges. Test-only
  descriptors are registry-backed and their compile-only factories return a
  typed error if execution is attempted; no no-op endpoint success exists.
- Eleven focused tests, package strict Clippy, and format pass. This step adds no
  `start`, `Running`, runtime-success, endpoint worker, capture backend,
  provider implementation, mock product path, fallback, or loopback-only
  claim.

## W11 callback capture ownership contract — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-capture` now owns a platform-neutral
  prepare/open/stop-and-join contract for callback-oriented capture.
- A `CaptureOwner` retains the native backend, bounded captured-frame stream,
  typed runtime-event channel, and their authoritative observations. Prepared
  and active owners are distinct, and a prepared backend can open only once.
- Dropping the owner reclaims the backend through its RAII contract; explicit
  `stop_and_join` joins every native worker before returning final observations
  and maps a worker panic to typed `CaptureWorkerPanicked`. Drop performs the
  same reclamation best-effort without propagating failure. The existing
  pull-oriented `PlatformAdapter` remains a documented legacy compatibility
  path.
- Fifty package tests, strict clippy, focused format, and the full
  CODE_PROTOCOL gate pass.
- Thin target adapters now move the platform-neutral bounded delivery
  endpoints into the existing macOS, Windows, and Linux
  `DesktopCaptureSource` owners. macOS native check, tests, and strict Clippy
  pass. Windows ARM64 MSVC check and strict Clippy pass as cross-compilation
  evidence only; its existing typed runtime events now publish directly into
  the supplied Session channel with no forwarding thread or duplicate queue.
  macOS and Linux likewise move the supplied sender into their native
  callback/worker owner: CPAL and PipeWire callbacks publish a prebuilt,
  allocation-free one-shot failure event, and reader failure ownership closes
  when the native producer exits.
  Linux source integration is implemented, but macOS cross-compilation stops
  in `alsa-sys` before the crate builds because no ARM64 Linux ALSA/PipeWire
  pkg-config sysroot is installed. Linux therefore still requires a native VM
  check, and the target adapters are not yet real Session-path evidence.
- No live scaffold, mock, fallback, helper process, provider implementation, or
  loopback-only product path was introduced. Contract doubles are test-only.

## W11 immutable Session declaration foundation — 2026-07-26

- Status: `PARTIAL`; `pks-session` now owns a safe Rust declaration/freeze
  foundation, while runtime compilation, startup, endpoint ownership, stopping,
  the C conformance surface, and real-path migration remain open W11 work.
- `Session` builds one versioned `SessionSpec` from application/microphone
  selectors, open `OperatorId` plus `NodeTypeId` endpoint descriptors,
  configuration values, stems, endpoints, and routes. No closed
  provider/model/policy enum entered the crate.
- Freezing consumes the public Session builder and closes the shared draft
  before validation. Cloned stem handles cannot mutate a frozen draft; foreign
  endpoint handles fail immediately and create no route.
- The specification exposes immutable slices and typed identifiers. It exports
  no graph IR, runtime plan, pool, Tokio type, Rust trait object, provider
  client, platform object, or raw foreign handle.
- Six focused tests pass for distinct routes, foreign endpoints, cloned/stale
  post-freeze mutation, invalid open operator identity, and fail-closed
  unrouted stems. Focused strict Clippy and format pass.
- This step adds no `run`, `start`, `Running`, stop, runtime-success,
  `RuntimeNotIntegrated`, C ABI, mock, scaffold, fallback, or loopback-only
  path.

## W11 Session runtime preparation — 2026-07-26

- Status: `SAFE-TO-TEST`; consuming a `CompiledSession` now instantiates the
  real `RealtimePlanExecutor`, creates one bounded `PlanSource` channel for
  each declared stem, and retains each non-realtime edge receiver with its
  exact route, stem, and endpoint identity.
- Preparation validates that the compiled plan produces exactly one worker
  receiver per route, rejects missing, invalid, unknown, duplicate, or
  mismatched route metadata with typed errors, and rolls all instantiated
  nodes and bounded channels back through ownership drop on any failure.
- The public prepared surface exposes Session, stem, route, and endpoint
  identities plus counts and observations. Graph node IDs, the executor,
  source consumers, worker receivers, and cancellation ownership remain
  internal for the later `RunningSession` transition.
- Sixteen focused `pks-session` tests pass. Four preparation tests use explicit
  test factories to prove two independently bounded source channels, six
  independently mapped worker receivers, and zero live nodes after both
  receiver-mapping and source-channel failures. Focused strict Clippy and the
  full workspace CODE_PROTOCOL gate pass.
- This step opens no capture backend, starts no endpoint worker or runtime
  thread, publishes no `Running` state, and adds no production no-op node,
  scaffold, mock, fallback, provider implementation, or loopback-only path.

## W11 lineage-authoritative runtime routing — 2026-07-26

- Status: `SAFE-TO-TEST`; canonical `pks-runtime` source ingress, realtime
  execution, bounded edge fan-out, and worker delivery now retain the exact
  capture-time `FrameLineage`.
- `PlanSourceSender` accepts only `LineagedAudioFrame`. Full or cancelled sends
  return a non-allocating outcome containing a small typed reason and the
  rejected exclusive frame, preserving explicit drop-newest ownership without
  boxing on the hot path.
- `RealtimePlanExecutor` temporarily separates samples from lineage only while
  a registered `RuntimeNode` processes the exclusive `AudioFrame`. It validates
  the returned frame identity and reattaches the same immutable lineage;
  source, sequence, or timestamp mutation fails typed execution instead of
  reconstructing source generation, discontinuity, or permission epochs.
- `PlanEdgeRouter` preserves lineage through both shared-reference fan-out and
  branch-pool copies. Receivers expose the retained lineage and count an
  authoritative discontinuity-epoch transition beside existing sequence and
  timestamp discontinuities.
- Raw `dispatch_from` and `execute_from` remain documented compatibility entry
  points for existing callers only. The canonical runner cannot enter that
  path, and lineaged/raw edge variants fail closed if mixed during execution.
- Focused tests prove exact lineage across three independent copied branches,
  realtime operator processing, discontinuity-epoch observation, independent
  multi-source dispatch, bounded cancellation, and zero-allocation lineaged
  router, executor, and runner processing.
- No frame allocates, no lineage epoch is inferred on the canonical path, and
  no Session lifecycle authority, provider implementation, scaffold, mock,
  fallback, or loopback-only product behavior was introduced.

## W11 bounded multi-source runtime runner — 2026-07-26

- Status: `SAFE-TO-TEST`; this is a reusable runtime component, not a complete
  Session or a real-path W11 acceptance artifact.
- `RealtimePlanRunner` owns one prepared `RealtimePlanExecutor` and drains
  independent preallocated source-input rings with bounded round-robin work.
  It spawns no thread and publishes no Session lifecycle state.
- `PlanRunnerCancellation` prevents new source delivery after cancellation.
  Finalization uses an explicit `DrainQueued` or `DiscardQueued` policy, a
  caller-supplied frame budget, and counted discard observations. The owning
  Session must stop capture producers before final drain so no producer can
  race terminal resource reclamation.
- Each source input exposes capacity, current/peak depth, enqueue/delivery,
  full/cancelled rejection, and shutdown-discard counts. A full source input
  drops only its newest frame.
- Focused tests prove two sources dispatch independently, cancellation drains
  no more than its declared frame budget, remaining frames discard explicitly,
  discard-only finalization executes no frame, and prepared runner processing
  performs zero heap allocation.
- Acceptance passes: scoped format, locked package check, all 43 runtime unit
  tests plus three allocation tests, strict all-target `pks-runtime` Clippy,
  and the full CODE_PROTOCOL gate.
- No Session type, lifecycle authority, worker thread, capture implementation,
  endpoint policy, provider path, scaffold, mock, fallback, or loopback-only
  product behavior was introduced.

## W11 immutable frame-lineage envelope — 2026-07-26

- Status: `PARTIAL`; `pks-frame` now provides exclusive and shared audio
  envelopes that retain the frozen `FrameLineage` snapshot through bounded
  fan-out and `CopyToBranchPool`.
- Construction rejects source, sequence, or timestamp mismatches without
  allocation. Dynamic source-generation, discontinuity, and permission epochs
  therefore remain attached to the samples they describe.
- The focused `pks-frame` tests and clippy gate pass. Runtime edge integration
  is still required before this becomes real Session-path evidence.
- No scaffold, mock, provider implementation, fallback, helper process, or
  loopback-only path was introduced.

## W11 Session ownership correction — 2026-07-26

- Status: `PARTIAL`; the safe engine and portable adapter ownership contracts
  are accepted, while no `pks-session` implementation, C artifact, or
  conformance result exists yet.
- `pks-session` is the thin safe-Rust composition and lifecycle owner. It
  freezes declarations, coordinates real graph compilation and transactional
  startup, owns all running resources, and coordinates bounded stop, join, and
  recording finalization.
- `pks-graph` retains graph compilation and `RuntimePlan`; `pks-runtime`
  retains scheduling, execution, routing, and runtime observations;
  `pks-frame` retains pools and frames; native capture and endpoint
  implementations remain in their existing smallest owners.
- The portable Session C projection remains a separate pending adapter task. It
  does not exist as a checked-in crate today; when introduced it owns ABI
  records, generational handles, marshalling, polling, leases, panic
  containment, reproducible headers, and C conformance. It does not own
  Session semantics.
- Rust uses the future `pocketstation` façade directly over `pks-session`.
  Python and Node may use direct PyO3 and Node-API adapters. Swift and Kotlin
  may use language-owned adapters over the C/control boundary and setup-time
  bounded buffers. Browser JavaScript remains relay-native.
- The reusable lifecycle currently assembled by the CLI proof command is the
  extraction seam. Proof policy, artifact rendering, connector provider code,
  relay mechanics, and product thresholds remain outside the engine.
- The unaccepted standalone `pks-runtime::RuntimeHandle` overlay was rejected:
  it could fabricate lifecycle without owning capture, endpoint workers,
  rollback, or recorder finalization.
- This correction adds no scaffold, mock, provider implementation, fallback,
  helper process, or loopback-only path.

## W11 Session host foundation — 2026-07-27

- Status: `SAFE-TO-TEST`; `pks-session` now exposes `SessionEngineHost` and
  `SessionEngineHostBuilder` as the safe owner for the canonical Session
  engine, the exact application and microphone capture backends, and any
  retained bounded polled-audio receipts used for foreign projection.
- The host builder reuses the real `SessionEngineBuilder` registration seam,
  rejects missing application or microphone backends before a host exists, and
  registers polled-audio endpoints only through the real bounded
  `PolledAudioEndpoint` factory. No side runtime, synthetic queue, or adapter-
  local media path was introduced.
- The host starts only through the canonical engine and therefore preserves the
  real compile, prepare, start, rollback, stop, and bounded lease path already
  proven by `pks-session`.
- Four focused host tests now prove typed missing-backend rejection, retained
  receipt ownership, host-owned polled-audio delivery through the real runtime,
  and typed capture-start failure propagation.
- All 48 `pks-session` tests, strict `cargo clippy -p pks-session --all-targets
  --locked -- -D warnings`, and the full workspace `bash scripts/check_protocol.sh`
  pass on 2026-07-27.
- `pks-session-c` still does not exist. Versioned ABI records, generational
  foreign handles, header packaging, panic containment, and the real C
  conformance harness remain the open W11 portable-binding work.

## W11 initial pks-session-c boundary slice — 2026-07-27

- Status: `SAFE-TO-TEST`; the sibling `pks-session-c` crate now exists and owns
  the first real portable C boundary slice above `pks-session`.
- The crate ships checked-in ABI version/status records, opaque generational
  handle records, an internal handle table, panic-contained exported entry
  points for ABI version query plus runtime open/close/live checks, and the
  packaged public header `include/pks_session.h`.
- Header packaging is non-hermetic: `build.rs` copies the checked-in header into
  `OUT_DIR` only. It does not write back into source.
- The first conformance fixture is compile-level only: a C translation unit
  includes the public header, checks stable layout expectations, and compiles
  against the exported symbol declarations. It does not yet execute the full
  Session lifecycle from C.
- Focused gates pass on 2026-07-27:
  `cargo test -p pks-session-c --offline` and
  `cargo clippy -p pks-session-c --all-targets --offline -- -D warnings`.
- This is still `PARTIAL` W11 portable binding work. Real Session declaration,
  lifecycle, bounded event/metric/audio polling, generational engine/session
  handle ownership, and the executable C conformance harness remain open.

## W11 embedded Session engine boundary decision — 2026-07-25

- Status: `PARTIAL`; W10 PASS opened W11 and the engine deployment/ownership
  decision is accepted, while no W11 engine code, C header, language binding,
  or bindability artifact exists yet.
- AUDIO-029 selects one embedded native Session engine. The embedding
  application remains the visible permission identity; desktop packages may
  load a dynamic library and future mobile adapters may statically link the
  same engine. A signed local helper and process IPC remain evidence-triggered
  alternatives, not parallel implementations.
- The binding contract is a versioned C ABI over opaque generational handles.
  It exposes Session declaration, compile/start/stop, bounded event and metric
  polling, and bounded multi-frame audio leases. It does not expose Rust graph
  IR, runtime/pool layouts, traits, Tokio types, platform objects, provider
  enums, or per-frame foreign-language callbacks.
- Foreign audio retention receives a preallocated `CopyToBranchPool` copy under
  AUDIO-027. Batch leases are immutable, explicitly released, and bounded at
  compilation; queue or lease exhaustion drops the newest delivery and records
  the exact reason.
- `CRATE_OWNERSHIP.md` assigns the authoritative Session specification,
  lifecycle, error/event/metric projections, leases, and C ABI to the precise
  `pks-session` crate. `pks-runtime`, `pks-frame`, capture adapters, and endpoint
  implementations retain their existing lower-layer ownership.
- The existing `pks-audio` façade now re-exports the canonical `pks-session`
  surface. The remaining W11 gap is the portable C adapter and its conformance
  harness, not a second Rust Session runtime.
- This documentation slice adds no code, generated header, mock, scaffold,
  fallback, provider path, helper process, or loopback-only behavior. W11 exit
  still requires a non-Rust lifecycle/lease harness, typed panic containment,
  the quickstart compile gate, and full protocol acceptance.

## macOS native ring-loss telemetry — 2026-07-23

- Status: `SAFE-TO-TEST`; the 11 macOS component tests pass, while no
  post-change real-device artifact yet proves the new counters on an active
  CoreAudio process tap, microphone, or AudioServerPlugin fallback.
- CoreAudio process-tap and AudioServerPlugin readers now convert native ring
  overwrite deltas into `dispatch_queue_full_total`. The microphone callback
  reports bounded Rust-ring rejection through the same canonical counter.
- Process-tap, AudioServerPlugin, and microphone capture expose canonical
  `CaptureObservations` for callback buffers, enqueued frames, pool exhaustion,
  bounded-ring loss, oversize rejection, and stream errors where each backend
  can observe them. `DesktopCaptureSource::observations()` no longer fabricates
  or omits the process-tap boundary.
- Existing macOS real-device artifacts predate this telemetry and therefore do
  not prove zero native-ring loss for the new boundary. A new physical-device
  capture artifact is required before upgrading this section.
- No queue was made unbounded, and no mock, fallback, provider path, or new
  loopback-only behavior was introduced.

## Windows active-source lifecycle events — 2026-07-23

- Status: `SAFE-TO-TEST`; the shared contract and Windows ARM64 compilation
  gates pass, while native active-process-exit and endpoint-invalidation
  artifacts remain open.
- `pks-capture` now owns a typed `SourceRuntimeEvent` control stream. It carries
  the exact stable identity and generation, distinguishes authoritative
  `SourceUnavailable` from `BackendFailure`, and requires explicit rediscovery
  plus a new Session after disappearance.
- The control stream is bounded to eight events in the Windows backend.
  Publication uses nonblocking `try_send`; a full channel drops the newest
  event and exposes exact enqueued/dropped totals. Event creation and
  publication happen only on the capture worker after a terminal condition,
  never in the audio delivery callback.
- Active Windows process-loopback retains a synchronization handle for the
  process incarnation verified at open. A signaled handle emits typed
  `SourceUnavailable` with `SourceInstanceExited`; PID reuse cannot silently
  retarget the active Session.
- WASAPI query, read, and event-wait failures retain their exact Windows
  HRESULT when present. Only `AUDCLNT_E_DEVICE_INVALIDATED` is classified as
  source disappearance. Resource invalidation, process-watch errors, oversize
  packets, and all other WASAPI classes remain typed backend failures rather
  than being guessed as disappearance.
- Acceptance: 44 shared capture tests and 15 Windows host-neutral tests pass;
  `cargo check -p pks-capture-windows --target aarch64-pc-windows-msvc` and
  target strict Clippy pass.
- `pks proof sources` and `pks capture from` now consume the typed Windows
  events and retain exact failure plus event-channel observations. This CLI
  integration has host tests but no native Windows active-invalidation
  artifact.
- Residual boundaries: process-tree capture observes termination of the
  selected root process, not each descendant; the bounded event stream may
  report counted overflow; and native Windows active-invalidation evidence is
  still required.
- No scaffold, mock, automatic restart, replacement following, fallback,
  provider path, or new loopback-only behavior was introduced.

## Windows process-instance identity hardening — 2026-07-23

- Status: `SAFE-TO-TEST`; deterministic identity tests and Windows ARM64
  cross-checks pass, while a native PID-reuse execution artifact remains open.
- Discovered WASAPI application sources now encode the native process creation
  `FILETIME` with the PID in `StableSourceId`. Discovery verifies the creation
  time again after resolving display metadata and omits a session that changed
  or disappeared during enumeration.
- `ExactApplication` parses that fingerprint and verifies PID plus creation time
  immediately before and after `ActivateAudioInterfaceAsync`. A missing,
  replaced, recycled, or malformed exact identity fails closed; no system-mix
  or replacement-process fallback is permitted.
- Direct `Process(pid)` remains a raw process-lifetime selector. It checks that
  the PID is queryable and pins the observed incarnation only across that open,
  but intentionally persists no creation-time identity and makes no
  restart-stability claim.
- Missing/recycled exact instances return typed `SourceUnavailable` with the
  selected stable key. Access-denied, malformed identity, cleanup, and generic
  backend failures remain backend errors rather than being guessed as source
  disappearance or permission denial.
- Four platform-neutral fingerprint parsing/matching tests join the eight
  lifecycle/packet tests. Host tests, host and Windows ARM64 strict Clippy,
  Windows ARM64 check/tests, and scoped format pass. The latest full
  CODE_PROTOCOL rerun passes across 85 central Rust files.
- No public capture-type redesign, scaffold, mock, automatic restart, fallback,
  provider path, or new loopback-only behavior was introduced.

## Windows backend open and packet-delivery hardening — 2026-07-23

- Status: `SAFE-TO-TEST`; Windows target compilation and focused lifecycle
  contracts pass, while a new native Windows execution artifact remains open.
- The five-second backend-open deadline is now caller-bounded even when
  `ActivateAudioInterfaceAsync` does not return. Timeout marks the open
  cancelled, signals stop, drops the unopened worker handle without joining,
  and returns an error. Dispatch starts only after successful backend open.
- A late activation completion observes cancellation before process-loopback
  initialization. Every backend mode also rejects a late success notification,
  stops a stream that raced with timeout, and releases WASAPI/COM objects on
  the capture worker. Exact application capture never falls back to system mix.
- WASAPI packet size is checked with overflow-safe byte accounting before
  reading into the fixed buffer and checked again against the delivered frame
  count. Announced/delivered oversize increments
  `oversized_buffer_total`; query/read failure increments
  `stream_errors_total`; each condition terminates the failed stream instead
  of silently breaking only the packet-drain loop.
- Missing exact process-lifetime and input-device selectors now return
  `SourceUnavailable` with their selected stable key. Access-denied or otherwise
  inaccessible process/device failures remain backend errors rather than being
  guessed as disappearance or permission denial.
- Eight platform-neutral lifecycle/packet tests pass on the host. Targeted
  host tests, host strict Clippy, Windows ARM64 check/tests, Windows ARM64
  strict Clippy, workspace format, and CODE_PROTOCOL checks pass.
- The pinned `wasapi` activation helper has no cancellation API. If Windows
  never completes activation, one detached setup worker can remain until
  process exit; caller latency stays bounded and no capture or dispatch
  success is reported.
- No scaffold, mock capture, fallback, provider path, unbounded queue, or new
  loopback-only behavior was introduced.

## Linux native capture loss telemetry — 2026-07-23

- Status: `SAFE-TO-TEST`; native Linux execution remains required before a new
  `VM-PROVEN` artifact can claim zero loss at this boundary.
- Both PipeWire process callbacks and the ALSA capture loop now route pool
  acquisition through one hot-path helper. Exhaustion increments
  `pool_exhausted_total` with one relaxed atomic operation and drops the newest
  buffer without blocking.
- Both PipeWire producer paths route bounded SPSC pushes through one hot-path
  helper. A rejected push increments `dispatch_queue_full_total`; a successful
  push increments `frames_enqueued_total`. The helpers allocate, lock, block,
  log, await, and panic nowhere.
- Deterministic GWT tests exercise a real exhausted `AudioBufferPool`, a real
  full `rtrb` producer, exact counter snapshots, and pool-slot reclamation.
  All seven Linux crate tests pass in a Linux ARM64 Rust container with the
  PipeWire/ALSA development libraries. Native device execution remains an
  explicit acceptance command rather than a product-proof claim.
- An exact stable application or stable input device absent at open now returns
  typed `SourceUnavailable` with the selected stable key. Default-device,
  fuzzy-name, malformed-node, and generic backend failures retain their prior
  classifications. Selection remains fail-closed with no fallback.
- Focused Linux-source formatting, host package check/strict Clippy, and the
  Linux ARM64 crate test suite pass. The latest full protocol, hot-path
  allocation, workspace-format, and strict workspace-Clippy gates pass.
- No scaffold, mock, fallback, replacement-following, provider integration, or
  new loopback-only path was introduced.

## Native capture fault and identity closeout — 2026-07-23

- Status: `SAFE-TO-TEST`, with deterministic Linux live-source and Windows
  fail-closed cells `VM-PROVEN`.
- Windows WASAPI process-loopback now queries the selected PID before opening;
  an invalid PID can no longer initialize a silent stream and report capture
  success.
- `CaptureSource::identity_strength()` is now the canonical truth boundary.
  Windows process-only application sources report `ProcessId` rather than
  overstating executable names as stable application IDs. macOS and Linux keep
  their stronger identity only where platform evidence supplies it.
- The change is setup/control-path only. It adds no allocation, lock, blocking,
  logging, async work, or platform call to an audio callback.
- Linux live application and virtual-microphone disappearance cells retain
  exact lineage, return `failed-continuity`, finalize recording, and show zero
  reported capture-bridge, normalization, recorder-edge, or discontinuity
  drops. One initial-alignment gap per stem remains explicit. That artifact
  predates the Linux ring/pool telemetry above, so it does not claim zero loss
  for the newly visible boundary. Windows invalid PID/microphone cells return
  nonzero with diagnostics matching the expected patterns and leave no WAV
  after the CLI open-order correction.
- Output-device identities with a device UID now report
  `StableDeviceUid`, matching input-device identity semantics.
- Acceptance: 33 `pks-capture` tests, Windows ARM64 backend cross-check, native
  Windows ARM64 build, and final native negative cells pass. Full evidence:
  `../../docs/reports/2026-07-23-cross-platform-native-capture-proof.md`.
- No fallback, replacement-following, mock, scaffold, provider path, or new
  loopback-only product behavior was introduced.

## Full-workspace protocol and downstream contract gate — 2026-07-23

- Status: `SAFE-TO-TEST`. `scripts/check_protocol.sh` now scans all 85 Rust
  source files under `src`, `crates`, and `examples`; it no longer defaults to
  changed files or silently passes because the workspace has no root `src`.
- The gate enforces measurement suffixes, section-banner removal, GWT test
  names, canonical vocabulary, semantic choice types, and a new executable
  LAW-14 check requiring a nearby `SAFETY:` invariant for every unsafe block.
- Existing code was brought to the gate: measurement names retain explicit
  units/ratios, legacy tests use GWT names, PipeWire callback ownership is
  called a stream subscription, and normalized resampling writes into
  preallocated storage without `Vec::push` on `process()`.
- LAW-15 now executes zero-allocation gates for pipeline processing, runtime
  plan dispatch/execution, and the audio encode/decode path instead of emitting
  a non-binding grep warning.
- The full workspace protocol gate, strict all-target Clippy, and all central
  workspace tests pass, including the allocation gates.
- Downstream impact was checked in `pks`, `control-plane`, `relay`, and
  `web-receiver`. The current `pks` suite has 177 passing tests and strict
  Clippy passes. Control-plane short tests, relay short race
  tests, and web type-check/build pass. Their long soak packages were
  intentionally not rerun as part of this naming/protocol repair.
- No capture fallback, unbounded queue, mock, scaffold, provider integration,
  or loopback-only product path was introduced.

## Native ARM64 platform proof — 2026-07-23

- Status: Linux and Windows are `SAFE-TO-TEST` and `VM-PROVEN`; macOS remains
  the only `REAL-DEVICE-PROVEN` product slice.
- Windows natively builds the current backend and CLI and passes discovery,
  system mix, exact Edge process-tree capture, virtualized microphone capture,
  WAV, and three same-source capture-reopen cells per source. A concurrent independent
  distractor is rejected by 69.529 dB.
- Linux natively builds the current backend and CLI and passes discovery,
  system mix, exact PipeWire application capture, named virtual microphone
  capture, WAV, and three same-source capture-reopen cells per source. Stable
  application identity remains exact even when PipeWire exposes no PID; no
  system-mix fallback is permitted. A concurrent distractor is rejected by
  123.361 dB.
- Linux corrections cover one-time PipeWire initialization, joined object
  lifetimes, `InputDevice`, exact stable-node targeting, no-fallback session
  policy, SPA chunk/layout handling, and separation of the OS callback maximum
  from PocketStation's 10/20 ms transport frames.
- The final Linux matrix validates exit status, at least 80% requested WAV
  duration, and RMS above 0.001. All twelve cells pass; main WAVs are exactly
  5.00 seconds and expected tones survive.
- Evidence and remaining physical-device/drop-telemetry boundaries:
  `../../docs/reports/2026-07-23-cross-platform-native-capture-proof.md`.
- Linux also passes a bounded two-stem recording plus VM-to-host
  relay/browser cell: 750 source frames per stem, zero runtime/relay-edge
  drops or discontinuities, and 561 browser packets per bus with zero packet
  loss. Windows relay/browser and both platform connector cells remain open.
- No provider code, model graph, unbounded queue, mock capture fallback, or
  physical-device claim was introduced.

## Native ARM64 platform audit checkpoint — 2026-07-21

- Status: macOS remains `REAL-DEVICE-PROVEN`; Linux is `PARTIAL` and
  `VM-PROVEN`; Windows is `SAFE-TO-TEST` at the cross-compile boundary and
  native execution is `BLOCKED`.
- Mechanical Windows repairs restore
  `cargo check -p pks-capture-windows --target aarch64-pc-windows-msvc` and
  targeted format. They update stale exports/imports and current frame/source
  field names; they do not claim native capture.
- Strict Windows Clippy still reports the existing range-loop and
  eight-argument capture-loop findings. They were not suppressed.
- Native Linux evidence proves system-mix capture, while exact-process capture
  fails and then aborts during PipeWire teardown. `InputDevice` remains
  unsupported on Linux and Windows.
- Static inspection also found that Windows exact-process capture passes
  `include_tree = false`, which pinned `wasapi` maps to excluding the target
  process tree. This is a P0 correctness blocker for the next implementation
  step, not a product claim.
- Full evidence and ordered repair plan:
  `../../docs/reports/2026-07-21-cross-platform-native-capture-audit.md`.
- No capture redesign, provider code, scaffold, mock, fallback, or new
  loopback-only product path was introduced.

## W7.6 fast destination-fault matrix correction — 2026-07-20

- Status: `SAFE-TO-TEST`; deterministic destination isolation passes without a
  long soak or a simulated real-device claim.
- The local product-proof example hard-coded a 64-frame edge after branch-pool
  ownership gained one explicit receiver-in-flight slot. That requested 65
  slots from the fixed 64-slot atomic pool and panicked before every fault cell.
  The example now consumes the runtime plan's published maximum edge capacity
  instead of duplicating an invalid constant.
- A regression constructs every normal, slow-connector, slow-recorder,
  connector-failure, and recorder-failure topology and proves all bounded pools
  are valid before media starts.
- Five parallel two-second cells pass. Normal flush delivers 100/100 frames per
  destination. Slow connectors drop only their own 46–47 frames while both
  browser branches deliver 100/100. Connector failures emit one worker failure
  per connector while browsers deliver 100/100. Recorder failure finalizes
  incomplete while all connector/browser branches deliver 100/100. Slow
  recorder drops only recorder frames while connector/browser branches deliver
  102/102. Evidence is under
  `pocketstation-lab/artifacts/product-proof/w7-fast-*-2s-pass-2026-07-20`.
- This is deterministic bounded-runtime evidence, not permission, physical
  device, network reconnect, or production-scale evidence. No scaffold, mock,
  hidden fallback, provider code, unbounded queue, or hot-path work was added.

## W7.5 saturated-edge classification correction — 2026-07-20

- Status: `SAFE-TO-TEST`; the new long candidate remains open.
- The first corrected 60-minute candidate proved capture and recording
  continuity but exposed one synchronized destination recovery burst between
  2,470 and 2,480 seconds. Recorder edges peaked at 15/50 with zero loss;
  every eight-frame relay/connector edge saturated and dropped. Evidence:
  `pocketstation-lab/artifacts/product-proof/w7-soak-60m-corrected-2026-07-20`.
- When a branch copy pool and its queue are simultaneously full, the router now
  observes queue state before classifying a failed copy acquisition. Saturated
  edges report `queue_full`; `branch_pool_exhausted` remains reserved for copy
  ownership exhaustion while queue capacity is still available.
- The receiver-in-flight regression now explicitly requests
  `CopyToBranchPool`. It proves both sides of the contract: a popped in-flight
  frame does not prevent the next enqueue, and queue-plus-in-flight saturation
  is classified as queue-full without pretending the copy pool is undersized.
- Acceptance passes: all 36 runtime tests, both debug and release allocation
  gates, targeted strict Clippy, and workspace format. No queue wait, blocking,
  allocation, lock, logging, panic, scaffold, mock, or loopback-only path was
  added to dispatch.

## W7.4 branch ownership stress correction — 2026-07-20

- Status: `SAFE-TO-TEST`; the corrected isolated long candidate remains open.
- The rejected 60-minute candidate stayed live for 3,600 seconds with zero
  capture-bridge, normalization, recorder-edge, worker, and observation drops,
  but correctly failed the continuity gate. At one common host scheduling
  stall, the default eight-frame remote/connector edges reported one or two
  branch-pool exhaustion drops and both normalized inputs exposed one source
  sequence boundary. Evidence:
  `pocketstation-lab/artifacts/product-proof/w7-soak-60m-binding-2026-07-19`.
- The branch failures exposed an ownership-plan defect: a copy pool reserved
  only the queue capacity even though its sequential receiver can own one
  already-popped frame while the queue accepts the next frame. The memory plan
  now names that maximum in-flight ownership, reserves queue capacity plus the
  receiver allowance, and includes the extra slot in bounded memory accounting.
- A regression test holds the popped frame from a one-frame edge and proves the
  next frame is enqueued without pool exhaustion. A genuinely full slow edge
  remains isolated and is now classified as `queue_full`, not mislabeled as
  branch-pool exhaustion.
- The committed correction passed a 300-second real-device vertical slice:
  application delivered 15,000 frames and microphone delivered 15,001 frames
  independently to recording, relay/browser, and example connector branches.
  Every edge reported zero drops, zero branch-pool exhaustion, zero continuity
  events, and zero worker failures; recording finalized complete. Evidence:
  `pocketstation-lab/artifacts/product-proof/w7-branch-ownership-300s-2026-07-20`.
- The simultaneous source sequence boundary is not suppressed or reclassified.
  The corrected candidate must run without competing builds/fault injections;
  any repeated source boundary remains a W7 failure requiring native capture
  ownership/drop evidence.
- Acceptance passes: 71 `pks-graph` tests, 36 `pks-runtime` tests, both release
  allocation tests, targeted strict Clippy, and workspace format check. No
  scaffold, mock, loopback-only path, hot-path allocation, lock, blocking,
  async work, logging, or panic was introduced.

## W13 consolidated-candidate operational contract — 2026-08-02

- Status: `PARTIAL`, active for the exact W16 single-package candidate.
- The next focused gate binds already-owned capture authorization observations,
  source-generation recovery, route drop accounting, common-clock latency
  coverage, bounded queue/resource observations, and destination isolation into
  one deterministic integration contract.
- The contract must report attempted/delivered/dropped counts, explicit drop
  reasons, nanosecond units, latency sample coverage, and queue capacity/peak;
  zero samples or inferred permission state fail closed.
- This is component evidence only. It adds no automatic source substitution,
  permission guessing, virtual-driver dependency, provider implementation,
  product fallback, or physical-device claim.

## W7.3 exact-source and authorization truth — 2026-07-19

- Status: `SAFE-TO-TEST`; real permission-transition cells and the corrected
  long candidate remain open.
- Capture authorization snapshots now accept authoritative platform
  observations, carry a monotonic observation timestamp, and report an
  unavailable source as unavailable instead of synthesizing capability.
- The macOS control path reads microphone authorization without prompting.
  CoreAudio process-tap creation/start returns the exact operation stage and
  raw `OSStatus`; only the documented permission status maps to typed
  `PermissionDenied`, while every other status remains a typed backend failure.
- Recorder permission events now identify their scope as the explicit Session
  capture grant. They do not claim that a Session grant proves OS permission.
- The first exact-PID real smoke exposed an identity mismatch: process capture
  emitted a PID-derived frame source while the selected source and recorder
  retained the stable application identity. `CaptureMode::ExactApplication`
  now carries both the pinned PID and resolved stable ID. The macOS adapter
  opens only that PID and emits the stable frame identity; Linux/Windows
  targeted paths retain the same contract.
- Corrected real evidence:
  `pocketstation-lab/artifacts/product-proof/w7-exact-pid-auth-12s-pass2-2026-07-19`.
  Both stems reached recording, relay/browser, and connector branches with zero
  drops or continuity events; the microphone permission observation was
  authoritative `allowed` and the lifecycle event log was empty.
- macOS discovery now labels unbundled audio processes from their executable
  basename when AppKit has no application display name. The PID remains the
  exact process-lifetime identity; the fallback label is never promoted to a
  stable or security identity.
- No authorization query, allocation, lock, log, or error classification was
  added to an audio callback. No automatic restart, source substitution,
  system-mix fallback, scaffold, mock, or loopback-only path was introduced.
- Acceptance passes: `cargo test --workspace`, strict workspace Clippy, format,
  and the accepted `product_proof_local` example compile gate.

## W0 product-proof baseline — 2026-07-19

- Protected the existing graph/runtime, capture-adapter, relay, benchmark, and
  documentation worktrees before the Session façade implementation.
- Recorded repository HEADs, overlap classification, and exact acceptance
  results in the factory-root
  `docs/reports/PHASE2_W0_BASELINE_2026-07-19.md`.
- Baseline result: component tests, format, workspace Clippy, and
  `scripts/check_protocol.sh` pass; the accepted `product_quickstart` build fails
  only because the example does not yet exist.
- Added `PHASE2_QUEUE.md` with W1–W5 and the AUDIO-027/AUDIO-028 dependency
  decisions in execution order.
- Product state remains `PARTIAL`; W0 changed no runtime code and created no
  scaffold, mock, or loopback path.

## W1 Session API and lineage freeze — 2026-07-19

- Added the canonical `pks-audio` Session façade, safe source selectors,
  reusable destination handles, declarative routes, typed lifecycle failures,
  and an idempotent stop handle.
- Added compact `FrameLineage` and route-specific `DeliveryLineage` contracts in
  `pks-frame`, with nanosecond units and epoch semantics.
- Added the authoritative `product_quickstart`; it declares application and
  microphone capture to the same external connector and browser receiver plus
  two recording stems.
- A valid draft returns typed `RuntimeNotIntegrated` until W3 rather than fake
  success. The `PARTIAL` path is in the scaffold inventory.
- Acceptance: quickstart build, 121 tests plus one doc test, format, strict
  workspace Clippy, and `scripts/check_protocol.sh` pass.
- Freeze report:
  `docs/reports/SESSION_API_LINEAGE_FREEZE_2026-07-20.md`.
- Product state remains `PARTIAL`; W2 frame ownership is next.

## W2 frame fan-out ownership decision — 2026-07-19

- Accepted `docs/adr/AUDIO-027-frame-fanout-ownership.md` before changing pooled
  frame behavior.
- Decision: mutable exclusive capture/DSP frames freeze into immutable shared
  frames with per-slot atomic references; mutating branches use explicit
  preallocated copies.
- Frozen edge policies: `MoveExclusive`, `ShareReadOnly`, and
  `CopyToBranchPool`.
- Implemented exclusive-to-immutable freeze, per-slot atomic shared references,
  explicit preallocated branch copies, shutdown-draining shared edge channels,
  and planner ownership validation/memory accounting.
- Acceptance passes: 113 targeted debug tests plus one doc test, 21 release
  frame tests, strict workspace Clippy, `scripts/check_protocol.sh`, and
  `pool_bench` at 8.065–8.628 ns acquire/drop with no statistically detected
  regression.
- Runtime plan execution remains W3.

## W3 RuntimePlan edges and bounded Bridges — 2026-07-19

- Replaced linear-only execution as the canonical path with connected
  `RuntimePlan` node/edge execution in validated topological order.
- Preallocates one bounded edge queue and telemetry object per destination;
  realtime fan-out uses W2 move/share/copy ownership without per-frame heap
  allocation.
- Realtime-to-worker edges are returned as independent bounded partition
  crossings. Receiver shutdown drains queued frame references deterministically,
  and a failed/full destination cannot stop another branch.
- Per-edge observations now expose capacity/depth/peak, enqueue/delivery/drop,
  precise drop reasons, overruns, discontinuities, age, latency percentiles,
  worker failures, and shutdown discards.
- Added zero-allocation dispatch and connected-plan integration tests. Fixed the
  release gate so `assert_no_alloc` remains active instead of compiling out its
  allocator.
- Acceptance passes: 138 targeted debug tests plus graph doc test, 30 runtime
  release tests plus two release allocation tests, workspace format, strict
  Clippy, and `scripts/check_protocol.sh`.
- W3 status is `REAL` for compiled local runtime edges and bounded crossings;
  W4 supplies the first real file-I/O worker destination.

## W4 aligned multistem recording — 2026-07-19

- Accepted `docs/adr/AUDIO-028-multistem-proof-format.md` before recorder code.
- Added immutable source-to-session `TimelineMapping` in `pks-timing`; recorder
  workers consume the mapping and do not become a second clock authority.
- Added one independent `MultistemRecording` worker per compiled stem edge. File
  allocation, F32 WAV writes, checksums, event sidecars, metric sidecars, and
  finalization stay off realtime partitions.
- The proof directory contains `manifest.json`, independent stem WAVs,
  discontinuity/permission JSONL, and destination/summary metrics. Timestamp
  gaps receive silence and explicit events; overlaps are rejected visibly.
- Clean finish and explicit cancellation both drain bounded queues and finalize
  playable WAV headers. Worker errors produce `incomplete` state, exact errors,
  and `worker_failures_total` without stopping a healthy branch.
- Removed the registered atomic-counter `sink.recording` scaffold and its
  inventory row; recording now means file evidence, not a tally.
- Acceptance passes: 40 `pks-nodes` tests, 30 `pks-runtime` tests, two runtime
  allocation tests, the `product_quickstart` build, workspace strict Clippy,
  format, and `scripts/check_protocol.sh`.
- W4 status is `REAL` for local component recording. W5 now integrates two
  deterministic source-aware stems with connector/browser doubles and this real
  recorder; no remote/device claim is made.

## W5 local isolated vertical slice — 2026-07-19

- Added `pocketstation-lab/e2e/product-proof-local.sh` and the central
  `product_proof_local` example. The runner executes normal, slow connector,
  slow recorder, connector failure, and recorder failure cells concurrently.
- Found and fixed a real isolation defect: non-realtime edges retained shared
  capture-pool frames. They now default to preallocated branch-pool copies, with
  a planner regression test proving capture ownership isolation.
- The binding five-minute run generated 15,000 frames per normal stem. Every
  healthy browser and connector branch delivered every frame with zero drops;
  only the intentionally slow or failed branch reported drops/failure.
- Every cell produced two playable mono 48 kHz WAVs, manifests, integrity
  checksums, permission/discontinuity sidecars, and per-destination metrics.
- Evidence:
  `docs/reports/PHASE2_BOUNDED_EXECUTION_2026-07-27.md` and
  `pocketstation-lab/artifacts/product-proof/local-2026-07-19`.
- Acceptance passes: workspace tests, format, strict Clippy,
  `scripts/check_protocol.sh`, and the 300-second parallel proof runner.
- W5 is complete. Product status remains `PARTIAL` and `LOOPBACK-ONLY` because
  the integrated sources and connector/browser destinations are deterministic
  doubles. W6 real application plus physical microphone is next.

## W6.1 physical microphone capture checkpoint — 2026-07-19

- Found an exposed-but-nonfunctional `mic` path: `pks` parsed and queried an
  input device, but the shared `CaptureMode` could only represent system,
  application, and process output capture.
- Added typed `InputDeviceSelector::{Default, StableId}` and
  `CaptureMode::InputDevice`. macOS input discovery now uses CPAL's stable
  device identifiers and selects a concrete device rather than a PID/name
  approximation.
- Added `MacosInputSource`: CPAL's CoreAudio callback writes f32 frames into a
  preallocated pool and bounded `rtrb` queue; a worker invokes the caller. The
  callback allocates, locks, blocks, logs, awaits, and panics nowhere. Pool,
  queue, oversized callback, and stream failures are atomic observations.
- Added `DesktopCaptureSource` so application/system loopback and physical input
  dispatch remain explicit; `SystemLoopbackSource` was not made semantically
  dishonest by teaching it to open microphones.
- `pks capture from mic` and `pks publish mic` now resolve and open the physical
  input path when a device exists. Stable-selector preservation is tested, and
  CLI IDs retain the device UID instead of collapsing to `mic:0`.
- Fixed the existing profile normalization boundary: normalized frames retain
  source/stream identity, monotonic timestamps, sequence continuity, source tag,
  and encryption state. Standard ring and normalized-output drops are explicit
  counters instead of silent loss.
- Added the real `pks proof sources` W6.1 executable slice. It requires one exact
  discovered application and one exact physical microphone, opens each once,
  routes canonical frames over independent bounded edges, and produces the real
  multistem recorder artifact plus source/drop/queue evidence. It does not claim
  connector or browser delivery.
- Refactored the existing WebRTC publisher at the frame-receiver boundary and
  wired optional proof-session credentials to two independent remote edges.
  With `--session` plus `--token`, the same captured application and microphone
  frames feed the recorder and real publishers named `application` and
  `microphone`; no destination recaptures its source. RTP/drop/edge evidence is
  emitted. Browser-side receipt and RTCStats are still open, so W6.3 is
  `PARTIAL`, not passed.
- Extended the existing real Chromium RTCStats collector to accept repeated
  named `--bus` selections. It opens independent peer connections inside one
  browser context, labels every sample with its AudioBus, matches relay
  downlink/source-clock telemetry by bus, and fails unless every requested bus
  receives packets. A 10-second local component run received 406 application
  and 402 microphone packets with zero reported packet loss and exact relay bus
  matches. This is `LOOPBACK-ONLY` source evidence, not the missing physical
  app+mic artifact.
- Added `pocketstation-lab/e2e/product-proof-smoke.sh`. It creates relay
  credentials, runs the capture-once CLI proof and dual-bus browser collector
  concurrently, preserves logs/process status, validates all three
  destinations, and rejects incomplete/zero/drop/failure evidence. Its
  fail-closed unavailable-source cell creates no artifact. The binding
  five-minute real-device execution remains open.
- Wired optional example-owned Whisper delivery into the same capture-once
  topology. The application and microphone each have a separate compiled,
  bounded connector edge and worker. Workers consume live normalized frames,
  preserve source/stem/timestamp-range evidence, create independent 16 kHz mono
  WAV inputs, then run the existing `whisper-transcribe-example` processes in
  parallel after capture finalization. Queue, drop, failure, inference-latency,
  transcript, and input-path evidence enter the proof summary. This is W6.2
  delivery code, not a public `through()` API or a streaming-incremental STT
  claim.
- A later elevated real-device discovery exposed Spotify's application tap and
  the built-in CoreAudio microphone. The earlier sandbox-limited empty-device
  observation is retained only as a fail-closed diagnostic, not current host
  status.
- Acceptance passes: central workspace tests, strict Clippy, all 124 `pks`
  tests, format/diff checks, and `scripts/check_protocol.sh`. The unavailable
  source execution creates no false artifact.

## W6 real application + microphone proof complete — 2026-07-19

- A first 300-second execution failed visibly instead of producing a false
  PASS. It exposed callback-arrival timestamps being mistaken for source media
  time and a stereo initial-silence rounding defect in the recorder.
- Added `CaptureSampleTimeline`: macOS application, input, and ASP capture now
  anchor source time once and derive subsequent timestamps from cumulative
  device sample frames. Dropped observed buffers still advance both source time
  and sequence so real discontinuities remain visible. Small-chunk drift and
  callback-arrival-jitter regressions are covered by tests.
- Recorder silence sizing now rounds in sample frames before multiplying by the
  channel count, so every stereo gap remains interleaved-channel aligned and
  WAV finalization cannot fail on an odd sample count.
- Capture starts only after recorder, connector, and remote consumers are
  active. The smoke runner and Chromium collector use an explicit readiness
  file; the proof clock starts only after both named browser subscriptions are
  connected. This removed startup edge drops and end-of-run concealment caused
  by mismatched measurement windows.
- Connector acceptance now follows the binding W6 contract: nonzero delivered
  frames, successful real inference execution, zero edge drops/failures, and
  preserved lineage. An empty STT result is valid for a silent or non-speech
  stem and is no longer mislabeled as delivery failure.
- The binding 300-second real run used `app:com.spotify.client` and
  `mic:coreaudio:BuiltInMicrophoneDevice`. Application/microphone delivered
  14,994/14,987 frames to recorder, connector, and relay branches with zero
  capture, normalization, edge, encoder, or stale drops.
- Chromium received 14,948/14,941 packets on the exact `application` and
  `microphone` buses with zero packet loss and zero discarded packets. Final
  cumulative concealment was 0.002%/0.032%; maximum observed cumulative
  concealment was 0.004%/0.271%. Relay output sequence/timestamp
  discontinuities and pacer queue/stale/late drops were all zero.
- Both multistem WAVs finalized `complete`, with zero stale frames and one
  expected initial alignment range per stem. Checksums are
  `a28b17407b99326b` (application) and `526e2fa079144a63` (microphone).
- Evidence:
  `pocketstation-lab/artifacts/product-proof/real-app-mic-w6-pass-2026-07-19`.
  Status: `REAL-DEVICE-PROVEN`. W6 is complete; W7 reliability, permission,
  observation, and 60-minute soak work is next.
- Post-fix acceptance passes: canonical `cargo test --workspace`, strict
  workspace Clippy, all CODE_PROTOCOL laws, 126 `pks` tests, strict `pks`
  Clippy, Rust format/diff checks, lab TypeScript checking, shell syntax, and
  lab protocol lint with zero hard failures. The finalized W6 manifest passes
  the machine continuity predicate added after the run.

## W7.1/W7.2 normal-path checkpoint — 2026-07-19

- Status: `REAL-DEVICE-PROVEN` 30-second checkpoint; W7 remains `PARTIAL`.
- `pks-capture` now models authorization after a real open attempt: capability,
  OS permission observation, application policy observation, explicit Session
  grant, exact capture scope, identity strength, permission epoch, and open
  outcome. Unknown OS state is serialized as `not-observable`; backend failure
  is not guessed to mean permission denial.
- Runtime edge observations now expose capacity/depth/peak, every typed drop
  reason, typed continuity events, enqueue-to-receive latency with sample
  coverage, source-timestamp validity counts, worker failure, and shutdown
  discard. Attempted-frame counts and drop percentage use enqueued plus dropped
  dispatches as their explicit denominator. Receive-before-enqueue samples are
  counted as invalid rather than silently becoming zero latency. Both capture
  Bridges expose source/sink counters and executor and accumulator pool
  availability/failures.
- The first long soak exposed a receiver-instrumentation race without media
  loss: a destination worker sampled its receive timestamp before attempting an
  empty-queue pop, allowing a producer enqueue between those operations. The
  router now owns `PlanEdgeReceiver::try_recv`, which pops first and samples the
  canonical `pks-timing` process clock second. Explicit `recv_at` remains for
  deterministic runtime schedulers/tests and is crate-private so destination
  workers cannot repeat the race. A second candidate identified and removed the
  recorder's remaining pre-pop call. Both rejected attempts remain negative
  evidence; no tolerance or metric was relaxed.
- Physical microphone timestamps now use CPAL's authoritative relative capture-
  to-callback duration in the shared process clock. The CLI normalization
  Bridge derives output timestamps from cumulative normalized samples and no
  longer reanchors each full frame to callback jitter. A failing intermediate
  run proved why this matters: 1,500 microphone frames reached the remote and
  connector with no drops, but recorder rejection converted benign timestamp
  jitter into 1,024 synthetic gap events and failed halfway. The corrected run
  delivered and recorded all 1,500 frames with zero continuity events and zero
  future timestamps on every microphone edge.
- Runtime observations are written at start, every ten seconds, and finalization.
  The final proof decision uses router-owned edge telemetry after consumers have
  stopped, requires enqueue/delivery parity, and reconciles remote delivered,
  encoded, and RTP frame counts with source dispatch.
- A synchronized macOS run opened exact Spotify application capture and the
  built-in physical microphone. Each application edge enqueued and delivered
  1,499 frames and each microphone edge delivered 1,500, with zero drops,
  overruns, discontinuities, worker failures, or shutdown discards. Both
  recordings and connector branches completed; the remote publishers sent
  1,499/1,500 RTP packets with exact delivery/encode/RTP parity.
- Browser receipt had zero packet loss on both buses. The example connector used
  an English-only tiny Whisper model while Spotify content was uncontrolled and
  could use any language, so this proves connector delivery/execution, not STT
  recognition accuracy.
- Evidence:
  `pocketstation-lab/artifacts/product-proof/w7-normalized-clock-30s-pass4-2026-07-19`.
  The browser ended with zero packet loss, 0.141% cumulative application
  concealment, and 0% microphone concealment. These are same-host receiver
  observations, not competitive transport claims.
- The real browser-disconnect and connector-process-failure cells pass. In
  both cases the named failed branch was observed and every unrelated branch
  completed with exact frame delivery, zero drops, and complete recordings.
  Proof finalization now joins all destination workers and writes failure
  outcomes even if recorder finalization is incomplete or errors. Evidence:
  `w7-fault-browser-disconnect-30s-2026-07-19` and
  `w7-fault-connector-failure-30s-2026-07-19` under the lab product artifacts.
  The new soak wrapper also passed a 30-second acceptance execution with five
  runtime batches and nine RSS samples at
  `pocketstation-lab/artifacts/product-proof/w7-soak-30s-pass3-2026-07-19`;
  it is correctly labeled `SAFE-TO-TEST`, not the W7 60-minute soak.
  Open W7 gates are real permission transitions, source/relay/recorder restart
  and recovery paths, the 60-minute soak, and clean-checkout proof.

## Repository and timing partition - 2026-07-16

- Renamed the local central workspace from `pocketstation` to `pocketstation`; the
  workspace is the product center while `pks-*` crates keep narrow ownership.
- Removed `media-clock` from the central workspace dependency graph.
- Removed the unrelated `StreamProfile -> media_clock::Contract` mapping from
  `pks-codec`; codec profiles now own codec configuration only.
- Kept drift/correction and the compiled, tested experimental `SegmentGate` in
  `pks-timing`.
- Confirmed network pacing, RTP sequence/timestamp continuity, repair, and RTCP
  clock lineage remain relay media-plane responsibilities; no `pks-playout`
  crate was added.
- Added allocation-stable Opus PLC decoding to `pks-codec` so benchmark and
  receiver code do not need a separate codec wrapper.
- Acceptance: `cargo test -p pks-codec -p pks-timing` passes (32 tests total).
- The neutral benchmark's final `media-clock` dependency was removed: benchmark
  drift uses `pks-timing`, Opus PLC uses `pks-codec`, and reproducibility-only
  reorder/holdback stays private to the harness. Product docs now mark the old
  workspace archived rather than compatibility-active; the remote archive was
  verified 2026-07-16.
- Linux capture now uses the canonical `sample_rate_hz` field on `AudioFrame`
  and `CaptureSource`, closing the cross-platform strict-clippy failure without
  changing capture behavior.
- CI benchmark compilation and the allocation-free integration gate now target
  the canonical `pks-audio` package name instead of the retired
  `pocketstation-audio` name.

## Runtime timing ownership - 2026-07-16

- Added `pks-timing` as the single owner of clock drift estimation and PI clock
  correction.
- Replaced `pks-pipeline`'s duplicate `ClockSync` implementation with the
  runtime-owned controller while retaining a compatibility alias.
- Stopped treating an absolute frame timestamp as a measured clock offset in
  `ResampleNode`; correction now requires an explicit inter-clock observation.
- Preserved the future voice-output interruption state machine as compiled,
  tested `pks_timing::experimental::SegmentGate` code without exposing it as a
  current product feature.
- `media-clock` compatibility wrappers delegate to the new owner; the live
  CLI/codec path has since been decoupled from that workspace.
## Local Whisper connector example - 2026-07-13

- Added `examples/whisper-transcribe` as an example-owned `AsyncNode`; no provider dependency entered first-party crates.
- Binary WAV input crosses the async boundary and text output preserves sequence/timestamp lineage.
- Missing process/model and subprocess crashes fail visibly.
- Real whisper.cpp tiny English E2E passed in CPU mode with a 3.84-second spoken fixture; measured wall time was 1.08 seconds.
- GPU remains explicit opt-in because Homebrew whisper.cpp 1.9.1 crashed in the Metal backend on this machine.
## Bounded captured-frame stream - 2026-07-13

- Added a stable `FnMut(AudioFrame)` capture callback contract across the platform adapters.
- Added a bounded, non-blocking SPSC `CapturedFrameStream` with explicit delivered/drop counters and no hidden runtime.
- Unit tests pass for delivery, overflow, closure, callback adaptation, and invalid capacity.
- Real macOS exact-process capture passed with 281 consumed frames, 287,744
  samples, RMS 0.141005, and zero drops at the then-visible captured-frame
  stream boundary. That artifact predates the native-ring counters documented
  above and makes no zero-loss claim for them.
- All 112 CLI tests pass against the updated capture API.
- The capture-stream example is target-gated so Linux and Windows all-targets
  checks compile without pretending the macOS system-loopback endpoint exists.
- Linux capture tests explicitly reject the stream-capacity setup error so the
  cross-platform `CaptureError` contract remains exhaustively checked.

## Capture timestamp epoch observability — 2026-07-24

- `pks-capture` now exposes `timestamp_epoch_clamps_total` with the other native
  capture observations.
- Capture backends can initialize the shared monotonic timestamp domain during
  setup, before a realtime callback reads it.
- The macOS input adapter maps capture instants before the process epoch to the
  earliest representable non-zero timestamp and records each mapping. Zero
  remains reserved for an unavailable timestamp.
- The change adds no allocation, blocking, logging, or async work to the
  callback. The callback performs an initialized clock read, bounded
  pool/ring work, and relaxed atomic observations.
- The physical built-in-microphone acceptance produced 150/150 frames with no
  pipeline loss. Evidence is owned by
  `pocketstation-io/pks/artifacts/macos-mic-timestamp-repair-2026-07-24`.
- `bash scripts/check_protocol.sh`, focused capture/timing tests, strict clippy,
  the product quickstart build, and the full workspace tests pass.

## Session control events and capture-owned lineage — 2026-07-26

- `pks-session` now owns a bounded, non-blocking control-event queue with
  public polling and queue observations. Event values preserve stable session,
  stem, route, and endpoint IDs and keep source, endpoint, rollback, and
  finalization failures separate in the terminal outcome. Construction and
  publication authority remain crate-private.
- Queue overflow drops the newest event and records depth, peak depth,
  enqueue, drop, and receiver-closure counts. Six focused tests pass for FIFO
  delivery, overflow, closure, and terminal failure preservation.
- `pks-capture` now establishes `CaptureOpenMetadata` only after native open
  succeeds. `CaptureOwner` wraps frames with the authoritative session/stem
  seed, the declared monotonic capture clock, named initial source and
  permission epochs, and discontinuity state advanced from typed source runtime
  events. Session code no longer needs to fabricate lineage epoch literals.
- `cargo test -p pks-capture` passes all 50 tests and
  `cargo clippy -p pks-capture --all-targets -- -D warnings` passes.
- `RunningSession` now publishes the typed lifecycle, source, rollback,
  endpoint, finalization, and terminal events and exposes the sole receiver.
  Failed startup returns `SessionStartFailure`, retaining the root typed error,
  the bounded receiver, and every exact rollback failure instead of reducing
  rollback truth to a count or dropping the only receiver.
- The focused `pks-session` tests pass 27/27, strict all-target Clippy passes,
  and the full 107-file CODE_PROTOCOL gate passes. This is component-level
  `SAFE-TO-TEST` evidence; concrete relay/browser and example connector
  endpoint adapters plus the installed real-path Session proof remain open.

## W11 transactional RunningSession — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-session` now owns one real startup and shutdown
  transaction over the accepted graph, runtime, capture, and endpoint owners.
- Startup validates the narrow one-application plus one-microphone topology,
  prepares exact endpoint batches, starts endpoint workers behind one closed
  gate, opens both capture owners, creates the real runner, transfers all
  resources through a bounded worker Start command, waits for readiness, and
  opens the gate exactly once before publishing `Running`.
- Thread-spawn failure retains the captures and runner for caller-thread
  rollback. Every later failure unwinds runner, captures, started endpoints,
  and prepared endpoints in reverse acquisition order. Failed startup returns
  the root error, exact typed rollback details, and the bounded event receiver;
  it publishes `Failed` and a failed terminal outcome before returning.
- Runtime ingress accepts only capture-owned `LineagedAudioFrame` values.
  Typed source disappearance terminates the Session path and is preserved in
  the terminal outcome. Full bounded source queues, lineage rejection, runner
  failure, capture finalization failure, endpoint stop failure, endpoint join
  failure, and worker panic all prevent a successful stop result.
- Stop is idempotent. It requests runtime termination, joins capture and runner
  ownership, requests endpoint stop, joins and finalizes every endpoint, then
  publishes `Stopped` only for a clean result or `Failed` otherwise.
- Four transactional tests cover five endpoint owners for six routes,
  two authoritative source lineages, zero pre-gate delivery, repeated stop,
  endpoint prepare/start rollback, second-capture-open rollback, exact failure
  events, and zero leaked test owners.
- Integrated acceptance passes: 50 `pks-capture`, 43 `pks-runtime` plus three
  allocation gates, six `pks-endpoint`, 47 `pks-nodes`, and 27 `pks-session`
  tests; focused strict Clippy; workspace format; and the full 107-file
  CODE_PROTOCOL gate.
- No concrete relay/browser or connector driver, public compatibility façade,
  C adapter, provider implementation, mock product path, fallback, helper
  process, or new loopback-only path was introduced. Those remain later W11
  gates.

## W11 exact application process-instance declaration — 2026-07-26

- Status: `SAFE-TO-TEST`; the Session declaration now has an explicit
  `ApplicationSelector::ProcessInstance` form that retains both the selected
  process ID and the discovered stable application identity.
- Compilation preserves the two values as separate typed source-node
  configuration fields. Session startup lowers only this strong form to
  `CaptureMode::ExactApplication`; it is never weakened to the legacy bare
  `Process` or name-based modes.
- The existing `ProcessId` selector remains explicitly process-lifetime scoped.
  The existing stable-application selector remains stable-identity scoped.
  Linux and macOS capture-mode behavior is unchanged; the Windows backend
  continues to parse the stable process-incarnation fingerprint and verify the
  PID plus creation time before and after WASAPI activation.
- Focused tests prove compiled-configuration and runtime-mode preservation.
  `cargo test -p pks-session -p pks-capture --locked` passes 36 and 50 tests,
  strict focused Clippy passes, and both `pks-session` and
  `pks-capture-windows` cross-check for `aarch64-pc-windows-msvc`. The full
  108-file CODE_PROTOCOL gate passes.
- No fallback, mock, scaffold, provider integration, CLI/SDK behavior, capture
  hot-path work, or loopback-only product claim was introduced.

## W11 live plan-edge observation handle — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-runtime` now exposes a cloneable, read-only
  `PlanEdgeObservationHandle` from `PlanEdgeReceiver` before receiver ownership
  moves into an endpoint worker.
- The handle is exported from `pks-runtime`; downstream endpoint adapters do
  not reach through a private module or duplicate snapshots.
- The handle shares the existing authoritative `EdgeTelemetry` atomics. It
  introduces no duplicate counters and exposes no mutation, lifecycle, sender,
  or receiver authority.
- Live snapshots retain queue capacity/depth/peak, enqueue/delivery/drop
  reasons, latency coverage, discontinuities, worker failures, and shutdown
  discards after the receiver and router have been dropped.
- Three Given/When/Then tests prove producer-side queue saturation and full
  drops, consumer-side sequence/timestamp discontinuities, and post-drop
  shutdown-discard snapshots through a cloned handle. The focused plan-router
  gate passes 16/16 tests, strict all-target `pks-runtime` Clippy passes, and
  workspace formatting passes.
- No provider implementation, mock, scaffold, fallback, new counter source, or
  loopback-only product path was introduced. Concrete endpoint adapters still
  own the next W11 integration step.

## W11 portable codec build flags — 2026-07-28

- Status: `SAFE-TO-TEST`; the repository no longer injects host-specific
  `target-cpu=native` and `-march=native` flags into every Rust and C build.
- The global flags made release artifacts depend on the build machine and
  broke the real `aarch64-linux-android` codec archive at the NDK compiler
  boundary. The NDK correctly rejected the inherited host-only C flag.
- Native CPU tuning is now an explicit benchmark or local-development choice,
  not an implicit workspace policy. Release and SDK artifacts use the selected
  target toolchain's portable defaults.
- `scripts/build-android-codec-c.sh` pins NDK `26.1.10909125`, API 29,
  `aarch64-linux-android`, and `arm64-v8a`, supplies the exact NDK linker and
  archiver, and writes the immutable archive layout consumed by the Android
  SDK and lab gate.
- This change introduces no scaffold, mock, fallback, or loopback-only path.
  The Android linked-component gate remains responsible for proving the exact
  archive, JNI shared library, definitions, unresolved symbols, and hashes.

## W11 active architecture truth — 2026-07-28

- Active repository instructions now name `pks-dsp`, `pks-session`,
  `pks-session-c`, and `pks-codec-c` according to their accepted ownership.
- Superseded ADR implementation notes now distinguish retained decision history
  from current package and FFI guidance.
- The broad v3 AudioGraph document remains available as historical product
  vision, but its header and footer both point to the binding product,
  repository, execution, and crate contracts.
- No implementation, API, wire behavior, product claim, scaffold, mock, or
  loopback path changed.

## W12 Rust registry-role policy — 2026-07-28

- Status: `SAFE-TO-TEST`; the workspace now identifies the supported public
  Rust registry surface with machine-readable package roles.
- `pocketstation` is the single `public-facade`. Only its exact transitive
  workspace normal/target dependency closure is marked `facade-dependency`
  and remains publishable.
- `pks-codec`, `pks-codec-c`, and `pks-session-c` are explicitly `deferred`
  and `publish = false`; the Whisper example is explicitly `example` and
  remains non-publishable. These packages are not dependencies of the
  supported façade.
- `scripts/publish.sh` now derives and validates the façade closure instead of
  selecting every publishable workspace package. It rejects missing/unknown
  roles, role/closure drift, accidental non-closure publication, incomplete
  dependency ordering, and any order that does not place `pocketstation`
  last.
- This policy does not publish crates and does not create a provider,
  compatibility path, mock, scaffold, fallback, or loopback-only product
  behavior.

## W12 Linux protocol-gate dependency — 2026-07-28

- Status: `SAFE-TO-TEST`; the CI and release-validation environments now
  install `ripgrep` before invoking `scripts/check_protocol.sh`.
- The first main-branch W12 run passed workspace tests, strict Clippy, the
  release quickstart, and architecture constraints, then failed closed because
  the Ubuntu runner did not provide the protocol scanner's required `rg`
  executable.
- The correction changes no Rust source, product API, runtime behavior,
  publication closure, or release trigger. The publish job remains gated by
  the complete validation job.
- No scaffold, mock, fallback, provider, or loopback-only path is introduced.

## W12 crates.io partial-publication recovery — 2026-07-28

- Status: `SAFE-TO-TEST`; the first protected `0.1.0` publication passed its
  complete validation job and published six dependency crates before
  crates.io rejected the seventh first-time crate with HTTP 429 and an
  explicit retry timestamp.
- Publication is now exact-version idempotent: registry-visible versions are
  skipped, missing versions resume in dependency order, first-time crate names
  use conservative pacing, and only a crates.io 429 with a parseable retry
  timestamp can trigger a bounded retry. Registry-query failures and every
  other publish error fail closed.
- Manual recovery requires the exact version-matched release tag, current
  `main`, the release commit as an ancestor, and a path allowlist proving that
  no package or product source changed after the release.
- A fake Cargo/registry/sleep contract proves six-version skip and nine-version
  resume, 429 retry timing, and non-429 fail-closed behavior without network or
  publication.
- The partial-resume assertion compares the exact crate set rather than
  platform-specific ordering among independent topological nodes; dependency
  order and the public façade-last invariant remain enforced by the publisher.
- No crate source, public API, runtime behavior, scaffold, mock, provider, or
  loopback-only path changed.

## W12 public façade docs.rs correction — 2026-07-28

- Status: `SAFE-TO-MERGE`; docs.rs received `pocketstation 0.1.0` but build
  `3978948` failed before rustdoc because its default Linux target selected the
  native PipeWire backend and the docs.rs sandbox does not provide
  `libpipewire-0.3`.
- The public façade API surface is target-independent; native runtime backends
  remain platform-specific. Documentation will use one explicitly pinned
  Windows cross-compilation target, which exercises the public Rust surface
  without requiring a host audio development package.
- The façade patch release is independent from the unchanged `0.1.0` internal
  dependency closure. CI and protected release validation cross-document the
  configured docs.rs target before publication.
- The exact cross-target `cargo doc`, normalized package inspection, full
  workspace tests, strict Clippy, quickstart release build, architecture gate,
  CODE_PROTOCOL, Actionlint, and the 15-package closure dry run pass locally.
- This correction changes no Session API, runtime behavior, capture path,
  provider integration, scaffold, mock, fallback, or loopback-only claim.

## W12 Session-owned observed-lineage multistem recorder — 2026-07-28

- Status: `SAFE-TO-TEST`; `pks-nodes` now exposes the additive
  `SessionMultistemEndpointCoordinator::new(output_root, group_id)` declaration
  without accepting caller-fabricated Session, endpoint, route, stem, source,
  clock, generation, or permission identities.
- Exact `prepare_batch` inputs supply the Session, endpoint, typed stem/route,
  sample specification, label, recording group, and one common Session
  timeline origin. Missing typed context, mixed origins, and duplicate
  endpoint, route, stem, or label identities fail during preparation.
- The first delivered frame for every stem must carry authoritative
  `FrameLineage`. Its source, clock, source generation, and permission epoch
  initialize the recorder manifest; raw frames and later identity drift fail
  closed. Successful Session delivery records only
  `SessionCaptureGrant/Allowed` and does not claim an operating-system
  permission decision.
- Both stems map the common `SessionTimelineOrigin` to Session time zero, so
  independently captured application and microphone frames retain one
  comparable recording timeline.
- The public 0.1 `MultistemRecording::start` and explicit-config coordinator
  remain source-compatible. Their raw-frame compatibility path is unchanged;
  required-lineage behavior is selected by a private typed mode used only by
  the canonical Session coordinator.
- Focused tests prove two-stem derivation and final completion, common origin,
  missing typed route rejection, raw first-frame rejection without an
  artifact, and later permission-lineage mismatch with an incomplete outcome.
  All 56 `pks-nodes` tests, strict all-target/all-feature Clippy, workspace
  diff checks, and the complete 125-file CODE_PROTOCOL gate pass.
- No provider, connector, mock, scaffold, fallback, operating-system
  permission assertion, or loopback-only product claim was introduced. The
  full Rust façade and Lab reference artifact remain separate W12 acceptance
  gates.

## W12 central package-boundary repair — 2026-07-29

- Status: `SAFE-TO-MERGE`; the package repair defined by AUDIO-032 is
  complete on candidate `pks-20260729-w12-central-boundary-repair`.
- `pks-session` now owns stable language-neutral Session result codes and the
  bounded polled-audio projection consumed by foreign-language adapters. Its
  supported implementation no longer depends on the transitional
  `pks-nodes` package.
- `pks-recording` now owns the concrete multistem WAV endpoint, recording
  coordinator, finalization, manifest, and recording outcomes.
  `pks-nodes` keeps deprecated compatibility re-exports only while callers
  migrate.
- `pks-nodes` and `pks-dsp` are non-publishable deferred packages. The
  validated façade publication closure is now 14 packages and contains only
  packages required by `pocketstation`.
- Full workspace tests, strict workspace/all-target Clippy, formatting,
  release-mode `product_quickstart`, architecture constraints,
  `CODE_PROTOCOL`, and the 14-package publish dry run all pass at commit
  `364ebac`.
- This is a component and package-ownership result. It introduces no provider,
  connector, scaffold, mock, fallback, loopback-only path, or product-proof
  claim. The W12 Rust reference and Lab evidence remain the next acceptance
  slice.

## W12 public recording reference integration — 2026-07-29

- Status: `SAFE-TO-TEST`; the focused implementation and component gates pass,
  while the exact registry-installed Lab artifact remains the W12 exit.
- The accepted `pks-recording` coordinator, `StemHandle::record` declaration,
  Session host registration, and safe recording receipt remain the canonical
  owners. The active slice only projects their setup and outcomes through the
  public `pocketstation` façade.
- `Session::builder().recording_root(...)` registers that existing owner.
  Recording routes without an explicit root fail with
  `session.missing_recording_configuration`; `RunningSession` retains the
  terminal recording outcome. Stable recording codes cover every concrete
  recorder failure plus incomplete and not-yet-finalized outcomes without
  exposing string-message matching to language adapters.
- The public quickstart observes both source-aware stems under a bounded
  deadline and requires complete two-stem finalization. The deterministic
  conformance fixture emits truthful frame timing and proves that a saturated
  bounded polled-audio branch drops independently while both recording edges
  deliver 16 continuous frames without drops.
- Focused acceptance passes: 17 recording tests, 59 Session tests, eight
  façade tests including conformance, and strict all-target Clippy. The CLI
  migration and Lab exact-registry artifact remain separate repository gates.
- Registry patch identities now match the changed publication boundary:
  `pks-endpoint 0.1.1`, new `pks-recording 0.1.0`, `pks-session 0.1.1`, and
  public `pocketstation 0.1.2`. The public façade advances past the already
  published documentation-only `pocketstation 0.1.1`; the other ten closure
  packages remain at their
  already-visible `0.1.0` versions. The 14-package dry run passes in exact
  dependency order; no version already visible on crates.io is overwritten.
- No provider, new runtime, new package, mock, scaffold, fallback, or
  loopback-only production path is introduced.
- The pre-publication registry audit found that the documentation-only
  `pocketstation 0.1.1` release already occupies that façade version. The W12
  façade therefore advances to unused version `0.1.2`; the new endpoint,
  recording, and Session versions remain `0.1.1`, `0.1.0`, and `0.1.1`.
- A repeated release-gate run exposed a scheduler-dependent conformance
  failure: the finite fixture emitted four-sample frames on a three-millisecond
  wall cadence, allowing its eight-frame recording edge to overflow. The
  fixture now emits eight real 20-millisecond, 960-sample frames per source and
  waits on source-completion and destination-saturation counters rather than a
  sleep. The formerly failing recording-isolation case passes 20 consecutive
  runs.
- After that correction, full workspace tests, strict all-target Clippy,
  release quickstart compilation, `CODE_PROTOCOL`, architecture constraints,
  formatting, and the exact 14-package publication dry run pass for
  `pocketstation 0.1.2`.
- The `pocketstation-v0.1.2` validation run correctly withheld publication
  when its recovery test still expected the superseded 15-package closure.
  The recovery fixture now expects six already-visible packages and the exact
  eight missing packages, including `pks-recording` and excluding deferred
  `pks-dsp` and `pks-nodes`; it expects seven inter-package pacing waits.

## W13 macOS microphone authorization boundary — 2026-08-06

- Status: `SAFE-TO-MERGE`; physical acceptance remains bound to a newly
  committed and packaged candidate.
- The macOS input backend now rejects `Denied`, `Restricted`, and `Revoked`
  observations before opening CPAL, with the stable typed operation
  `opening the macOS microphone input stream`. Allowed and non-authoritative
  observations retain the native open path.
- Focused permission tests and strict workspace all-target Clippy pass. No
  prompt automation, permission inference, capture fallback, mock, scaffold,
  or loopback-only product path was introduced.
- Audit of the earlier frozen `pocketstation-0.1.2.crate` proved that it
  predates this repair. Its physical permission artifacts are retained as
  diagnostic history but cannot close exact-candidate W13 acceptance.

## W13 Windows bounded-worker scheduling repair — 2026-08-08

- Status: `SAFE-TO-TEST`; Windows guest acceptance remains a separate Lab
  artifact and no physical-device claim is made.
- A fresh W20-based ARM64 guest matrix passed system mix, exact application,
  microphone, two restart rounds, exact-application isolation, and the
  15-second concurrent app-plus-microphone Session proof. All three third
  restart cells then failed closed with explicit bounded dispatch/capture
  queue loss.
- The WASAPI capture producer already joined the Windows `Audio` MMCSS task,
  but its owned dispatch consumer did not. The consumer now joins the same
  task for its lifetime so ordinary scheduler pressure cannot starve the
  bounded handoff while the producer continues delivering packets.
- MMCSS and the one-millisecond timer request are now paired with explicit
  teardown on the owning worker. Registration remains fail-open, while every
  existing drop counter remains authoritative and fail-closed.
- No queue was made unbounded, no capacity or loss threshold changed, and no
  fallback, mock, scaffold, loopback product path, or soak was introduced.

## W18 public Session engine source registration — 2026-08-08

- Status: `SAFE-TO-MERGE` for the registration slice only; Session source
  declaration, compilation, preparation, and runtime lifecycle remain separate
  W18 acceptance tasks and are not claimed here.
- `SessionEngineBuilder::register_source_factory` now owns the validated open
  `SourceFactory` registry. `build()` transfers that registry into the one
  `SessionEngine`; read-only manifest lookup proves the registration is retained
  rather than written into an unused temporary registry.
- Registration is keyed by open `SourceTypeId`. Complete manifest validation is
  enforced at the public boundary, zero revisions fail typed, and a duplicate
  stable identity cannot silently replace the first registered revision.
- Three focused `session::source_registration` tests, strict all-target and
  all-feature Clippy, the release `product_quickstart` build, and the full
  `CODE_PROTOCOL` gate pass.
- No closed source enum, industry/customer vocabulary, capture callback change,
  runtime source worker, scaffold, mock, fallback, loopback-only path, platform
  matrix, or soak was introduced.

## W19 signal-aware async Operator preparation — 2026-08-08

- Status: `SAFE-TO-MERGE` for `W19-ASYNC-PREPARE-CONTEXT` only. Public named
  Session composition, Session-owned multi-stage runtime, and generated-audio
  reentry remain separate gated W19 tasks.
- `AsyncNode::prepare` now receives an `AsyncOperatorPrepareContext` containing
  the exact execution partition and bounded input/output edge records. Each
  record carries its named port, negotiated `SignalSpec`, `MediaCaps`, complete
  `EdgeContract`, optional compiled `EdgeId`, and capacity in signals.
- Session runtime preparation no longer converts an Operator input through
  `prepare_context_for_media` or fabricates `SampleSpec`. It preserves the
  compiler's named input, negotiated media and contract, and the exact bounded
  capacity from the typed-edge or audio-edge plan. Audio endpoint workers keep
  their specialized audio-only `PrepareContext`.
- Direct external harnesses with wildcard manifests can supply an explicitly
  negotiated signal-shaped context through `spawn_with_context`; its ports,
  contracts, and capacities must agree with the actual bounded worker edges or
  preparation fails closed.
- Real prepare execution is covered for text, event, metrics, control, binary,
  and schema-backed custom signals. A rejection test proves that a capacity
  mismatch cannot pass as accepted preparation. The existing Session-owned
  audio-to-Operator route and the external Whisper connector both pass using
  negotiated audio media.
- Focused async-operator and graph gates, all 508 library tests plus all
  integration/ABI/hot-path targets, strict all-target/all-feature Clippy,
  Whisper's 15 tests and strict Clippy, release `product_quickstart`, and the
  complete `CODE_PROTOCOL` gate pass.
- No realtime callback, audio pool, hot-path executor, provider vocabulary,
  customer/domain type, scaffold, mock, fallback, loopback-only product path,
  platform matrix, release, or soak was introduced.

## W19 Session operator instance and named-connection declaration — 2026-08-08

- Status: `SAFE-TO-MERGE` for `W19-OPERATOR-INSTANCE-CONNECTIONS` only.
  Session-owned multi-input runtime execution remains the next separately gated
  task; this checkpoint does not claim that a multi-input Session can start.
- `Session::operator` now declares exactly one `OperatorInstanceSpec`.
  `StemHandle`, `SourceOutputHandle`, and `DerivedStreamHandle` connect to an
  exact `OperatorInputHandle`, while `OperatorInstanceHandle::output` exposes
  exact named outputs. `through()` writes the same instance and connection
  records and is no longer a separate one-input schema.
- Session schema 1.4 separates operator identity/configuration from
  `OperatorConnectionSpec`. One instance can own multiple named input
  connections and multiple named derived routes without hidden duplicate
  nodes. The previous `OperatorSpec` name remains only as an explicitly
  deprecated compatibility alias.
- The Session compiler creates all declared operator nodes before wiring
  connections. This permits operator-to-operator references independent of
  declaration order and validates unknown ports, missing required inputs,
  duplicate single-multiplicity inputs, signal/media compatibility, direction,
  and graph cycles before runtime preparation.
- Focused `session::operator_connections` and `graph::named_ports` gates pass.
  Compiler tests prove that two independent audio stems enter one two-input
  operator node and that its two named outputs leave that same node. All 516
  library tests and every integration/ABI/hot-path target pass, as do strict
  all-target/all-feature Clippy, release `product_quickstart`, architecture
  constraints, formatting, and the complete `CODE_PROTOCOL` gate.
- No runtime owner, callback, audio pool, hot-path executor, provider/customer
  vocabulary, scaffold, mock, fallback, loopback-only path, platform matrix,
  release, or soak was introduced.

## W19 Session-owned composed operator runtime — 2026-08-08

- Status: `SAFE-TO-MERGE` for `W19-SESSION-COMPOSED-RUNTIME` only.
  Generated-audio reentry, the W19 fault matrix, and every W20 release/freeze
  task remain separately gated and are not claimed here.
- `PreparedSession` and `RunningSession` now own one worker per declared
  operator instance across compiled audio inputs, external typed-source inputs,
  and upstream operator outputs. Three-stage chains and named multi-input/output
  instances use one bounded typed-edge implementation and retain exact port,
  signal, media, edge-contract, capacity, lifecycle, cancellation, join, and
  finalization observations.
- Public `RunningSession` metrics expose each Session-owned external source,
  operator instance, exact input port, and derived route without exposing
  internal worker or fanout constructors. Shutdown cancels composed operators
  in reverse dependency order and preserves the existing graceful drain order.
- The external public-Session consumer found and fixed a compiler authority
  defect: a derived endpoint whose chain originated at an external source had
  inherited the source-input connection route ID. Endpoint configuration now
  always carries the actual derived route ID while source identity remains
  independent metadata.
- The focused public-Session composition test passes, all 517 library tests and
  every integration/ABI/hot-path target pass, strict all-target/all-feature
  Clippy passes, the release `product_quickstart` builds, and the complete
  `CODE_PROTOCOL` gate passes.
- This slice changes no capture callback, audio pool, or realtime executor. It
  adds no unbounded queue, manual fixture worker/fanout, generated-audio claim,
  provider/customer/domain vocabulary, scaffold, mock, loopback product path,
  platform matrix, release, freeze, or soak.

## W20 typed compiled Session bindings — 2026-08-09

- Status: `SAFE-TO-MERGE` for `W20-TYPED-SESSION-BINDINGS` only. Endpoint and
  recording ownership, public API reduction, module decomposition, executable
  extension ABI, sidecar lifecycle, packaging, release, and Core 1.0 freeze
  remain separately gated.
- `CompiledSession` now owns a typed `CompiledSessionBindings` table keyed by
  compiled `NodeId`. Each Session-created node carries its exact source, stem,
  operator, endpoint route, stream origin, or generated-audio ownership without
  encoding those identities into `NodeConfig` string keys.
- Runtime preparation consumes those typed bindings for built-in and external
  sources, operator inputs and outputs, raw and derived endpoints, and generated
  audio ingress. Missing or incompatible bindings fail with typed preparation
  errors; Session metadata is no longer reparsed from extension configuration.
- `NodeConfig` remains the open, opaque extension-owned configuration surface.
  An external endpoint may use a key that previously collided with an internal
  metadata name without overriding the compiled Session identity. No provider,
  customer, or domain enum was introduced.
- The polled-audio endpoint now receives its route through typed
  `EndpointRouteContext`; this removed its last dependency on a string route ID
  and fixed every affected Session host and C-ABI regression cell.
- Focused compiler tests pass 13/13, focused runtime-preparation tests pass 5/5,
  and all 410 all-target/all-feature tests pass. Strict Clippy, the release
  `product_quickstart`, formatting, hot-path checks, and the complete
  `CODE_PROTOCOL` gate pass.
- This slice changes no capture callback, audio buffer pool, realtime executor,
  queue capacity, platform implementation, product path, scaffold, mock,
  loopback claim, physical evidence, release, freeze, or soak.

## W20 endpoint and recording ownership — 2026-08-09

- Status: `SAFE-TO-MERGE` for `W20-ENDPOINT-RECORDING-OWNERSHIP` only. Public
  API reduction, module decomposition, executable extension ABI, sidecar
  lifecycle, packaging, release, and Core 1.0 freeze remain separately gated.
- Every endpoint input now uses one `EndpointPortInput` containing its exact
  port, `SignalSpec`, media, `EdgeContract`, receiver, and required typed
  Session route context. `EndpointReceiver` represents either the bounded
  realtime audio receiver with its prepare context or the bounded signal
  receiver; provenance does not select a different endpoint interface.
- `EndpointDriverFactory` has one `prepare` method and
  `EndpointDriverRegistry` has one `prepare_batch` dispatch. The former
  `prepare_derived`, `DerivedEndpointDriverInput`, separate signal-route
  context, and derived-contract option fields are removed.
- Raw audio and operator-derived routes are assembled into the same pending
  input representation and enter one endpoint batch-preparation function.
  Default factories own one lifecycle per route; factories may declare an open
  typed shared group without Session recognizing recorder or provider names.
- The multistem recording factory now owns group selection and validation.
  Recorder descriptor/configuration construction and the `record` convenience
  API live in the recording-specific Session extension; generic Session draft
  and runtime code no longer inspect recording group policy.
- Focused endpoint tests pass 31/31 and recording tests pass 12/12. All 410
  all-target/all-feature tests, strict Clippy, release `product_quickstart`,
  formatting, hot-path checks, and the complete `CODE_PROTOCOL` gate pass.
- This slice changes no capture callback, audio buffer pool, realtime executor,
  queue capacity, platform implementation, scaffold, mock, physical proof,
  release, freeze, or soak.

## W20 capability and performance recovery — 2026-08-09

- Status: `SAFE-TO-MERGE` for the named recovery task; candidate
  `pks-20260809-capability-performance-recovery-3` is bound by the hashed
  acceptance manifest. This is not public API, release, Core 1.0 freeze,
  physical-platform, or competitive acceptance.
- Real libopus encode/decode/PLC and deterministic timing drift/correction are
  again compiled in the single `pocketstation` engine. Focused codec/timing,
  100-frame allocation gates, and the previously compiled C++ codec consumer
  pass. Criterion encode/decode/PLC targets compile; measured baselines remain
  pending.
- The Rust capture handoff now has direct zero-allocation gates for normal
  delivery, full-ring drop-newest, closed-start discard, and pool exhaustion.
  Seventy-one focused capture tests pass. These gates do not substitute for
  physical CoreAudio, PipeWire, or WASAPI callback evidence.
- Typed asynchronous edges now enforce both count and byte bounds: 64 signals
  maximum per branch, 1 MiB default payload, and 16 MiB hard payload maximum.
  Oversized payloads are rejected before partial fan-out, and observations
  expose maximum payload and maximum buffered payload bytes. Focused
  typed-edge and async-operator tests pass; measured baselines remain pending.
- The stale empty `src/dsp/` and `src/runtime/nodes/` directories were removed.
  Simplified historical AEC/denoise/VAD/watermark code remains deliberately
  absent rather than being restored as a production claim.
- No long soak, provider/customer/domain vocabulary, unbounded queue, second
  engine, or LiveKit superiority claim was introduced.

### Measured recovery update

- Corrected every Criterion target to use `harness = false`; the benches now
  execute rather than merely compile under Rust's default benchmark harness.
- Added measured local baselines for pool acquire/drop, synthetic capture
  handoff, captured-frame SPSC, Opus encode/decode/PLC, typed one/three-branch
  fan-out, three-branch runtime routing, realtime executor, two-source runner,
  generated-audio reentry, and public two-source Session lifecycle. Exact
  intervals and limitations are recorded in
  `docs/execution/evidence/W20-CAPABILITY-PERFORMANCE-RECOVERY/LOCAL_SHORT_BASELINE_2026-08-09.md`.
- Replaced the generated-audio bridge's 1 ms idle polling dependency with an
  explicit non-realtime consumer-thread wakeup. The local typed-PCM to pooled
  audio round trip measured 3.2206–3.3499–3.5198 us. A 100 ms timeout remains
  only as lost-wakeup/abandonment fallback.
- Added a standalone `conformance-fixtures` feature compile correction; it no
  longer relies on `internal-testing` to name the crate-private capture delivery
  result. The Session lifecycle benchmark is explicitly `LOOPBACK-ONLY`.
- The queue inventory found variable-size control events that were count-bound
  but not byte-bound. Capture-runtime events now enforce a 64 KiB per-event and
  `capacity × 64 KiB` owned-memory ceiling; Session events enforce 1 MiB and
  `capacity × 1 MiB`. Both expose byte depth/peak and oversized-drop truth and
  have direct rejection tests. Fixed-size trace records and capacity-one async
  lifecycle rendezvous are now explicit in the capability ledger.
- Removed historical codec policy profiles from Core. Explicit `OpusConfig`
  remains the codec primitive; product/transport presets belong outside Core.
- `pocketstation::codec` and `pocketstation::timing` are now deliberate public
  capability namespaces. The Opus API uses caller-owned buffers, validates the
  configured frame duration, supports bounded 10/20/40/60 ms frames, and has no
  shipping mock encoder/decoder or allocating vector convenience path.
- Added direct-libopus calibration cases beside the wrapper Criterion cases.
  The later local wrapper slowdown matched direct libopus in the same process,
  so it is recorded as a host power/scheduling regime change rather than a Core
  regression; no unsupported threshold or LiveKit comparison is claimed.
- Audited the target capture boundaries in source. Rust CPAL, PipeWire, and
  WASAPI paths plus the native macOS tap/ASP ring paths now have a protocol
  regression gate forbidding allocation, locks, blocking, async, logging, and
  panic operations at their callback/write boundary. macOS RMS calculation was
  moved from the tap callback to the non-realtime reader. Physical target proof
  remains separately pending and is not inferred from this source audit.
- Added a measured timing baseline for drift observation/snapshot and PI
  correction tick. Exact intervals, command lines, host limitations, and all
  other local measurements remain in the named evidence document.
- All 440 unit tests plus integration/ABI/allocation targets pass. Strict
  all-target/all-feature Clippy, release quickstart, all benchmark compilation,
  and `CODE_PROTOCOL` pass. Physical platform and competitive claims remain
  pending and are not inferred from these local component measurements.

## W20 ownership remediation in progress — 2026-08-09

- Status: `PARTIAL`. These edits are implementation work on the dirty current
  candidate and have not been rerun through acceptance. Earlier green evidence
  does not approve them.
- Session lifecycle telemetry no longer lives inside `running.rs`: operator
  input aggregation and runtime/final metric bindings now have private,
  lifecycle-owned modules. This changes no queue, counter, pool, capture
  callback, worker scheduling, or endpoint behavior.
- The async signal runtime now separates bounded operator I/O, observation
  state, and worker failure vocabulary from worker execution. The existing
  typed-edge count/byte bounds and saturation outcomes remain the authority;
  no unbounded or realtime signal queue was introduced.
- Session compilation and preparation now separate immutable compiled output,
  typed runtime mappings, and compile/prepare errors from their coordinating
  algorithms. The compiler still emits `CompiledSessionBindings` and runtime
  preparation still consumes those bindings directly; `NodeConfig` remains
  extension configuration rather than an internal identity transport.
- The deferred gate remains unchanged: no tests, benchmarks, soak, platform
  claim, public API acceptance, ABI acceptance, release, or Core 1.0 freeze is
  inferred from this structural slice.

## W20 public API boundary — 2026-08-12

- Status: `SAFE-TO-MERGE` for `W20-PUBLIC-API-BOUNDARY`; this does not accept
  module decomposition, native ABI callbacks, sidecar hosting, release, or the
  Core 1.0 freeze.
- The normal crate root now exposes the Session façade, checked signal and
  audio ownership contracts, source/operator/endpoint authoring contracts,
  stable errors, and immutable observations. Runtime workers, routers, queue
  constructors, registries, compiler/preparation owners, and sidecar framing
  remain private; the hidden `internal-testing` feature is the only benchmark
  reach-through.
- `Stream<T>` remains Rust compile-time façade metadata. Runtime and ABI
  identity remain `SignalSpec` plus stable schema identifiers; no Rust `T`,
  provider type, customer type, or industry vocabulary entered the engine.
- A packaged external repository compiled and ran supported capture, source,
  operator, endpoint, typed-stream, and Session declarations without
  `internal-testing` or a source-tree reach-through.
- The exact all-target/all-feature tests, all-feature rustdoc, packaged
  consumer, strict Clippy, release `product_quickstart`, and `CODE_PROTOCOL`
  gates pass. The acceptance manifest is
  `docs/execution/evidence/W20-PUBLIC-API-BOUNDARY.acceptance.json` in the
  workspace execution authority.
- No capture callback, pool ownership, queue capacity, realtime executor,
  physical platform path, competitive claim, or endurance claim changed.

## W20 module decomposition — 2026-08-12

- Status: `SAFE-TO-MERGE` for `W20-MODULE-DECOMPOSITION`; native callbacks,
  sidecar hosting, release, and Core 1.0 remain unaccepted.
- Graph contracts now have the canonical public `pocketstation::graph`
  namespace while compiler, plan, registry, and runtime machinery stay private.
- Session composition for polled audio and recording now lives under Session
  extensions; lower frame, capture, graph, runtime, endpoint, recording,
  codec, and timing modules do not import Session.
- Legacy direct operator-worker construction is test/internal-only. The public
  shipping path is Session-owned composition, not a second runtime.
- No source directory is empty. Codec and timing are compiled owners; advanced
  DSP remains intentionally external rather than represented by a placeholder.
- All-target/all-feature tests, strict Clippy, release quickstart, and protocol
  gates pass. The acceptance manifest is
  `docs/execution/evidence/W20-MODULE-DECOMPOSITION.acceptance.json`.
- No callback, pool, bounded queue, hot-path `Drop`, realtime executor,
  physical path, or endurance claim changed, so no new long soak is required.

## W20 native executable ABI — 2026-08-12

- Status: `SAFE-TO-MERGE` for `W20-NATIVE-EXECUTABLE-ABI`; candidate
  `pks-20260812-w20-native-abi-3` is hash-accepted. This does not accept the
  sidecar, managed SDK parity, release, physical proof, or Core freeze.
- Extension ABI 1.1 copies descriptor data and owns validation, create,
  prepare, produce/process/consume, request-stop, finish, instance destroy,
  and registration destroy through the existing Session engine. C callbacks
  execute only on source/operator/endpoint workers and cannot declare the
  reserved realtime PCM signal namespace.
- Payload buffers and typed queues are bounded. A C metrics snapshot projects
  the native Session source/operator/route/endpoint counters, byte ceilings,
  saturation, and failures; a typed-only Session reports no synthetic audio
  queue.
- The packaged installed-header consumer passes success and operator-failure
  lifecycle cases with C11 atomic observations. A consumer compiled against a
  frozen Extension ABI 1.0 header still links and runs against the candidate.
- All 443 unit tests plus integration/ABI/allocation/benchmark targets, strict
  Clippy, release quickstart, and `CODE_PROTOCOL` pass. Acceptance is
  `docs/execution/evidence/W20-NATIVE-EXECUTABLE-ABI.acceptance.json`.
- No capture callback, audio pool, `rtrb` edge, realtime executor, or hot-path
  `Drop` changed; no new endurance run is required.
- Sidecar lifecycle, managed-language conformance, release, and Core freeze are
  explicitly outside this task.

## W20 cross-language conformance — 2026-08-12

- Status: `LOOPBACK-ONLY` and hash-accepted for
  `W20-CROSS-LANGUAGE-CONFORMANCE`; candidate
  `pks-20260808-w20-language-conformance-4` is not a product, remote,
  physical-device, release, or Core 1.0 claim.
- Rust `Stream<T>`, executable C source/operator/endpoint callbacks, Python,
  JavaScript, and a Python PKSS sidecar execute the canonical public Session
  implementation from the same installed `pocketstation-0.1.2.crate` source
  artifact.
- Every embedded language matches one neutral success and injected
  operator-failure vector: stable `SignalSpec`/schema/role identity, source,
  operator and endpoint lifecycle counts, bounded queue/route capacities,
  maximum buffered bytes, terminal observations, and stop outcome.
- The Session-owned sidecar additionally proves stable signal/schema identity,
  success echo, crash isolation, and deterministic child reaping through the
  bounded PKSS protocol.
- Rust `T` remains façade-local. `TypeId`, `PhantomData`, and `Stream<T>` do
  not cross the C header, language-neutral vector, or PKSS protocol. The C ABI
  remains C++-parsable, fixed-width, explicitly sized/versioned, and based on
  opaque Session handles, which is the frozen contract for later Swift/Kotlin
  projections without implementing another engine now.
- The exact three acceptance commands and independent verifier pass. The
  acceptance manifest is
  `docs/execution/evidence/W20-CROSS-LANGUAGE-CONFORMANCE.acceptance.json` in
  the workspace authority.
- No capture callback, audio pool, `rtrb` edge, realtime executor, codec, or
  timing owner changed; no long soak was run or required.

## W20 clean installed consumer — 2026-08-12

- Status: `LOOPBACK-ONLY` and hash-accepted for
  `W20-CLEAN-INSTALLED-CONSUMER`; no product, remote, physical-device,
  release, or Core 1.0 claim is inferred.
- The proof creates and commits a separate source repository, resolves Cargo
  only to the extracted `pocketstation-0.1.2.crate`, installs a built Python
  wheel and JavaScript native module, links the installed C header/library,
  and drives the installed bounded PKSS sidecar protocol.
- The clean repository supplies typed and PCM sources, three chained
  operators, a named multi-input/output operator, typed and audio endpoints,
  generated-audio reentry, executable C callbacks, Python, JavaScript, and a
  Python sidecar without a workspace reference, relative dependency, source
  patch, `internal-testing`, or a core edit.
- Generated audio delivers four PCM frames with queue peak 4/8, exact 61,440
  byte queue-plus-pool bound, zero unexplained loss, and joined Session-owned
  lifecycle. The committed external Git tree remains byte-clean after all
  consumers execute.
- The exact three acceptance commands and independent hash verifier pass. The
  acceptance manifest is
  `docs/execution/evidence/W20-CLEAN-INSTALLED-CONSUMER.acceptance.json` in the
  workspace authority.
- No capture callback, pool, realtime queue/executor, codec, timing, or hot
  path changed; no endurance run was performed.

## W20 final W14-W17 requalification — 2026-08-12

- Status: `LOOPBACK-ONLY` and hash-accepted for
  `W20-FINAL-REQUALIFICATION`; candidate
  `pks-20260812-w20-final-requalification-9` is the accepted installed-package
  regression candidate, not a physical, remote, release, or Core 1.0 claim.
- The extracted package passed 443 all-feature unit tests, public Session and
  SessionTrace tests, signal/core-extension tests, C and C++ ABI conformance,
  all-target/all-feature compilation, and the public quickstart build.
- The packaged native surface contains the real process-tap, authorization,
  ASP reader and explicitly opt-in direct ASP driver sources. The obsolete
  native stub and broken platform example are absent from the package.
- The external Whisper provider and its consumer compile and test against the
  extracted package without `internal-testing` or private runtime imports.
- The public Session real-Whisper cell delivered 60 raw frames on an
  independent audio branch and one typed transcript through two successful
  real provider invocations. Bounded saturation accounted for two input drops;
  source close, operator process, lineage, endpoint, runtime and finalization
  outcomes remained clean and Session stop succeeded.
- Immutable W15, W16 and W17 artifacts were independently reverified without
  rerunning W10 endurance or changing any historical evidence classification.
  Acceptance is
  `docs/execution/evidence/W20-FINAL-REQUALIFICATION.acceptance.json` in the
  workspace execution authority.

## W20 Core 1.0 release preparation — 2026-08-13

- Status: `SAFE-TO-TEST` for release candidate
  `pks-20260813-w20-release-1-0-0-17`; publication and independent registry
  consumption remain acceptance predicates and are not claimed here.
- The single package version is `1.0.0`. Release notes state the exact
  extension-complete architecture and preserve the accepted physical,
  loopback, platform, and competitive evidence boundaries.
- The package includes the native sources and public C header. The publish
  gate no longer permits `--allow-dirty` or `--no-verify`; publication must
  originate from one clean intentional release commit.
- No Session behavior, public Rust API, C ABI, PKSS protocol, callback, pool,
  queue, codec, timing, or runtime execution path changed in this slice.

## Core 1.0.1 documentation and packaging correction — 2026-08-13

- Status: `SAFE-TO-TEST`. Version `1.0.0` was published, but docs.rs selected
  the configured Windows target on its Linux builder and failed while building
  bundled Opus with an incompatible cross toolchain. The Core freeze remains
  gated until the corrective patch is published, documented, and independently
  consumed.
- Version `1.0.1` selects docs.rs' native Linux target and builds the public
  contracts with `default-features = false`. Native capture remains the default
  product feature on macOS, Windows, and Linux; disabling default features now
  provides an explicit contracts-only build for rustdoc and tooling.
- The public README is now the crate-level docs.rs landing page and leads with
  developer outcomes, a compiling Session example, the two-lane execution
  model, realtime guarantees, extension contracts, platform evidence
  boundaries, prerequisites, and direct documentation paths. The linked public
  guides are included in the crate archive.
- The single-package publish guard permits exactly those six curated public
  documentation files and rejects every other `docs/` path, preserving the
  original ban on execution evidence, ADRs, internal standards, and historical
  reports in the registry artifact.
- Contracts-only Clippy and rustdoc pass with warnings denied; all-feature
  Clippy also passes. No callback algorithm, pool capacity, queue policy,
  `AudioFrame`, codec, timing, C ABI, PKSS frame, or Session execution semantic
  changed.
- The compatibility tools now accept an explicitly hash-pinned registry
  archive and discover its versioned package root, allowing the patch release
  to be checked directly against immutable `1.0.0` bytes instead of only the
  pre-release `0.1.2` baseline.
- Public permission documentation now distinguishes an unavailable preflight
  observation from actual capture authorization: macOS provides the current
  authoritative query, while every backend reports the authoritative selected
  source result during Session prepare/open. `NotObservable` is explicitly
  neither success nor denial.

## Windows microphone permission preflight — 2026-08-13

- Status: `SAFE-TO-TEST` for patch `1.0.2`. Windows 10 version 1903 and newer
  now uses the non-prompting `AppCapability("Microphone").CheckAccess()`
  authority. Allowed, user-denied, system-restricted/not-declared, and
  prompt-required states map to the existing stable permission vocabulary;
  API or platform failure remains honestly `NotObservable`.
- The query initializes WinRT only on the calling control thread, accepts an
  already initialized apartment, balances initialization ownership, never
  requests access, and never runs on a capture callback.
- Linux intentionally remains `NotObservable` at preflight because XDG portal,
  PipeWire/WirePlumber, direct ALSA, ACL, sandbox, and container policy do not
  expose one stable process-wide microphone permission authority. Selected
  source prepare/open outcomes remain authoritative on every platform.

## Linux capture module ownership correction — 2026-08-13

- Status: `SAFE-TO-MERGE`. The Linux platform module now names its concrete
  PipeWire/ALSA implementation `pipewire` instead of nesting a second `linux`
  module inside `capture::platform::linux`.
- The callback source-contract test follows the renamed implementation and
  continues to audit the same realtime callback source.
- Strict workspace Clippy passes with all targets, all features, the locked
  dependency graph, and warnings denied. No capture behavior, callback code,
  pool capacity, queue policy, permission semantics, or public API changed.

## Registry documentation gate alignment — 2026-08-13

- Status: `SAFE-TO-MERGE`. CI and release validation now execute the exact
  documentation configuration declared for docs.rs: native Linux,
  `default-features = false`, and no dependency documentation.
- The stale Ubuntu-to-Windows MSVC documentation command was invalid because
  the non-Rust Opus build requires a native Windows toolchain. Windows runtime
  qualification remains an independent native-platform gate and is not
  reclassified by this documentation correction.
- No Rust source, public API, ABI, capture path, codec behavior, or realtime
  execution contract changed.

## W20 public documentation narrative correction — 2026-08-14

- Status: `SAFE-TO-MERGE` for documentation candidate
  `pks-20260814-w20-public-docs-20`; Core 1.0 remains frozen.
- The public README now leads with the implemented developer workflow: capture
  one desktop application and microphone once, preserve independent
  source-aware stems, and fan them out concurrently to Operators, application
  callbacks, remote delivery, and aligned multistem recording.
- Public documentation now explains the stronger architectural boundary as a
  provenance-preserving bounded execution contract across realtime audio,
  typed signals, Rust, C, and process sidecars. Established realtime
  primitives remain engineering guarantees rather than novelty claims.
- The compatibility guide no longer assigns PocketStation the label
  "innovative." It requires concrete workflow, contract, and evidence language
  and preserves the separate burden for novelty or superiority claims.
- Contracts-only rustdoc with warnings denied, the release public quickstart,
  the complete `CODE_PROTOCOL` gate, and the Core freeze-policy gate pass. No
  Rust source, API, ABI, PKSS protocol, callback, pool, queue, runtime behavior,
  scaffold, mock, fallback, provider, or evidence classification changed.

## PocketStation 1.0.3 documentation patch release — 2026-08-14

- Status: `BLOCKED` for release candidate
  `pks-20260814-w20-docs-release-21`. The immutable source tag and GitHub
  Release exist, but publication stopped before crates.io because the clean
  runner's `CODE_PROTOCOL` gate unconditionally required a sibling `pks`
  checkout. Workflow `31850004592` preserves the failed gate.
- The package version and public install snippets advance from `1.0.2` to
  `1.0.3`, and the accepted W20 public narrative is included in the registry
  archive and crate-level docs.
- Release notes now live in one canonical 1.x-focused `RELEASE_NOTES.md` with a
  concise compatible-line contract and release history. Immutable tags and
  GitHub Releases preserve exact earlier bytes. The numbered top-level
  patch-note files are removed from the current source and package archive.
- This patch changes only documentation and release metadata. It does not
  change dependencies, feature defaults, Rust API, C ABI, PKSS protocol,
  capture, callback, pool, queue, codec, timing, graph, Session runtime, or
  evidence classifications.

## PocketStation 1.0.4 documentation patch release — 2026-08-14

- Status: `BLOCKED` for replacement candidate
  `pks-20260814-w20-docs-release-22`. The clean workflow passed the corrected
  protocol and package dry-run, then stopped before crates.io because it still
  referenced a recovery-test script removed during single-package
  consolidation. Workflow `31851000841` preserves the failed gate.
- The deterministic failure is reduced to the standalone Core checkout. The
  cross-repository `pks` ownership assertion now reports `NOT OBSERVABLE` when
  its sibling checkout is absent and remains fully enforced when the sibling
  is present. This changes release qualification only, not product runtime.
- The package version and public install snippets advance to `1.0.4`. The
  canonical 1.x `RELEASE_NOTES.md` records both the failed 1.0.3 publication
  attempt and this correction without recreating numbered patch-note files.

## PocketStation 1.0.5 documentation patch release — 2026-08-14

- Status: `SAFE-TO-TEST` for replacement candidate
  `pks-20260814-w20-docs-release-23`; publication and external consumption are
  not claimed until the immutable registry, docs.rs, and clean-consumer gates
  pass.
- `scripts/publish.sh` now checks the exact package version on crates.io before
  an actual upload. A visible version completes idempotently, a missing version
  publishes once, and an unavailable or unexpected registry response fails
  closed without attempting publication.
- `scripts/test-publish-recovery.sh` deterministically covers all four cases
  for the consolidated single package. No product runtime or frozen contract
  changes.

## Canonical 1.0.0 registry reset — 2026-08-16

- Status: `SAFE-TO-MERGE`. The owner deleted the complete `pocketstation`
  package from crates.io; the registry subsequently returned `404` for the
  package name. The source package, lockfile, and public install snippets are
  reset to the canonical `1.0.0` version.
- `RELEASE_NOTES.md` now describes the compatible 1.x product and contract
  globally instead of presenting documentation-only publication attempts as
  product releases. Historical evidence above remains unchanged.
- `cargo package --allow-dirty --locked` verified the generated
  `pocketstation-1.0.0.crate`, and
  `cargo build --release --example product_quickstart --locked` passed.
- The immediate crates.io upload was rejected by the registry's deleted-name
  reuse hold. No replacement package was published by this step. No runtime,
  Rust API, C ABI, PKSS, capture, hot-path, scaffold, mock, or evidence
  classification changed.

## W21 SDK-neutral native extension library host — 2026-08-16

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260816-w21-core-native-extension-host-bridge-1`. The focused dynamic
  library tests, full all-feature/all-target suite, strict Clippy, formatting,
  no-default-features check, release quickstart, C ABI compatibility gate,
  CODE_PROTOCOL gate, and executor document/state validators pass.
- The prior Python blocker was narrowed after a complete engine audit: Core
  already executes C source/operator/endpoint callbacks through the canonical
  Session. The missing primitive was a versioned packaged-library entrypoint,
  absolute-path loader, atomic import into the public Rust `Session`, and a
  retained executable-code lifetime.
- Extension ABI 1.2 adds `pks_extension_library_v1` without introducing a new
  descriptor, callback model, registry, compiler, runtime, graph, or lifecycle
  owner. Extension ABI 1.1 layouts, offsets, and Core symbols remain the
  required compatibility baseline subset.
- A separately compiled dynamic-library fixture passes six focused tests:
  canonical source → operator → endpoint execution, exactly-once instance and
  registration destruction, relative-path rejection, missing entrypoint,
  unsupported ABI, malformed acquired-registration cleanup, and transactional
  duplicate import.
- Foreign callbacks remain on blocking/async/external partitions. No callback,
  allocation, dynamic loading, locking, or managed execution was added to the
  capture or realtime PCM path. The plugin is a conformance fixture only; this
  step does not claim remote, physical-device, language-SDK parity, or release
  readiness.

## W21 provider-neutral connector authoring contract — 2026-08-17

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260817-w21-connector-authoring-contract-1`; every mandatory executor
  predicate is bound to the task acceptance manifest.
- `pocketstation::connector` now provides an inspectable manifest, typed and
  finite configuration, redacted secrets, bounded delivery/retry/readiness
  policies, explicit readiness transitions, stable classified errors,
  observations, Session-scoped registration, and conformance fixtures.
- The new registration path lowers into the existing `NodeDefinition` and
  `EndpointDriverFactory` authorities. It adds no registry, compiler, route
  table, scheduler, worker lifecycle, provider type, or execution engine.
- Ten connector-contract tests pass through the public Session path, including
  configuration rejection, secret redaction, duplicate identity, preparation
  rollback, start failure, saturation accounting, cancellation, join failure,
  and worker-panic containment. The existing public Session fan-out test also
  passes after migrating its observed connector to the new contract.
- Full acceptance passes: 444 library tests plus every workspace target and
  benchmark target, strict all-target/all-feature Clippy, rustdoc warnings as
  errors, release quickstart, architecture constraints, `CODE_PROTOCOL`, Core
  freeze policy, and both executor validators.
- Concrete protocols remain outside Core. Rust packages implement the factory;
  managed SDKs consume packaged native connectors or supported bounded
  extension/sidecar boundaries. The current native extension ABI remains
  typed-signal-only, so this step does not claim arbitrary pure-Python or
  pure-JavaScript PCM connector authoring.

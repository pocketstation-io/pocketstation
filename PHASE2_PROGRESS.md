# Phase 2 Progress - PocketStation Runtime

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
- `pks-session-c` is the sibling stable C projection. It owns ABI records,
  generational handles, marshalling, polling, leases, panic containment,
  reproducible headers, and C conformance. It does not own Session semantics.
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
- The existing `pks-audio` Session still returns typed
  `RuntimeNotIntegrated`; its `PARTIAL` scaffold inventory row remains live
  until real execution migrates and the compatibility façade delegates to the
  canonical engine.
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

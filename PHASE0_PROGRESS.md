# Phase 0 Progress — audio-core

## Graph-crate rescue — Wave 6 (DONE 2026-06-28)

**Branch:** wave6/runtime-execution
**Scope:** the `RuntimePlan` becomes executable, and `pocketstation-ml` migrates off the
legacy slice trait onto `RuntimeNode` — burning the `legacy` module entirely.

### What was built
- **`pocketstation-runtime`** (new crate): `EdgeChannel`/`EdgeSender`/`EdgeReceiver` (bounded
  SPSC over rtrb, drop-newest, alloc/lock/block-free), `RealtimeExecutor` (threads a frame
  through realtime nodes in topo order, short-circuits on gating, LAW-15 hot path),
  `RunMetrics`, `ExecError`, `PlanScheduler::build_realtime_executor` (instantiates nodes
  from the registry per the plan's realtime partition). Proven: a real
  passthrough→gain→passthrough graph applies gain through the node path.
- **`pocketstation-ml` migration**: VAD / NoiseSuppressor / EchoCanceller / AudioWatermark
  now implement `RuntimeNode` (prepare/process/flush/close). DSP cores preserved byte-for-byte
  as `process_slices`; the frame path copies into a pre-allocated scratch via `mem::take`
  (O(1) swap, no hot-path alloc) and writes back into the frame buffer. New integration test
  proves the RuntimeNode path equals the slice core for every node.
- **legacy module BURNED**: `graph::legacy::{GraphProcessor, FRAME_LEN_SAMPLES}` deleted; ml
  uses its own `FRAME_SAMPLES_48K_20MS`. The Wave-3 deferred-deletion debt is now retired.
- **facade**: `pocketstation-audio` re-exports the runtime executor (targeted, not glob).

### Verification
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings      PASS (0 warnings)
cargo test --workspace                                     PASS (181 tests, was 165; +16)
cargo run --example holy_shit_demo                          PASS (11 nodes, 15 edges)
cargo bench --no-run -p pocketstation-audio                PASS
```
ml's 17 DSP tests preserved unchanged (correctness held across the migration).

### Deviations (documented, not silent)
- `pocketstation-runtime` is a **new crate**, not a rename of `pipeline`. Rationale: `pipeline`
  still holds nodes (Gain/Resample/MonoMix) that belong in `pocketstation-nodes` (Wave 7);
  renaming now then re-splitting later is churn. `pipeline` folds in during Wave 7.
- Async/model/network partition execution is **out of scope** this wave (documented in the
  executor): only realtime partitions run; bounded-channel wiring for async boundaries is Wave 7.
- `ExecError` is hand-rolled `Display`/`Error` (runtime Cargo.toml has no thiserror dep yet).

### Staff Bar Self-Check — Wave 6
- Real execution proven end-to-end (build→compile→plan→run frames, gain applied on real path).
- DSP correctness preserved (byte-identical cores; 17 tests unchanged + 4 new wrapper tests).
- Hot path alloc-free (scratch via mem::take; SPSC ring; assert_no_alloc gate still green).
- New scaffold: none. legacy debt retired. Phase scope: Phase 0 correction.

## Graph-crate rescue — Wave 5 (DONE 2026-06-27)

**Branch:** wave5/lowering-runtimeplan
**Scope:** lower a validated `GraphIr` into a `RuntimePlan` (partitions, memory, edge metrics,
fan-in/out). Two passes deliberately deferred with an ADR — not stubbed.

### What was built (in `pocketstation-graph`)
- `plan.rs` — `RuntimePlan` (node_order, partitions, memory_plan, edge_metrics, fan_out,
  fan_in, edge_count), `ExecutionPartition`, `MemoryPlan`/`EdgeBufferPlan` (with `total_bytes`),
  `EdgeMetricId`, `FanOutGroup`/`FanInGroup`, `PlanError`. Constants `FRAME_BYTES_MONO_48K`,
  `EDGE_RING_CAPACITY_FRAMES`.
- `planner.rs` — `RuntimePlanner::plan(ir)` runs the lowering pipeline:
  1. `lower_fan_out` — outputs feeding multiple edges → `FanOutGroup`.
  2. `lower_fan_in_mix` — multiple edges into one input → `FanInGroup`; `FanInOnSinglePort`
     error if the port multiplicity is `One`.
  3. `partition_execution_domains` — bucket nodes by `ExecutionClass`, ordered by `rank()`.
     Realtime never shares a partition with Network/ModelRemote (the rescue plan's core rule).
  4. `plan_memory` — per-edge `EdgeBufferPlan`; `realtime_pool_bytes` over realtime consumers.
  5. `instrument_edges` — stable `EdgeMetricId` per edge.
  6. emit `RuntimePlan`.
- `docs/adr/AUDIO-026` — records the two deferrals: `InsertAdapterNodes` (needs Wave 7 nodes)
  and fusion (post-functional optimization). Rejected alternatives: stub passes, placeholder adapters.

### Verification
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings      PASS (0 warnings)
cargo test --workspace                                     PASS (165 tests, was 156; +9)
cargo run --example holy_shit_demo                          PASS (11 nodes, 15 edges)
cargo bench --no-run -p pocketstation-audio                PASS
```
Tests include a golden RuntimePlan snapshot + a proptest (random realtime chain → one partition).

### Staff Bar Self-Check — Wave 5
- Real lowering, no stub passes: deferred passes are documented in AUDIO-026, not faked.
- The compile pipeline now goes Spec → validated IR → RuntimePlan (partitions + memory + metrics).
- New scaffold: none. New dependency: none.
- Phase scope: Phase 0 correction.

## Graph-crate rescue — Wave 4 (DONE 2026-06-27)

**Branch:** wave4/graph-ir-verify
**Scope:** typed `GraphIr` + the compiler verification-pass pipeline. The inert `GraphSpec`
now compiles into a validated IR that rejects bad topologies with specific `CompileError`s.

### What was built (in `pocketstation-graph`)
- `ir.rs` — `ResolvedNode` (spec + descriptor), `ResolvedEdge` (spec + negotiated media +
  contract), `GraphIr` (nodes/edges/topo_order with accessors).
- `compiler.rs` — `CompileError` (8 specific variants), `CompileContext`, `GraphPass` trait,
  and the ordered pass pipeline:
  1. `ValidateNodeIdsPass` — edges reference existing nodes.
  2. `ValidatePortsPass` — ports exist with correct direction.
  3. `NegotiateCapsPass` — media compatibility via `caps`; `MediaMismatch` on incompatible.
  4. *(deferred: InsertAdapterNodes — Wave 5 lowering; comment marker, no stub)*
  5. `ValidateRealtimeBoundariesPass` — async→realtime edges must be non-blocking
     (`InvalidRealtimeEdge` otherwise) — the rescue plan's realtime-boundary rule.
  6. `CycleDetectionPass` — deterministic Kahn BFS → `topo_order`; `CycleDetected` on back-edge.
  `Compiler::compile(spec, registry)` resolves factories (`UnknownNodeType`/`InvalidConfig`),
  then runs the passes.

### Verification
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings      PASS (0 warnings)
cargo test --workspace                                     PASS (156 tests, was 145; +11)
cargo run --example holy_shit_demo                          PASS (11 nodes, 15 edges)
cargo bench --no-run -p pocketstation-audio                PASS
```
Tests include a golden IR snapshot (topo order asserted inline) and a proptest (random
linear chain always compiles to insertion order).

### Staff Bar Self-Check — Wave 4
- Real validation, no stub passes: every pass does its full job with GWT tests, or is absent
  (InsertAdapterNodes deferred to Wave 5 with a comment marker — not a fake `Ok(())`).
- Correctness-first ordering per the rescue plan (validation before lowering/fusion).
- New scaffold: none. New dependency: `proptest` added as a `[dev-dependencies]` to the graph
  crate (test-only, workspace-inherited; matches pocketstation-caps).
- Phase scope: Phase 0 correction.

## Graph-crate rescue — Wave 3 (DONE 2026-06-27)

**Branch:** wave3/dsl-graphspec
**Scope:** the builder stops executing — it now assembles a typed `GraphSpec`. Deleted the
fake provider factories and the executing graph; preserved node specs (no more discarded `_spec`).

### What changed (in `pocketstation-graph`)
- **New** `spec.rs` — `GraphSpec`, `NodeSpec` (id + `NodeTypeId` + `NodeConfig`, preserved
  verbatim), `EdgeSpec` (with optional requested `EdgeContract`), `OutputPortRef`/`InputPortRef`,
  `NodeId`/`EdgeId`.
- **New** `dsl.rs` — `AudioGraph` builder: `add_node(type_id, config)`, `connect`/`connect_with`,
  `into_spec`/`spec`. Produces a `GraphSpec`; never executes.
- **New** `legacy.rs` — `GraphProcessor` + `FRAME_LEN_SAMPLES` retained ONLY for
  `pocketstation-ml` (re-exported at crate root) until ml migrates to `RuntimeNode` (Wave 6/7).
- **Deleted** — `deepgram()`/`openai_realtime()`/`local_model()`/`ModelProvider`; the closed
  node enums (`SourceNode`/`TransformNode`/`ModelNode`/`PolicyNode`/`TransportNode`/`SinkNode`);
  the executing `AudioGraph::compile()`/`run()`/`output()`, `GraphPlan`, `GraphError`,
  `ConnectionSpec`, `NodeSelector`; and the old LAW-10 banner comments.
- **`holy_shit_demo`** — rewritten to build the fundable-graph `GraphSpec` (11 nodes, 15 edges)
  and print it; no execution (returns in Waves 4–6). Fan-in now = multiple edges into one port.

### Verification
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings      PASS (0 warnings)
cargo test --workspace                                     PASS (145 tests; old fake compile/run
                                                                  tests deleted, +5 spec/dsl tests)
cargo run --example holy_shit_demo                          PASS (GraphSpec built — 11 nodes, 15 edges)
cargo bench --no-run -p pocketstation-audio                PASS
```

### Staff Bar Self-Check — Wave 3
- The audit's worst offenders are gone: no discarded `_spec`, no fake `deepgram()`/`openai_realtime()`,
  no universal-execution `AudioGraph::run()`, no string-enum nodes. Builder → spec only.
- `pocketstation-ml` kept green untouched via the legacy re-export (verified: ml builds + tests pass).
- Deferred deletion (recorded, not fake): `legacy::GraphProcessor` + `FRAME_LEN_SAMPLES` removed when
  ml is rewired to `RuntimeNode`. It is a real, in-use trait — not a scaffold — so no inventory row.
- New scaffold: none. New dependency: none.
- Phase scope: Phase 0 correction (no Phase N+1 code).

## Graph-crate rescue — Wave 2 (DONE 2026-06-27)

**Branch:** wave2/node-model-registry
**Scope:** node declaration model + registry + `RuntimeNode` lifecycle (the compiler-facing
and execution-contract layer). Additive new modules in `pocketstation-graph`; the legacy
`AudioGraph`/`GraphProcessor` stay untouched (frozen prototype, replaced in Wave 3).

### What was built (all in `pocketstation-graph`)
- `node.rs` — `NodeTypeId`, `NodeKind` (+`is_terminal`), `ExecutionClass` (8 variants,
  `is_realtime`/`rank`), `NodeConfig` (builder + typed getters), `ConfigError`/`NodeError`
  (thiserror), `NodeDescriptor` (inputs/outputs as `caps::PortSpec`), `PrepareContext`.
- `runtime_node.rs` — `RuntimeNode` lifecycle trait: `prepare`/`process(frame)->Result<Option<frame>>`/
  `flush`/`close`; mirrors the established `AudioProcessorNode` frame-ownership idiom + LAW-15
  realtime invariant doc. Lives in graph (dependency-correct: runtime will depend on graph).
- `registry.rs` — `NodeFactory` trait (descriptor/validate_config/instantiate) + `NodeRegistry`.
- `builtins.rs` — real `PassthroughFactory`/`GainFactory` producing real `RuntimeNode`s
  (gain is alloc-free in-place dB→linear sample scaling); `register_builtins()`.

### Verification
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings      PASS (0 warnings)
cargo test --workspace                                     PASS (147 tests, was 130)
cargo bench --no-run -p pocketstation-audio                PASS (compiles)
```

### Staff Bar Self-Check — Wave 2
- Smallest correct design: yes — additive modules; legacy API untouched; no scheduler pulled forward.
- Correctness-first: declaration + validation + single-node lifecycle delivered; multi-node
  scheduling/execution deferred to Wave 6 (runtime) where contexts become real — no stub contexts.
- Hot-path safe: `GainNode::process` scales in place, no alloc (LAW 15); realtime invariant documented.
- Registry enables third-party nodes (no enum editing) — the core rescue requirement.
- New scaffold: none. New dependency: none (thiserror is a workspace dep; caps/frame are path deps).
- Phase scope: Phase 0 correction (no Phase N+1 code).

## Graph-crate rescue — Wave 1 (DONE 2026-06-27)

**Branch:** wave1/caps-and-frame
**Scope:** typed media contracts + frame extensions (foundation for the DSL→IR→runtime rewrite).
See `docs/GRAPH_RESCUE_EXECUTION_PLAN.md` for the governing resolution and full wave list.

### What was built
- **`pocketstation-caps`** (new crate) — `MediaKind`, `ChannelLayout`, `AudioCaps`, `MediaCaps`
  (with `is_compatible_with`/`negotiate`), `PortSpec`, `ClockDomain`, `BackpressurePolicy`,
  `DeliverySemantics`, `CopyPolicy`, `LossPolicy`, `EdgeObservabilityLevel`, `EdgeContract`
  (`voice_default`/`model_default` per CODE_PROTOCOL §C-6). 14 GWT tests incl. proptest
  (compat reflexive + symmetric).
- **`pocketstation-frame`** — added `SampleSpec`, `EncodedCodec`, `EncodedFrame`, `EventPayload`,
  `EventFrame`; migrated the 5 existing tests to GWT names (bodies unchanged); +7 new GWT tests.
- **`pocketstation-audio`** — explicit re-export disambiguates the canonical `frame::EncodedFrame`
  from the leaner `codec::EncodedFrame` (no consumer used the facade name; behavior-preserving).
- **`scripts/check_protocol.sh`** — added LAW-13 forbidden-vocabulary check (room/listener/track).
- **`prototype/graph-engine-frozen`** tag — froze the demo-grade graph engine before the rewrite.

### Verification
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings      PASS (0 warnings)
cargo test --workspace                                     PASS (130 tests, was 109)
check_protocol.sh (run inside caps + frame)                PASS (LAW 1/10/13/15/16/18)
```

### Staff Bar Self-Check — Wave 1
- Smallest correct design: yes — pure additive type layer + one disambiguating re-export.
- Hot-path safe: yes — `EncodedFrame`/`EventFrame` heap fields documented as off-callback (LAW 15).
- Public API change: additive only; `pocketstation_audio::EncodedFrame` now = `frame::EncodedFrame` (no consumer used it).
- New scaffold: none — both crates fully real; no FAKE_SCAFFOLD row needed.
- New dependency: none (caps reuses workspace `proptest` dev-dep).
- Phase scope: Phase 0 correction (no Phase N+1 code). Pre-existing non-GWT tests in 4 untouched
  crates recorded in the execution plan for a dedicated, deliberate compliance-migration wave.

## Phase 0 COMPLETE — 2026-06-27 (v3.0 exit gate)

**Status: COMPLETE**

90 tests pass across 10 crates. `holy_shit_demo` compiles and runs. Clippy clean. fmt clean.
Exit criteria met: `pocketstation-graph` crate created; `AudioGraph::new().connect().compile().run()` API compiles.

## Task 0.A — pocketstation-graph crate (DONE 2026-06-27)

### What was built
- `crates/pocketstation-graph/src/lib.rs` — public AudioGraph API: NodeHandle, OutputPortRef,
  InputPortRef, ConnectionSpec (fan-in via const generics), SourceNode, TransformNode, ModelNode,
  PolicyNode (Duck/Gate/Failover), TransportNode, SinkNode, GraphError, GraphPlan.
- `crates/pocketstation-audio/examples/holy_shit_demo.rs` — Phase 0 exit gate example.
- `Cargo.toml` workspace — `pocketstation-graph` added as member.

### Demo lines enabled
- `AudioGraph::new()` through `graph.run(plan)` — the full BUILD_GUIDE_NORTH_STAR.md Rust example compiles.

### Verification
```
cargo fmt --all -- --check                                     PASS
cargo clippy --workspace --all-targets -- -D warnings          PASS (0 warnings)
cargo test --workspace                                          PASS (90 tests, 0 failures)
cargo run -p pocketstation-audio --example holy_shit_demo      PASS (11 nodes, 15 edges)
```

### Staff Bar Self-Check — Task 0.A
- Smallest correct design: yes — public API types + scaffold impls; no over-engineering
- Tests added: yes — 5 given_when_then tests in pocketstation-graph
- Hot-path safe: yes — graph is construction only; no callback path in this crate
- Public API changed: yes — new crate (additive only)
- New dependency: no external deps; pocketstation-graph depends on std only
- Phase scope respected: yes — Phase 0 only; no Phase 1+ runtime code
- Unsafe added: no
- FAKE_SCAFFOLD_INVENTORY updated: yes — S-graph-run-01, S-graph-compile-01 added

### Remaining Phase 0 gaps (unchanged from 2026-05-20)
- `AudioGraph::run()` is a stub — Phase 1 wires real scheduling
- `AudioGraph::compile()` accepts any topology — Phase 1 adds cycle detection
- No async runtime wired — Phase 1 task

Remaining items carried to Phase 2:

- Opus real bindings (libopus-sys) — burn the Opus MOCK
- ClockSync PI controller per ADR-006
- DHAT CI integration (zero per-frame heap allocation gate)
- First crates.io publish of `pocketstation-audio` v0.1.0

Next phase: Phase 2 — sdk-ios + crates.io publish.
See ADR-014 for the Phase 1 integration state and Phase 2 gate.

---

## Task 0 — Fix CI blocker (DONE 2026-05-20)

### Changes
- `pocketstation-frame`: removed dead `ChannelLayout { Mono, Stereo }` (glob clash with `pocketstation-graph::ChannelLayout`)
- `pocketstation-frame`: added `Debug` for `AudioBufferHandle` and `AudioFrame`
- `pocketstation-frame`: added `is_empty()` to `AudioBufferHandle` (clippy `len_without_is_empty`)
- `pocketstation-frame`: suppressed `clippy::mut_from_ref` on `slot_mut` (SAFETY comment explains invariant)
- `pocketstation-bus`: fixed `PushError::Full(frame)` pattern match replacing non-existent `.into_inner()`

### Verification
- `cargo fmt --all -- --check` — clean
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo test --workspace` — 5/5 tests pass

### Staff Bar Self-Check — Task 0
- Smallest correct design: yes — removed dead code, fixed incorrect API usage
- Tests added or updated: yes — existing tests confirmed green; clippy clean
- Hot-path safe: yes — changes were removals and type fixes only
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none

---

## Task 1 — Fix sine_to_wav example (DONE 2026-05-20)

### Changes
- `pocketstation-audio/examples/sine_to_wav.rs`: sized ring to 64 (matches pool) so all 50 frames fit; replaced panicking `.expect("ring full")` with `let _ =` (backpressure drop policy intentional)
- `pocketstation-audio/Cargo.toml`: added `[[example]]` entry documenting the run command

### Verification
- `cargo run -p pocketstation-audio --example sine_to_wav` — exits cleanly, writes 48000 samples

### Staff Bar Self-Check — Task 1
- Smallest correct design: yes — sized ring to match pool capacity; no logic added
- Tests added or updated: not applicable (example binary, not a library function)
- Hot-path safe: yes — no hot-path changes
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none

---

## Task 2 — Harden AudioBufferPool release/drop (DONE 2026-05-20)

### Changes
- `release()`: check free_mask BEFORE fetch_or in debug mode so double-release assertion fires before any state mutation
- Added tests: acquire all 64, 65th returns None; drop releases slot; is_in_use tracks state

### Verification
- `cargo test -p pocketstation-frame` — 5/5 pass

### Staff Bar Self-Check — Task 2
- Smallest correct design: yes — one-line reordering in debug path; no production-build change
- Tests added or updated: yes — 3 new tests covering pool exhaustion, slot reuse, is_in_use
- Hot-path safe: yes — `release()` is wait-free, allocation-free; debug check is `#[cfg(debug_assertions)]`
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no — existing SAFETY comment on `UnsafeCell` access unchanged
- Remaining risk: pool memory layout uses `Box<[UnsafeCell<Box<[f32]>>]>` (pointer-per-slot) vs architecture spec `Box<[f32]>` (contiguous). Non-blocking for Phase 0 correctness; Phase 1 should evaluate cache behaviour. Document in Phase 1 ADR.

---

## Task 3 — FrameBus/rtrb invariant tests (DONE 2026-05-20)

### Changes
- `pocketstation-bus`: added tests for bounded capacity, drop-newest policy, FIFO order, empty-ring pop

### Verification
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo test -p pocketstation-bus` — 5/5 pass

### Staff Bar Self-Check — Task 3
- Smallest correct design: yes — tests only; no production code changed
- Tests added or updated: yes — 4 invariant tests added
- Hot-path safe: not applicable (test code only)
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none

---

## Task 4 — BusMetrics (DONE 2026-05-20)

### Changes
- `pocketstation-metrics`: added `sum_ns()` accessor to `SimpleHistogram`
- Added 6 tests covering Counter, Gauge, SimpleHistogram, and BusMetrics

### Verification
- `cargo test -p pocketstation-metrics` — 6/6 pass
- `cargo clippy --workspace --all-targets -- -D warnings` — clean

### Staff Bar Self-Check — Task 4
- Smallest correct design: yes — one missing accessor plus tests; no structure added
- Tests added or updated: yes — 6 tests covering all primitive types and field independence
- Hot-path safe: yes — all atomics, Relaxed ordering; no allocation, no lock
- Public API changed: yes — `sum_ns()` added to `SimpleHistogram` (additive, non-breaking)
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none

---

## Task 5 — ClockSync Phase 0 (DONE 2026-05-20)

### Changes
- `pocketstation-bus`: made `ClockSync` fields private, exposed `correction_ratio()`, `drift_ppm_estimate()`, `target_sample_rate()` as the stable Phase 1 API surface
- Added 3 tests: zero drift, positive drift convergence, negative drift direction

### Verification
- `cargo test -p pocketstation-bus` — 8/8 pass
- `cargo clippy --workspace --all-targets -- -D warnings` — clean

### Staff Bar Self-Check — Task 5
- Smallest correct design: yes — exponential smoother placeholder; ADR-006 owns the full PI controller
- Tests added or updated: yes — 3 tests covering initialization, convergence, and sign direction
- Hot-path safe: yes — pure f32 arithmetic, no allocation, no lock
- Public API changed: yes — fields made private; 3 read-only getters added (non-breaking, additive)
- New dependency: no
- Phase scope respected: yes — explicitly marked as Phase 0 placeholder in doc comment
- Unsafe added: no
- Remaining risk: exponential smoother is not the ADR-006 dual-stage PI; Phase 1 must replace with tested PI before production use

---

## Task 6 — JitterBuffer Phase 0 scaffold (DONE 2026-05-20)

### Changes
- `pocketstation-codec`: added doc comment clearly marking JitterBuffer as NOT production NetEQ
- Added `sequence_gap_ahead()` as a PLC hook point (ADR-009 placeholder)
- Added 5 tests: depth gating, FIFO order, late frame (documents non-reorder), missing frame gap detection, contiguous no-gap

### Verification
- `cargo test -p pocketstation-codec` — 6/6 pass

### Staff Bar Self-Check — Task 6
- Smallest correct design: yes — FIFO queue with depth gate; explicitly not production NetEQ
- Tests added or updated: yes — 5 tests; late-frame non-reorder is documented as a known Phase 0 limitation
- Hot-path safe: not applicable — Phase 0 scaffold; no audio callback path wired
- Public API changed: yes — `JitterBuffer`, `push`, `pop_ready`, `sequence_gap_ahead`, `depth` added (new type)
- New dependency: no — uses `std::collections::VecDeque`
- Phase scope respected: yes — ADR-009 references in doc comments; full NetEQ explicitly deferred
- Unsafe added: no
- Remaining risk: `next_expected_seq` field initially added but drove no behavior — removed in compliance pass (see Task 11)

---

## Task 7 — Codec allocation story (DONE 2026-05-20)

### Changes
- `MockOpusEncoder`/`MockOpusDecoder` doc comments mark them explicitly as test/demo only
- Added `encode_into(&mut Vec<u8>)` and `decode_into(&mut Vec<f32>)` allocation-free APIs for hot-path use
- `encode()` and `decode_to_vec()` delegate to `_into` variants (tests/examples unchanged)
- TODO(Phase 1, ADR-008) references added at Vec allocation points
- `real-opus` feature already correctly gated via `dep:opus`; compiles without libopus installed

### Staff Bar Self-Check — Task 7
- Smallest correct design: yes — allocation-free `_into` variants added; convenience wrappers unchanged
- Tests added or updated: not applicable (mock encoder/decoder shape; behavior covered by integration test)
- Hot-path safe: yes — `encode_into`/`decode_slice_into` are allocation-free when called with pre-allocated buffers
- Public API changed: yes — 2 new public methods added on mock types (non-breaking, additive)
- New dependency: no
- Phase scope respected: yes — real-opus gated; mocks clearly marked
- Unsafe added: no
- Remaining risk: `EncodedFrame.payload: Vec<u8>` forces one heap allocation per frame in the convenience path; Phase 1 ADR-008 should evaluate pool-backed byte buffer

---

## Task 8 — Allocation-check command (DONE 2026-05-20)

### Changes
- `pocketstation-alloccheck/src/main.rs`: exercises acquire→encode_into→decode_slice_into→release cycle with reused caller-owned buffers; asserts 0 pool failures
- `pocketstation-codec`: added `decode_slice_into()` to avoid intermediate EncodedFrame allocation in the hot path

### Verification
- `cargo run -p pocketstation-alloccheck` — prints "Phase 0 OK", 0 pool failures
- DHAT gate documented as Phase 1 work (ADR-TBD)

### Staff Bar Self-Check — Task 8
- Smallest correct design: yes — harness exercises the hot-path API shape without claiming DHAT integration
- Tests added or updated: not applicable (binary tool, not library; behavior verified by running the binary)
- Hot-path safe: yes — harness itself is a test driver, not on the audio callback path
- Public API changed: yes — `decode_slice_into` added to `MockOpusDecoder` (non-breaking, additive)
- New dependency: no
- Phase scope respected: yes — DHAT integration explicitly deferred with ADR-TBD reference
- Unsafe added: no
- Remaining risk: no actual allocation counting; Phase 1 must add a `#[global_allocator]` shim or DHAT to prove zero per-frame allocation on the hot path

---

## Task 9 — Benchmark/soak placeholders (DONE 2026-05-20)

### Changes
- `pocketstation-audio/examples/soak.rs`: 60-second soak (3000 × 20 ms frames), asserts 0 drops, 0 pool failures, exact sample count
- `pocketstation-audio/Cargo.toml`: registered `[[example]] soak`
- Criterion next steps documented in soak.rs header (add `[[bench]]`, `benches/`, CI gate)

### Verification
- `cargo run -p pocketstation-audio --example soak` — 3000 frames in ~307 ms, PASS
- Criterion: NOT added (no existing `[[bench]]` config); Phase 1 steps documented

### Staff Bar Self-Check — Task 9
- Smallest correct design: yes — soak example only; no Criterion dependency added (one real use case not yet present)
- Tests added or updated: yes — soak example asserts correctness invariants for the full pipeline
- Hot-path safe: not applicable (soak is a test driver)
- Public API changed: no
- New dependency: no
- Phase scope respected: yes — Criterion gating and `benches/` directories deferred to Phase 1
- Unsafe added: no
- Remaining risk: soak is not a latency benchmark; Phase 1 needs Criterion benches to prove per-frame timing budget

---

## Task 10 — Final Phase 0 audit (DONE 2026-05-20)

### Checks run
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings     PASS (0 warnings)
cargo test --workspace                                     PASS (26 tests)
cargo run -p pocketstation-audio --example sine_to_wav    PASS (48000 samples)
```

### Staff Bar Self-Check — Task 10
- Smallest correct design: yes — audit only; no code changes in this task
- Tests added or updated: not applicable
- Hot-path safe: not applicable
- Public API changed: no
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: see Phase 1 gaps below

### Remaining Phase 1 gaps (not blockers for Phase 0 exit)

- **DHAT allocation gate**: `pocketstation-alloccheck` exercises the hot path but does not yet count heap allocations. Needs DHAT or a custom `#[global_allocator]` shim (ADR-TBD). Document the per-frame budget in `PocketStation-v2.3.md`.
- **Criterion benchmarks**: `criterion = "0.5"` is in workspace deps but no `[[bench]]` sections exist. Add `benches/` directories and CI gate in Phase 1.
- **ClockSync full PI controller**: ADR-006 owns the dual-stage anti-windup PI. Current stub is an exponential smoother only.
- **JitterBuffer adaptive depth + PLC**: ADR-009 owns the real adaptive NetEQ design. `sequence_gap_ahead()` is a hook only; no PLC is generated.
- **Codec real-opus feature**: `dep:opus` is gated correctly but requires `libopus` installed at link time. CI needs the `real-opus` feature tested in a separate job with libopus available.
- **EncodedFrame payload allocation**: `EncodedFrame.payload: Vec<u8>` means the struct itself forces one heap allocation. Phase 1 should consider a pool-backed byte buffer or a newtype over `Arc<[u8]>`.
- **Pool memory layout**: current `Box<[UnsafeCell<Box<[f32]>>]>` is pointer-per-slot; architecture spec shows `Box<[f32]>` contiguous. Non-blocking for Phase 0 correctness; Phase 1 should evaluate cache behaviour and decide via ADR.

---

## Task 11 — Staff Engineering Bar compliance pass (DONE 2026-05-20)

### Changes
- `docs/standards/STAFF_ENGINEERING_BAR.md` and `docs/standards/STRUCTURE_NAMING_STYLE_THINKING.md`: tracked in git (were present on disk but untracked)
- `AGENTS.md`: added Engineering Standards section cross-referencing docs/standards/
- `pocketstation-bus`: removed dead `BackpressurePolicy` enum (defined `pub`, never referenced); renamed GWT-prefixed tests to plain sentence style matching existing convention; added Given/When/Then comments to all tests
- `pocketstation-codec`: fixed TODO format to `TODO(Phase N, ADR-NNN)` across all comment sites; removed unused `next_expected_seq` field from `JitterBuffer` (computed in `push` but gap detection reads queue positions directly); renamed tests to sentence style; added GWT comments
- `pocketstation-frame`: added Given/When/Then comments to all 5 tests
- `pocketstation-metrics`: renamed tests to sentence style; added Given/When/Then comments

### Verification
```
cargo fmt --all -- --check                                 PASS
cargo clippy --workspace --all-targets -- -D warnings     PASS (0 warnings)
cargo test --workspace                                     PASS (26 tests)
```

### Staff Bar Self-Check — Task 11
- Smallest correct design: yes — removals, renames, and comment additions only; no behavior change
- Tests added or updated: yes — all tests updated with Given/When/Then structure; test names normalized
- Hot-path safe: yes — no production code behavior changed
- Public API changed: no — `BackpressurePolicy` removal is non-breaking (it was defined but not used by any consumer)
- New dependency: no
- Phase scope respected: yes
- Unsafe added: no
- Remaining risk: none — all clippy clean, all tests pass

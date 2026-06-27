# Graph Crate Rescue — Execution Plan (audio-core)

Source documents (binding, read in full before any work):
- `recap_v2.md` — the audit (why the current graph crate is a demo, not a runtime)
- `PocketStation_Graph_Crate_Rescue_Plan.md` — target architecture + 8-PR sequence
- `pocketstation_graph_target_skeleton.rs` — intended *shape* (not a contract)
- `docs/standards/CODE_PROTOCOL.md` — the contract; 21 laws. On any dispute, this wins.

## Governing resolution (how the docs reconcile with CODE_PROTOCOL)

1. **Correctness now / defer-with-ADR later.** Rewriting the fake Phase-0 graph crate
   into the correct typed compiler/runtime is *Phase-0 correction*, not Phase N+1
   (LAW 21 satisfied). Optimization / remote exec / all-OS providers / SFrame are
   deferred with an ADR + a `FAKE_SCAFFOLD_INVENTORY` row. Fusion is last.
2. **No half-passes.** A compiler pass is either fully implemented with
   `given_when_then` tests, or it is absent from the pipeline — never present-but-`Ok(())`.
   The skeleton's `Ok(())` bodies are targets for the author, not deliverables.
3. **Protocol beats skeleton.** The skeleton's shape is transcribed, but every line is
   rewritten to pass all 21 laws. The skeleton's universal `process(&[f32],&mut[f32])`
   is on the delete list; the canonical node interface is
   `AudioProcessorNode::process(frame) -> Option<frame>` (already in pipeline) plus the
   typed `RuntimeNode` lifecycle (prepare/process/flush/close).
4. **Green every wave.** No wave leaves the workspace broken or with a fake pass.

## End-state crate map

| Crate | Action | Holds |
|---|---|---|
| `pocketstation-frame` | extend | `EncodedFrame`, `EventFrame`, `SampleSpec` (+ existing pool/handle/AudioFrame) |
| `pocketstation-caps` | **new** | `MediaCaps`, `PortSpec`, `EdgeContract`, `ClockDomain`, `BackpressurePolicy`, … |
| `pocketstation-graph` | **rewrite** | DSL→`GraphSpec`, `NodeSpec`, `NodeRegistry`, `GraphIr`, compiler passes, emits `RuntimePlan` |
| `pocketstation-runtime` | rename `pipeline` | `Scheduler`, `Executor`, `EdgeChannel`, memory planner |
| `pocketstation-nodes` | **new** | built-in VAD/Gain/Duck/Mixer/StemRecorder/Passthrough |
| `pocketstation-ml` | rewire | local model nodes onto `RuntimeNode` lifecycle |
| `pocketstation-capture*` | extend | `SourceProvider`/`SourceDescriptor`/`SourceQuery` model |
| `pocketstation-transport` | **new** | relay client / local pipe / WebRTC / encoded-frame transport |
| `pocketstation-codec`/`-metrics`/`-audio` | keep/extend | metrics gain per-edge IDs; audio keeps facade + real demo |

## Waves (each green & validatable; one branch + PR per wave)

- **Wave 1** — freeze prototype tag · real per-crate gate · `pocketstation-caps` + frame extensions
- **Wave 2** — node model + registry + `RuntimeNode` lifecycle
- **Wave 3** — DSL→`GraphSpec` (builder stops executing; delete fake provider factories)
- **Wave 4** — typed `GraphIr` + verification passes 01–09
- **Wave 5** — lowering→emit passes 10–18 → `RuntimePlan` (fusion deferred)
- **Wave 6** — `pocketstation-runtime`: scheduler + executors + `EdgeChannel`; execute plan
- **Wave 7** — providers + transport + graph-relay wiring
- **Wave 8** — the fundable demo + per-edge observability

## Sync-test gate

Runnable now (every wave, fatal): `cargo fmt --all -- --check` · `cargo clippy --workspace
--all-targets -- -D warnings` · `cargo test --workspace` · proptest · `assert_no_alloc`
hot-path test. New code gates to 100% CODE_PROTOCOL compliance via `scripts/check_protocol.sh`
run inside each delivered crate.

Not installed (flagged, not run): `nextest`, `cargo deny`, `miri`, `llvm-cov`, `loom`,
`insta`. Golden IR snapshots use committed-file diffs (no `insta` dep) unless approved.

## Known pre-existing debt (recorded, NOT fixed drive-by)

The shipped `scripts/check_protocol.sh` scans `./src/` and is meant to run **per crate**;
at the workspace root those grep-laws no-op. ~39 pre-existing tests in `codec`, `metrics`,
`pipeline`, `capture` are not `given_when_then` named. LAW-1/LAW-10 crude greps yield
mostly false positives (e.g. `use std::f32::consts::PI`, dimensionless DSP coefficients).
A dedicated **protocol-compliance migration wave** (deliberate, project-wide per org policy)
will address test naming; it is out of scope for the feature waves above.

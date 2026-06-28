# AUDIO-026 — Graph compiler: adapter-insertion and fusion pass deferrals

**Status:** Accepted (2026-06-27)
**Context:** Graph-crate rescue, Wave 5 (lowering → RuntimePlan).

## Decision

The rescue plan's 18-pass compiler pipeline includes two passes that are deliberately
deferred rather than implemented in Wave 5. They are recorded here so their absence is a
documented decision, not an omission.

### 1. `InsertAdapterNodes` (pass 07) — deferred to Wave 7 (nodes crate)

Adapter insertion rewrites an edge whose endpoints have *convertible but unequal* media
caps (e.g. 48 kHz → 44.1 kHz, or mono → stereo) by splicing in a converter node
(resampler / channel-mixer). This requires real adapter **node types and factories**, which
live in `pocketstation-nodes` (Wave 7). Until those exist there is nothing to insert.

Consequence for Wave 5: the verification pipeline (Wave 4 `NegotiateCapsPass`) already
rejects incompatible edges with `MediaMismatch`, so every edge reaching the lowering stage
is already caps-compatible. The planner therefore assumes compatible caps and does not need
adapters yet. When Wave 7 lands the adapter factories, `InsertAdapterNodes` slots in
*before* `NegotiateCapsPass`'s final decision (it turns a would-be mismatch into a
compatible two-hop path).

### 2. `FusionEligibility` / `FuseLinearAudioKernels` (passes 13–14) — deferred (post-functional)

Fusion (collapsing a linear chain of realtime audio kernels into one fused kernel to remove
per-node call/copy overhead) is an optimization, not a correctness requirement. The rescue
plan is explicit: "Do not start with clever fusion. Start with correct IR and validation.
Fusion is a compiler pass, not a first-month feature." It is deferred until the runtime
executes real graphs and a benchmark demonstrates the per-node overhead worth removing.

## Alternatives rejected

- **Stub passes returning `Ok(())`** — rejected: a present-but-fake pass violates the
  rescue plan's "no half-passes" rule and would need a `FAKE_SCAFFOLD_INVENTORY` row for
  something that does nothing. A documented absence is honest; a fake pass is not.
- **Implementing adapters now with placeholder converter nodes** — rejected: placeholder
  converters that don't actually resample/remix would be fake work failing the production bar.

## Revisit when

- `InsertAdapterNodes`: when `pocketstation-nodes` ships resampler + channel-mixer factories (Wave 7).
- Fusion: when the Wave 6 runtime runs real graphs and a criterion benchmark shows
  measurable per-node dispatch overhead on a realtime chain.

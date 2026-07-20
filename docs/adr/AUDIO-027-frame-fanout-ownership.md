# AUDIO-027 — Frame fan-out ownership

**Status:** Accepted (2026-07-19)
**Scope:** `pks-frame`, `pks-caps`, `pks-graph`, and `pks-runtime`

## Context

The product captures each source once and sends the resulting stem to several
independent bounded destinations. The planner currently describes shared-buffer
edges, but `AudioBufferHandle` is exclusively mutable and releases its pool slot
on its first `Drop`. Copying that handle would permit mutation races and early
pool reuse; moving it permits only one branch.

The callback path must remain allocation-free, lock-free, blocking-free,
async-free, log-free, and panic-free.

## Decision

Use a two-state pooled slot:

```text
exclusive AudioBufferHandle
  mutable; one owner; shared reference count = 0
        │ freeze consumes exclusive ownership
        ▼
SharedAudioBufferHandle
  immutable; one preallocated per-slot atomic reference per live handle
```

`AudioFrame` remains the exclusive mutable frame used during capture and DSP.
`AudioFrame::freeze()` produces `SharedAudioFrame` at the fan-out boundary.
Read-only branches receive independent `SharedAudioFrame` handles. A mutating
branch must acquire a slot from its declared branch pool and use
`SharedAudioFrame::copy_to_pool()` before executing.

The edge copy policies are exact and open to compiler validation:

```text
MoveExclusive
ShareReadOnly
CopyToBranchPool
```

The runtime never infers policy from an operator, endpoint, or provider name.

## Slot lifecycle

Each pool slot has a free bit and an atomic shared-reference count.

| State | Free bit | Shared references | Allowed access |
|---|---:|---:|---|
| available | 1 | 0 | none |
| exclusive | 0 | 0 | one mutable handle |
| shared | 0 | 1..N | immutable handles only |

Freeze changes the reference count from zero to one while the free bit remains
clear. `try_clone()` uses a bounded compare/exchange loop and fails if the count
is zero or would overflow. Shared `Drop` decrements with compare/exchange; only
the final reference returns the slot to the free mask.

An impossible underflow or state mismatch leaks the affected slot instead of
making it available unsafely. Drop never allocates, locks, blocks, logs, or
panics. Pool exhaustion returns `None` and is counted by the existing acquire
failure counter.

## Branch drops and shutdown

- a full destination queue rejects and drops only its new shared handle;
- queued handles are released when their receiver/worker is dropped;
- shutdown drains when useful, then dropping the remaining bounded queue
  releases all references;
- no branch can retain mutable access after freeze;
- one slow branch may retain slots only up to its configured bounded capacity;
  it cannot grow memory or block another branch;
- an async, blocking, or external destination may not retain the capture pool:
  it receives a preallocated branch-pool copy so its bounded lifetime cannot
  exhaust capture slots.

## Planner contract

- use `MoveExclusive` only when the compiler proves one consumer and transfers
  the mutable frame;
- use `ShareReadOnly` only where immutable consumers share a proven bounded
  lifetime and the owning pool is sized for that lifetime;
- use `CopyToBranchPool` for any async/blocking/external partition crossing and
  any consumer requiring mutation after fan-out;
- memory planning includes branch-pool slots for every
  `CopyToBranchPool` edge;
- a fan-out plan containing `MoveExclusive` on more than one target is invalid.

W2 freezes these types and policy names. W3 is responsible for executing plan
edges and enforcing them in the scheduler.

## Alternatives rejected

- `Arc<Vec<f32>>` per frame: adds separately allocated payload ownership and
  does not preserve the fixed pool lifecycle.
- blindly implementing `Clone` on the mutable handle: permits mutation races and
  early release.
- copying for every same-lifetime realtime destination: safe but wastes
  bandwidth and pool memory where immutable references have a proven bounded
  lifetime. Independent worker/network/recorder lifetimes still require copies.
- one shared queue for all destinations: couples backpressure and violates branch
  isolation.
- reference saturation or panic on overflow/underflow: unsafe for the realtime
  path; failure must be explicit and non-panicking.

## Consequences

Same-lifetime immutable fan-out adds atomic reference operations but no payload
copy or heap allocation. Mutating and independent-lifetime branches pay one
explicit preallocated copy. Pool sizing accounts for each bounded destination
queue. W3 observations report queue depth and drops so retention is visible.

## W5 isolation amendment — 2026-07-19

The first five-cell product slice exposed that defaulting a realtime-to-async
edge to `ShareReadOnly` allowed a slow connector to retain both slots of a
two-slot capture pool. Capture then failed before the connector edge itself
reached its declared capacity. The planner now defaults every non-realtime
consumer to `CopyToBranchPool`. This preserves capture-once semantics while
moving destination retention into the destination's own preallocated memory
budget.

## Acceptance

Tests must prove single/N-consumer final release, rejection/drop release, queued
shutdown release, immutable branch isolation, final-reference pool reuse,
overflow/underflow safety, copy-to-branch-pool behavior, and protocol/lint
compliance in debug and release builds.

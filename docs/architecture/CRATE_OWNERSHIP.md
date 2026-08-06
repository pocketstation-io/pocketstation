# PocketStation module ownership contract

**Status:** Binding
**Superseded package topology:** AUDIO-033

PocketStation ships one Cargo package. This document retains its historical
filename so existing references do not break, but its contract is module
ownership—not micro-crate ownership.

## Governing rule

> Create a package only for an independently consumed, versioned, shipped, or
> toolchain-isolated deliverable. Use modules for internal ownership.

The only central package is `pocketstation`. The Rust developer surface is
`pocketstation::Session`; the native surface is `pocketstation.h` linked against
`libpocketstation`.

## Ownership map

| Module | Owns | Must not own |
|---|---|---|
| `frame` | buffers, source/stem/session IDs, formats, timestamps, sequences, discontinuities, lineage | provider or product meaning |
| `timing` | clock-domain estimation and correction | RTP pacing, receiver playout, generic date/time utilities |
| `graph` | `SignalSpec`, ports, open manifests, edge contracts, compiler, plan, partitions, safety contracts | worker lifecycle, capture, concrete providers |
| `runtime` | compiled-plan execution, bounded Bridges, routing, backpressure, drops, latency and operational observations | graph compilation, Session policy, provider meaning |
| `capture` | selection, permissions, stable source identity, source events, captured-frame streams, target-native backends | Session orchestration, endpoint finalization |
| `endpoint` | open endpoint IDs, registration, preparation, start/stop/join/finalization contracts | concrete provider catalogs, Session transaction policy |
| `recording` | staged multistem artifacts, aligned timeline mapping, WAV writing, checksums, outcomes, rollback | graph/runtime/session semantics |
| `codec` | encode/decode, codec validation, retained compatibility symbols | transport signaling or product workflow |
| `dsp` | bounded local audio operators | provider SDKs or model catalogs |
| `session` | public declaration, exact-source resolution, compilation coordination, transactional lifecycle, cancellation, bounded polling, events, observations, final outcomes | duplicate compiler/runtime/capture/recording implementations |
| `abi` | versioned C records, handles, marshalling, bounded polling, leases, panic containment | a second engine or separate product identity |

`runtime::metrics` is merely the representation of runtime observations. It is
not an independent metrics architecture, package, or public developer concept.

## Dependency direction

Rust visibility and imports enforce the logical direction:

```text
frame + timing
      ↓
graph → runtime
      ↓      ↓
capture + endpoint + recording + codec
                 ↓
               session
                 ↓
         public façade + abi
```

Cycles are blocked in review. A module may consume a lower contract but must
not reproduce that lower layer's algorithms or semantic truth.

## Open contracts

- `SignalSpec` is owned by `graph`; do not reintroduce `SignalType` elsewhere.
- Operators, endpoints, and connectors use open IDs/manifests, never closed
  provider/model enums.
- `ExecutionPartition` states where work runs; `SafetyContract` states what it
  guarantees. They remain distinct.
- Every realtime crossing is a bounded Bridge with explicit overflow and
  discontinuity observations.
- Capture owns source identity and native timestamps. Session orchestrates but
  does not fabricate them.

## Language and ABI boundary

Python, Node, Swift, Kotlin, and C project the same Session semantics. They do
not implement schedulers, capture engines, recording coordinators, or separate
error vocabularies. Foreign frame delivery is bounded and batched; no per-frame
Python/JavaScript/Swift/JNI callback is allowed on the capture hot path.

The C header is `include/pocketstation.h`. Retained `pks_*` symbols are a
time-bounded ABI compatibility measure. New symbols and documentation use the
PocketStation product identity without creating a `*-c` package.

## Hot path

Audio callbacks, realtime partitions, and hot-path destructors are:

```text
allocation-free · lock-free · blocking-free · async-free · log-free · panic-free
```

## Package exception gate

A proposal for another Cargo package must demonstrate at least one:

1. independently shipped binary/static/dynamic artifact;
2. independent third-party consumption and versioning;
3. a security or process isolation boundary;
4. native toolchain isolation impossible to express with target-specific Cargo
   dependencies;
5. measured build isolation that materially benefits users.

Names, conceptual ownership, test grouping, or historical precedent are not
sufficient.

## Enforcement

```bash
cargo metadata --no-deps --format-version 1
bash scripts/lint/check-architecture-constraints.sh
bash scripts/check_protocol.sh
```

Cargo metadata must report one workspace member and one package named
`pocketstation`. Provider names must not leak into core modules, and no live
manifest or dependency may restore a `pks-*`, `*-core`, or `*-c` package.

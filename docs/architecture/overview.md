# Current architecture

PocketStation is a developer-facing execution model for source-aware live
media, shipped as one Cargo package and one native library. The public center
is `Session`; internal module boundaries retain single semantic ownership.

```text
authorized application / microphone / external source
                         ↓
          stable source, stem, timing and lineage
                         ↓
                       Session
                         ↓
              compiled typed RuntimePlan
              ┌──────────┴──────────┐
              ↓                     ↓
     realtime AudioFrame lane   typed async lane
              └──────────┬──────────┘
                         ↓
               independent bounded routes
             ┌───────────┼───────────┐
             ↓           ↓           ↓
          recording   callback    endpoint/transport
```

The realtime audio lane stays specialized: pooled ownership, bounded rings,
nonblocking fan-out, explicit discontinuities, and no allocation, locks,
blocking, async, logging, or panic on callbacks. Typed sources, external
operators, managed-language sidecars, model work, and endpoint I/O execute
outside that callback lane.

`SignalSpec`, schemas, named ports, lineage/timing, and `EdgeContract` are the
dynamic and cross-language authority. Rust `Stream<T>` is declaration-time
type checking only and compiles into those stable contracts. No Rust `TypeId`
or generic type crosses the C ABI or sidecar protocol.

Core owns execution primitives. Providers, customer protocols, export formats,
application policy, and business logic are external extensions.

## Why the boundary matters

Application capture, media graphs, bounded queues, and native extension
mechanisms are established systems primitives. PocketStation's useful boundary
is their shared execution contract: a source does not lose its identity when
it enters an Operator, a process extension does not invent a separate lifecycle,
and one saturated destination does not silently redefine delivery for every
other branch.

The contract spans:

```text
capture + source lineage + Session compilation
        + bounded realtime and typed lanes
        + named composition and generated-audio reentry
        + endpoints, recording, observations, and final outcomes
        + Rust, C, and sidecar projections
```

This is the architectural claim. Novelty, universal platform parity, and
overall superiority are separate evidence questions and are not inferred from
the existence of the implementation.

# How a Session owns and routes audio

Use this concept page to decide which PocketStation API should own new
work. PocketStation ships as one Cargo package and one native library. The
public lifecycle begins at `Session`; internal modules keep one owner for each
execution concern.

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

`SignalSpec`, schemas, named ports, lineage, timing, and `EdgeContract` carry
runtime and cross-language meaning. Rust `Stream<T>` adds declaration-time type
checking and compiles into those stable contracts. No Rust `TypeId` or generic
type crosses the C ABI or sidecar protocol.

Core owns execution primitives. Providers, customer protocols, export formats,
application policy, and business logic are external extensions.

## Choose the API that owns the work

Application capture, media graphs, bounded queues, and native extension
mechanisms are established systems primitives. PocketStation's useful role
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
        + Rust, C, and sidecar APIs
```

Use the [Connector guide](../guides/connectors.md) for an outbound provider, or
the [extension guide](../guides/extensions.md) for another Source, Operator,
Endpoint, native library, or managed process. Platform and network behavior
remain limited to the environments named by their evidence.

# Current architecture

PocketStation is one Cargo package and one native library. The public center is
`Session`; internal module boundaries retain single semantic ownership.

```text
authorized application / microphone / external source
                         ↓
          stable source, stem, timing and lineage
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

# Extend PocketStation

An extension adds capability to the existing Session engine. It declares what
it consumes and produces, the execution partition it needs, its boundedness and
failure policy, and its lifecycle. It does not own a second compiler, runtime,
lineage model, or scheduler.

Choose the boundary that matches the work:

| Need | Contract | Execution rule |
|---|---|---|
| New source | `SourceFactory` / `SourceDriver` | Emit only validated manifest ports through bounded ingress |
| New computation | operator manifest/factory | Declare ports, schemas, partition, permissions, deadline, cancellation, and failure policy |
| New destination | endpoint driver/factory | Consume a bounded route and expose lifecycle/failure observations |
| Rust type safety | `Stream<T>` / `TypedOperator<I, O>` | Declaration-only; runtime identity remains `SignalSpec` |
| Native C extension | `pocketstation.h` extension ABI | Versioned callback tables with explicit context ownership and bounded worker execution |
| Managed process | `PKSS` sidecar protocol | Bounded framed IPC outside audio callbacks |

An extension must not require `internal-testing` or edits to central. It must
not build a second graph, scheduler, recorder, lineage system, or lifecycle.
Provider/model code, customer protocols, and business policy remain in the
extension package or sidecar.

Generated audio returns through the bounded generated-audio bridge, which
validates format and frame size, copies into the dedicated pool, assigns
authoritative timing/lineage, and nonblockingly enters the existing audio plan.
It never executes a managed-language or provider callback on the capture path.

## Language boundaries

Rust uses `Stream<T>` for local type checking. C and managed languages bind
their native wrappers to stable `SignalSpec` and schema identifiers. The
sidecar protocol transports those identities and bounded payloads across a
process boundary. Rust `TypeId` and generic parameters never cross either
boundary.

The installed-consumer conformance artifact proves these contracts from a
published package in a clean repository. Its `LOOPBACK-ONLY` classification is
specific and intentional: it proves packaging and execution, not a remote or
physical-device deployment.

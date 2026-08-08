# External extensions

Choose the boundary that matches the work:

| Need | Contract | Execution rule |
|---|---|---|
| New source | `SourceFactory` / `SourceDriver` | Emit only validated manifest ports through bounded ingress |
| New computation | operator manifest/factory | Declare ports, schemas, partition, permissions, deadline, cancellation, and failure policy |
| New destination | endpoint driver/factory | Consume a bounded route and expose lifecycle/failure observations |
| Rust type safety | `Stream<T>` / `TypedOperator<I, O>` | Declaration-only; runtime identity remains `SignalSpec` |
| Native extension metadata | `pocketstation.h` extension ABI | Versioned, size-checked, pointer-checked descriptors |
| Managed process | `PKSS` sidecar protocol | Bounded framed IPC outside audio callbacks |

An extension must not require `internal-testing` or edits to central. It must
not build a second graph, scheduler, recorder, lineage system, or lifecycle.
Provider/model code, customer protocols, and business policy remain in the
extension package or sidecar.

Generated audio returns through the bounded generated-audio bridge, which
validates format and frame size, copies into the dedicated pool, assigns
authoritative timing/lineage, and nonblockingly enters the existing audio plan.
It never executes a managed-language or provider callback on the capture path.

The W20 Lab artifact is the external installed-consumer conformance reference.
Its `LOOPBACK-ONLY` classification proves contract execution, not remote or
physical-device deployment.

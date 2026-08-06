# PocketStation

PocketStation is a local-first Rust capture primitive for turning one permitted
desktop application and one microphone into independent, source-aware live
stems that can be observed, fanned out, transported, and recorded.

```text
application + microphone
          ↓
independent timed stems
          ↓
application callback + example transport + multistem recording
```

The product ships as one Cargo package, `pocketstation`. Its internal engine,
capture backends, graph, runtime, recording, codec, timing, observations, and C
projection are modules of that package. The native deliverable is
`libpocketstation`; C consumers include `pocketstation.h`.

## Public Surface

The supported Rust entry point is `pocketstation::Session`. The narrow contract
selects an application and microphone, starts capture, receives each stem
independently, observes lifecycle/errors, stops, and receives recording
outcomes. Provider and transport integrations belong in examples or external
packages, not in the engine.

The `internal-testing` Cargo feature exposes implementation types only to
repository-owned conformance fixtures, the CLI, and the neutral benchmark. It
is not a supported application API.

## Internal Ownership

| Module | Responsibility |
|---|---|
| `session` | Public lifecycle, composition, cancellation, polling, and outcomes |
| `frame` | Buffers, source/stem identity, timestamps, sequence, and lineage |
| `timing` | Clock-domain estimation and correction |
| `graph` | Open signal/operator/endpoint contracts, compiler, and plan |
| `runtime` | Execution, bounded Bridges, fan-out, drops, and observations |
| `capture` | Capture contracts and macOS/Windows/Linux implementations |
| `endpoint` | Open destination lifecycle and registration |
| `recording` | Concrete aligned multistem recording |
| `codec` | Codec behavior and compatibility ABI implementation |
| `dsp` | Bounded local audio processing |
| `abi` | C projection of the same Session and codec implementation |

Metrics are operational observations owned by `runtime`; they are not a
separate product subsystem or crate.

## Artifact Names

- Rust: `pocketstation`
- C header: `pocketstation.h`
- Unix library: `libpocketstation.a`, `libpocketstation.dylib`, or
  `libpocketstation.so`
- Windows library: `pocketstation.dll` / `pocketstation.lib`
- Apple framework target: `PocketStation.framework`

Retained `pks_*` function symbols are temporary binary compatibility only.
There are no separately marketed `*-core` or `*-c` products.

## Development

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
cargo build --release --example product_quickstart --locked
bash scripts/check_protocol.sh
```

Passing component tests is not a real-device claim. Product acceptance still
requires the real-path permission, recovery, destination-failure, latency,
drop-rate, soak, integrity, and clean-checkout evidence defined by the factory
operating contract and execution registry.

# Rust quickstart

Applications depend on one package:

```toml
[dependencies]
pocketstation = "1.0.4"
```

The primary desktop path declares one application and one microphone,
routes each source-aware stem independently, starts one Session, observes both
stems, stops, and reads the recording outcome. The compiling reference is
[`examples/product_quickstart.rs`](../../examples/product_quickstart.rs).

Important boundaries:

- the host chooses selectors and handles operating-system consent UX;
- `PermissionObservation::NotObservable` is an unknown preflight state, never
  an implicit grant; prepare/open returns the authoritative result for the
  selected backend;
- every destination is bounded and may fail independently;
- source, stem, clock, sequence, permission epoch, and discontinuity identity
  come from captured frame lineage, not caller strings;
- a complete recording outcome is available only after `stop()` finalizes the
  recording workers;
- provider and transport implementations are examples or external packages.

Build the exact reference with:

```bash
cargo build --release --example product_quickstart --locked
```

## Native prerequisites

macOS requires the Xcode command-line tools. Windows requires the MSVC Rust
toolchain and Windows SDK. Debian/Ubuntu requires:

```bash
sudo apt install build-essential cmake pkg-config \
  libasound2-dev libpipewire-0.3-dev
```

The example compiling does not prove that a selected physical source exists or
that the current host granted permission. Those remain runtime facts surfaced
as typed outcomes.

# Rust quickstart

Applications depend on one package:

```toml
[dependencies]
pocketstation = "0.1.2"
```

The supported product path declares one application and one microphone,
routes each source-aware stem independently, starts one Session, observes both
stems, stops, and reads the recording outcome. The compiling reference is
[`examples/product_quickstart.rs`](../../examples/product_quickstart.rs).

Important boundaries:

- the host chooses selectors and handles operating-system consent UX;
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

The example compiling does not prove that a selected physical source exists or
that the current host granted permission. Those are runtime/evidence facts.

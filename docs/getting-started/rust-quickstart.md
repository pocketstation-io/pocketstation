# Capture an application and microphone in Rust

This quickstart runs one `Session`, observes a desktop application and the
default microphone as separate stems, and finalizes a two-stem recording.

## Prerequisites

- Rust 1.95 or newer;
- a desktop application named `PocketStation Demo` running on the host;
- an available default microphone;
- operating-system permission to capture the application and microphone.

Native build requirements:

- macOS: Xcode command-line tools;
- Windows: MSVC Rust toolchain and Windows SDK;
- Debian or Ubuntu:

  ```bash
  sudo apt install build-essential cmake pkg-config \
    libasound2-dev libpipewire-0.3-dev
  ```

## Add PocketStation

```toml
[dependencies]
pocketstation = "1.1.1"
```

The complete program is
[`examples/product_quickstart.rs`](../../examples/product_quickstart.rs).

## Build and run

From this repository:

```bash
cargo build --release --example product_quickstart --locked
cargo run --release --example product_quickstart --locked
```

The Session stops after it observes at least two frames from both stems. It
then requires a successful Session outcome and a completed recording with two
stems.

## Inspect the result

Completed artifacts are written under `pocketstation-recordings/`. The
application and microphone remain separately identified by their source and
stem lineage.

If setup fails, inspect the typed permission or source error. A preflight
`PermissionObservation::NotObservable` is not permission approval; source
opening determines whether the selected backend can run. If only one source
produces media, the example exits without claiming a complete recording.

Remove `pocketstation-recordings/` when you no longer need the artifacts.

## Continue developing

- [Understand the Session architecture](../architecture/overview.md)
- [Build a Connector](../guides/connectors.md)
- [Add another extension boundary](../guides/extensions.md)

The compile command verifies the installed API and example. Running on one host
does not establish behavior for other devices, operating systems, or networks.

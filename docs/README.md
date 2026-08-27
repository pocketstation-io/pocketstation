# PocketStation developer documentation

Use these pages to build, understand, and operate a source-aware PocketStation
`Session`. Start with a runnable task, then move to concepts and reference as
your integration requires them.

## Get started

- [Capture a desktop application in Rust](getting-started/rust-quickstart.md)

## Develop

- [Build a Connector](guides/connectors.md)
- [Add a Source, Operator, Endpoint, or process extension](guides/extensions.md)

## Understand the system

- [Session architecture and ownership](architecture/overview.md)
- [Signals, streams, lineage, and runtime identity](concepts/signals-and-streams.md)

## Operate and upgrade

- [Compatibility and version boundaries](compatibility/README.md)
- [Release notes](../RELEASE_NOTES.md)

## Reference

- [Rust API on docs.rs](https://docs.rs/pocketstation/latest/pocketstation/)
- [C header](../include/pocketstation.h)

The public API, guides, compatibility page, and release notes describe the
features available to developers.

Evidence is scoped to the environment named by each artifact. Component tests
do not establish physical-device, cross-network, or cross-platform behavior.

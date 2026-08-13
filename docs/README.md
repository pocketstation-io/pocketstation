# PocketStation developer documentation

PocketStation is one Session engine for realtime audio and typed signals. Use
this documentation to integrate the Rust package, understand its execution
model, or author an external source, Operator, Endpoint, native extension, or
sidecar.

## Start here

- [Rust quickstart](getting-started/rust-quickstart.md)
- [Current architecture](architecture/overview.md)
- [Signals and typed streams](concepts/signals-and-streams.md)
- [External extensions](guides/extensions.md)
- [Compatibility and Core lifecycle](development/compatibility-and-freeze.md)

## Contract hierarchy

The public API and current guides describe shipping behavior. ADRs explain why
boundaries exist. Files explicitly marked historical or retained as reports do
not define the current API.

Product claims are evidence-scoped. Passing component or loopback tests does
not establish physical-device, cross-network, or cross-platform parity.

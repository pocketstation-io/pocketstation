# PocketStation developer documentation

PocketStation captures a desktop application and microphone once, preserves
them as independent source-aware stems, and fans them out through one bounded
`Session` to application code, external Operators, remote delivery, and
multistem recording.

The central contract preserves provenance, timing, discontinuities,
backpressure, lifecycle, and failure observations across realtime audio, typed
signals, Rust, C, and process sidecars. Use these guides to build on that
contract without creating a second media runtime inside each integration.

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

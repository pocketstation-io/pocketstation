# PocketStation 1.0.0

PocketStation 1.0.0 freezes the first extension-complete single engine.

The supported architecture is one `Session` implementation projected through
Rust, the C ABI, managed-language bindings, and the bounded PKSS sidecar
protocol. Rust `Stream<T>` is compile-time façade metadata; stable
`SignalSpec` and schema identities cross ABI and process boundaries.

Core 1.0 includes:

- independent application and microphone capture with source-aware lineage;
- fixed-capacity audio pools and bounded realtime `rtrb` edges;
- bounded byte- and count-limited typed signal edges;
- open external source, Operator, and Endpoint contracts;
- named multi-input/output composition and Operator chaining;
- bounded generated-audio reentry into the specialized audio runtime;
- multistem recording, Opus primitives, timing correction, observations, and
  explicit lifecycle/fault outcomes;
- executable versioned C extension callbacks and a Session-owned sidecar
  lifecycle.

Realtime callbacks remain allocation-free, lock-free, blocking-free,
async-free, log-free, and panic-free by contract and accepted gates.

Provider integrations, customer protocols, model implementations, advanced
DSP, exporters, transports, storage policy, and product business logic remain
outside Core and use the extension contracts.

Evidence classifications remain exact: cross-language and neutral transport
comparison artifacts are `LOOPBACK-ONLY`; accepted physical macOS evidence is
reported separately. This release does not claim universal platform parity or
overall performance superiority over LiveKit.

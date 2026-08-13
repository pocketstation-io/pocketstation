# Compatibility and Core 1.0 freeze

**Current state (2026-08-13): release candidate accepted; publication is in
progress and the Core 1.0 freeze is not active yet.** W18 Session source
integration, W19 Session composition/audio reentry, and the W20 executable C
ABI, sidecar, cross-language, installed-consumer, requalification, physical
macOS, compatibility, and performance gates are hash-accepted. Publication of
immutable `1.0.0` and independent registry consumption remain the final entry
gate for the dated freeze.

PocketStation has one engine: the Rust package and `libpocketstation`. Rust, C,
managed SDKs, and sidecars project the same Session, signal, lineage, error,
cancellation, observation, and outcome semantics.

Compatibility rules:

- Rust public changes follow SemVer and require an external packaged-consumer
  gate.
- C records carry ABI version and `struct_size_bytes`; new compatible fields
  are appended and old fields are not reinterpreted.
- Sidecar frames carry major/minor versions and reject unsupported versions or
  unbounded/invalid input.
- Stable `SignalSpec` identifiers and schemas—not Rust type identity—cross
  language and process boundaries.
- Existing compiled C consumers remain part of acceptance.

Core 1.0 will be a freeze decision, not a claim that every platform is equally
qualified. Its extension-completeness artifact can be `DONE` while desktop
support remains honestly classified per W13 evidence, including a narrower
macOS-first support declaration.

For 24 months after the Core 1.0 freeze, customer/product behavior must be an
external source, operator, endpoint/connector, transport, SDK projection, or
sidecar whenever the frozen contracts can express it. Central changes are
limited to correctness, security, OS/toolchain compatibility, measured
performance regressions, API/ABI repairs, or a genuinely new execution
primitive that the extension model cannot represent.

Adding a provider, customer protocol, exporter, model, or application policy is
not such a primitive.

# Compatibility and Core 1.0 freeze

**Current state: Core 1.0 is published, independently consumed, and the
24-month extension-first freeze is active from 2026-08-13 through
2028-08-13.** W18
Session source integration, W19 Session composition/audio reentry, and the W20
executable C ABI, sidecar, cross-language, installed-consumer,
requalification, physical macOS, compatibility, performance, and registry
release gates are hash-accepted. Documentation corrections through `1.0.2`
do not change the accepted runtime, Rust API, C ABI, or PKSS contract.

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

Core 1.0 is an architecture freeze, not a claim that every platform is equally
qualified. Extension completeness can be `DONE` while desktop support remains
honestly classified per W13 evidence, including a narrower macOS-first support
declaration.

Through 2028-08-13, customer/product behavior must be an external source,
operator, endpoint/connector, transport, SDK projection, or sidecar whenever
the frozen contracts can express it. Central changes are limited to
correctness, security, OS/toolchain compatibility, measured performance
regressions, API/ABI repairs, or a genuinely new execution primitive that the
extension model cannot represent. Every central change records that category,
the extension-model analysis, compatibility impact, realtime impact, and
evidence in its review.

Adding a provider, customer protocol, exporter, model, or application policy is
not such a primitive.

The freeze protects implementation contracts; it does not freeze clearer
documentation, better examples, onboarding repairs, or evidence-backed product
language. Public documentation should demonstrate the concrete developer
workflow and the implemented cross-boundary contract rather than awarding the
project labels such as "innovative." Claims of first-of-kind novelty or overall
superiority require separate prior-art or neutral comparative evidence.

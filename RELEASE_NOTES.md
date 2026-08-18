# PocketStation 1.x release notes

PocketStation 1.x is the compatible release line for one source-aware desktop
audio `Session`.

## Scope of the 1.x line

The core workflow captures one desktop application and one microphone once,
keeps them as independent stems, and fans them out concurrently to Operators,
application callbacks, remote delivery, and aligned multistem recording.

The stable execution contract preserves source, stream, stem, sequence,
timestamp, clock, discontinuity, capacity, backpressure, loss, lifecycle, and
failure semantics across:

- the specialized realtime audio lane;
- bounded typed-signal lanes;
- Rust and the versioned C ABI;
- bounded PKSS process sidecars;
- external source, Operator, Endpoint, Connector, and generated-audio
  extensions.

Provider implementations, customer protocols, models, transports, exporters,
storage policy, and application business logic remain outside Core.

## Compatibility promise

Compatible 1.x patches may correct documentation, packaging, security,
correctness, OS or toolchain integration, and measured regressions without
redefining the accepted Core contract. Rust API changes follow SemVer. The C
ABI uses versioned, size-prefixed records. PKSS frames carry explicit protocol
versions and stable `SignalSpec` identities.

The extension-first Core freeze is active through 2028-08-13. A new provider,
model, customer workflow, or exporter belongs in an extension rather than a
new Core category whenever the 1.x contracts can express it.

## What ships in 1.x

- Source-aware application and microphone capture with independent stems.
- Bounded realtime audio and typed-signal lanes with explicit capacity,
  backpressure, loss, discontinuity, and lifecycle behavior.
- Open Operator, Endpoint, Connector, external-source, and generated-audio
  extension contracts rather than closed provider categories.
- A managed Connector authoring path where Core owns bounded receiver polling,
  accounting, drain/abort, panic containment, and joined finalization while
  providers implement typed preparation and delivery only. The lower-level
  Endpoint-backed worker path remains available for specialized transports.
- Typed Connector configuration, redacted and destruction-cleared secrets,
  per-route edge authority, orthogonal provider service status, and structured
  failure codes/retryability preserved in terminal Session outcomes.
- Named composition, generated-audio reentry, aligned multistem recording,
  runtime observations, and explicit stop and fault outcomes.
- A versioned C ABI with executable callbacks and bounded PKSS process
  sidecars.
- Non-prompting microphone permission observation where the operating system
  exposes an authoritative query. `NotObservable` remains unknown; selected
  source prepare/open is authoritative on every platform.
- A self-contained Core release gate, exact-version publication recovery, a
  compiling public quickstart, extension guidance, platform prerequisites,
  and explicit evidence boundaries.

Realtime callbacks remain allocation-free, lock-free, blocking-free,
async-free, log-free, and panic-free by contract and accepted gates. Evidence
classifications remain exact; 1.x does not claim universal platform parity or
overall performance superiority.

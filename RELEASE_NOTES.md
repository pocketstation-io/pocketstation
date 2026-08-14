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

## Release history

### 1.0.3

- Reframed the public documentation around the concrete capture-once workflow
  and provenance-preserving bounded execution contract.
- Replaced self-awarded novelty language with implemented guarantees,
  extension boundaries, and exact evidence.
- Consolidated patch notes into this canonical 1.x document.

### 1.0.2

- Added a non-prompting Windows microphone `AppCapability` observation on
  supported Windows hosts.
- Kept Linux preflight explicitly `NotObservable`; selected-source
  prepare/open remains authoritative on every platform.

### 1.0.1

- Corrected docs.rs packaging to build the contracts-only surface on its native
  Linux target.
- Added the public quickstart, extension guide, architecture links, platform
  prerequisites, and evidence boundaries to the crate archive.

### 1.0.0

- Froze the first extension-complete single engine: source-aware capture,
  bounded realtime and typed lanes, open extension contracts, named
  composition, generated-audio reentry, multistem recording, observations,
  explicit lifecycle/fault outcomes, executable C callbacks, and bounded
  sidecars.

Realtime callbacks remain allocation-free, lock-free, blocking-free,
async-free, log-free, and panic-free by contract and accepted gates. Evidence
classifications remain exact; 1.x does not claim universal platform parity or
overall performance superiority.

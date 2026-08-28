# PocketStation 1.x release notes

PocketStation 1.x is the compatible release line for one source-aware desktop
audio `Session`.

## 1.1.3

PocketStation 1.1.3 corrects lifecycle, platform, and output-cancellation
behavior without adding a provider or another runtime:

- application-owned generated audio can carry an output generation, allowing a
  Session to discard pending frames for one cancelled response while unrelated
  Sources and routes continue;
- Endpoints receive that output identity through the existing bounded delivery
  path, and multistem recording preserves the Session's `Drain` or `Abort`
  request during finalization;
- the Windows native-capture library builds through the supported observation
  boundary; and
- macOS application capture accepts an exact display name or bundle identifier,
  preserves the discovered stable identity, and rejects ambiguous display
  names before capture begins.

The release does not add turn, model, provider, or conversation policy to Core.
Those concerns remain in SDK composition and provider packages.

## 1.1.2

PocketStation 1.1.2 completes the Core contracts required by the Python SDK
without changing the established 1.1 API:

- application-owned PCM uses the existing bounded Source lifecycle;
- Session lifecycle, compiler diagnostics, faults, timing, discontinuities,
  delivery, and recording observations have typed SDK projections;
- Python-authored Operators can return generated PCM through the existing
  bounded audio reentry path;
- recording metadata uses stable additive constants without changing the
  externally constructible `RecordingOutcome` structure; and
- source-aware Operator input bindings remain intact when multiple stems use
  the same Operator.

The stable Session declaration continues to provide application and microphone
Sources. System-output discovery remains available, but this patch does not add
a new exhaustive `Source` enum variant. `cargo-semver-checks` passes all 223
applicable checks against 1.1.1.

## Scope of the 1.x line

The core workflow keeps one desktop application and one microphone as
independent stems while a Session routes them concurrently to Operators,
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

During the 1.x line, a new provider, model, customer workflow, or exporter
belongs in an extension rather than a new Core category whenever the public
contracts can express it.

The extension-first Core freeze is active from 2026-08-13 through 2028-08-13.
During that period, additions use the existing open boundaries unless a
reviewed compatibility change shows that those contracts cannot express the
task.

## What ships in 1.x

- Source-aware application and microphone capture with independent stems.
- Bounded realtime audio and typed-signal lanes with explicit capacity,
  backpressure, loss, discontinuity, and lifecycle behavior.
- Open Operator, Endpoint, Connector, external-source, and generated-audio
  extension contracts rather than closed provider categories.
- A Connector driver authoring path where Core owns bounded receiver polling,
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
- A compiling public quickstart, extension guidance, platform prerequisites,
  compatibility checks, and scoped evidence.

Realtime callbacks remain allocation-free, lock-free, blocking-free,
async-free, log-free, and panic-free by contract and repository checks. Each
evidence artifact names its environment; 1.x does not claim universal platform
parity or overall performance superiority.

# Release and version information

<!-- claims: CLM-DOC-057-CAP-001,CLM-DOC-057-CAP-002,CLM-DOC-057-SOURCE-001 -->

PocketStation's package version at the analyzed snapshot is 1.1.1. Release automation and package metadata remain the publication authority; this page preserves the repository's declared release record.

## Scope

- **Install and feature-select the crate.** Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.
- **Build and publish repository artifacts.** Run architecture, protocol, package, platform, and release checks used by the repository publication workflow.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Snapshot release record

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

The extension-first Core freeze is active from 2026-08-13 through 2028-08-13.
A new provider, model, customer workflow, or exporter belongs in an extension
rather than a new Core category whenever the 1.x contracts can express it.

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
- A self-contained Core release gate, exact-version publication recovery, a
  compiling public quickstart, extension guidance, platform prerequisites,
  and explicit evidence boundaries.

Realtime callbacks remain allocation-free, lock-free, blocking-free,
async-free, log-free, and panic-free by contract and accepted gates. Evidence
classifications remain exact; 1.x does not claim universal platform parity or
overall performance superiority.

## Evidence scope

Release notes are declared evidence. They do not replace executable checks, physical qualification artifacts, or the compatibility baseline.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Cargo features and build surfaces](/docs/concepts/cargo-features.md)
- [Install PocketStation](/docs/getting-started/installation.md)
- [PocketStation documentation](/docs/README.md)
- [Release evidence boundary](/docs/concepts/release-evidence.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)
- `.github/workflows/publish.yml:1-161` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

# AUDIO-033: Single-package product boundary

**Status:** Accepted
**Date:** 2026-08-02
**Decision owners:** PocketStation runtime maintainers
**Scope:** Central Rust package, internal ownership, and native ABI artifacts

## Context

The central repository accumulated independently versioned `pks-*` packages
while architecture ownership was unsettled. Those packages had no independent
external consumers, yet imposed publication sequencing, re-export layers,
cross-package type conversions, duplicated feature/test configuration, and
artificially public internal APIs. The package topology became work in itself
without improving the narrow developer workflow.

The supported product is one Session implementation with one Rust developer
entry point and one native library. Internal concepts still require strict
ownership, but Rust modules, visibility, target configuration, tests, and
architecture checks provide that ownership without separate packages.

## Decision

The central repository contains exactly one Cargo package named
`pocketstation`.

- Frame, timing, graph, runtime, capture, endpoint, recording, codec, DSP,
  Session, observations, and ABI are internal modules.
- macOS, Windows, and Linux capture compile as target-selected submodules with
  target-specific dependencies in the root manifest.
- The Rust developer API remains rooted at `pocketstation::Session`.
- The package produces `rlib`, `cdylib`, and `staticlib` forms.
- Native consumers include `pocketstation.h` and link `libpocketstation` (or
  the conventional Windows equivalent).
- C is a consumption surface, not a separately named product or package.
- Repository-owned fixtures, benchmarks, and the CLI may use the explicitly
  unsupported `internal-testing` feature while migration completes.
- Provider and transport examples remain outside the engine.

No new Cargo package may be created without an independently consumed or
shipped artifact, independent versioning contract, process/security boundary,
unavoidable native toolchain boundary, or measured user-relevant build benefit.

## Compatibility

Existing `pks_*` C function symbols remain temporarily so already compiled
consumers can migrate without an immediate binary break. They are compatibility
symbols inside `libpocketstation`, not permission to retain `pks-codec-c`,
`pks-session-c`, `pocketstation-c`, or `pocketstation-core` product identities.
Their removal requires an explicit ABI version and migration gate.

The historical package-boundary decisions in AUDIO-008, AUDIO-029, AUDIO-031,
and AUDIO-032 remain evidence of their time but are superseded wherever they
prescribe multiple central packages, publication closures, or separate C
artifacts.

## Acceptance

1. Cargo metadata reports exactly one package/member named `pocketstation`.
2. No live central dependency references an old `pks-*` package.
3. Root tests, strict Clippy, formatting, quickstart, C/C++ ABI conformance,
   architecture checks, and packaging pass.
4. CLI, Python, Node, Lab fixtures, and benchmark consumers build against the
   root package.
5. Existing W12/W15/flight-recorder local artifacts verify without changing
   their evidence classification.
6. Current ownership and execution documents describe modules and the unified
   artifact names.

## Consequences

The consolidation removes manifest and publication churn while preserving the
semantic boundaries and test coverage. A change can now cross internal modules
without forcing coordinated package releases, but review must still block
ownership violations and cycles.

This is a behavior-preserving packaging refactor. It adds no provider, mock,
loopback path, platform claim, or real-device evidence. Existing local Lab
artifacts keep their `LOOPBACK-ONLY` or `PARTIAL` classification.

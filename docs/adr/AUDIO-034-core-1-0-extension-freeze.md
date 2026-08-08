# AUDIO-034: Core 1.0 extension completeness and freeze

**Status:** Accepted
**Date:** 2026-08-08
**Decision owners:** PocketStation runtime maintainers
**Scope:** Public extension contracts, language boundaries, and post-freeze ownership

## Context

The single-package decision removed package churn, but a durable freeze also
requires new sources, computations, destinations, generated audio, and managed
integrations to live outside central. A universal generic runtime would weaken
the specialized audio path and duplicate the existing dynamic signal system.

## Decision

PocketStation retains a specialized realtime `AudioFrame` lane and one
source-independent typed async lane. Stable `SignalSpec`, schema, named ports,
lineage/timing, `EdgeContract`, and `RuntimePlan` are runtime authority.

External extension points are source factories/drivers, operator manifests and
factories, endpoint drivers, the versioned C descriptor ABI, and the bounded
versioned sidecar protocol. Rust additionally exposes declaration-only
`Stream<T>`/`TypedOperator<I, O>` ergonomics. External packages own `T`; Rust
type identity never crosses a language or process boundary.

Provider, customer, industrial/domain, exporter, and application-policy types
remain outside core. Managed-language work and IPC never execute on audio
callbacks. No second engine is permitted in an SDK.

After the evidence-bound Core 1.0 freeze, this boundary is binding for 24
months. Central changes require correctness, security, OS/toolchain,
compatibility, measured-regression, or unexpressible-execution-primitive cause.

## Evidence and limitations

W17 proves generic signal identity; W18 open sources and shared bounded typed
ingress; W19 chaining, named ports, and generated-audio reentry; W20 packaged
Rust/C/Python extension consumption. W20 is a `LOOPBACK-ONLY` architecture and
compatibility proof. W13 separately owns physical desktop qualification and
must not be inferred from W20.

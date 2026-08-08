# AUDIO-034: Core 1.0 extension completeness and freeze

**Status:** Provisional; boundary direction accepted, freeze not active
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

The intended external extension points are source factories/drivers, operator
manifests and factories, endpoint drivers, an executable versioned C extension
ABI, and a bounded versioned sidecar lifecycle. Rust additionally exposes declaration-only
`Stream<T>`/`TypedOperator<I, O>` ergonomics. External packages own `T`; Rust
type identity never crosses a language or process boundary.

Provider, customer, industrial/domain, exporter, and application-policy types
remain outside core. Managed-language work and IPC never execute on audio
callbacks. No second engine is permitted in an SDK.

After the evidence-bound Core 1.0 freeze, this boundary is binding for 24
months. Central changes require correctness, security, OS/toolchain,
compatibility, measured-regression, or unexpressible-execution-primitive cause.

## Evidence and limitations

W17 proves generic signal identity. The W18 artifact proves low-level source
registration and shared bounded typed ingress but not Session source ingress.
The W19 artifact proves declaration compilation and low-level composition and
generated-audio primitives but not Session-owned execution. W20 proves
descriptor validation, PKSS framing, a Python round trip, and locally patched
packaged-source consumption; it does not prove executable C registration,
Session-owned sidecar lifecycle, a clean installed/published artifact, or a
`1.0.0` release. W20 remains a `LOOPBACK-ONLY` component/compatibility proof.
W13 separately owns physical desktop qualification and must not be inferred
from W20. This ADR becomes Accepted only when those exits pass.

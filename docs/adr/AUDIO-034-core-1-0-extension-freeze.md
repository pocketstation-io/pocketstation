# AUDIO-034: Core 1.0 extension completeness and freeze

**Status:** Accepted; 24-month extension-first freeze active
**Decision date:** 2026-08-13
**Freeze window:** 2026-08-13 through 2028-08-13
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

This boundary is binding from 2026-08-13 through 2028-08-13. Central changes
require a recorded correctness, security, OS/toolchain, compatibility,
measured-regression, or unexpressible-execution-primitive cause. A provider,
customer protocol, model, exporter, storage system, or application policy is
not a missing execution primitive.

## Evidence and limitations

W17 generic signal identity, W18 public Session source ingress, W19 named
composition and generated-audio reentry, and the W20 executable C ABI,
Session-owned sidecar, cross-language, clean installed-consumer,
requalification, physical macOS, compatibility, and performance gates are
hash-accepted. The cross-language and neutral comparison evidence remains
honestly `LOOPBACK-ONLY`; physical desktop qualification remains owned by W13
and the accepted macOS final proof.

Immutable Core 1.0 registry bytes were published and independently consumed
from a clean external repository. Documentation corrections through `1.0.2`
preserve the accepted Rust API, C ABI, PKSS wire contract, and realtime data
path. The accepted registry artifact remains `LOOPBACK-ONLY`; activation of
this architecture boundary does not upgrade Windows/Linux physical-device
evidence or establish novelty or overall superiority over another system.

# AUDIO-032: Package boundary convergence

**Status:** Accepted  
**Date:** 2026-07-29  
**Decision owners:** PocketStation runtime maintainers  
**Scope:** Supported Rust façade dependency closure and concrete endpoint
implementation ownership

## Context

The first published `pocketstation` façade has a 15-package workspace
dependency closure. The count is not itself a defect, but inspection found
three concrete ownership problems:

1. stable language-neutral Session codes were declared in `pocketstation`
   instead of the canonical `pks-session` semantic owner;
2. `pks-nodes` combines unrelated source fixtures, DSP adapters, bounded
   foreign-audio projection, mixing, and concrete multistem recording;
3. `pks-session` reaches both recording and foreign-audio projection through
   `pks-nodes`, pulling unrelated `pks-dsp` into every supported façade build
   and release.

Blindly merging packages would weaken dependency direction and make later
language adapters depend on a monolith. Leaving the current closure unchanged
would copy ambiguous ownership into Python and Node.

## Decision

### Canonical Session semantics

`pks-session` owns stable language-neutral Session declaration, start,
runtime, polling, stop, and failure code contracts. The `pocketstation` façade
re-exports those contracts and maps only its façade-specific wrapper errors.

### Concrete recording

Create `pks-recording` as the concrete multistem WAV endpoint implementation.
It owns:

- recorder declarations and validated stem labels;
- staged artifact creation and rollback;
- WAV writing, aligned Session timeline mapping, discontinuity and permission
  sidecars;
- checksums, manifest construction, observations, outcomes, and finalization;
- the concrete grouped endpoint driver implementing `pks-endpoint` lifecycle
  contracts.

It must not own graph compilation, runtime scheduling, Session lifecycle,
public façade ergonomics, capture, providers, or connector behavior.

### Bounded foreign-audio projection

Move the bounded polled-audio endpoint implementation into `pks-session`.
That endpoint is the canonical Session projection used by the Rust façade and
portable bindings, not a generally registered first-party audio operator.
Queue, batch, lease, and observation semantics remain unchanged.

### `pks-nodes` transition

`pks-nodes` stops being a dependency of `pks-session`. During the additive
0.1 transition it may re-export recording types from `pks-recording` for
source compatibility, but the supported façade closure must not contain
`pks-nodes` or `pks-dsp`.

Source fixtures, graph smoke operators, and DSP adapters remain in
`pks-nodes` only for current examples and compatibility until each has an
actual consumer or is retired. No new production responsibility may enter
that package.

### Later graph/runtime/platform convergence

`pks-caps`, `pks-metrics`, duplicate source-kind naming, and target capture
package convergence remain ordered work under
`W15-GRAPH-CONTRACT-CONVERGENCE`. They are not changed while W12 freezes the
cross-language Session reference.

## Dependency direction

```text
pocketstation
  └─ pks-session
       ├─ pks-recording ── pks-endpoint ── pks-runtime
       ├─ pks-capture
       ├─ pks-graph
       ├─ pks-runtime
       ├─ pks-frame
       └─ pks-timing

pks-nodes
  ├─ pks-recording  (temporary compatibility re-export)
  └─ pks-dsp        (optional operator/example surface)
```

`pks-recording` never depends on `pks-session`. `pks-session` composes the
recording factory, preventing a cycle and retaining one lifecycle owner.

## Compatibility

- Existing `pocketstation` public methods and stable strings remain
  source-compatible.
- Existing `pks-nodes` recording paths remain additive compatibility
  re-exports for the current 0.1 line.
- New supported publication uses a new compatible version.
- No accepted W10/W11 artifact is rewritten.

## Acceptance

The decision is implemented when:

1. `pks-session` owns and tests stable Session codes;
2. multistem implementation and tests compile under `pks-recording`;
3. bounded polled-audio implementation compiles under `pks-session`;
4. `pks-session` has no `pks-nodes` dependency;
5. the derived façade closure excludes `pks-nodes` and `pks-dsp`;
6. focused/full tests, strict Clippy, quickstart, architecture,
   CODE_PROTOCOL, and publication dry run pass.

## Consequences

The immediate closure becomes 14 packages rather than 15 because
`pks-recording` replaces the broad `pks-nodes` dependency and removes the
transitive `pks-dsp` dependency. This is an ownership correction, not a claim
that 14 is the final ideal count.

The later W15 convergence can reduce the closure further after cross-language
Session semantics are frozen and native platform evidence can detect
regressions.

This decision introduces no production scaffold, mock, fallback, provider,
unbounded queue, or new loopback-only path.

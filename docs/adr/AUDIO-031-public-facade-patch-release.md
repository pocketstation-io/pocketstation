# AUDIO-031 — Public Facade Patch Release

## Status

Accepted for Phase 2. This decision narrows the lockstep rule in AUDIO-008 for
public-facade-only patch releases.

## Context

`pocketstation 0.1.0` and its internal dependency closure were published with
one workspace version. Its docs.rs build failed because the default Linux
documentation target selected PipeWire and the docs.rs sandbox did not provide
the required native development package.

The crates.io archive is immutable. Repairing only public-facade packaging or
documentation metadata does not require republishing unchanged internal
packages.

## Decision

The `pocketstation` public façade may receive an independent patch version when
all of these conditions hold:

1. The change is limited to façade packaging, documentation, or compatible
   façade code.
2. Its published internal dependencies remain SemVer-compatible.
3. The exact existing dependency versions are registry-visible.
4. Runtime behavior and internal package APIs are unchanged.
5. CI and protected release validation reproduce the corrected package gate.

Feature or internal API releases continue to use the coordinated dependency
order and version policy from AUDIO-008.

## Consequences

- `pocketstation 0.1.1` may depend on the unchanged `0.1.0` internal closure.
- The public release tag follows the façade version.
- The publisher skips registry-visible internal versions and publishes only the
  missing façade version.
- Failed documentation for `0.1.0` remains immutable historical evidence and
  is not hidden by yanking the release.

## Verification

- Cross-document the façade for the configured docs.rs target.
- Package the façade and inspect its normalized metadata.
- Run the exact publication-closure dry run and protected release validation.

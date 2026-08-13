## Linked issue

Closes #

## Summary

## Scope control

- [ ] I modified only this repo.
- [ ] I respected the repo phase gate.
- [ ] I did not edit architecture docs unless explicitly assigned.
- [ ] I did not add dependencies without approval.

## Core 1.0 freeze classification

Check exactly one when this PR changes `src/`, `include/`, `build.rs`, or the
public package manifest.

- [ ] No Core semantic change (mechanical refactor, documentation, or evidence only)
- [ ] Correctness bug
- [ ] Security vulnerability
- [ ] OS/API/toolchain breakage
- [ ] Measured performance regression
- [ ] API/ABI compatibility repair
- [ ] Missing execution primitive that no source, operator, endpoint/connector, transport, SDK projection, or sidecar can express

### Extension-model analysis

Explain why this belongs in Core. For a missing primitive, list the attempted
extension shapes and the concrete contract that could not express the need.

### Compatibility and realtime impact

Record Rust API, C ABI, PKSS, bounded-memory, and realtime callback impact.
Write `none` only after checking each boundary.

## Tests run

```bash

```

## Risks

## Reviewer focus

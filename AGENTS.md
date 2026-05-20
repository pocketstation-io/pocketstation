# AGENTS.md — pocketstation-io/audio-core

## Source of truth

Before editing, read:

1. `docs/architecture/PocketStation-v2.3.md`
2. `docs/REPO_CONTRACT.md`
3. Relevant ADRs in `docs/adr/`
4. The assigned GitHub issue

## Phase gate

This repo activates in **Phase 0**.

If the current project phase is earlier, do not implement code here unless the issue has `phase-exception-approved`.

## Rules

- One issue = one branch = one PR.
- Do not edit unrelated repos.
- Do not create `pocketstation-io/protocol` before Phase 2.
- Do not change v2.3 architecture unless explicitly assigned.
- Do not add dependencies without approval.
- Do not bypass CI.


## Engineering Standards

Before code changes, every agent must read:

- `docs/standards/STAFF_ENGINEERING_BAR.md`
- `docs/standards/STRUCTURE_NAMING_STYLE_THINKING.md`
- `docs/REPO_CONTRACT.md`
- relevant ADRs
- relevant phase progress file

All code must follow the structure, naming, documentation,
test naming, comment style, and thinking process defined there.

Every non-trivial implementation documents:

- invariant
- ownership model
- failure behavior
- test coverage
- phase scope
- what is intentionally not implemented
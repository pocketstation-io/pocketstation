# Feature flags

<!-- claims: CLM-REF-014-CAP-001,CLM-REF-014-CAP-002,CLM-REF-014-SOURCE-001 -->

## Scope

- **Install and feature-select the crate.** Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

The scope of **Feature flags** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Feature flags**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|

## Interpretation

The **Feature flags** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Run the examples](/docs/getting-started/examples.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [Cargo features and build surfaces](/docs/concepts/cargo-features.md)
- [Conformance and qualification](/docs/concepts/conformance.md)

## Evidence boundary

The claims on **Feature flags** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)

For **Feature flags**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

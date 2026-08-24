# Security boundaries

<!-- claims: CLM-SEC-001-SCOPE-001,CLM-SEC-001-TEXT-001,CLM-SEC-001-TEXT-002,CLM-SEC-001-TEXT-003,CLM-SEC-001-TEXT-004,CLM-SEC-001-SOURCE-001 -->

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.
- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.

The scope of **Security boundaries** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Secret values

Connector configuration distinguishes secret values from ordinary text. The repository's secret owner overwrites initialized string bytes during clearing, and connector diagnostics use redacted representations. This does not claim that every upstream or downstream copy is erased.

## Executable extension trust

Native loading requires an absolute path, canonicalizes it, checks for a regular file, validates the ABI descriptor, and imports registrations transactionally. The host still decides whether the file is trusted executable code.

## Process boundary

Sidecars are executable process and protocol boundaries. Configure finite messages and deadlines. Do not infer authentication or sandbox guarantees absent from the contract.

## C ABI ownership

Use header-defined handle and callback ownership. Keep libraries alive while callback contexts remain reachable, and release handles through matching functions.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Configuration reference](/docs/reference/configuration.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)

## Evidence boundary

The claims on **Security boundaries** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/secret.rs:3-12` (`DIRECT`)
- `src/native_extension/library.rs:19-19` (`DIRECT`)
- `src/native_extension/library.rs:21-24` (`DIRECT`)
- `src/native_extension/library.rs:22-22` (`DIRECT`)
- `src/native_extension/library.rs:23-23` (`DIRECT`)
- `src/native_extension/library.rs:33-230` (`DIRECT`)
- `src/native_extension/library.rs:232-263` (`DIRECT`)
- `src/native_extension/library.rs:265-271` (`DIRECT`)

For **Security boundaries**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

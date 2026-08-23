# Security boundaries

<!-- claims: CLM-SEC-001-CAP-001,CLM-SEC-001-CAP-002,CLM-SEC-001-CAP-003,CLM-SEC-001-CAP-004,CLM-SEC-001-SOURCE-001 -->

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.
- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/secret.rs:1-13` (`DIRECT`)
- `src/native_extension/library.rs:1-272` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.

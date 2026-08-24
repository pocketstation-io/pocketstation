# Protocol surface index

<!-- claims: CLM-REF-020-SCOPE-001,CLM-REF-020-TEXT-001,CLM-REF-020-TEXT-002,CLM-REF-020-SOURCE-001 -->

## Scope

- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.
- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.
- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.

The scope of **Protocol surface index** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Protocol surface index**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Inventory

| Evidence record | Boundary | Declaration | Source |
|---|---|---|---|
| protocol-293c47976ad558b3d2b0 | binary_connector_protocol | `connector-audio-record-v1` | `src/connector/transport.rs:12` |
| protocol-2ea9ff75233cc6e4e21e | binary_process_protocol | `sidecar-frame-v1` | `src/runtime/lifecycle/sidecar_protocol.rs:1` |
| protocol-569e84d68311830f8df0 | c_abi | `session-c-abi-v1` | `src/abi/session/abi.rs:6` |
| protocol-7b1052ed6f9381798f08 | c_abi | `opus-c-abi-v1` | `src/abi/codec.rs:46` |
| protocol-9a7f24edbf8f3a1e77ce | binary_connector_protocol | `connector-configuration-record-v1` | `src/connector/transport.rs:22` |
| protocol-f6cecae06276f5cc5d4b | c_abi | `native-extension-c-abi-v1` | `src/abi/extension.rs:12` |

## Interpretation

The **Protocol surface index** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Security boundaries](/docs/security/boundaries.md)
- [Author a connector](/docs/guides/connectors.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Configuration reference](/docs/reference/configuration.md)

## Evidence boundary

The claims on **Protocol surface index** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `include/pocketstation.h:7-27` (`DIRECT`)
- `src/connector/mod.rs:61-65` (`DIRECT`)
- `src/connector/mod.rs:62-62` (`DIRECT`)
- `src/connector/mod.rs:63-63` (`DIRECT`)
- `src/connector/mod.rs:64-64` (`DIRECT`)
- `src/connector/mod.rs:68-81` (`DIRECT`)
- `src/connector/mod.rs:88-105` (`DIRECT`)
- `src/connector/mod.rs:112-117` (`DIRECT`)
- `src/connector/mod.rs:119-121` (`DIRECT`)
- `src/connector/mod.rs:124-124` (`DIRECT`)
- `src/connector/mod.rs:125-129` (`DIRECT`)
- `src/connector/mod.rs:126-126` (`DIRECT`)
- `src/connector/mod.rs:127-127` (`DIRECT`)
- `src/connector/mod.rs:128-128` (`DIRECT`)
- `src/connector/mod.rs:132-134` (`DIRECT`)
- `src/connector/mod.rs:136-138` (`DIRECT`)
- `src/connector/mod.rs:140-151` (`DIRECT`)
- `src/connector/mod.rs:153-157` (`DIRECT`)
- `src/connector/mod.rs:159-179` (`DIRECT`)
- `src/connector/mod.rs:182-184` (`DIRECT`)
- `src/connector/mod.rs:183-183` (`DIRECT`)
- `src/connector/mod.rs:187-189` (`DIRECT`)
- `src/connector/mod.rs:191-200` (`DIRECT`)
- `src/connector/mod.rs:204-221` (`DIRECT`)
- `src/connector/mod.rs:224-224` (`DIRECT`)

For **Protocol surface index**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

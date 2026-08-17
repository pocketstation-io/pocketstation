# Build a connector

A connector is an externally packaged endpoint implementation. It consumes
one or more named Session routes and publishes them to a protocol, provider or
customer boundary without changing Core.

## Contract

Create a `ConnectorManifest` and an `EndpointDriverFactory`, then register the
package once and declare one or more configured instances:

```rust,no_run
use std::sync::Arc;
use pocketstation::connector::{Connector, ConnectorConfiguration};
use pocketstation::Session;

# fn manifest() -> pocketstation::connector::ConnectorManifest { todo!() }
# struct RelayFactory;
# impl pocketstation::EndpointDriverFactory for RelayFactory {
#   fn prepare(&self, _: Vec<pocketstation::EndpointPortInput>)
#     -> Result<Box<dyn pocketstation::PreparedEndpointDriver>, pocketstation::EndpointFailure>
#   { todo!() }
# }
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let package = Connector::new(manifest(), Arc::new(RelayFactory))?;
let relay = session.register_connector(package)?;
let endpoint = relay.declare(&session, ConnectorConfiguration::new())?;
# let _ = endpoint;
# Ok(())
# }
```

The full compiling example is
[`examples/connector_authoring.rs`](../../examples/connector_authoring.rs).

## Manifest

The manifest is inspectable before execution. It contains:

- stable open operator and node type identities;
- package and manifest revisions;
- named input ports with `SignalSpec` and `MediaCaps`;
- typed configuration fields, defaults, constraints and deprecations;
- delivery, worker-queue, retry, timeout and readiness policies;
- open capability and resource-requirement identifiers.

Connector API revision 1 models an endpoint, so it requires at least one input
and rejects outputs. Generated audio returns through the existing audio-reentry
Bridge, not through a connector output port.

## Configuration and secrets

Use `ConnectorConfigurationValue` instead of parsing an untyped map. Unknown
fields, missing required values, wrong types, invalid defaults and constraint
violations fail before Session compilation.

Use `ConnectorSecret` for credentials. Its normal `Debug` output is redacted,
and the sensitive classification survives lowering into the existing
`EndpointConfiguration` and `NodeConfig`. A factory may explicitly read the
secret during setup or worker execution; it must not copy it into errors,
logs, metrics or serialized observations.

## Lifecycle

The existing endpoint lifecycle remains authoritative:

1. `prepare` acquires resources but does not consume media.
2. `start` creates the running owner behind the closed `EndpointStartGate`.
3. Media can be consumed only after the Session opens the gate.
4. `request_stop` is idempotent and non-panicking.
5. `join_and_finalize` waits for owned workers and reports final observations.
6. `Drop` only signals and releases ownership; it never blocks the realtime
   path.

Use `ConnectorObservationHandle` for readiness and connector-level counters.
Use `ConnectorErrorCode`, stage and retryability for machine-readable failures.
Retries and provider protocol state remain owned by the concrete connector;
they must obey the finite manifest policy.

## Package boundary

Do not add provider dependencies to `pocketstation`. Put the implementation in
an independent connector package and keep only the provider-neutral contract
in Core.

Managed SDK users configure and consume packaged connector implementations.
Rust authors implement `EndpointDriverFactory` directly. A Python or
JavaScript process may use the bounded sidecar or native-extension boundary
for supported signals, but it never runs a managed callback on the realtime
PCM path. The current native extension ABI does not provide arbitrary dynamic
PCM endpoint authoring.

Enable `conformance-fixtures` and use `pocketstation::connector::conformance`
for the canonical deterministic Session. A supported connector package must
cover every `REQUIRED_CONNECTOR_CONFORMANCE_CASES` entry and add real protocol
integration evidence in its own repository.


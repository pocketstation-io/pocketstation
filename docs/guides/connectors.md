# Build a connector

A Connector is an externally packaged Endpoint integration. It consumes one
or more named Session routes and publishes them to a protocol, provider, or
customer boundary without adding that provider to Core.

## Authoring surface

Implement `ConnectorFactory` to validate and acquire provider resources, then
return one `ConnectorWorker`. Core adapts that worker to the canonical Endpoint
transaction:

```rust,no_run
use std::sync::Arc;
use pocketstation::connector::{
    Connector, ConnectorConfiguration, ConnectorError, ConnectorFactory,
    ConnectorWorker,
};
use pocketstation::{EndpointPortInput, Session};

# fn manifest() -> pocketstation::connector::ConnectorManifest { todo!() }
# struct RelayFactory;
# impl ConnectorFactory for RelayFactory {
#   fn prepare(&self, _: Vec<EndpointPortInput>)
#     -> Result<Box<dyn ConnectorWorker>, ConnectorError>
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

The complete compiling example is
[`examples/connector_authoring.rs`](../../examples/connector_authoring.rs).

## What Core supplies

The author does not create an Endpoint lifecycle or a Session registry. The
private adapter supplies:

- execution behind the closed Session start gate;
- one stop token shared with the worker;
- startup-readiness deadline supervision;
- panic containment and terminal error classification;
- preparation cancellation;
- joined shutdown; and
- canonical `EndpointDriverObservations` for delivery accounting.

The Connector worker consumes the bounded `EndpointPortInput` receivers,
reports delivery through `ConnectorContext`, and exits only after stop is
requested or with a `ConnectorRunOutcome::failure`.

## Manifest and routing

`ConnectorManifest` is inspectable before execution. It contains:

- stable open operator and node type identities;
- package and manifest revisions;
- named inputs with the existing `SignalSpec` and `MediaCaps`;
- typed configuration fields, defaults, constraints, and deprecations;
- the existing `EdgeContract` for bounded route behavior;
- a finite startup-readiness deadline and probe thresholds; and
- open capability and resource-requirement identifiers.

Connector API revision 1 is an Endpoint integration: it requires at least one
input and rejects outputs. Generated audio re-enters through the existing audio
Bridge, not through a Connector output port.

## Configuration and secrets

Use `ConnectorConfigurationValue` instead of parsing an untyped map. Unknown
fields, missing required values, wrong types, invalid defaults, and constraint
violations fail before Session compilation.

Use `ConnectorSecret` for credentials. Its `Debug` output is redacted, and the
sensitive classification survives lowering into `EndpointConfiguration` and
`NodeConfig`. A connector may explicitly read a secret during setup or worker
execution; it must never copy it into errors, logs, metrics, or observations.

## Service status and failures

Call `ConnectorContext::report_readiness_success` or
`report_readiness_failure` when using thresholds, or set readiness directly
after a provider handshake. Report health and recovery independently:

```text
Ready + Healthy + Idle          normal delivery
Ready + Degraded + Idle         delivering with reduced service quality
NotReady + Degraded + Reconnecting  reconnect in progress
```

Use `ConnectorErrorCode`, `ConnectorErrorStage`, and
`ConnectorRetryability` for machine-readable failures. Connector packages own
their actual retry/backoff protocol and must keep it finite. Core does not
pretend to enforce an unused generic retry policy.

`RegisteredConnector::observations` returns provider observations beside the
canonical Endpoint observations. Session route metrics remain authoritative
for queue capacity, backpressure, and route drops.

## Grouping and shutdown

Override `ConnectorFactory::preparation_group` when several declared routes
must share one provider connection. Returning one shared
`EndpointPreparationGroup` lets application and microphone buses use one
worker and one joined provider lifecycle while retaining independent route and
lineage identities.

The worker must:

1. acquire resources in `prepare` without consuming media;
2. report ready only after the provider can accept delivery;
3. poll or block only on bounded/non-realtime integration primitives;
4. stop when `ConnectorContext::is_stop_requested` becomes true; and
5. return a truthful success or classified failure outcome.

## Package and language boundary

Do not add provider dependencies to `pocketstation`. Production providers live
in independent connector packages. The first-party PocketStation Relay package
is named `pocketstation-relay` and is owned by the connectors repository.

Managed SDK users configure and consume packaged connector implementations. A
Python or JavaScript process may use a supported bounded sidecar or extension
boundary, but it never runs managed code on the realtime PCM path. The current
native extension ABI does not provide arbitrary dynamic PCM Endpoint authoring.

Enable `conformance-fixtures` and implement every
`REQUIRED_CONNECTOR_CONFORMANCE_CASES` case. Core component conformance is not
provider proof: a supported package must also test its real authentication,
network setup, readiness, reconnect, multi-bus delivery, and receiver-visible
outcome in its owning repository.

# Build a connector

A Connector is an externally packaged Endpoint integration. It consumes one
or more named Session routes and publishes them to a protocol, provider, or
customer boundary without adding that provider to Core.

## Authoring surface

Implement `ConnectorDriverFactory` to validate and acquire provider
resources, then return one `ConnectorDriver`. Core owns bounded input polling,
delivery accounting, drain/abort, and the canonical Endpoint transaction:

```rust,no_run
use std::sync::Arc;
use pocketstation::connector::{
    Connector, ConnectorConfiguration, ConnectorDeliveryOutcome,
    ConnectorError, ConnectorInputDescriptor, ConnectorItem,
    ConnectorDriver, ConnectorDriverFactory,
};
use pocketstation::Session;

# fn manifest() -> pocketstation::connector::ConnectorManifest { todo!() }
# struct RelayFactory;
# impl ConnectorDriverFactory for RelayFactory {
#   fn prepare(&self, _: &[ConnectorInputDescriptor])
#     -> Result<Box<dyn ConnectorDriver>, ConnectorError>
#   { todo!() }
# }
# struct Relay;
# impl ConnectorDriver for Relay {
#   fn deliver(
#     &mut self,
#     _: ConnectorItem<'_>,
#     _: &pocketstation::connector::ConnectorContext,
#   ) -> Result<ConnectorDeliveryOutcome, ConnectorError> {
#     Ok(ConnectorDeliveryOutcome::Delivered)
#   }
# }
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let connector = Connector::with_driver(manifest(), Arc::new(RelayFactory))?;
let relay = session.register_connector(connector)?;
let endpoint = relay.declare(
    &session,
    ConnectorConfiguration::new(),
    pocketstation::EdgeContract::realtime_audio(),
)?;
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
- one shutdown token shared with the worker, preserving drain versus abort;
- startup-readiness deadline supervision;
- panic containment and terminal error classification;
- preparation cancellation;
- joined shutdown; and
- canonical `EndpointDriverObservations` for delivery accounting.

The driver adapter consumes the bounded `EndpointPortInput` receivers. The
provider handles typed `ConnectorItem` values and returns an explicit delivered
or dropped result. `ConnectorFactory` and `ConnectorWorker` remain the advanced
escape hatch when a protocol requires a specialized off-realtime worker.

## Manifest and routing

`ConnectorManifest` is inspectable before execution. It contains:

- stable open operator and node type identities;
- package and manifest revisions;
- named inputs with the existing `SignalSpec` and `MediaCaps`;
- typed configuration fields, defaults, constraints, and deprecations;
- a finite startup-readiness deadline and probe thresholds; and
- open capability and resource-requirement identifiers.

`EdgeContract` is supplied when the connector endpoint is declared. Capacity,
loss, backpressure, copy, and latency policy belong to that exact Graph route,
not to a provider manifest.

Connector API revision 1 is an Endpoint integration: it requires at least one
input and rejects outputs. Generated audio re-enters through the existing audio
Bridge, not through a Connector output port.

## Configuration and secrets

Use `ConnectorConfigurationValue` instead of parsing an untyped map. Unknown
fields, missing required values, wrong types, invalid defaults, and constraint
violations fail before Session compilation.

Use `ConnectorSecret` for credentials. Its `Debug` output is redacted, the
sensitive classification survives lowering into `EndpointConfiguration` and
`NodeConfig`, and sensitive owned strings are overwritten on destruction. A
connector may explicitly read a secret during setup or worker execution; it
must never copy it into errors, logs, metrics, or observations.

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

Override `ConnectorDriverFactory::preparation_group` when several declared routes
must share one provider connection. Returning one shared
`EndpointPreparationGroup` lets application and microphone buses use one
worker and one joined provider lifecycle while retaining independent route and
lineage identities.

The connector driver must:

1. acquire resources in `prepare` without consuming media;
2. report ready only after the provider can accept delivery;
3. keep each provider operation finite and off realtime;
4. return an explicit delivery result or structured error; and
5. finalize provider resources in `shutdown`.

Core handles receiver polling and monotonic shutdown: an abort can upgrade
drain, but a later drain cannot weaken an abort. Low-level Connector workers
remain responsible for their own bounded loop and must follow the same rule.

## Package and language boundary

Do not add provider dependencies to `pocketstation`. The authoritative
`pocketstation-relay` implementation lives in `pocketstation-io/connectors/relay`.
`pks`, Python, JavaScript, Lab, and Bench consume or project that package; none
owns a second Relay media engine. Protocol owns portable wire semantics and
conformance vectors.

A Python or JavaScript process may use its supported native projection or a
bounded sidecar/extension boundary, but it never runs foreign-language code on the
realtime PCM path. The current native extension ABI does not provide arbitrary
dynamic PCM Endpoint authoring. Do not claim that third-party Python or
JavaScript code can author an audio connector until a versioned managed/native
audio boundary and cross-language conformance suite exist.

Enable `conformance-fixtures` and execute the deterministic Session fixtures in
`pocketstation::conformance` against the package's real `Connector`
registration. Connector does not publish a second conformance namespace. A
checklist or self-reported result is not conformance. Core component
conformance is not provider proof: a supported package must also test its real
authentication, network setup, readiness, reconnect, multi-bus delivery, and
receiver-visible outcome in its owning repository.

# Send Session audio to an external system

A Connector is an externally packaged Endpoint integration. It consumes one
or more named Session routes and publishes them to a protocol, provider, or
customer system without adding that provider to Core.

Use a Connector when audio must leave PocketStation for an existing service or
application: a WebSocket publisher, a call transport, a monitoring system, or
a provider SDK. Use an `Operator` for audio-to-text or other computation. Use a
`Source` when the external system sends media into the Session.

## Choose a Connector for outbound delivery

```text
Source or AudioInput
        ↓
bounded Session route, retaining source and stem identity
        ↓
Core-owned Connector worker
        ↓
provider-owned send operation
        ↓
external system
```

The concise API removes registration code; it does not bypass the Session.
Every destination still uses the normal graph compiler, Endpoint start gate,
finite route capacity, delivery observations, and joined shutdown. A slow or
failed destination cannot make its queue unbounded and does not become a
capture callback.

## Send audio with a function

Use a function when the destination needs only one audio delivery operation:

```rust,no_run
use pocketstation::connector::Connector;
use pocketstation::{Session, Source};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let destination = session.destination(Connector::from_audio_fn(|frame| {
    println!(
        "source={} samples={}",
        frame.source_id().get(),
        frame.samples().len()
    );
    Ok(())
})?)?;

let application = session.capture(Source::application("Spotify"))?;
application.send(destination)?;
# Ok(())
# }
```

The function runs on Core's bounded Connector worker, never on an audio
callback. The frame includes its source, stream, sequence, timestamp, clock,
and discontinuity lineage.

## Reuse a provider

Implement `AudioConnector` when a provider opens and closes resources:

```rust,no_run
use pocketstation::connector::{AudioConnector, Connector, ConnectorError};
use pocketstation::EndpointAudioFrame;

struct Provider;

impl AudioConnector for Provider {
    fn start(&mut self) -> Result<(), ConnectorError> {
        Ok(())
    }

    fn send(&mut self, frame: &EndpointAudioFrame) -> Result<(), ConnectorError> {
        let _samples = frame.samples();
        Ok(())
    }

    fn stop(&mut self) -> Result<(), ConnectorError> {
        Ok(())
    }
}

# fn build() -> Result<Connector, Box<dyn std::error::Error>> {
let destination = Connector::from_audio(Provider)?;
# Ok(destination)
# }
```

One `Connector` value represents one destination and one lifecycle. Sending
application and microphone stems to its declared Endpoint creates independent
bounded routes while calling `start` and `stop` once.

This is useful when one provider connection carries several named stems. Create
two `Connector` values when the destinations need separate credentials,
connections, failure domains, or shutdown outcomes.

## Lifecycle and failure behavior

| Provider operation | Meaning | Core behavior |
|---|---|---|
| `start()` | Open the configured destination. | Runs on the managed worker after the Session start gate. A failure is retained in the terminal outcome and calls `stop()` once. |
| `send(frame)` | Deliver one source-aware PCM frame. | Runs off realtime through a finite route. A failure is retained as a Connector and Endpoint outcome. |
| `stop()` | Release provider resources. | Runs once after drain, abort, startup rollback, or provider failure. |

`EndpointAudioFrame` preserves the source, stream, stem, clock, sequence,
timestamp, discontinuity, route, and output identity available when Core
delivers the frame. Connector code should use those fields for protocol
metadata and diagnostics instead of inferring identity from call order.

## Build a distributable integration

Implement `ConnectorDriverFactory` to validate and acquire provider
resources, then return one `ConnectorDriver`. Core owns bounded input polling,
delivery accounting, drain/abort, and the Endpoint transaction:

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

The concise and advanced compiling examples are
[`examples/connector.rs`](../../examples/connector.rs) and
[`examples/connector_authoring.rs`](../../examples/connector_authoring.rs).

Use the advanced API only when a distributable package needs portable identity,
typed configuration, secret fields, named signal inputs, custom readiness, or
provider-specific observations. Both forms use the same Endpoint and Session
runtime.

## Rely on Core for lifecycle and delivery safety

The author does not create an Endpoint lifecycle or a Session registry. The
private adapter supplies:

- execution behind the closed Session start gate;
- one shutdown token shared with the worker, preserving drain versus abort;
- startup-readiness deadline supervision;
- panic containment and terminal error classification;
- preparation cancellation;
- joined shutdown; and
- `EndpointDriverObservations` for delivery accounting.

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

`EdgeContract` is supplied when the Connector Endpoint is declared. It records
the accepted media plus capacity, loss, backpressure, copy, and latency
settings for that route. Those choices belong to the route, not to the provider
manifest.

Connector API revision 1 is an Endpoint integration: it requires at least one
input and rejects outputs. Generated audio re-enters through the existing audio
Bridge, not through a Connector output port.

## Configuration and secrets

Use `ConnectorConfigurationValue` instead of parsing an untyped map. Unknown
fields, missing required values, wrong types, invalid defaults, and constraint
violations fail before Session compilation.

Use `ConnectorSecret` for credentials. Its `Debug` output is redacted, the
sensitive classification is retained when Core creates `EndpointConfiguration` and
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
Endpoint observations. Session route metrics remain authoritative
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

## Keep provider code in its package

Keep provider dependencies in the Connector package, not in `pocketstation`.
The package owns authentication, protocol framing, provider deadlines,
reconnection, and provider-specific errors. Core continues to own Session
lifecycle, bounded routing, lineage, and Endpoint shutdown.

Language SDKs can expose this Connector lifecycle when they provide a supported
native or managed integration. Provider callbacks never run on realtime PCM
callbacks. Check the SDK documentation before depending on a language-specific
authoring feature.

Enable `conformance-fixtures` and execute the deterministic Session fixtures in
`pocketstation::conformance` against the package's `Connector`
registration. Connector does not publish a second conformance namespace. A
checklist or self-reported result is not conformance. A provider package must
also test its own authentication, network setup, readiness, reconnect,
multi-bus delivery, and a receiver-visible result.

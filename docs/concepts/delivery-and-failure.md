# Set delivery behavior for each route

Every destination receives data through its own finite queue. A slow model,
network connection, recorder, or application consumer can affect its route;
it cannot silently create an unbounded backlog or stall unrelated routes.

The normal APIs choose finite settings for their work. Advanced packages use
`RouteSettings` when they need to select the accepted media or change delivery
behavior explicitly.

## Start with the normal APIs

```rust,no_run
use pocketstation::{Session, Source};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let application = session.capture(Source::application("Spotify"))?;

application.send(session.polled_audio()?)?;
application.record("application")?;
# Ok(())
# }
```

Each call creates a separate route. Polled audio may saturate while recording
continues, and recording may fail while another Connector continues. The final
Session outcome reports each failure.

## Understand the delivery settings

`RouteSettings` contains two decisions:

- `MediaCaps` describes the media accepted by the route.
- `DeliveryPolicy` describes timing, queue pressure, loss, copying, and
  observations.

The complete settings are:

| Setting | Question it answers |
|---|---|
| `MediaCaps` | Which signal class, sample format, rate, channels, or schema can move? |
| `ClockDomain` | Which clock supplies timestamps? |
| latency and jitter budgets | How much queued age may the route accept? |
| `BackpressurePolicy` | What happens when capacity is full? |
| `DeliverySemantics` | Is delivery realtime best effort, ordered, or non-realtime exact delivery? |
| `LossPolicy` | May data be dropped, concealed, or treated as terminal failure? |
| `CopyPolicy` | May a frame be shared, moved, or copied to a branch pool? |
| `RouteObservability` | Which counters and timing observations are retained? |
| maximum payload bytes | What is the largest accepted typed payload? |

`RouteSettings::realtime_audio()` selects nonblocking delivery for callback-fed
PCM. `RouteSettings::bounded_async()` selects a finite queue for off-realtime
signals. Connected ports still negotiate their concrete media format.

Change delivery without rebuilding the media requirements:

```rust
use pocketstation::{BackpressurePolicy, DeliveryPolicy, RouteSettings};

let delivery = DeliveryPolicy::realtime_audio()
    .with_backpressure(BackpressurePolicy::DropOldest);
let settings = RouteSettings::realtime_audio().with_delivery_policy(delivery);
```

## Choose backpressure deliberately

`DropNewest` preserves data already queued and rejects the newest item.
`DropOldest` favors fresh realtime media by evicting the oldest item.
`BoundedQueue` permits a producer to wait only where blocking is allowed.
`BlockForbidden` turns saturation into an explicit upstream error.

Do not put a blocking policy on a capture callback or realtime worker. The
compiler rejects incompatible safety and execution settings before start.

## Keep queued age visible

Capacity alone does not prevent stale media. A finite queue can still contain
audio that is too old for a live conversation. Read route latency observations
and enforce the workflow's latency budget. Report p50, p95, p99, and maximum
with units when making a latency claim.

For generated speech, cancellation removes matching output that remains in
Core's sender queues. A Connector and receiver must separately report what they
discarded. Core cannot recall packets already sent or prove which loudspeaker
sample a person heard.

## Preserve discontinuities

A dropped frame, restarted Source, changed generation, clock reset, or cancelled
output must remain visible. Do not repair sequence numbers by pretending the
missing media existed. Use discontinuity epochs and observations to tell a
consumer when continuity changed.

## Handle route failure

When a destination fails:

1. inspect its route metrics and structured failure;
2. determine whether the Source and other routes remain healthy;
3. stop or cancel according to application policy;
4. require the terminal Session outcome; and
5. retry only after the provider's stated retry condition is met.

Provider retries must have a finite attempt count, per-attempt deadline, total
elapsed deadline, and cancellation. Core does not invent provider credentials,
reconnection tokens, or retry rules.

## Continue developing

- [Send Session audio to an external system](../guides/connectors.md)
- [Troubleshoot capture, delivery, and shutdown](../troubleshooting.md)
- [Read events, metrics, outcomes, and errors](../reference/events-and-errors.md)

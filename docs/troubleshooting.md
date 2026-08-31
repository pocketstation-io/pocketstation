# Troubleshoot capture, delivery, and shutdown

Start with the boundary that did not produce the expected result. Source
opening, route delivery, provider work, recording, and receiver playout have
separate outcomes.

## No application audio arrives

1. Call `discover_sources()` and confirm that the application is present and
   producing audio.
2. Select it by display name, application identifier, process instance, or
   discovered stable identity.
3. Check the operating-system permission for the process running PocketStation.
4. Inspect Session events for permission, source-open, and source-unavailable
   failures.

A successful Session start does not prove that every source has delivered a
frame. Use a finite media deadline and report a missing source separately.

## A destination misses frames

Inspect the route's capacity, queue depth, delivered frames, drops, and
discontinuities. Every destination has its own bounded `EdgeContract`. A slow
destination can apply pressure or lose frames according to that contract, but
it must not grow an unbounded queue or stall unrelated routes.

Move model inference, network waits, and file conversion away from capture
callbacks and realtime partitions.

## Application-owned audio is rejected

`AudioInput::try_write` accepts one complete frame matching the configured
sample format and frame size. Handle `Full`, `Closed`, `Cancelled`, and
`InvalidBuffer` separately. Retry `Full` only within an application-owned
deadline; otherwise drop or slow the producer according to the workflow's
policy.

## Generated speech continues after cancellation

Core cancellation removes matching output that is still inside Core's bounded
sender routes. It cannot recall packets already accepted by a Connector or
samples buffered by a receiver. Check Connector clearing, receiver playout
acknowledgement, and acoustic output separately before reporting an audible
result.

## Shutdown does not complete

Close application-owned inputs when no more frames will arrive. Call `stop()`
to drain accepted work, or `cancel()` when active asynchronous work must abort.
Then inspect the Session stop result, Endpoint failures, and recording outcome
to find the owner that did not join.

When reporting an issue, include the PocketStation version, operating system,
source selector, Session events, route metrics, and structured terminal error.

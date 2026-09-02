# Troubleshoot capture, delivery, and shutdown

Start with the component that did not produce the expected result. Source
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

If the application restarted, do not reuse its old process ID. Stop the current
Session, discover the source again, and create a new Session from the strongest
identity returned by discovery. PocketStation does not silently switch to a
different process or device.

## Permission changes are not reflected

`microphone_permission_observation()` never prompts. Treat `NotObservable` as
unknown and let the typed Source open result decide. On macOS, restart the host
application after changing screen-recording or microphone consent. On Windows
and Linux, verify that the current user or service owns access to the selected
audio session and device.

Do not add an automatic system-mix or default-device fallback after a denied
application Source. Ask the user to grant permission, select another Source,
or stop the workflow.

## A destination misses frames

Inspect the route's capacity, queue depth, delivered frames, drops, and
discontinuities. Every destination has finite delivery settings. A slow
destination can apply pressure or lose frames according to those settings, but
it must not grow an unbounded queue or stall unrelated routes. Advanced
packages can inspect the complete settings through `EdgeContract`.

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

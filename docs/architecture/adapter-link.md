# Adapter and Link — Session Boundary

## Boundary rule

```
Adapter opens Link.
Link owns live connection.
MediaClock enforces timing inside Link/Route where needed.
Relay forwards.
```

## Adapter

An Adapter knows how to connect to one external system. It holds configuration
and credentials but no live state.

Examples:
- `OpenAIAdapter` -- connects to OpenAI Realtime API
- `RelayAdapter` -- connects to a PocketStation relay via WebRTC
- `FileAdapter` -- reads/writes audio files
- `DeviceAdapter` -- wraps platform audio capture/output

## Link

A Link is a live connection opened by an Adapter. It owns the connection
lifecycle, sends and receives audio, and hosts MediaClock components.

A Link may use:
- `MediaClock::Ingress` for incoming audio (receive-side timing)
- `MediaClock::Egress` for outgoing audio (send-side timing)
- `Gate` for interruption handling (AI voice agents)
- `Trace` for timing proof

## Example: OpenAI voice agent

```
OpenAIAdapter opens OpenAILink.
OpenAILink uses MediaClock::Ingress(Contract::direct()) for user audio to model.
OpenAILink uses MediaClock::Egress(Contract::interactive()) for generated speech.
Gate handles barge-in: user interrupts -> Gate::interrupt(segment_id, played_ms).
Trace reports: ingress latency, egress pacing, gate flush events, drift ppm.
```

## Example: Music broadcast

```
FileAdapter opens FileLink.
FileLink uses MediaClock::Egress(Contract::fidelity()) for stereo output.
No Gate (music does not get interrupted by barge-in).
Trace reports: egress pacing, queue depth.
```

## MediaClock is not the session boundary

Adapter/Link is the session boundary. MediaClock is a timing tool used
inside Links and Routes. A Link without timing needs (e.g., a simple file
reader) may skip MediaClock entirely.

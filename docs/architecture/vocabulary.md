# Vocabulary — PocketStation v3.0

## Locked public vocabulary

These terms are the only words that appear in CLI, SDK, docs, and product API.

| Term | Definition |
|---|---|
| **Adapter** | Opens a Link to an external system (OpenAI, Twilio, file, device). |
| **Link** | A live connection owned by an Adapter. Carries audio in/out. |
| **Source** | An origin of audio frames (mic, app, file, network). |
| **Tap** | A read-only observation point on a Bus or Route. |
| **Stream** | A sequence of audio frames identified by StreamId. |
| **Route** | A directed path from Source to output through processing. |
| **Bus** | A named forwarding lane carrying one audio stream (voice, music, agent). |
| **Contract** | Declares timing behavior for a Route or Link (Direct, Interactive, etc.). |
| **MediaClock** | Standalone timing engine: Ingress, Egress, Drift, Gate, Trace. |
| **Trace** | Timing/recovery/pacing proof emitted by MediaClock components. |
| **Stem** | An isolated audio channel within a mix (e.g., vocals stem, drums stem). |
| **Mix** | A combination of multiple Stems or Buses into one output. |

## Banned public vocabulary

These terms must never appear in CLI, SDK, docs, or product API.
Historical references in migration notes are the only exception.

| Banned term | Replacement |
|---|---|
| `Room` | `GraphSession` |
| `listener` / `subscriber` | `BusSubscription` |
| `Track` | `AudioBus` |
| `routes` map | `RouteTable` |
| `Flow` | `Route` |
| `ReceiverBackend` | `Contract` |
| `ReceiverMode` | `Contract` |
| `SendMode` / `ReceiveMode` | `Contract` |
| `PocketEQ` (as umbrella product) | `MediaClock` |
| `pks-pocketeq` | `media-clock` |
| `PocketEqEngine` | `MediaClock` |
| `PocketEqConfig` / `PocketEqMetrics` | `Contract` / `ClockTrace` |
| `AiRaw` / `AiStreaming` | `Contract::Direct` |
| `HumanVoicePocketEq` | `Contract::Conversational` |
| `TtsBurstPaced` | `Contract::Interactive` |
| `MusicStereoSafe` | `Contract::Fidelity` |
| `BroadcastSmooth` | `Contract::Continuity` |
| `Transceiver` (as public product word) | `Adapter` + `Link` |

## Internal-only escape hatch

These terms may appear in internal implementation but never in CLI, docs, or product API:

```
TimingPlan, ClockPolicy, WebRtcTraceAdapter,
TransportSession, GraphIR, RuntimePlan
```

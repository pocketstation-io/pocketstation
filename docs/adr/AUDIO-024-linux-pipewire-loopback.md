# AUDIO-024 — Linux System Audio Loopback via PipeWire / ALSA snd-aloop

## Status

Accepted and merged. Phase 3, Wave C. linux.rs, four-way platform split, and Wave C GWT tests landed in main.

## Context

PocketStation needs system audio loopback on Linux to complete the three-platform capture story
(Wave A: macOS ScreenCaptureKit, Wave B: Windows WASAPI, Wave C: Linux PipeWire/ALSA).

Linux has two practical loopback mechanisms:

1. **PipeWire** — the modern Linux audio server (replaces PulseAudio + JACK on most desktop
   distros since 2021). The `pw::stream` API can open a sink-monitor input, which taps the
   system-wide output mix with low latency (~2–3 ms quantum). Available whenever
   `$XDG_RUNTIME_DIR/pipewire-0` exists.

2. **ALSA snd-aloop** — the kernel `snd-aloop` module creates a loopback sound card.
   Requires `sudo modprobe snd-aloop` (or persistent configuration). Capture side is
   `hw:Loopback,1,0`. Latency depends on the period size configured.

The PipeWire Rust crate (`pipewire = "0.10"`, GStreamer team) is not `Send`/`Sync`, so all
PipeWire objects must be owned by a single OS thread for their entire lifetime.

## Decision

**Primary backend:** PipeWire via `pw::stream` with sink-monitor capture.

**Fallback backend:** ALSA `snd-aloop` (`hw:Loopback,1,0`) when the PipeWire socket is absent.

**Thread architecture:**

```
PipeWire process callback (RT, owned by pks-pipewire-capture thread)
  |-- AudioBufferPool::acquire()  [lock-free CAS]
  |-- ptr::copy_nonoverlapping   [no alloc]
  |-- mpsc::SyncSender::try_send  [non-blocking; drops frame if full]
       |
       v
pks-pipewire-dispatch thread -- calls user callback
```

**Stop mechanism:** A 50 ms repeating timer attached to the `MainLoop` via `loop_().add_timer`
polls a `mpsc::sync_channel<()>`. When the stop signal arrives the timer closure calls
`mainloop.quit()`. This avoids `MainLoop::run()` blocking indefinitely on drop.

**CaptureMode support:**

| Mode | PipeWire | ALSA |
|---|---|---|
| `SystemMix` | Sink-monitor (`STREAM_CAPTURE_SINK = "true"`) | `hw:Loopback,1,0` |
| `Application(name)` | `properties! { NODE_NAME => name }` — links that node only | `ModeUnsupported` |
| `Process(pid)` | `ModeUnsupported` | `ModeUnsupported` |

**PipeWire stream constants:**

- `PW_NODE_LATENCY = "128/48000"` (~2.67 ms quantum at 48 kHz)
- `StreamFlags::AUTOCONNECT | MAP_BUFFERS | RT_PROCESS`
- Format: `F32LE`, 48 kHz, stereo

**Cargo dependencies (Linux only):**

```toml
[target.'cfg(target_os = "linux")'.dependencies]
pipewire = "0.10"
alsa = "0.9"
```

## Options considered

A) **JACK** — requires manual user setup; not suitable as a default.

B) **PulseAudio** — superseded by PipeWire on modern distros; PipeWire presents a PulseAudio
   compatibility layer so PipeWire covers both.

C) **PipeWire only, no ALSA fallback** — would fail on headless servers and older kernels.
   Rejected: ALSA snd-aloop fallback costs ~30 lines and covers the remaining cases.

Chosen: **PipeWire primary + ALSA snd-aloop fallback** (option A rejected, B covered by PipeWire
compatibility layer, C extended with fallback).

## Consequences

- `linux.rs` compiled only on `#[cfg(target_os = "linux")]`.
- Hot path: no alloc, lock, log, or panic in the PipeWire process callback.
- `pipewire_available()` performs one `stat(2)` on `$XDG_RUNTIME_DIR/pipewire-0`; safe to call
  before any threads are spawned.
- `SystemLoopbackSource::drop` sends a stop signal; the PipeWire thread exits within ~50 ms.
- ALSA path: callback is called inline on the capture thread (no dispatch thread needed;
  a dummy dispatch thread is spawned for uniform `SystemLoopbackSource` struct layout).
- CI: Linux runner required for real PipeWire validation. Current status: PARTIAL (code complete;
  CI runner with PipeWire daemon pending).

## Test plan

- `cargo test -p pocketstation-loopback` passes on macOS CI (4 new Wave C GWT tests are
  platform-independent or guarded with `#[cfg(not(target_os = "linux"))]`).
- Real-device validation: Linux desktop with PipeWire + `DYLD_LIBRARY_PATH` workaround for
  macOS CI rpath issue.
- `given_pw_node_latency_const_when_parsed_then_numerator_is_128_and_denominator_is_48000` —
  validates the `PW_NODE_LATENCY` constant format.
- `given_non_linux_host_when_pipewire_socket_path_checked_then_does_not_exist` — validates
  socket probe logic on non-Linux hosts.

## Reversal trigger

A future Linux audio server replaces PipeWire (unlikely within v2.3 timeframe), or
the `pipewire = "0.10"` crate API changes incompatibly before a stable 1.0 release.

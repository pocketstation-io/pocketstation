# AUDIO-023-windows-wasapi-loopback -- Windows WASAPI Loopback Capture

## Status

Accepted for v2.3. Phase 4 (Wave B of system-audio loopback roadmap).

## Context

PocketStation's `any audio -> any route -> any output` thesis requires capturing
system audio on Windows in addition to macOS (AUDIO-022, ScreenCaptureKit).
Windows exposes two WASAPI loopback surfaces:

1. **System-mix loopback** (Vista+): initialize the default *render* endpoint
   with `Direction::Capture`; the WASAPI driver adds
   `AUDCLNT_STREAMFLAGS_LOOPBACK` automatically.  Captures the entire system
   mix.

2. **Process-specific loopback** (Windows 10 2004+ / build 19041): use
   `ActivateAudioInterfaceAsync` with
   `AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK` (exposed in the `wasapi`
   crate as `AudioClient::new_application_loopback_client(pid, include_tree)`).
   Captures audio from a single process (and optionally its child processes).

The implementation must satisfy the PocketStation hot-path invariants:
no allocation, locking, logging, or panicking in the audio delivery callback.

## Decision

Implement `src/windows.rs` in the `pocketstation-loopback` crate with:

- **`CaptureMode::SystemMix`** -> `run_system_loopback`: `DeviceEnumerator` ->
  default render device -> `initialize_client` with `Direction::Capture` and
  `StreamMode::EventsShared`.
- **`CaptureMode::Process(pid)`** -> `run_process_loopback`:
  `AudioClient::new_application_loopback_client(pid, false)` (do not capture
  child processes by default).
- Event-driven capture via `set_get_eventhandle` + `wait_for_event`.
- **MMCSS scheduling**: `AvSetMmThreadCharacteristicsW(w!("Audio"), ...)` +
  `timeBeginPeriod(1)` on the dedicated capture thread.  Non-fatal if it fails.
- **Dedicated capture thread** (`std::thread::Builder`) spawned in
  `SystemLoopbackSource::capture_mode`.  Stop mechanism: `SyncSender<()>` in
  `Drop`; thread exits after the current `wait_for_event` (max 200 ms).
- **Lock-free `AudioBufferPool`** from `pocketstation-frame`.  Pool slot
  acquired via atomic CAS in `deliver_packet`; no allocation on hot path.
- Stack-allocated raw buffer (`[0u8; CAPTURE_FRAME_SAMPLES * 4 * 2]`) sized
  for two 20 ms stereo f32 frames.
- Output: 48 kHz stereo f32, matching `DEFAULT_SAMPLE_RATE` /
  `DEFAULT_SLOT_SAMPLES_MONO_20MS` from `pocketstation-frame`.
- `CaptureMode::Application(String)` is **not wired** (returns
  `ModeUnsupported`); AUDIO-022 app-bundle mode is macOS-only.

### Known WASAPI bug -- process-loopback period

When an `AudioClient` is created via `new_application_loopback_client`,
`get_device_period` returns `Not implemented` and `get_buffer_size` returns
garbage (e.g. 3,131,961,357).  The period passed to `initialize_client` is
documented as irrelevant in this mode.  We use
`WASAPI_PROCESS_LOOPBACK_PERIOD_100NS = 100_000` (10 ms in 100-ns units) as a
safe non-zero placeholder.

## Options considered

A) **CPAL** (`cpal` crate) -- no loopback support; only microphone input.
   Rejected.

B) **VB-Audio Virtual Cable** -- requires user installation of a third-party
   driver.  Not self-contained.  Rejected.

C) **WaveRT kernel streaming** -- lower latency but requires a kernel driver
   and is not available from safe Rust.  Deferred to Phase 6 (AUDIO-019).

D) **wasapi 0.23 WASAPI bindings** -- safe Rust, covers Vista+ system loopback
   and Win10 2004+ process loopback, actively maintained.  Chosen.

## Consequences

- `wasapi = "0.23"` added as a Windows-only dependency to
  `pocketstation-loopback/Cargo.toml`.
- `lib.rs` gains `CaptureMode` enum (identical to Wave A definition for
  trivial future merge) and two new `LoopbackError` variants: `BackendInit`
  and `ModeUnsupported`.
- Platform split: `#[cfg(target_os = "macos")]` / `#[cfg(target_os = "windows")]` /
  `#[cfg(not(any(target_os = "macos", target_os = "windows")))]`.
- CLI `source.rs` gains `--pid` flag on `SystemArgs`; `run_system` supports
  both macOS and Windows via `capture_with_mode`.
- Compilation on Windows must be verified on a Windows host or in a Windows
  CI runner; macOS build only confirms the platform-split compiles cleanly.
- `FAKE_SCAFFOLD_INVENTORY.md`: Windows row upgraded from STUB to PARTIAL.
  Linux (PipeWire) remains STUB / Wave C.

# Echo cancellation

The development checkout includes `Session::echo_cancel` in the default
`echo-cancellation` feature. It uses WebRTC audio processing internally. No
example-package dependency or factory registration is required. This API is
unreleased and physical acoustic qualification is incomplete.

## Select the two inputs

```rust,no_run
use pocketstation::{AudioInputConfig, PlaybackReference, SampleFormat, SampleSpec, Session};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let format = SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved);
let reference = session.audio_input(AudioInputConfig::new(format, 8, 960)?)?;
let microphone = session.audio_input(AudioInputConfig::new(format, 8, 960)?)?;
let processed = session.echo_cancel(
    microphone.output(),
    PlaybackReference::rendered_audio(reference.output()),
)?;
processed.audio().send(session.polled_audio()?)?;
let mut running = session.start()?;
// Supply microphone and actual playback PCM through the AudioInput writers.
let observation = processed.observations();
let _ = running.stop();
# Ok(())
# }
```

`PlaybackReference::selected_application` declares that only the selected
application's audio is available. `output_mix` declares a separately authorized
and captured playback mix. `rendered_audio` declares caller-owned playback PCM.
These selections never open additional capture devices or applications. They do
not verify that the selected audio is the complete speaker signal.

The original microphone and application remain independently routable. App-only
capture does not require declaring echo cancellation or opening a microphone.
The processed stem identifies a new derived stream; `microphone_origin` and
`reference_origin` retain its declared input relationships. Observations retain
the actual microphone and reference source IDs after processing starts.

## Current input requirements

- Both inputs use the Session's 48 kHz interleaved float format, with matching
  mono or stereo channel counts and 10 ms or 20 ms frames.
- Source timestamps must be in the same declared clock domain. Core admits
  render and microphone frames independently; it neither splices reference
  samples nor requires an extra reference frame at EOF. The engine estimates
  how playback appears in the microphone from those streams. Separate hardware
  clocks are not automatically mapped or resampled.
- Per-source timestamp residuals up to 1 ms do not reset adaptation. They are
  reported as `maximum_cadence_error_ns`. A larger timestamp jump, changed source
  identity, sequence gap or discontinuity epoch resets the engine and discards
  queued frames with counters. This tolerance is not hardware-drift qualification.
- Stereo reference channels remain separate; opposite-polarity playback is
  exercised by the Session regression test.
- Missing reference is held for at most 80 ms of unmatched microphone samples.
  Exhausting that frame limit fails the processor; it does not return raw samples
  labelled as cancelled audio. This is a retained-audio limit, not a wall-clock
  timeout when inputs stop arriving.
- A reference start more than 80 ms ahead of a pending microphone frame fails
  explicitly. Once processing starts, a reference older than 80 ms holds pending
  microphone audio within the same 80 ms queue; exhaustion fails visibly.
  Missing reference samples are never replaced with invented silence.

These requirements currently limit the API to prepared PCM inputs. They are not
a qualification of arbitrary application and microphone device pairs. Reference
acquisition, clock adaptation and acoustic recovery are continuing Core work.

## Observe processing and shutdown

`observations()` reports queue depths, discarded frames, resets, processing
durations, processing generation, reference age/lead and the last error.
`analyzed_reference_frames_total` counts real frames admitted to the engine.
`Processing` means the engine ran; it does not assert convergence or measured
echo removal. Processing errors remain observable after shutdown; late native
replies cannot erase a terminal error. An interrupted request increments
`interrupted_requests_total` and reports `Interrupted` until shutdown completes.
An interruption can result from stop or a deadline; it does not by itself prove
an engine failure. An independent application branch keeps delivering audio
when echo processing fails.

Preparation establishes the native capture and render formats, then clears its
initialization audio before accepting actual input. Initialization samples never
enter source counts or output audio. `Session.stop()` drains admitted operator
input and calls flush; `cancel()` discards queued work. Closing individual source
writers does not finish Session-owned routing queues.

Native work runs on one dedicated thread per instance. The Session executor
awaits results; capture callbacks do not run the engine. Commands and retained
audio have fixed queue limits. A native call cannot be forcibly interrupted;
executor timeout is not a promise to terminate hung native code. Normal shutdown
closes the command queue and joins its worker outside the Session executor.

Output timestamps retain the microphone input interval. The qualified
algorithmic-delay field is `None`; measured processing duration is CPU execution
time, not acoustic or algorithmic delay. Do not use it to align a transcript or
claim end-to-end latency. The complete native filter response at EOF is not yet
delivered. Derived audio has its own source identity; this handle retains the
microphone relationship, but per-frame derivation is not retained through audio
reentry and recording. These limitations are open release gates. Simultaneous
human speech, physical speaker echo,
device changes and additional operating systems still require qualification.

## Build requirements

The default native dependency requires a C/C++ toolchain, Meson, Ninja,
pkg-config, libclang and Rust's `llvm-tools` component. Its build can download
checksum-pinned Abseil source. A configured dependency cache supports offline
builds. Minimal `--no-default-features` builds omit this API and engine.

The Core implementation is in `src/aec`; `tests/session_aec.rs` exercises the
normal Session API with deterministic PCM and the actual native engine. Those
tests establish software integration, not physical acoustic quality.

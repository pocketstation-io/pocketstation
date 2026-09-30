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
- Source timestamps must be in the same clock domain, with at most 1 ms
  first-sample skew. Separate hardware clocks are not automatically corrected.
- Stereo reference channels remain separate; opposite-polarity playback is
  exercised by the Session regression test.
- Missing reference is held for at most 80 ms of unmatched microphone samples.
  Exhausting that frame limit fails the processor; it does not return raw samples
  labelled as cancelled audio. This is a retained-audio limit, not a wall-clock
  timeout when inputs stop arriving.

These requirements currently limit the API to prepared PCM inputs. They are not
a qualification of arbitrary application and microphone device pairs. Reference
acquisition, clock adaptation and acoustic recovery are continuing Core work.

## Observe processing and shutdown

`observations()` reports queue depths, discarded frames, resets, processing
durations, processing generation and the last error. `Processing` means that the
engine ran; it does not assert convergence or measured echo removal. Processing
errors remain observable after shutdown. An independent application branch keeps
delivering audio after the echo processor fails.

Native work runs on one dedicated thread per instance. The Session executor
awaits results; capture callbacks do not run the engine. Commands and retained
audio have fixed queue limits. A native call cannot be forcibly interrupted;
executor timeout is not a promise to terminate hung native code. Normal shutdown
closes the command queue and joins its worker outside the Session executor.

Output timestamps retain the microphone input interval. The qualified
algorithmic-delay field is `None`; measured processing duration is CPU execution
time, not acoustic or algorithmic delay. Do not use it to align a transcript or
claim end-to-end latency. Simultaneous human speech, physical speaker echo,
device changes and additional operating systems still require qualification.

## Build requirements

The default native dependency requires a C/C++ toolchain, Meson, Ninja,
pkg-config, libclang and Rust's `llvm-tools` component. Its build can download
checksum-pinned Abseil source. A configured dependency cache supports offline
builds. Minimal `--no-default-features` builds omit this API and engine.

The Core implementation is in `src/echo_cancellation`; `tests/session_aec.rs` exercises the
normal Session API with deterministic PCM and the actual native engine. Those
tests establish software integration, not physical acoustic quality.

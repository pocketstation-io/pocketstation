# Write application-owned audio into a Session

Use `Session::audio_input` when your application already owns PCM, such as
generated speech, decoded call audio, or media received from another SDK. The
input becomes a normal source and can be recorded, polled, processed, or sent
to a Connector.

## Declare the input and its routes

The input format must match the Session format. This example declares mono
48 kHz `f32` audio with 10 ms frames and space for eight frames:

```rust,no_run
use pocketstation::{
    AudioFrameDuration, AudioInputConfig, SampleFormat, SampleSpec, Session,
};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let sample_spec = SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved);
let session = Session::builder()
    .sample_spec(sample_spec)
    .audio_frame_duration(AudioFrameDuration::Ms10)
    .recording_root("recordings")
    .build();
let configuration = AudioInputConfig::new(sample_spec, 8, 480)?;
let mut assistant = session.audio_input(configuration)?;

assistant.output().record("assistant")?;
assistant.output().send(session.polled_audio()?)?;
let mut running = session.start()?;
# assistant.close();
# let _ = running.stop();
# Ok(())
# }
```

Declare routes before starting the Session. The source, stream, stem, sequence,
timing, and discontinuity identities are assigned by PocketStation.

## Write complete frames

`try_write` accepts one complete interleaved frame and never waits for space.
Handle `AudioInputWriteErrorKind::Full` as backpressure: retry only within your
own finite deadline, drop according to application policy, or slow the
producer. Closed, cancelled, and invalid frames are separate outcomes.

For fewer copies, acquire one preallocated input buffer with `try_acquire`, fill
it, and submit it with `try_send`. The buffer remains owned by the input until
submission succeeds or returns it in an error.

## Cancel generated speech without stopping capture

Generated speech may become irrelevant after an interruption. Begin an owned
output, write its frames through `try_write_for_output`, and cancel that output
when it is no longer current. Core discards matching frames that remain in its
bounded sender routes while microphone capture, recording, and unrelated
outputs continue.

Cancellation cannot recall packets already sent to a transport or samples
already buffered by a receiver. A Connector or receiver must expose its own
clear-playout capability before an application can claim end-to-end audible
cancellation.

## Close and inspect the input

Call `close` when no more frames will arrive. Accepted frames drain during
normal Session shutdown. Inspect `observations`,
`discarded_output_frames_total`, and `cancelled_output_writes_total` to
distinguish normal completion, pressure, and cancellation.

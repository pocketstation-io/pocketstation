# Transcribe a WAV file with whisper.cpp

This example sends a mono WAV payload through a PocketStation Operator that
runs `whisper-cli` outside the realtime lane. The output is text with the input
sequence number and timestamp preserved.

## Prerequisites

- a local `whisper-cli` installation;
- a whisper.cpp-compatible GGML model;
- a 16-bit, mono, 16 kHz WAV file.

On macOS, install the executable with Homebrew:

```bash
brew install whisper-cpp
```

PocketStation does not download or own the model. Obtain it from the upstream
whisper.cpp project and review its license before use.

## Run

```bash
cargo run -p whisper-transcribe-example -- \
  "$(command -v whisper-cli)" \
  <model-file> \
  <audio-file>
```

The process prints the transcript returned by `whisper-cli`. CPU inference is
the default. Call `with_gpu(true)` only after verifying the accelerator backend
on the target host.

This example verifies subprocess Operator composition. It is not a live
capture, streaming transcription, or performance claim.

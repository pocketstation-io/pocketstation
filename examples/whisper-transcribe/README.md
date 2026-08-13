# Local Whisper connector example

This example keeps Whisper outside PocketStation's first-party crates. It accepts
a 16-bit, mono, 16 kHz WAV as `SignalPayload::Bytes`, invokes the local
`whisper-cli` process after the realtime Bridge, and emits `SignalPayload::Text`
with the input sequence number and timestamp preserved.

CPU inference is the deterministic default. Call `with_gpu(true)` only after
validating the installed whisper.cpp accelerator backend on the target machine.

```sh
brew install whisper-cpp
cargo run -p whisper-transcribe-example -- \
  "$(command -v whisper-cli)" \
  /path/to/ggml-tiny.en.bin \
  /path/to/mono-16khz.wav
```

The model is intentionally not downloaded or owned by PocketStation. Use a
whisper.cpp-compatible GGML model from the upstream project.

# Phase 2 Progress - audio-core

## Local Whisper connector example - 2026-07-13

- Added `examples/whisper-transcribe` as an example-owned `AsyncNode`; no provider dependency entered first-party crates.
- Binary WAV input crosses the async boundary and text output preserves sequence/timestamp lineage.
- Missing process/model and subprocess crashes fail visibly.
- Real whisper.cpp tiny English E2E passed in CPU mode with a 3.84-second spoken fixture; measured wall time was 1.08 seconds.
- GPU remains explicit opt-in because Homebrew whisper.cpp 1.9.1 crashed in the Metal backend on this machine.

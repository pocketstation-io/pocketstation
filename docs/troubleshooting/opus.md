# Opus conversion fails

<!-- claims: CLM-TRBL-011-SCOPE-001,CLM-TRBL-011-TEXT-001,CLM-TRBL-011-TEXT-002,CLM-TRBL-011-TEXT-003,CLM-TRBL-011-TEXT-004,CLM-TRBL-011-TEXT-005,CLM-TRBL-011-TEXT-006,CLM-TRBL-011-SOURCE-001 -->

## Symptom

Opus encoder or decoder construction or conversion returns an error.

## Evidenced causes

- The sample rate, channels, frame duration, or bitrate is unsupported.
- PCM length does not match the configured frame shape.
- A packet is invalid for the decoder or codec state reports an Opus failure.

## Distinguish the causes

Compare the configured profile with the input frame's `SampleSpec`, samples per channel, packet bounds, and encoder or decoder error variant.

## Diagnostic signals

- `pocketstation::codec::decoder::OpusDecodeError` (`error-3d9be3e3f583928d23f4`)
- `pocketstation::codec::decoder::OpusDecodeError` / `FrameDurationExceedsConfiguredMaximum` (`error-84f459285f24936c9e00`)
- `pocketstation::codec::decoder::OpusDecodeError` / `Opus` (`error-e714868780d8cd7a5a64`)
- `pocketstation::codec::encoder::OpusEncodeError` (`error-eda36a61ae1109dce21c`)
- `pocketstation::codec::encoder::OpusEncodeError` / `InvalidFrameSampleCount` (`error-bef40f89fba6bbf83b1c`)
- `pocketstation::codec::encoder::OpusEncodeError` / `Opus` (`error-0100bb86433fb5001cce`)

## Executable evidence

- `given_encoded_opus_packet_when_decoded_then_contains_960_samples` exercises given encoded opus packet when decoded then contains 960 samples under its recorded setup (`test-18e769bfba9f736148f1`).
- `given_20ms_opus_frame_when_sampled_at_48khz_then_contains_960_samples` exercises given 20ms opus frame when sampled at 48khz then contains 960 samples under its recorded setup (`test-c6b1615ef4535f8bea99`).
- `given_30s_sine_pcm_when_opus_round_trip_then_golden_file_invariants_pass` exercises given 30s sine pcm when opus round trip then golden file invariants pass under its recorded setup (`test-77bb4d2c9b220a4d7130`).
- `given_sine_wave_when_opus_round_trip_runs_then_approximate_magnitude_is_preserved` exercises given sine wave when opus round trip runs then approximate magnitude is preserved under its recorded setup (`test-8c9746a0022eeccced89`).
- `given_20ms_decoder_when_60ms_concealment_is_requested_then_typed_bound_error_is_returned` exercises given 20ms decoder when 60ms concealment is requested then typed bound error is returned under its recorded setup (`test-f9eb27f8c697619303a9`).
- `given_mono_decoder_when_concealing_10ms_then_480_samples_are_appended` exercises given mono decoder when concealing 10ms then 480 samples are appended under its recorded setup (`test-f3d9f0899054eb532922`).
- `given_presized_output_when_concealing_repeatedly_then_capacity_stays_fixed` exercises given presized output when concealing repeatedly then capacity stays fixed under its recorded setup (`test-0a190981712f34057776`).
- `given_stereo_decoder_when_concealing_20ms_then_1920_samples_are_appended` exercises given stereo decoder when concealing 20ms then 1920 samples are appended under its recorded setup (`test-fa484e4be3d65e78f9fc`).
- `given_60ms_stereo_configuration_when_exact_frame_arrives_then_fixed_scratch_accepts_it` exercises given 60ms stereo configuration when exact frame arrives then fixed scratch accepts it under its recorded setup (`test-0c8f77a1048c94269c79`).
- `given_960_sample_frame_when_encoded_then_packet_is_not_empty` exercises given 960 sample frame when encoded then packet is not empty under its recorded setup (`test-751e9c684179c8d414af`).
- `given_configured_20ms_encoder_when_10ms_frame_arrives_then_exact_duration_is_enforced` exercises given configured 20ms encoder when 10ms frame arrives then exact duration is enforced under its recorded setup (`test-5183ec8e78897fa604e5`).
- `given_optimised_encode_when_same_input_then_packet_bytes_identical` exercises given optimised encode when same input then packet bytes identical under its recorded setup (`test-afeb7f4f7006561b144e`).
- `given_optimised_pipeline_when_round_trip_then_snr_above_minus_1db` exercises given optimised pipeline when round trip then snr above minus 1db under its recorded setup (`test-ce093aae4da89b29c34b`).
- `given_oversized_frame_when_encoded_then_error_is_typed_and_output_is_cleared` exercises given oversized frame when encoded then error is typed and output is cleared under its recorded setup (`test-655f8034c539e1dc9c27`).
- `given_partial_stereo_frame_when_encoded_then_error_is_typed` exercises given partial stereo frame when encoded then error is typed under its recorded setup (`test-9e747f83ae1e676c3e7e`).

## Corrective action

Recreate codec state with a supported profile or supply a frame or packet that matches the existing state.

## Retry and incomplete state

Do not assume the same state can be replayed after a codec failure. The failing packet or frame may not have produced output.

## Related reference

- [Opus Codec](/docs/concepts/opus-codec.md)
- [Codec](/docs/reference/codec.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Frame and codec failures](/docs/errors/frames-and-codec.md)
- [Encode and decode Opus](/docs/how-to/opus-roundtrip.md)
- [A capture source disappears](/docs/troubleshooting/source-loss.md)
- [Error and status model](/docs/concepts/error-model.md)

## Evidence boundary

The claims on **Opus conversion fails** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/codec/encoder.rs:6-6` (`DIRECT`)
- `src/codec/encoder.rs:6-6` (`DIRECT`)
- `src/codec/encoder.rs:6-6` (`DIRECT`)
- `src/codec/encoder.rs:7-12` (`DIRECT`)
- `src/codec/encoder.rs:8-8` (`DIRECT`)
- `src/codec/encoder.rs:9-9` (`DIRECT`)
- `src/codec/encoder.rs:10-10` (`DIRECT`)
- `src/codec/encoder.rs:11-11` (`DIRECT`)
- `src/codec/encoder.rs:15-22` (`DIRECT`)
- `src/codec/encoder.rs:26-26` (`DIRECT`)
- `src/codec/encoder.rs:26-26` (`DIRECT`)
- `src/codec/encoder.rs:26-26` (`DIRECT`)
- `src/codec/encoder.rs:27-30` (`DIRECT`)
- `src/codec/encoder.rs:28-28` (`DIRECT`)
- `src/codec/encoder.rs:29-29` (`DIRECT`)
- `src/codec/encoder.rs:33-38` (`DIRECT`)
- `src/codec/encoder.rs:43-43` (`DIRECT`)
- `src/codec/encoder.rs:43-43` (`DIRECT`)
- `src/codec/encoder.rs:43-43` (`DIRECT`)
- `src/codec/encoder.rs:44-46` (`DIRECT`)
- `src/codec/encoder.rs:45-45` (`DIRECT`)
- `src/codec/encoder.rs:49-53` (`DIRECT`)
- `src/codec/encoder.rs:57-57` (`DIRECT`)
- `src/codec/encoder.rs:57-57` (`DIRECT`)
- `src/codec/decoder.rs:15-22` (`DIRECT`)
- `src/codec/decoder.rs:16-16` (`DIRECT`)
- `src/codec/decoder.rs:20-20` (`DIRECT`)
- `src/codec/decoder.rs:21-21` (`DIRECT`)
- `src/codec/decoder.rs:24-24` (`DIRECT`)
- `src/codec/decoder.rs:24-24` (`DIRECT`)
- `src/codec/decoder.rs:24-24` (`DIRECT`)
- `src/codec/decoder.rs:24-24` (`DIRECT`)
- `src/codec/decoder.rs:25-35` (`DIRECT`)
- `src/codec/decoder.rs:29-32` (`DIRECT`)
- `src/codec/decoder.rs:30-30` (`DIRECT`)
- `src/codec/decoder.rs:31-31` (`DIRECT`)
- `src/codec/decoder.rs:34-34` (`DIRECT`)
- `src/codec/decoder.rs:34-34` (`DIRECT`)
- `src/codec/decoder.rs:39-41` (`DIRECT`)
- `src/codec/decoder.rs:44-46` (`DIRECT`)
- `src/codec/decoder.rs:53-66` (`DIRECT`)
- `src/codec/decoder.rs:81-109` (`DIRECT`)
- `src/codec/decoder.rs:116-150` (`DIRECT`)
- `src/codec/decoder.rs:153-172` (`DIRECT`)
- `src/codec/decoder.rs:175-177` (`DIRECT`)

For **Opus conversion fails**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

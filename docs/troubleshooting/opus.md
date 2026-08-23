# Opus conversion fails

<!-- claims: CLM-TRBL-011-CAP-001,CLM-TRBL-011-CAP-002,CLM-TRBL-011-CAP-003,CLM-TRBL-011-SOURCE-001 -->

## Symptom

Opus encoder or decoder construction or conversion returns an error.

## Evidenced causes

- The sample rate, channels, frame duration, or bitrate is unsupported.
- PCM length does not match the configured frame shape.
- A packet is invalid for the decoder or codec state reports an Opus failure.

## Distinguish the causes

Compare the configured profile with the input frame's `SampleSpec`, samples per channel, packet bounds, and encoder or decoder error variant.

## Diagnostic signals

- `pocketstation::codec::decoder::OpusDecodeError` (`error-9b6a20dfec56d0f963ec`)
- `pocketstation::codec::decoder::OpusDecodeError` / `FrameDurationExceedsConfiguredMaximum` (`error-4055838a830f20f7900a`)
- `pocketstation::codec::decoder::OpusDecodeError` / `Opus` (`error-7b6f20bfd81327986061`)
- `pocketstation::codec::encoder::OpusEncodeError` (`error-ae09263b8f4f85f0d5e8`)
- `pocketstation::codec::encoder::OpusEncodeError` / `InvalidFrameSampleCount` (`error-edbece7c0fc9e4199d02`)
- `pocketstation::codec::encoder::OpusEncodeError` / `Opus` (`error-3beedf48b3ab09500606`)

## Executable evidence

- `given_encoded_opus_packet_when_decoded_then_contains_960_samples` exercises given encoded opus packet when decoded then contains 960 samples under its recorded setup (`test-e9a038c59aa6148e49f9`).
- `given_20ms_opus_frame_when_sampled_at_48khz_then_contains_960_samples` exercises given 20ms opus frame when sampled at 48khz then contains 960 samples under its recorded setup (`test-c4d783dc09c5ae8fcc9c`).
- `given_30s_sine_pcm_when_opus_round_trip_then_golden_file_invariants_pass` exercises given 30s sine pcm when opus round trip then golden file invariants pass under its recorded setup (`test-d58d461257beb4e70632`).
- `given_sine_wave_when_opus_round_trip_runs_then_approximate_magnitude_is_preserved` exercises given sine wave when opus round trip runs then approximate magnitude is preserved under its recorded setup (`test-a80dbf6b40aa2cc2df6c`).
- `given_20ms_decoder_when_60ms_concealment_is_requested_then_typed_bound_error_is_returned` exercises given 20ms decoder when 60ms concealment is requested then typed bound error is returned under its recorded setup (`test-f2b28e6d34edfbf95af0`).
- `given_mono_decoder_when_concealing_10ms_then_480_samples_are_appended` exercises given mono decoder when concealing 10ms then 480 samples are appended under its recorded setup (`test-7f059c025029ab48d42b`).
- `given_presized_output_when_concealing_repeatedly_then_capacity_stays_fixed` exercises given presized output when concealing repeatedly then capacity stays fixed under its recorded setup (`test-3a6668288d371a23c543`).
- `given_stereo_decoder_when_concealing_20ms_then_1920_samples_are_appended` exercises given stereo decoder when concealing 20ms then 1920 samples are appended under its recorded setup (`test-a33c833b56c360107d03`).
- `given_60ms_stereo_configuration_when_exact_frame_arrives_then_fixed_scratch_accepts_it` exercises given 60ms stereo configuration when exact frame arrives then fixed scratch accepts it under its recorded setup (`test-27e8b931577e02ce30ec`).
- `given_960_sample_frame_when_encoded_then_packet_is_not_empty` exercises given 960 sample frame when encoded then packet is not empty under its recorded setup (`test-6bdc61a7c604b08c13a4`).
- `given_configured_20ms_encoder_when_10ms_frame_arrives_then_exact_duration_is_enforced` exercises given configured 20ms encoder when 10ms frame arrives then exact duration is enforced under its recorded setup (`test-bf950e1644072a871d53`).
- `given_optimised_encode_when_same_input_then_packet_bytes_identical` exercises given optimised encode when same input then packet bytes identical under its recorded setup (`test-b46acca2f325d3bbcfb1`).
- `given_optimised_pipeline_when_round_trip_then_snr_above_minus_1db` exercises given optimised pipeline when round trip then snr above minus 1db under its recorded setup (`test-792afd9b5fceaa2832c6`).
- `given_oversized_frame_when_encoded_then_error_is_typed_and_output_is_cleared` exercises given oversized frame when encoded then error is typed and output is cleared under its recorded setup (`test-91992c707d12d6b613a9`).
- `given_partial_stereo_frame_when_encoded_then_error_is_typed` exercises given partial stereo frame when encoded then error is typed under its recorded setup (`test-807b1db710dc9ad5f27a`).

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

The claims on **Opus conversion fails** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/codec/encoder.rs:1-860` (`DIRECT`)
- `src/codec/decoder.rs:1-267` (`DIRECT`)

For **Opus conversion fails**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.

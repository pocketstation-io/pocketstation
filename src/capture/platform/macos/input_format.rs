//! Native microphone format selection and off-callback PCM normalization.

use crate::capture::{CaptureNativeFormat, CaptureSampleRepresentation};
use cpal::{SampleFormat, SupportedStreamConfig, SupportedStreamConfigRange};

pub(crate) const CANONICAL_INPUT_SAMPLE_RATE_HZ: u32 = 48_000;
pub(crate) const CANONICAL_INPUT_CHANNEL_COUNT: u8 = 1;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SelectedInputFormat {
    pub(crate) config: SupportedStreamConfig,
    pub(crate) native: CaptureNativeFormat,
    pub(crate) sample_size_bytes: usize,
}

pub(crate) fn select_input_config(
    configurations: impl IntoIterator<Item = SupportedStreamConfigRange>,
) -> Option<SelectedInputFormat> {
    configurations
        .into_iter()
        .filter_map(|range| {
            let sample_representation = sample_representation(range.sample_format())?;
            let sample_rate_hz = preferred_rate_in_range(&range);
            let config = range.try_with_sample_rate(sample_rate_hz)?;
            let native = CaptureNativeFormat {
                sample_rate_hz,
                channel_count: config.channels(),
                sample_representation,
            };
            let score = (
                sample_rate_hz.abs_diff(CANONICAL_INPUT_SAMPLE_RATE_HZ),
                u8::from(config.channels() != 1),
                representation_rank(sample_representation),
                config.channels(),
            );
            Some((score, config, native))
        })
        .min_by_key(|(score, _, _)| *score)
        .map(|(_, config, native)| SelectedInputFormat {
            sample_size_bytes: config.sample_format().sample_size(),
            config,
            native,
        })
}

fn preferred_rate_in_range(range: &SupportedStreamConfigRange) -> u32 {
    let minimum = range.min_sample_rate();
    let maximum = range.max_sample_rate();
    CANONICAL_INPUT_SAMPLE_RATE_HZ.clamp(minimum, maximum)
}

pub(crate) fn sample_representation(
    sample_format: SampleFormat,
) -> Option<CaptureSampleRepresentation> {
    match sample_format {
        SampleFormat::I8 => Some(CaptureSampleRepresentation::SignedInteger8),
        SampleFormat::I16 => Some(CaptureSampleRepresentation::SignedInteger16),
        SampleFormat::I24 => Some(CaptureSampleRepresentation::SignedInteger24),
        SampleFormat::I32 => Some(CaptureSampleRepresentation::SignedInteger32),
        SampleFormat::I64 => Some(CaptureSampleRepresentation::SignedInteger64),
        SampleFormat::U8 => Some(CaptureSampleRepresentation::UnsignedInteger8),
        SampleFormat::U16 => Some(CaptureSampleRepresentation::UnsignedInteger16),
        SampleFormat::U24 => Some(CaptureSampleRepresentation::UnsignedInteger24),
        SampleFormat::U32 => Some(CaptureSampleRepresentation::UnsignedInteger32),
        SampleFormat::U64 => Some(CaptureSampleRepresentation::UnsignedInteger64),
        SampleFormat::F32 => Some(CaptureSampleRepresentation::Float32),
        SampleFormat::F64 => Some(CaptureSampleRepresentation::Float64),
        SampleFormat::DsdU8 | SampleFormat::DsdU16 | SampleFormat::DsdU32 => None,
        _ => None,
    }
}

fn representation_rank(sample_representation: CaptureSampleRepresentation) -> u8 {
    match sample_representation {
        CaptureSampleRepresentation::Float32 => 0,
        CaptureSampleRepresentation::Float64 => 1,
        CaptureSampleRepresentation::SignedInteger32 => 2,
        CaptureSampleRepresentation::SignedInteger24 => 3,
        CaptureSampleRepresentation::SignedInteger16 => 4,
        CaptureSampleRepresentation::SignedInteger8 => 5,
        CaptureSampleRepresentation::SignedInteger64 => 6,
        CaptureSampleRepresentation::UnsignedInteger32 => 7,
        CaptureSampleRepresentation::UnsignedInteger24 => 8,
        CaptureSampleRepresentation::UnsignedInteger16 => 9,
        CaptureSampleRepresentation::UnsignedInteger8 => 10,
        CaptureSampleRepresentation::UnsignedInteger64 => 11,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputConversionError {
    MisalignedBuffer,
    OutputCapacity,
    UnsupportedRepresentation,
}

/// Converts native interleaved PCM into canonical mono `f32`.
///
/// The caller owns both fixed-capacity buffers. This function runs on the
/// reader worker, never in the operating-system audio callback.
pub(crate) fn decode_interleaved_to_mono(
    input: &[u8],
    format: CaptureNativeFormat,
    output: &mut [f32],
) -> Result<usize, InputConversionError> {
    let sample_size = representation_sample_size(format.sample_representation);
    let channels = usize::from(format.channel_count);
    if sample_size == 0 || channels == 0 {
        return Err(InputConversionError::UnsupportedRepresentation);
    }
    let bytes_per_frame = sample_size
        .checked_mul(channels)
        .ok_or(InputConversionError::MisalignedBuffer)?;
    if !input.len().is_multiple_of(bytes_per_frame) {
        return Err(InputConversionError::MisalignedBuffer);
    }
    let frame_count = input.len() / bytes_per_frame;
    if frame_count > output.len() {
        return Err(InputConversionError::OutputCapacity);
    }

    for (frame_index, frame) in input.chunks_exact(bytes_per_frame).enumerate() {
        let mut sum = 0.0f64;
        for encoded in frame.chunks_exact(sample_size) {
            sum += decode_sample(encoded, format.sample_representation)? as f64;
        }
        output[frame_index] = (sum / channels as f64) as f32;
    }
    Ok(frame_count)
}

pub(crate) const fn representation_sample_size(
    sample_representation: CaptureSampleRepresentation,
) -> usize {
    match sample_representation {
        CaptureSampleRepresentation::SignedInteger8
        | CaptureSampleRepresentation::UnsignedInteger8 => 1,
        CaptureSampleRepresentation::SignedInteger16
        | CaptureSampleRepresentation::UnsignedInteger16 => 2,
        CaptureSampleRepresentation::SignedInteger24
        | CaptureSampleRepresentation::SignedInteger32
        | CaptureSampleRepresentation::UnsignedInteger24
        | CaptureSampleRepresentation::UnsignedInteger32
        | CaptureSampleRepresentation::Float32 => 4,
        CaptureSampleRepresentation::SignedInteger64
        | CaptureSampleRepresentation::UnsignedInteger64
        | CaptureSampleRepresentation::Float64 => 8,
    }
}

fn decode_sample(
    bytes: &[u8],
    sample_representation: CaptureSampleRepresentation,
) -> Result<f32, InputConversionError> {
    let sample = match sample_representation {
        CaptureSampleRepresentation::SignedInteger8 => (bytes[0] as i8) as f32 / 128.0,
        CaptureSampleRepresentation::SignedInteger16 => {
            i16::from_ne_bytes([bytes[0], bytes[1]]) as f32 / 32_768.0
        }
        CaptureSampleRepresentation::SignedInteger24 => {
            i32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f32 / 8_388_608.0
        }
        CaptureSampleRepresentation::SignedInteger32 => {
            i32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64 as f32
                / 2_147_483_648.0
        }
        CaptureSampleRepresentation::SignedInteger64 => {
            i64::from_ne_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]) as f64 as f32
                / 9_223_372_036_854_775_808.0
        }
        CaptureSampleRepresentation::UnsignedInteger8 => (f32::from(bytes[0]) - 128.0) / 128.0,
        CaptureSampleRepresentation::UnsignedInteger16 => {
            (u16::from_ne_bytes([bytes[0], bytes[1]]) as f32 - 32_768.0) / 32_768.0
        }
        CaptureSampleRepresentation::UnsignedInteger24 => {
            (u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f32 - 8_388_608.0)
                / 8_388_608.0
        }
        CaptureSampleRepresentation::UnsignedInteger32 => {
            ((u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64
                - 2_147_483_648.0)
                / 2_147_483_648.0) as f32
        }
        CaptureSampleRepresentation::UnsignedInteger64 => {
            ((u64::from_ne_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]) as f64
                - 9_223_372_036_854_775_808.0)
                / 9_223_372_036_854_775_808.0) as f32
        }
        CaptureSampleRepresentation::Float32 => {
            f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        }
        CaptureSampleRepresentation::Float64 => f64::from_ne_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]) as f32,
    };
    Ok(sample)
}

/// Streaming linear converter from one native rate to the canonical rate.
///
/// State crosses callback packets, so non-integer rate ratios do not restart
/// phase at each native buffer. The implementation uses integer positions for
/// scheduling and only floating-point arithmetic for interpolation.
#[derive(Debug)]
pub(crate) struct StreamingLinearResampler {
    source_rate_hz: u32,
    target_rate_hz: u32,
    previous_sample: Option<f32>,
    current_source_index: u64,
    next_output_index: u64,
}

impl StreamingLinearResampler {
    pub(crate) fn new(source_rate_hz: u32, target_rate_hz: u32) -> Self {
        Self {
            source_rate_hz,
            target_rate_hz,
            previous_sample: None,
            current_source_index: 0,
            next_output_index: 0,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.previous_sample = None;
        self.current_source_index = 0;
        self.next_output_index = 0;
    }

    pub(crate) fn process(
        &mut self,
        input: &[f32],
        output: &mut [f32],
    ) -> Result<usize, InputConversionError> {
        if self.source_rate_hz == 0 || self.target_rate_hz == 0 {
            return Err(InputConversionError::UnsupportedRepresentation);
        }
        let mut output_count = 0usize;
        for &current_sample in input {
            let Some(previous_sample) = self.previous_sample else {
                if output.is_empty() {
                    return Err(InputConversionError::OutputCapacity);
                }
                output[0] = current_sample;
                output_count = 1;
                self.previous_sample = Some(current_sample);
                self.next_output_index = 1;
                continue;
            };

            let current_source_index = self.current_source_index.saturating_add(1);
            let current_source_position =
                u128::from(current_source_index).saturating_mul(u128::from(self.target_rate_hz));
            while u128::from(self.next_output_index).saturating_mul(u128::from(self.source_rate_hz))
                <= current_source_position
            {
                if output_count == output.len() {
                    return Err(InputConversionError::OutputCapacity);
                }
                let desired_source_position = u128::from(self.next_output_index)
                    .saturating_mul(u128::from(self.source_rate_hz));
                let previous_source_position = u128::from(current_source_index - 1)
                    .saturating_mul(u128::from(self.target_rate_hz));
                let fraction = desired_source_position.saturating_sub(previous_source_position)
                    as f64
                    / f64::from(self.target_rate_hz);
                output[output_count] =
                    previous_sample + (current_sample - previous_sample) * fraction as f32;
                output_count += 1;
                self.next_output_index = self.next_output_index.saturating_add(1);
            }
            self.previous_sample = Some(current_sample);
            self.current_source_index = current_source_index;
        }
        Ok(output_count)
    }
}

pub(crate) fn maximum_resampled_frames(
    native_frame_capacity: usize,
    native_rate_hz: u32,
) -> Option<usize> {
    native_frame_capacity
        .checked_add(1)?
        .checked_mul(usize::try_from(CANONICAL_INPUT_SAMPLE_RATE_HZ).ok()?)?
        .checked_add(usize::try_from(native_rate_hz).ok()?.saturating_sub(1))?
        .checked_div(usize::try_from(native_rate_hz).ok()?)?
        .checked_add(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpal::SupportedBufferSize;

    fn supported(
        channels: u16,
        minimum_rate_hz: u32,
        maximum_rate_hz: u32,
        sample_format: SampleFormat,
    ) -> SupportedStreamConfigRange {
        SupportedStreamConfigRange::new(
            channels,
            minimum_rate_hz,
            maximum_rate_hz,
            SupportedBufferSize::Unknown,
            sample_format,
        )
    }

    #[test]
    fn given_advertised_pcm_ranges_when_selected_then_real_rates_are_not_fabricated() {
        for rate_hz in [8_000, 16_000, 24_000, 32_000, 44_100, 48_000, 96_000] {
            let selected = select_input_config([supported(1, rate_hz, rate_hz, SampleFormat::I16)])
                .expect("advertised PCM format must be selected");
            assert_eq!(selected.native.sample_rate_hz, rate_hz);
            assert_eq!(selected.config.sample_rate(), rate_hz);
            assert_eq!(
                selected.native.sample_representation,
                CaptureSampleRepresentation::SignedInteger16
            );
        }
    }

    #[test]
    fn given_multiple_advertised_formats_when_selected_then_canonical_preferences_apply() {
        let selected = select_input_config([
            supported(2, 44_100, 44_100, SampleFormat::F32),
            supported(2, 48_000, 48_000, SampleFormat::I16),
            supported(1, 48_000, 48_000, SampleFormat::F64),
            supported(1, 48_000, 48_000, SampleFormat::F32),
        ])
        .expect("one PCM format must be selected");

        assert_eq!(selected.native.sample_rate_hz, 48_000);
        assert_eq!(selected.native.channel_count, 1);
        assert_eq!(
            selected.native.sample_representation,
            CaptureSampleRepresentation::Float32
        );
    }

    #[test]
    fn given_dsd_only_input_when_selected_then_format_is_rejected_as_non_pcm() {
        assert!(select_input_config([supported(1, 48_000, 48_000, SampleFormat::DsdU8)]).is_none());
    }

    #[test]
    fn given_supported_pcm_representations_when_advertised_then_each_is_selectable() {
        let cases = [
            (
                SampleFormat::I8,
                CaptureSampleRepresentation::SignedInteger8,
            ),
            (
                SampleFormat::I16,
                CaptureSampleRepresentation::SignedInteger16,
            ),
            (
                SampleFormat::I24,
                CaptureSampleRepresentation::SignedInteger24,
            ),
            (
                SampleFormat::I32,
                CaptureSampleRepresentation::SignedInteger32,
            ),
            (
                SampleFormat::I64,
                CaptureSampleRepresentation::SignedInteger64,
            ),
            (
                SampleFormat::U8,
                CaptureSampleRepresentation::UnsignedInteger8,
            ),
            (
                SampleFormat::U16,
                CaptureSampleRepresentation::UnsignedInteger16,
            ),
            (
                SampleFormat::U24,
                CaptureSampleRepresentation::UnsignedInteger24,
            ),
            (
                SampleFormat::U32,
                CaptureSampleRepresentation::UnsignedInteger32,
            ),
            (
                SampleFormat::U64,
                CaptureSampleRepresentation::UnsignedInteger64,
            ),
            (SampleFormat::F32, CaptureSampleRepresentation::Float32),
            (SampleFormat::F64, CaptureSampleRepresentation::Float64),
        ];

        for (sample_format, expected_representation) in cases {
            let selected = select_input_config([supported(1, 48_000, 48_000, sample_format)])
                .expect("advertised PCM representation must be selectable");
            assert_eq!(
                selected.native.sample_representation,
                expected_representation
            );
            assert_eq!(
                selected.sample_size_bytes,
                representation_sample_size(expected_representation)
            );
        }
    }

    fn encode_i16(values: &[i16]) -> Vec<u8> {
        values
            .iter()
            .flat_map(|sample| sample.to_ne_bytes())
            .collect()
    }

    #[test]
    fn given_signed_integer_stereo_when_decoded_then_canonical_float_mono_is_produced() {
        let input = encode_i16(&[i16::MIN, i16::MAX, 16_384, 16_384]);
        let mut output = [0.0; 2];
        let count = decode_interleaved_to_mono(
            &input,
            CaptureNativeFormat {
                sample_rate_hz: 16_000,
                channel_count: 2,
                sample_representation: CaptureSampleRepresentation::SignedInteger16,
            },
            &mut output,
        )
        .expect("aligned native PCM must convert");

        assert_eq!(count, 2);
        assert!((output[0] + 1.0 / 65_536.0).abs() < 0.000_001);
        assert!((output[1] - 0.5).abs() < 0.000_001);
    }

    #[test]
    fn given_supported_representations_when_equilibrium_is_decoded_then_zero_is_produced() {
        let cases: &[(CaptureSampleRepresentation, &[u8])] = &[
            (CaptureSampleRepresentation::SignedInteger8, &[0]),
            (CaptureSampleRepresentation::SignedInteger16, &[0, 0]),
            (CaptureSampleRepresentation::SignedInteger24, &[0, 0, 0, 0]),
            (CaptureSampleRepresentation::SignedInteger32, &[0, 0, 0, 0]),
            (
                CaptureSampleRepresentation::SignedInteger64,
                &[0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (CaptureSampleRepresentation::UnsignedInteger8, &[128]),
            (
                CaptureSampleRepresentation::UnsignedInteger16,
                &32_768u16.to_ne_bytes(),
            ),
            (
                CaptureSampleRepresentation::UnsignedInteger24,
                &8_388_608u32.to_ne_bytes(),
            ),
            (
                CaptureSampleRepresentation::UnsignedInteger32,
                &2_147_483_648u32.to_ne_bytes(),
            ),
            (
                CaptureSampleRepresentation::UnsignedInteger64,
                &9_223_372_036_854_775_808u64.to_ne_bytes(),
            ),
            (CaptureSampleRepresentation::Float32, &0.0f32.to_ne_bytes()),
            (CaptureSampleRepresentation::Float64, &0.0f64.to_ne_bytes()),
        ];

        for (sample_representation, bytes) in cases {
            let mut output = [1.0];
            assert_eq!(
                decode_interleaved_to_mono(
                    bytes,
                    CaptureNativeFormat {
                        sample_rate_hz: 48_000,
                        channel_count: 1,
                        sample_representation: *sample_representation,
                    },
                    &mut output,
                ),
                Ok(1)
            );
            assert_eq!(output[0], 0.0, "{sample_representation:?}");
        }
    }

    #[test]
    fn given_streaming_rate_conversion_when_packets_cross_then_phase_is_preserved() {
        let mut converter = StreamingLinearResampler::new(16_000, 48_000);
        let first = (0..160).map(|value| value as f32).collect::<Vec<_>>();
        let second = (160..320).map(|value| value as f32).collect::<Vec<_>>();
        let mut first_output = [0.0; 482];
        let mut second_output = [0.0; 482];

        let first_count = converter
            .process(&first, &mut first_output)
            .expect("first packet must fit");
        let second_count = converter
            .process(&second, &mut second_output)
            .expect("second packet must fit");

        assert_eq!(first_count, 478);
        assert_eq!(second_count, 480);
        assert!((first_output[477] - 159.0).abs() < 0.000_1);
        assert!((second_output[0] - (159.0 + 1.0 / 3.0)).abs() < 0.000_1);
        assert!((second_output[1] - (159.0 + 2.0 / 3.0)).abs() < 0.000_1);
    }

    #[test]
    fn given_supported_native_rates_when_converted_then_canonical_storage_stays_bounded() {
        for source_rate_hz in [8_000, 16_000, 24_000, 32_000, 44_100, 48_000, 96_000] {
            let input_count = usize::try_from(source_rate_hz / 100).unwrap();
            let input = (0..input_count)
                .map(|index| index as f32 / input_count as f32)
                .collect::<Vec<_>>();
            let output_capacity = maximum_resampled_frames(input_count, source_rate_hz)
                .expect("test rate must have bounded output storage");
            let mut output = vec![0.0; output_capacity];
            let mut converter =
                StreamingLinearResampler::new(source_rate_hz, CANONICAL_INPUT_SAMPLE_RATE_HZ);

            let output_count = converter
                .process(&input, &mut output)
                .expect("bounded output capacity must hold the converted packet");
            let expected_count = ((u64::try_from(input_count - 1).unwrap()
                * u64::from(CANONICAL_INPUT_SAMPLE_RATE_HZ))
                / u64::from(source_rate_hz)
                + 1) as usize;

            assert_eq!(output_count, expected_count, "source rate {source_rate_hz}");
            assert!(output[..output_count]
                .iter()
                .all(|sample| sample.is_finite()));
        }
    }

    #[test]
    fn given_insufficient_output_capacity_when_rate_converted_then_request_is_rejected() {
        let mut converter = StreamingLinearResampler::new(8_000, 48_000);
        assert_eq!(
            converter.process(&[0.0, 1.0], &mut [0.0; 1]),
            Err(InputConversionError::OutputCapacity)
        );
    }
}

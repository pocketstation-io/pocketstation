use crate::{
    ClockDomainId, ConnectorId, EndpointId, FrameLineage, RouteId, SampleFormat, SessionId,
    SourceId, StemId, StreamId,
};
use std::mem::size_of;

use super::ConnectorItem;
use super::{
    ConnectorConfiguration, ConnectorConfigurationValue, ConnectorSecret,
    ResolvedConnectorConfiguration, MAX_CONNECTOR_CONFIGURATION_FIELDS,
    MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES,
};

const MAGIC: [u8; 4] = *b"PKCA";
const HEADER_BYTES: usize = 136;
const FLAG_CONNECTOR_ID_PRESENT: u32 = 1;
const SAMPLE_FORMAT_F32_INTERLEAVED: u8 = 1;

pub const CONNECTOR_AUDIO_RECORD_MAJOR: u16 = 1;
pub const CONNECTOR_AUDIO_RECORD_MINOR: u16 = 0;
pub const MAX_CONNECTOR_AUDIO_RECORD_PORT_BYTES: usize = 256;
pub const MAX_CONNECTOR_AUDIO_RECORD_SAMPLES: usize = 262_144;

const CONFIGURATION_MAGIC: [u8; 4] = *b"PKCC";
const CONFIGURATION_HEADER_BYTES: usize = 16;
const CONFIGURATION_ENTRY_HEADER_BYTES: usize = 8;
const CONFIGURATION_TEXT: u8 = 1;
const CONFIGURATION_BOOLEAN: u8 = 2;
const CONFIGURATION_SIGNED_INTEGER: u8 = 3;
const CONFIGURATION_UNSIGNED_INTEGER: u8 = 4;
const CONFIGURATION_DURATION_MILLISECONDS: u8 = 5;
const CONFIGURATION_BYTE_COUNT: u8 = 6;
const CONFIGURATION_SECRET: u8 = 7;

pub const CONNECTOR_CONFIGURATION_RECORD_MAJOR: u16 = 1;
pub const CONNECTOR_CONFIGURATION_RECORD_MINOR: u16 = 0;

/// Canonical typed configuration handed to a connector sidecar during its
/// bounded Configure handshake. Secret classification survives the boundary;
/// Debug output continues to redact secret values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorConfigurationRecord(ConnectorConfiguration);

impl ConnectorConfigurationRecord {
    pub fn from_resolved(configuration: &ResolvedConnectorConfiguration) -> Self {
        let mut values = ConnectorConfiguration::new();
        for (name, value) in configuration.iter() {
            values.insert(name, value.clone());
        }
        Self(values)
    }

    pub const fn configuration(&self) -> &ConnectorConfiguration {
        &self.0
    }

    pub fn into_configuration(self) -> ConnectorConfiguration {
        self.0
    }

    pub fn encode(&self) -> Result<Vec<u8>, ConnectorConfigurationRecordError> {
        if self.0.len() > MAX_CONNECTOR_CONFIGURATION_FIELDS {
            return Err(ConnectorConfigurationRecordError::TooManyFields);
        }
        let mut output = Vec::new();
        output.extend_from_slice(&CONFIGURATION_MAGIC);
        output.extend_from_slice(&CONNECTOR_CONFIGURATION_RECORD_MAJOR.to_le_bytes());
        output.extend_from_slice(&CONNECTOR_CONFIGURATION_RECORD_MINOR.to_le_bytes());
        push_u32(&mut output, self.0.len())
            .map_err(|_| ConnectorConfigurationRecordError::LengthOverflow)?;
        output.extend_from_slice(&0u32.to_le_bytes());
        for (name, value) in self.0.iter() {
            if name.trim().is_empty() || name.len() > MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES {
                return Err(ConnectorConfigurationRecordError::InvalidFieldName);
            }
            let (kind, bytes) = encode_configuration_value(value)?;
            if bytes.len() > MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES {
                return Err(ConnectorConfigurationRecordError::ValueTooLarge);
            }
            let name_len = u16::try_from(name.len())
                .map_err(|_| ConnectorConfigurationRecordError::LengthOverflow)?;
            output.extend_from_slice(&name_len.to_le_bytes());
            output.push(kind);
            output.push(0);
            push_u32(&mut output, bytes.len())
                .map_err(|_| ConnectorConfigurationRecordError::LengthOverflow)?;
            output.extend_from_slice(name.as_bytes());
            output.extend_from_slice(&bytes);
        }
        Ok(output)
    }

    pub fn decode(input: &[u8]) -> Result<Self, ConnectorConfigurationRecordError> {
        if input.len() < CONFIGURATION_HEADER_BYTES {
            return Err(ConnectorConfigurationRecordError::Truncated);
        }
        if input[..4] != CONFIGURATION_MAGIC {
            return Err(ConnectorConfigurationRecordError::InvalidMagic);
        }
        let major = read_u16(input, 4).map_err(map_audio_record_read_error)?;
        let minor = read_u16(input, 6).map_err(map_audio_record_read_error)?;
        if major != CONNECTOR_CONFIGURATION_RECORD_MAJOR {
            return Err(ConnectorConfigurationRecordError::UnsupportedMajor(major));
        }
        if minor > CONNECTOR_CONFIGURATION_RECORD_MINOR {
            return Err(ConnectorConfigurationRecordError::UnsupportedMinor(minor));
        }
        let field_count = read_u32(input, 8).map_err(map_audio_record_read_error)? as usize;
        if field_count > MAX_CONNECTOR_CONFIGURATION_FIELDS {
            return Err(ConnectorConfigurationRecordError::TooManyFields);
        }
        if read_u32(input, 12).map_err(map_audio_record_read_error)? != 0 {
            return Err(ConnectorConfigurationRecordError::ReservedFieldSet);
        }
        let mut cursor = CONFIGURATION_HEADER_BYTES;
        let mut configuration = ConnectorConfiguration::new();
        for _ in 0..field_count {
            let header_end = cursor
                .checked_add(CONFIGURATION_ENTRY_HEADER_BYTES)
                .ok_or(ConnectorConfigurationRecordError::LengthOverflow)?;
            let header = input
                .get(cursor..header_end)
                .ok_or(ConnectorConfigurationRecordError::Truncated)?;
            let name_len = u16::from_le_bytes([header[0], header[1]]) as usize;
            let kind = header[2];
            if header[3] != 0 {
                return Err(ConnectorConfigurationRecordError::ReservedFieldSet);
            }
            let value_len = u32::from_le_bytes(
                header[4..8]
                    .try_into()
                    .map_err(|_| ConnectorConfigurationRecordError::Truncated)?,
            ) as usize;
            if name_len == 0 || name_len > MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES {
                return Err(ConnectorConfigurationRecordError::InvalidFieldName);
            }
            if value_len > MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES {
                return Err(ConnectorConfigurationRecordError::ValueTooLarge);
            }
            cursor = header_end;
            let name_end = cursor
                .checked_add(name_len)
                .ok_or(ConnectorConfigurationRecordError::LengthOverflow)?;
            let name = std::str::from_utf8(
                input
                    .get(cursor..name_end)
                    .ok_or(ConnectorConfigurationRecordError::Truncated)?,
            )
            .map_err(|_| ConnectorConfigurationRecordError::InvalidFieldName)?;
            cursor = name_end;
            let value_end = cursor
                .checked_add(value_len)
                .ok_or(ConnectorConfigurationRecordError::LengthOverflow)?;
            let bytes = input
                .get(cursor..value_end)
                .ok_or(ConnectorConfigurationRecordError::Truncated)?;
            cursor = value_end;
            let value = decode_configuration_value(kind, bytes)?;
            if configuration.insert(name, value).is_some() {
                return Err(ConnectorConfigurationRecordError::DuplicateField);
            }
        }
        if cursor != input.len() {
            return Err(ConnectorConfigurationRecordError::TrailingBytes);
        }
        Ok(Self(configuration))
    }
}

fn encode_configuration_value(
    value: &ConnectorConfigurationValue,
) -> Result<(u8, Vec<u8>), ConnectorConfigurationRecordError> {
    let encoded = match value {
        ConnectorConfigurationValue::Text(value) => (CONFIGURATION_TEXT, value.as_bytes().to_vec()),
        ConnectorConfigurationValue::Boolean(value) => {
            (CONFIGURATION_BOOLEAN, vec![u8::from(*value)])
        }
        ConnectorConfigurationValue::SignedInteger(value) => {
            (CONFIGURATION_SIGNED_INTEGER, value.to_le_bytes().to_vec())
        }
        ConnectorConfigurationValue::UnsignedInteger(value) => {
            (CONFIGURATION_UNSIGNED_INTEGER, value.to_le_bytes().to_vec())
        }
        ConnectorConfigurationValue::DurationMilliseconds(value) => (
            CONFIGURATION_DURATION_MILLISECONDS,
            value.to_le_bytes().to_vec(),
        ),
        ConnectorConfigurationValue::ByteCount(value) => {
            (CONFIGURATION_BYTE_COUNT, value.to_le_bytes().to_vec())
        }
        ConnectorConfigurationValue::Secret(value) => (
            CONFIGURATION_SECRET,
            value.expose_secret().as_bytes().to_vec(),
        ),
    };
    Ok(encoded)
}

fn decode_configuration_value(
    kind: u8,
    bytes: &[u8],
) -> Result<ConnectorConfigurationValue, ConnectorConfigurationRecordError> {
    match kind {
        CONFIGURATION_TEXT => Ok(ConnectorConfigurationValue::Text(decode_text(bytes)?)),
        CONFIGURATION_BOOLEAN => match bytes {
            [0] => Ok(ConnectorConfigurationValue::Boolean(false)),
            [1] => Ok(ConnectorConfigurationValue::Boolean(true)),
            _ => Err(ConnectorConfigurationRecordError::InvalidValue),
        },
        CONFIGURATION_SIGNED_INTEGER => Ok(ConnectorConfigurationValue::SignedInteger(
            i64::from_le_bytes(decode_eight(bytes)?),
        )),
        CONFIGURATION_UNSIGNED_INTEGER => Ok(ConnectorConfigurationValue::UnsignedInteger(
            u64::from_le_bytes(decode_eight(bytes)?),
        )),
        CONFIGURATION_DURATION_MILLISECONDS => {
            Ok(ConnectorConfigurationValue::DurationMilliseconds(
                u64::from_le_bytes(decode_eight(bytes)?),
            ))
        }
        CONFIGURATION_BYTE_COUNT => Ok(ConnectorConfigurationValue::ByteCount(u64::from_le_bytes(
            decode_eight(bytes)?,
        ))),
        CONFIGURATION_SECRET => Ok(ConnectorConfigurationValue::Secret(
            ConnectorSecret::new(decode_text(bytes)?)
                .map_err(|_| ConnectorConfigurationRecordError::InvalidValue)?,
        )),
        other => Err(ConnectorConfigurationRecordError::UnknownValueKind(other)),
    }
}

fn decode_text(bytes: &[u8]) -> Result<String, ConnectorConfigurationRecordError> {
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| ConnectorConfigurationRecordError::InvalidValue)
}

fn decode_eight(bytes: &[u8]) -> Result<[u8; 8], ConnectorConfigurationRecordError> {
    bytes
        .try_into()
        .map_err(|_| ConnectorConfigurationRecordError::InvalidValue)
}

fn map_audio_record_read_error(
    _error: ConnectorAudioRecordError,
) -> ConnectorConfigurationRecordError {
    ConnectorConfigurationRecordError::Truncated
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ConnectorConfigurationRecordError {
    #[error("connector configuration record is truncated")]
    Truncated,
    #[error("connector configuration record has trailing bytes")]
    TrailingBytes,
    #[error("connector configuration record magic is invalid")]
    InvalidMagic,
    #[error("connector configuration record major {0} is unsupported")]
    UnsupportedMajor(u16),
    #[error("connector configuration record minor {0} is unsupported")]
    UnsupportedMinor(u16),
    #[error("connector configuration record reserved field is non-zero")]
    ReservedFieldSet,
    #[error("connector configuration record has too many fields")]
    TooManyFields,
    #[error("connector configuration record field name is invalid")]
    InvalidFieldName,
    #[error("connector configuration record contains a duplicate field")]
    DuplicateField,
    #[error("connector configuration record value is too large")]
    ValueTooLarge,
    #[error("connector configuration record value is invalid")]
    InvalidValue,
    #[error("connector configuration record value kind {0} is unknown")]
    UnknownValueKind(u8),
    #[error("connector configuration record length overflowed")]
    LengthOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorAudioMetadata {
    pub endpoint_id: EndpointId,
    pub connector_id: Option<ConnectorId>,
    pub route_id: RouteId,
    pub stream_id: StreamId,
    pub lineage: FrameLineage,
    pub sample_rate_hz: u32,
    pub channels: u8,
    pub sample_format: SampleFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectorAudioRecord {
    port_name: String,
    metadata: ConnectorAudioMetadata,
    samples: Vec<f32>,
}

impl ConnectorAudioRecord {
    pub fn try_new(
        port_name: impl Into<String>,
        metadata: ConnectorAudioMetadata,
        samples: Vec<f32>,
    ) -> Result<Self, ConnectorAudioRecordError> {
        let record = Self {
            port_name: port_name.into(),
            metadata,
            samples,
        };
        record.validate()?;
        Ok(record)
    }

    pub fn from_item(item: &ConnectorItem<'_>) -> Result<Self, ConnectorAudioRecordError> {
        let ConnectorItem::Audio { input, frame } = item else {
            return Err(ConnectorAudioRecordError::NotAudio);
        };
        let lineage = frame.lineage();
        Self::try_new(
            input.port_name(),
            ConnectorAudioMetadata {
                endpoint_id: input.endpoint_id(),
                connector_id: input.connector_id(),
                route_id: input.route_id(),
                stream_id: frame.stream_id(),
                lineage,
                sample_rate_hz: frame.sample_rate_hz(),
                channels: frame.channels(),
                sample_format: frame.sample_format(),
            },
            frame.samples().to_vec(),
        )
    }

    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    pub const fn metadata(&self) -> &ConnectorAudioMetadata {
        &self.metadata
    }

    pub fn samples(&self) -> &[f32] {
        &self.samples
    }

    pub fn encode(&self) -> Result<Vec<u8>, ConnectorAudioRecordError> {
        self.validate()?;
        let sample_bytes = self
            .samples
            .len()
            .checked_mul(size_of::<f32>())
            .ok_or(ConnectorAudioRecordError::LengthOverflow)?;
        let capacity = HEADER_BYTES
            .checked_add(self.port_name.len())
            .and_then(|value| value.checked_add(sample_bytes))
            .ok_or(ConnectorAudioRecordError::LengthOverflow)?;
        let mut output = Vec::with_capacity(capacity);
        output.extend_from_slice(&MAGIC);
        output.extend_from_slice(&CONNECTOR_AUDIO_RECORD_MAJOR.to_le_bytes());
        output.extend_from_slice(&CONNECTOR_AUDIO_RECORD_MINOR.to_le_bytes());
        push_u32(&mut output, HEADER_BYTES)?;
        let flags = if self.metadata.connector_id.is_some() {
            FLAG_CONNECTOR_ID_PRESENT
        } else {
            0
        };
        output.extend_from_slice(&flags.to_le_bytes());
        push_u32(&mut output, self.port_name.len())?;
        push_u32(&mut output, self.samples.len())?;
        output.extend_from_slice(&self.metadata.sample_rate_hz.to_le_bytes());
        output.push(self.metadata.channels);
        output.push(SAMPLE_FORMAT_F32_INTERLEAVED);
        output.extend_from_slice(&0u16.to_le_bytes());
        output.extend_from_slice(&self.metadata.endpoint_id.get().to_le_bytes());
        output.extend_from_slice(
            &self
                .metadata
                .connector_id
                .map_or(0, ConnectorId::get)
                .to_le_bytes(),
        );
        output.extend_from_slice(&self.metadata.route_id.get().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.session_id().get().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.source_id().get().to_le_bytes());
        output.extend_from_slice(&self.metadata.stream_id.get().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.stem_id().get().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.clock_id().get().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.source_generation().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.sequence_number().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.timestamp_start_ns().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.duration_ns().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.discontinuity_epoch().to_le_bytes());
        output.extend_from_slice(&self.metadata.lineage.permission_epoch().to_le_bytes());
        debug_assert_eq!(output.len(), HEADER_BYTES);
        output.extend_from_slice(self.port_name.as_bytes());
        for sample in &self.samples {
            output.extend_from_slice(&sample.to_le_bytes());
        }
        Ok(output)
    }

    pub fn decode(input: &[u8]) -> Result<Self, ConnectorAudioRecordError> {
        if input.len() < HEADER_BYTES {
            return Err(ConnectorAudioRecordError::Truncated);
        }
        if input[..4] != MAGIC {
            return Err(ConnectorAudioRecordError::InvalidMagic);
        }
        let major = read_u16(input, 4)?;
        let minor = read_u16(input, 6)?;
        if major != CONNECTOR_AUDIO_RECORD_MAJOR {
            return Err(ConnectorAudioRecordError::UnsupportedMajor(major));
        }
        if minor > CONNECTOR_AUDIO_RECORD_MINOR {
            return Err(ConnectorAudioRecordError::UnsupportedMinor(minor));
        }
        if read_u32(input, 8)? as usize != HEADER_BYTES {
            return Err(ConnectorAudioRecordError::InvalidHeaderSize);
        }
        let flags = read_u32(input, 12)?;
        if flags & !FLAG_CONNECTOR_ID_PRESENT != 0 {
            return Err(ConnectorAudioRecordError::ReservedFieldSet);
        }
        let port_bytes = read_u32(input, 16)? as usize;
        let sample_count = read_u32(input, 20)? as usize;
        let sample_rate_hz = read_u32(input, 24)?;
        let channels = input[28];
        if input[29] != SAMPLE_FORMAT_F32_INTERLEAVED || read_u16(input, 30)? != 0 {
            return Err(ConnectorAudioRecordError::UnsupportedSampleFormat);
        }
        if port_bytes == 0 || port_bytes > MAX_CONNECTOR_AUDIO_RECORD_PORT_BYTES {
            return Err(ConnectorAudioRecordError::InvalidPortName);
        }
        if sample_count == 0 || sample_count > MAX_CONNECTOR_AUDIO_RECORD_SAMPLES {
            return Err(ConnectorAudioRecordError::InvalidSampleCount);
        }
        let sample_bytes = sample_count
            .checked_mul(size_of::<f32>())
            .ok_or(ConnectorAudioRecordError::LengthOverflow)?;
        let expected = HEADER_BYTES
            .checked_add(port_bytes)
            .and_then(|value| value.checked_add(sample_bytes))
            .ok_or(ConnectorAudioRecordError::LengthOverflow)?;
        if input.len() != expected {
            return Err(if input.len() < expected {
                ConnectorAudioRecordError::Truncated
            } else {
                ConnectorAudioRecordError::TrailingBytes
            });
        }
        let port_name = std::str::from_utf8(&input[HEADER_BYTES..HEADER_BYTES + port_bytes])
            .map_err(|_| ConnectorAudioRecordError::InvalidPortName)?
            .to_owned();
        let lineage = FrameLineage::try_new(
            SessionId::new(read_u64(input, 56)?),
            SourceId::new(read_u64(input, 64)?),
            StemId::new(read_u64(input, 80)?),
            ClockDomainId::new(read_u32(input, 88)?),
            read_u64(input, 96)?,
            read_u64(input, 104)?,
            read_u64(input, 112)?,
            read_u32(input, 92)?,
            read_u64(input, 120)?,
            read_u64(input, 128)?,
        )
        .map_err(|_| ConnectorAudioRecordError::InvalidLineage)?;
        let encoded_connector_id = read_u64(input, 40)?;
        let connector_id = if flags & FLAG_CONNECTOR_ID_PRESENT != 0 {
            if encoded_connector_id == 0 {
                return Err(ConnectorAudioRecordError::InvalidConnectorId);
            }
            Some(ConnectorId::new(encoded_connector_id))
        } else {
            if encoded_connector_id != 0 {
                return Err(ConnectorAudioRecordError::ReservedFieldSet);
            }
            None
        };
        let mut samples = Vec::with_capacity(sample_count);
        let (encoded_samples, remainder) =
            input[HEADER_BYTES + port_bytes..].as_chunks::<{ size_of::<f32>() }>();
        if !remainder.is_empty() {
            return Err(ConnectorAudioRecordError::Truncated);
        }
        for encoded in encoded_samples {
            samples.push(f32::from_le_bytes(*encoded));
        }
        Self::try_new(
            port_name,
            ConnectorAudioMetadata {
                endpoint_id: EndpointId::new(read_u64(input, 32)?),
                connector_id,
                route_id: RouteId::new(read_u64(input, 48)?),
                stream_id: StreamId::new(read_u64(input, 72)?),
                lineage,
                sample_rate_hz,
                channels,
                sample_format: SampleFormat::F32Interleaved,
            },
            samples,
        )
    }

    fn validate(&self) -> Result<(), ConnectorAudioRecordError> {
        if self.port_name.trim().is_empty()
            || self.port_name.len() > MAX_CONNECTOR_AUDIO_RECORD_PORT_BYTES
        {
            return Err(ConnectorAudioRecordError::InvalidPortName);
        }
        if self.metadata.sample_rate_hz == 0 || self.metadata.channels == 0 {
            return Err(ConnectorAudioRecordError::InvalidSampleSpec);
        }
        if self.metadata.sample_format != SampleFormat::F32Interleaved {
            return Err(ConnectorAudioRecordError::UnsupportedSampleFormat);
        }
        if self.samples.is_empty()
            || self.samples.len() > MAX_CONNECTOR_AUDIO_RECORD_SAMPLES
            || !self
                .samples
                .len()
                .is_multiple_of(usize::from(self.metadata.channels))
        {
            return Err(ConnectorAudioRecordError::InvalidSampleCount);
        }
        Ok(())
    }
}

fn push_u32(output: &mut Vec<u8>, value: usize) -> Result<(), ConnectorAudioRecordError> {
    output.extend_from_slice(
        &u32::try_from(value)
            .map_err(|_| ConnectorAudioRecordError::LengthOverflow)?
            .to_le_bytes(),
    );
    Ok(())
}

fn read_u16(input: &[u8], offset: usize) -> Result<u16, ConnectorAudioRecordError> {
    let bytes = input
        .get(offset..offset + 2)
        .ok_or(ConnectorAudioRecordError::Truncated)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(input: &[u8], offset: usize) -> Result<u32, ConnectorAudioRecordError> {
    let bytes = input
        .get(offset..offset + 4)
        .ok_or(ConnectorAudioRecordError::Truncated)?;
    Ok(u32::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| ConnectorAudioRecordError::Truncated)?,
    ))
}

fn read_u64(input: &[u8], offset: usize) -> Result<u64, ConnectorAudioRecordError> {
    let bytes = input
        .get(offset..offset + 8)
        .ok_or(ConnectorAudioRecordError::Truncated)?;
    Ok(u64::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| ConnectorAudioRecordError::Truncated)?,
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ConnectorAudioRecordError {
    #[error("connector item is not PCM audio")]
    NotAudio,
    #[error("connector audio record is truncated")]
    Truncated,
    #[error("connector audio record has trailing bytes")]
    TrailingBytes,
    #[error("connector audio record magic is invalid")]
    InvalidMagic,
    #[error("connector audio record major {0} is unsupported")]
    UnsupportedMajor(u16),
    #[error("connector audio record minor {0} is unsupported")]
    UnsupportedMinor(u16),
    #[error("connector audio record header size is invalid")]
    InvalidHeaderSize,
    #[error("connector audio record reserved field is non-zero")]
    ReservedFieldSet,
    #[error("connector audio record port name is invalid")]
    InvalidPortName,
    #[error("connector audio record sample specification is invalid")]
    InvalidSampleSpec,
    #[error("connector audio record sample format is unsupported")]
    UnsupportedSampleFormat,
    #[error("connector audio record sample count is invalid")]
    InvalidSampleCount,
    #[error("connector audio record lineage is invalid")]
    InvalidLineage,
    #[error("connector audio record connector identity is invalid")]
    InvalidConnectorId,
    #[error("connector audio record length overflowed")]
    LengthOverflow,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record() -> ConnectorAudioRecord {
        ConnectorAudioRecord::try_new(
            "application",
            ConnectorAudioMetadata {
                endpoint_id: EndpointId::new(2),
                connector_id: Some(ConnectorId::new(3)),
                route_id: RouteId::new(4),
                stream_id: StreamId::new(5),
                lineage: FrameLineage::try_new(
                    SessionId::new(6),
                    SourceId::new(7),
                    StemId::new(8),
                    ClockDomainId::new(9),
                    10,
                    11,
                    12,
                    13,
                    14,
                    15,
                )
                .expect("lineage"),
                sample_rate_hz: 48_000,
                channels: 2,
                sample_format: SampleFormat::F32Interleaved,
            },
            vec![0.25, -0.25, 0.5, -0.5],
        )
        .expect("record")
    }

    #[test]
    fn given_audio_record_when_round_tripped_then_transport_and_lineage_identity_are_preserved() {
        let record = record();
        let encoded = record.encode().expect("encode");
        let decoded = ConnectorAudioRecord::decode(&encoded).expect("decode");

        assert_eq!(decoded, record);
    }

    #[test]
    fn given_invalid_audio_record_when_decoded_then_trailing_and_oversized_payloads_are_rejected() {
        let mut encoded = record().encode().expect("encode");
        encoded.push(0);
        assert_eq!(
            ConnectorAudioRecord::decode(&encoded),
            Err(ConnectorAudioRecordError::TrailingBytes)
        );
        assert_eq!(
            ConnectorAudioRecord::try_new(
                "application",
                record().metadata,
                vec![0.0; MAX_CONNECTOR_AUDIO_RECORD_SAMPLES + 1],
            ),
            Err(ConnectorAudioRecordError::InvalidSampleCount)
        );
    }

    #[test]
    fn given_typed_configuration_when_round_tripped_then_types_and_secret_redaction_are_preserved()
    {
        let mut configuration = ConnectorConfiguration::new();
        configuration.insert(
            "destination",
            ConnectorConfigurationValue::Text("wss://relay.example".to_owned()),
        );
        configuration.insert("enabled", ConnectorConfigurationValue::Boolean(true));
        configuration.insert("signed", ConnectorConfigurationValue::SignedInteger(-7));
        configuration.insert("unsigned", ConnectorConfigurationValue::UnsignedInteger(8));
        configuration.insert(
            "timeout_ms",
            ConnectorConfigurationValue::DurationMilliseconds(9),
        );
        configuration.insert("bytes", ConnectorConfigurationValue::ByteCount(10));
        configuration.insert(
            "token",
            ConnectorConfigurationValue::Secret(
                ConnectorSecret::new("private-value").expect("secret"),
            ),
        );
        let record = ConnectorConfigurationRecord(configuration);
        let encoded = record.encode().expect("encode");
        let decoded = ConnectorConfigurationRecord::decode(&encoded).expect("decode");

        assert_eq!(decoded, record);
        assert!(!format!("{decoded:?}").contains("private-value"));
        assert!(format!("{decoded:?}").contains("<redacted>"));
    }

    #[test]
    fn given_invalid_configuration_record_when_decoded_then_unknown_kinds_and_trailing_bytes_are_rejected(
    ) {
        let record = ConnectorConfigurationRecord(ConnectorConfiguration::new());
        let mut trailing = record.encode().expect("encode");
        trailing.push(0);
        assert_eq!(
            ConnectorConfigurationRecord::decode(&trailing),
            Err(ConnectorConfigurationRecordError::TrailingBytes)
        );

        let mut configuration = ConnectorConfiguration::new();
        configuration.insert("value", ConnectorConfigurationValue::Text("x".to_owned()));
        let mut unknown = ConnectorConfigurationRecord(configuration)
            .encode()
            .expect("encode");
        unknown[CONFIGURATION_HEADER_BYTES + 2] = u8::MAX;
        assert_eq!(
            ConnectorConfigurationRecord::decode(&unknown),
            Err(ConnectorConfigurationRecordError::UnknownValueKind(u8::MAX))
        );
    }
}

const MAGIC: [u8; 4] = *b"PKSS";
const HEADER_BYTES: usize = 52;

pub const SIDECAR_PROTOCOL_MAJOR: u16 = 1;
pub const SIDECAR_PROTOCOL_MINOR: u16 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SidecarMessageKind {
    Signal = 1,
    Ready = 2,
    Error = 3,
    Cancel = 4,
    Close = 5,
    Hello = 6,
    Manifest = 7,
    Configure = 8,
    Observation = 9,
    Closed = 10,
}

impl TryFrom<u8> for SidecarMessageKind {
    type Error = SidecarProtocolError;

    fn try_from(value: u8) -> Result<Self, SidecarProtocolError> {
        match value {
            1 => Ok(Self::Signal),
            2 => Ok(Self::Ready),
            3 => Ok(Self::Error),
            4 => Ok(Self::Cancel),
            5 => Ok(Self::Close),
            6 => Ok(Self::Hello),
            7 => Ok(Self::Manifest),
            8 => Ok(Self::Configure),
            9 => Ok(Self::Observation),
            10 => Ok(Self::Closed),
            _ => Err(SidecarProtocolError::UnknownMessageKind(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SidecarProtocolLimits {
    pub max_signal_id_bytes: usize,
    pub max_role_bytes: usize,
    pub max_schema_bytes: usize,
    pub max_payload_bytes: usize,
}

impl Default for SidecarProtocolLimits {
    fn default() -> Self {
        Self {
            max_signal_id_bytes: 256,
            max_role_bytes: 256,
            max_schema_bytes: 1_024,
            max_payload_bytes: 1_048_576,
        }
    }
}

impl SidecarProtocolLimits {
    pub fn max_frame_bytes(self) -> Result<usize, SidecarProtocolError> {
        HEADER_BYTES
            .checked_add(self.max_signal_id_bytes)
            .and_then(|total| total.checked_add(self.max_role_bytes))
            .and_then(|total| total.checked_add(self.max_schema_bytes))
            .and_then(|total| total.checked_add(self.max_payload_bytes))
            .ok_or(SidecarProtocolError::FrameLengthOverflow)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarMessage {
    pub kind: SidecarMessageKind,
    pub terminal: bool,
    pub stream_id: u64,
    pub sequence_number: u64,
    pub timestamp_ns: u64,
    pub signal_id: String,
    pub role: Option<String>,
    pub schema: Option<String>,
    pub payload: Vec<u8>,
}

impl SidecarMessage {
    pub fn encode(&self, limits: SidecarProtocolLimits) -> Result<Vec<u8>, SidecarProtocolError> {
        self.validate(limits)?;
        let role = self.role.as_deref().unwrap_or("").as_bytes();
        let schema = self.schema.as_deref().unwrap_or("").as_bytes();
        let signal = self.signal_id.as_bytes();
        let frame_bytes = HEADER_BYTES
            .checked_add(signal.len())
            .and_then(|total| total.checked_add(role.len()))
            .and_then(|total| total.checked_add(schema.len()))
            .and_then(|total| total.checked_add(self.payload.len()))
            .ok_or(SidecarProtocolError::FrameLengthOverflow)?;
        if frame_bytes > limits.max_frame_bytes()? {
            return Err(SidecarProtocolError::FrameTooLarge);
        }
        let mut output = Vec::with_capacity(frame_bytes);
        output.extend_from_slice(&MAGIC);
        output.extend_from_slice(&SIDECAR_PROTOCOL_MAJOR.to_le_bytes());
        output.extend_from_slice(&SIDECAR_PROTOCOL_MINOR.to_le_bytes());
        output.push(self.kind as u8);
        output.push(u8::from(self.terminal));
        output.extend_from_slice(&0u16.to_le_bytes());
        output.extend_from_slice(&self.stream_id.to_le_bytes());
        output.extend_from_slice(&self.sequence_number.to_le_bytes());
        output.extend_from_slice(&self.timestamp_ns.to_le_bytes());
        push_len(&mut output, signal.len())?;
        push_len(&mut output, role.len())?;
        push_len(&mut output, schema.len())?;
        push_len(&mut output, self.payload.len())?;
        output.extend_from_slice(signal);
        output.extend_from_slice(role);
        output.extend_from_slice(schema);
        output.extend_from_slice(&self.payload);
        Ok(output)
    }

    pub fn decode(
        input: &[u8],
        limits: SidecarProtocolLimits,
    ) -> Result<Self, SidecarProtocolError> {
        if input.len() < HEADER_BYTES {
            return Err(SidecarProtocolError::Truncated);
        }
        if input[..4] != MAGIC {
            return Err(SidecarProtocolError::InvalidMagic);
        }
        let major = read_u16(input, 4)?;
        let minor = read_u16(input, 6)?;
        if major != SIDECAR_PROTOCOL_MAJOR {
            return Err(SidecarProtocolError::UnsupportedMajor(major));
        }
        if minor > SIDECAR_PROTOCOL_MINOR {
            return Err(SidecarProtocolError::UnsupportedMinor(minor));
        }
        let kind = SidecarMessageKind::try_from(input[8])?;
        let terminal = match input[9] {
            0 => false,
            1 => true,
            value => return Err(SidecarProtocolError::InvalidTerminal(value)),
        };
        if read_u16(input, 10)? != 0 {
            return Err(SidecarProtocolError::ReservedFieldSet);
        }
        let stream_id = read_u64(input, 12)?;
        let sequence_number = read_u64(input, 20)?;
        let timestamp_ns = read_u64(input, 28)?;
        let signal_len = read_u32(input, 36)? as usize;
        let role_len = read_u32(input, 40)? as usize;
        let schema_len = read_u32(input, 44)? as usize;
        let payload_len = read_u32(input, 48)? as usize;
        validate_length(signal_len, limits.max_signal_id_bytes, "signal id")?;
        validate_length(role_len, limits.max_role_bytes, "role")?;
        validate_length(schema_len, limits.max_schema_bytes, "schema")?;
        validate_length(payload_len, limits.max_payload_bytes, "payload")?;
        let expected = HEADER_BYTES
            .checked_add(signal_len)
            .and_then(|total| total.checked_add(role_len))
            .and_then(|total| total.checked_add(schema_len))
            .and_then(|total| total.checked_add(payload_len))
            .ok_or(SidecarProtocolError::FrameLengthOverflow)?;
        if expected != input.len() || expected > limits.max_frame_bytes()? {
            return Err(if expected > input.len() {
                SidecarProtocolError::Truncated
            } else {
                SidecarProtocolError::TrailingBytes
            });
        }
        let mut cursor = HEADER_BYTES;
        let signal_id = read_text(input, &mut cursor, signal_len, "signal id")?;
        if signal_id.is_empty() {
            return Err(SidecarProtocolError::EmptySignalId);
        }
        let role = optional_text(read_text(input, &mut cursor, role_len, "role")?);
        let schema = optional_text(read_text(input, &mut cursor, schema_len, "schema")?);
        let payload = input[cursor..cursor + payload_len].to_vec();
        let message = Self {
            kind,
            terminal,
            stream_id,
            sequence_number,
            timestamp_ns,
            signal_id,
            role,
            schema,
            payload,
        };
        message.validate(limits)?;
        Ok(message)
    }

    pub fn validate(&self, limits: SidecarProtocolLimits) -> Result<(), SidecarProtocolError> {
        if self.signal_id.is_empty() {
            return Err(SidecarProtocolError::EmptySignalId);
        }
        validate_length(
            self.signal_id.len(),
            limits.max_signal_id_bytes,
            "signal id",
        )?;
        validate_length(
            self.role.as_ref().map_or(0, String::len),
            limits.max_role_bytes,
            "role",
        )?;
        validate_length(
            self.schema.as_ref().map_or(0, String::len),
            limits.max_schema_bytes,
            "schema",
        )?;
        validate_length(self.payload.len(), limits.max_payload_bytes, "payload")
    }
}

fn push_len(output: &mut Vec<u8>, len: usize) -> Result<(), SidecarProtocolError> {
    let len = u32::try_from(len).map_err(|_| SidecarProtocolError::FrameLengthOverflow)?;
    output.extend_from_slice(&len.to_le_bytes());
    Ok(())
}

fn read_u16(input: &[u8], offset: usize) -> Result<u16, SidecarProtocolError> {
    let bytes = input
        .get(offset..offset + 2)
        .ok_or(SidecarProtocolError::Truncated)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(input: &[u8], offset: usize) -> Result<u32, SidecarProtocolError> {
    let bytes = input
        .get(offset..offset + 4)
        .ok_or(SidecarProtocolError::Truncated)?;
    Ok(u32::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| SidecarProtocolError::Truncated)?,
    ))
}

fn read_u64(input: &[u8], offset: usize) -> Result<u64, SidecarProtocolError> {
    let bytes = input
        .get(offset..offset + 8)
        .ok_or(SidecarProtocolError::Truncated)?;
    Ok(u64::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_| SidecarProtocolError::Truncated)?,
    ))
}

fn read_text(
    input: &[u8],
    cursor: &mut usize,
    len: usize,
    field: &'static str,
) -> Result<String, SidecarProtocolError> {
    let end = cursor
        .checked_add(len)
        .ok_or(SidecarProtocolError::FrameLengthOverflow)?;
    let bytes = input
        .get(*cursor..end)
        .ok_or(SidecarProtocolError::Truncated)?;
    *cursor = end;
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| SidecarProtocolError::InvalidUtf8(field))
}

fn optional_text(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}

fn validate_length(
    actual: usize,
    maximum: usize,
    field: &'static str,
) -> Result<(), SidecarProtocolError> {
    if actual > maximum {
        Err(SidecarProtocolError::FieldTooLarge {
            field,
            actual,
            maximum,
        })
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SidecarProtocolError {
    #[error("sidecar frame is truncated")]
    Truncated,
    #[error("sidecar frame has trailing bytes")]
    TrailingBytes,
    #[error("sidecar frame magic is invalid")]
    InvalidMagic,
    #[error("sidecar protocol major {0} is unsupported")]
    UnsupportedMajor(u16),
    #[error("sidecar protocol minor {0} is unsupported")]
    UnsupportedMinor(u16),
    #[error("sidecar message kind {0} is unknown")]
    UnknownMessageKind(u8),
    #[error("sidecar terminal flag {0} is invalid")]
    InvalidTerminal(u8),
    #[error("sidecar reserved field is non-zero")]
    ReservedFieldSet,
    #[error("sidecar signal id is empty")]
    EmptySignalId,
    #[error("sidecar {field} length {actual} exceeds {maximum}")]
    FieldTooLarge {
        field: &'static str,
        actual: usize,
        maximum: usize,
    },
    #[error("sidecar {0} is not valid UTF-8")]
    InvalidUtf8(&'static str),
    #[error("sidecar frame length overflowed")]
    FrameLengthOverflow,
    #[error("sidecar frame exceeds the configured bound")]
    FrameTooLarge,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_core_extension_sidecar_message_when_round_tripped_then_identity_is_stable() {
        let message = SidecarMessage {
            kind: SidecarMessageKind::Signal,
            terminal: true,
            stream_id: 7,
            sequence_number: 11,
            timestamp_ns: 13,
            signal_id: "dev.pocketstation.fixture.v1".to_owned(),
            role: Some("result.final".to_owned()),
            schema: Some("urn:pocketstation:fixture:v1".to_owned()),
            payload: vec![1, 2, 3],
        };
        let limits = SidecarProtocolLimits::default();
        let encoded = message.encode(limits).expect("encoded");
        assert!(encoded.len() <= limits.max_frame_bytes().expect("bound"));
        assert_eq!(SidecarMessage::decode(&encoded, limits), Ok(message));
    }

    #[test]
    fn given_core_extension_oversized_sidecar_payload_when_encoded_then_fails_closed() {
        let limits = SidecarProtocolLimits {
            max_payload_bytes: 1,
            ..SidecarProtocolLimits::default()
        };
        let message = SidecarMessage {
            kind: SidecarMessageKind::Signal,
            terminal: false,
            stream_id: 1,
            sequence_number: 1,
            timestamp_ns: 1,
            signal_id: "pks.signal.control.v1".to_owned(),
            role: None,
            schema: None,
            payload: vec![1, 2],
        };
        assert!(matches!(
            message.encode(limits),
            Err(SidecarProtocolError::FieldTooLarge {
                field: "payload",
                ..
            })
        ));
    }
}

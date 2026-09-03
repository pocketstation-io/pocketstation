//! SignalSpec — typed signal declarations for graph ports.
//!
//! Every port in the graph declares a `SignalSpec`.  The compiler validates
//! `SignalClass` compatibility on edge connections; `SemanticRole` is advisory
//! metadata for humans, tooling, and port-matching hints — the core never
//! enumerates roles.
//!
//! Design constraints (CRATE_OWNERSHIP.md):
//!   - `Custom(SignalId)` uses a stable string identifier, NOT a Rust TypeId.
//!     Rust TypeId is process-local and breaks Python/Swift/Kotlin/Protobuf/remote ops.
//!   - `SemanticRole` is a free-form string, not an enum. Application semantics
//!     belong to the role layer, not the class.
//!   - `SchemaRef` identifies an external schema document; the core never parses it.

use std::borrow::Cow;

/// Opaque identifier for a custom signal type.
///
/// Use a stable reverse-domain identifier such as `"com.example.signal.v1"`.
/// This survives serialization, cross-language use, and remote operator manifests.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SignalId(Cow<'static, str>);

impl SignalId {
    pub fn new(id: impl Into<SignalId>) -> Self {
        id.into()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Reports whether this custom signal ID is portable across packages,
    /// languages, and processes.
    pub fn is_portable(&self) -> bool {
        crate::graph::identifier::is_portable_contract_id(self.as_str())
    }
}

impl From<&str> for SignalId {
    fn from(s: &str) -> Self {
        Self(Cow::Owned(s.to_owned()))
    }
}

impl From<String> for SignalId {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

/// Semantic role annotation on a port.
///
/// Free-form dot-namespaced string such as `"result.partial"` or
/// `"state.final"`. The core never matches on role values for routing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemanticRole(Cow<'static, str>);

impl SemanticRole {
    pub fn new(role: impl Into<SemanticRole>) -> Self {
        role.into()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for SemanticRole {
    fn from(s: &str) -> Self {
        Self(Cow::Owned(s.to_owned()))
    }
}

impl From<String> for SemanticRole {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

/// Reference to an external schema document.
///
/// Identifies the shape of a custom or structured signal — e.g. a Protobuf
/// message name, JSON Schema URI, or Flatbuffers table identifier.
/// The core never parses schema content; this is for tooling and validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaRef(Cow<'static, str>);

impl SchemaRef {
    pub fn new(schema: impl Into<SchemaRef>) -> Self {
        schema.into()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for SchemaRef {
    fn from(s: &str) -> Self {
        Self(Cow::Owned(s.to_owned()))
    }
}

impl From<String> for SchemaRef {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

/// Audio encoding format for `SignalClass::EncodedAudio`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Codec {
    Opus,
    Aac,
    Mp3,
    G711Ulaw,
    G711Alaw,
    WebmOpus,
}

/// Text encoding hint for `SignalClass::Text`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextFormat {
    Utf8,
    Json,
    Markdown,
}

/// Event structure hint for `SignalClass::Event`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventFormat {
    Json,
    Protobuf,
    Flatbuffers,
    Cbor,
}

/// Binary encoding hint for `SignalClass::Binary`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryFormat {
    Raw,
    Protobuf,
    Flatbuffers,
    Cbor,
}

/// The fundamental class of data flowing through a port.
///
/// The compiler validates class compatibility between connected ports.
/// `Custom` breaks out of the fixed taxonomy using a stable string identifier.
///
/// Do not add application semantics here; those belong in open roles.
/// Those belong in `SemanticRole` — a free-form label that the core never enumerates.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignalClass {
    /// Wildcard accepted only at deliberately open graph boundaries.
    Any,
    /// Interleaved PCM audio samples (format described by the edge AudioCaps).
    PcmAudio,
    /// Compressed audio bitstream (Opus packet, AAC frame, …).
    EncodedAudio(Codec),
    /// UTF-8 or structured text.
    Text(TextFormat),
    /// Discrete event payloads.
    Event(EventFormat),
    /// Telemetry and observability counters / gauges.
    Metrics,
    /// Graph control messages (route patches, session lifecycle, mute/unmute).
    Control,
    /// Opaque binary blob.
    Binary(BinaryFormat),
    /// Extension point for community / vendor signals.
    /// Use a stable reverse-domain identifier.
    Custom(SignalId),
}

impl SignalClass {
    /// Returns `true` for classes that carry real-time audio on the hot path.
    pub fn is_audio(self: &SignalClass) -> bool {
        matches!(self, Self::PcmAudio | Self::EncodedAudio(_))
    }

    /// Returns `true` if two signal classes are compatible for edge wiring.
    ///
    /// Compatibility is exact-match for concrete classes.  The compiler uses
    /// this to reject mismatched port wiring at graph validation time.
    pub fn is_compatible_with(&self, other: &SignalClass) -> bool {
        matches!(self, Self::Any) || matches!(other, Self::Any) || self == other
    }
}

/// Complete signal declaration for one port.
///
/// ```
/// use pocketstation::{SignalSpec, TextFormat};
///
/// // Mic audio port
/// let mic = SignalSpec::audio().with_role("voice");
///
/// // Application-defined structured result port
/// let result = SignalSpec::text(TextFormat::Utf8).with_role("result.partial");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalSpec {
    /// What kind of data (class-level compatibility — validated by compiler).
    pub(crate) class: SignalClass,
    /// What the data means (advisory; humans + tooling only).
    pub(crate) role: Option<SemanticRole>,
    /// Schema document reference for structured payloads.
    pub(crate) schema: Option<SchemaRef>,
}

impl SignalSpec {
    pub const fn class(&self) -> &SignalClass {
        &self.class
    }

    pub const fn role(&self) -> Option<&SemanticRole> {
        self.role.as_ref()
    }

    pub const fn schema(&self) -> Option<&SchemaRef> {
        self.schema.as_ref()
    }
    pub fn new(class: SignalClass) -> Self {
        Self {
            class,
            role: None,
            schema: None,
        }
    }

    /// Stable language-neutral identifier for the fundamental wire class.
    /// Semantic role and schema remain separate fields.
    pub fn wire_id(&self) -> &str {
        match &self.class {
            SignalClass::Any => "pks.signal.any.v1",
            SignalClass::PcmAudio => "pks.signal.pcm-audio.v1",
            SignalClass::EncodedAudio(Codec::Opus) => "pks.signal.encoded.opus.v1",
            SignalClass::EncodedAudio(Codec::Aac) => "pks.signal.encoded.aac.v1",
            SignalClass::EncodedAudio(Codec::Mp3) => "pks.signal.encoded.mp3.v1",
            SignalClass::EncodedAudio(Codec::G711Ulaw) => "pks.signal.encoded.g711-ulaw.v1",
            SignalClass::EncodedAudio(Codec::G711Alaw) => "pks.signal.encoded.g711-alaw.v1",
            SignalClass::EncodedAudio(Codec::WebmOpus) => "pks.signal.encoded.webm-opus.v1",
            SignalClass::Text(TextFormat::Utf8) => "pks.signal.text.utf8.v1",
            SignalClass::Text(TextFormat::Json) => "pks.signal.text.json.v1",
            SignalClass::Text(TextFormat::Markdown) => "pks.signal.text.markdown.v1",
            SignalClass::Event(EventFormat::Json) => "pks.signal.event.json.v1",
            SignalClass::Event(EventFormat::Protobuf) => "pks.signal.event.protobuf.v1",
            SignalClass::Event(EventFormat::Flatbuffers) => "pks.signal.event.flatbuffers.v1",
            SignalClass::Event(EventFormat::Cbor) => "pks.signal.event.cbor.v1",
            SignalClass::Metrics => "pks.signal.metrics.v1",
            SignalClass::Control => "pks.signal.control.v1",
            SignalClass::Binary(BinaryFormat::Raw) => "pks.signal.binary.raw.v1",
            SignalClass::Binary(BinaryFormat::Protobuf) => "pks.signal.binary.protobuf.v1",
            SignalClass::Binary(BinaryFormat::Flatbuffers) => "pks.signal.binary.flatbuffers.v1",
            SignalClass::Binary(BinaryFormat::Cbor) => "pks.signal.binary.cbor.v1",
            SignalClass::Custom(id) => id.as_str(),
        }
    }

    /// Convenience constructor for a deliberately open port.
    pub fn any() -> Self {
        Self::new(SignalClass::Any)
    }

    /// Convenience constructor for PCM audio ports.
    pub fn audio() -> Self {
        Self::new(SignalClass::PcmAudio)
    }

    /// Convenience constructor for encoded audio ports.
    pub fn encoded_audio(codec: Codec) -> Self {
        Self::new(SignalClass::EncodedAudio(codec))
    }

    /// Convenience constructor for text ports.
    pub fn text(format: TextFormat) -> Self {
        Self::new(SignalClass::Text(format))
    }

    /// Convenience constructor for event ports.
    pub fn event(format: EventFormat) -> Self {
        Self::new(SignalClass::Event(format))
    }

    /// Convenience constructor for metrics ports.
    pub fn metrics() -> Self {
        Self::new(SignalClass::Metrics)
    }

    /// Convenience constructor for control ports.
    pub fn control() -> Self {
        Self::new(SignalClass::Control)
    }

    /// Convenience constructor for opaque or schema-backed binary ports.
    pub fn binary(format: BinaryFormat) -> Self {
        Self::new(SignalClass::Binary(format))
    }

    /// Convenience constructor for custom / vendor extension ports.
    pub fn custom(id: impl Into<SignalId>) -> Self {
        Self::new(SignalClass::Custom(id.into()))
    }

    /// Attach a semantic role annotation.
    pub fn with_role(mut self, role: impl Into<SemanticRole>) -> Self {
        self.role = Some(role.into());
        self
    }

    /// Attach a schema reference.
    pub fn with_schema(mut self, schema: impl Into<SchemaRef>) -> Self {
        self.schema = Some(schema.into());
        self
    }

    /// Returns `true` if this spec can connect to `other` on an edge.
    ///
    /// Class must match exactly.  Role and schema are informational only
    /// and never block wiring.
    pub fn is_compatible_with(&self, other: &SignalSpec) -> bool {
        self.class.is_compatible_with(&other.class)
    }

    pub fn validate(&self) -> Result<(), SignalSpecError> {
        if matches!(&self.class, SignalClass::Custom(id) if id.as_str().trim().is_empty()) {
            return Err(SignalSpecError::EmptyCustomId);
        }
        if self
            .role
            .as_ref()
            .is_some_and(|role| role.as_str().trim().is_empty())
        {
            return Err(SignalSpecError::EmptyRole);
        }
        if self
            .schema
            .as_ref()
            .is_some_and(|schema| schema.as_str().trim().is_empty())
        {
            return Err(SignalSpecError::EmptySchema);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SignalSpecError {
    #[error("custom signal identifier cannot be empty")]
    EmptyCustomId,
    #[error("signal semantic role cannot be empty")]
    EmptyRole,
    #[error("signal schema reference cannot be empty")]
    EmptySchema,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_two_pcm_audio_specs_when_compatible_check_then_true() {
        let a = SignalSpec::audio().with_role("voice");
        let b = SignalSpec::audio().with_role("music"); // different role — irrelevant
        assert!(a.is_compatible_with(&b));
    }

    #[test]
    fn given_audio_and_text_specs_when_compatible_check_then_false() {
        let audio = SignalSpec::audio();
        let text = SignalSpec::text(TextFormat::Utf8);
        assert!(!audio.is_compatible_with(&text));
    }

    #[test]
    fn given_pcm_audio_spec_when_is_audio_then_true() {
        assert!(SignalClass::PcmAudio.is_audio());
        assert!(SignalClass::EncodedAudio(Codec::Opus).is_audio());
    }

    #[test]
    fn given_text_spec_when_is_audio_then_false() {
        assert!(!SignalClass::Text(TextFormat::Utf8).is_audio());
        assert!(!SignalClass::Control.is_audio());
        assert!(!SignalClass::Metrics.is_audio());
    }

    #[test]
    fn given_custom_signal_with_role_when_built_then_fields_accessible() {
        let spec = SignalSpec::custom("com.acme.sentiment.v1")
            .with_role("emotion.stress")
            .with_schema("proto:acme.Sentiment");
        assert_eq!(
            spec.class,
            SignalClass::Custom(SignalId::new("com.acme.sentiment.v1"))
        );
        assert_eq!(
            spec.role.as_ref().map(|r| r.as_str()),
            Some("emotion.stress")
        );
        assert_eq!(
            spec.schema.as_ref().map(|s| s.as_str()),
            Some("proto:acme.Sentiment")
        );
    }

    #[test]
    fn given_two_different_custom_ids_when_compatible_check_then_false() {
        let a = SignalSpec::custom("com.acme.foo.v1");
        let b = SignalSpec::custom("com.acme.bar.v1");
        assert!(!a.is_compatible_with(&b));
    }

    #[test]
    fn given_same_custom_ids_when_compatible_check_then_true() {
        let a = SignalSpec::custom("com.acme.foo.v1");
        let b = SignalSpec::custom("com.acme.foo.v1");
        assert!(a.is_compatible_with(&b));
    }

    #[test]
    fn given_signal_id_when_as_str_then_returns_inner() {
        let id = SignalId::new("com.acme.widget.v2");
        assert_eq!(id.as_str(), "com.acme.widget.v2");
    }

    #[test]
    fn given_semantic_role_when_as_str_then_returns_inner() {
        let role = SemanticRole::new("transcript.partial");
        assert_eq!(role.as_str(), "transcript.partial");
    }

    #[test]
    fn given_empty_custom_id_role_or_schema_when_validated_then_rejected() {
        assert_eq!(
            SignalSpec::custom("").validate(),
            Err(SignalSpecError::EmptyCustomId)
        );
        assert_eq!(
            SignalSpec::control().with_role("").validate(),
            Err(SignalSpecError::EmptyRole)
        );
        assert_eq!(
            SignalSpec::binary(BinaryFormat::Raw)
                .with_schema("")
                .validate(),
            Err(SignalSpecError::EmptySchema)
        );
    }
}

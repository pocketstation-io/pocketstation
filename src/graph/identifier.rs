//! Syntax checks for the distinct identifier categories owned by the graph.
//!
//! Portable identifiers shared by public graph types.
//! First-party structural and configuration conventions are test-only audits;
//! external providers retain their native identifier and configuration syntax.

pub(crate) const MAX_IDENTIFIER_BYTES: usize = 255;
#[cfg(test)]
pub(crate) const MAX_CONFIGURATION_KEY_BYTES: usize = 128;

pub(crate) fn is_portable_contract_id(value: &str) -> bool {
    if !is_bounded_ascii(value, MAX_IDENTIFIER_BYTES) {
        return false;
    }
    let Some((namespace, revision)) = value.rsplit_once('.') else {
        return false;
    };
    namespace.split('.').count() >= 3
        && namespace.split('.').all(is_kebab_segment)
        && is_revision_segment(revision)
}

#[cfg(test)]
pub(crate) fn is_first_party_structural_node_id(value: &str) -> bool {
    is_bounded_ascii(value, MAX_IDENTIFIER_BYTES) && value.split('.').all(is_snake_segment)
}

pub(crate) fn is_node_type_id(value: &str) -> bool {
    is_bounded_ascii(value, MAX_IDENTIFIER_BYTES) && value.split('.').all(is_node_segment)
}

#[cfg(test)]
pub(crate) fn is_protocol_id(value: &str) -> bool {
    if !is_bounded_ascii(value, MAX_IDENTIFIER_BYTES) {
        return false;
    }
    let Some((namespace, revision)) = value.rsplit_once('.') else {
        return false;
    };
    namespace.split('.').count() >= 2
        && namespace.split('.').all(is_kebab_segment)
        && is_revision_segment(revision)
}

#[cfg(test)]
pub(crate) fn is_first_party_configuration_key(value: &str) -> bool {
    value.len() <= MAX_CONFIGURATION_KEY_BYTES && is_snake_segment(value)
}

fn is_bounded_ascii(value: &str, maximum_bytes: usize) -> bool {
    !value.is_empty() && value.len() <= maximum_bytes && value.is_ascii()
}

fn is_kebab_segment(segment: &str) -> bool {
    is_segment(segment, b'-')
}

#[cfg(test)]
fn is_snake_segment(segment: &str) -> bool {
    is_segment(segment, b'_')
}

fn is_node_segment(segment: &str) -> bool {
    let bytes = segment.as_bytes();
    if bytes.is_empty()
        || !bytes[0].is_ascii_lowercase()
        || !bytes[bytes.len() - 1].is_ascii_alphanumeric()
    {
        return false;
    }
    let mut previous_was_separator = false;
    for byte in bytes {
        let is_separator = matches!(*byte, b'-' | b'_');
        if !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || is_separator)
            || (is_separator && previous_was_separator)
        {
            return false;
        }
        previous_was_separator = is_separator;
    }
    true
}

fn is_segment(segment: &str, separator: u8) -> bool {
    let bytes = segment.as_bytes();
    if bytes.is_empty()
        || !bytes[0].is_ascii_lowercase()
        || !bytes[bytes.len() - 1].is_ascii_alphanumeric()
    {
        return false;
    }
    let mut previous_was_separator = false;
    for byte in bytes {
        let is_separator = *byte == separator;
        if !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || is_separator)
            || (is_separator && previous_was_separator)
        {
            return false;
        }
        previous_was_separator = is_separator;
    }
    true
}

fn is_revision_segment(segment: &str) -> bool {
    let Some(revision) = segment.strip_prefix('v') else {
        return false;
    };
    !revision.is_empty()
        && revision.bytes().all(|byte| byte.is_ascii_digit())
        && !revision.starts_with('0')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoint::POLLED_AUDIO_OPERATOR_ID;
    use crate::graph::builtins::{
        GAIN_CONFIGURATION_KEY, GAIN_NODE_TYPE_ID, MONO_MIX_NODE_TYPE_ID, PASSTHROUGH_NODE_TYPE_ID,
    };
    use crate::recording::{MULTISTEM_GROUP_CONFIGURATION_KEY, MULTISTEM_NAME_CONFIGURATION_KEY};
    use crate::session::declaration::{
        BROWSER_NODE_TYPE_ID, BROWSER_OPERATOR_ID, BROWSER_RECEIVER_URI_CONFIGURATION_KEY,
        CONNECTOR_NODE_TYPE_ID,
    };
    use crate::session::extensions::builtins::{
        APPLICATION_SOURCE_NODE_TYPE_ID, EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID,
        GENERATED_AUDIO_BRIDGE_NODE_TYPE_ID, GENERATED_AUDIO_INGRESS_NODE_TYPE_ID,
        MICROPHONE_SOURCE_NODE_TYPE_ID,
    };
    use crate::session::extensions::{
        PCM_SOURCE_TYPE_ID, RECORDER_NODE_TYPE_ID, RECORDER_OPERATOR_ID,
    };
    use crate::{BinaryFormat, Codec, EventFormat, SignalSpec, TextFormat};

    #[test]
    fn given_first_party_contract_ids_when_audited_then_each_is_portable_and_versioned() {
        for identifier in [
            PCM_SOURCE_TYPE_ID,
            BROWSER_OPERATOR_ID,
            RECORDER_OPERATOR_ID,
            POLLED_AUDIO_OPERATOR_ID,
        ] {
            assert!(is_portable_contract_id(identifier), "{identifier}");
        }
    }

    #[test]
    fn given_first_party_structural_ids_when_audited_then_each_uses_snake_segments() {
        for identifier in [
            PASSTHROUGH_NODE_TYPE_ID,
            GAIN_NODE_TYPE_ID,
            MONO_MIX_NODE_TYPE_ID,
            APPLICATION_SOURCE_NODE_TYPE_ID,
            MICROPHONE_SOURCE_NODE_TYPE_ID,
            EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID,
            GENERATED_AUDIO_INGRESS_NODE_TYPE_ID,
            GENERATED_AUDIO_BRIDGE_NODE_TYPE_ID,
            CONNECTOR_NODE_TYPE_ID,
            BROWSER_NODE_TYPE_ID,
            RECORDER_NODE_TYPE_ID,
        ] {
            assert!(
                is_first_party_structural_node_id(identifier),
                "{identifier}"
            );
        }
    }

    #[test]
    fn given_first_party_configuration_keys_when_audited_then_each_is_bounded_snake_case() {
        for key in [
            GAIN_CONFIGURATION_KEY,
            BROWSER_RECEIVER_URI_CONFIGURATION_KEY,
            MULTISTEM_GROUP_CONFIGURATION_KEY,
            MULTISTEM_NAME_CONFIGURATION_KEY,
        ] {
            assert!(is_first_party_configuration_key(key), "{key}");
        }
    }

    #[test]
    fn given_shipped_signal_wire_ids_when_audited_then_protocol_namespace_stays_versioned() {
        for identifier in [
            SignalSpec::any().wire_id(),
            SignalSpec::audio().wire_id(),
            SignalSpec::encoded_audio(Codec::Opus).wire_id(),
            SignalSpec::encoded_audio(Codec::Aac).wire_id(),
            SignalSpec::encoded_audio(Codec::Mp3).wire_id(),
            SignalSpec::encoded_audio(Codec::G711Ulaw).wire_id(),
            SignalSpec::encoded_audio(Codec::G711Alaw).wire_id(),
            SignalSpec::encoded_audio(Codec::WebmOpus).wire_id(),
            SignalSpec::text(TextFormat::Utf8).wire_id(),
            SignalSpec::text(TextFormat::Json).wire_id(),
            SignalSpec::text(TextFormat::Markdown).wire_id(),
            SignalSpec::event(EventFormat::Json).wire_id(),
            SignalSpec::event(EventFormat::Protobuf).wire_id(),
            SignalSpec::event(EventFormat::Flatbuffers).wire_id(),
            SignalSpec::event(EventFormat::Cbor).wire_id(),
            SignalSpec::control().wire_id(),
            SignalSpec::metrics().wire_id(),
            SignalSpec::binary(BinaryFormat::Raw).wire_id(),
            SignalSpec::binary(BinaryFormat::Protobuf).wire_id(),
            SignalSpec::binary(BinaryFormat::Flatbuffers).wire_id(),
            SignalSpec::binary(BinaryFormat::Cbor).wire_id(),
            crate::runtime::CONTROL_SIGNAL_ID,
        ] {
            assert!(is_protocol_id(identifier), "{identifier}");
        }
    }

    #[test]
    fn given_wrong_category_syntax_when_validated_then_it_is_rejected() {
        assert!(!is_portable_contract_id("source.pcm"));
        assert!(!is_portable_contract_id("io.pocketstation.source.pcm.v0"));
        assert!(!is_first_party_structural_node_id("source.generated-audio"));
        assert!(!is_first_party_structural_node_id("Source.generated_audio"));
        assert!(is_node_type_id("io.example.source.generated-audio.v1"));
        assert!(is_node_type_id("source.generated_audio"));
        assert!(!is_node_type_id("source.generated audio"));
        assert!(!is_first_party_configuration_key("writer-token"));
        assert!(!is_first_party_configuration_key("__reservation"));
        assert!(!is_protocol_id("pks.signal.control"));
    }

    #[cfg(feature = "internal-testing")]
    #[test]
    fn given_internal_runtime_node_ids_when_audited_then_they_use_endpoint_vocabulary() {
        assert!(is_first_party_structural_node_id(
            crate::runtime::nodes::SYSTEM_OUTPUT_NODE_TYPE_ID
        ));
        assert!(is_first_party_structural_node_id(
            crate::runtime::nodes::BRIDGE_ENDPOINT_NODE_TYPE_ID
        ));
        assert!(crate::runtime::nodes::BRIDGE_ENDPOINT_NODE_TYPE_ID.starts_with("endpoint."));
    }
}

use std::collections::BTreeSet;

use serde_json::Value;

const VECTORS: &str = include_str!("../../protocol/conformance/connector/v1/vectors.json");

fn nonempty_text(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|text| !text.trim().is_empty())
}

fn bounded_text(value: Option<&Value>, maximum: usize) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|text| !text.trim().is_empty() && text.len() <= maximum)
}

fn positive_u64(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_u64)
        .is_some_and(|number| number > 0)
}

fn validate_manifest(case: &Value, limits: &Value) -> Result<(), &'static str> {
    let manifest = case.get("manifest").ok_or("connector.manifest.missing")?;
    if manifest.get("api_revision").and_then(Value::as_u64) != Some(1)
        || !positive_u64(manifest.get("manifest_revision"))
    {
        return Err("connector.manifest.invalid_revision");
    }
    let maximum_text = limits["maximum_manifest_text_bytes"]
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .expect("finite manifest text limit");
    if !bounded_text(manifest.get("package_id"), maximum_text)
        || !bounded_text(manifest.get("package_version"), maximum_text)
    {
        return Err("connector.manifest.invalid_identity");
    }

    let configuration = manifest
        .get("configuration")
        .ok_or("connector.configuration.missing_schema")?;
    if !positive_u64(configuration.get("revision")) {
        return Err("connector.configuration.invalid_revision");
    }
    let fields = configuration["fields"]
        .as_array()
        .ok_or("connector.configuration.invalid_fields")?;
    let maximum_fields = limits["maximum_configuration_fields"]
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .expect("finite field limit");
    if fields.len() > maximum_fields {
        return Err("connector.configuration.too_many_fields");
    }
    let mut field_names = BTreeSet::new();
    for field in fields {
        let name = field["name"]
            .as_str()
            .filter(|name| !name.is_empty())
            .ok_or("connector.configuration.invalid_field")?;
        if !field_names.insert(name) {
            return Err("connector.configuration.duplicate_field");
        }
        if !nonempty_text(field.get("documentation")) {
            return Err("connector.configuration.invalid_field");
        }
        let kind = field["kind"].as_str().unwrap_or_default();
        let requirement = field["requirement"].as_str().unwrap_or_default();
        if kind == "secret" && field.get("default").is_some() {
            return Err("connector.configuration.secret_default_forbidden");
        }
        if requirement == "defaulted" && field.get("default").is_none() {
            return Err("connector.configuration.default_missing");
        }
    }

    let components = manifest["components"]
        .as_array()
        .ok_or("connector.manifest.invalid_components")?;
    let maximum_entries = limits["maximum_manifest_entries"]
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .expect("finite entry limit");
    if components.len() > maximum_entries {
        return Err("connector.manifest.too_many_components");
    }
    let mut component_ids = BTreeSet::new();
    for component in components {
        let component_id = component["component_id"]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or("connector.manifest.invalid_component")?;
        if !component_ids.insert(component_id) {
            return Err("connector.manifest.duplicate_component");
        }
        if !matches!(
            component["kind"].as_str(),
            Some("source" | "operator" | "endpoint")
        ) {
            return Err("connector.manifest.invalid_component_kind");
        }
        let mut port_names = BTreeSet::new();
        for port in component["ports"]
            .as_array()
            .ok_or("connector.manifest.invalid_ports")?
        {
            let port_name = port["name"]
                .as_str()
                .filter(|name| !name.is_empty())
                .ok_or("connector.manifest.invalid_port")?;
            if !port_names.insert(port_name) {
                return Err("connector.manifest.duplicate_port");
            }
            if !matches!(port["direction"].as_str(), Some("input" | "output"))
                || !matches!(port["multiplicity"].as_str(), Some("one" | "many"))
                || !nonempty_text(port.get("signal_spec_id"))
            {
                return Err("connector.manifest.invalid_port");
            }
        }
    }

    if let Some(endpoint) = case.get("endpoint") {
        let endpoint_component = endpoint["component_id"].as_str().unwrap_or_default();
        if !components.iter().any(|component| {
            component["component_id"].as_str() == Some(endpoint_component)
                && component["kind"].as_str() == Some("endpoint")
        }) {
            return Err("connector.endpoint.unknown_component");
        }
        if !positive_u64(endpoint.get("maximum_inflight_items"))
            || !positive_u64(endpoint.get("maximum_payload_bytes"))
            || !positive_u64(endpoint.get("startup_deadline_ms"))
            || !positive_u64(endpoint.get("shutdown_deadline_ms"))
            || !positive_u64(endpoint.get("probe_interval_ms"))
            || !positive_u64(endpoint.get("success_threshold"))
            || !positive_u64(endpoint.get("failure_threshold"))
        {
            return Err("connector.endpoint.invalid_deadline");
        }
        if endpoint["probe_interval_ms"].as_u64() > endpoint["startup_deadline_ms"].as_u64() {
            return Err("connector.endpoint.invalid_deadline");
        }
    }
    Ok(())
}

fn validate_case(case: &Value, limits: &Value) -> Result<(), &'static str> {
    if let Some(status) = case.get("service_status") {
        if !matches!(
            status["delivery_readiness"].as_str(),
            Some("not_ready" | "ready")
        ) || !matches!(
            status["health"].as_str(),
            Some("healthy" | "degraded" | "unhealthy")
        ) || !matches!(status["recovery"].as_str(), Some("idle" | "reconnecting"))
            || !positive_u64(status.get("revision"))
        {
            return Err("connector.status.invalid");
        }
        return Ok(());
    }
    validate_manifest(case, limits)
}

#[test]
fn canonical_connector_vectors_match_core_contract_semantics() {
    let corpus: Value = serde_json::from_str(VECTORS).expect("canonical connector vectors");
    assert_eq!(corpus["schema_revision"], 1);
    assert_eq!(
        corpus["limits"]["maximum_manifest_entries"],
        pocketstation::connector::MAX_CONNECTOR_MANIFEST_ENTRIES
    );
    assert_eq!(
        corpus["limits"]["maximum_manifest_text_bytes"],
        pocketstation::connector::MAX_CONNECTOR_MANIFEST_TEXT_BYTES
    );
    assert_eq!(
        corpus["limits"]["maximum_configuration_fields"],
        pocketstation::connector::MAX_CONNECTOR_CONFIGURATION_FIELDS
    );
    assert_eq!(
        corpus["limits"]["maximum_configuration_text_bytes"],
        pocketstation::connector::MAX_CONNECTOR_CONFIGURATION_TEXT_BYTES
    );
    assert_eq!(
        corpus["limits"]["maximum_error_message_bytes"],
        pocketstation::connector::MAX_CONNECTOR_ERROR_MESSAGE_BYTES
    );

    let cases = corpus["cases"].as_array().expect("finite vector cases");
    assert!(!cases.is_empty());
    for case in cases {
        let result = validate_case(case, &corpus["limits"]);
        match case["expected"].as_str() {
            Some("accept") => assert!(result.is_ok(), "{}: {result:?}", case["id"]),
            Some("reject") => assert_eq!(
                result.expect_err("negative vector must reject"),
                case["error_code"].as_str().expect("stable rejection code"),
                "{}",
                case["id"]
            ),
            expectation => panic!("unknown vector expectation: {expectation:?}"),
        }
    }
}

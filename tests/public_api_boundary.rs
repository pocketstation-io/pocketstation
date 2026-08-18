//! Compile and source boundary for the normal (non-`internal-testing`) crate.

use std::sync::Arc;

use pocketstation::graph::SignalSpec;
use pocketstation::{
    ActiveCaptureBackend, AsyncNode, AsyncOperatorFactory, AudioBufferHandle, AudioBufferPool,
    CallbackCaptureBackend, CaptureDelivery, EndpointAudioFrame, EndpointAudioReceiver,
    EndpointDriverFactory, EndpointPortInput, EndpointSignalReceiver, NodeDefinition,
    PreparedCaptureBackend, PreparedEndpointDriver, RunningEndpointDriver, Session,
    SharedAudioBufferHandle, SharedAudioFrame, SignalEnvelope, SourceDriver, SourceFactory, Stream,
};

fn accepts_public_contract<T: ?Sized>() {}

#[test]
fn given_supported_contracts_when_named_from_crate_root_then_they_compile() {
    accepts_public_contract::<Session>();
    accepts_public_contract::<Stream<SignalEnvelope>>();
    accepts_public_contract::<dyn CallbackCaptureBackend>();
    accepts_public_contract::<dyn PreparedCaptureBackend>();
    accepts_public_contract::<dyn ActiveCaptureBackend>();
    accepts_public_contract::<CaptureDelivery>();
    accepts_public_contract::<dyn SourceFactory>();
    accepts_public_contract::<dyn SourceDriver>();
    accepts_public_contract::<dyn AsyncOperatorFactory>();
    accepts_public_contract::<dyn AsyncNode>();
    accepts_public_contract::<dyn NodeDefinition>();
    accepts_public_contract::<dyn EndpointDriverFactory>();
    accepts_public_contract::<EndpointPortInput>();
    accepts_public_contract::<EndpointAudioReceiver>();
    accepts_public_contract::<EndpointAudioFrame>();
    accepts_public_contract::<EndpointSignalReceiver>();
    accepts_public_contract::<dyn PreparedEndpointDriver>();
    accepts_public_contract::<dyn RunningEndpointDriver>();
    accepts_public_contract::<Arc<AudioBufferPool>>();
    accepts_public_contract::<AudioBufferHandle>();
    accepts_public_contract::<SharedAudioBufferHandle>();
    accepts_public_contract::<SharedAudioFrame>();
    accepts_public_contract::<SignalSpec>();
}

#[test]
fn given_normal_crate_root_when_scanned_then_implementation_owners_are_private() {
    let source = include_str!("../src/lib.rs");
    let public_prelude = source
        .split("pub mod internal")
        .next()
        .expect("normal public prelude");
    for forbidden in [
        "AsyncOperatorInput,",
        "AsyncOperatorNamedOutput",
        "AsyncOperatorTypedInput",
        "AsyncOperatorWorker",
        "EndpointDriverRegistry",
        "EndpointPrepareError",
        "PlanRunnerCancellation",
        "PlanSourceSender",
        "PreparedSourceRuntime",
        "SourceOutputBranchSpec",
        "SourceOutputReceiver",
        "SourceRegistry",
        "SourceRuntime,",
        "TypedEdgeBranchSpec",
        "TypedEdgeFanout",
        "TypedEdgeReceiver",
        "plan_source_channel",
    ] {
        assert!(
            !public_prelude.contains(forbidden),
            "implementation owner leaked from crate root: {forbidden}"
        );
    }
    for private_module in ["abi", "capture", "frame", "recording", "runtime", "session"] {
        assert!(
            public_prelude.contains(&format!("mod {private_module};")),
            "implementation module is not private: {private_module}"
        );
        assert!(
            !public_prelude.contains(&format!("pub mod {private_module};")),
            "implementation module leaked publicly: {private_module}"
        );
    }
    assert!(public_prelude.contains("pub mod graph;"));
}

#[test]
fn given_endpoint_spi_when_source_is_scanned_then_connector_policy_never_flows_downward() {
    let endpoint_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/endpoint");
    for path in rust_sources(&endpoint_root) {
        let source = std::fs::read_to_string(&path).expect("endpoint source");
        assert!(
            !source.contains("crate::connector") && !source.contains("connector::"),
            "endpoint SPI must not depend on connector policy: {}",
            path.display()
        );
    }
}

#[test]
fn given_connector_authoring_layer_when_scanned_then_it_does_not_duplicate_core_runtime_policy() {
    let connector_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/connector");
    let sources = rust_sources(&connector_root);
    let combined = sources
        .iter()
        .map(|path| std::fs::read_to_string(path).expect("connector source"))
        .collect::<Vec<_>>()
        .join("\n");
    for forbidden in [
        "enum ConnectorLifecycle",
        "enum ConnectorReadiness {",
        "struct ConnectorDeliveryPolicy",
        "struct ConnectorRetryPolicy",
        "worker_queue_capacity_items",
        "items_received_total",
        "items_delivered_total",
        "items_dropped_total",
        "pub struct ConnectorPackage",
        "pub mod conformance",
        "pub const fn input_edge",
        "DeliveryMediaMismatch",
        "source_registrations.lock",
        "operator_registrations.lock",
        "endpoint_extensions.lock",
        "endpoint_registrations.lock",
    ] {
        assert!(
            !combined.contains(forbidden),
            "connector authoring layer duplicated canonical runtime policy: {forbidden}"
        );
    }
}

fn rust_sources(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory).expect("source directory") {
            let path = entry.expect("source entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(std::ffi::OsStr::to_str) == Some("rs") {
                sources.push(path);
            }
        }
    }
    sources
}

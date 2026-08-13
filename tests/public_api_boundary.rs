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

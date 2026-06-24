use pocketstation_frame::SourceId;

#[derive(Debug, Clone)]
pub enum RouteKind {
    LocalPlayback,
    LanDirect,
    CloudRelay,
    VoiceAgentBackend,
    RecordingFile,
    VirtualMicrophone,
    PeerToPeer,
    HardwareBridge,
    PublicChannel,
    PrivateRoom,
}

#[derive(Debug, Clone)]
pub enum TransportKind {
    Local,
    WebRtc,
    RtpUdp,
    File,
}

#[derive(Debug, Clone)]
pub enum RouteEncryptionMode {
    TransportOnly,
    SFrameE2EE,
    EnterpriseKeyManager,
}

// Backwards-compat alias.
pub use RouteEncryptionMode as EncryptionMode;

#[derive(Debug, Clone)]
pub enum OutputTarget {
    LocalSpeaker,
    BluetoothDevice,
    WebListener,
    MobileApp,
    DesktopApp,
    VoiceAgentBackend,
    RecordingFile,
    VirtualMicrophone,
    PublicRoom,
    PrivateRoom,
    HardwareAccessory,
}

#[derive(Debug, Clone)]
pub struct RoutePlan {
    pub source: SourceId,
    pub outputs: Vec<OutputTarget>,
    pub transport: TransportKind,
    pub encryption: RouteEncryptionMode,
    pub latency_budget_ms: u32,
    pub fallback_routes: Vec<RouteKind>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocketstation_frame::SourceId;

    #[test]
    fn given_route_plan_when_constructed_then_fields_accessible() {
        let plan = RoutePlan {
            source: SourceId(1),
            outputs: vec![OutputTarget::WebListener],
            transport: TransportKind::WebRtc,
            encryption: RouteEncryptionMode::TransportOnly,
            latency_budget_ms: 100,
            fallback_routes: vec![RouteKind::CloudRelay],
        };
        assert!(matches!(plan.transport, TransportKind::WebRtc));
    }
}

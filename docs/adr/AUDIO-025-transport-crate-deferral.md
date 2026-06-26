# AUDIO-025 — pocketstation-transport crate deferral

**Status:** Accepted  
**Date:** 2026-06-24  
**Deciders:** Raphael Avocegamou

---

## Context

`pocketstation-transport` was created as part of the P1/P10 crate boundary split
(AUDIO-008 §26.5) to hold route planning and transport metadata types. The intent
was correct: a clean boundary between the audio processing layer and the transport
layer. The execution was not.

What was built:

```rust
enum RouteKind       { LocalPlayback, LanDirect, CloudRelay, VoiceAgentBackend,
                       RecordingFile, VirtualMicrophone, PeerToPeer,
                       HardwareBridge, PublicChannel, PrivateRoom }
enum TransportKind   { Local, WebRtc, RtpUdp, File }
enum RouteEncryptionMode { TransportOnly, SFrameE2EE, EnterpriseKeyManager }
struct RoutePlan     { source, outputs, transport, encryption,
                       latency_budget_ms, fallback_routes }
```

None of these types are used by any real caller. `RoutePlan` has no execution
engine. `RouteKind` has speculative variants (`PeerToPeer`, `HardwareBridge`,
`PublicChannel`) that correspond to no implemented path. The relay is Go/Pion and
cannot import a Rust crate. iOS and Android SDKs use native WebRTC APIs. The Rust
audio-core produces Opus frames and hands them to the SDK layer — it does not
initiate or manage transport connections.

A standalone crate signals a stable, consumed public API boundary. This one has
zero external callers and zero internal dispatch logic. It is scaffolding
presented as architecture.

---

## Decision

Delete `pocketstation-transport` as a standalone crate.

Move the two types with legitimate near-term use into `pocketstation-audio`
as inline definitions:

```rust
// pocketstation-audio/src/lib.rs

/// Transport mechanism used to carry encoded audio frames.
/// Used for session metadata and FAKE_SCAFFOLD_INVENTORY tracking.
/// Not dispatched on until a Rust transport layer exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportKind { Local, WebRtc, RtpUdp, File }

/// Encryption mode applied at the transport or frame layer.
/// SFrameE2EE follows RFC 9605. EnterpriseKeyManager is deferred to Phase 5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteEncryptionMode { TransportOnly, SFrameE2EE, EnterpriseKeyManager }
```

`RouteKind`, `RoutePlan`, `OutputTarget` (transport's copy), and `EncryptionMode`
alias are deleted entirely.

---

## When to create a real transport crate

Create `pocketstation-transport` as a proper crate when **at least one** of these
is true:

1. **Rust relay exists** — a Rust process receives and forwards RTP packets,
   requiring `RtpFrame`, `TransportSession` trait, and concrete impls.
2. **Direct P2P path** — Rust-native peer-to-peer without the Go relay,
   requiring `IceCandidate`, `DtlsSession`, `SrtpContext`.
3. **Multi-transport backend** — a single `TransportSession` trait dispatches
   across WebRTC / raw RTP-UDP / QUIC / recording-file, requiring the trait
   abstraction to be shared across crates.

At that point the crate's minimum viable surface is:

```rust
pub struct RtpFrame {
    pub sequence:     u16,
    pub timestamp:    u32,
    pub ssrc:         u32,
    pub payload_type: u8,
    pub payload:      Bytes,
}

pub trait TransportSession: Send {
    fn send(&mut self, frame: RtpFrame) -> Result<(), TransportError>;
    fn recv(&mut self) -> Result<RtpFrame, TransportError>;
    fn close(&mut self);
}

pub struct WebRtcSession { ... }
pub struct UdpSession    { ... }
pub struct FileSession   { ... }
```

`TransportKind` and `RouteEncryptionMode` migrate from `pocketstation-audio` into
this crate at that time.

---

## Consequences

- `pocketstation-audio/Cargo.toml`: removes `pocketstation-transport` dependency.
- `Cargo.toml` (workspace): removes `crates/pocketstation-transport` from members.
- `scripts/publish.sh`: removes transport from publish order.
- `docs/standards/FAKE_SCAFFOLD_INVENTORY.md`: no row needed — the crate is gone,
  not stubbed.
- Future: when the real crate is created, add it to `FAKE_SCAFFOLD_INVENTORY.md`
  with explicit "replace by Phase N" gate before any impl is marked production.

---

## Rejected alternatives

**Keep the crate, mark it DEFERRED in FAKE_SCAFFOLD_INVENTORY.**  
Rejected. A DEFERRED row implies a real implementation is planned at a known
phase. The current types are not the right types for the real implementation —
`RoutePlan` would be rewritten from scratch. Keeping the crate preserves an
incorrect mental model of the boundary.

**Expand the crate now to include real RTP types.**  
Rejected. The Rust layer does not own transport today. Premature abstraction over
a boundary the Go relay currently owns creates a fake API contract with no
consumer and no way to validate it against a real wire format.

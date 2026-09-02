use crate::frame::ClockDomainId;

const NANOSECOND_TICKS_PER_SECOND: u64 = 1_000_000_000;

/// The clock source that defines timestamps carried by one clock-domain ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockDomainKind {
    Unspecified,
    ProcessMonotonic,
    ProviderDefined,
}

/// The origin against which timestamps in one clock domain are measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockDomainOrigin {
    Unspecified,
    ProcessStart,
    ProviderDefined,
}

/// Finite description of a clock identity carried by frame and signal lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockDomainDescriptor {
    id: ClockDomainId,
    kind: ClockDomainKind,
    origin: ClockDomainOrigin,
    tick_rate_hz: Option<u64>,
}

impl ClockDomainDescriptor {
    pub const fn id(self) -> ClockDomainId {
        self.id
    }

    pub const fn kind(self) -> ClockDomainKind {
        self.kind
    }

    pub const fn origin(self) -> ClockDomainOrigin {
        self.origin
    }

    pub const fn tick_rate_hz(self) -> Option<u64> {
        self.tick_rate_hz
    }
}

/// Describes the stable semantics Core can assert for a clock-domain ID.
///
/// ID zero means no clock source was declared. ID one is PocketStation's
/// process-wide monotonic nanosecond clock. Other IDs are owned by the Source
/// or provider that introduced them; Core preserves their identity and
/// nanosecond unit without inventing their epoch.
pub const fn describe_clock_domain(id: ClockDomainId) -> ClockDomainDescriptor {
    match id.get() {
        0 => ClockDomainDescriptor {
            id,
            kind: ClockDomainKind::Unspecified,
            origin: ClockDomainOrigin::Unspecified,
            tick_rate_hz: None,
        },
        1 => ClockDomainDescriptor {
            id,
            kind: ClockDomainKind::ProcessMonotonic,
            origin: ClockDomainOrigin::ProcessStart,
            tick_rate_hz: Some(NANOSECOND_TICKS_PER_SECOND),
        },
        _ => ClockDomainDescriptor {
            id,
            kind: ClockDomainKind::ProviderDefined,
            origin: ClockDomainOrigin::ProviderDefined,
            tick_rate_hz: Some(NANOSECOND_TICKS_PER_SECOND),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_known_and_provider_clocks_when_described_then_authority_is_not_inferred() {
        let unspecified = describe_clock_domain(ClockDomainId::new(0));
        assert_eq!(unspecified.kind(), ClockDomainKind::Unspecified);
        assert_eq!(unspecified.origin(), ClockDomainOrigin::Unspecified);
        assert_eq!(unspecified.tick_rate_hz(), None);

        let process = describe_clock_domain(ClockDomainId::new(1));
        assert_eq!(process.kind(), ClockDomainKind::ProcessMonotonic);
        assert_eq!(process.origin(), ClockDomainOrigin::ProcessStart);
        assert_eq!(process.tick_rate_hz(), Some(1_000_000_000));

        let provider = describe_clock_domain(ClockDomainId::new(42));
        assert_eq!(provider.kind(), ClockDomainKind::ProviderDefined);
        assert_eq!(provider.origin(), ClockDomainOrigin::ProviderDefined);
        assert_eq!(provider.tick_rate_hz(), Some(1_000_000_000));
    }
}

//! Compact stable identities carried by plans and hot-path frames.

macro_rules! u64_identity {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub(crate) u64);

        impl $name {
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

u64_identity!(StreamId);
u64_identity!(SourceId);
u64_identity!(SessionId);
u64_identity!(StemId);
u64_identity!(EndpointId);
u64_identity!(ConnectorId);
u64_identity!(RouteId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClockDomainId(pub(crate) u32);

impl ClockDomainId {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

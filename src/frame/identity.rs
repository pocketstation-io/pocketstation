//! Compact stable identities carried by plans and hot-path frames.

macro_rules! u64_identity {
    ($name:ident, $description:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[doc = $description]
        pub struct $name(pub(crate) u64);

        impl $name {
            #[doc = concat!("Creates a new `", stringify!($name), "`.")]
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            #[doc = concat!("Returns the numeric value held by `", stringify!($name), "`.")]
            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

u64_identity!(StreamId, "Uniquely identifies a declared stream.");
u64_identity!(SourceId, "Uniquely identifies a capture source.");
u64_identity!(SessionId, "Uniquely identifies a Session.");
u64_identity!(StemId, "Uniquely identifies an audio stem.");
u64_identity!(EndpointId, "Uniquely identifies an endpoint.");
u64_identity!(ConnectorId, "Uniquely identifies a connector.");
u64_identity!(RouteId, "Uniquely identifies a compiled route.");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = "Uniquely identifies clock domain."]
pub struct ClockDomainId(pub(crate) u32);

impl ClockDomainId {
    #[doc = "Creates a new `ClockDomainId`."]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[doc = "Returns the value held by `ClockDomainId`."]
    pub const fn get(self) -> u32 {
        self.0
    }
}

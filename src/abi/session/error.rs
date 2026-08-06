use crate::abi::session::abi::{PksSessionStatus, PksSessionStatusCode};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AbiError {
    #[error("output pointer is null")]
    NullArgument,
    #[error("requested ABI major is unsupported")]
    UnsupportedAbiMajor,
    #[error("requested ABI minor is newer than this engine")]
    UnsupportedAbiMinor,
    #[error("record struct_size is smaller than required")]
    InvalidStructSize,
    #[error("handle is invalid")]
    InvalidHandle,
    #[error("handle is stale")]
    StaleHandle,
    #[error("handle table has no free slot")]
    NoCapacity,
    #[error("pointer alignment does not satisfy the record contract")]
    MisalignedPointer,
    #[error("argument is invalid")]
    InvalidArgument,
    #[error("handle belongs to another engine")]
    ForeignHandle,
    #[error("operation is invalid for the current lifecycle state")]
    InvalidLifecycleState,
    #[error("bounded queue is empty")]
    WouldBlock,
    #[error("native Session engine operation failed")]
    BackendFailure,
    #[error("operation was cancelled")]
    Cancelled,
    #[error("index is outside the bounded result")]
    IndexOutOfRange,
}

impl AbiError {
    pub const fn status(self) -> PksSessionStatus {
        match self {
            Self::NullArgument => PksSessionStatus::new(PksSessionStatusCode::NullArgument, 0),
            Self::UnsupportedAbiMajor => {
                PksSessionStatus::new(PksSessionStatusCode::UnsupportedAbiMajor, 0)
            }
            Self::UnsupportedAbiMinor => {
                PksSessionStatus::new(PksSessionStatusCode::UnsupportedAbiMinor, 0)
            }
            Self::InvalidStructSize => {
                PksSessionStatus::new(PksSessionStatusCode::InvalidStructSize, 0)
            }
            Self::InvalidHandle => PksSessionStatus::new(PksSessionStatusCode::InvalidHandle, 0),
            Self::StaleHandle => PksSessionStatus::new(PksSessionStatusCode::StaleHandle, 0),
            Self::NoCapacity => PksSessionStatus::new(PksSessionStatusCode::NoCapacity, 0),
            Self::MisalignedPointer => {
                PksSessionStatus::new(PksSessionStatusCode::MisalignedPointer, 0)
            }
            Self::InvalidArgument => {
                PksSessionStatus::new(PksSessionStatusCode::InvalidArgument, 0)
            }
            Self::ForeignHandle => PksSessionStatus::new(PksSessionStatusCode::ForeignHandle, 0),
            Self::InvalidLifecycleState => {
                PksSessionStatus::new(PksSessionStatusCode::InvalidLifecycleState, 0)
            }
            Self::WouldBlock => PksSessionStatus::new(PksSessionStatusCode::WouldBlock, 0),
            Self::BackendFailure => PksSessionStatus::new(PksSessionStatusCode::BackendFailure, 0),
            Self::Cancelled => PksSessionStatus::new(PksSessionStatusCode::Cancelled, 0),
            Self::IndexOutOfRange => {
                PksSessionStatus::new(PksSessionStatusCode::IndexOutOfRange, 0)
            }
        }
    }
}

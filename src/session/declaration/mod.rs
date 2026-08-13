//! Public Session declaration model.
//!
//! This module owns the developer-facing declaration state only. It does not
//! compile graphs, allocate runtime queues, or start platform resources.

mod draft;
mod endpoint;
#[cfg(test)]
mod operator_connection_tests;
mod selector;
mod spec;
mod typed_stream;

pub use draft::{
    DerivedStreamHandle, EndpointHandle, Operator, OperatorInputHandle, OperatorInstanceHandle,
    Session, SourceInstanceHandle, SourceOutputHandle, StemHandle,
};
pub use endpoint::{
    EndpointConfiguration, EndpointDescriptor, OperatorId, BROWSER_NODE_TYPE_ID,
    BROWSER_OPERATOR_ID, CONNECTOR_NODE_TYPE_ID,
};
pub use selector::{ApplicationSelector, DeviceId, DeviceSelector, ProcessId, Source};
#[cfg(any(test, feature = "internal-testing"))]
pub use spec::StemSpec;
pub use spec::{
    ConnectionSpec, ConnectionTarget, EndpointSpec, OperatorInstanceId, OperatorInstanceSpec,
    SessionSpec, SourceInstanceId, SourceInstanceSpec, SourceOutputSpec, StreamOrigin,
};
#[cfg(any(test, feature = "internal-testing"))]
#[allow(deprecated)]
pub use spec::{OperatorSpec, SessionSpecVersion, SESSION_SPEC_VERSION};
pub use typed_stream::{Stream, StreamSignal, TypedOperator, TypedStreamError};

use std::marker::PhantomData;

use super::draft::InternalStreamHandle;
use crate::graph::{
    AsyncOperatorManifest, OperatorId, PortDirection, PortSpec, SignalClass, SignalSpec,
};
use crate::session::{
    EndpointHandle, Operator, RouteId, SessionError, SourceOutputHandle, StemHandle,
};

/// Compile-time marker supplied by an SDK or external package.
///
/// `SignalSpec` remains the runtime and cross-language representation. Rust type
/// identity is never serialized or exposed through the C ABI.
pub trait StreamSignal: Send + Sync + 'static {
    fn signal_spec() -> SignalSpec;
}

#[derive(Debug, Clone)]
pub struct TypedOperator<Input, Output> {
    operator: Operator,
    input_port: String,
    output_port: String,
    input_spec: SignalSpec,
    output_spec: SignalSpec,
    marker: PhantomData<fn(Input) -> Output>,
}

impl<Input: StreamSignal, Output: StreamSignal> TypedOperator<Input, Output> {
    pub fn new(
        operator: Operator,
        manifest: &AsyncOperatorManifest,
        input_port: Option<&str>,
        output_port: Option<&str>,
    ) -> Result<Self, TypedStreamError> {
        manifest
            .validate()
            .map_err(|error| TypedStreamError::InvalidManifest(error.to_string()))?;
        if manifest.operator_id != *operator.operator_id() {
            return Err(TypedStreamError::OperatorIdentityMismatch {
                declaration: operator.operator_id().as_str().to_owned(),
                manifest: manifest.operator_id.as_str().to_owned(),
            });
        }
        let input = select_port(manifest.input_ports(), input_port, PortDirection::Input)?;
        let output = select_port(manifest.output_ports(), output_port, PortDirection::Output)?;
        let input_spec = Input::signal_spec();
        let output_spec = Output::signal_spec();
        validate_signal(&input_spec)?;
        validate_signal(&output_spec)?;
        if input.signal != input_spec {
            return Err(TypedStreamError::InputSignalMismatch {
                port: input.name.clone(),
            });
        }
        if output.signal != output_spec {
            return Err(TypedStreamError::OutputSignalMismatch {
                port: output.name.clone(),
            });
        }
        Ok(Self {
            operator,
            input_port: input.name.clone(),
            output_port: output.name.clone(),
            input_spec,
            output_spec,
            marker: PhantomData,
        })
    }

    pub const fn operator_id(&self) -> &OperatorId {
        self.operator.operator_id()
    }

    pub fn input_port(&self) -> &str {
        &self.input_port
    }

    pub fn output_port(&self) -> &str {
        &self.output_port
    }

    pub const fn input_spec(&self) -> &SignalSpec {
        &self.input_spec
    }

    pub const fn output_spec(&self) -> &SignalSpec {
        &self.output_spec
    }
}

/// Typed Rust declaration façade compiled into stable dynamic signal, schema,
/// port, and EdgeContract settings. This wrapper carries no frames and is not a
/// generic runtime queue.
#[derive(Clone)]
pub struct Stream<Signal> {
    handle: InternalStreamHandle,
    signal_spec: SignalSpec,
    marker: PhantomData<fn() -> Signal>,
}

impl<Signal: StreamSignal> Stream<Signal> {
    pub fn from_stem(stem: StemHandle) -> Result<Self, TypedStreamError> {
        let signal_spec = Signal::signal_spec();
        validate_signal(&signal_spec)?;
        if !matches!(signal_spec.class, SignalClass::PcmAudio) {
            return Err(TypedStreamError::StemRequiresPcmAudio);
        }
        Ok(Self {
            handle: stem.stream,
            signal_spec,
            marker: PhantomData,
        })
    }

    /// Wraps a public external-source output in the same typed Rust façade.
    /// Runtime identity remains the output's stable `SignalSpec` and schema.
    pub fn from_source_output(output: SourceOutputHandle) -> Result<Self, TypedStreamError> {
        let signal_spec = Signal::signal_spec();
        validate_signal(&signal_spec)?;
        Ok(Self {
            handle: output.stream,
            signal_spec,
            marker: PhantomData,
        })
    }

    pub const fn signal_spec(&self) -> &SignalSpec {
        &self.signal_spec
    }

    pub fn through<Output: StreamSignal>(
        self,
        operator: TypedOperator<Signal, Output>,
    ) -> Result<Stream<Output>, TypedStreamError> {
        if self.signal_spec != operator.input_spec {
            return Err(TypedStreamError::StreamInputMismatch);
        }
        let derived = self.handle.through_ports(
            operator.operator,
            Some(operator.input_port),
            Some(operator.output_port),
        )?;
        Ok(Stream {
            handle: derived.stream,
            signal_spec: Output::signal_spec(),
            marker: PhantomData,
        })
    }

    pub fn send(&self, endpoint: EndpointHandle) -> Result<RouteId, TypedStreamError> {
        self.handle.send_to(endpoint, None).map_err(Into::into)
    }
}

fn select_port<'a>(
    mut ports: impl Iterator<Item = &'a PortSpec>,
    selected: Option<&str>,
    direction: PortDirection,
) -> Result<&'a PortSpec, TypedStreamError> {
    if let Some(selected) = selected {
        return ports.find(|port| port.name == selected).ok_or_else(|| {
            TypedStreamError::UnknownPort {
                direction,
                port: selected.to_owned(),
            }
        });
    }
    let first = ports
        .next()
        .ok_or(TypedStreamError::MissingPort { direction })?;
    if ports.next().is_some() {
        return Err(TypedStreamError::AmbiguousPort { direction });
    }
    Ok(first)
}

fn validate_signal(signal: &SignalSpec) -> Result<(), TypedStreamError> {
    signal
        .validate()
        .map_err(|error| TypedStreamError::InvalidSignal(error.to_string()))
}

#[derive(Debug, thiserror::Error)]
pub enum TypedStreamError {
    #[error("typed stream signal is invalid: {0}")]
    InvalidSignal(String),
    #[error("typed operator manifest is invalid: {0}")]
    InvalidManifest(String),
    #[error("operator declaration '{declaration}' does not match manifest '{manifest}'")]
    OperatorIdentityMismatch {
        declaration: String,
        manifest: String,
    },
    #[error("typed stream port '{port}' is not declared for direction {direction:?}")]
    UnknownPort {
        direction: PortDirection,
        port: String,
    },
    #[error("typed stream manifest has no port for direction {direction:?}")]
    MissingPort { direction: PortDirection },
    #[error("typed stream manifest requires explicit port selection for direction {direction:?}")]
    AmbiguousPort { direction: PortDirection },
    #[error("typed operator input marker does not match port '{port}'")]
    InputSignalMismatch { port: String },
    #[error("typed operator output marker does not match port '{port}'")]
    OutputSignalMismatch { port: String },
    #[error("typed capture stem markers must describe PCM audio")]
    StemRequiresPcmAudio,
    #[error("typed stream output does not match the next operator input")]
    StreamInputMismatch,
    #[error(transparent)]
    Session(#[from] SessionError),
}

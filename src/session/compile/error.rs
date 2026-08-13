//! Typed failures produced while lowering and compiling a Session declaration.

use crate::graph::compile::CompileError;
use crate::session::{OperatorInstanceId, SessionError, SourceTypeId};

#[derive(Debug, thiserror::Error)]
pub enum SessionCompileError {
    #[error(transparent)]
    InvalidSpec(#[from] SessionError),
    #[error("operator {operator_id} is not registered")]
    UnknownOperator { operator_id: String },
    #[error(
        "operator {operator_id} resolves to node type {registered_node_type_id}, not {declared_node_type_id}"
    )]
    OperatorNodeTypeMismatch {
        operator_id: String,
        registered_node_type_id: String,
        declared_node_type_id: String,
    },
    #[error("async operator {operator_id} is not registered")]
    UnknownAsyncOperator { operator_id: String },
    #[error("derived endpoint node type {node_type_id} is not registered")]
    UnknownEndpointNodeType { node_type_id: String },
    #[error(
        "derived endpoint node type {node_type_id} has {input_ports_total} inputs; send(destination) requires exactly one"
    )]
    AmbiguousEndpointInput {
        node_type_id: String,
        input_ports_total: usize,
    },
    #[error("operator {operator_id} requires explicit {direction} port selection")]
    AmbiguousOperatorPort {
        operator_id: String,
        direction: &'static str,
    },
    #[error("operator {operator_id} has no {direction} port named '{port_name}'")]
    UnknownOperatorPort {
        operator_id: String,
        direction: &'static str,
        port_name: String,
    },
    #[error("operator instance {operator_instance_id:?} required input port '{port_name}' is not connected")]
    MissingRequiredOperatorInput {
        operator_instance_id: OperatorInstanceId,
        port_name: String,
    },
    #[error(
        "operator instance {operator_instance_id:?} output '{output_port}' cannot enter the audio bridge because it is not concrete PCM"
    )]
    InvalidAudioBridgeOutput {
        operator_instance_id: OperatorInstanceId,
        output_port: String,
    },
    #[error(
        "operator instance {operator_instance_id:?} output '{output_port}' must have exactly one generated-audio consumer"
    )]
    AudioBridgeOutputNotExclusive {
        operator_instance_id: OperatorInstanceId,
        output_port: String,
    },
    #[error("operator instance {operator_instance_id:?} input port '{port_name}' is connected more than once")]
    DuplicateOperatorInputConnection {
        operator_instance_id: OperatorInstanceId,
        port_name: String,
    },
    #[error("required source node type {node_type_id} is not registered")]
    UnknownSourceNodeType { node_type_id: String },
    #[error("external source type {source_type_id} is not registered on SessionEngine")]
    UnknownExternalSource { source_type_id: SourceTypeId },
    #[error(
        "external source type {source_type_id} has no declared output port named '{output_port}'"
    )]
    UnknownExternalSourceOutput {
        source_type_id: SourceTypeId,
        output_port: String,
    },
    #[error("external source type {source_type_id} configuration is invalid: {reason}")]
    InvalidExternalSourceConfiguration {
        source_type_id: SourceTypeId,
        reason: String,
    },
    #[error("endpoint node type {node_type_id} has no input port named '{port_name}'")]
    UnknownEndpointInputPort {
        node_type_id: String,
        port_name: String,
    },
    #[error(transparent)]
    GraphCompile(#[from] CompileError),
    #[error(transparent)]
    RuntimePlan(#[from] crate::graph::plan::PlanError),
}

//! Typed failures produced while lowering and compiling a Session declaration.

use crate::graph::compile::CompileError;
use crate::session::{OperatorInstanceId, SessionError, SourceTypeId};

#[derive(Debug, thiserror::Error)]
#[doc = "Classifies failures reported as session compile error."]
pub enum SessionCompileError {
    #[error(transparent)]
    #[doc = "Reports invalid spec."]
    InvalidSpec(#[from] SessionError),
    #[error("operator {operator_id} is not registered")]
    #[doc = "Reports unknown operator."]
    UnknownOperator {
        #[doc = "Identifies the operator associated with `UnknownOperator`."]
        operator_id: String,
    },
    #[error(
        "operator {operator_id} resolves to node type {registered_node_type_id}, not {declared_node_type_id}"
    )]
    #[doc = "Reports operator node type mismatch."]
    OperatorNodeTypeMismatch {
        #[doc = "Identifies the operator associated with `OperatorNodeTypeMismatch`."]
        operator_id: String,
        #[doc = "Identifies the registered node type associated with `OperatorNodeTypeMismatch`."]
        registered_node_type_id: String,
        #[doc = "Identifies the declared node type associated with `OperatorNodeTypeMismatch`."]
        declared_node_type_id: String,
    },
    #[error("async operator {operator_id} is not registered")]
    #[doc = "Reports unknown async operator."]
    UnknownAsyncOperator {
        #[doc = "Identifies the operator associated with `UnknownAsyncOperator`."]
        operator_id: String,
    },
    #[error("derived endpoint node type {node_type_id} is not registered")]
    #[doc = "Reports unknown endpoint node type."]
    UnknownEndpointNodeType {
        #[doc = "Identifies the node type associated with `UnknownEndpointNodeType`."]
        node_type_id: String,
    },
    #[error(
        "derived endpoint node type {node_type_id} has {input_ports_total} inputs; send(destination) requires exactly one"
    )]
    #[doc = "Reports ambiguous endpoint input."]
    AmbiguousEndpointInput {
        #[doc = "Identifies the node type associated with `AmbiguousEndpointInput`."]
        node_type_id: String,
        #[doc = "Counts the total number of input ports observed by `AmbiguousEndpointInput`."]
        input_ports_total: usize,
    },
    #[error("operator {operator_id} requires explicit {direction} port selection")]
    #[doc = "Reports ambiguous operator port."]
    AmbiguousOperatorPort {
        #[doc = "Identifies the operator associated with `AmbiguousOperatorPort`."]
        operator_id: String,
        #[doc = "Stores the direction associated with `AmbiguousOperatorPort`."]
        direction: &'static str,
    },
    #[error("operator {operator_id} has no {direction} port named '{port_name}'")]
    #[doc = "Reports unknown operator port."]
    UnknownOperatorPort {
        #[doc = "Identifies the operator associated with `UnknownOperatorPort`."]
        operator_id: String,
        #[doc = "Stores the direction associated with `UnknownOperatorPort`."]
        direction: &'static str,
        #[doc = "Stores the port name associated with `UnknownOperatorPort`."]
        port_name: String,
    },
    #[error("operator instance {operator_instance_id:?} required input port '{port_name}' is not connected")]
    #[doc = "Reports missing required operator input."]
    MissingRequiredOperatorInput {
        #[doc = "Identifies the operator instance associated with `MissingRequiredOperatorInput`."]
        operator_instance_id: OperatorInstanceId,
        #[doc = "Stores the port name associated with `MissingRequiredOperatorInput`."]
        port_name: String,
    },
    #[error(
        "operator instance {operator_instance_id:?} output '{output_port}' cannot enter the audio bridge because it is not concrete PCM"
    )]
    #[doc = "Reports invalid audio bridge output."]
    InvalidAudioBridgeOutput {
        #[doc = "Identifies the operator instance associated with `InvalidAudioBridgeOutput`."]
        operator_instance_id: OperatorInstanceId,
        #[doc = "Stores the output port associated with `InvalidAudioBridgeOutput`."]
        output_port: String,
    },
    #[error(
        "operator instance {operator_instance_id:?} output '{output_port}' must have exactly one generated-audio consumer"
    )]
    #[doc = "Reports audio bridge output not exclusive."]
    AudioBridgeOutputNotExclusive {
        #[doc = "Identifies the operator instance associated with `AudioBridgeOutputNotExclusive`."]
        operator_instance_id: OperatorInstanceId,
        #[doc = "Stores the output port associated with `AudioBridgeOutputNotExclusive`."]
        output_port: String,
    },
    #[error("operator instance {operator_instance_id:?} input port '{port_name}' is connected more than once")]
    #[doc = "Reports duplicate operator input connection."]
    DuplicateOperatorInputConnection {
        #[doc = "Identifies the operator instance associated with `DuplicateOperatorInputConnection`."]
        operator_instance_id: OperatorInstanceId,
        #[doc = "Stores the port name associated with `DuplicateOperatorInputConnection`."]
        port_name: String,
    },
    #[error("required source node type {node_type_id} is not registered")]
    #[doc = "Reports unknown source node type."]
    UnknownSourceNodeType {
        #[doc = "Identifies the node type associated with `UnknownSourceNodeType`."]
        node_type_id: String,
    },
    #[error("external source type {source_type_id} is not registered on SessionEngine")]
    #[doc = "Reports unknown external source."]
    UnknownExternalSource {
        #[doc = "Identifies the source type associated with `UnknownExternalSource`."]
        source_type_id: SourceTypeId,
    },
    #[error(
        "external source type {source_type_id} has no declared output port named '{output_port}'"
    )]
    #[doc = "Reports unknown external source output."]
    UnknownExternalSourceOutput {
        #[doc = "Identifies the source type associated with `UnknownExternalSourceOutput`."]
        source_type_id: SourceTypeId,
        #[doc = "Stores the output port associated with `UnknownExternalSourceOutput`."]
        output_port: String,
    },
    #[error("external source type {source_type_id} configuration is invalid: {reason}")]
    #[doc = "Reports invalid external source configuration."]
    InvalidExternalSourceConfiguration {
        #[doc = "Identifies the source type associated with `InvalidExternalSourceConfiguration`."]
        source_type_id: SourceTypeId,
        #[doc = "Carries the reason reported by `InvalidExternalSourceConfiguration`."]
        reason: String,
    },
    #[error("endpoint node type {node_type_id} has no input port named '{port_name}'")]
    #[doc = "Reports unknown endpoint input port."]
    UnknownEndpointInputPort {
        #[doc = "Identifies the node type associated with `UnknownEndpointInputPort`."]
        node_type_id: String,
        #[doc = "Stores the port name associated with `UnknownEndpointInputPort`."]
        port_name: String,
    },
    #[error(transparent)]
    #[doc = "Reports graph compile."]
    GraphCompile(#[from] CompileError),
    #[error(transparent)]
    #[doc = "Reports runtime plan."]
    RuntimePlan(#[from] crate::graph::plan::PlanError),
}

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
        #[doc = "Identifies the operator identifier recorded by `UnknownOperator`."]
        operator_id: String,
    },
    #[error(
        "operator {operator_id} resolves to node type {registered_node_type_id}, not {declared_node_type_id}"
    )]
    #[doc = "Reports operator node type mismatch."]
    OperatorNodeTypeMismatch {
        #[doc = "Identifies the operator identifier recorded by `OperatorNodeTypeMismatch`."]
        operator_id: String,
        #[doc = "Identifies the registered node type identifier recorded by `OperatorNodeTypeMismatch`."]
        registered_node_type_id: String,
        #[doc = "Identifies the declared node type identifier recorded by `OperatorNodeTypeMismatch`."]
        declared_node_type_id: String,
    },
    #[error("async operator {operator_id} is not registered")]
    #[doc = "Reports unknown async operator."]
    UnknownAsyncOperator {
        #[doc = "Identifies the operator identifier recorded by `UnknownAsyncOperator`."]
        operator_id: String,
    },
    #[error("derived endpoint node type {node_type_id} is not registered")]
    #[doc = "Reports unknown endpoint node type."]
    UnknownEndpointNodeType {
        #[doc = "Identifies the node type identifier recorded by `UnknownEndpointNodeType`."]
        node_type_id: String,
    },
    #[error(
        "derived endpoint node type {node_type_id} has {input_ports_total} inputs; send(destination) requires exactly one"
    )]
    #[doc = "Reports ambiguous endpoint input."]
    AmbiguousEndpointInput {
        #[doc = "Identifies the node type identifier recorded by `AmbiguousEndpointInput`."]
        node_type_id: String,
        #[doc = "Counts the total number of input ports observed by `AmbiguousEndpointInput`."]
        input_ports_total: usize,
    },
    #[error("operator {operator_id} requires explicit {direction} port selection")]
    #[doc = "Reports ambiguous operator port."]
    AmbiguousOperatorPort {
        #[doc = "Identifies the operator identifier recorded by `AmbiguousOperatorPort`."]
        operator_id: String,
        #[doc = "Stores the direction used by `AmbiguousOperatorPort`."]
        direction: &'static str,
    },
    #[error("operator {operator_id} has no {direction} port named '{port_name}'")]
    #[doc = "Reports unknown operator port."]
    UnknownOperatorPort {
        #[doc = "Identifies the operator identifier recorded by `UnknownOperatorPort`."]
        operator_id: String,
        #[doc = "Stores the direction used by `UnknownOperatorPort`."]
        direction: &'static str,
        #[doc = "Stores the port name used by `UnknownOperatorPort`."]
        port_name: String,
    },
    #[error("operator instance {operator_instance_id:?} required input port '{port_name}' is not connected")]
    #[doc = "Reports missing required operator input."]
    MissingRequiredOperatorInput {
        #[doc = "Identifies the operator instance identifier recorded by `MissingRequiredOperatorInput`."]
        operator_instance_id: OperatorInstanceId,
        #[doc = "Stores the port name used by `MissingRequiredOperatorInput`."]
        port_name: String,
    },
    #[error(
        "operator instance {operator_instance_id:?} output '{output_port}' cannot enter the audio bridge because it is not concrete PCM"
    )]
    #[doc = "Reports invalid audio bridge output."]
    InvalidAudioBridgeOutput {
        #[doc = "Identifies the operator instance identifier recorded by `InvalidAudioBridgeOutput`."]
        operator_instance_id: OperatorInstanceId,
        #[doc = "Stores the output port used by `InvalidAudioBridgeOutput`."]
        output_port: String,
    },
    #[error(
        "operator instance {operator_instance_id:?} output '{output_port}' must have exactly one generated-audio consumer"
    )]
    #[doc = "Reports audio bridge output not exclusive."]
    AudioBridgeOutputNotExclusive {
        #[doc = "Identifies the operator instance identifier recorded by `AudioBridgeOutputNotExclusive`."]
        operator_instance_id: OperatorInstanceId,
        #[doc = "Stores the output port used by `AudioBridgeOutputNotExclusive`."]
        output_port: String,
    },
    #[error("operator instance {operator_instance_id:?} input port '{port_name}' is connected more than once")]
    #[doc = "Reports duplicate operator input connection."]
    DuplicateOperatorInputConnection {
        #[doc = "Identifies the operator instance identifier recorded by `DuplicateOperatorInputConnection`."]
        operator_instance_id: OperatorInstanceId,
        #[doc = "Stores the port name used by `DuplicateOperatorInputConnection`."]
        port_name: String,
    },
    #[error("required source node type {node_type_id} is not registered")]
    #[doc = "Reports unknown source node type."]
    UnknownSourceNodeType {
        #[doc = "Identifies the node type identifier recorded by `UnknownSourceNodeType`."]
        node_type_id: String,
    },
    #[error("external source type {source_type_id} is not registered on SessionEngine")]
    #[doc = "Reports unknown external source."]
    UnknownExternalSource {
        #[doc = "Identifies the source type identifier recorded by `UnknownExternalSource`."]
        source_type_id: SourceTypeId,
    },
    #[error(
        "external source type {source_type_id} has no declared output port named '{output_port}'"
    )]
    #[doc = "Reports unknown external source output."]
    UnknownExternalSourceOutput {
        #[doc = "Identifies the source type identifier recorded by `UnknownExternalSourceOutput`."]
        source_type_id: SourceTypeId,
        #[doc = "Stores the output port used by `UnknownExternalSourceOutput`."]
        output_port: String,
    },
    #[error("external source type {source_type_id} configuration is invalid: {reason}")]
    #[doc = "Reports invalid external source configuration."]
    InvalidExternalSourceConfiguration {
        #[doc = "Identifies the source type identifier recorded by `InvalidExternalSourceConfiguration`."]
        source_type_id: SourceTypeId,
        #[doc = "Carries the reason reported by `InvalidExternalSourceConfiguration`."]
        reason: String,
    },
    #[error("endpoint node type {node_type_id} has no input port named '{port_name}'")]
    #[doc = "Reports unknown endpoint input port."]
    UnknownEndpointInputPort {
        #[doc = "Identifies the node type identifier recorded by `UnknownEndpointInputPort`."]
        node_type_id: String,
        #[doc = "Stores the port name used by `UnknownEndpointInputPort`."]
        port_name: String,
    },
    #[error(transparent)]
    #[doc = "Reports graph compile."]
    GraphCompile(#[from] CompileError),
    #[error(transparent)]
    #[doc = "Reports runtime plan."]
    RuntimePlan(#[from] crate::graph::plan::PlanError),
}

/// Stable, language-neutral location facts for one Session compilation error.
///
/// The diagnostic deliberately excludes implementation error types. Bindings
/// can expose these finite fields without parsing Display or Debug output.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionCompileDiagnostic {
    code: String,
    node_index: Option<u32>,
    edge_index: Option<u32>,
    operator_id: Option<String>,
    operator_instance_id: Option<u64>,
    node_type_id: Option<String>,
    source_type_id: Option<String>,
    port_name: Option<String>,
    direction: Option<String>,
    expected: Option<String>,
    actual: Option<String>,
}

impl SessionCompileDiagnostic {
    #[doc = "Returns the stable error or status code represented by `SessionCompileDiagnostic`."]
    pub fn code(&self) -> &str {
        &self.code
    }

    #[doc = "Returns the node index held by `SessionCompileDiagnostic`."]
    pub const fn node_index(&self) -> Option<u32> {
        self.node_index
    }

    #[doc = "Returns the edge index held by `SessionCompileDiagnostic`."]
    pub const fn edge_index(&self) -> Option<u32> {
        self.edge_index
    }

    #[doc = "Returns the operator identifier held by `SessionCompileDiagnostic`."]
    pub fn operator_id(&self) -> Option<&str> {
        self.operator_id.as_deref()
    }

    #[doc = "Returns the operator instance identifier held by `SessionCompileDiagnostic`."]
    pub const fn operator_instance_id(&self) -> Option<u64> {
        self.operator_instance_id
    }

    #[doc = "Returns the node type identifier held by `SessionCompileDiagnostic`."]
    pub fn node_type_id(&self) -> Option<&str> {
        self.node_type_id.as_deref()
    }

    #[doc = "Returns the source type identifier held by `SessionCompileDiagnostic`."]
    pub fn source_type_id(&self) -> Option<&str> {
        self.source_type_id.as_deref()
    }

    #[doc = "Returns the port name held by `SessionCompileDiagnostic`."]
    pub fn port_name(&self) -> Option<&str> {
        self.port_name.as_deref()
    }

    #[doc = "Returns the direction held by `SessionCompileDiagnostic`."]
    pub fn direction(&self) -> Option<&str> {
        self.direction.as_deref()
    }

    /// Returns the expected value when the diagnostic compares two values.
    pub fn expected(&self) -> Option<&str> {
        self.expected.as_deref()
    }

    /// Returns the observed value when the diagnostic compares two values.
    pub fn actual(&self) -> Option<&str> {
        self.actual.as_deref()
    }

    fn new(code: &str) -> Self {
        Self {
            code: code.to_owned(),
            ..Self::default()
        }
    }
}

impl SessionCompileError {
    /// Converts this compiler failure into stable language-neutral location
    /// and comparison fields.
    pub fn diagnostic(&self) -> SessionCompileDiagnostic {
        use crate::graph::compile::CompileError;
        use crate::graph::plan::PlanError;

        match self {
            Self::InvalidSpec(error) => {
                SessionCompileDiagnostic::new(crate::session_declaration_error_code(error).as_str())
            }
            Self::UnknownOperator { operator_id } => SessionCompileDiagnostic {
                operator_id: Some(operator_id.clone()),
                ..SessionCompileDiagnostic::new("compile.unknown_operator")
            },
            Self::OperatorNodeTypeMismatch {
                operator_id,
                registered_node_type_id,
                declared_node_type_id,
            } => SessionCompileDiagnostic {
                operator_id: Some(operator_id.clone()),
                expected: Some(declared_node_type_id.clone()),
                actual: Some(registered_node_type_id.clone()),
                ..SessionCompileDiagnostic::new("compile.operator_node_type_mismatch")
            },
            Self::UnknownAsyncOperator { operator_id } => SessionCompileDiagnostic {
                operator_id: Some(operator_id.clone()),
                ..SessionCompileDiagnostic::new("compile.unknown_async_operator")
            },
            Self::UnknownEndpointNodeType { node_type_id } => SessionCompileDiagnostic {
                node_type_id: Some(node_type_id.clone()),
                ..SessionCompileDiagnostic::new("compile.unknown_endpoint_node_type")
            },
            Self::AmbiguousEndpointInput {
                node_type_id,
                input_ports_total,
            } => SessionCompileDiagnostic {
                node_type_id: Some(node_type_id.clone()),
                actual: Some(input_ports_total.to_string()),
                expected: Some("1".to_owned()),
                ..SessionCompileDiagnostic::new("compile.ambiguous_endpoint_input")
            },
            Self::AmbiguousOperatorPort {
                operator_id,
                direction,
            } => SessionCompileDiagnostic {
                operator_id: Some(operator_id.clone()),
                direction: Some((*direction).to_owned()),
                ..SessionCompileDiagnostic::new("compile.ambiguous_operator_port")
            },
            Self::UnknownOperatorPort {
                operator_id,
                direction,
                port_name,
            } => SessionCompileDiagnostic {
                operator_id: Some(operator_id.clone()),
                direction: Some((*direction).to_owned()),
                port_name: Some(port_name.clone()),
                ..SessionCompileDiagnostic::new("compile.unknown_operator_port")
            },
            Self::MissingRequiredOperatorInput {
                operator_instance_id,
                port_name,
            } => SessionCompileDiagnostic {
                operator_instance_id: Some(operator_instance_id.value()),
                port_name: Some(port_name.clone()),
                direction: Some("input".to_owned()),
                ..SessionCompileDiagnostic::new("compile.missing_required_operator_input")
            },
            Self::InvalidAudioBridgeOutput {
                operator_instance_id,
                output_port,
            } => SessionCompileDiagnostic {
                operator_instance_id: Some(operator_instance_id.value()),
                port_name: Some(output_port.clone()),
                direction: Some("output".to_owned()),
                ..SessionCompileDiagnostic::new("compile.invalid_audio_bridge_output")
            },
            Self::AudioBridgeOutputNotExclusive {
                operator_instance_id,
                output_port,
            } => SessionCompileDiagnostic {
                operator_instance_id: Some(operator_instance_id.value()),
                port_name: Some(output_port.clone()),
                direction: Some("output".to_owned()),
                ..SessionCompileDiagnostic::new("compile.audio_bridge_output_not_exclusive")
            },
            Self::DuplicateOperatorInputConnection {
                operator_instance_id,
                port_name,
            } => SessionCompileDiagnostic {
                operator_instance_id: Some(operator_instance_id.value()),
                port_name: Some(port_name.clone()),
                direction: Some("input".to_owned()),
                ..SessionCompileDiagnostic::new("compile.duplicate_operator_input_connection")
            },
            Self::UnknownSourceNodeType { node_type_id } => SessionCompileDiagnostic {
                node_type_id: Some(node_type_id.clone()),
                ..SessionCompileDiagnostic::new("compile.unknown_source_node_type")
            },
            Self::UnknownExternalSource { source_type_id } => SessionCompileDiagnostic {
                source_type_id: Some(source_type_id.as_str().to_owned()),
                ..SessionCompileDiagnostic::new("compile.unknown_external_source")
            },
            Self::UnknownExternalSourceOutput {
                source_type_id,
                output_port,
            } => SessionCompileDiagnostic {
                source_type_id: Some(source_type_id.as_str().to_owned()),
                port_name: Some(output_port.clone()),
                direction: Some("output".to_owned()),
                ..SessionCompileDiagnostic::new("compile.unknown_external_source_output")
            },
            Self::InvalidExternalSourceConfiguration { source_type_id, .. } => {
                SessionCompileDiagnostic {
                    source_type_id: Some(source_type_id.as_str().to_owned()),
                    ..SessionCompileDiagnostic::new("compile.invalid_external_source_configuration")
                }
            }
            Self::UnknownEndpointInputPort {
                node_type_id,
                port_name,
            } => SessionCompileDiagnostic {
                node_type_id: Some(node_type_id.clone()),
                port_name: Some(port_name.clone()),
                direction: Some("input".to_owned()),
                ..SessionCompileDiagnostic::new("compile.unknown_endpoint_input_port")
            },
            Self::GraphCompile(error) => match error {
                CompileError::UnknownNodeType(node_type_id) => SessionCompileDiagnostic {
                    node_type_id: Some(node_type_id.clone()),
                    ..SessionCompileDiagnostic::new("compile.graph.unknown_node_type")
                },
                CompileError::InvalidConfig { type_id, .. } => SessionCompileDiagnostic {
                    node_type_id: Some(type_id.clone()),
                    ..SessionCompileDiagnostic::new("compile.graph.invalid_config")
                },
                CompileError::UnknownNode(node_index) => SessionCompileDiagnostic {
                    node_index: Some(*node_index),
                    ..SessionCompileDiagnostic::new("compile.graph.unknown_node")
                },
                CompileError::UnknownPort { node, port } => SessionCompileDiagnostic {
                    node_index: Some(*node),
                    port_name: Some(port.clone()),
                    ..SessionCompileDiagnostic::new("compile.graph.unknown_port")
                },
                CompileError::WrongPortDirection { node, port } => SessionCompileDiagnostic {
                    node_index: Some(*node),
                    port_name: Some(port.clone()),
                    ..SessionCompileDiagnostic::new("compile.graph.wrong_port_direction")
                },
                CompileError::ClockDomainMismatch {
                    node,
                    port,
                    expected,
                    found,
                } => SessionCompileDiagnostic {
                    node_index: Some(*node),
                    port_name: Some(port.clone()),
                    expected: Some(format!("{expected:?}").to_lowercase()),
                    actual: Some(format!("{found:?}").to_lowercase()),
                    ..SessionCompileDiagnostic::new("compile.graph.clock_domain_mismatch")
                },
                CompileError::MediaMismatch { edge, from, to } => SessionCompileDiagnostic {
                    edge_index: Some(*edge),
                    expected: Some(to.clone()),
                    actual: Some(from.clone()),
                    ..SessionCompileDiagnostic::new("compile.graph.media_mismatch")
                },
                CompileError::SignalMismatch { edge, from, to } => SessionCompileDiagnostic {
                    edge_index: Some(*edge),
                    expected: Some(to.clone()),
                    actual: Some(from.clone()),
                    ..SessionCompileDiagnostic::new("compile.graph.signal_mismatch")
                },
                CompileError::InvalidSafetyContract { node, type_id, .. } => {
                    SessionCompileDiagnostic {
                        node_index: Some(*node),
                        node_type_id: Some(type_id.clone()),
                        ..SessionCompileDiagnostic::new("compile.graph.invalid_safety_contract")
                    }
                }
                CompileError::InvalidRealtimeEdge { edge, .. } => SessionCompileDiagnostic {
                    edge_index: Some(*edge),
                    ..SessionCompileDiagnostic::new("compile.graph.invalid_realtime_edge")
                },
                CompileError::CycleDetected => {
                    SessionCompileDiagnostic::new("compile.graph.cycle_detected")
                }
                CompileError::AdapterUnavailable { edge, type_id } => SessionCompileDiagnostic {
                    edge_index: Some(*edge),
                    node_type_id: Some(type_id.clone()),
                    ..SessionCompileDiagnostic::new("compile.graph.adapter_unavailable")
                },
            },
            Self::RuntimePlan(error) => match error {
                PlanError::FanInOnSinglePort { node, port } => SessionCompileDiagnostic {
                    node_index: Some(*node),
                    port_name: Some(port.clone()),
                    ..SessionCompileDiagnostic::new("compile.plan.fan_in_on_single_port")
                },
                PlanError::MoveExclusiveFanOut { node, port } => SessionCompileDiagnostic {
                    node_index: Some(*node),
                    port_name: Some(port.clone()),
                    ..SessionCompileDiagnostic::new("compile.plan.move_exclusive_fan_out")
                },
                PlanError::MissingEdgeContract { edge } => SessionCompileDiagnostic {
                    edge_index: Some(edge.index()),
                    ..SessionCompileDiagnostic::new("compile.plan.missing_edge_contract")
                },
                PlanError::MissingOutputSignal { edge } => SessionCompileDiagnostic {
                    edge_index: Some(edge.index()),
                    ..SessionCompileDiagnostic::new("compile.plan.missing_output_signal")
                },
            },
        }
    }
}

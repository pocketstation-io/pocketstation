use std::collections::HashMap;
use std::sync::Arc;

use crate::graph::{MediaCaps, NodeConfig, NodeTypeId, Pipeline};
use crate::session::compile::{
    select_operator_port, CompiledNodeBinding, CompiledSessionBindings, LoweredOperator,
    SessionCompileError, SessionGraphLowerer, SessionSourceLoweringContext,
};
use crate::session::{EndpointSpec, OperatorInstanceId, SessionSpec, StemId};

pub(crate) const GENERATED_AUDIO_INGRESS_NODE_TYPE_ID: &str = "source.generated-audio-ingress";
pub(crate) const GENERATED_AUDIO_BRIDGE_NODE_TYPE_ID: &str = "bridge.generated-audio";
const AUDIO_INPUT_PORT: &str = "audio";

/// Registers the built-in bounded asynchronous-audio reentry package through
/// the same compiler seam used by any future structural graph component.
pub(crate) fn audio_reentry_lowerer() -> Arc<dyn SessionGraphLowerer> {
    Arc::new(AudioReentryLowerer)
}

struct AudioReentryLowerer;

impl SessionGraphLowerer for AudioReentryLowerer {
    fn lower_source_nodes(
        &self,
        spec: &SessionSpec,
        context: &mut SessionSourceLoweringContext<'_>,
    ) -> Result<(), SessionCompileError> {
        for ingress in spec.generated_audio_ingresses() {
            let node = context.pipeline.add_node(
                NodeTypeId::from(GENERATED_AUDIO_INGRESS_NODE_TYPE_ID),
                NodeConfig::new(),
            );
            context.bindings.insert_node(
                node.id(),
                CompiledNodeBinding::GeneratedAudioIngress {
                    stem_id: ingress.stem_id(),
                },
            );
            context.source_nodes.insert(ingress.stem_id(), node);
        }
        Ok(())
    }

    fn lower_operator_edges(
        &self,
        spec: &SessionSpec,
        pipeline: &mut Pipeline,
        operator_nodes: &HashMap<OperatorInstanceId, LoweredOperator>,
        bindings: &mut CompiledSessionBindings,
    ) -> Result<(), SessionCompileError> {
        for ingress in spec.generated_audio_ingresses() {
            let operator = operator_nodes.get(&ingress.operator_instance_id()).ok_or(
                crate::session::SessionError::UnknownOperatorInstance {
                    operator_instance_id: ingress.operator_instance_id(),
                },
            )?;
            let output = select_operator_port(
                &operator.manifest,
                crate::graph::PortDirection::Output,
                ingress.output_port(),
            )?;
            let concrete_pcm = output.signal.class.is_audio()
                && matches!(
                    output.media,
                    MediaCaps::Audio(audio)
                        if audio.sample_rate_hz.is_some()
                            && audio.frame_samples.is_some()
                            && !matches!(audio.channel_layout, crate::graph::ChannelLayout::Any)
                );
            if !concrete_pcm {
                return Err(SessionCompileError::InvalidAudioBridgeOutput {
                    operator_instance_id: ingress.operator_instance_id(),
                    output_port: output.name.clone(),
                });
            }
            let ordinary_consumers = spec
                .connections()
                .iter()
                .filter(|connection| {
                    let crate::session::StreamOrigin::OperatorOutput {
                        operator_instance_id,
                        output_port,
                    } = connection.origin()
                    else {
                        return false;
                    };
                    if *operator_instance_id != ingress.operator_instance_id() {
                        return false;
                    }
                    select_operator_port(
                        &operator.manifest,
                        crate::graph::PortDirection::Output,
                        output_port.as_deref(),
                    )
                    .is_ok_and(|candidate| candidate.name == output.name)
                })
                .count();
            let reentry_consumers = spec
                .generated_audio_ingresses()
                .iter()
                .filter(|candidate| {
                    if candidate.operator_instance_id() != ingress.operator_instance_id() {
                        return false;
                    }
                    select_operator_port(
                        &operator.manifest,
                        crate::graph::PortDirection::Output,
                        candidate.output_port(),
                    )
                    .is_ok_and(|candidate_port| candidate_port.name == output.name)
                })
                .count();
            if ordinary_consumers != 0 || reentry_consumers != 1 {
                return Err(SessionCompileError::AudioBridgeOutputNotExclusive {
                    operator_instance_id: ingress.operator_instance_id(),
                    output_port: output.name.clone(),
                });
            }
            let bridge = pipeline.add_node(
                NodeTypeId::from(GENERATED_AUDIO_BRIDGE_NODE_TYPE_ID),
                NodeConfig::new(),
            );
            bindings.insert_node(
                bridge.id(),
                CompiledNodeBinding::GeneratedAudioBridge {
                    stem_id: ingress.stem_id(),
                    operator_instance_id: ingress.operator_instance_id(),
                },
            );
            pipeline.connect_with(
                operator.node.out(&output.name),
                bridge.in_(AUDIO_INPUT_PORT),
                operator.manifest.output_edge,
            );
        }
        Ok(())
    }

    fn endpoint_config(
        &self,
        spec: &SessionSpec,
        stem_id: StemId,
        endpoint: &EndpointSpec,
        _route_id: crate::session::RouteId,
    ) -> Result<Option<NodeConfig>, SessionCompileError> {
        let Some(_ingress) = spec
            .generated_audio_ingresses()
            .iter()
            .find(|ingress| ingress.stem_id() == stem_id)
        else {
            return Ok(None);
        };
        let mut config = NodeConfig::new();
        for (key, value) in endpoint.configuration().iter() {
            config = config.with(key, value);
        }
        Ok(Some(config))
    }
}

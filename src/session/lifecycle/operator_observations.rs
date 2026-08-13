//! Internal projection of operator input-edge observations into Session metrics.

use super::observations::SessionOperatorInputMetrics;
use crate::runtime::{EdgeObservations, PlanEdgeObservationHandle, TypedEdgeObservationHandle};

pub(super) enum OperatorInputObservation {
    Plan(PlanEdgeObservationHandle),
    Typed(TypedEdgeObservationHandle),
}

pub(super) struct OperatorInputObservationBinding {
    pub(super) port_name: String,
    pub(super) observation: OperatorInputObservation,
}

impl OperatorInputObservationBinding {
    pub(super) fn aggregate(inputs: &[Self]) -> EdgeObservations {
        if let [Self {
            observation: OperatorInputObservation::Plan(handle),
            ..
        }] = inputs
        {
            return handle.observations();
        }

        let mut aggregate = EdgeObservations::default();
        for input in inputs {
            let edge = input.edge_observations();
            aggregate.queue_capacity_frames = aggregate
                .queue_capacity_frames
                .saturating_add(edge.queue_capacity_frames);
            aggregate.queue_depth_frames = aggregate
                .queue_depth_frames
                .saturating_add(edge.queue_depth_frames);
            aggregate.queue_peak_frames = aggregate
                .queue_peak_frames
                .saturating_add(edge.queue_peak_frames);
            aggregate.frames_enqueued_total = aggregate
                .frames_enqueued_total
                .saturating_add(edge.frames_enqueued_total);
            aggregate.frames_delivered_total = aggregate
                .frames_delivered_total
                .saturating_add(edge.frames_delivered_total);
            aggregate.frames_dropped_total = aggregate
                .frames_dropped_total
                .saturating_add(edge.frames_dropped_total);
            aggregate.overruns_total = aggregate.overruns_total.saturating_add(edge.overruns_total);
            aggregate.queue_full_drops_total = aggregate
                .queue_full_drops_total
                .saturating_add(edge.queue_full_drops_total);
            aggregate.discontinuities_total = aggregate
                .discontinuities_total
                .saturating_add(edge.discontinuities_total);
            aggregate.worker_failures_total = aggregate
                .worker_failures_total
                .saturating_add(edge.worker_failures_total);
            aggregate.shutdown_discarded_total = aggregate
                .shutdown_discarded_total
                .saturating_add(edge.shutdown_discarded_total);
        }
        aggregate
    }

    pub(super) fn per_port(inputs: &[Self]) -> Box<[SessionOperatorInputMetrics]> {
        inputs
            .iter()
            .map(|input| SessionOperatorInputMetrics {
                port_name: input.port_name.clone(),
                edge: input.edge_observations(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn edge_observations(&self) -> EdgeObservations {
        match &self.observation {
            OperatorInputObservation::Plan(handle) => handle.observations(),
            OperatorInputObservation::Typed(handle) => {
                let typed = handle.snapshot();
                EdgeObservations {
                    queue_capacity_frames: typed.capacity_signals,
                    queue_depth_frames: typed.depth_signals,
                    queue_peak_frames: typed.peak_depth_signals,
                    frames_enqueued_total: typed.enqueued_total,
                    frames_delivered_total: typed.received_total,
                    frames_dropped_total: typed.dropped_total,
                    queue_full_drops_total: typed.dropped_total,
                    ..EdgeObservations::default()
                }
            }
        }
    }
}

use crate::frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, SessionId, SourceId, StemId, StreamId,
};

use super::*;

const ENDPOINT_ID: EndpointId = EndpointId(31);
const CONNECTOR_ID: ConnectorId = ConnectorId(41);
const ROUTE_ID: RouteId = RouteId(51);

fn lineaged_frame(
    pool: &Arc<AudioBufferPool>,
    sequence_number: u64,
    sample_linear: f32,
) -> LineagedAudioFrame {
    let mut buffer = pool.acquire().expect("test pool slot");
    buffer
        .try_copy_from_slice(&[sample_linear, sample_linear, sample_linear, sample_linear])
        .expect("test samples fit the fixed-capacity buffer");
    let frame = AudioFrame::new(
        StreamId(1),
        SourceId(2),
        sequence_number,
        sequence_number.saturating_mul(1_000_000),
        1,
        buffer,
    );
    let lineage = FrameLineage {
        session_id: SessionId(3),
        source_id: SourceId(2),
        stem_id: StemId(4),
        clock_id: ClockDomainId(5),
        sequence_num: sequence_number,
        timestamp_start_ns: sequence_number.saturating_mul(1_000_000),
        duration_ns: 83_333,
        source_generation: 1,
        discontinuity_epoch: 0,
        permission_epoch: 1,
    };
    LineagedAudioFrame::new(frame, lineage).expect("valid test lineage")
}

fn delivered(frame: LineagedAudioFrame) -> DeliveredAudioFrame {
    DeliveredAudioFrame {
        endpoint_id: ENDPOINT_ID,
        connector_id: CONNECTOR_ID,
        route_id: ROUTE_ID,
        route_enqueued_at_ns: 10,
        route_received_at_ns: 20,
        endpoint_enqueued_at_ns: 30,
        polled_at_ns: 0,
        frame,
    }
}

#[test]
fn given_shared_branch_when_published_then_it_is_rejected_and_counted() {
    let config = PolledAudioEndpointConfig::default();
    let shared = ReceiptShared::new(config);
    let observations = WorkerObservations::default();

    let lineaged_shared_pool = AudioBufferPool::new(1, 4);
    let lineaged_shared = lineaged_frame(&lineaged_shared_pool, 3, 0.3)
        .freeze()
        .expect("shared lineaged frame");
    assert!(prepare_delivered_frame(
        crate::endpoint::EndpointAudioFrame::from_route_delivery(
            PlanEdgeFrame::Shared(lineaged_shared),
            10,
            20,
        ),
        ENDPOINT_ID,
        CONNECTOR_ID,
        ROUTE_ID,
        &shared,
        &observations,
    )
    .is_none());

    assert_eq!(
        observations
            .invalid_ownership_drops_total
            .load(Ordering::Relaxed),
        1
    );
    assert_eq!(
        shared.invalid_ownership_drops_total.load(Ordering::Relaxed),
        1
    );
}

#[test]
fn given_held_batch_when_polled_then_samples_stay_stable_and_lease_exhaustion_is_counted() {
    let config = PolledAudioEndpointConfig {
        queue_capacity_frames: 2,
        max_batch_frames: 1,
        max_outstanding_leases: 1,
    };
    let shared = Arc::new(ReceiptShared::new(config));
    let receipt = PolledAudioReceipt {
        shared: Arc::clone(&shared),
    };
    let (mut producer, consumer) = RingBuffer::new(config.queue_capacity_frames);
    let consumer_slot = shared
        .register_consumer(consumer, config.queue_capacity_frames)
        .expect("consumer registration");
    let pool = AudioBufferPool::new(2, 4);
    let worker = WorkerObservations::default();
    publish_delivered_frame(
        &mut producer,
        delivered(lineaged_frame(&pool, 1, 0.25)),
        &shared,
        &worker,
    );
    publish_delivered_frame(
        &mut producer,
        delivered(lineaged_frame(&pool, 2, 0.75)),
        &shared,
        &worker,
    );

    let lease = receipt.try_poll().expect("first lease");
    let frame = lease.frame(0).expect("leased frame");
    let samples_pointer = frame.samples().as_ptr();
    assert_eq!(frame.route_enqueued_at_ns(), 10);
    assert_eq!(frame.route_received_at_ns(), 20);
    assert_eq!(frame.endpoint_enqueued_at_ns(), 30);
    assert!(frame.polled_at_ns() >= frame.endpoint_enqueued_at_ns());
    assert_eq!(frame.samples(), &[0.25, 0.25, 0.25, 0.25]);
    assert_eq!(frame.samples().as_ptr(), samples_pointer);
    assert!(matches!(
        receipt.try_poll(),
        Err(PolledAudioPollError::LeaseCapacityExhausted)
    ));
    assert_eq!(receipt.observations().outstanding_leases, 1);
    assert_eq!(receipt.observations().lease_exhausted_total, 1);

    drop(lease);
    let second = receipt.try_poll().expect("recycled lease");
    assert_eq!(
        second.frame(0).expect("second frame").samples(),
        &[0.75, 0.75, 0.75, 0.75]
    );
    drop(second);
    drop(producer);
    shared
        .remove_consumer(consumer_slot, config.queue_capacity_frames)
        .expect("consumer removal");
    assert_eq!(receipt.observations().outstanding_leases, 0);
}

#[test]
fn given_empty_receipt_when_wait_deadline_expires_then_no_batch_is_fabricated() {
    let config = PolledAudioEndpointConfig::default();
    let (factory, receipt) = PolledAudioEndpointFactory::new(config).expect("valid config");

    let started = Instant::now();
    assert!(receipt
        .wait_poll(Duration::from_millis(5))
        .expect("bounded wait")
        .is_none());
    assert!(started.elapsed() >= Duration::from_millis(4));
    drop(factory);
}

#[test]
fn given_waiting_receipt_when_frame_arrives_then_existing_batch_is_returned() {
    let config = PolledAudioEndpointConfig {
        queue_capacity_frames: 2,
        max_batch_frames: 1,
        max_outstanding_leases: 1,
    };
    let shared = Arc::new(ReceiptShared::new(config));
    let receipt = PolledAudioReceipt {
        shared: Arc::clone(&shared),
    };
    let (mut producer, consumer) = RingBuffer::new(config.queue_capacity_frames);
    let consumer_slot = shared
        .register_consumer(consumer, config.queue_capacity_frames)
        .expect("consumer registration");
    let producer_shared = Arc::clone(&shared);
    let producer_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5));
        let pool = AudioBufferPool::new(1, 4);
        let worker = WorkerObservations::default();
        publish_delivered_frame(
            &mut producer,
            delivered(lineaged_frame(&pool, 7, 0.5)),
            &producer_shared,
            &worker,
        );
    });

    let batch = receipt
        .wait_poll(Duration::from_secs(1))
        .expect("bounded wait")
        .expect("published batch");
    assert_eq!(batch.frame(0).expect("frame").lineage().sequence_num, 7);
    drop(batch);
    producer_thread.join().expect("producer thread");
    shared
        .remove_consumer(consumer_slot, config.queue_capacity_frames)
        .expect("consumer removal");
}

#[test]
fn given_concurrent_publish_and_poll_when_observed_then_depth_stays_bounded_and_returns_to_zero() {
    let config = PolledAudioEndpointConfig {
        queue_capacity_frames: 8,
        max_batch_frames: 4,
        max_outstanding_leases: 2,
    };
    let shared = Arc::new(ReceiptShared::new(config));
    let receipt = PolledAudioReceipt {
        shared: Arc::clone(&shared),
    };
    let (mut producer, consumer) = RingBuffer::new(config.queue_capacity_frames);
    let consumer_slot = shared
        .register_consumer(consumer, config.queue_capacity_frames)
        .expect("consumer registration");
    let producer_shared = Arc::clone(&shared);
    let producer_done = Arc::new(AtomicBool::new(false));
    let thread_done = Arc::clone(&producer_done);
    let worker = Arc::new(WorkerObservations::default());
    let thread_worker = Arc::clone(&worker);
    let producer_thread = thread::spawn(move || {
        let pool = AudioBufferPool::new(16, 4);
        for sequence_number in 0..2_000 {
            let frame = loop {
                if let Some(buffer) = pool.acquire() {
                    let frame = AudioFrame::new(
                        StreamId(1),
                        SourceId(2),
                        sequence_number,
                        sequence_number.saturating_mul(1_000_000),
                        1,
                        buffer,
                    );
                    let lineage = FrameLineage {
                        session_id: SessionId(3),
                        source_id: SourceId(2),
                        stem_id: StemId(4),
                        clock_id: ClockDomainId(5),
                        sequence_num: sequence_number,
                        timestamp_start_ns: sequence_number.saturating_mul(1_000_000),
                        duration_ns: 83_333,
                        source_generation: 1,
                        discontinuity_epoch: 0,
                        permission_epoch: 1,
                    };
                    break LineagedAudioFrame::new(frame, lineage)
                        .expect("valid concurrent lineage");
                }
                thread::yield_now();
            };
            publish_delivered_frame(
                &mut producer,
                delivered(frame),
                &producer_shared,
                &thread_worker,
            );
            if sequence_number % 7 == 0 {
                thread::yield_now();
            }
        }
        thread_done.store(true, Ordering::Release);
    });

    let mut maximum_depth = 0;
    loop {
        match receipt.try_poll() {
            Ok(lease) => drop(lease),
            Err(PolledAudioPollError::Empty) => thread::yield_now(),
            Err(error) => panic!("unexpected concurrent poll result: {error}"),
        }
        let depth = receipt.observations().queue_depth_frames;
        maximum_depth = maximum_depth.max(depth);
        assert!(depth <= config.queue_capacity_frames as u64);
        if producer_done.load(Ordering::Acquire) && depth == 0 {
            break;
        }
    }
    producer_thread.join().expect("producer thread");
    assert!(maximum_depth <= config.queue_capacity_frames as u64);
    assert_eq!(receipt.observations().queue_depth_frames, 0);
    assert_eq!(
        receipt.observations().queue_depth_invariant_failures_total,
        0
    );
    shared
        .remove_consumer(consumer_slot, config.queue_capacity_frames)
        .expect("consumer removal");
}

#[test]
fn given_untrusted_oversized_capacities_when_constructed_then_all_fail_before_allocation() {
    for (config, expected) in [
        (
            PolledAudioEndpointConfig {
                queue_capacity_frames: MAX_QUEUE_CAPACITY_FRAMES + 1,
                ..PolledAudioEndpointConfig::default()
            },
            PolledAudioEndpointConfigError::QueueCapacityTooLarge,
        ),
        (
            PolledAudioEndpointConfig {
                max_batch_frames: MAX_BATCH_CAPACITY_FRAMES + 1,
                ..PolledAudioEndpointConfig::default()
            },
            PolledAudioEndpointConfigError::BatchCapacityTooLarge,
        ),
        (
            PolledAudioEndpointConfig {
                max_outstanding_leases: MAX_OUTSTANDING_LEASES + 1,
                ..PolledAudioEndpointConfig::default()
            },
            PolledAudioEndpointConfigError::LeaseCapacityTooLarge,
        ),
    ] {
        assert_eq!(
            PolledAudioEndpointFactory::new(config)
                .err()
                .expect("oversized config must fail"),
            expected
        );
    }
}

#[test]
fn given_impossible_dequeue_when_observed_then_depth_saturates_and_failure_is_explicit() {
    let shared = ReceiptShared::new(PolledAudioEndpointConfig::default());

    shared.observe_dequeued(1);

    assert_eq!(shared.observations().queue_depth_frames, 0);
    assert_eq!(
        shared.observations().queue_depth_invariant_failures_total,
        1
    );
}

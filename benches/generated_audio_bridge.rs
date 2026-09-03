use std::hint::spin_loop;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pocketstation::internal::frame::{
    AudioBufferPool, AudioFrame, ClockDomainId, FrameLineage, SampleFormat, SampleSpec, SessionId,
    SourceId, StemId, StreamId,
};
use pocketstation::internal::graph::{NodeId, RouteSettings, SignalEnvelope};
use pocketstation::internal::runtime::{
    plan_source_channel, GeneratedAudioBridge, GeneratedAudioBridgeSpec, PlanRunnerCancellation,
    TypedEdgeBranchSpec, TypedEdgeFanout,
};

const FRAME_SAMPLES: usize = 960;

fn bench_generated_audio_bridge(c: &mut Criterion) {
    let mut group = c.benchmark_group("generated_audio_bridge");
    group.throughput(Throughput::Elements(1));
    group.bench_function("typed_pcm_to_pooled_audio_round_trip", |b| {
        let cancellation = PlanRunnerCancellation::new();
        let (sender, mut input) =
            plan_source_channel(NodeId::from_index(1), 8, cancellation).expect("source channel");
        let (mut fanout, mut receivers) = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 8,
            route_settings: RouteSettings::bounded_async(),
        }])
        .expect("typed audio edge");
        let bridge = GeneratedAudioBridge::spawn(
            receivers.remove(0),
            sender,
            GeneratedAudioBridgeSpec {
                session_id: SessionId::new(1),
                stem_id: StemId::new(2),
                stream_id: StreamId::new(3),
                source_id: SourceId::new(4),
                clock_id: ClockDomainId::new(5),
                sample_spec: SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
                samples_per_frame: FRAME_SAMPLES,
                pool_slots: 8,
            },
        )
        .expect("generated-audio bridge");
        let input_pool = AudioBufferPool::new(64, FRAME_SAMPLES);
        let mut sequence_number = 0_u64;

        b.iter(|| {
            sequence_number = sequence_number.wrapping_add(1);
            let timestamp_ns = sequence_number.saturating_mul(20_000_000);
            let frame = AudioFrame::try_new(
                StreamId::new(30),
                SourceId::new(40),
                sequence_number,
                timestamp_ns,
                SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved),
                input_pool.acquire().expect("input pool"),
            )
            .expect("input frame");
            let lineage = FrameLineage::try_new(
                SessionId::new(9),
                SourceId::new(40),
                StemId::new(10),
                ClockDomainId::new(11),
                sequence_number,
                timestamp_ns,
                20_000_000,
                1,
                0,
                1,
            )
            .expect("input lineage");
            fanout
                .publish(SignalEnvelope::from_audio(frame, Some(lineage)), false)
                .expect("typed audio publish");

            let output = loop {
                if let Some(frame) = input.try_recv_for_testing() {
                    break frame;
                }
                spin_loop();
                std::thread::yield_now();
            };
            assert_eq!(output.frame().sequence_number(), sequence_number);
            black_box(output);
        });

        drop(fanout);
        bridge.finish_and_join();
    });
    group.finish();
}

criterion_group!(benches, bench_generated_audio_bridge);
criterion_main!(benches);

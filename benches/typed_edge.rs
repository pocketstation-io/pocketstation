use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pocketstation::internal::graph::{EventFormat, RouteSettings, SignalSpec};
use pocketstation::internal::runtime::{TypedEdgeBranchSpec, TypedEdgeFanout};
use pocketstation::{SignalEnvelope, SignalPayload};

fn bench_typed_edge(c: &mut Criterion) {
    let mut group = c.benchmark_group("typed_edge");
    group.throughput(Throughput::Elements(1));

    group.bench_function("publish_receive_one_branch_256_bytes", |b| {
        let (mut fanout, mut receivers) = TypedEdgeFanout::new(&[TypedEdgeBranchSpec {
            capacity_signals: 8,
            route_settings: RouteSettings::bounded_async(),
        }])
        .expect("bounded typed edge");
        b.iter(|| {
            let envelope = SignalEnvelope::untracked(
                SignalPayload::Bytes(vec![black_box(0_u8); 256]),
                SignalSpec::event(EventFormat::Json),
                black_box(0),
            );
            fanout.publish(envelope, false).expect("typed publish");
            black_box(receivers[0].recv().expect("typed receive"));
        });
    });

    group.bench_function("publish_receive_three_branches_256_bytes", |b| {
        let branch = TypedEdgeBranchSpec {
            capacity_signals: 8,
            route_settings: RouteSettings::bounded_async(),
        };
        let (mut fanout, mut receivers) =
            TypedEdgeFanout::new(&[branch, branch, branch]).expect("bounded typed fanout");
        b.iter(|| {
            let envelope = SignalEnvelope::untracked(
                SignalPayload::Bytes(vec![black_box(0_u8); 256]),
                SignalSpec::event(EventFormat::Json),
                black_box(0),
            );
            fanout.publish(envelope, false).expect("typed publish");
            for receiver in &mut receivers {
                black_box(receiver.recv().expect("typed receive"));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_typed_edge);
criterion_main!(benches);

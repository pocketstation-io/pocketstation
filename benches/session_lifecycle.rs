//! LOOPBACK-ONLY Session lifecycle benchmark.
//!
//! This measures public declaration, compile/prepare, deterministic fixture
//! thread startup, and clean stop/join. It does not use a physical capture
//! device and is intentionally separate from per-frame execution benchmarks.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pocketstation::{ApplicationSelector, Source};

fn bench_session_lifecycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("session_lifecycle_loopback_only");
    group.bench_function("declare_prepare_start_stop_two_sources", |b| {
        b.iter(|| {
            let session = pocketstation::conformance::session().expect("fixture Session");
            let application = session
                .capture(Source::application(ApplicationSelector::name(
                    "benchmark application",
                )))
                .expect("application declaration");
            let microphone = session
                .capture(Source::microphone_default())
                .expect("microphone declaration");
            application
                .send(session.polled_audio().expect("application endpoint"))
                .expect("application route");
            microphone
                .send(session.polled_audio().expect("microphone endpoint"))
                .expect("microphone route");

            let mut running = session.start().expect("running benchmark Session");
            let outcome = running.stop();
            assert!(outcome.is_success());
            black_box(outcome);
        });
    });
    group.finish();
}

criterion_group!(benches, bench_session_lifecycle);
criterion_main!(benches);

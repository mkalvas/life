use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use gol::app::{Pattern, State};

fn step_n(group: &mut BenchmarkGroup<WallTime>, pattern: Pattern, n: usize) {
    let base_state = State::new(&pattern);
    group.bench_function(format!("{}-{n}", pattern.as_str()).as_str(), |b| {
        b.iter(|| {
            let mut state = base_state.clone();
            for _ in 1..=n {
                state.step();
            }
        })
    });
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("step");
    group.sampling_mode(criterion::SamplingMode::Auto);
    step_n(&mut group, Pattern::SimpleExtinct, 10);
    group.sampling_mode(criterion::SamplingMode::Flat);
    step_n(&mut group, Pattern::Max, 10);
    step_n(&mut group, Pattern::Max, 100);
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

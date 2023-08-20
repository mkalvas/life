use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use gol::app::{InitPattern, State};

fn create_state(pattern: &str) -> State {
    State::new(InitPattern::Pattern(pattern.to_string()))
}

fn step_n(group: &mut BenchmarkGroup<WallTime>, pattern: &str, n: usize) {
    let base_state = create_state(pattern);
    group.bench_function(format!("{pattern}-{n}").as_str(), |b| {
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
    step_n(&mut group, "simple-extinct", 10);
    group.sampling_mode(criterion::SamplingMode::Flat);
    step_n(&mut group, "max", 10);
    step_n(&mut group, "max", 100);
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

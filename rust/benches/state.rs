use criterion::{criterion_group, criterion_main, Criterion};
use gol::app::{InitPattern, State};

fn create_state(pattern: &str) -> State {
    State::new(InitPattern::Pattern(pattern.to_string()))
}

fn step_n(c: &mut Criterion, pattern: &str, n: usize) {
    let base_state = create_state(pattern);
    c.bench_function(format!("step {n} {pattern}").as_str(), |b| {
        b.iter(|| {
            let mut state = base_state.clone();
            for _ in 1..=n {
                state.step();
            }
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    step_n(c, "simple-extinct", 10);
    step_n(c, "max", 10)
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

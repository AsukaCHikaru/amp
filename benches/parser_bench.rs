use amp::parser::amp::Amp;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const SMALL: &str = include_str!("../bench/fixtures/small.md");
const MEDIUM: &str = include_str!("../bench/fixtures/medium.md");
const LARGE: &str = include_str!("../bench/fixtures/large.md");

fn bench_parse(c: &mut Criterion) {
    let amp = Amp::new();

    c.bench_function("small document", |b| {
        b.iter(|| amp.parse(black_box(SMALL)))
    });

    c.bench_function("medium document", |b| {
        b.iter(|| amp.parse(black_box(MEDIUM)))
    });

    c.bench_function("large document", |b| {
        b.iter(|| amp.parse(black_box(LARGE)))
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);

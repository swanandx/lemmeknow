use criterion::{criterion_group, criterion_main, Criterion};
use lemmeknow::Identifier;

fn criterion_benchmark(c: &mut Criterion) {
 let identifier = Identifier::default();
    c.bench_function("lmk bench", |b| b.iter(|| identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
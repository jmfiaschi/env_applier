use criterion::{black_box, criterion_group, criterion_main, Criterion};
use env_applier::*;

fn simple_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("String");

    let config = r#"{"home":"{{ HOME }}","path":"{{ PATH }}"}"#;
    group.bench_function("Bench apply", |b| {
        b.iter(|| {
            black_box(config.apply());
        })
    });

    group.finish();
}

criterion_group!(benches, simple_string);
criterion_main!(benches);

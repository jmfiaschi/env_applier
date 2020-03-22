use criterion::{black_box, criterion_group, criterion_main, Criterion};
use env_applier::*;
use std::env::*;

fn simple_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("String");
    let text = r#"{"home":"{{ HOME }}","path":"{{ PATH }}"}"#;
    group.bench_function("Bench Vars::apply(string).", |b| {
        b.iter(|| black_box(Vars::apply(text.to_string())))
    });
    group.bench_function("Bench VarsOs::apply(string).", |b| {
        b.iter(|| black_box(VarsOs::apply(text.to_string())))
    });
    group.finish();
}

criterion_group!(benches, simple_string);
criterion_main!(benches);

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use pprof::criterion::{Output, PProfProfiler};

fn bench_parse_file(c: &mut Criterion) {
    let source: &str = include_str!("../src/lib.rs");
    let mut group = c.benchmark_group("parse_file");

    group.bench_function("moxy", |b| {
        b.iter(|| black_box(moxy::parse!(source as Vec<moxy::ast::Item>).unwrap()))
    });

    group.bench_function("syn", |b| b.iter(|| black_box(syn::parse_file(source).unwrap())));

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(
        PProfProfiler::new(
            100,
            Output::Flamegraph(None),
        ),
    );
    targets = bench_parse_file
}

criterion_main!(benches);

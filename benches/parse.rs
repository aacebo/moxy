use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

const ATTRIBUTED_USES: &str = include_str!("../src/lib.rs");
const MIXED_ITEMS: &str = include_str!("fixtures/mixed_items.rs");
const LARGE_ITEMS: &str = include_str!("fixtures/large_items.rs");
const ATTRIBUTES_DERIVES: &str = include_str!("fixtures/attributes_derives.rs");
const MACRO_HEAVY: &str = include_str!("fixtures/macro_heavy.rs");

const CONTROL_FLOW_EXPR: &str = include_str!("fixtures/control_flow_expr.rs");

const GENERIC_DEPTH_8: &str = include_str!("fixtures/generic_depth_8.rs");
const GENERIC_DEPTH_32: &str = include_str!("fixtures/generic_depth_32.rs");
const GENERIC_DEPTH_128: &str = include_str!("fixtures/generic_depth_128.rs");

const MALFORMED_EXPR: &str = "if ready";
const INVALID_DEEP_GENERIC: &str = include_str!("fixtures/invalid_deep_generic.rs");
const INVALID_MACRO: &str = include_str!("fixtures/invalid_macro.rs");
const INVALID_LARGE_FILE_TAIL: &str = include_str!("fixtures/invalid_large_file_tail.rs");

struct FileFixture {
    name: &'static str,
    source: &'static str,
    expected_items: usize,
}

struct TypeFixture {
    name: &'static str,
    source: &'static str,
}

struct InvalidFileFixture {
    name: &'static str,
    source: &'static str,
}

fn bench_file_pair(c: &mut Criterion, fixture: FileFixture) {
    let source = fixture.source;
    let moxy_items: Vec<moxy::ast::Item> = moxy::parse!(source).expect(fixture.name);
    let syn_file = syn::parse_file(source).expect(fixture.name);

    assert_eq!(
        moxy_items.len(),
        fixture.expected_items,
        "moxy item count differs for {}",
        fixture.name
    );
    assert_eq!(
        syn_file.items.len(),
        fixture.expected_items,
        "syn item count differs for {}",
        fixture.name
    );

    let mut group = c.benchmark_group(format!("parse_file/{}", fixture.name));

    group.throughput(Throughput::Bytes(fixture.source.len() as u64));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(fixture.source);
            let items: Vec<moxy::ast::Item> = moxy::parse!(source).unwrap();
            black_box(items)
        })
    });

    group.bench_function("syn", |b| {
        b.iter(|| {
            let source = black_box(fixture.source);
            black_box(syn::parse_file(source).unwrap())
        })
    });

    group.finish();
}

fn bench_expression_pair(c: &mut Criterion) {
    let _: moxy::ast::Expr = moxy::parse!(CONTROL_FLOW_EXPR).expect("moxy fixture must parse");
    let _: syn::Expr = syn::parse_str(CONTROL_FLOW_EXPR).expect("syn fixture must parse");
    let mut group = c.benchmark_group("parse_expr/control_flow");

    group.throughput(Throughput::Bytes(CONTROL_FLOW_EXPR.len() as u64));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(CONTROL_FLOW_EXPR);
            let expr: moxy::ast::Expr = moxy::parse!(source).unwrap();
            black_box(expr)
        })
    });

    group.bench_function("syn", |b| {
        b.iter(|| {
            let source = black_box(CONTROL_FLOW_EXPR);
            black_box(syn::parse_str::<syn::Expr>(source).unwrap())
        })
    });

    group.finish();
}

fn bench_type_pair(c: &mut Criterion, fixture: TypeFixture) {
    let source = fixture.source;
    let _: moxy::ast::Type = moxy::parse!(source).expect("moxy fixture must parse");
    let _: syn::Type = syn::parse_str(source).expect("syn fixture must parse");
    let mut group = c.benchmark_group(format!("parse_type/{}", fixture.name));

    group.throughput(Throughput::Bytes(fixture.source.len() as u64));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(fixture.source);
            let ty: moxy::ast::Type = moxy::parse!(source).unwrap();
            black_box(ty)
        })
    });

    group.bench_function("syn", |b| {
        b.iter(|| {
            let source = black_box(fixture.source);
            black_box(syn::parse_str::<syn::Type>(source).unwrap())
        })
    });

    group.finish();
}

fn bench_invalid_expression_pair(c: &mut Criterion) {
    let moxy_result: Result<moxy::ast::Expr, _> = moxy::parse!(MALFORMED_EXPR);
    assert!(moxy_result.is_err());
    assert!(syn::parse_str::<syn::Expr>(MALFORMED_EXPR).is_err());

    let mut group = c.benchmark_group("parse_invalid/expression");

    group.throughput(Throughput::Bytes(MALFORMED_EXPR.len() as u64));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(MALFORMED_EXPR);
            let result: Result<moxy::ast::Expr, _> = moxy::parse!(source);
            black_box(result)
        })
    });

    group.bench_function("syn", |b| {
        b.iter(|| {
            let source = black_box(MALFORMED_EXPR);
            black_box(syn::parse_str::<syn::Expr>(source))
        })
    });

    group.finish();
}

fn bench_invalid_file_pair(c: &mut Criterion, fixture: InvalidFileFixture) {
    let source = fixture.source;
    let moxy_result: Result<Vec<moxy::ast::Item>, _> = moxy::parse!(source);
    assert!(moxy_result.is_err(), "moxy invalid fixture parsed: {}", fixture.name);
    assert!(
        syn::parse_file(source).is_err(),
        "syn invalid fixture parsed: {}",
        fixture.name
    );

    let mut group = c.benchmark_group(format!("parse_invalid/{}", fixture.name));

    group.throughput(Throughput::Bytes(fixture.source.len() as u64));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(fixture.source);
            let result: Result<Vec<moxy::ast::Item>, _> = moxy::parse!(source);
            black_box(result)
        })
    });

    group.bench_function("syn", |b| {
        b.iter(|| {
            let source = black_box(fixture.source);
            black_box(syn::parse_file(source))
        })
    });

    group.finish();
}

pub fn run(c: &mut Criterion) {
    bench_file_pair(
        c,
        FileFixture {
            name: "attributed_uses",
            source: ATTRIBUTED_USES,
            expected_items: 12,
        },
    );
    bench_file_pair(
        c,
        FileFixture {
            name: "mixed_items",
            source: MIXED_ITEMS,
            expected_items: 6,
        },
    );
    bench_file_pair(
        c,
        FileFixture {
            name: "large_items",
            source: LARGE_ITEMS,
            expected_items: 25,
        },
    );
    bench_file_pair(
        c,
        FileFixture {
            name: "attributes_derives",
            source: ATTRIBUTES_DERIVES,
            expected_items: 9,
        },
    );
    bench_file_pair(
        c,
        FileFixture {
            name: "macro_heavy",
            source: MACRO_HEAVY,
            expected_items: 13,
        },
    );
    bench_expression_pair(c);
    bench_type_pair(
        c,
        TypeFixture {
            name: "generic_depth_8",
            source: GENERIC_DEPTH_8,
        },
    );
    bench_type_pair(
        c,
        TypeFixture {
            name: "generic_depth_32",
            source: GENERIC_DEPTH_32,
        },
    );
    bench_type_pair(
        c,
        TypeFixture {
            name: "generic_depth_128",
            source: GENERIC_DEPTH_128,
        },
    );
    bench_invalid_expression_pair(c);
    bench_invalid_file_pair(
        c,
        InvalidFileFixture {
            name: "deep_generic",
            source: INVALID_DEEP_GENERIC,
        },
    );
    bench_invalid_file_pair(
        c,
        InvalidFileFixture {
            name: "macro",
            source: INVALID_MACRO,
        },
    );
    bench_invalid_file_pair(
        c,
        InvalidFileFixture {
            name: "large_file_tail",
            source: INVALID_LARGE_FILE_TAIL,
        },
    );
}

criterion_group! {
    name = benchmark_group;
    config = Criterion::default();
    targets = run
}

criterion_main!(benchmark_group);

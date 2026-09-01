use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use pprof::criterion::{Output, PProfProfiler};

const ATTRIBUTED_USES: &str = include_str!("../src/lib.rs");
const MIXED_ITEMS: &str = r#"
#[derive(Clone, Debug)]
pub struct Envelope<'a, T: Clone> {
    pub id: u64,
    pub payload: &'a T,
}

pub enum Message<T> {
    Empty,
    Value(T),
    Record { id: u64, value: T },
}

pub trait Service<T>: Send {
    type Error;
    fn call(&self, value: T) -> Result<T, Self::Error>;
}

impl<'a, T: Clone + Send> Envelope<'a, T> {
    pub fn map<U: Clone>(&self, value: U) -> Envelope<'_, U> {
        Envelope { id: self.id, payload: &value }
    }
}

pub fn transform<T: Clone>(items: Vec<T>) -> Option<T> {
    items.into_iter().next()
}

macro_rules! passthrough {
    ($value:expr) => { $value };
}
"#;

const CONTROL_FLOW_EXPR: &str = r#"
match request {
    Some(value) if value > 10 => {
        let adjusted = value * 2 + compute(value)?;
        if adjusted > limit { adjusted } else { limit }
    }
    Some(value) => value,
    None => return fallback(),
}
"#;

const NESTED_TYPE: &str =
    "::std::collections::HashMap<String, Vec<Option<Result<Box<dyn Iterator<Item = &'static [u8]> + Send>, Error>>>>";

const MALFORMED_EXPR: &str = "if ready";

fn bench_file_pair(c: &mut Criterion, name: &str, source: &'static str, expected_items: usize) {
    let moxy_items = moxy::parse!(source as Vec<moxy::ast::Item>).expect("moxy fixture must parse");
    let syn_file = syn::parse_file(source).expect("syn fixture must parse");

    assert_eq!(moxy_items.len(), expected_items);
    assert_eq!(syn_file.items.len(), expected_items);

    let mut group = c.benchmark_group(format!("parse_file/{name}"));

    group.throughput(Throughput::Bytes(source.len() as u64));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(source);
            black_box(moxy::parse!(source as Vec<moxy::ast::Item>).unwrap())
        })
    });

    group.bench_function("syn", |b| {
        b.iter(|| {
            let source = black_box(source);
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
            black_box(moxy::parse!(source as moxy::ast::Expr).unwrap())
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

fn bench_type_pair(c: &mut Criterion) {
    let _: moxy::ast::Type = moxy::parse!(NESTED_TYPE).expect("moxy fixture must parse");
    let _: syn::Type = syn::parse_str(NESTED_TYPE).expect("syn fixture must parse");
    let mut group = c.benchmark_group("parse_type/nested");

    group.throughput(Throughput::Bytes(NESTED_TYPE.len() as u64));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(NESTED_TYPE);
            black_box(moxy::parse!(source as moxy::ast::Type).unwrap())
        })
    });

    group.bench_function("syn", |b| {
        b.iter(|| {
            let source = black_box(NESTED_TYPE);
            black_box(syn::parse_str::<syn::Type>(source).unwrap())
        })
    });

    group.finish();
}

fn bench_invalid_pair(c: &mut Criterion) {
    assert!(moxy::parse!(MALFORMED_EXPR as moxy::ast::Expr).is_err());
    assert!(syn::parse_str::<syn::Expr>(MALFORMED_EXPR).is_err());

    let mut group = c.benchmark_group("parse_invalid/expression");

    group.throughput(Throughput::Bytes(MALFORMED_EXPR.len() as u64));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(MALFORMED_EXPR);
            black_box(moxy::parse!(source as moxy::ast::Expr))
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

pub fn run(c: &mut Criterion) {
    bench_file_pair(c, "attributed_uses", ATTRIBUTED_USES, 11);
    bench_file_pair(c, "mixed_items", MIXED_ITEMS, 6);
    bench_expression_pair(c);
    bench_type_pair(c);
    bench_invalid_pair(c);
}

criterion_group! {
    name = benchmark_group;
    config = Criterion::default().with_profiler(
        PProfProfiler::new(
            100,
            Output::Flamegraph(None),
        ),
    );
    targets = run
}

criterion_main!(benchmark_group);

//! Benchmarks comparing zynix parse / to_tokens against syn on a shared corpus.
//!
//! Run with: `cargo bench -p zynix --features ast,proc-macro2`
#![cfg(all(feature = "ast", feature = "proc-macro2"))]

use std::str::FromStr;

use criterion::{Criterion, criterion_group, criterion_main};
use quote::ToTokens as _;
use zynix::token::ToTokenStream as _;
use zynix::{Parse, TokenStream};

const EXPR: &str = "a + b * c - d.method::<u8>(x, y)?.field";
const TYPE: &str = "std::collections::HashMap<String, Vec<Box<dyn Fn(u8) -> bool>>>";
const ITEM: &str = "pub fn frobnicate<T: Clone + Send>(x: T, y: u8) -> Vec<T> where T: 'static { vec![x] }";

fn zynix_parse<T: Parse>(src: &str) -> T {
    let ts = TokenStream::from_str(src).unwrap();
    ts.parse().parse::<T>().unwrap()
}

fn bench_parse(c: &mut Criterion) {
    let mut g = c.benchmark_group("parse/expr");
    g.bench_function("zynix", |b| b.iter(|| zynix_parse::<zynix::ast::Expr>(EXPR)));
    g.bench_function("syn", |b| b.iter(|| syn::parse_str::<syn::Expr>(EXPR).unwrap()));
    g.finish();

    let mut g = c.benchmark_group("parse/type");
    g.bench_function("zynix", |b| b.iter(|| zynix_parse::<zynix::ast::Type>(TYPE)));
    g.bench_function("syn", |b| b.iter(|| syn::parse_str::<syn::Type>(TYPE).unwrap()));
    g.finish();

    let mut g = c.benchmark_group("parse/item");
    g.bench_function("zynix", |b| b.iter(|| zynix_parse::<zynix::ast::Item>(ITEM)));
    g.bench_function("syn", |b| b.iter(|| syn::parse_str::<syn::Item>(ITEM).unwrap()));
    g.finish();
}

fn bench_to_tokens(c: &mut Criterion) {
    let z_item = zynix_parse::<zynix::ast::Item>(ITEM);
    let s_item = syn::parse_str::<syn::Item>(ITEM).unwrap();

    let mut g = c.benchmark_group("to_tokens/item");
    g.bench_function("zynix", |b| b.iter(|| z_item.to_token_stream()));
    g.bench_function("syn", |b| b.iter(|| s_item.to_token_stream()));
    g.finish();
}

criterion_group!(benches, bench_parse, bench_to_tokens);
criterion_main!(benches);

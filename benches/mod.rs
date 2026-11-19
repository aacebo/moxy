mod parse;
mod pipeline;

use criterion::{criterion_group, criterion_main};

criterion_group!(benches, parse::run, pipeline::run);
criterion_main!(benches);

mod parse;

use criterion::{criterion_group, criterion_main};

criterion_group!(benches, parse::run);
criterion_main!(benches);

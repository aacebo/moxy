use std::hint::black_box;
use std::time::Duration;

use criterion::{Criterion, Throughput};
use moxy::ast::{Field, Item, Visit};
use moxy::token::{Ident, ToTokenStream};
use quote::{ToTokens, format_ident, quote};

// The repository's own item fixtures, repeated into a source of realistic size
fn large_source() -> String {
    let fixtures = [
        include_str!("../../fixtures/parse/item/large_items.rs"),
        include_str!("../../fixtures/parse/item/mixed_items.rs"),
        include_str!("../../fixtures/parse/item/attributes_derives.rs"),
        include_str!("../../fixtures/parse/item/macro_heavy.rs"),
        include_str!("../../fixtures/parse/item/declaration_heavy.rs"),
        include_str!("../../fixtures/parse/item/pattern_heavy.rs"),
    ];

    (0..20).map(|_| fixtures.concat()).collect::<Vec<_>>().join("\n")
}

// A struct of the size an attribute macro meets
fn struct_source() -> String {
    let fields: String = (0..64)
        .map(|index| format!("    #[doc = \"field {index}\"]\n    pub field_{index}: Vec<u32>,\n"))
        .collect();
    format!("#[derive(Debug, Clone)]\npub struct Config<T: Clone> where T: Send {{\n{fields}    pub extra: T,\n}}\n")
}

#[derive(Default)]
struct FieldNames(Vec<Ident>);

impl<'ast> Visit<'ast> for FieldNames {
    fn visit_field_def(&mut self, node: &'ast Field) {
        if let Some(ident) = &node.ident {
            self.0.push(ident.clone());
        }
    }
}

pub fn run(c: &mut Criterion) {
    let source = large_source();
    let file: moxy::ast::File = moxy::parse!((source.clone())).expect("the large source parses");
    let syn_file = syn::parse_file(&source).expect("syn parses the large source");

    let mut group = c.benchmark_group("pipeline/lex");
    group.throughput(Throughput::Bytes(source.len() as u64));
    group.measurement_time(Duration::from_secs(4));
    group.bench_function("moxy", |b| {
        b.iter(|| black_box(&source).parse::<moxy::token::TokenStream>().unwrap())
    });
    group.bench_function("proc-macro2", |b| {
        b.iter(|| black_box(&source).parse::<proc_macro2::TokenStream>().unwrap())
    });
    group.finish();

    let mut group = c.benchmark_group("pipeline/parse_file");
    group.throughput(Throughput::Bytes(source.len() as u64));
    group.measurement_time(Duration::from_secs(4));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(source.as_str());
            let file: moxy::ast::File = moxy::parse!(source).unwrap();
            file
        })
    });
    group.bench_function("syn", |b| b.iter(|| syn::parse_file(black_box(&source)).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("pipeline/emit");
    group.measurement_time(Duration::from_secs(4));
    group.bench_function("moxy", |b| b.iter(|| black_box(&file).to_token_stream()));
    group.bench_function("syn", |b| b.iter(|| black_box(&syn_file).to_token_stream()));
    group.finish();

    let mut group = c.benchmark_group("pipeline/format");
    group.measurement_time(Duration::from_secs(4));
    group.bench_function("moxy", |b| b.iter(|| moxy::fmt!(black_box(&file)).unwrap()));
    group.finish();

    let names: Vec<Ident> = (0..200).map(|index| moxy::ident!(format!("field_{index}"))).collect();
    let syn_names: Vec<proc_macro2::Ident> = (0..200_u32).map(|index| format_ident!("field_{}", index)).collect();
    let mut group = c.benchmark_group("pipeline/template");
    group.measurement_time(Duration::from_secs(4));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let (names, checked) = (black_box(&names), black_box(true));
            moxy::template! {
                pub struct Generated {
                    @for name in names { pub {{ name }}: u32, }
                }

                impl Generated {
                    pub fn total(&self) -> u32 {
                        0 @for name in names { @if checked { + self.{{ name }} } @else { + 0 } }
                    }
                }
            }
        })
    });
    group.bench_function("quote", |b| {
        b.iter(|| {
            let (names, checked) = (black_box(&syn_names), black_box(true));
            let terms = names
                .iter()
                .map(|name| if checked { quote!(+ self.#name) } else { quote!(+ 0) });
            quote! {
                pub struct Generated {
                    #(pub #names: u32,)*
                }

                impl Generated {
                    pub fn total(&self) -> u32 {
                        0 #(#terms)*
                    }
                }
            }
        })
    });
    group.finish();

    let item_source = struct_source();
    let mut group = c.benchmark_group("pipeline/attribute_macro");
    group.measurement_time(Duration::from_secs(4));
    group.bench_function("moxy", |b| {
        b.iter(|| {
            let source = black_box(item_source.as_str());
            let item: Item = moxy::parse!(source).unwrap();
            let mut names = FieldNames::default();
            names.visit_item(&item);
            let names = names.0;
            moxy::template! {
                impl Config {
                    @for name in &names {
                        pub fn {{ name }}(&self) -> &Vec<u32> { &self.{{ name }} }
                    }
                }
            }
        })
    });
    group.bench_function("syn", |b| {
        b.iter(|| {
            let item: syn::ItemStruct = syn::parse_str(black_box(&item_source)).unwrap();
            let names: Vec<&proc_macro2::Ident> = item.fields.iter().filter_map(|field| field.ident.as_ref()).collect();
            quote! {
                impl Config {
                    #(pub fn #names(&self) -> &Vec<u32> { &self.#names })*
                }
            }
        })
    });
    group.finish();
}

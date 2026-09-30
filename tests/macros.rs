extern crate proc_macro;

use moxy::ToTokens;
use moxy::ast::Item;
use moxy::token::{Spanner, ToTokenStream};

#[derive(ToTokens)]
#[moxy(template {
    pub const GENERATED: &str = {{ self.value }};
})]
struct Model {
    value: String,
}

macro_rules! double {
    (
        $(#[$($attr:meta),*])*
        $vis:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block
    ) => {
        $(#[$($attr),*])*
        $vis fn $name($($args)*) $(-> $ret)? {
            let value = $body;
            value * 2
        }
    };
}

macro_rules! plus_3 {
    (
        $(#[$($attr:meta),*])*
        $vis:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block
    ) => {
        $(#[$($attr),*])*
        $vis fn $name($($args)*) $(-> $ret)? {
            let value = $body;
            value + 3
        }
    };
}

macro_rules! times_10 {
    (
        $(#[$($attr:meta),*])*
        $vis:vis fn $name:ident($($args:tt)*) $(-> $ret:ty)? $body:block
    ) => {
        $(#[$($attr),*])*
        $vis fn $name($($args)*) $(-> $ret)? {
            let value = $body;
            value * 10
        }
    };
}

#[moxy::apply(double)]
fn double_add(a: isize, b: isize) -> isize {
    a + b
}

#[moxy::apply(double, plus_3, times_10)]
#[inline]
fn add(a: isize, b: isize) -> isize {
    a + b
}

#[test]
fn derive_output_completes_a_constant_syntax_pipeline() {
    let tokens = Model { value: "ready".into() }.to_token_stream();
    let item: Item = moxy::parse!(tokens).unwrap();
    let constant = item.as_const().unwrap();
    assert_eq!(constant.ident.text(), "GENERATED");
    assert!(constant.vis.is_public());
    assert!(!constant.span().is_empty());
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub const GENERATED: &str = \"ready\";");
}

#[test]
fn derive_compiler_contracts_are_stable() {
    let cases = trybuild::TestCases::new();
    cases.pass("fixtures/trybuild/derive/pass/*.rs");
    cases.compile_fail("fixtures/trybuild/derive/fail/*.rs");

    let files = moxy::parse_files!("fixtures/trybuild/template/pass/*.rs");
    assert_eq!(files.len(), 2);
    assert!(files.iter().all(|file| !file.items.is_empty()));
    assert_eq!(files.iter().map(|file| file.items.len()).sum::<usize>(), 5);
}

#[test]
fn apply_macro_rules_as_attribute() {
    assert_eq!(double_add(2, 5), 14);
    assert_eq!(add(2, 5), 170);
}

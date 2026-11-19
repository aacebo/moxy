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

#[test]
fn a_macro_call_in_parentheses_or_brackets_ends_with_a_semicolon() {
    for source in ["m!();", "m![];", "m! {}", "fn f() { m!() }", "fn f() { m!{}; m!(); m![] }"] {
        assert!(moxy::parse!(source as moxy::ast::File).is_ok(), "`{source}`");
    }

    for source in ["m!()", "m![]", "m!{};", "fn f() { m!() b }", "fn f() { m![] m!() }"] {
        assert!(moxy::parse!(source as moxy::ast::File).is_err(), "`{source}`");
    }
}

#[test]
fn declarative_macros_of_the_macro_form_parse_as_items() {
    for (source, params) in [
        ("macro m { () => {} }", false),
        ("pub(crate) macro m { (a) => {}, (b) => {} }", false),
        ("pub macro m($x:expr) { $x }", true),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let mac = item.as_decl_macro().unwrap();
        assert_eq!((mac.ident.text(), mac.params.is_some()), ("m", params), "`{source}`");
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(item.to_token_stream().to_string(), lexed.to_string());
        let formatted = moxy::fmt!(&item).unwrap();
        let again: Item = moxy::parse!(formatted).unwrap();
        assert_eq!(again.to_token_stream().to_string(), lexed.to_string());
    }

    // As in rustc's parser, the body is in braces
    for source in ["macro m($x:expr) ( $x )", "macro m;", "macro m [ () => {} ]"] {
        assert!(moxy::parse!(source as Item).is_err(), "`{source}`");
    }

    let block: moxy::ast::StmtBlock = moxy::parse!("{ macro m { () => {} } m!() }").unwrap();
    assert!(block.stmts.inner[0].as_item().unwrap().is_decl_macro());
}

#[test]
fn macro_rules_without_a_name_is_a_macro_call() {
    for source in ["macro_rules! {}", "macro_rules! ();", "macro_rules! [];"] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert!(item.is_macro(), "`{source}`");
    }

    let item: Item = moxy::parse!("macro_rules! m {}").unwrap();
    assert!(item.is_macro2());
}

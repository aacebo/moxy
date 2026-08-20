#![cfg(feature = "derive")]

use moxy::ToTokens;
use moxy::ast::Item;
use moxy::token::{Spanner, ToTokenStream};

#[derive(ToTokens)]
#[template {
    pub const GENERATED: &str = {{ self.value }};
}]
struct Model {
    value: String,
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
    cases.pass("tests/ui/derives/pass/*.rs");
    cases.compile_fail("tests/ui/derives/fail/*.rs");
}

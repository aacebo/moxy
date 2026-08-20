use moxy_ast::{Crate, Declaration, Expr, Item, Member, Pattern, Stmt, Type};
use moxy_token::{Spanner, ToTokenStream, TokenStream};

#[test]
fn expressions_cover_primary_postfix_unary_binary_block_and_jump_forms() {
    for source in [
        "42",
        "true",
        "path::to::value",
        "(value)",
        "(a, b, c)",
        "[a, b, c]",
        "[value; 4]",
        "Point { x: 1, y, ..base }",
        "async move { work().await? }",
        "const { 1 + 2 }",
        "unsafe { call() }",
        "try { operation()? }",
        "loop { break 1 }",
        "while let Some(x) = next() { continue }",
        "for item in items { consume(item); }",
        "if condition { yes() } else if other { maybe() } else { no() }",
        "match value { Some(x) if x > 0 => x, None => 0, _ => 1 }",
        "|x: i32| -> i32 { x + 1 }",
        "move || value",
        "object.field",
        "tuple.0",
        "object.method::<T>(a, b)",
        "function::<T>(a, b)",
        "array[index]",
        "future.await",
        "value?",
        "-value",
        "!flag",
        "&mut value",
        "value as u64",
        "a + b * c - d / e",
        "a == b && c != d || ready",
        "target = value",
        "target += value",
        "start..end",
        "start..=end",
        "return value",
        "break 'label value",
        "continue 'label",
        "yield value",
        "let Some(value) = option",
        "vec![1, 2, 3]",
    ] {
        let expression: Expr = moxy_token::parse!(source).unwrap();
        assert!(!expression.span().is_empty(), "missing span for {source}");
        let rendered = expression.to_token_stream().to_string();
        let reparsed: Expr = moxy_token::parse!(rendered).unwrap();
        assert_eq!(
            reparsed.to_token_stream().to_string(),
            rendered,
            "roundtrip failed for {source}"
        );
    }
}

#[test]
fn patterns_cover_bindings_structures_ranges_and_type_annotations() {
    for source in [
        "_",
        "..",
        "name",
        "ref mut name @ Some(_)",
        "&mut value",
        "(a, b, ..)",
        "[first, .., last]",
        "Some(value)",
        "Point { x, y: renamed, .. }",
        "A | B | C",
        "1",
        "1..=10",
        "path::CONST",
        "box value",
        "const { 1 }",
        "value: Option<T>",
    ] {
        let pattern: Pattern = moxy_token::parse!(source).unwrap_or_else(|error| panic!("failed to parse {source}: {error}"));
        let rendered = pattern.to_token_stream().to_string();
        let reparsed: Pattern = moxy_token::parse!(rendered).unwrap();
        assert_eq!(
            reparsed.to_token_stream().to_string(),
            rendered,
            "roundtrip failed for {source}"
        );
    }
}

#[test]
fn types_cover_every_public_shape_and_nested_arguments() {
    for source in [
        "!",
        "_",
        "T",
        "std::collections::HashMap<String, Vec<u8>>",
        "<T as Trait>::Assoc",
        "&'a mut [T]",
        "*const T",
        "*mut T",
        "[u8]",
        "[u8; 32]",
        "(T)",
        "(A, B, C)",
        "()",
        "impl Clone + Send + 'static",
        "dyn Trait<Item = T> + Send + 'a",
        "unsafe extern \"C\" fn(&'a str) -> Result<T, E>",
        "m!(T)",
    ] {
        let ty: Type = moxy_token::parse!(source).unwrap_or_else(|error| panic!("failed to parse {source}: {error}"));
        let rendered = ty.to_token_stream().to_string();
        let reparsed: Type = moxy_token::parse!(rendered).unwrap();
        assert_eq!(
            reparsed.to_token_stream().to_string(),
            rendered,
            "roundtrip failed for {source}"
        );
    }
}

#[test]
fn statements_cover_locals_items_macros_and_terminated_expressions() {
    for source in [
        "let value = 1;",
        "let mut value: u64 = 1 else { return; };",
        "function();",
        "value",
        "macro_call!();",
        "const LOCAL: usize = 1;",
    ] {
        let statement: Stmt = moxy_token::parse!(source).unwrap();
        let rendered = statement.to_token_stream().to_string();
        assert!(!rendered.is_empty());
    }
}

#[test]
fn items_cover_the_complete_top_level_and_member_grammar() {
    let mut items = Vec::new();
    for source in [
        "use std::{collections::{HashMap, HashSet}, fmt as formatting, io::*};",
        "extern crate core as rust_core;",
        "mod inline { pub const VALUE: usize = 1; }",
        "mod external;",
        "pub extern \"C\" fn function<T: Clone>(value: T, ...) -> T where T: Send { value }",
        "pub struct Named<T> where T: Clone { pub value: T, hidden: usize }",
        "pub struct Tuple(pub i32, String);",
        "pub struct Unit;",
        "pub enum Choice<T> { Unit, Tuple(T), Named { value: T } }",
        "pub union Storage { integer: u64, float: f64 }",
        "pub unsafe auto trait Marker: Send {}",
        "pub trait Service<T>: Send where T: Clone { const LIMIT: usize = 1; type Output: Clone; fn call(&self, value: T) -> Self::Output; macro_call!(); }",
        "pub trait Alias = Clone + Send;",
        "impl<T: Clone> Service<T> for Named<T> { const LIMIT: usize = 2; type Output = T; fn call(&self, value: T) -> T { value } macro_call!(); }",
        "type AliasType<T> = Option<T>;",
        "const CONST_VALUE: usize = 4;",
        "static mut STATIC_VALUE: usize = 5;",
        "macro_call!(tokens);",
        "macro_rules! local_macro { () => {}; }",
        "extern \"C\" { static FOREIGN: u8; type ForeignType; fn foreign(value: i32) -> i32; macro_call!(); }",
    ] {
        let item: Item = moxy_token::parse!(source).unwrap_or_else(|error| panic!("failed to parse {source}: {error}"));
        let rendered = item.to_token_stream().to_string();
        let reparsed: Item =
            moxy_token::parse!(rendered.clone()).unwrap_or_else(|error| panic!("failed to reparse {rendered}: {error}"));
        assert_eq!(reparsed.to_token_stream().to_string(), rendered);
        items.push(item);
    }

    assert_eq!(items.len(), 20);
    assert!(items.iter().any(Item::is_use));
    assert!(items.iter().any(Item::is_extern_crate));
    assert!(items.iter().any(Item::is_mod));
    assert!(items.iter().any(Item::is_fn));
    assert!(items.iter().any(Item::is_struct));
    assert!(items.iter().any(Item::is_enum));
    assert!(items.iter().any(Item::is_union));
    assert!(items.iter().any(Item::is_trait));
    assert!(items.iter().any(Item::is_trait_alias));
    assert!(items.iter().any(Item::is_impl));
    assert!(items.iter().any(Item::is_type_alias));
    assert!(items.iter().any(Item::is_const));
    assert!(items.iter().any(Item::is_static));
    assert!(items.iter().any(Item::is_macro));
    assert!(items.iter().any(Item::is_macro2));
    assert!(items.iter().any(Item::is_foreign_mod));

    let parsed: Crate = moxy_token::parse!("#![allow(dead_code)] const VALUE: usize = 1;").unwrap();
    assert_eq!(parsed.attrs.len(), 1);
    assert_eq!(parsed.items.len(), 1);
    let rendered = parsed.to_token_stream().to_string();
    let reparsed: Crate = moxy_token::parse!(rendered.clone()).unwrap();
    assert_eq!(reparsed.to_token_stream().to_string(), rendered);
}

#[test]
fn declarations_expose_common_accessors_and_variant_downcasts() {
    let enum_decl = moxy_token::parse!("#[repr(u8)] pub enum Choice<T> { One, Value(T) }" as Declaration).unwrap();
    assert!(enum_decl.is_enum());
    assert!(!enum_decl.is_struct());
    assert_eq!(enum_decl.ident().text(), "Choice");
    assert_eq!(enum_decl.attrs().len(), 1);
    assert!(enum_decl.as_enum().is_some());
    assert!(enum_decl.as_struct().is_none());

    let struct_decl = moxy_token::parse!("pub struct Record<T> where T: Clone { value: T }" as Declaration).unwrap();
    assert!(struct_decl.is_struct());
    assert!(struct_decl.as_struct().unwrap().fields.is_named());
    assert!(!struct_decl.generics().params.is_empty());
    assert_eq!(struct_decl.vis().to_token_stream().to_string(), "pub");

    let union_decl = moxy_token::parse!("union Bits { integer: u64, float: f64 }" as Declaration).unwrap();
    assert!(union_decl.is_union());
    assert!(union_decl.as_union().is_some());
    assert!(union_decl.as_enum().is_none());
}

#[test]
#[ignore]
fn tuple_members_require_unsuffixed_decimal_indices() {
    assert!(moxy_token::parse!("0" as Member).is_ok());
    assert!(moxy_token::parse!("field" as Member).is_ok());

    for invalid in ["0u8", "0x1", "1_0", "4294967296"] {
        let result: Result<Member, _> = moxy_token::parse!(invalid);
        assert!(result.is_err(), "accepted invalid member {invalid}");
    }
}

#[test]
fn malformed_syntax_reports_errors_without_consuming_valid_prefixes() {
    assert!(moxy_token::parse!("*T" as Type).is_err());
    assert!(moxy_token::parse!("if condition" as Expr).is_err());
    assert!(moxy_token::parse!("struct" as Item).is_err());
    assert!(moxy_token::parse!("let = 1;" as Stmt).is_err());
    assert!(moxy_token::parse!("&" as Pattern).is_err());

    let tokens: TokenStream = "name trailing".parse().unwrap();
    let mut stream = tokens.parse();
    assert_eq!(stream.parse::<moxy_ast::Ident>().unwrap().text(), "name");
    assert_eq!(stream.parse::<moxy_ast::Ident>().unwrap().text(), "trailing");
}

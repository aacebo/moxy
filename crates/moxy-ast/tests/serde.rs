#![cfg(feature = "serde")]

use moxy_ast::{Crate, Declaration, Expr, Item, Pattern, Stmt, Type};

#[test]
fn serde_covers_representative_public_ast_shapes() {
    let values = [
        serde_json::to_value(moxy_token::parse!("pub struct Record<T> { value: T }" as Declaration).unwrap()).unwrap(),
        serde_json::to_value(moxy_token::parse!("if ready { call() } else { fallback() }" as Expr).unwrap()).unwrap(),
        serde_json::to_value(moxy_token::parse!("Point { x, y: value, .. }" as Pattern).unwrap()).unwrap(),
        serde_json::to_value(moxy_token::parse!("dyn Trait<Item = T> + Send" as Type).unwrap()).unwrap(),
        serde_json::to_value(moxy_token::parse!("#![allow(dead_code)] const VALUE: usize = 1;" as Crate).unwrap()).unwrap(),
    ];

    for value in values {
        assert!(value.is_object() || value.is_string());
        assert!(!value.to_string().is_empty());
    }
}

#[test]
fn serde_expression_variants_have_the_expected_public_tags_and_payloads() {
    for (source, outer, inner) in [
        ("-value", "Unary", "Unary"),
        ("&mut value", "Unary", "Reference"),
        ("value as u64", "Unary", "Cast"),
        ("value?", "Unary", "Try"),
        ("a + b", "Binary", "Binary"),
        ("target = value", "Binary", "Assign"),
        ("target += value", "Binary", "AssignOp"),
        ("start..=end", "Binary", "Range"),
        ("function(a, b)", "Postfix", "Call"),
        ("object.method(a)", "Postfix", "MethodCall"),
        ("object.field", "Postfix", "Field"),
        ("array[index]", "Postfix", "Index"),
        ("future.await", "Postfix", "Await"),
        ("{ value }", "Block", "Brace"),
        ("if ready { yes() } else { no() }", "Block", "If"),
        ("while ready { work(); }", "Block", "While"),
        ("for item in items { use_item(item); }", "Block", "ForLoop"),
        ("loop { break; }", "Block", "Loop"),
        ("match value { Some(x) => x, _ => 0 }", "Block", "Match"),
        ("async move { work().await }", "Block", "Async"),
        ("unsafe { call() }", "Block", "Unsafe"),
        ("const { 1 }", "Block", "Const"),
        ("try { work()? }", "Block", "TryBlock"),
        ("return value", "Jump", "Return"),
        ("break 'label value", "Jump", "Break"),
        ("continue 'label", "Jump", "Continue"),
        ("yield value", "Jump", "Yield"),
        ("42", "Primary", "Lit"),
        ("path::value", "Primary", "Path"),
        ("Point { x: 1, ..base }", "Primary", "Struct"),
        ("|x: i32| x + 1", "Primary", "Closure"),
        ("(a, b)", "Primary", "Tuple"),
        ("[a, b]", "Primary", "Array"),
        ("[value; 3]", "Primary", "Repeat"),
        ("let Some(value) = option", "Primary", "Let"),
        ("(value)", "Primary", "Paren"),
        ("macro_call!(tokens)", "Primary", "Macro"),
    ] {
        let expression: Expr = moxy_token::parse!(source).unwrap();
        let value = serde_json::to_value(&expression).unwrap();
        let outer_value = value.get(outer).unwrap_or_else(|| panic!("missing {outer} in {value}"));
        assert!(
            outer_value.get(inner).is_some(),
            "missing {inner} for {source} in {outer_value}"
        );
    }
}

#[test]
fn serde_pattern_and_type_variants_have_expected_public_tags() {
    for (source, tag) in [
        ("_", "Wild"),
        ("..", "Rest"),
        ("ref mut name @ Some(_)", "Ident"),
        ("path::CONST", "Path"),
        ("(a, b, ..)", "Tuple"),
        ("Some(value)", "TupleStruct"),
        ("Point { x, y: renamed, .. }", "Struct"),
        ("[first, .., last]", "Slice"),
        ("&mut value", "Reference"),
        ("A | B | C", "Or"),
        ("1", "Lit"),
        ("1..=10", "Lit"),
        ("box value", "Box"),
        ("const { 1 }", "Const"),
    ] {
        let pattern: Pattern = moxy_token::parse!(source).unwrap();
        let value = serde_json::to_value(&pattern).unwrap();
        assert!(
            value.get(tag).is_some() || value.as_str() == Some(tag),
            "missing {tag} for {source}: {value}"
        );
    }

    let macro_pattern = Pattern::Macro(moxy_token::parse!("m!(value)" as moxy_ast::MacroCall).unwrap());
    let value = serde_json::to_value(macro_pattern).unwrap();
    assert!(value.get("Macro").is_some(), "missing Macro in {value}");

    let typed_pattern = moxy_ast::pat::PatType {
        attrs: Default::default(),
        pat: Box::new(moxy_token::parse!("value" as Pattern).unwrap()),
        colon: Default::default(),
        ty: Box::new(moxy_token::parse!("Option<T>" as Type).unwrap()),
    }
    .into_pattern();
    let value = serde_json::to_value(typed_pattern).unwrap();
    assert!(value.get("Type").is_some(), "missing Type in {value}");

    for (source, tag) in [
        ("!", "Never"),
        ("_", "Infer"),
        ("std::vec::Vec<T>", "Path"),
        ("(A, B)", "Tuple"),
        ("[u8; 32]", "Array"),
        ("[u8]", "Slice"),
        ("&'a mut T", "Reference"),
        ("*const T", "Pointer"),
        ("unsafe extern \"C\" fn(&str) -> usize", "BareFn"),
        ("impl Clone + Send", "ImplTrait"),
        ("dyn Trait<Item = T> + Send", "TraitObject"),
        ("(T)", "Paren"),
        ("m!(T)", "Macro"),
    ] {
        let ty: Type = moxy_token::parse!(source).unwrap();
        let value = serde_json::to_value(&ty).unwrap();
        assert!(value.get(tag).is_some(), "missing {tag} for {source}: {value}");
    }
}

#[test]
fn serde_items_statements_and_members_expose_real_structured_output() {
    for (source, tag) in [
        ("use std::{fmt as formatting, io::*};", "Use"),
        ("extern crate core as rust_core;", "ExternCrate"),
        ("mod inline { pub const VALUE: usize = 1; }", "Mod"),
        ("pub fn function<T: Clone>(value: T) -> T { value }", "Fn"),
        ("pub struct Named<T> { pub value: T }", "Struct"),
        ("pub enum Choice<T> { Unit, Tuple(T), Named { value: T } }", "Enum"),
        ("pub union Storage { integer: u64, float: f64 }", "Union"),
        (
            "pub trait Service<T>: Send { const LIMIT: usize = 1; type Output; fn call(&self, value: T); macro_call!(); }",
            "Trait",
        ),
        ("pub trait Alias = Clone + Send;", "TraitAlias"),
        (
            "impl<T> Service<T> for Named<T> { const LIMIT: usize = 2; type Output = T; fn call(&self, value: T) { value; } macro_call!(); }",
            "Impl",
        ),
        ("type AliasType<T> = Option<T>;", "TypeAlias"),
        ("const CONST_VALUE: usize = 4;", "Const"),
        ("static mut STATIC_VALUE: usize = 5;", "Static"),
        ("macro_call!(tokens);", "Macro"),
        ("macro_rules! local_macro { () => {}; }", "Macro2"),
        (
            "extern \"C\" { static FOREIGN: u8; type ForeignType; fn foreign(value: i32) -> i32; macro_call!(); }",
            "ForeignMod",
        ),
    ] {
        let item: Item = moxy_token::parse!(source).unwrap();
        let value = serde_json::to_value(&item).unwrap();
        assert!(value.get(tag).is_some(), "missing {tag} for {source}: {value}");
    }

    for (source, tag) in [
        ("let mut value: usize = 1;", "Local"),
        ("{ work(); }", "Block"),
        ("const VALUE: usize = 1;", "Item"),
        ("work();", "Expr"),
        ("macro_call!();", "Item"),
    ] {
        let statement: Stmt = moxy_token::parse!(source).unwrap();
        let value = serde_json::to_value(&statement).unwrap();
        assert!(value.get(tag).is_some(), "missing {tag} for {source}: {value}");
    }

    let statement = moxy_token::parse!("macro_call!();" as moxy_ast::stmt::StmtMacro)
        .unwrap()
        .into_stmt();
    let value = serde_json::to_value(statement).unwrap();
    assert!(value.get("Macro").is_some(), "missing Macro in {value}");
}

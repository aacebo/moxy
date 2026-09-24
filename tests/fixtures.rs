use moxy::ast::{Expr, Item, Type};

const ATTRIBUTES_DERIVES: &str = include_str!("../fixtures/parse/item/attributes_derives.rs");
const MIXED_ITEMS: &str = include_str!("../fixtures/parse/item/mixed_items.rs");
const LARGE_ITEMS: &str = include_str!("../fixtures/parse/item/large_items.rs");
const MACRO_HEAVY: &str = include_str!("../fixtures/parse/item/macro_heavy.rs");

const CONTROL_FLOW_EXPR: &str = include_str!("../fixtures/parse/expr/control_flow.rs");

const GENERIC_DEPTH_8: &str = include_str!("../fixtures/parse/type/generic_depth_8.rs");
const GENERIC_DEPTH_32: &str = include_str!("../fixtures/parse/type/generic_depth_32.rs");
const GENERIC_DEPTH_128: &str = include_str!("../fixtures/parse/type/generic_depth_128.rs");

const INVALID_DEEP_GENERIC: &str = include_str!("../fixtures/parse/invalid/deep_generic.rs");
const INVALID_MACRO: &str = include_str!("../fixtures/parse/invalid/macro.rs");
const INVALID_LARGE_FILE_TAIL: &str = include_str!("../fixtures/parse/invalid/large_file_tail.rs");

#[test]
fn attributes_derives_fixture_parses_as_items() {
    let items: Vec<Item> = moxy::parse!(ATTRIBUTES_DERIVES).unwrap();
    assert_eq!(items.len(), 9);
}

#[test]
fn mixed_items_fixture_parses_as_items() {
    let items: Vec<Item> = moxy::parse!(MIXED_ITEMS).unwrap();
    assert_eq!(items.len(), 6);
}

#[test]
fn large_items_fixture_parses_as_items() {
    let items: Vec<Item> = moxy::parse!(LARGE_ITEMS).unwrap();
    assert_eq!(items.len(), 25);
}

#[test]
fn macro_heavy_fixture_parses_as_items() {
    let items: Vec<Item> = moxy::parse!(MACRO_HEAVY).unwrap();
    assert_eq!(items.len(), 13);
}

#[test]
fn control_flow_expression_fixture_parses_as_expression() {
    let _: Expr = moxy::parse!(CONTROL_FLOW_EXPR).unwrap();
}

#[test]
fn generic_depth_8_fixture_parses_as_type() {
    let _: Type = moxy::parse!(GENERIC_DEPTH_8).unwrap();
}

#[test]
fn generic_depth_32_fixture_parses_as_type() {
    let _: Type = moxy::parse!(GENERIC_DEPTH_32).unwrap();
}

#[test]
fn generic_depth_128_fixture_parses_as_type() {
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            let _: Type = moxy::parse!(GENERIC_DEPTH_128).unwrap();
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn invalid_deep_generic_fixture_is_rejected_as_items() {
    let result: Result<Vec<Item>, _> = moxy::parse!(INVALID_DEEP_GENERIC);
    assert!(result.is_err());
}

#[test]
fn invalid_macro_fixture_is_rejected_as_items() {
    let result: Result<Vec<Item>, _> = moxy::parse!(INVALID_MACRO);
    assert!(result.is_err());
}

#[test]
fn invalid_large_file_tail_fixture_is_rejected_as_items() {
    let result: Result<Vec<Item>, _> = moxy::parse!(INVALID_LARGE_FILE_TAIL);
    assert!(result.is_err());
}

#[test]
fn malformed_expression_fixture_is_rejected_as_expression() {
    let result: Result<Expr, _> = moxy::parse!("if ready");
    assert!(result.is_err());
}

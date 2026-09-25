use std::{collections::BTreeSet, fs, path::Path};

use moxy::ast::{Expr, Item, Pattern, Stmt, Type};
use moxy::token::{Lit, ToTokenStream};
use moxy_ast::File;

const ATTRIBUTES_DERIVES: &str = include_str!("../fixtures/parse/item/attributes_derives.rs");
const MIXED_ITEMS: &str = include_str!("../fixtures/parse/item/mixed_items.rs");
const LARGE_ITEMS: &str = include_str!("../fixtures/parse/item/large_items.rs");
const MACRO_HEAVY: &str = include_str!("../fixtures/parse/item/macro_heavy.rs");
const DECLARATION_HEAVY: &str = include_str!("../fixtures/parse/item/declaration_heavy.rs");
const PATTERN_HEAVY: &str = include_str!("../fixtures/parse/item/pattern_heavy.rs");

const CONTROL_FLOW_EXPR: &str = include_str!("../fixtures/parse/expr/control_flow.rs");
const POSTFIX_ASYNC_EXPR: &str = include_str!("../fixtures/parse/expr/postfix_async.rs");

const GENERIC_DEPTH_8: &str = include_str!("../fixtures/parse/type/generic_depth_8.rs");
const GENERIC_DEPTH_32: &str = include_str!("../fixtures/parse/type/generic_depth_32.rs");
const GENERIC_DEPTH_128: &str = include_str!("../fixtures/parse/type/generic_depth_128.rs");
const COMPLEX_BOUNDS: &str = include_str!("../fixtures/parse/type/complex_bounds.rs");

const INVALID_DEEP_GENERIC: &str = include_str!("../fixtures/parse/invalid/deep_generic.rs");
const INVALID_MACRO: &str = include_str!("../fixtures/parse/invalid/macro.rs");
const INVALID_LARGE_FILE_TAIL: &str = include_str!("../fixtures/parse/invalid/large_file_tail.rs");
const INVALID_MALFORMED_DECLARATION: &str = include_str!("../fixtures/parse/invalid/malformed_declaration.rs");

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
fn declaration_heavy_fixture_parses_as_items() {
    let _: syn::File = syn::parse_file(DECLARATION_HEAVY).unwrap();
    let items: Vec<Item> = moxy::parse!(DECLARATION_HEAVY).unwrap();
    assert_eq!(items.len(), 3);
}

#[test]
fn pattern_heavy_fixture_parses_as_items() {
    let _: syn::File = syn::parse_file(PATTERN_HEAVY).unwrap();
    let items: Vec<Item> = moxy::parse!(PATTERN_HEAVY).unwrap();
    assert_eq!(items.len(), 1);
}

#[test]
fn control_flow_expression_fixture_parses_as_expression() {
    let _: Expr = moxy::parse!(CONTROL_FLOW_EXPR).unwrap();
}

#[test]
fn postfix_async_expression_fixture_parses_as_expression() {
    let _: syn::Expr = syn::parse_str(POSTFIX_ASYNC_EXPR).unwrap();
    let _: Expr = moxy::parse!(POSTFIX_ASYNC_EXPR).unwrap();
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
fn complex_bounds_fixture_parses_as_type() {
    let _: syn::Type = syn::parse_str(COMPLEX_BOUNDS).unwrap();
    let _: Type = moxy::parse!(COMPLEX_BOUNDS).unwrap();
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
fn malformed_declaration_fixture_is_rejected_as_items() {
    assert!(syn::parse_file(INVALID_MALFORMED_DECLARATION).is_err());
    let result: Result<Vec<Item>, _> = moxy::parse!(INVALID_MALFORMED_DECLARATION);
    assert!(result.is_err());
}

#[test]
fn malformed_expression_fixture_is_rejected_as_expression() {
    let result: Result<Expr, _> = moxy::parse!("if ready");
    assert!(result.is_err());
}

#[derive(serde::Deserialize)]
struct GrammarMatrix {
    entries: Vec<GrammarEntry>,
}

#[derive(serde::Deserialize)]
struct GrammarEntry {
    id: String,
    reference: GrammarReference,
    baseline: GrammarBaseline,
    upstream: GrammarUpstream,
    parse_root: String,
    support_expectation: String,
    fixture_directory: String,
}

#[derive(serde::Deserialize)]
struct GrammarReference {
    production: String,
    anchor: String,
}

#[derive(serde::Deserialize)]
struct GrammarBaseline {
    toolchain: String,
    edition: String,
}

#[derive(serde::Deserialize)]
struct GrammarUpstream {
    repository: String,
    commit: String,
    paths: Vec<String>,
}

fn parse_moxy(root: &str, source: &str) -> bool {
    match root {
        "crate" => moxy::parse!(source as File).is_ok(),
        "item" => moxy::parse!(source as Item).is_ok(),
        "expr" => moxy::parse!(source as Expr).is_ok(),
        "type" => moxy::parse!(source as Type).is_ok(),
        "pattern" => moxy::parse!(source as Pattern).is_ok(),
        "statement" => moxy::parse!(source as Stmt).is_ok(),
        "literal" => moxy::parse!(source as Lit).is_ok(),
        other => panic!("unknown grammar parse root `{other}`"),
    }
}

fn grammar_fixture_directories(root: &Path) -> BTreeSet<String> {
    let mut directories = BTreeSet::new();

    for family in fs::read_dir(root).unwrap() {
        let family = family.unwrap().path();

        if !family.is_dir() {
            continue;
        }

        for form in fs::read_dir(&family).unwrap() {
            let form = form.unwrap().path();

            if form.is_dir() {
                directories.insert(form.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap().display().to_string());
            }
        }
    }

    directories
}

#[test]
fn stable_rust_grammar_matrix_is_complete_and_conformant() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"));
    let matrix_path = workspace.join("fixtures/grammar/matrix.json");
    let matrix: GrammarMatrix = serde_json::from_str(&fs::read_to_string(matrix_path).unwrap()).unwrap();
    let mut ids = BTreeSet::new();
    let mut tracked = BTreeSet::new();
    let mut unsupported = Vec::new();

    for entry in &matrix.entries {
        assert!(ids.insert(&entry.id), "duplicate grammar matrix ID: {}", entry.id);
        assert!(
            !entry.reference.production.is_empty() && !entry.reference.anchor.is_empty(),
            "{} lacks Rust Reference provenance",
            entry.id
        );
        assert_eq!(
            entry.baseline.toolchain, "stable",
            "{} has the wrong toolchain baseline",
            entry.id
        );
        assert_eq!(entry.baseline.edition, "2024", "{} has the wrong edition baseline", entry.id);
        assert_eq!(
            entry.upstream.repository, "rust-lang/rust",
            "{} has the wrong upstream repository",
            entry.id
        );
        assert!(
            !entry.upstream.commit.is_empty(),
            "{} lacks pinned rust-lang/rust commit",
            entry.id
        );
        assert!(
            !entry.upstream.paths.is_empty(),
            "{} lacks rust-lang/rust UI provenance",
            entry.id
        );
        assert!(
            matches!(
                entry.support_expectation.as_str(),
                "moxy_accepts" | "moxy_rejects" | "extension"
            ),
            "{} has an unknown support expectation",
            entry.id
        );

        let directory = workspace.join(&entry.fixture_directory);
        assert!(
            tracked.insert(entry.fixture_directory.clone()),
            "duplicate grammar fixture directory: {}",
            entry.fixture_directory
        );

        let pass = fs::read_to_string(directory.join("pass.rs")).unwrap_or_else(|_| panic!("{} is missing pass.rs", entry.id));
        let _fail = fs::read_to_string(directory.join("fail.rs")).unwrap_or_else(|_| panic!("{} is missing fail.rs", entry.id));
        let context = format!(
            "{} ({}, rust-lang/rust@{}:{})",
            entry.id,
            entry.reference.production,
            entry.upstream.commit,
            entry.upstream.paths.join(", ")
        );

        let pass_result = parse_moxy(&entry.parse_root, &pass);

        match entry.support_expectation.as_str() {
            "moxy_accepts" | "extension" => {
                if !pass_result {
                    unsupported.push(context);
                }
            }
            "moxy_rejects" => {
                assert!(!pass_result, "moxy unexpectedly accepts unsupported syntax: {context}");
            }
            _ => unreachable!(),
        }
    }

    let discovered = grammar_fixture_directories(&workspace.join("fixtures/grammar"));
    assert_eq!(tracked, discovered, "untracked or missing grammar fixture directory");
    assert!(
        unsupported.is_empty(),
        "moxy does not yet support stable syntax:\n{}",
        unsupported.join("\n")
    );
}

#[test]
fn audited_stable_forms_parse_and_round_trip() {
    for source in [
        "#[unsafe(no_mangle)] fn f() {}",
        "safe fn f();",
        "safe static X: u8;",
        "const _: u8 = 0;",
        "const X: u8;",
        "static X: u8;",
        "type X: Send = u8;",
        "type X where Self: Sized;",
        "enum E { pub A }",
    ] {
        let item: Item = moxy::parse!(source).unwrap();
        let emitted = item.to_token_stream().to_string();
        assert!(moxy::parse!(emitted as Item).is_ok());
    }

    for source in ["fn f(u8);", "fn f((x, y): (u8, u8));"] {
        let item: Item = moxy::parse!(source).unwrap();
        let emitted = item.to_token_stream().to_string();
        assert!(moxy::parse!(emitted as Item).is_ok());
    }

    for source in ["impl Sized + use<>", "impl Sized + use<Self>", "Trait + Send"] {
        let ty: Type = moxy::parse!(source).unwrap();
        let emitted = ty.to_token_stream().to_string();
        assert!(moxy::parse!(emitted as Type).is_ok());
    }

    let pat: Pattern = moxy::parse!("a ... b").unwrap();
    let emitted = pat.to_token_stream().to_string();
    assert!(moxy::parse!(emitted as Pattern).is_ok());
    let stmt: Stmt = moxy::parse!(";").unwrap();
    let emitted = stmt.to_token_stream().to_string();
    assert!(moxy::parse!(emitted as Stmt).is_ok());
}

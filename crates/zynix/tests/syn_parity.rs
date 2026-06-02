//! Differential parity tests: for a corpus of syn-supported snippets, assert
//! zynix parses what syn parses and that our re-emitted tokens re-parse.
//!
//! Run with: `cargo test -p zynix --features ast,proc-macro2 --test syn_parity`
#![cfg(all(feature = "ast", feature = "proc-macro2"))]

use std::str::FromStr;

use zynix::token::ToTokenStream;
use zynix::{Parse, TokenStream, ast};

/// Parse `src` into our `T`, returning Err on lex or parse failure.
fn zynix_parse<T: Parse>(src: &str) -> Result<T, String> {
    let ts = TokenStream::from_str(src).map_err(|e| format!("lex: {e:?}"))?;
    let mut ps = ts.parse();
    ps.parse::<T>().map_err(|e| format!("parse: {e:?}"))
}

/// Assert: syn parses `src` as `Syn`, we parse it as `Our`, and our re-emitted
/// tokens re-parse as `Our` (round-trip stable).
fn check<Syn, Our>(src: &str)
where
    Syn: syn::parse::Parse,
    Our: Parse + ToTokenStream,
{
    let syn_ok = syn::parse_str::<Syn>(src).is_ok();
    assert!(syn_ok, "corpus snippet not valid syn: {src:?}");

    let parsed: Our = zynix_parse(src).unwrap_or_else(|e| panic!("zynix failed on {src:?}: {e}"));

    // Re-emit and re-parse for round-trip stability.
    let reemitted = parsed.to_token_stream().to_string();
    zynix_parse::<Our>(&reemitted).unwrap_or_else(|e| panic!("zynix re-parse failed for {src:?} -> {reemitted:?}: {e}"));
}

#[test]
fn exprs() {
    for src in [
        "1 + 2 * 3",
        "a - b - c",
        "0..10",
        "0..=10",
        "a..",
        "..b",
        "..",
        "x as u32",
        "x = y",
        "x += 1",
        "f(x, y)",
        "a.b.c",
        "a.method()",
        "x.collect::<Vec<u8>>()",
        "a[0]",
        "x?",
        "x.await",
        "&x",
        "&mut x",
        "-x",
        "!flag",
        "(a, b)",
        "[1, 2, 3]",
        "[0; 4]",
        "Foo { a: 1, b }",
        "if a { b } else { c }",
        "if let Some(x) = o { x } else { 0 }",
        "while let Some(x) = it.next() { }",
        "match x { 1 => a, _ => b }",
        "loop { break }",
        "'a: loop { break 'a 1 }",
        "for x in xs { }",
        "unsafe { 1 }",
        "async { 1 }",
        "async move { x }",
        "const { 1 }",
        "|x| x + 1",
        "move |x: u8| -> u8 { x }",
        "async || 1",
        "return",
        "return x",
        "vec![1, 2, 3]",
        "<T as Trait>::CONST",
        "::std::mem::swap",
    ] {
        check::<syn::Expr, ast::Expr>(src);
    }
}

#[test]
fn types() {
    for src in [
        "u8",
        "Vec<T>",
        "Vec<Box<T>>",
        "HashMap<K, V>",
        "&'a T",
        "&mut T",
        "*const T",
        "*mut T",
        "(A, B)",
        "(T,)",
        "[T]",
        "[u8; 4]",
        "!",
        "_",
        "impl Clone + Send",
        "dyn Fn(u8) -> bool",
        "fn(u8) -> bool",
        "<T as Trait>::Item",
        "Iterator<Item = u8>",
        "std::collections::HashMap<String, Vec<u8>>",
    ] {
        check::<syn::Type, ast::Type>(src);
    }
}

#[test]
fn patterns() {
    for src in [
        "x",
        "_",
        "mut x",
        "ref x",
        "&x",
        "(a, b)",
        "Some(x)",
        "Point { x, y }",
        "Point { x, .. }",
        "[a, b, c]",
        "1",
        "A | B | C",
        "x @ 1",
    ] {
        // `syn::Pat` isn't `Parse`; use the multi-pattern entry (allows or-patterns).
        let syn_ok = syn::parse::Parser::parse_str(syn::Pat::parse_multi_with_leading_vert, src).is_ok();
        assert!(syn_ok, "corpus snippet not valid syn pattern: {src:?}");

        let parsed: ast::Pattern = zynix_parse(src).unwrap_or_else(|e| panic!("zynix failed on pat {src:?}: {e}"));
        let reemitted = parsed.to_token_stream().to_string();
        zynix_parse::<ast::Pattern>(&reemitted)
            .unwrap_or_else(|e| panic!("zynix re-parse failed for pat {src:?} -> {reemitted:?}: {e}"));
    }
}

#[test]
fn items() {
    for src in [
        "fn f() {}",
        "fn f<T: Clone>(x: T) -> T where T: Send { x }",
        "pub fn g() {}",
        "pub(crate) fn h() {}",
        "unsafe fn u() {}",
        "const fn c() -> u8 { 0 }",
        "struct S;",
        "struct T(u8, u16);",
        "struct U { a: u8, b: u16 }",
        "pub struct G<T> { v: T }",
        "enum E { A, B(u8), C { x: i32 } }",
        "trait Tr { fn m(&self); type Out; const N: u8; }",
        "unsafe trait UT {}",
        "auto trait AT {}",
        "impl S {}",
        "impl<T> Tr for S<T> { fn m(&self) {} }",
        "impl !Send for S {}",
        "type Alias = u8;",
        "const X: u8 = 1;",
        "static Y: u8 = 2;",
        "mod m { fn a() {} }",
        "use a::{b, c as d, e::*};",
        "pub use x::y;",
        "extern crate foo;",
        "macro_rules! m { () => {} }",
    ] {
        check::<syn::Item, ast::Item>(src);
    }
}

#[test]
fn stmts() {
    for src in [
        "let x = 1;",
        "let x: u8 = 1;",
        "let x;",
        "let Some(y) = o else { return; };",
        "foo();",
    ] {
        check::<syn::Stmt, ast::Stmt>(src);
    }
}

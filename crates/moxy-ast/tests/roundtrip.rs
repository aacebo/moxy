use std::str::FromStr;

use moxy_token::TokenStream;

#[test]
fn raw_idents() {
    for src in ["r#foo", "r#fn"] {
        let once = src.parse::<TokenStream>().unwrap().to_string();
        assert!(
            TokenStream::from_str(&once).is_ok(),
            "round-trip of {src:?} did not re-lex: {once:?}"
        );
    }
}

#[test]
fn keywords() {
    for src in [
        "struct S;",
        "impl S { pub fn make() -> Self { Self } }",
        "let x = 3; match x { 3 => x, _ => 0 }",
    ] {
        let once = src.parse::<TokenStream>().unwrap().to_string();
        assert!(
            TokenStream::from_str(&once).is_ok(),
            "round-trip of {src:?} did not re-lex: {once:?}"
        );
    }
}

#[test]
fn lifetimes() {
    for src in ["&'a [i32]", "fn takes<'a>(x: &'a str) {}", "'static"] {
        let once = src.parse::<TokenStream>().unwrap().to_string();
        assert!(
            TokenStream::from_str(&once).is_ok(),
            "round-trip of {src:?} did not re-lex: {once:?}"
        );
    }
}

#[test]
fn literals() {
    for src in [
        "1u8",
        "1.5f64",
        "0xffu16",
        "0b1010",
        "1_000usize",
        "b'x'",
        "'c'",
        "\"s\"",
        "b\"x\"",
        "c\"x\"",
        "r#\"raw\"#",
    ] {
        let once = src.parse::<TokenStream>().unwrap().to_string();
        assert!(
            TokenStream::from_str(&once).is_ok(),
            "round-trip of {src:?} did not re-lex: {once:?}"
        );
    }
}

#[test]
fn groups_and_multitoken() {
    for src in ["(1 + (2 * 3))", "vec![1, 2, 3]", "1 == 1 && 2 <= 3"] {
        let once = src.parse::<TokenStream>().unwrap().to_string();
        assert!(
            TokenStream::from_str(&once).is_ok(),
            "round-trip of {src:?} did not re-lex: {once:?}"
        );
    }
}

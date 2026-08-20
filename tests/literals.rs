use moxy::ast::Expr;
use moxy::token::{Lit, Spanner, ToTokenStream};

#[test]
fn every_rust_literal_family_preserves_its_representation() {
    for (source, expected_kind) in [
        ("42", 0),
        ("42u64", 1),
        ("3.5f32", 2),
        (r#""hello\\nworld""#, 3),
        (r#"b"bytes""#, 4),
        (r#"c"name""#, 5),
        (r#"'x'"#, 6),
        (r#"b'x'"#, 7),
        ("true", 8),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        let literal = &expression
            .as_primary()
            .unwrap()
            .as_lit()
            .unwrap_or_else(|| panic!("{source} did not parse as a literal expression"))
            .lit;
        assert_eq!(
            [
                matches!(literal, Lit::Int(_)),
                matches!(literal, Lit::UInt(_)),
                matches!(literal, Lit::Float(_)),
                matches!(literal, Lit::Str(_)),
                matches!(literal, Lit::ByteStr(_)),
                matches!(literal, Lit::CStr(_)),
                matches!(literal, Lit::Char(_)),
                matches!(literal, Lit::Byte(_)),
                matches!(literal, Lit::Bool(_)),
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert_eq!(literal.repr(), source);
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), source);
    }
}

#[test]
#[ignore = "lexer currently rejects unsuffixed integers wider than u64"]
fn large_unsuffixed_integer_literals_survive_the_syntax_pipeline() {
    let source = "340282366920938463463374607431768211455";
    let expression: Expr = moxy::parse!(source).unwrap();
    let literal = &expression.as_primary().unwrap().as_lit().unwrap().lit;
    assert!(matches!(literal, Lit::UInt(_)));
    assert_eq!(literal.repr(), source);
    assert_eq!(moxy::fmt!(&expression).unwrap(), source);
}

#[test]
#[ignore = "identifier lookahead currently panics while lexing a non-ASCII character literal"]
fn unicode_character_literals_complete_the_syntax_pipeline() {
    let expression: Expr = moxy::parse!("'λ'").unwrap();
    let literal = &expression.as_primary().unwrap().as_lit().unwrap().lit;
    assert!(matches!(literal, Lit::Char(_)));
    assert_eq!(literal.repr(), "'λ'");
    assert_eq!(moxy::fmt!(&expression).unwrap(), "'λ'");
}

#[test]
#[ignore = "high-byte escape currently parses as a path expression instead of a byte literal"]
fn high_byte_escapes_complete_the_syntax_pipeline() {
    let expression: Expr = moxy::parse!(r#"b'\xFF'"#).unwrap();
    let literal = &expression.as_primary().unwrap().as_lit().unwrap().lit;
    assert!(matches!(literal, Lit::Byte(_)));
    assert_eq!(literal.repr(), r#"b'\xFF'"#);
    assert_eq!(moxy::fmt!(&expression).unwrap(), r#"b'\xFF'"#);
}

#[test]
fn boolean_literals_bridge_as_values_in_expressions() {
    for source in ["true", "false"] {
        let expression: Expr = moxy::parse!(source).unwrap();
        let literal = &expression.as_primary().unwrap().as_lit().unwrap().lit;
        assert!(matches!(literal, Lit::Bool(_)));
        assert_eq!(literal.repr(), source);
        assert_eq!(moxy::fmt!(&expression).unwrap(), source);
    }
}

#[test]
fn numeric_literal_suffixes_match_their_public_syntax_values() {
    for expected in [
        Lit::u8_suffixed(8),
        Lit::u8_unsuffixed(8),
        Lit::u16_suffixed(16),
        Lit::u16_unsuffixed(16),
        Lit::u32_suffixed(32),
        Lit::u32_unsuffixed(32),
        Lit::u64_suffixed(64),
        Lit::u64_unsuffixed(64),
        Lit::usize_suffixed(128),
        Lit::usize_unsuffixed(128),
        Lit::i8_suffixed(8),
        Lit::i8_unsuffixed(8),
        Lit::i16_suffixed(16),
        Lit::i16_unsuffixed(16),
        Lit::i32_suffixed(32),
        Lit::i32_unsuffixed(32),
        Lit::i64_suffixed(64),
        Lit::i64_unsuffixed(64),
        Lit::isize_suffixed(128),
        Lit::isize_unsuffixed(128),
        Lit::f32_suffixed(1.5),
        Lit::f32_unsuffixed(1.5),
        Lit::f64_suffixed(2.5),
        Lit::f64_unsuffixed(2.5),
    ] {
        let source = expected.repr();
        let expression: Expr = moxy::parse!(source).unwrap();
        let literal = &expression.as_primary().unwrap().as_lit().unwrap().lit;
        assert_eq!(literal.repr(), source);
        assert_eq!(expected.to_token_stream().to_string(), source);
        assert!(expected.to_token_tree().is_literal());
        assert!(expected.clone().into_token_tree().as_literal().is_some());
        assert_eq!(expected.is_int(), !expected.is_float());
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), source);
    }
}

#[test]
fn unsuffixed_numeric_literals_match_their_public_syntax_values() {
    for (source, expected) in [
        ("1", Lit::i32_unsuffixed(1)),
        ("256", Lit::i32_unsuffixed(256)),
        ("65536", Lit::i32_unsuffixed(65_536)),
        ("1.5", Lit::f64_unsuffixed(1.5)),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        let literal = &expression.as_primary().unwrap().as_lit().unwrap().lit;
        assert_eq!(literal, &expected);
        assert_eq!(literal.repr(), source);
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), source);
    }
}

#[test]
fn cooked_raw_string_character_and_byte_syntax_decode_to_real_values() {
    for (source, expected) in [
        (r#""line\nnext""#, "line\nnext"),
        (r##"r"raw\n""##, "raw\\n"),
        (r#"'\t'"#, "\t"),
        (r#"b'\x41'"#, "A"),
        (r#"b"bytes""#, "bytes"),
        (r#"c"ffi""#, "ffi"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        let literal = &expression.as_primary().unwrap().as_lit().unwrap().lit;
        match literal {
            Lit::Str(value) => assert_eq!(value.value(), expected),
            Lit::Char(value) => assert_eq!(value.value().to_string(), expected),
            Lit::Byte(value) => assert_eq!((value.value() as char).to_string(), expected),
            Lit::ByteStr(value) => assert_eq!(value.value(), expected.as_bytes()),
            Lit::CStr(value) => assert_eq!(value.value(), expected.as_bytes()),
            _ => panic!("{source} parsed as the wrong literal family"),
        }
        assert_eq!(literal.repr(), source);
        assert!(!literal.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), source);
    }
}

#[test]
#[ignore = "equivalent integer values currently produce different hashes"]
fn equivalent_integer_spellings_compare_and_hash_by_value() {
    use std::hash::{DefaultHasher, Hash, Hasher};

    let decimal: Expr = moxy::parse!("1").unwrap();
    let padded: Expr = moxy::parse!("01").unwrap();
    let decimal = &decimal.as_primary().unwrap().as_lit().unwrap().lit;
    let padded = &padded.as_primary().unwrap().as_lit().unwrap().lit;
    assert_eq!(decimal, padded);
    let mut decimal_hash = DefaultHasher::new();
    decimal.hash(&mut decimal_hash);
    let mut padded_hash = DefaultHasher::new();
    padded.hash(&mut padded_hash);
    assert_eq!(decimal_hash.finish(), padded_hash.finish());
    assert_eq!(moxy::fmt!(&moxy::parse!("1" as Expr).unwrap()).unwrap(), "1");
    assert_eq!(moxy::fmt!(&moxy::parse!("01" as Expr).unwrap()).unwrap(), "01");
}

#[cfg(feature = "proc-macro2")]
#[test]
#[ignore = "proc-macro bridge currently classifies booleans as identifiers"]
fn proc_macro_boolean_literals_complete_the_syntax_pipeline() {
    use std::str::FromStr;

    for source in ["true", "false"] {
        let proc_tokens = proc_macro2::TokenStream::from_str(source).unwrap();
        let owned = moxy::token::TokenStream::from(proc_tokens);
        let expression: Expr = moxy::parse!(owned).unwrap();
        let literal = &expression.as_primary().unwrap().as_lit().unwrap().lit;
        assert!(matches!(literal, Lit::Bool(_)));
        assert_eq!(literal.repr(), source);
        assert_eq!(moxy::fmt!(&expression).unwrap(), source);
    }
}

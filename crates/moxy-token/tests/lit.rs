use moxy_token::TokenStream;

#[test]
fn accept() {
    macro_rules! scan {
        ($($check:ident => $source:literal),+ $(,)?) => {
            $(
                let tokens: TokenStream = $source.parse().expect(concat!("literal ", $source, " could not be scanned"));
                let mut iter = tokens.into_iter();
                let token = iter.next().expect(concat!("no tokens scanned from ", $source));
                let token = token.as_literal().expect("expected literal");
                assert!(token.$check());
            )+
        };
    }

    scan! {
        is_int => "42u8",
        is_float => "3.5",
        is_float => "3.5f32",
        is_float => "3.5f64",
        is_str => r#""text""#,
        is_byte_str => r#"b"text""#,
        is_c_str => r#"c"text""#,
        is_char => "'x'",
        is_byte => "b'x'",
        is_bool => "true",
        is_bool => "false",
    }
}

#[test]
fn reject() {
    let cases = ["test", "dyn", ",", "|"];

    for case in cases {
        let tokens: TokenStream = case.parse().expect("invalid");
        let mut iter = tokens.into_iter();
        let token = iter.next().expect("no tokens scanned");
        assert!(!token.is_literal());
    }
}

#[test]
fn string_line_continuations_skip_the_newline_and_indentation() {
    let tokens: TokenStream = "\"first \\\n    second\" 'x' b\"one \\\r\n\ttwo\" c\"a \\\n b\""
        .parse()
        .unwrap();
    let literals: Vec<moxy_token::Lit> = tokens
        .iter()
        .map(|token| token.as_literal().expect("expected a literal").clone())
        .collect();

    assert_eq!(literals.len(), 4);
    let moxy_token::Lit::Str(text) = &literals[0] else {
        panic!("expected a string");
    };
    assert_eq!(text.value(), "first second");
    let moxy_token::Lit::ByteStr(bytes) = &literals[2] else {
        panic!("expected a byte string");
    };
    assert_eq!(bytes.value(), b"one two");
    let moxy_token::Lit::CStr(c_text) = &literals[3] else {
        panic!("expected a C string");
    };
    assert_eq!(c_text.value(), b"a b");
}

#[test]
fn suffixed_integer_constructors_keep_the_suffix() {
    for (literal, text) in [
        (moxy_token::Lit::u8_suffixed(7), "7u8"),
        (moxy_token::Lit::u64_suffixed(10923128598635462381), "10923128598635462381u64"),
        (moxy_token::Lit::usize_suffixed(3), "3usize"),
        (moxy_token::Lit::i32_suffixed(5), "5i32"),
        (moxy_token::Lit::u64_unsuffixed(9), "9"),
    ] {
        assert_eq!(literal.to_string(), text);
    }
}

#[test]
fn a_trailing_dot_ends_a_float_literal() {
    for (source, tokens) in [
        ("3.", vec!["3."]),
        ("(x / 3.).floor()", vec!["(x / 3.)", ".", "floor", "()"]),
        ("[1., 2.]", vec!["[1. , 2.]"]),
        ("1..2", vec!["1", ".", ".", "2"]),
        ("3.max(1)", vec!["3", ".", "max", "(1)"]),
        ("x.0._a", vec!["x", ".", "0", ".", "_a"]),
    ] {
        let stream: TokenStream = source.parse().unwrap();
        let texts: Vec<String> = stream.into_iter().map(|token| token.to_string()).collect();
        assert_eq!(texts, tokens, "`{source}`");
    }

    let stream: TokenStream = "3.".parse().unwrap();
    let token = stream.into_iter().next().unwrap();
    assert!(token.as_literal().unwrap().is_float());
}

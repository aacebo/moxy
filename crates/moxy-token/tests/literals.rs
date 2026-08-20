use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use moxy_token::{
    Lit, LitBool, LitByte, LitByteStr, LitCStr, LitChar, LitF32, LitF64, LitFloat, LitI8, LitI16, LitI32, LitI64, LitISize,
    LitInt, LitStr, LitU8, LitU16, LitU32, LitU64, LitUInt, LitUSize, Parse, Span, Spanner, ToTokenStream, TokenStream,
};

#[test]
fn constructors_cover_every_numeric_width() {
    let cases = [
        (Lit::u8_suffixed(8), "8u8", true),
        (Lit::u8_unsuffixed(8), "8", true),
        (Lit::u16_suffixed(16), "16u16", true),
        (Lit::u16_unsuffixed(16), "16", true),
        (Lit::u32_suffixed(32), "32u32", true),
        (Lit::u32_unsuffixed(32), "32", true),
        (Lit::u64_suffixed(64), "64u64", true),
        (Lit::u64_unsuffixed(64), "64", true),
        (Lit::usize_suffixed(128), "128usize", true),
        (Lit::usize_unsuffixed(128), "128", true),
        (Lit::i8_suffixed(8), "8i8", true),
        (Lit::i8_unsuffixed(8), "8", true),
        (Lit::i16_suffixed(16), "16i16", true),
        (Lit::i16_unsuffixed(16), "16", true),
        (Lit::i32_suffixed(32), "32i32", true),
        (Lit::i32_unsuffixed(32), "32", true),
        (Lit::i64_suffixed(64), "64i64", true),
        (Lit::i64_unsuffixed(64), "64", true),
        (Lit::isize_suffixed(128), "128isize", true),
        (Lit::isize_unsuffixed(128), "128", true),
        (Lit::f32_suffixed(1.5), "1.5f32", false),
        (Lit::f32_unsuffixed(1.5), "1.5", false),
        (Lit::f64_suffixed(2.5), "2.5f64", false),
        (Lit::f64_unsuffixed(2.5), "2.5", false),
    ];

    for (lit, repr, integer) in cases {
        assert_eq!(lit.repr(), repr);
        assert_eq!(lit.is_int(), integer);
        assert_eq!(lit.is_float(), !integer);
        assert_eq!(lit.to_token_stream().to_string(), repr);
        assert!(lit.to_token_tree().is_literal());
        assert!(lit.clone().into_token_tree().as_literal().is_some());
        assert_eq!(lit.span(), Spanner::span(&lit));
    }
}

#[test]
fn typed_numeric_literals_expose_values_suffixes_and_conversions() {
    let values = TokenStream::from_str("1i8 2i16 3i32 4i64 5isize 6u8 7u16 8u32 9u64 10usize 1.25f32 2.5f64").unwrap();
    let mut stream = values.parse();

    let i8_value = stream.parse::<LitI8>().unwrap();
    assert_eq!(i8_value.value(), 1);
    assert!(i8_value.suffixed());
    assert_eq!(i8_value.repr(), "1i8");
    let i16_value = stream.parse::<LitI16>().unwrap();
    assert_eq!(i16_value.value(), 2);
    assert!(i16_value.suffixed());
    let i32_value = stream.parse::<LitI32>().unwrap();
    assert_eq!(i32_value.value(), 3);
    let i64_value = stream.parse::<LitI64>().unwrap();
    assert_eq!(i64_value.value(), 4);
    let isize_value = stream.parse::<LitISize>().unwrap();
    assert_eq!(isize_value.value(), 5);
    let u8_value = stream.parse::<LitU8>().unwrap();
    assert_eq!(u8_value.value(), 6);
    let u16_value = stream.parse::<LitU16>().unwrap();
    assert_eq!(u16_value.value(), 7);
    let u32_value = stream.parse::<LitU32>().unwrap();
    assert_eq!(u32_value.value(), 8);
    let u64_value = stream.parse::<LitU64>().unwrap();
    assert_eq!(u64_value.value(), 9);
    let usize_value = stream.parse::<LitUSize>().unwrap();
    assert_eq!(usize_value.value(), 10);
    let f32_value = stream.parse::<LitF32>().unwrap();
    assert_eq!(f32_value.value(), 1.25);
    assert!(f32_value.suffixed());
    let f64_value = stream.parse::<LitF64>().unwrap();
    assert_eq!(f64_value.value(), 2.5);
    assert!(f64_value.suffixed());
    assert!(stream.is_empty());

    assert!(matches!(LitInt::from(i8_value), LitInt::I8(_)));
    assert!(matches!(LitInt::from(i16_value), LitInt::I16(_)));
    assert!(matches!(LitInt::from(i32_value), LitInt::I32(_)));
    assert!(matches!(LitInt::from(i64_value), LitInt::I64(_)));
    assert!(matches!(LitInt::from(isize_value), LitInt::ISize(_)));
    assert!(matches!(LitUInt::from(u8_value), LitUInt::U8(_)));
    assert!(matches!(LitUInt::from(u16_value), LitUInt::U16(_)));
    assert!(matches!(LitUInt::from(u32_value), LitUInt::U32(_)));
    assert!(matches!(LitUInt::from(u64_value), LitUInt::U64(_)));
    assert!(matches!(LitUInt::from(usize_value), LitUInt::USize(_)));
    assert!(matches!(LitFloat::from(f32_value), LitFloat::F32(_)));
    assert!(matches!(LitFloat::from(f64_value), LitFloat::F64(_)));
}

#[cfg(feature = "serde")]
#[test]
fn every_concrete_numeric_literal_exposes_real_value_span_display_hash_and_serde_output() {
    let span = Span::mixed_site();

    let mut value = LitI8::new(-8, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (-8, true, "-8i8", span, "-8i8".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "-8i8");
    assert_eq!(value, value.clone());
    assert_eq!(Lit::from(value.clone()).as_u64(), None);

    let mut value = LitI16::new(-16, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (-16, true, "-16i16", span, "-16i16".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "-16i16");
    assert_eq!(value, value.clone());

    let mut value = LitI32::new(-32, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (-32, true, "-32i32", span, "-32i32".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "-32i32");
    assert_eq!(value, value.clone());

    let mut value = LitI64::new(-64, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (-64, true, "-64i64", span, "-64i64".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "-64i64");
    assert_eq!(value, value.clone());

    let mut value = LitISize::new(-128, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (-128, true, "-128isize", span, "-128isize".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "-128isize");
    assert_eq!(value, value.clone());

    let mut value = LitU8::new(8, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (8, true, "8u8", span, "8u8".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "8u8");
    assert_eq!(value, value.clone());
    assert_eq!(Lit::from(value.clone()).as_u64(), Some(8));

    let mut value = LitU16::new(16, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (16, true, "16u16", span, "16u16".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "16u16");
    assert_eq!(value, value.clone());

    let mut value = LitU32::new(32, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (32, true, "32u32", span, "32u32".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "32u32");
    assert_eq!(value, value.clone());

    let mut value = LitU64::new(64, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (64, true, "64u64", span, "64u64".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "64u64");
    assert_eq!(value, value.clone());

    let mut value = LitUSize::new(128, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (128, true, "128usize", span, "128usize".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "128usize");
    assert_eq!(value, value.clone());

    let mut value = LitF32::new(1.25, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (1.25, true, "1.25f32", span, "1.25f32".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "1.25f32");
    assert_eq!(value, value.clone());

    let mut value = LitF64::new(2.5, true, Default::default());
    value.set_span(span);
    assert_eq!(
        (value.value(), value.suffixed(), value.repr(), value.span(), value.to_string()),
        (2.5, true, "2.5f64", span, "2.5f64".into())
    );
    assert_eq!(serde_json::to_value(value.clone()).unwrap(), "2.5f64");
    assert_eq!(value, value.clone());
}

#[test]
fn literal_enum_dispatches_every_family_to_real_values_and_mutated_spans() {
    let mut literals = [
        Lit::i32_suffixed(-1),
        Lit::u32_suffixed(2),
        Lit::f64_suffixed(3.5),
        Lit::Str(LitStr::new("text", Default::default())),
        Lit::parse(&mut TokenStream::from_str("b\"bytes\"").unwrap().parse()).unwrap(),
        Lit::parse(&mut TokenStream::from_str("c\"ffi\"").unwrap().parse()).unwrap(),
        Lit::Char(LitChar::new('x', Default::default())),
        Lit::parse(&mut TokenStream::from_str("b'y'").unwrap().parse()).unwrap(),
        Lit::Bool(LitBool::new(true, Default::default())),
        Lit::from_repr("@@@", Default::default()),
    ];
    let expected = [
        "-1i32",
        "2u32",
        "3.5f64",
        "\"text\"",
        "b\"bytes\"",
        "c\"ffi\"",
        "'x'",
        "b'y'",
        "true",
        "@@@",
    ];

    for (literal, expected) in literals.iter_mut().zip(expected) {
        literal.set_span(Span::mixed_site());
        assert_eq!(literal.span(), Span::mixed_site());
        assert_eq!(literal.repr(), expected);
        assert_eq!(literal.to_string(), expected);
        assert_eq!(literal.to_token_tree().as_literal().unwrap().repr(), expected);
        assert_eq!(literal.clone().into_token_tree().as_literal().unwrap().repr(), expected);
    }

    assert!(literals[0].is_int());
    assert!(literals[1].is_int());
    assert!(literals[2].is_float());
    assert!(literals[3].is_str());
    assert!(literals[8].is_bool());
    assert_eq!(literals[3].as_str().unwrap().value(), "text");
    assert!(literals[4].as_str().is_none());
    assert!(literals[8].as_bool().unwrap().value());
    assert!(literals[7].as_bool().is_none());
}

#[test]
fn text_and_character_literals_decode_and_roundtrip() {
    let stream =
        TokenStream::from_str("true false 'x' '\\n' b'x' \"a\\tb\" r#\"raw\\n\"# b\"bytes\" br#\"raw\"# c\"ffi\" cr#\"raw\"#")
            .unwrap();
    let mut parse = stream.parse();

    let yes = parse.parse::<LitBool>().unwrap();
    assert!(yes.value());
    assert_eq!(yes.repr(), "true");
    let no = parse.parse::<LitBool>().unwrap();
    assert!(!no.value());
    let character = parse.parse::<LitChar>().unwrap();
    assert_eq!(character.value(), 'x');
    assert_eq!(parse.parse::<LitChar>().unwrap().value(), '\n');
    assert_eq!(parse.parse::<LitByte>().unwrap().value(), b'x');
    let string = parse.parse::<LitStr>().unwrap();
    assert_eq!(string.value(), "a\tb");
    assert_eq!(parse.parse::<LitStr>().unwrap().value(), "raw\\n");
    assert_eq!(parse.parse::<LitByteStr>().unwrap().value(), b"bytes");
    assert_eq!(parse.parse::<LitByteStr>().unwrap().value(), b"raw");
    assert_eq!(parse.parse::<LitCStr>().unwrap().value(), b"ffi");
    assert_eq!(parse.parse::<LitCStr>().unwrap().value(), b"raw");
    assert!(parse.is_empty());

    let made_string = Lit::string("hello");
    assert!(made_string.is_str());
    assert_eq!(made_string.as_str().unwrap().value(), "hello");
    let made_char = Lit::char('z');
    assert_eq!(made_char.repr(), "'z'");
    let made_bool = Lit::Bool(LitBool::new(true, Span::default()));
    assert!(made_bool.is_bool());
    assert!(made_bool.as_bool().unwrap().value());
}

#[test]
fn cooked_and_raw_literal_escapes_decode_to_exact_character_and_byte_output() {
    let string = TokenStream::from_str(r#""\n\r\t\\\'\"\0\x41\u{1F600}""#)
        .unwrap()
        .parse()
        .parse::<LitStr>()
        .unwrap();
    assert_eq!(string.value(), "\n\r\t\\\'\"\0A😀");
    assert_eq!(string.repr(), r#""\n\r\t\\\'\"\0\x41\u{1F600}""#);

    for (source, expected) in [
        (r"'\n'", '\n'),
        (r"'\r'", '\r'),
        (r"'\t'", '\t'),
        (r"'\\'", '\\'),
        (r"'\''", '\''),
        (r#"'\"'"#, '"'),
        (r"'\0'", '\0'),
        (r"'\x41'", 'A'),
        (r"'\u{1F600}'", '😀'),
    ] {
        let value = TokenStream::from_str(source).unwrap().parse().parse::<LitChar>().unwrap();
        assert_eq!(value.value(), expected, "wrong decoded char for {source}");
        assert_eq!(value.repr(), source);
    }

    for (source, expected) in [
        (r"b'\n'", b'\n'),
        (r"b'\r'", b'\r'),
        (r"b'\t'", b'\t'),
        (r"b'\\'", b'\\'),
        (r"b'\''", b'\''),
        (r#"b'\"'"#, b'"'),
        (r"b'\0'", b'\0'),
        (r"b'\x41'", b'A'),
    ] {
        let value = TokenStream::from_str(source).unwrap().parse().parse::<LitByte>().unwrap();
        assert_eq!(value.value(), expected, "wrong decoded byte for {source}");
        assert_eq!(value.repr(), source);
    }

    let bytes = TokenStream::from_str(r#"b"\n\r\t\\\'\"\0\x41""#)
        .unwrap()
        .parse()
        .parse::<LitByteStr>()
        .unwrap();
    assert_eq!(bytes.value(), b"\n\r\t\\'\"\0A");
    assert_eq!(bytes.repr(), r#"b"\n\r\t\\\'\"\0\x41""#);
    let c_string = TokenStream::from_str(r#"c"\n\r\t\\\'\"\0\x41""#)
        .unwrap()
        .parse()
        .parse::<LitCStr>()
        .unwrap();
    assert_eq!(c_string.value(), b"\n\r\t\\'\"\0A");
    assert_eq!(c_string.repr(), r#"c"\n\r\t\\\'\"\0\x41""#);

    for (source, expected) in [
        (r##"r"raw\n""##, "raw\\n"),
        (r###"r#"raw " quote"#"###, "raw \" quote"),
        (r####"r##"raw #" quote"##"####, "raw #\" quote"),
    ] {
        let value = TokenStream::from_str(source).unwrap().parse().parse::<LitStr>().unwrap();
        assert_eq!(value.value(), expected);
        assert_eq!(value.repr(), source);
    }
    let bytes = TokenStream::from_str(r###"br#"raw\n"#"###)
        .unwrap()
        .parse()
        .parse::<LitByteStr>()
        .unwrap();
    assert_eq!(bytes.value(), b"raw\\n");
    let c_string = TokenStream::from_str(r###"cr#"raw\n"#"###)
        .unwrap()
        .parse()
        .parse::<LitCStr>()
        .unwrap();
    assert_eq!(c_string.value(), b"raw\\n");
}

#[cfg(feature = "serde")]
#[test]
fn concrete_text_literals_preserve_value_repr_span_hash_display_and_serde_output() {
    let span = Span::mixed_site();
    let mut string = TokenStream::from_str(r#""value""#)
        .unwrap()
        .parse()
        .parse::<LitStr>()
        .unwrap();
    string.set_span(span);
    assert_eq!(
        (string.value(), string.repr(), string.span(), string.to_string()),
        ("value", "\"value\"", span, "\"value\"".into())
    );
    assert_eq!(serde_json::to_value(string.clone()).unwrap(), "\"value\"");
    assert_eq!(string, LitStr::new("value", Default::default()));

    let mut character = LitChar::new('x', Default::default());
    character.set_span(span);
    assert_eq!(
        (character.value(), character.repr(), character.span(), character.to_string()),
        ('x', "'x'", span, "'x'".into())
    );
    assert_eq!(serde_json::to_value(character.clone()).unwrap(), "'x'");

    let mut byte = TokenStream::from_str("b'x'").unwrap().parse().parse::<LitByte>().unwrap();
    byte.set_span(span);
    assert_eq!(
        (byte.value(), byte.repr(), byte.span(), byte.to_string()),
        (b'x', "b'x'", span, "b'x'".into())
    );
    assert_eq!(serde_json::to_value(byte.clone()).unwrap(), "b'x'");

    let mut bytes = TokenStream::from_str(r#"b"bytes""#)
        .unwrap()
        .parse()
        .parse::<LitByteStr>()
        .unwrap();
    bytes.set_span(span);
    assert_eq!(
        (bytes.value(), bytes.repr(), bytes.span(), bytes.to_string()),
        (b"bytes".as_slice(), "b\"bytes\"", span, "b\"bytes\"".into())
    );
    assert_eq!(serde_json::to_value(bytes.clone()).unwrap(), "b\"bytes\"");

    let mut c_string = TokenStream::from_str(r#"c"ffi""#)
        .unwrap()
        .parse()
        .parse::<LitCStr>()
        .unwrap();
    c_string.set_span(span);
    assert_eq!(
        (c_string.value(), c_string.repr(), c_string.span(), c_string.to_string()),
        (b"ffi".as_slice(), "c\"ffi\"", span, "c\"ffi\"".into())
    );
    assert_eq!(serde_json::to_value(c_string.clone()).unwrap(), "c\"ffi\"");

    let mut boolean = LitBool::new(false, Default::default());
    boolean.set_span(span);
    assert_eq!(
        (boolean.value(), boolean.repr(), boolean.span(), boolean.to_string()),
        (false, "false", span, "false".into())
    );
    assert_eq!(serde_json::to_value(boolean.clone()).unwrap(), "false");

    let pairs = [
        (Lit::from(string), Lit::string("value")),
        (Lit::from(character), Lit::char('x')),
        (Lit::from(byte.clone()), Lit::from(byte)),
        (Lit::from(bytes.clone()), Lit::from(bytes)),
        (Lit::from(c_string.clone()), Lit::from(c_string)),
        (Lit::from(boolean.clone()), Lit::from(boolean)),
    ];
    for (left, right) in pairs {
        let mut left_hash = DefaultHasher::new();
        left.hash(&mut left_hash);
        let mut right_hash = DefaultHasher::new();
        right.hash(&mut right_hash);
        assert_eq!(left, right);
        assert_eq!(left_hash.finish(), right_hash.finish());
    }
}

#[test]
fn literal_parse_reports_the_requested_type() {
    let cases = [
        TokenStream::from_str("1u8")
            .unwrap()
            .parse()
            .parse::<LitI8>()
            .unwrap_err()
            .to_string(),
        TokenStream::from_str("1i8")
            .unwrap()
            .parse()
            .parse::<LitU8>()
            .unwrap_err()
            .to_string(),
        TokenStream::from_str("1.0")
            .unwrap()
            .parse()
            .parse::<LitBool>()
            .unwrap_err()
            .to_string(),
        TokenStream::from_str("'x'")
            .unwrap()
            .parse()
            .parse::<LitStr>()
            .unwrap_err()
            .to_string(),
        TokenStream::from_str("\"x\"")
            .unwrap()
            .parse()
            .parse::<LitChar>()
            .unwrap_err()
            .to_string(),
        TokenStream::from_str("b\"x\"")
            .unwrap()
            .parse()
            .parse::<LitByte>()
            .unwrap_err()
            .to_string(),
        TokenStream::from_str("c\"x\"")
            .unwrap()
            .parse()
            .parse::<LitByteStr>()
            .unwrap_err()
            .to_string(),
        TokenStream::from_str("b\"x\"")
            .unwrap()
            .parse()
            .parse::<LitCStr>()
            .unwrap_err()
            .to_string(),
    ];

    for message in cases {
        assert!(message.contains("expected"));
        assert!(message.contains("literal"));
    }
}

#[test]
#[ignore]
fn valid_byte_escapes_preserve_all_byte_values() {
    let byte = TokenStream::from_str("b'\\xFF'").unwrap().parse().parse::<LitByte>().unwrap();
    assert_eq!(byte.value(), 0xff);

    let bytes = TokenStream::from_str("b\"\\x00\\x7F\\x80\\xFF\"")
        .unwrap()
        .parse()
        .parse::<LitByteStr>()
        .unwrap();
    assert_eq!(bytes.value(), &[0x00, 0x7f, 0x80, 0xff]);
}

#[test]
#[ignore]
fn unsuffixed_integer_keeps_values_above_i32() {
    for source in ["2147483648", "4294967295", "18446744073709551615"] {
        let tokens = TokenStream::from_str(source).unwrap();
        let literal = Lit::parse(&mut tokens.parse()).unwrap();
        assert_eq!(literal.repr(), source);
        assert!(literal.is_int());
    }
}

#[test]
#[ignore]
fn equal_numeric_values_have_equal_hashes() {
    for (left, right) in [("10", "1_0"), ("10u32", "0xAu32"), ("1.0f64", "1.00f64")] {
        let left = Lit::parse(&mut TokenStream::from_str(left).unwrap().parse()).unwrap();
        let right = Lit::parse(&mut TokenStream::from_str(right).unwrap().parse()).unwrap();
        assert_eq!(left, right);

        let mut left_hash = DefaultHasher::new();
        left.hash(&mut left_hash);
        let mut right_hash = DefaultHasher::new();
        right.hash(&mut right_hash);
        assert_eq!(left_hash.finish(), right_hash.finish());
    }
}

#[test]
fn verbatim_fallback_preserves_unknown_repr_and_span() {
    let span = Span::mixed_site();
    let mut literal = Lit::from_repr("@@@", span);
    assert_eq!(literal.repr(), "@@@");
    assert_eq!(literal.span(), span);
    assert_eq!(literal.to_string(), "@@@");
    literal.set_span(Span::def_site());
    assert_eq!(literal.span(), Span::def_site());
}

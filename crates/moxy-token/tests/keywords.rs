use moxy_token::{Keyword, Span, Spanner, ToTokenStream, TokenStream};

#[test]
fn every_keyword_preserves_exact_text_variant_span_and_tokens() {
    for text in [
        "as",
        "async",
        "auto",
        "await",
        "become",
        "box",
        "break",
        "const",
        "continue",
        "crate",
        "default",
        "do",
        "dyn",
        "else",
        "enum",
        "extern",
        "final",
        "fn",
        "for",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "macro",
        "macro_rules",
        "match",
        "mod",
        "move",
        "mut",
        "override",
        "priv",
        "pub",
        "raw",
        "ref",
        "return",
        "Self",
        "self",
        "static",
        "struct",
        "super",
        "trait",
        "try",
        "type",
        "typeof",
        "union",
        "unsafe",
        "unsized",
        "use",
        "virtual",
        "where",
        "while",
        "yield",
    ] {
        let mut keyword = Keyword::from_str(text, Span::call_site()).unwrap();
        assert_eq!(keyword.as_str(), text);
        assert_eq!(keyword.to_string(), text);
        assert_eq!(keyword.to_token_stream().to_string(), text);
        assert_eq!(keyword.to_token_tree().to_string(), text);
        assert_eq!(keyword.into_token_tree().to_string(), text);
        keyword.set_span(Span::mixed_site());
        assert_eq!(keyword.span(), Span::mixed_site());
        assert_eq!(Spanner::span(&keyword), Span::mixed_site());
        #[cfg(feature = "serde")]
        assert_eq!(serde_json::to_value(keyword).unwrap(), text);

        let tree = TokenStream::from(text.parse::<TokenStream>().unwrap().to_vec())
            .into_iter()
            .next()
            .unwrap();
        assert!(tree.is_keyword());
        assert_eq!(tree.as_keyword().unwrap().as_str(), text);
        match text {
            "as" => assert!(tree.is_keyword_as() && tree.as_keyword_as().unwrap().as_str() == text),
            "async" => assert!(tree.is_keyword_async() && tree.as_keyword_async().unwrap().as_str() == text),
            "auto" => assert!(tree.is_keyword_auto() && tree.as_keyword_auto().unwrap().as_str() == text),
            "await" => assert!(tree.is_keyword_await() && tree.as_keyword_await().unwrap().as_str() == text),
            "become" => assert!(tree.is_keyword_become() && tree.as_keyword_become().unwrap().as_str() == text),
            "box" => assert!(tree.is_keyword_box() && tree.as_keyword_box().unwrap().as_str() == text),
            "break" => assert!(tree.is_keyword_break() && tree.as_keyword_break().unwrap().as_str() == text),
            "const" => assert!(tree.is_keyword_const() && tree.as_keyword_const().unwrap().as_str() == text),
            "continue" => assert!(tree.is_keyword_continue() && tree.as_keyword_continue().unwrap().as_str() == text),
            "crate" => assert!(tree.is_keyword_crate() && tree.as_keyword_crate().unwrap().as_str() == text),
            "default" => assert!(tree.is_keyword_default() && tree.as_keyword_default().unwrap().as_str() == text),
            "do" => assert!(tree.is_keyword_do() && tree.as_keyword_do().unwrap().as_str() == text),
            "dyn" => assert!(tree.is_keyword_dyn() && tree.as_keyword_dyn().unwrap().as_str() == text),
            "else" => assert!(tree.is_keyword_else() && tree.as_keyword_else().unwrap().as_str() == text),
            "enum" => assert!(tree.is_keyword_enum() && tree.as_keyword_enum().unwrap().as_str() == text),
            "extern" => assert!(tree.is_keyword_extern() && tree.as_keyword_extern().unwrap().as_str() == text),
            "final" => assert!(tree.is_keyword_final() && tree.as_keyword_final().unwrap().as_str() == text),
            "fn" => assert!(tree.is_keyword_fn() && tree.as_keyword_fn().unwrap().as_str() == text),
            "for" => assert!(tree.is_keyword_for() && tree.as_keyword_for().unwrap().as_str() == text),
            "if" => assert!(tree.is_keyword_if() && tree.as_keyword_if().unwrap().as_str() == text),
            "impl" => assert!(tree.is_keyword_impl() && tree.as_keyword_impl().unwrap().as_str() == text),
            "in" => assert!(tree.is_keyword_in() && tree.as_keyword_in().unwrap().as_str() == text),
            "let" => assert!(tree.is_keyword_let() && tree.as_keyword_let().unwrap().as_str() == text),
            "loop" => assert!(tree.is_keyword_loop() && tree.as_keyword_loop().unwrap().as_str() == text),
            "macro" => assert!(tree.is_keyword_macro() && tree.as_keyword_macro().unwrap().as_str() == text),
            "macro_rules" => assert!(tree.is_keyword_macro_rules() && tree.as_keyword_macro_rules().unwrap().as_str() == text),
            "match" => assert!(tree.is_keyword_match() && tree.as_keyword_match().unwrap().as_str() == text),
            "mod" => assert!(tree.is_keyword_mod() && tree.as_keyword_mod().unwrap().as_str() == text),
            "move" => assert!(tree.is_keyword_move() && tree.as_keyword_move().unwrap().as_str() == text),
            "mut" => assert!(tree.is_keyword_mut() && tree.as_keyword_mut().unwrap().as_str() == text),
            "override" => assert!(tree.is_keyword_override() && tree.as_keyword_override().unwrap().as_str() == text),
            "priv" => assert!(tree.is_keyword_priv() && tree.as_keyword_priv().unwrap().as_str() == text),
            "pub" => assert!(tree.is_keyword_pub() && tree.as_keyword_pub().unwrap().as_str() == text),
            "raw" => assert!(tree.is_keyword_raw() && tree.as_keyword_raw().unwrap().as_str() == text),
            "ref" => assert!(tree.is_keyword_ref() && tree.as_keyword_ref().unwrap().as_str() == text),
            "return" => assert!(tree.is_keyword_return() && tree.as_keyword_return().unwrap().as_str() == text),
            "Self" => assert!(tree.is_keyword_self_type() && tree.as_keyword_self_type().unwrap().as_str() == text),
            "self" => assert!(tree.is_keyword_self_value() && tree.as_keyword_self_value().unwrap().as_str() == text),
            "static" => assert!(tree.is_keyword_static() && tree.as_keyword_static().unwrap().as_str() == text),
            "struct" => assert!(tree.is_keyword_struct() && tree.as_keyword_struct().unwrap().as_str() == text),
            "super" => assert!(tree.is_keyword_super() && tree.as_keyword_super().unwrap().as_str() == text),
            "trait" => assert!(tree.is_keyword_trait() && tree.as_keyword_trait().unwrap().as_str() == text),
            "try" => assert!(tree.is_keyword_try() && tree.as_keyword_try().unwrap().as_str() == text),
            "type" => assert!(tree.is_keyword_type() && tree.as_keyword_type().unwrap().as_str() == text),
            "typeof" => assert!(tree.is_keyword_typeof() && tree.as_keyword_typeof().unwrap().as_str() == text),
            "union" => assert!(tree.is_keyword_union() && tree.as_keyword_union().unwrap().as_str() == text),
            "unsafe" => assert!(tree.is_keyword_unsafe() && tree.as_keyword_unsafe().unwrap().as_str() == text),
            "unsized" => assert!(tree.is_keyword_unsized() && tree.as_keyword_unsized().unwrap().as_str() == text),
            "use" => assert!(tree.is_keyword_use() && tree.as_keyword_use().unwrap().as_str() == text),
            "virtual" => assert!(tree.is_keyword_virtual() && tree.as_keyword_virtual().unwrap().as_str() == text),
            "where" => assert!(tree.is_keyword_where() && tree.as_keyword_where().unwrap().as_str() == text),
            "while" => assert!(tree.is_keyword_while() && tree.as_keyword_while().unwrap().as_str() == text),
            "yield" => assert!(tree.is_keyword_yield() && tree.as_keyword_yield().unwrap().as_str() == text),
            _ => unreachable!(),
        }
    }
    assert!(Keyword::from_str("ordinary_identifier", Span::call_site()).is_none());
}

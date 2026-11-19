use std::str::FromStr;

use moxy::Token;
use moxy::ast::{Parse, Parser};
use moxy::token::{Delim, TokenStream, TokenTree};

#[test]
fn peek_and_optional_parses_do_not_advance() {
    let tokens = TokenStream::from_str("fn value").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let remaining = parser.remaining();

    assert!(<Token![fn]>::peek(parser.cursor()));
    assert_eq!(parser.remaining(), remaining);

    let optional = Option::<Token![struct]>::parse(&parser).unwrap();
    assert!(optional.is_none());
    assert_eq!(parser.remaining(), remaining);

    <Token![fn]>::parse(&parser).unwrap();
    assert_eq!(parser.remaining(), remaining - 1);

    let tokens = TokenStream::from_str("fn").unwrap();
    let parser = Parser::from_tokens(&tokens);
    assert!(<Token![struct]>::parse(&parser).is_err());
    assert!(parser.is_empty());
}

#[test]
fn terminating_repetition_leaves_the_first_nonmatching_token() {
    let tokens = TokenStream::from_str("fn fn struct").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let parsed = parser.parse_while::<Token![fn]>();

    assert_eq!(parsed.len(), 2);
    assert!(<Token![struct]>::peek(parser.cursor()));

    <Token![struct]>::parse(&parser).unwrap();
    assert!(parser.is_empty());
}

#[test]
fn cursor_access_is_bounds_safe_and_tracks_advancement() {
    let empty = TokenStream::new();
    let parser = Parser::from_tokens(&empty);
    assert!(parser.curr().is_none());
    assert!(parser.next().is_none());
    assert!(parser.prev().is_none());
    assert!(parser.nth(usize::MAX).is_none());
    assert!(parser.advance().is_none());

    let tokens = TokenStream::from_str("fn value").unwrap();
    let parser = Parser::from_tokens(&tokens);
    assert!(matches!(parser.curr(), Some(TokenTree::Keyword(_))));
    assert!(matches!(parser.next(), Some(TokenTree::Ident(_))));
    parser.advance();
    assert!(matches!(parser.prev(), Some(TokenTree::Keyword(_))));
    assert!(matches!(parser.curr(), Some(TokenTree::Ident(_))));
}

#[test]
fn group_parsing_creates_an_independent_nested_parser() {
    let tokens = TokenStream::from_str("(fn)").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let inner = parser.parse_group(Delim::Paren).unwrap();
    assert!(parser.is_empty());

    <Token![fn]>::parse(&inner).unwrap();
    assert!(inner.is_empty());
}

#[test]
fn public_parse_rejects_trailing_tokens() {
    assert!(moxy::parse!("fn value" as Token![fn]).is_err());
}

#[test]
fn delimited_lists_that_stop_early_are_errors() {
    for source in [
        "fn f(x: u8 y: u8) {}",
        "fn f(x: Vec<u8 u8>) {}",
        "fn f() { g(a b) }",
        "fn f() { match x { 1 => a b } }",
        "fn f() { let s = S { a: 1 b: 2 }; }",
        "fn f() { [1 2] }",
        "fn f() { (1 2) }",
        "fn f() { x.y::<u8 u8>() }",
        "impl A { fn f() {} @ }",
        "mod m { fn f() {} @ }",
    ] {
        let error = moxy::parse!(source as moxy::ast::Item)
            .err()
            .unwrap_or_else(|| panic!("`{source}` parsed"));
        assert_eq!(error.message(), "unexpected trailing input", "`{source}`");
    }
}

#[test]
fn combined_errors_emit_one_compile_error_each() {
    let tokens = TokenStream::from_str("first second third").unwrap();
    let spans: Vec<moxy::token::Span> = tokens.iter().map(moxy::token::Spanner::span).collect();
    let error = moxy::ast::ParseError::new(spans[0], "first failed")
        .combine(moxy::ast::ParseError::new(spans[1], "second failed"))
        .combine(moxy::ast::ParseError::new(spans[2], "third failed"));

    assert_eq!(
        error.to_compile_error().to_string(),
        "compile_error ! (\"first failed\") ; compile_error ! (\"second failed\") ; compile_error ! (\"third failed\") ;"
    );
}

#[test]
fn spanned_errors_run_from_the_first_token_to_the_last() {
    let tokens = TokenStream::from_str("call(a, b) + Vec<u8>").unwrap();
    let error = moxy::ast::ParseError::new_spanned(&tokens, "unsupported");
    let emitted: Vec<TokenTree> = error.to_compile_error().iter().cloned().collect();
    let TokenTree::Ident(first) = tokens.iter().next().unwrap() else {
        panic!("expected an identifier");
    };
    let last = tokens.iter().last().unwrap().span();

    assert_eq!(error.span(), first.span());
    assert_eq!(emitted[0].span(), first.span());
    assert_eq!(emitted[1].span(), first.span());
    assert_eq!(emitted[3].span(), last);
    assert_eq!(error.to_compile_error().to_string(), "compile_error ! (\"unsupported\") ;");

    let tokens = TokenStream::from_str("(a, b)").unwrap();
    let TokenTree::Group(group) = tokens.iter().next().unwrap() else {
        panic!("expected a group");
    };
    let emitted: Vec<TokenTree> = moxy::ast::ParseError::new_spanned(&tokens, "unsupported")
        .to_compile_error()
        .iter()
        .cloned()
        .collect();
    assert_eq!(emitted[0].span(), group.span.open());
    assert_eq!(emitted[3].span(), group.span.close());
}

#[cfg(feature = "diagnostic")]
#[test]
fn diagnostics_from_spanned_errors_keep_both_ends() {
    let tokens = TokenStream::from_str("call(a, b) + Vec<u8>").unwrap();
    let error = moxy::ast::ParseError::new_spanned(&tokens, "unsupported");
    let diagnostic = moxy::diagnostic::Diagnostic::from(&error);

    assert_eq!(diagnostic.spans(), [error.span(), error.end_span()]);
    let emitted: Vec<TokenTree> = diagnostic.to_compile_error().iter().cloned().collect();
    assert_eq!(emitted[0].span(), error.span());
    assert_eq!(emitted[3].span(), error.end_span());
}
